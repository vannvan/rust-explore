# Libs 模块说明

本目录包含了重构后的语雀工具核心功能模块，实现了合理的功能划分和模块化管理。

## 模块结构

### 1. models.rs - 数据模型

- 包含所有数据结构定义
- `YuqueAccount`: 语雀账户信息
- `YuqueUserInfo`: 用户信息
- `DocItem`: 文档项目结构
- `BookItem`: 知识库项目结构
- `ApiResponse<T>`: 通用 API 响应结构

### 2. http_utils.rs - HTTP 工具

- `HttpUtils::build_headers()`: 构建 HTTP 请求头
- `HttpUtils::create_client()`: 创建 HTTP 客户端
- `HttpUtils::gen_timestamp()`: 生成时间戳

### 3. crypto.rs - 加密工具

- `CryptoUtils::encrypt_password()`: RSA 密码加密
- 使用语雀的 RSA 公钥进行密码加密

### 4. doc_parser.rs - 文档解析

- `DocParser::parse_toc_to_docs()`: 解析目录数据为文档列表
- `DocParser::extract_docs_from_html()`: 从 HTML 内容中提取文档数据
- 递归解析目录结构

### 5. export_utils.rs - 导出工具

- `ExportUtils::export_document()`: 导出单个文档
- `ExportUtils::export_documents()`: 批量导出文档
- `ExportUtils::get_markdown_content()`: 获取 Markdown 内容

## 重构效果

### 代码行数对比

- **重构前**: `yuque_service.rs` - 1446 行
- **重构后**: `yuque_service.rs` - 约 500 行
- **减少**: 约 65%的代码量

### 模块化优势

1. **单一职责**: 每个模块专注于特定功能
2. **易于维护**: 代码结构清晰，便于定位和修改
3. **可重用性**: 工具函数可以在其他地方复用
4. **测试友好**: 每个模块可以独立测试
5. **团队协作**: 不同开发者可以并行开发不同模块

## 使用方式

### 在 yuque_service.rs 中引入

```rust
use crate::libs::{
    models::*,
    http_utils::HttpUtils,
    crypto::CryptoUtils,
    doc_parser::DocParser,
    export_utils::ExportUtils,
};
```

### 调用示例

```rust
// 使用加密工具
let encrypted = CryptoUtils::encrypt_password(&password);

// 使用HTTP工具
let headers = HttpUtils::build_headers(&cookies);

// 使用文档解析器
let docs = DocParser::extract_docs_from_html(&html_content)?;

// 使用导出工具
let file_path = ExportUtils::export_document(
    &client, doc, book_slug, output_dir, &cookies, &user_login
).await?;
```

## 注意事项

1. **依赖关系**: 确保所有必要的依赖都已正确引入
2. **错误处理**: 保持一致的错误处理模式
3. **测试覆盖**: 为每个模块编写相应的测试用例
4. **文档更新**: 及时更新相关文档和注释
