import React, { useState } from 'react'
import { Button } from './ui/button'
import { Card, CardContent, CardHeader, CardTitle } from './ui/card'
import { Badge } from './ui/badge'
import {
  XMarkIcon,
  DocumentArrowDownIcon,
  CheckCircleIcon,
  ExclamationTriangleIcon,
  ClockIcon,
} from '@heroicons/react/24/outline'

export interface ExportTask {
  id: string
  title: string
  status: 'pending' | 'exporting' | 'completed' | 'failed'
  progress: number
  filePath?: string
  error?: string
  startTime: Date
  endTime?: Date
  // 存储完整的文档信息，用于导出
  docInfo: {
    title: string
    type: string
    uuid: string
    slug: string
    bookSlug: string
    url: string
    docFullPath: string
    // 新增：包含更多 TreeNode 字段，减少手动映射
    level?: number
    child_uuid?: string
    parent_uuid?: string
    visible?: number
    doc_id?: string
    id?: string
    open_window?: number
    prev_uuid?: string
    sibling_uuid?: string
  }
}

interface ExportQueuePanelProps {
  tasks: ExportTask[]
  onClearCompleted: () => void
  onClearAll: () => void
  // 支持自定义宽度
  width?: string
}

const ExportQueuePanel: React.FC<ExportQueuePanelProps> = ({
  tasks,
  onClearCompleted,
  onClearAll,
  width = 'w-80',
}) => {
  const [isCollapsed, setIsCollapsed] = useState(false)

  console.log('=== ExportQueuePanel 渲染 ===')
  console.log('tasks 数量:', tasks.length)

  const pendingTasks = tasks.filter((task) => task.status === 'pending')
  const exportingTasks = tasks.filter((task) => task.status === 'exporting')
  const completedTasks = tasks.filter((task) => task.status === 'completed')
  const failedTasks = tasks.filter((task) => task.status === 'failed')

  const getStatusIcon = (status: ExportTask['status']) => {
    switch (status) {
      case 'pending':
        return <ClockIcon className="w-4 h-4 text-gray-500" />
      case 'exporting':
        return <DocumentArrowDownIcon className="w-4 h-4 text-blue-500 animate-pulse" />
      case 'completed':
        return <CheckCircleIcon className="w-4 h-4 text-green-500" />
      case 'failed':
        return <ExclamationTriangleIcon className="w-4 h-4 text-red-500" />
    }
  }

  const getStatusColor = (status: ExportTask['status']) => {
    switch (status) {
      case 'pending':
        return 'bg-gray-100 text-gray-700'
      case 'exporting':
        return 'bg-blue-100 text-blue-700'
      case 'completed':
        return 'bg-green-100 text-green-700'
      case 'failed':
        return 'bg-red-100 text-red-700'
    }
  }

  const getStatusText = (status: ExportTask['status']) => {
    switch (status) {
      case 'pending':
        return '等待中'
      case 'exporting':
        return '导出中'
      case 'completed':
        return '已完成'
      case 'failed':
        return '失败'
    }
  }

  const formatDuration = (startTime: Date, endTime?: Date) => {
    const end = endTime || new Date()
    const duration = end.getTime() - startTime.getTime()
    const seconds = Math.floor(duration / 1000)

    if (seconds < 60) {
      return `${seconds}秒`
    } else {
      const minutes = Math.floor(seconds / 60)
      const remainingSeconds = seconds % 60
      return `${minutes}分${remainingSeconds}秒`
    }
  }

  return (
    <div className={`fixed ${isCollapsed ? 'bottom-0' : 'bottom-4'} right-6 z-[1001]`}>
      <div
        className="absolute top-0 z-40 right-0 flex flex-col items-center justify-between cursor-pointer bg-blue-400 text-white rounded-2 gap-1 px-2"
        onClick={() => setIsCollapsed(!isCollapsed)}
      >
        <span>{isCollapsed ? '▼' : 'x'}</span>
        {/* {isCollapsed && <span className="text-xs">{tasks.length}</span>} */}
      </div>
      <Card
        className={`${width} shadow-xl transition-all duration-300 flex flex-col ${
          isCollapsed ? 'h-16' : 'h-96'
        } ${isCollapsed ? 'opacity-0' : 'opacity-100'}`}
      >
        {!isCollapsed && (
          <CardContent className="pt-8 flex flex-col flex-1 min-h-0">
            {/* 任务统计 */}
            <div className="grid grid-cols-4 gap-2 mb-4 text-xs flex-shrink-0">
              <div className="text-center">
                <div className="font-medium text-gray-600 mb-1">等待</div>
                <Badge variant="outline" className="text-xs">
                  {pendingTasks.length}
                </Badge>
              </div>
              <div className="text-center">
                <div className="font-medium text-blue-600 mb-1">导出中</div>
                <Badge variant="outline" className="text-xs bg-blue-50">
                  {exportingTasks.length}
                </Badge>
              </div>
              <div className="text-center">
                <div className="font-medium text-green-600 mb-1">已完成</div>
                <Badge variant="outline" className="text-xs bg-green-50">
                  {completedTasks.length}
                </Badge>
              </div>
              <div className="text-center">
                <div className="font-medium text-red-600 mb-1">失败</div>
                <Badge variant="outline" className="text-xs bg-red-50">
                  {failedTasks.length}
                </Badge>
              </div>
            </div>

            {/* 任务列表 - 可滚动区域 */}
            <div className="flex-1 overflow-y-auto min-h-0 space-y-2 scrollbar-thin scrollbar-thumb-gray-300 scrollbar-track-gray-100">
              {tasks.map((task) => (
                <div
                  key={task.id}
                  className="flex items-center space-x-2 p-2 rounded-lg border border-gray-200 hover:border-gray-300 transition-colors"
                >
                  {getStatusIcon(task.status)}

                  <div className="flex-1 min-w-0">
                    <div className="flex items-center justify-between">
                      <span className="text-sm font-medium truncate" title={task.title}>
                        {task.title}
                      </span>
                      <Badge className={`text-xs ${getStatusColor(task.status)}`}>
                        {getStatusText(task.status)}
                      </Badge>
                    </div>

                    {task.status === 'exporting' && (
                      <div className="mt-1">
                        <div className="w-full bg-gray-200 rounded-full h-1.5">
                          <div
                            className="bg-blue-600 h-1.5 rounded-full transition-all duration-300"
                            style={{ width: `${task.progress}%` }}
                          ></div>
                        </div>
                        <span className="text-xs text-gray-500">{task.progress}%</span>
                      </div>
                    )}

                    {task.status === 'completed' && task.filePath && (
                      <div className="text-xs text-gray-500 truncate mt-1" title={task.filePath}>
                        保存至: {task.filePath}
                      </div>
                    )}

                    {task.status === 'failed' && task.error && (
                      <div className="text-xs text-red-500 truncate mt-1" title={task.error}>
                        错误: {task.error}
                      </div>
                    )}

                    <div className="text-xs text-gray-400 mt-1">
                      {formatDuration(task.startTime, task.endTime)}
                    </div>
                  </div>
                </div>
              ))}

              {tasks.length === 0 && (
                <div className="text-center py-8 text-gray-500">
                  <DocumentArrowDownIcon className="w-8 h-8 mx-auto mb-2 text-gray-300" />
                  <p className="text-sm">暂无导出任务</p>
                </div>
              )}
            </div>

            {/* 操作按钮 */}
            <div className="mt-4 pt-3 border-t border-gray-200 flex-shrink-0 gap-2 flex">
              {/* 清空所有任务按钮 - 始终显示 */}
              <Button
                variant="outline"
                size="sm"
                onClick={onClearAll}
                className="w-full text-xs text-red-600 border-red-200 hover:bg-red-50 hover:border-red-300"
                disabled={tasks.length === 0}
              >
                清空所有任务 ({tasks.length})
              </Button>

              {/* 清除已完成任务按钮 - 只在有已完成任务时显示 */}
              {/* {completedTasks.length > 0 && ( */}
              <Button
                variant="outline"
                size="sm"
                onClick={onClearCompleted}
                className="w-full text-xs"
              >
                清除已完成任务 ({completedTasks.length})
              </Button>
              {/* )} */}
            </div>
          </CardContent>
        )}
      </Card>
    </div>
  )
}

export default ExportQueuePanel
