import { useState, useCallback, useRef, useEffect } from 'react'
import { tauriApi } from '../services/tauriApi'
import type { ExportTask } from '../components/ExportQueuePanel'
import type { TreeNode } from '../types/yuque'

export const useExportManager = (bookSlug: string) => {
  const [tasks, setTasks] = useState<ExportTask[]>([])
  const [isQueueVisible, setIsQueueVisible] = useState(false)
  const taskQueueRef = useRef<ExportTask[]>([])
  const isProcessingRef = useRef(false)
  const docMapRef = useRef<Map<string, TreeNode>>(new Map()) // 存储taskId到TreeNode的映射

  // 每秒更新一次任务状态
  useEffect(() => {
    const interval = setInterval(() => {
      setTasks((prevTasks) => [...prevTasks])
    }, 1000)

    return () => clearInterval(interval)
  }, [])

  // 创建导出任务的公共函数
  const createExportTask = useCallback(
    (doc: TreeNode, isBatch: boolean = false): ExportTask => {
      const timestamp = Date.now()
      const randomId = isBatch ? Math.random() : 0
      const taskId = isBatch ? `${doc.uuid}-${timestamp}-${randomId}` : `${doc.uuid}-${timestamp}`

      return {
        id: taskId,
        title: doc.title,
        status: 'pending',
        progress: 0,
        startTime: new Date(),
        docInfo: {
          ...doc,
          bookSlug: bookSlug,
          docFullPath: doc.docFullPath || doc.title,
          level: doc.level,
          slug: doc.slug ?? '', // fix: ensure slug is always a string
        },
      }
    },
    [bookSlug]
  )

  // 添加任务到队列的公共函数
  const addTasksToQueue = useCallback((newTasks: ExportTask[], docs: TreeNode[]) => {
    // 存储 TreeNode 的映射
    newTasks.forEach((task, index) => {
      docMapRef.current.set(task.id, docs[index])
    })

    setTasks((prevTasks) => [...prevTasks, ...newTasks])
    taskQueueRef.current.push(...newTasks)
    setIsQueueVisible(true)

    // 开始处理队列
    processQueue()
  }, [])

  // 处理任务队列
  const processQueue = useCallback(async () => {
    if (isProcessingRef.current || taskQueueRef.current.length === 0) {
      console.log('队列处理条件不满足:', {
        isProcessing: isProcessingRef.current,
        queueLength: taskQueueRef.current.length,
      })
      return
    }

    isProcessingRef.current = true
    console.log('开始处理导出队列，当前队列长度:', taskQueueRef.current.length)

    while (taskQueueRef.current.length > 0) {
      const task = taskQueueRef.current.shift()!
      console.log('处理任务:', task.title, '任务ID:', task.id)

      // 更新任务状态为导出中
      setTasks((prevTasks) =>
        prevTasks.map((t) =>
          t.id === task.id ? { ...t, status: 'exporting' as const, progress: 0 } : t
        )
      )

      try {
        // 模拟进度更新
        const progressInterval = setInterval(() => {
          setTasks((prevTasks) =>
            prevTasks.map((t) =>
              t.id === task.id
                ? { ...t, progress: Math.min(t.progress + Math.random() * 20, 90) }
                : t
            )
          )
        }, 200)

        // 调用导出API
        console.log('准备调用导出API...')
        const doc = docMapRef.current.get(task.id)
        if (!doc) {
          console.error('找不到对应的文档信息，任务ID:', task.id)
          throw new Error('找不到对应的文档信息')
        }
        console.log('找到文档信息:', {
          title: doc.title,
          type: doc.type,
          uuid: doc.uuid,
          slug: doc.slug,
          docFullPath: doc.docFullPath,
        })
        console.log('任务中的 docInfo:', task.docInfo)
        console.log('调用tauriApi.exportDocument，bookSlug:', bookSlug)

        // 使用 task.docInfo 中的信息，确保包含 docFullPath
        const result = await tauriApi.exportDocument(
          {
            ...doc,
            docFullPath: task.docInfo.docFullPath,
          },
          bookSlug
        )
        console.log('导出API调用结果:', result)

        clearInterval(progressInterval)

        if (result.success) {
          // 导出成功
          setTasks((prevTasks) =>
            prevTasks.map((t) =>
              t.id === task.id
                ? {
                    ...t,
                    status: 'completed' as const,
                    progress: 100,
                    filePath: result.filePath,
                    endTime: new Date(),
                  }
                : t
            )
          )
          console.log(`导出成功: ${task.title}`)
        } else {
          // 导出失败
          setTasks((prevTasks) =>
            prevTasks.map((t) =>
              t.id === task.id
                ? {
                    ...t,
                    status: 'failed' as const,
                    error: result.error || '导出失败',
                    endTime: new Date(),
                  }
                : t
            )
          )
          console.error(`导出失败: ${task.title} - ${result.error}`)
        }
      } catch (error) {
        // 导出异常
        setTasks((prevTasks) =>
          prevTasks.map((t) =>
            t.id === task.id
              ? {
                  ...t,
                  status: 'failed' as const,
                  error: error instanceof Error ? error.message : '导出异常',
                  endTime: new Date(),
                }
              : t
          )
        )
      }

      // 延迟一下，避免API调用过于频繁
      await new Promise((resolve) => setTimeout(resolve, 500))
    }

    isProcessingRef.current = false
  }, [])

  // 添加导出任务（单个）
  const addExportTask = useCallback(
    (doc: TreeNode) => {
      console.log('=== addExportTask 被调用 ===')
      console.log('接收到的文档:', {
        title: doc.title,
        type: doc.type,
        uuid: doc.uuid,
        slug: doc.slug,
        docFullPath: doc.docFullPath,
      })
      console.log('当前bookSlug:', bookSlug)

      const task = createExportTask(doc, false)
      console.log('创建的任务:', task)

      // 使用公共函数添加任务
      addTasksToQueue([task], [doc])
    },
    [createExportTask, addTasksToQueue]
  )

  // 批量添加导出任务
  const addBatchExportTasks = useCallback(
    (docs: TreeNode[]) => {
      console.log('=== addBatchExportTasks 被调用 ===')
      console.log('接收到的文档数量:', docs.length)
      console.log('当前bookSlug:', bookSlug)

      const newTasks = docs.map((doc) => createExportTask(doc, true))
      console.log('创建的批量任务数量:', newTasks.length)

      // 使用公共函数添加任务
      addTasksToQueue(newTasks, docs)
    },
    [createExportTask, addTasksToQueue]
  )

  // 清除已完成的任务
  const clearCompletedTasks = useCallback(() => {
    setTasks((prevTasks) => prevTasks.filter((task) => task.status !== 'completed'))
  }, [])

  // 关闭队列面板
  const closeQueue = useCallback(() => {
    setIsQueueVisible(false)
  }, [])

  // 显示队列面板
  const showQueue = useCallback(() => {
    console.log('=== showQueue 被调用 ===')
    console.log('设置 isQueueVisible 为 true')
    setIsQueueVisible(true)
  }, [])

  return {
    tasks,
    isQueueVisible,
    addExportTask,
    addBatchExportTasks,
    clearCompletedTasks,
    closeQueue,
    showQueue,
  }
}
