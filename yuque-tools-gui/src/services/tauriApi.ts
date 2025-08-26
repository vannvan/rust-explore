import { invoke } from '@tauri-apps/api/tauri'

// Tauri API 服务
class TauriApiService {
  // 登录语雀
  async login(account: YuqueAccount): Promise<LoginResponse> {
    try {
      const response = await invoke<LoginResponse>('login_yuque', { account })
      return response
    } catch (error) {
      console.error('登录失败:', error)
      return {
        success: false,
        message: error instanceof Error ? error.message : '登录失败',
      }
    }
  }

  // 获取用户信息
  async getUserInfo(): Promise<ApiResponse<YuqueUserInfo>> {
    try {
      const response = await invoke<ApiResponse<YuqueUserInfo>>('get_user_info')
      return response
    } catch (error) {
      console.error('获取用户信息失败:', error)
      return {
        success: false,
        message: error instanceof Error ? error.message : '获取用户信息失败',
      }
    }
  }

  // 检查登录状态
  async checkLoginStatus(): Promise<boolean> {
    try {
      const response = await invoke<boolean>('check_login_status')
      return response
    } catch (error) {
      console.error('检查登录状态失败:', error)
      return false
    }
  }

  // 获取缓存的用户信息
  async getCachedUserInfo(): Promise<YuqueUserInfo | null> {
    try {
      const response = await invoke<YuqueUserInfo | null>('get_cached_user_info')
      return response
    } catch (error) {
      console.error('获取缓存用户信息失败:', error)
      return null
    }
  }

  // 获取缓存的 cookies
  async getCachedCookies(): Promise<string[]> {
    try {
      const response = await invoke<string[]>('get_cached_cookies')
      return response
    } catch (error) {
      console.error('获取缓存 cookies 失败:', error)
      return []
    }
  }

  // 设置缓存的用户信息
  async setCachedUserInfo(userInfo: YuqueUserInfo): Promise<void> {
    try {
      await invoke('set_cached_user_info', { userInfo })
    } catch (error) {
      console.error('设置缓存用户信息失败:', error)
    }
  }

  // 设置缓存的 cookies
  async setCachedCookies(cookies: string[]): Promise<void> {
    try {
      await invoke('set_cached_cookies', { cookies })
    } catch (error) {
      console.error('设置缓存 cookies 失败:', error)
    }
  }

  // 清除登录状态
  async clearLoginStatus(): Promise<void> {
    try {
      await invoke('clear_login_status')
    } catch (error) {
      console.error('清除登录状态失败:', error)
    }
  }

  // 清除知识库缓存
  async clearBooksCache(): Promise<void> {
    try {
      await invoke('clear_books_cache')
    } catch (error) {
      console.error('清除知识库缓存失败:', error)
    }
  }

  // 清除文档缓存
  async clearDocsCache(): Promise<void> {
    try {
      await invoke('clear_docs_cache')
    } catch (error) {
      console.error('清除文档缓存失败:', error)
    }
  }

  // 获取个人知识库列表
  async getPersonalBooks(): Promise<BooksResponse> {
    try {
      const response = await invoke<BooksResponse>('get_personal_books')
      return response
    } catch (error) {
      console.error('获取个人知识库列表失败:', error)
      return {
        success: false,
        message: error instanceof Error ? error.message : '获取个人知识库列表失败',
      }
    }
  }

  // 获取团队知识库列表
  async getTeamBooks(): Promise<BooksResponse> {
    try {
      const response = await invoke<BooksResponse>('get_team_books')
      return response
    } catch (error) {
      console.error('获取团队知识库列表失败:', error)
      return {
        success: false,
        message: error instanceof Error ? error.message : '获取团队知识库列表失败',
      }
    }
  }

  // 获取所有知识库列表
  async getBookStacks(): Promise<BooksResponse> {
    try {
      const response = await invoke<BooksResponse>('get_book_stacks')
      return response
    } catch (error) {
      console.error('获取知识库列表失败:', error)
      return {
        success: false,
        message: error instanceof Error ? error.message : '获取知识库列表失败',
      }
    }
  }

