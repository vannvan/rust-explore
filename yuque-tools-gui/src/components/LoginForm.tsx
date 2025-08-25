import React, { useState } from 'react'

interface LoginFormProps {
  onLogin: (account: YuqueAccount) => Promise<void>
  isLoading: boolean
}

const LoginForm: React.FC<LoginFormProps> = ({ onLogin, isLoading }) => {
  const [formData, setFormData] = useState<YuqueAccount>({
    username: '',
    password: '',
  })
  const [errors, setErrors] = useState<Partial<YuqueAccount>>({})

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target
    setFormData((prev) => ({
      ...prev,
      [name]: value,
    }))

    // 清除对应字段的错误
    if (errors[name as keyof YuqueAccount]) {
      setErrors((prev) => ({
        ...prev,
        [name]: undefined,
      }))
    }
  }

  const validateForm = (): boolean => {
    const newErrors: Partial<YuqueAccount> = {}

    if (!formData.username.trim()) {
      newErrors.username = '用户名不能为空'
    }

    if (!formData.password.trim()) {
      newErrors.password = '密码不能为空'
    }

    setErrors(newErrors)
    return Object.keys(newErrors).length === 0
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()

    if (!validateForm()) {
      return
    }

    try {
      await onLogin(formData)
    } catch (error) {
      console.error('登录失败:', error)
    }
  }

  return (
    <div className="w-full h-[455px] mx-auto">
      <div className="bg-white h-full rounded-lg shadow-lg p-6">
        <div className="text-center mb-4">
          <h2 className="text-xl font-bold text-gray-900 mb-2">登录语雀</h2>
          <p className="text-sm text-gray-600">请输入您的语雀账号信息</p>
        </div>

        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label htmlFor="username" className="block text-sm font-medium text-gray-700 mb-2">
              用户名/手机号
            </label>
            <input
              type="text"
              id="username"
              name="username"
              value={formData.username}
              onChange={handleInputChange}
              className={`w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 ${
                errors.username ? 'border-red-500' : 'border-gray-300'
              }`}
              placeholder="请输入用户名或手机号"
              disabled={isLoading}
            />
            {errors.username && <p className="mt-1 text-sm text-red-600">{errors.username}</p>}
          </div>

          <div>
            <label htmlFor="password" className="block text-sm font-medium text-gray-700 mb-2">
              密码
            </label>
            <input
              type="password"
              id="password"
              name="password"
              value={formData.password}
              onChange={handleInputChange}
              className={`w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 ${
                errors.password ? 'border-red-500' : 'border-gray-300'
              }`}
              placeholder="请输入密码"
              disabled={isLoading}
            />
            {errors.password && <p className="mt-1 text-sm text-red-600">{errors.password}</p>}
          </div>

          <button
            type="submit"
            disabled={isLoading}
            className={`w-full py-2 px-4 rounded-lg font-medium text-white transition-colors duration-200 ${
              isLoading
                ? 'bg-gray-400 cursor-not-allowed'
                : 'bg-blue-600 hover:bg-blue-700 focus:ring-2 focus:ring-blue-500 focus:ring-offset-2'
            }`}
          >
            {isLoading ? (
              <div className="flex items-center justify-center">
                <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white mr-2"></div>
                登录中...
              </div>
            ) : (
              '登录'
            )}
          </button>
        </form>

        <div className="mt-4 text-center">
          <p className="text-xs text-gray-500">登录后即可使用语雀工具的各项功能</p>
        </div>
      </div>
    </div>
  )
}

export default LoginForm
