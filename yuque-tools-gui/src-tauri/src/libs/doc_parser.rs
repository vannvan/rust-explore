use crate::libs::models::DocItem;
use regex::Regex;
use serde_json::Value;

/// 文档解析工具模块
pub struct DocParser;

impl DocParser {
    /// 解析目录数据为文档列表
    pub fn parse_toc_to_docs(
        toc_data: &Value,
    ) -> Result<Vec<DocItem>, Box<dyn std::error::Error + Send + Sync>> {
        let mut docs = Vec::new();

        if let Some(toc_array) = toc_data.as_array() {
            Self::parse_toc_recursive(toc_array, &mut docs, "".to_string());
        }

        println!("Debug: 解析到 {} 个文档", docs.len());
        Ok(docs)
    }

    /// 递归解析目录结构
    fn parse_toc_recursive(
        toc_array: &[Value],
        docs: &mut Vec<DocItem>,
        _parent_uuid: String, // 添加下划线前缀，因为现在不再使用这个参数
    ) {
        for item in toc_array {
            let node_type = item
                .get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("TITLE")
                .to_string();

            let uuid = item
                .get("uuid")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let title = item
                .get("title")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let visible = item.get("visible").and_then(|v| v.as_u64()).unwrap_or(1) as u8;

            let url = item
                .get("url")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let child_uuid = item
                .get("child_uuid")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            // 提取其他字段
            let doc_id = item
                .get("doc_id")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            let id = item
                .get("id")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            let open_window = item
                .get("open_window")
                .and_then(|v| v.as_u64())
                .map(|v| v as u8);

            let prev_uuid = item
                .get("prev_uuid")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            let sibling_uuid = item
                .get("sibling_uuid")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            // 从 TOC 数据中提取原始的 parent_uuid 和 level
            let original_parent_uuid = item
                .get("parent_uuid")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let original_level = item.get("level").and_then(|v| v.as_u64()).map(|v| v as u8);

            // 调试信息（在移动值之前打印）
            println!(
                "Debug: 解析文档 '{}' - type: {}, uuid: {}, parent_uuid: '{}', level: {:?}",
                title, node_type, uuid, original_parent_uuid, original_level
            );

            // 添加所有节点到文档列表（包括文档和目录）
            let doc_item = DocItem {
                title,
                node_type,
                uuid: uuid.clone(),
                child_uuid,
                parent_uuid: original_parent_uuid, // 使用原始的 parent_uuid
                visible,
                url,
                slug: Some(uuid.clone()), // 使用uuid作为slug的默认值
                doc_id,
                id,
                open_window,
                prev_uuid,
                sibling_uuid,
                level: original_level, // 使用原始的 level 字段
                doc_full_path: None,   // 在解析时暂时不设置完整路径
            };

            docs.push(doc_item);

            // 如果有子目录，递归处理
            if let Some(children) = item.get("children") {
                if let Some(children_array) = children.as_array() {
                    Self::parse_toc_recursive(children_array, docs, uuid.clone());
                }
            }
        }
    }

    /// 从HTML内容中提取文档数据
    pub fn extract_docs_from_html(
        html_content: &str,
    ) -> Result<Vec<DocItem>, Box<dyn std::error::Error + Send + Sync>> {
        // 使用正则表达式提取文档数据
        // 参考 yuque-tools 的实现，查找 decodeURIComponent 中的 JSON 数据
        let re = Regex::new(r#"decodeURIComponent\("([^"]+)"\)"#).unwrap();

        if let Some(captures) = re.captures(html_content) {
            if let Some(encoded_data) = captures.get(1) {
                let decoded_data = urlencoding::decode(encoded_data.as_str())
                    .map_err(|e| format!("URL decode failed: {}", e))?;

                // 解析 JSON 数据
                let json_data: Value = serde_json::from_str(&decoded_data)?;

                if let Some(book_data) = json_data.get("book") {
                    if let Some(toc_data) = book_data.get("toc") {
                        return Self::parse_toc_to_docs(toc_data);
                    }
                }
            }
        }

        println!("Debug: [接口获取] 未找到知识库的文档数据");
        Ok(vec![])
    }
}
