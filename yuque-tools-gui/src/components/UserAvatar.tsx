import React, { useState } from 'react'
import Popup from './Popup'

interface UserAvatarProps {
  userInfo: YuqueUserInfo
  onLogout: () => void
}

const UserAvatar: React.FC<UserAvatarProps> = ({ userInfo, onLogout }) => {
  const [isPopupOpen, setIsPopupOpen] = useState(false)

  const togglePopup = () => {
    setIsPopupOpen(!isPopupOpen)
  }

  const closePopup = () => {
    setIsPopupOpen(false)
  }

  const handleLogout = () => {
    closePopup()
    onLogout()
  }

  return (
    <div className="relative">
      {/* 头像按钮 */}
      <button
        onClick={() => {
          // 如果Popup是打开的，则关闭它
          if (isPopupOpen) {
            closePopup()
          } else {
            togglePopup()
          }
        }}
        className="flex items-center space-x-2 p-2 rounded-full hover:bg-gray-100 transition-colors duration-200"
        aria-label="用户菜单"
      >
        <div className="w-8 h-8 rounded-full overflow-hidden bg-gray-200 flex-shrink-0">
          {userInfo.avatar_url ? (
            <img
              src={userInfo.avatar_url}
              alt={userInfo.name}
              className="w-full h-full object-cover"
            />
          ) : (
            <div className="w-full h-full bg-blue-500 flex items-center justify-center text-white font-medium text-sm">
              {userInfo.name.charAt(0).toUpperCase()}
            </div>
          )}
        </div>
        <span className="text-sm text-gray-700 font-medium hidden md:block">{userInfo.name}</span>
        <svg
          className={`w-4 h-4 text-gray-500 transition-transform duration-200 ${
            isPopupOpen ? 'rotate-180' : ''
          }`}
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            d="M19 9l-7 7-7-7"
            // 注意：SVG <path> 元素上的 onClick 在大多数浏览器中不会触发，除非 pointer-events 被允许，且通常事件会被父 <button> 捕获
            // 这里的 onClick 实际不会生效，建议将 onClick 绑定在 <button> 上，而不是 <path> 上
            // 所以可以直接移除此 onClick，交互逻辑应由 <button> 控制
          />
        </svg>
      </button>

      {/* 用户信息弹窗 */}
      <Popup isOpen={isPopupOpen} onClose={closePopup} position="bottom">
        <div className="p-4">
          {/* 用户基本信息 */}
          <div className="flex items-center space-x-3 pb-3 border-b border-gray-200">
            <div className="w-12 h-12 rounded-full overflow-hidden bg-gray-200 flex-shrink-0">
              {userInfo.avatar_url ? (
                <img
                  src={userInfo.avatar_url}
                  alt={userInfo.name}
                  className="w-full h-full object-cover"
                />
              ) : (
                <div className="w-full h-full bg-blue-500 flex items-center justify-center text-white font-medium text-lg">
                  {userInfo.name.charAt(0).toUpperCase()}
                </div>
              )}
            </div>
            <div className="flex-1 min-w-0">
              <h3 className="text-sm font-semibold text-gray-900 truncate">{userInfo.name}</h3>
              <p className="text-xs text-gray-500 truncate">@{userInfo.login}</p>
              {userInfo.description && (
                <p className="text-xs text-gray-600 mt-1 line-clamp-2">{userInfo.description}</p>
              )}
            </div>
          </div>

          {/* 用户统计信息 */}
          <div className="py-3 border-b border-gray-200">
            <div className="grid grid-cols-2 gap-4 text-center">
              <div>
                <div className="text-lg font-semibold text-gray-900">
                  {userInfo.followers_count || 0}
                </div>
                <div className="text-xs text-gray-500">关注者</div>
              </div>
              <div>
                <div className="text-lg font-semibold text-gray-900">
                  {userInfo.following_count || 0}
                </div>
                <div className="text-xs text-gray-500">关注中</div>
              </div>
            </div>
          </div>

          {/* 会员信息 */}
          {userInfo.hasMemberLevel && (
            <div className="py-3 border-b border-gray-200">
              <div className="flex items-center justify-between">
                <span className="text-xs text-gray-500">会员等级</span>
                <span className="text-xs font-medium text-blue-600">
                  {userInfo.memberLevelName || '免费用户'}
                </span>
              </div>
              {userInfo.isPaid !== undefined && (
                <div className="flex items-center justify-between mt-1">
                  <span className="text-xs text-gray-500">付费状态</span>
                  <span
                    className={`text-xs font-medium ${
                      userInfo.isPaid ? 'text-green-600' : 'text-gray-500'
                    }`}
                  >
                    {userInfo.isPaid ? '已付费' : '未付费'}
                  </span>
                </div>
              )}
            </div>
          )}

          {/* 操作按钮 */}
          <div className="pt-3">
            <button
              onClick={handleLogout}
              className="w-full px-3 py-2 text-sm text-red-600 hover:bg-red-50 rounded-md transition-colors duration-200 flex items-center justify-center space-x-2"
            >
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"
                />
              </svg>
              <span>退出登录</span>
            </button>
          </div>
        </div>
      </Popup>
    </div>
  )
}

export default UserAvatar
