// 装饰器工具

import { withTimeout } from './timeout'

/**
 * 超时装饰器 - 自动为方法添加超时功能
 * @param timeoutMs 超时时间（毫秒），默认10秒
 * @param timeoutMessage 超时错误消息
 */
export function Timeout(timeoutMs: number = 10000, timeoutMessage?: string) {
  return function (target: any, propertyKey: string, descriptor: PropertyDescriptor) {
    const originalMethod = descriptor.value

    descriptor.value = async function (...args: any[]) {
      // 如果原始方法返回的是 Promise，则添加超时
      const result = originalMethod.apply(this, args)
      if (result instanceof Promise) {
        const customMessage = timeoutMessage || `${propertyKey} 请求超时`
        return await withTimeout(result, timeoutMs, customMessage)
      }
      return result
    }

    return descriptor
  }
}

/**
 * 类级别的超时装饰器 - 为类的所有方法自动添加超时功能
 * @param timeoutMs 超时时间（毫秒），默认10秒
 */
export function TimeoutClass(timeoutMs: number = 10000) {
  return function <T extends { new (...args: any[]): object }>(constructor: T) {
    return class extends constructor {
      constructor(...args: any[]) {
        super(...args)

        // 获取类的所有方法
        const methods = Object.getOwnPropertyNames(constructor.prototype)

        methods.forEach((methodName) => {
          if (methodName !== 'constructor') {
            const method = (this as any)[methodName]
            if (typeof method === 'function') {
              // 为每个方法添加超时功能
              ;(this as any)[methodName] = async function (...args: any[]) {
                const result = method.apply(this, args)
                if (result instanceof Promise) {
                  return await withTimeout(result, timeoutMs, `${methodName} 请求超时`)
                }
                return result
              }
            }
          }
        })
      }
    }
  }
}

/**
 * 方法级别的超时装饰器 - 为特定方法添加超时功能
 * @param timeoutMs 超时时间（毫秒），默认10秒
 * @param timeoutMessage 超时错误消息
 */
export function TimeoutMethod(timeoutMs: number = 10000, timeoutMessage?: string) {
  return function (target: any, propertyKey: string, descriptor: PropertyDescriptor) {
    const originalMethod = descriptor.value

    descriptor.value = async function (...args: any[]) {
      const result = originalMethod.apply(this, args)
      if (result instanceof Promise) {
        const customMessage = timeoutMessage || `${propertyKey} 请求超时`
        return await withTimeout(result, timeoutMs, customMessage)
      }
      return result
    }

    return descriptor
  }
}

/**
 * 工厂函数 - 创建自定义超时装饰器
 * @param defaultTimeout 默认超时时间（毫秒）
 * @param defaultMessage 默认超时消息模板
 */
export function createTimeoutDecorator(
  defaultTimeout: number = 10000,
  defaultMessage: string = '{methodName} 请求超时'
) {
  return function (timeoutMs?: number, timeoutMessage?: string) {
    const actualTimeout = timeoutMs || defaultTimeout
    const actualMessage = timeoutMessage || defaultMessage

    return function (target: any, propertyKey: string, descriptor: PropertyDescriptor) {
      const originalMethod = descriptor.value

      descriptor.value = async function (...args: any[]) {
        const result = originalMethod.apply(this, args)
        if (result instanceof Promise) {
          const message = actualMessage.replace('{methodName}', propertyKey)
          return await withTimeout(result, actualTimeout, message)
        }
        return result
      }

      return descriptor
    }
  }
}
