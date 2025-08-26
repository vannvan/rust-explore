/// 语雀工具常量配置

/// HTTP请求头常量
pub struct Headers;

impl Headers {
    /// Content-Type
    pub const CONTENT_TYPE: &'static str = "application/x-www-form-urlencoded";

    /// Referer
    pub const REFERER: &'static str = "https://www.yuque.com";

    /// Origin
    pub const ORIGIN: &'static str = "https://www.yuque.com";

    /// User-Agent (移动端)
    pub const USER_AGENT_MOBILE: &'static str = "Mozilla/5.0 (iPhone; CPU iPhone OS 16_6_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/20G81 YuqueMobileApp/1.0.2 (AppBuild/650 Device/Phone Locale/zh-cn Theme/light YuqueType/public)";

    /// User-Agent (桌面端)
    pub const USER_AGENT_DESKTOP: &'static str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36";
}

/// 语雀主域名
pub const BASE_URL: &'static str = "https://www.yuque.com";

/// 导出相关常量
pub struct Export;

impl Export {
    /// Markdown导出参数
    pub const MARKDOWN_PARAMS: &'static str =
        "attachment=true&latexcode=false&anchor=false&linebreak=false";

    /// 文件名非法字符替换
    pub const ILLEGAL_CHARS: [char; 8] = ['<', '>', ':', '"', '\\', '|', '?', '*'];

    /// 替换字符
    pub const REPLACEMENT_CHAR: &'static str = "-";
}

/// 缓存相关常量
pub struct Cache;

impl Cache {
    /// 用户信息缓存键
    pub const USER_INFO_KEY: &'static str = "user_info";

    /// Cookies缓存键
    pub const COOKIES_KEY: &'static str = "cookies";

    /// 知识库缓存键前缀
    pub const BOOKS_CACHE_PREFIX: &'static str = "books_";

    /// 文档缓存键前缀
    pub const DOCS_CACHE_PREFIX: &'static str = "docs_";
}

/// 错误消息常量
pub struct ErrorMessages;

impl ErrorMessages {
    /// 未登录错误
    pub const NOT_LOGGED_IN: &'static str = "未登录";

    /// 登录失败错误
    pub const LOGIN_FAILED: &'static str = "登录失败，请检查用户名和密码";

    /// 获取用户信息失败
    pub const GET_USER_INFO_FAILED: &'static str = "获取用户信息失败";

    /// 获取知识库失败
    pub const GET_BOOKS_FAILED: &'static str = "获取知识库失败";

    /// 获取文档失败
    pub const GET_DOCS_FAILED: &'static str = "获取文档失败";

    /// 导出失败
    pub const EXPORT_FAILED: &'static str = "导出失败";

    /// 缓存操作失败
    pub const CACHE_OPERATION_FAILED: &'static str = "缓存操作失败";
}

/// 成功消息常量
pub struct SuccessMessages;

impl SuccessMessages {
    /// 登录成功
    pub const LOGIN_SUCCESS: &'static str = "登录成功";

    /// 个人知识库
    pub const PERSONAL_BOOKS: &'static str = "个人知识库";

    /// 团队知识库
    pub const TEAM_BOOKS: &'static str = "团队知识库";

    /// 所有知识库
    pub const ALL_BOOKS: &'static str = "所有知识库";
}
