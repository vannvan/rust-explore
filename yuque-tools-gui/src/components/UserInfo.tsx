import React from 'react'

interface UserInfoProps {
  userInfo: YuqueUserInfo
  onLogout: () => void
}

const UserInfo: React.FC<UserInfoProps> = ({ userInfo, onLogout }) => {
  return (
    <div className="max-w-md mx-auto">
      <div className="bg-white rounded-lg shadow-lg p-8">
        <div className="text-center mb-6">
          <div className="w-20 h-20 mx-auto mb-4">
            <img
              src={userInfo.avatar_url || '/default-avatar.svg'}
              alt="用户头像"
              className="w-full h-full rounded-full object-cover border-4 border-blue-100"
              onError={(e) => {
                const target = e.target as HTMLImageElement
                target.src = '/default-avatar.svg'
              }}
            />
          </div>
          <h2 className="text-2xl font-bold text-gray-900 mb-2">{userInfo.name}</h2>
          <p className="text-gray-600 text-sm">@{userInfo.login}</p>
          {userInfo.description && (
            <p className="text-gray-500 text-sm mt-2">{userInfo.description}</p>
          )}
        </div>

        <div className="space-y-4 mb-6">
          <div className="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
            <span className="text-sm font-medium text-gray-700">用户ID</span>
            <span className="text-sm text-gray-900">{userInfo.id}</span>
          </div>
          <div className="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
            <span className="text-sm font-medium text-gray-700">登录名</span>
            <span className="text-sm text-gray-900">{userInfo.login}</span>
          </div>
        </div>

        <div className="flex space-x-3">
          <button
            onClick={onLogout}
            className="flex-1 py-2 px-4 bg-red-600 hover:bg-red-700 text-white rounded-lg font-medium transition-colors duration-200"
          >
            退出登录
          </button>
          {/* <button className="flex-1 py-2 px-4 bg-blue-600 hover:bg-blue-700 text-white rounded-lg font-medium transition-colors duration-200">
            继续使用
          </button> */}
        </div>

        <div className="mt-6 text-center">
          <p className="text-sm text-gray-500">登录成功！您现在可以使用语雀工具的各项功能</p>
        </div>
      </div>
    </div>
  )
}

export default UserInfo
