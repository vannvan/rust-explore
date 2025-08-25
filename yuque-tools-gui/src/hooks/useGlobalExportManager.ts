import { useState, useCallback, useRef, useEffect } from 'react'
import { tauriApi } from '../services/tauriApi'
import type { ExportTask } from '../components/ExportQueuePanel'

export const useGlobalExportManager = () => {
  const [tasks, setTasks] = useState<ExportTask[]>([])
  const [isQueueVisible, setIsQueueVisible] = useState(false)
  const taskQueueRef = useRef<ExportTask[]>([])
  const isProcessingRef = useRef(false)
  const docMapRef = useRef<Map<string, DocItem>>(new Map()) // 存储taskId到DocItem的映射

  // 每秒更新一次任务状态
  useEffect(() => {
    const interval = setInterval(() => {
      setTasks((prevTasks) => [...prevTasks])
    }, 1000)

    return () => clearInterval(interval)
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
        })

        // 这里需要从调用方传入 bookSlug，暂时使用默认值
        const bookSlug = doc.bookSlug || 'default'
        console.log('调用tauriApi.exportDocument，bookSlug:', bookSlug)
        const result = await tauriApi.exportDocument(doc, bookSlug)
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
          // 显示成功消息
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
          // 显示失败消息
          console.error(`导出失败: ${task.title} - ${result.error}`)
        }
      } catch (error) {
        console.error('导出任务执行失败:', error)
        setTasks((prevTasks) =>
          prevTasks.map((t) =>
            t.id === task.id
              ? {
                  ...t,
                  status: 'failed' as const,
                  error: error instanceof Error ? error.message : '未知错误',
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

  // 添加导出任务（需要 bookSlug 参数）
  const addExportTask = useCallback(
    (doc: DocItem, bookSlug: string) => {
      console.log('=== addExportTask 被调用 ===')
      console.log('接收到的文档:', {
        title: doc.title,
        type: doc.type,
        uuid: doc.uuid,
        slug: doc.slug,
      })
      console.log('当前bookSlug:', bookSlug)

      const task: ExportTask = {
        id: `${doc.uuid}-${Date.now()}`,
        title: doc.title,
        status: 'pending',
        progress: 0,
        startTime: new Date(),
        docInfo: {
          title: doc.title,
          type: doc.type,
          uuid: doc.uuid,
          slug: doc.slug || doc.uuid, // 如果没有slug，使用uuid作为fallback
          bookSlug: bookSlug,
          url: doc.url,
        },
      }

      console.log('创建的任务:', task)

      // 存储DocItem的映射，并添加 bookSlug 信息
      const docWithBookSlug = { ...doc, bookSlug }
      docMapRef.current.set(task.id, docWithBookSlug)
      console.log('DocItem已存储到映射中')

      setTasks((prevTasks) => [...prevTasks, task])
      taskQueueRef.current.push(task)
      setIsQueueVisible(true)

      console.log('开始处理队列...')
      // 开始处理队列
      processQueue()
    },
    [processQueue]
  )

  // 批量添加导出任务
  const addBatchExportTasks = useCallback(
    (docs: DocItem[], bookSlug: string) => {
      const newTasks: ExportTask[] = docs.map((doc) => ({
        id: `${doc.uuid}-${Date.now()}-${Math.random()}`,
        title: doc.title,
        status: 'pending',
        progress: 0,
        startTime: new Date(),
        docInfo: {
          title: doc.title,
          type: doc.type,
          uuid: doc.uuid,
          slug: doc.slug || doc.uuid, // 如果没有slug，使用uuid作为fallback
          bookSlug: bookSlug,
          url: doc.url,
        },
      }))

      // 存储DocItem的映射，并添加 bookSlug 信息
      newTasks.forEach((task, index) => {
        const docWithBookSlug = { ...docs[index], bookSlug }
        docMapRef.current.set(task.id, docWithBookSlug)
      })

      setTasks((prevTasks) => [...prevTasks, ...newTasks])
      taskQueueRef.current.push(...newTasks)
      setIsQueueVisible(true)

      // 开始处理队列
      processQueue()
    },
    [processQueue]
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
