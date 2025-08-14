import { useState } from 'react'
import { User, Settings, Bell, Shield, HelpCircle, LogOut, Edit3, Camera } from 'lucide-react'

// 模拟用户数据
const mockUser = {
  id: 1,
  name: '张三',
  email: 'zhangsan@example.com',
  avatar: 'https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=150&h=150&fit=crop&crop=face',
  joinDate: '2024-01-15',
  totalTasks: 156,
  completedTasks: 142,
  streak: 28
}

const ProfilePage = () => {
  const [isEditing, setIsEditing] = useState(false)
  const [editName, setEditName] = useState(mockUser.name)

  const handleSaveName = () => {
    // 这里可以添加保存逻辑
    setIsEditing(false)
  }

  const menuItems = [
    {
      icon: Bell,
      title: '通知设置',
      subtitle: '管理推送和提醒',
      color: 'text-blue-500',
      bgColor: 'bg-blue-50'
    },
    {
      icon: Shield,
      title: '隐私设置',
      subtitle: '管理数据隐私',
      color: 'text-green-500',
      bgColor: 'bg-green-50'
    },
    {
      icon: HelpCircle,
      title: '帮助与反馈',
      subtitle: '获取帮助或提交反馈',
      color: 'text-purple-500',
      bgColor: 'bg-purple-50'
    },
    {
      icon: Settings,
      title: '应用设置',
      subtitle: '自定义应用偏好',
      color: 'text-orange-500',
      bgColor: 'bg-orange-50'
    }
  ]

  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-50 via-white to-purple-50">
      {/* Header */}
      <div className="bg-white/80 backdrop-blur-md border-b border-gray-200/50 sticky top-0 z-10">
        <div className="max-w-6xl mx-auto px-6 py-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center space-x-3">
              <div className="w-10 h-10 bg-gradient-to-r from-purple-500 to-pink-600 rounded-xl flex items-center justify-center">
                <User className="w-6 h-6 text-white" />
              </div>
              <div>
                <h1 className="text-2xl font-bold bg-gradient-to-r from-purple-600 to-pink-600 bg-clip-text text-transparent">
                  个人中心
                </h1>
                <p className="text-sm text-gray-500">管理您的个人信息和设置</p>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div className="max-w-6xl mx-auto px-6 py-8">
        {/* Profile Card */}
        <div className="bg-white/70 backdrop-blur-sm rounded-2xl shadow-soft border border-white/50 p-6 mb-8">
          <div className="flex items-center space-x-6">
            {/* Avatar */}
            <div className="relative">
              <img
                src={mockUser.avatar}
                alt="用户头像"
                className="w-24 h-24 rounded-full object-cover border-4 border-white shadow-lg"
              />
              <button className="absolute bottom-0 right-0 w-8 h-8 bg-blue-500 text-white rounded-full flex items-center justify-center hover:bg-blue-600 transition-colors duration-200">
                <Camera className="w-4 h-4" />
              </button>
            </div>

            {/* User Info */}
            <div className="flex-1">
              <div className="flex items-center space-x-3 mb-2">
                {isEditing ? (
                  <div className="flex items-center space-x-2">
                    <input
                      type="text"
                      value={editName}
                      onChange={(e) => setEditName(e.target.value)}
                      className="text-2xl font-bold text-gray-900 bg-transparent border-b border-blue-300 focus:outline-none focus:border-blue-500"
                    />
                    <button
                      onClick={handleSaveName}
                      className="px-3 py-1 bg-green-500 text-white text-sm rounded-lg hover:bg-green-600 transition-colors duration-200"
                    >
                      保存
                    </button>
                    <button
                      onClick={() => {
                        setIsEditing(false)
                        setEditName(mockUser.name)
                      }}
                      className="px-3 py-1 bg-gray-500 text-white text-sm rounded-lg hover:bg-gray-600 transition-colors duration-200"
                    >
                      取消
                    </button>
                  </div>
                ) : (
                  <>
                    <h2 className="text-2xl font-bold text-gray-900">{mockUser.name}</h2>
                    <button
                      onClick={() => setIsEditing(true)}
                      className="p-2 text-gray-400 hover:text-gray-600 transition-colors duration-200"
                    >
                      <Edit3 className="w-4 h-4" />
                    </button>
                  </>
                )}
              </div>
              
              <p className="text-gray-600 mb-3">{mockUser.email}</p>
              <p className="text-sm text-gray-500">加入时间: {mockUser.joinDate}</p>
            </div>
          </div>

          {/* Stats */}
          <div className="grid grid-cols-3 gap-4 mt-6 pt-6 border-t border-gray-100">
            <div className="text-center">
              <div className="text-2xl font-bold text-blue-600">{mockUser.totalTasks}</div>
              <div className="text-sm text-gray-500">总任务数</div>
            </div>
            <div className="text-center">
              <div className="text-2xl font-bold text-green-600">{mockUser.completedTasks}</div>
              <div className="text-sm text-gray-500">已完成</div>
            </div>
            <div className="text-center">
              <div className="text-2xl font-bold text-purple-600">{mockUser.streak}</div>
              <div className="text-sm text-gray-500">连续天数</div>
            </div>
          </div>
        </div>

        {/* Menu Items */}
        <div className="bg-white/70 backdrop-blur-sm rounded-2xl shadow-soft border border-white/50 p-6 mb-8">
          <h3 className="text-lg font-semibold text-gray-800 mb-4">设置选项</h3>
          <div className="space-y-3">
            {menuItems.map((item, index) => {
              const Icon = item.icon
              return (
                <button
                  key={index}
                  className="w-full p-4 rounded-xl border border-gray-100 hover:border-gray-200 hover:bg-gray-50 transition-all duration-200 text-left"
                >
                  <div className="flex items-center space-x-4">
                    <div className={`w-10 h-10 rounded-lg flex items-center justify-center ${item.bgColor}`}>
                      <Icon className={`w-5 h-5 ${item.color}`} />
                    </div>
                    <div className="flex-1">
                      <div className="font-medium text-gray-900">{item.title}</div>
                      <div className="text-sm text-gray-500">{item.subtitle}</div>
                    </div>
                    <div className="text-gray-400">
                      <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
                      </svg>
                    </div>
                  </div>
                </button>
              )
            })}
          </div>
        </div>

        {/* Quick Actions */}
        <div className="bg-white/70 backdrop-blur-sm rounded-2xl shadow-soft border border-white/50 p-6">
          <h3 className="text-lg font-semibold text-gray-800 mb-4">快速操作</h3>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <button className="p-4 rounded-xl border border-gray-100 hover:border-gray-200 hover:bg-gray-50 transition-all duration-200 text-left">
              <div className="flex items-center space-x-3">
                <div className="w-10 h-10 bg-blue-100 rounded-lg flex items-center justify-center">
                  <svg className="w-5 h-5 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
                  </svg>
                </div>
                <div>
                  <div className="font-medium text-gray-900">导出数据</div>
                  <div className="text-sm text-gray-500">备份您的任务数据</div>
                </div>
              </div>
            </button>

            <button className="p-4 rounded-xl border border-gray-100 hover:border-gray-200 hover:bg-gray-50 transition-all duration-200 text-left">
              <div className="flex items-center space-x-3">
                <div className="w-10 h-10 bg-green-100 rounded-lg flex items-center justify-center">
                  <svg className="w-5 h-5 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12" />
                  </svg>
                </div>
                <div>
                  <div className="font-medium text-gray-900">导入数据</div>
                  <div className="text-sm text-gray-500">从备份恢复数据</div>
                </div>
              </div>
            </button>
          </div>
        </div>

        {/* Logout Button */}
        <div className="mt-8 text-center">
          <button className="px-8 py-3 bg-red-500 text-white font-medium rounded-xl hover:bg-red-600 transition-all duration-200 flex items-center justify-center mx-auto space-x-2">
            <LogOut className="w-5 h-5" />
            <span>退出登录</span>
          </button>
        </div>
      </div>
    </div>
  )
}

export default ProfilePage
