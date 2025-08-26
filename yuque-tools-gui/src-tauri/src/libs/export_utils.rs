use crate::libs::constants::Export;
use crate::libs::models::DocItem;
use reqwest::Client;

/// 导出工具模块
pub struct ExportUtils;

impl ExportUtils {
    /// 导出单个文档
    pub async fn export_document(
        client: &Client,
        doc: &DocItem,
        book_slug: &str,
        output_dir: &str,
        cookies: &[String],
        user_login: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        println!("=== 开始导出文档 ===");
        // println!("文档标题: {}", doc.title);
        // println!("文档UUID: {}", doc.uuid);
        // println!("父级UUID: {}", doc.parent_uuid);
        println!("文档URL: {}", doc.url);
        // println!("文档类型: {}", doc.node_type);
        // println!("文档slug: {:?}", doc.slug);
        println!("知识库slug: {}", book_slug);
        println!("文档保存路径: {:?}", doc.doc_full_path);

        // 检查必要参数
        if doc.slug.is_none() {
            println!("错误: 文档缺少slug字段");
            return Err("文档缺少slug字段，无法构建导出URL".into());
        }

        // 构建文档路径 - 使用正确的格式：{知识库的slug}/{文档的slug}
        let doc_slug = doc.slug.as_ref().unwrap();
        let repos = format!("{}/{}", book_slug, doc.url);
        println!("构建的repos参数: {}", repos);

        // 检查repos参数的有效性
        if book_slug.is_empty() {
            println!("警告: 知识库slug为空");
        }
        if doc_slug.is_empty() {
            println!("警告: 文档slug为空");
        }

        // 获取Markdown内容
        println!("开始获取Markdown内容...");
        let content = Self::get_markdown_content(client, &repos, cookies, user_login).await?;

        if content.is_empty() {
            println!("错误: 获取到的内容为空");
            return Err("获取文档内容失败，非Markdown文件".into());
        }

        println!("成功获取内容，长度: {} 字符", content.len());

        // 使用 docFullPath 构建文件保存路径，保持目录结构
        let file_path = if let Some(doc_full_path) = &doc.doc_full_path {
            // 分离目录路径和文件名
            let path_parts: Vec<&str> = doc_full_path.split('/').collect();

            if path_parts.len() > 1 {
                // 有目录结构：创建目录路径，文件名单独处理
                let dir_path = path_parts[..path_parts.len() - 1].join("/");
                let file_name = path_parts.last().unwrap();

                // 只清理文件名中的非法字符，保持目录结构
                let clean_file_name =
                    file_name.replace(&Export::ILLEGAL_CHARS, Export::REPLACEMENT_CHAR);
                format!("{}/{}.md", dir_path, clean_file_name)
            } else {
                // 没有目录结构：直接使用路径作为文件名
                let clean_file_name =
                    doc_full_path.replace(&Export::ILLEGAL_CHARS, Export::REPLACEMENT_CHAR);
                format!("{}.md", clean_file_name)
            }
        } else {
            // 如果没有完整路径，使用标题作为文件名
            let clean_title = doc
                .title
                .replace(&Export::ILLEGAL_CHARS, Export::REPLACEMENT_CHAR);
            format!("{}.md", clean_title)
        };

        // 构建完整的输出路径
        let full_output_path = format!("{}/{}", output_dir, file_path);
        println!("输出文件路径: {}", full_output_path);

        // 确保目录存在
        if let Some(parent) = std::path::Path::new(&full_output_path).parent() {
            std::fs::create_dir_all(parent)?;
            println!("创建目录: {:?}", parent);
        }

        // 写入文件
        std::fs::write(&full_output_path, content)?;
        println!("文件写入成功: {}", full_output_path);

        Ok(full_output_path)
    }

    /// 批量导出文档
    pub async fn export_documents(
        client: &Client,
        docs: &[DocItem],
        book_slug: &str,
        output_dir: &str,
        cookies: &[String],
        user_login: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
        let mut exported_files = Vec::new();

        for doc in docs {
            match Self::export_document(client, doc, book_slug, output_dir, cookies, user_login)
                .await
            {
                Ok(file_path) => {
                    exported_files.push(file_path);
                    println!("导出成功: {}", doc.title);
                }
                Err(e) => {
                    println!("导出失败 {}: {}", doc.title, e);
                }
            }
        }

        Ok(exported_files)
    }

    /// 获取文档的Markdown内容
    async fn get_markdown_content(
        client: &Client,
        repos: &str,
        cookies: &[String],
        user_login: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // 使用正确的语雀导出API格式
        let url = crate::libs::api_config::Documents::markdown_export_url(user_login, repos);

        println!("=== 获取Markdown内容 ===");
        println!("完整URL: {}", url);
        println!("repos参数: {}", repos);

        // 打印cookies内容（脱敏处理）
        if !cookies.is_empty() {
            let cookie_preview = if cookies[0].len() > 50 {
                // 安全地获取前50个字符，确保不破坏UTF-8边界
                let mut chars: Vec<char> = cookies[0].chars().take(50).collect();
                chars.push('…'); // 使用省略号字符
                chars.into_iter().collect::<String>()
            } else {
                cookies[0].clone()
            };
            println!("Cookie预览: {}", cookie_preview);
        }

        println!("发送GET请求...");
        let response = client
            .get(&url)
            .header("Cookie", cookies.join("; "))
            .send()
            .await?;

        let status = response.status();
        println!("收到响应，状态码: {}", status);

        if status.is_success() {
            println!("请求成功，开始读取响应内容...");
            let content = response.text().await?;
            println!("响应内容长度: {} 字符", content.len());

            // 打印内容预览（前200字符）- 安全处理UTF-8边界
            let preview = if content.len() > 200 {
                // 安全地获取前200个字符，确保不破坏UTF-8边界
                let mut chars: Vec<char> = content.chars().take(200).collect();
                if content.chars().count() > 200 {
                    chars.push('…'); // 使用省略号字符
                }
                chars.into_iter().collect::<String>()
            } else {
                content.clone()
            };
            println!("内容预览: {}", preview);

            Ok(content)
        } else {
            println!("请求失败，状态码: {}", status);
            Err(format!("获取文档内容失败，非Markdown文件: {}", status).into())
        }
    }
}
