import React from 'react'
import { Button } from './ui/button'
import { Card, CardContent, CardHeader, CardTitle } from './ui/card'
import { ExclamationTriangleIcon } from '@heroicons/react/24/outline'

interface ConfirmDialogProps {
  isOpen: boolean
  onClose: () => void
  onConfirm: () => void
  title: string
  message: string
  confirmText?: string
  cancelText?: string
  type?: 'warning' | 'danger' | 'info'
}

const ConfirmDialog: React.FC<ConfirmDialogProps> = ({
  isOpen,
  onClose,
  onConfirm,
  title,
  message,
  confirmText = '确定',
  cancelText = '取消',
  type = 'warning',
}) => {
  if (!isOpen) return null

  const getTypeStyles = () => {
    switch (type) {
      case 'danger':
        return {
          iconColor: 'text-red-500',
          iconBg: 'bg-red-100',
          confirmButton: 'bg-red-600 hover:bg-red-700',
        }
      case 'info':
        return {
          iconColor: 'text-blue-500',
          iconBg: 'bg-blue-100',
          confirmButton: 'bg-blue-600 hover:bg-blue-700',
        }
      default: // warning
        return {
          iconColor: 'text-orange-500',
          iconBg: 'bg-orange-100',
          confirmButton: 'bg-orange-600 hover:bg-orange-700',
        }
    }
  }

  const styles = getTypeStyles()

  return (
    <div className="fixed inset-0 z-dialog overflow-hidden">
      {/* 弹窗内容 - 使用相对定位和z-10确保在遮罩之上 */}
      <div className="flex items-center justify-center min-h-screen p-4 relative z-10">
        <Card className="w-full max-w-md mx-auto shadow-xl">
          <CardHeader className="pb-4">
            <div className="flex items-center space-x-3">
              {/* 图标 */}
              <div
                className={`flex items-center justify-center w-10 h-10 rounded-full ${styles.iconBg}`}
              >
                <ExclamationTriangleIcon className={`w-6 h-6 ${styles.iconColor}`} />
              </div>
              <CardTitle className="text-lg font-semibold text-gray-900">{title}</CardTitle>
            </div>
          </CardHeader>

          <CardContent className="pt-0">
            {/* 消息内容 */}
            <div className="mb-6 mt-2">
              <p className="text-gray-600 leading-relaxed">{message}</p>
            </div>

            {/* 按钮组 */}
            <div className="flex space-x-3 justify-end">
              <Button variant="outline" onClick={onClose} className="min-w-20">
                {cancelText}
              </Button>
              <Button
                onClick={() => {
                  onConfirm()
                  onClose()
                }}
                className={`min-w-20 text-white ${styles.confirmButton}`}
              >
                {confirmText}
              </Button>
            </div>
          </CardContent>
        </Card>
      </div>

      {/* 背景遮罩 - 放在内容后面，使用-z-10确保在内容之下 */}
      <div
        className="absolute inset-0 bg-black bg-opacity-50 transition-opacity -z-10"
        onClick={onClose}
      />
    </div>
  )
}

export default ConfirmDialog
