import React from 'react'

interface BadgeProps {
  variant?: 'default' | 'secondary' | 'outline'
  children: React.ReactNode
  className?: string
}

const Badge: React.FC<BadgeProps> = ({ variant = 'default', children, className = '' }) => {
  const baseClasses = 'inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium'

  const variantClasses = {
    default: 'bg-blue-100 text-blue-800',
    secondary: 'bg-gray-100 text-gray-800',
    outline: 'border border-gray-300 bg-transparent text-gray-700',
  }

  const classes = `${baseClasses} ${variantClasses[variant]} ${className}`

  return <span className={classes}>{children}</span>
}

export { Badge }
