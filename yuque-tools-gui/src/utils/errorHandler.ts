// 错误处理工具

import { isTimeoutError } from './timeout'

/**
 * 处理 API 错误，返回用户友好的错误消息
 * @param error 错误对象
 * @param defaultMessage 默认错误消息
 * @returns 用户友好的错误消息
 */
export function handleApiError(error: unknown, defaultMessage: string = '操作失败'): string {
  if (isTimeoutError(error)) {
    return '请求超时，请检查网络连接后重试'
  }

  if (error instanceof Error) {
    // 如果是网络相关错误
    if (error.message.includes('NetworkError') || error.message.includes('fetch')) {
      return '网络连接失败，请检查网络设置'
    }

    // 如果是权限相关错误
    if (error.message.includes('permission') || error.message.includes('unauthorized')) {
      return '权限不足，请重新登录'
    }

    // 如果是服务器错误
    if (error.message.includes('500') || error.message.includes('Internal Server Error')) {
      return '服务器内部错误，请稍后重试'
    }

    // 如果是超时错误（通过消息内容判断）
    if (error.message.includes('超时') || error.message.includes('timeout')) {
      return '请求超时，请检查网络连接后重试'
    }

    return error.message || defaultMessage
  }

  return String(error) || defaultMessage
}

/**
 * 检查是否为网络相关错误
 */
export function isNetworkError(error: unknown): boolean {
  if (error instanceof Error) {
    const message = error.message.toLowerCase()
    return (
      message.includes('network') ||
      message.includes('fetch') ||
      message.includes('timeout') ||
      message.includes('超时') ||
      message.includes('连接')
    )
  }
  return false
}

/**
 * 检查是否为权限相关错误
 */
export function isPermissionError(error: unknown): boolean {
  if (error instanceof Error) {
    const message = error.message.toLowerCase()
    return (
      message.includes('permission') ||
      message.includes('unauthorized') ||
      message.includes('forbidden') ||
      message.includes('权限') ||
      message.includes('未授权')
    )
  }
  return false
}

/**
 * 获取错误类型，用于决定显示什么类型的消息
 */
export function getErrorType(
  error: unknown
): 'timeout' | 'network' | 'permission' | 'server' | 'unknown' {
  if (isTimeoutError(error)) {
    return 'timeout'
  }

  if (isNetworkError(error)) {
    return 'network'
  }

  if (isPermissionError(error)) {
    return 'permission'
  }

  if (error instanceof Error && error.message.includes('500')) {
    return 'server'
  }

  return 'unknown'
}
