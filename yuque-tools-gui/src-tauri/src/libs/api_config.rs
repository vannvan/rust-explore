use super::constants::BASE_URL;

/// 认证相关接口
pub struct Auth;

impl Auth {
    /// 移动端登录接口
    pub const LOGIN: &'static str = "api/mobile_app/accounts/login";

    /// 完整登录URL
    pub fn login_url() -> String {
        format!("{}/{}?language=zh-cn", BASE_URL, Self::LOGIN)
    }
}

/// 用户相关接口
pub struct User;

impl User {
    /// 获取用户信息
    pub const MINE: &'static str = "api/mine";

    /// 完整用户信息URL
    pub fn mine_url() -> String {
        format!("{}/{}", BASE_URL, Self::MINE)
    }
}

/// 知识库相关接口
pub struct Books;

impl Books {
    /// 获取个人知识库
    pub const PERSONAL_BOOKS: &'static str = "api/mine/book_stacks";

    /// 获取团队知识库
    pub const TEAM_BOOKS: &'static str = "api/mine/user_books";

    /// 完整个人知识库URL
    pub fn personal_books_url() -> String {
        format!("{}/{}", BASE_URL, Self::PERSONAL_BOOKS)
    }

    /// 完整团队知识库URL
    pub fn team_books_url() -> String {
        format!("{}/{}?user_type=Group", BASE_URL, Self::TEAM_BOOKS)
    }

    /// 根据用户和知识库slug构建知识库页面URL
    pub fn book_page_url(user_login: &str, book_slug: &str) -> String {
        format!("{}/{}/{}", BASE_URL, user_login, book_slug)
    }
}

/// 文档相关接口
pub struct Documents;

impl Documents {
    /// 根据用户、知识库和文档信息构建Markdown导出URL
    pub fn markdown_export_url(user_login: &str, repos: &str) -> String {
        format!(
            "{}/{}/{}/markdown?attachment=true&latexcode=false&anchor=false&linebreak=false",
            BASE_URL, user_login, repos
        )
    }

    /// 根据用户、知识库和文档slug构建文档URL
    pub fn doc_url(user_login: &str, book_slug: &str, doc_slug: &str) -> String {
        format!("{}/{}/{}/{}", BASE_URL, user_login, book_slug, doc_slug)
    }
}
