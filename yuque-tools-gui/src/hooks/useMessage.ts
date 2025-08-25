import { useState, useCallback } from 'react'
import type { MessageType } from '../components/Message'

export interface MessageItem {
  id: string
  type: MessageType
  message: string
  duration?: number
}

export const useMessage = () => {
  const [messages, setMessages] = useState<MessageItem[]>([])

  const addMessage = useCallback((type: MessageType, message: string, duration = 3000) => {
    const id = `${Date.now()}-${Math.random()}`
    const newMessage: MessageItem = { id, type, message, duration }

    setMessages((prev) => [...prev, newMessage])

    // 如果设置了自动消失，则自动移除
    if (duration > 0) {
      setTimeout(() => {
        removeMessage(id)
      }, duration)
    }

    return id
  }, [])

  const removeMessage = useCallback((id: string) => {
    setMessages((prev) => prev.filter((msg) => msg.id !== id))
  }, [])

  const clearMessages = useCallback(() => {
    setMessages([])
  }, [])

  // 便捷方法
  const success = useCallback(
    (message: string, duration?: number) => {
      return addMessage('success', message, duration)
    },
    [addMessage]
  )

  const error = useCallback(
    (message: string, duration?: number) => {
      return addMessage('error', message, duration)
    },
    [addMessage]
  )

  const warning = useCallback(
    (message: string, duration?: number) => {
      return addMessage('warning', message, duration)
    },
    [addMessage]
  )

  const info = useCallback(
    (message: string, duration?: number) => {
      return addMessage('info', message, duration)
    },
    [addMessage]
  )

  return {
    messages,
    addMessage,
    removeMessage,
    clearMessages,
    success,
    error,
    warning,
    info,
  }
}
