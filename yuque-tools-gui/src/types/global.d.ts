// 全局类型声明文件
// 所有类型都将自动可用，无需导入

declare global {
  // 语雀用户配置
  interface YuqueConfig {
    username: string
    password: string
    host: string
    toc_range: string[]
    skip: boolean
    line_break: boolean
    output: string
  }

  // 语雀账户信息
  interface YuqueAccount {
    username: string
    password: string
  }

  // 语雀用户信息
  interface YuqueUserInfo {
    followers_count: number
    following_count: number
    hasMemberLevel: any
    memberLevelName: string
    isPaid: undefined
    id: number
    login: string
    name: string
    avatar_url?: string
    description?: string
  }

  // 语雀知识库信息
  interface YuqueBook {
    id: number
    name: string
    slug: string
    description?: string
    user: {
      login: string
      name: string
    }
    namespace: string
    type: 'Book' | 'Group'
  }

  // 文档项目结构
  interface DocItem {
    title: string
    type: string // "DOC" 或 "TITLE"
    uuid: string
    child_uuid: string
    parent_uuid: string
    visible: number
    url: string
    slug?: string // 文档的slug，用于构建导出URL
    doc_id?: string | number
    id?: string | number
    open_window?: number
    prev_uuid?: string
    sibling_uuid?: string
    level?: number
    bookSlug?: string // 知识库的slug，用于导出时构建正确的URL
  }

  // 知识库项目结构
  interface BookItem {
    name: string
    slug: string
    stack_id?: string
    book_id?: number
    user_login: string
    user_name: string
    book_type: string // "owner" 或 "collab"
    docs: DocItem[]
  }

  // 知识库列表响应
  interface BooksResponse {
    success: boolean
    data?: BookItem[]
    message?: string
    total_count?: number
  }

  // 登录响应
  interface LoginResponse {
    success: boolean
    message: string
    user_info?: YuqueUserInfo
    cookies?: string[]
  }

  // API 响应格式
  interface ApiResponse<T> {
    data?: T
    message?: string
    success: boolean
  }
}

// 这个导出是必需的，让 TypeScript 知道这是一个模块
export {}
