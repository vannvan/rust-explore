// 语雀用户配置
export interface YuqueConfig {
  username: string
  password: string
  host: string
  toc_range: string[]
  skip: boolean
  line_break: boolean
  output: string
}

// 语雀账户信息
export interface YuqueAccount {
  username: string
  password: string
}

// 语雀用户信息
export interface YuqueUserInfo {
  id: number
  login: string
  name: string
  avatar_url?: string
  description?: string
}

// 语雀知识库信息
export interface YuqueBook {
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
export interface DocItem {
  title: string
  type: string // "DOC" 或 "TITLE"
  uuid: string
  child_uuid: string
  parent_uuid: string
  visible: number
  url: string
}

// 知识库项目结构
export interface BookItem {
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
export interface BooksResponse {
  success: boolean
  data?: BookItem[]
  message?: string
  total_count?: number
}

// 登录响应
export interface LoginResponse {
  success: boolean
  message: string
  user_info?: YuqueUserInfo
  cookies?: string[]
}

// API 响应格式
export interface ApiResponse<T> {
  data?: T
  message?: string
  success: boolean
}
