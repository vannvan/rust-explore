import React from 'react'
import Message from './Message'
import type { MessageItem } from '../hooks/useMessage'

interface MessageContainerProps {
  messages: MessageItem[]
  onRemove: (id: string) => void
}

const MessageContainer: React.FC<MessageContainerProps> = ({ messages, onRemove }) => {
  if (messages.length === 0) return null

  return (
    <div className="fixed top-4 right-4 z-[9999] space-y-2">
      {messages.map((message) => (
        <Message
          key={message.id}
          type={message.type}
          message={message.message}
          duration={message.duration}
          onClose={() => onRemove(message.id)}
        />
      ))}
    </div>
  )
}

export default MessageContainer
