import React, { useEffect, useRef } from 'react'

interface PopupProps {
  isOpen: boolean
  onClose: () => void
  children: React.ReactNode
  position?: 'top' | 'bottom' | 'left' | 'right'
  className?: string
}

const Popup: React.FC<PopupProps> = ({
  isOpen,
  onClose,
  children,
  position = 'bottom',
  className = '',
}) => {
  const popupRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (popupRef.current && !popupRef.current.contains(event.target as Node)) {
        onClose()
      }
    }

    if (isOpen) {
      document.addEventListener('mousedown', handleClickOutside)
    }

    return () => {
      document.removeEventListener('mousedown', handleClickOutside)
    }
  }, [isOpen, onClose])

  useEffect(() => {
    const handleEscape = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        onClose()
      }
    }

    if (isOpen) {
      document.addEventListener('keydown', handleEscape)
    }

    return () => {
      document.removeEventListener('keydown', handleEscape)
    }
  }, [isOpen, onClose])

  if (!isOpen) return null

  const getPositionClasses = () => {
    switch (position) {
      case 'top':
        return 'bottom-full right-0 mb-2'
      case 'bottom':
        return 'top-full right-0 mt-2'
      case 'left':
        return 'top-0 right-full mr-2'
      case 'right':
        return 'top-0 left-full ml-2'
      default:
        return 'top-full right-0 mt-2'
    }
  }

  return (
    <div className="relative">
      <div
        ref={popupRef}
        className={`absolute z-50 bg-white rounded-lg shadow-xl border border-gray-200 min-w-64 ${getPositionClasses()} ${className}`}
      >
        {children}
      </div>
    </div>
  )
}

export default Popup
