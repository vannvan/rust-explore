import { Home, Calendar, User } from 'lucide-react'
import { useLocation, useNavigate } from 'react-router-dom'

const BottomNavigation = () => {
  const location = useLocation()
  const navigate = useNavigate()

  const navItems = [
    {
      path: '/',
      icon: Home,
      label: '首页',
      active: location.pathname === '/',
    },
    {
      path: '/records',
      icon: Calendar,
      label: '代办记录',
      active: location.pathname === '/records',
    },
    {
      path: '/profile',
      icon: User,
      label: '个人中心',
      active: location.pathname === '/profile',
    },
  ]

  return (
    <div className="fixed bottom-0 left-0 right-0 bg-white/90 backdrop-blur-md border-t border-gray-200/50 z-50">
      <div className="flex items-center justify-around py-2">
        {navItems.map((item) => {
          const Icon = item.icon
          return (
            <button
              key={item.path}
              onClick={() => navigate(item.path)}
              className={`flex flex-col items-center justify-center py-2 px-4 rounded-lg transition-all duration-200 ${
                item.active
                  ? 'text-blue-600 bg-blue-50'
                  : 'text-gray-500 hover:text-blue-500 hover:bg-blue-50/50'
              }`}
            >
              <Icon
                size={20}
                className={`mb-1 ${item.active ? 'text-blue-600' : 'text-gray-500'}`}
              />
              <span className="text-xs font-medium">{item.label}</span>
            </button>
          )
        })}
      </div>
    </div>
  )
}

export default BottomNavigation
