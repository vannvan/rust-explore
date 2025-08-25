// 存储键名常量
const STORAGE_KEYS = {
  USER_INFO: 'yuque_user_info',
  USER_CONFIG: 'yuque_user_config',
  COOKIES: 'yuque_cookies',
  LOGIN_TIME: 'yuque_login_time',
}

// 本地存储服务
class StorageService {
  // 存储用户信息
  setUserInfo(userInfo: YuqueUserInfo): void {
    try {
      localStorage.setItem(STORAGE_KEYS.USER_INFO, JSON.stringify(userInfo))
      localStorage.setItem(STORAGE_KEYS.LOGIN_TIME, Date.now().toString())
    } catch (error) {
      console.error('存储用户信息失败:', error)
    }
  }

  // 获取用户信息
  getUserInfo(): YuqueUserInfo | null {
    try {
      const userInfoStr = localStorage.getItem(STORAGE_KEYS.USER_INFO)
      if (userInfoStr) {
        return JSON.parse(userInfoStr)
      }
    } catch (error) {
      console.error('获取用户信息失败:', error)
    }
    return null
  }

  // 存储用户配置
  setUserConfig(config: YuqueConfig): void {
    try {
      localStorage.setItem(STORAGE_KEYS.USER_CONFIG, JSON.stringify(config))
    } catch (error) {
      console.error('存储用户配置失败:', error)
    }
  }

  // 获取用户配置
  getUserConfig(): YuqueConfig | null {
    try {
      const configStr = localStorage.getItem(STORAGE_KEYS.USER_CONFIG)
      if (configStr) {
        return JSON.parse(configStr)
      }
    } catch (error) {
      console.error('获取用户配置失败:', error)
    }
    return null
  }

  // 存储 cookies
  setCookies(cookies: string[]): void {
    try {
      localStorage.setItem(STORAGE_KEYS.COOKIES, JSON.stringify(cookies))
    } catch (error) {
      console.error('存储 cookies 失败:', error)
    }
  }

  // 获取 cookies
  getCookies(): string[] {
    try {
      const cookiesStr = localStorage.getItem(STORAGE_KEYS.COOKIES)
      if (cookiesStr) {
        return JSON.parse(cookiesStr)
      }
    } catch (error) {
      console.error('获取 cookies 失败:', error)
    }
    return []
  }

  // 获取登录时间
  getLoginTime(): number | null {
    try {
      const loginTimeStr = localStorage.getItem(STORAGE_KEYS.LOGIN_TIME)
      if (loginTimeStr) {
        return parseInt(loginTimeStr, 10)
      }
    } catch (error) {
      console.error('获取登录时间失败:', error)
    }
    return null
  }

  // 检查登录是否过期（24小时）
  isLoginExpired(): boolean {
    const loginTime = this.getLoginTime()
    if (!loginTime) {
      return true
    }

    const now = Date.now()
    const expireTime = 24 * 60 * 60 * 1000 // 24小时
    return now - loginTime > expireTime
  }

  // 清除所有登录相关数据
  clearLoginData(): void {
    try {
      localStorage.removeItem(STORAGE_KEYS.USER_INFO)
      localStorage.removeItem(STORAGE_KEYS.COOKIES)
      localStorage.removeItem(STORAGE_KEYS.LOGIN_TIME)
    } catch (error) {
      console.error('清除登录数据失败:', error)
    }
  }

  // 清除所有数据
  clearAll(): void {
    try {
      localStorage.removeItem(STORAGE_KEYS.USER_INFO)
      localStorage.removeItem(STORAGE_KEYS.USER_CONFIG)
      localStorage.removeItem(STORAGE_KEYS.COOKIES)
      localStorage.removeItem(STORAGE_KEYS.LOGIN_TIME)
    } catch (error) {
      console.error('清除所有数据失败:', error)
    }
  }

  // 检查是否有有效的登录状态
  hasValidLogin(): boolean {
    const userInfo = this.getUserInfo()
    const cookies = this.getCookies()
    const isExpired = this.isLoginExpired()

    return !!(userInfo && cookies.length > 0 && !isExpired)
  }
}

// 创建单例实例
export const storageService = new StorageService()
export default storageService
