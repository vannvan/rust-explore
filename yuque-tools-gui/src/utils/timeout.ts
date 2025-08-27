// 超时工具函数

/**
 * 为异步函数添加超时功能
 * @param promise 要执行的异步函数
 * @param timeoutMs 超时时间（毫秒）
 * @param timeoutMessage 超时错误消息
 * @returns Promise<T> 返回原始结果或超时错误
 */
export function withTimeout<T>(
  promise: Promise<T>,
  timeoutMs: number = 10000, // 默认10秒
  timeoutMessage: string = '请求超时'
): Promise<T> {
  return Promise.race([
    promise,
    new Promise<never>((_, reject) => {
      setTimeout(() => {
        reject(new Error(`${timeoutMessage} (${timeoutMs}ms)`))
      }, timeoutMs)
    }),
  ])
}

/**
 * 超时错误类型
 */
export class TimeoutError extends Error {
  timeoutMs: number
  constructor(message: string, timeoutMs: number) {
    super(message)
    this.name = 'TimeoutError'
    this.timeoutMs = timeoutMs
  }
}

/**
 * 检查错误是否为超时错误
 */
export function isTimeoutError(error: unknown): error is TimeoutError {
  return error instanceof TimeoutError || (error instanceof Error && error.message.includes('超时'))
}
