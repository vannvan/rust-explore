# 语雀工具配置文件系统总结

## 配置文件结构

### 1. constants.rs - 常量配置

**位置**: `src-tauri/src/libs/constants.rs`

**包含内容**:

- **BASE_URL**: 语雀主域名 `https://www.yuque.com`
- **Headers**: HTTP 请求头常量
  - `CONTENT_TYPE`: `application/x-www-form-urlencoded`
  - `REFERER`: `https://www.yuque.com`
  - `ORIGIN`: `https://www.yuque.com`
  - `USER_AGENT_MOBILE`: 移动端 User-Agent
  - `USER_AGENT_DESKTOP`: 桌面端 User-Agent
- **Export**: 导出相关常量
  - `MARKDOWN_PARAMS`: Markdown 导出参数
  - `ILLEGAL_CHARS`: 文件名非法字符
  - `REPLACEMENT_CHAR`: 替换字符
- **ErrorMessages**: 错误消息常量
- **SuccessMessages**: 成功消息常量
- **DebugMessages**: 调试信息常量
- **Cache**: 缓存相关常量
- **Crypto**: 加密相关常量

**使用方式**:

```rust
use crate::libs::constants::{Headers, ErrorMessages, SuccessMessages};

// 使用HTTP头常量
headers.insert("Content-Type", Headers::CONTENT_TYPE.parse().unwrap());

// 使用错误消息
message: Some(ErrorMessages::NOT_LOGGED_IN.to_string())
```

### 2. api_config.rs - API 接口配置

**位置**: `src-tauri/src/libs/api_config.rs`

**包含内容**:

- **Auth**: 认证相关接口
  - `LOGIN`: `api/mobile_app/accounts/login`
  - `login_url()`: 完整登录 URL
- **User**: 用户相关接口
  - `MINE`: `api/mine`
  - `mine_url()`: 完整用户信息 URL
- **Books**: 知识库相关接口
  - `PERSONAL_BOOKS`: `api/mine/book_stacks`
  - `TEAM_BOOKS`: `api/mine/user_books`
  - `personal_books_url()`: 个人知识库 URL
  - `team_books_url()`: 团队知识库 URL
  - `book_page_url()`: 知识库页面 URL
- **Documents**: 文档相关接口
  - `markdown_export_url()`: Markdown 导出 URL
  - `doc_url()`: 文档 URL
- **其他接口**: 搜索、团队、通知、统计、文件、评论、标签、收藏、历史记录、导出、权限等

**使用方式**:

```rust
use crate::libs::api_config::{Auth, Books, User};

// 使用登录URL
.post(&Auth::login_url())

// 使用用户信息URL
.get(&User::mine_url())

// 使用知识库URL
.get(&Books::personal_books_url())
```

## 重构效果

### 1. 配置集中化

- 所有固定配置集中在 `constants.rs` 中
- 所有 API 接口配置集中在 `api_config.rs` 中
- 消除了代码中的硬编码字符串

### 2. 易于维护

- 修改配置只需在一个地方进行
- 新增接口只需在对应模块中添加
- 配置变更不会影响业务逻辑代码

### 3. 便于扩展

- 新增常量只需在对应结构体中添加
- 新增 API 接口只需创建新的结构体和实现
- 支持按功能模块分组管理

### 4. 代码质量提升

- 消除了魔法字符串
- 提高了代码的可读性
- 减少了拼写错误的可能性

## 扩展指南

### 添加新的常量

```rust
// 在 constants.rs 中添加
pub struct NewConstants;

impl NewConstants {
    pub const NEW_VALUE: &'static str = "new_value";
}
```

### 添加新的 API 接口

```rust
// 在 api_config.rs 中添加
pub struct NewApi;

impl NewApi {
    pub const NEW_ENDPOINT: &'static str = "api/new_endpoint";

    pub fn new_url() -> String {
        format!("{}/{}", BASE_URL, Self::NEW_ENDPOINT)
    }
}
```

### 使用新配置

```rust
// 在需要使用的地方导入
use crate::libs::constants::NewConstants;
use crate::libs::api_config::NewApi;

// 使用常量
let value = NewConstants::NEW_VALUE;

// 使用API配置
let url = NewApi::new_url();
```
