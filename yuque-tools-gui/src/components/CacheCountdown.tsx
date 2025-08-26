import React, { useState, useEffect } from 'react'

interface CacheCountdownProps {
  className?: string
}

const CacheCountdown: React.FC<CacheCountdownProps> = ({ className = '' }) => {
  const [remainingTime, setRemainingTime] = useState<number>(0)
  const [isExpired, setIsExpired] = useState<boolean>(false)

  useEffect(() => {
    // 缓存过期时间：30分钟
    const CACHE_DURATION = 30 * 60 * 1000 // 30分钟，单位：毫秒

    // 获取上次缓存时间（从localStorage）
    const getLastCacheTime = (): number => {
      const lastCacheTime = localStorage.getItem('lastCacheTime')
      return lastCacheTime ? parseInt(lastCacheTime, 10) : 0
    }

    // 计算剩余时间
    const calculateRemainingTime = (): number => {
      const lastCacheTime = getLastCacheTime()
      if (lastCacheTime === 0) return 0

      const now = Date.now()
      const elapsed = now - lastCacheTime
      const remaining = Math.max(0, CACHE_DURATION - elapsed)

      return remaining
    }

    // 更新剩余时间
    const updateRemainingTime = () => {
      const remaining = calculateRemainingTime()
      setRemainingTime(remaining)
      setIsExpired(remaining === 0)
    }

    // 初始化
    updateRemainingTime()

    // 每秒更新一次
    const interval = setInterval(updateRemainingTime, 1000)

    return () => clearInterval(interval)
  }, [])

  // 格式化时间显示
  const formatTime = (milliseconds: number): string => {
    if (milliseconds === 0) return '已过期'

    const totalSeconds = Math.floor(milliseconds / 1000)
    const minutes = Math.floor(totalSeconds / 60)
    const seconds = totalSeconds % 60

    return `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`
  }

  // 获取状态颜色
  const getStatusColor = (): string => {
    if (isExpired) return 'text-red-500'
    if (remainingTime < 5 * 60 * 1000) return 'text-yellow-500' // 少于5分钟显示黄色
    return 'text-green-500'
  }

  // 获取状态图标
  const getStatusIcon = () => {
    if (isExpired) {
      return (
        <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
          <path
            fillRule="evenodd"
            d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
            clipRule="evenodd"
          />
        </svg>
      )
    }

    if (remainingTime < 5 * 60 * 1000) {
      return (
        <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
          <path
            fillRule="evenodd"
            d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z"
            clipRule="evenodd"
          />
        </svg>
      )
    }

    return (
      <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
        <path
          fillRule="evenodd"
          d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
          clipRule="evenodd"
        />
      </svg>
    )
  }

  // 如果没有缓存时间，不显示组件
  if (remainingTime === 0 && !isExpired) {
    return null
  }

  return (
    <div
      className={`flex items-center space-x-2 px-3 py-2 bg-white rounded-lg shadow-sm border ${className}`}
    >
      <div className={`${getStatusColor()}`}>{getStatusIcon()}</div>
      <div className="text-sm">
        <span className="text-gray-600">缓存剩余：</span>
        <span className={`font-mono font-medium ${getStatusColor()}`}>
          {formatTime(remainingTime)}
        </span>
      </div>
    </div>
  )
}

export default CacheCountdown