  // 获取团队知识库
  async getSpaceBooks(): Promise<ApiResponse<unknown>> {
    try {
      const response = await invoke<ApiResponse<unknown>>('get_space_books')
      return response
    } catch (error) {
      console.error('获取团队知识库失败:', error)
      return {
        success: false,
        message: error instanceof Error ? error.message : '获取团队知识库失败',
      }
    }
  }

  // 展开窗口到主程序尺寸
  async expandWindow(): Promise<void> {
    try {
      await invoke('expand_window')
    } catch (error) {
      console.error('展开窗口失败:', error)
    }
  }

  // 收缩窗口到登录页面尺寸
  async shrinkWindow(): Promise<void> {
    try {
      await invoke('shrink_window')
    } catch (error) {
      console.error('收缩窗口失败:', error)
    }
  }

  // 导出单个文档
  async exportDocument(
    doc: DocItem,
    bookSlug: string
  ): Promise<{ success: boolean; filePath?: string; error?: string }> {
    try {
      console.log('导出文档信息:', {
        doc,
        bookSlug,
        docFullPath: doc.docFullPath, // 现在可以直接访问
      })

      const outputDir = (await invoke('get_downloads_path')) as string
      const filePath = (await invoke('export_document', {
        doc: {
          title: doc.title,
          type: doc.type, // 注意：Rust 后端期望 "type" 字段
          uuid: doc.uuid,
          child_uuid: doc.child_uuid,
          parent_uuid: doc.parent_uuid,
          visible: doc.visible,
          url: doc.url,
          slug: doc.slug || doc.uuid,
          doc_id: doc.doc_id,
          id: doc.id,
          open_window: doc.open_window,
          prev_uuid: doc.prev_uuid,
          sibling_uuid: doc.sibling_uuid,
          level: doc.level,
          doc_full_path: doc.docFullPath, // 确保字段名完全匹配
        },
        bookSlug: bookSlug,
        outputDir: `${outputDir}/yuque-exports`,
      })) as string

      return { success: true, filePath }
    } catch (error) {
      console.error('Failed to export document:', error)
      return { success: false, error: String(error) }
    }
  }

  // 批量导出文档
  async exportDocuments(
    docs: DocItem[],
    bookSlug: string
  ): Promise<{ success: boolean; filePaths?: string[]; error?: string }> {
    try {
      console.log('批量导出文档信息:', {
        docsCount: docs.length,
        bookSlug,
        docsWithPath: docs.map((doc) => ({
          title: doc.title,
          docFullPath: doc.docFullPath,
        })),
      })

      const outputDir = (await invoke('get_downloads_path')) as string
      const filePaths = (await invoke('export_documents', {
        docs: docs.map((doc) => ({
          title: doc.title,
          type: doc.type, // 注意：Rust 后端期望 "type" 字段
          uuid: doc.uuid,
          child_uuid: doc.child_uuid,
          parent_uuid: doc.parent_uuid,
          visible: doc.visible,
          url: doc.url,
          slug: doc.slug || doc.uuid,
          doc_id: doc.doc_id,
          id: doc.id,
          open_window: doc.open_window,
          prev_uuid: doc.prev_uuid,
          sibling_uuid: doc.sibling_uuid,
          level: doc.level,
          doc_full_path: doc.docFullPath, // 确保字段名完全匹配
        })),
        bookSlug: bookSlug,
        outputDir: `${outputDir}/yuque-exports`,
      })) as string[]

      return { success: true, filePaths }
    } catch (error) {
      console.error('Failed to export documents:', error)
      return { success: false, error: String(error) }
    }
  }

  // 获取下载目录路径
  async getDownloadsPath(): Promise<string> {
    try {
      return (await invoke('get_downloads_path')) as string
    } catch (error) {
      console.error('Failed to get downloads path:', error)
      return ''
    }
  }
}

// 创建单例实例
export const tauriApi = new TauriApiService()
export default tauriApi
