import { useState, useEffect } from 'react'
import LoginForm from './components/LoginForm'
import UserAvatar from './components/UserAvatar'
import BooksPage from './pages/BooksPage'
import MessageContainer from './components/MessageContainer'
import ExportQueuePanel from './components/ExportQueuePanel'
import { tauriApi } from './services/tauriApi'
import { storageService } from './services/storage'
import { useMessage } from './hooks/useMessage'
import { useExportStore } from './stores/exportStore'

function App() {
  const [isLoggedIn, setIsLoggedIn] = useState(false)
  const [isLoading, setIsLoading] = useState(false)
  const [userInfo, setUserInfo] = useState<YuqueUserInfo | null>(null)
  const [errorMessage, setErrorMessage] = useState<string>('')
  const {
    messages,
    removeMessage,
    error: showError,
    success: showSuccess,
    timeout: showTimeout,
  } = useMessage()
  const {
    tasks: exportTasks,
    clearCompletedTasks: clearCompletedExportTasks,
    clearAllTasks,
  } = useExportStore()

  // 检查登录状态
  useEffect(() => {
    checkLoginStatus()
  }, []) // eslint-disable-line react-hooks/exhaustive-deps

  const checkLoginStatus = async () => {
    try {
      // 检查本地存储
      if (storageService.hasValidLogin()) {
        const storedUserInfo = storageService.getUserInfo()
        const storedCookies = storageService.getCookies()

        if (storedUserInfo && storedCookies.length > 0) {
          // 设置 Rust 后端的缓存状态
          await tauriApi.setCachedUserInfo(storedUserInfo)
          await tauriApi.setCachedCookies(storedCookies)

          // 验证登录状态是否仍然有效
          const isValid = await tauriApi.checkLoginStatus()
          if (isValid) {
            setUserInfo(storedUserInfo)
            setIsLoggedIn(true)

            // 缓存登录成功时也要展开窗口
            expandWindowToMain()

            return
          }
        }
      }

      // 如果没有有效登录，清除所有数据
      handleLogout()
    } catch (error) {
      console.error('检查登录状态失败:', error)
      handleLogout()
    }
  }

  const handleLogin = async (account: YuqueAccount) => {
    setIsLoading(true)
    setErrorMessage('')

    try {
      const response = await tauriApi.login(account)

      if (response.success && response.user_info) {
        // 存储用户信息和 cookies
        storageService.setUserInfo(response.user_info)
        storageService.setCookies(response.cookies || [])

        // 设置 Rust 后端的缓存状态
        await tauriApi.setCachedUserInfo(response.user_info)
        await tauriApi.setCachedCookies(response.cookies || [])

        // 更新状态
        setUserInfo(response.user_info)
        setIsLoggedIn(true)
        setErrorMessage('')

        // 延迟展开窗口到主程序尺寸（等待状态更新完成）
        expandWindowToMain()
      } else {
        setErrorMessage(response.message || '登录失败')
      }
    } catch (error) {
      console.error('登录失败:', error)
      setErrorMessage('登录过程中发生错误，请重试')
    } finally {
      setIsLoading(false)
    }
  }

  // 通用的窗口展开函数
  const expandWindowToMain = async () => {
    setTimeout(async () => {
      console.log('Debug: 延迟展开窗口...')
      try {
        await tauriApi.expandWindow()
        console.log('Debug: 窗口展开成功')
      } catch (error) {
        console.error('Debug: 窗口展开失败:', error)
      }
    }, 100)
  }

  const handleLogout = async () => {
    // 清除 Rust 后端状态
    tauriApi.clearLoginStatus()

    // 清除本地存储
    storageService.clearLoginData()

    // 收缩窗口到登录页面尺寸
    console.log('Debug: 开始收缩窗口...')
    try {
      await tauriApi.shrinkWindow()
      console.log('Debug: 窗口收缩成功')
    } catch (error) {
      console.error('Debug: 窗口收缩失败:', error)
    }

    // 更新组件状态
    setUserInfo(null)
    setIsLoggedIn(false)
    setErrorMessage('')
  }

  return (
    <div className="h-full bg-gray-100 pt-2 pb-2 overflow-hidden box-border">
      <div className="w-full h-full px-4">
        {/* 头部标题 */}
        {/* <div className="text-center mb-8">
          <h1 className="text-4xl font-bold text-gray-900 mb-4">Yuque Tools GUI</h1>
        </div> */}

        {/* 错误消息显示 */}
        {errorMessage && (
          <div className="max-w-md mx-auto mb-6">
            <div className="bg-red-50 border border-red-200 rounded-lg p-4">
              <div className="flex">
                <div className="flex-shrink-0">
                  <svg className="h-5 w-5 text-red-400" viewBox="0 0 20 20" fill="currentColor">
                    <path
                      fillRule="evenodd"
                      d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 4.293 5.707a1 1 0 010-1.414z"
                      clipRule="evenodd"
                    />
                  </svg>
                </div>
                <div className="ml-3">
                  <p className="text-sm text-red-800">{errorMessage}</p>
                </div>
                <div className="ml-auto pl-3">
                  <button
                    onClick={() => setErrorMessage('')}
                    className="inline-flex text-red-400 hover:text-red-600"
                  >
                    <svg className="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                      <path
                        fillRule="evenodd"
                        d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L10 11.414l4.293 4.293a1 1 0 01-1.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 4.293 5.707a1 1 0 010-1.414z"
                        clipRule="evenodd"
                      />
                    </svg>
                  </button>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* 主要内容区域 */}
        {!isLoggedIn ? (
          <LoginForm onLogin={handleLogin} isLoading={isLoading} />
        ) : (
          <div className="w-full h-full p-2">
            {/* 导航栏 */}
            <div className="bg-white rounded-lg shadow-lg py-2 px-4 mb-4">
              <div className="flex items-center justify-between">
                <div className="flex space-x-4">
                  {/* <button className="px-4 py-2 rounded-lg font-medium bg-blue-600 text-white">
                    知识库信息
                  </button> */}
                  <h1 className="text-2xl font-bold text-blue-600">语雀资源导出工具</h1>
                </div>

                {/* 用户头像 */}
                {userInfo && <UserAvatar userInfo={userInfo} onLogout={handleLogout} />}
              </div>
            </div>

            {/* 页面内容 */}
            <BooksPage
              showSuccess={showSuccess}
              showError={showError}
              showInfo={(message: string) => showSuccess(message)} // 暂时用 success 替代 info
              showWarning={(message: string) => showError(message)} // 暂时用 error 替代 warning
              showTimeout={showTimeout}
            />
          </div>
        )}

        {/* 功能特性展示 */}
        {/* {!isLoggedIn && (
          <div className="mt-12">
            <div className="bg-white rounded-lg shadow-lg p-8">
              <h3 className="text-2xl font-bold text-gray-900 text-center mb-8">功能特性</h3>
              <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                <div className="bg-blue-50 p-6 rounded-lg border border-blue-200">
                  <h4 className="text-lg font-semibold text-blue-900 mb-2">知识库导出</h4>
                  <p className="text-blue-700">支持个人和团队知识库的批量导出</p>
                </div>

                <div className="bg-green-50 p-6 rounded-lg border border-green-200">
                  <h4 className="text-lg font-semibold text-green-900 mb-2">团队资源下载</h4>
                  <p className="text-green-700">一键下载团队空间中的所有资源文件</p>
                </div>

                <div className="bg-purple-50 p-6 rounded-lg border border-purple-200">
                  <h4 className="text-lg font-semibold text-purple-900 mb-2">智能缓存</h4>
                  <p className="text-purple-700">智能缓存管理，支持断点续传</p>
                </div>
              </div>
            </div>
          </div>
        )} */}
      </div>

      {/* 全局消息容器 */}
      <MessageContainer messages={messages} onRemove={removeMessage} />

      {/* 全局导出队列面板 - 只在登录后显示 */}
      {isLoggedIn && (
        <ExportQueuePanel
          tasks={exportTasks}
          onClearCompleted={clearCompletedExportTasks}
          onClearAll={clearAllTasks}
          width="w-96"
        />
      )}
    </div>
  )
}

export default App
