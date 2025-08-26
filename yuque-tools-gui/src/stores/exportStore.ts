import { create } from 'zustand'
import type { ExportTask } from '../components/ExportQueuePanel'
import { tauriApi } from '../services/tauriApi'

interface ExportStore {
  // 状态
  tasks: ExportTask[]
  isProcessing: boolean

  // 操作
  addTask: (task: ExportTask) => void
  updateTask: (id: string, updates: Partial<ExportTask>) => void
  removeTask: (id: string) => void
  clearCompletedTasks: () => void
  processQueue: () => void

  // 新增：检查函数
  isDocumentInQueue: (docUuid: string) => boolean
  getDocumentsInQueue: () => string[] // 返回所有在队列中的文档UUID

  // 新增：获取单个文档的任务状态
  getDocumentTaskStatus: (docUuid: string) => ExportTask['status'] | null

  // 新增：清空所有任务
  clearAllTasks: () => void
}

export const useExportStore = create<ExportStore>((set, get) => ({
  // 初始状态
  tasks: [],
  isProcessing: false,

  // 添加任务
  addTask: (task: ExportTask) => {
    set((state) => ({
      tasks: [...state.tasks, task],
    }))

    // 添加任务后自动开始处理队列
    get().processQueue()
  },

  // 更新任务
  updateTask: (id: string, updates: Partial<ExportTask>) => {
    set((state) => ({
      tasks: state.tasks.map((task) => (task.id === id ? { ...task, ...updates } : task)),
    }))
  },

  // 移除任务
  removeTask: (id: string) => {
    set((state) => ({
      tasks: state.tasks.filter((task) => task.id !== id),
    }))
  },

  // 清除已完成的任务
  clearCompletedTasks: () => {
    set((state) => ({
      tasks: state.tasks.filter((task) => task.status !== 'completed'),
    }))
  },

  // 处理任务队列
  processQueue: async () => {
    const state = get()
    if (state.isProcessing) return

    set({ isProcessing: true })
    console.log('开始处理导出队列...')

    // 持续处理队列，直到没有待处理的任务
    let hasMoreTasks = true
    while (hasMoreTasks) {
      const currentState = get()
      const pendingTasks = currentState.tasks.filter((task) => task.status === 'pending')

      if (pendingTasks.length === 0) {
        console.log('没有待处理的任务，队列处理完成')
        hasMoreTasks = false
        break
      }

      console.log(`当前有 ${pendingTasks.length} 个待处理任务，开始处理第一个任务`)

      // 获取第一个待处理的任务
      const pendingTask = pendingTasks[0]
      console.log(`开始处理任务: ${pendingTask.title} (ID: ${pendingTask.id})`)

      // 更新任务状态为导出中
      get().updateTask(pendingTask.id, { status: 'exporting', progress: 0 })

      try {
        // 模拟进度更新
        const progressInterval = setInterval(() => {
          const currentTask = get().tasks.find((t) => t.id === pendingTask.id)
          if (currentTask && currentTask.status === 'exporting') {
            const newProgress = Math.min(currentTask.progress + Math.random() * 20, 90)
            get().updateTask(pendingTask.id, { progress: newProgress })
          }
        }, 200)

        // 调用导出API
        console.log('准备调用导出API...')
        console.log('使用任务中的完整文档信息:', pendingTask.docInfo)

        const doc: DocItem = {
          title: pendingTask.docInfo.title,
          type: pendingTask.docInfo.type,
          uuid: pendingTask.docInfo.uuid,
          child_uuid: '',
          parent_uuid: '',
          visible: 1,
          url: pendingTask.docInfo.url,
          slug: pendingTask.docInfo.slug,
          level: 0,
          docFullPath: pendingTask.docInfo.docFullPath, // 添加 docFullPath 字段
        }

        const bookSlug = pendingTask.docInfo.bookSlug
        console.log('调用tauriApi.exportDocument，使用完整信息:', {
          bookSlug: bookSlug,
          url: doc.url,
          docFullPath: doc.docFullPath, // 添加 docFullPath 日志
        })

        const result = await tauriApi.exportDocument(doc, bookSlug)
        console.log('导出API调用结果:', result)

        clearInterval(progressInterval)

        if (result.success) {
          // 导出成功
          get().updateTask(pendingTask.id, {
            status: 'completed',
            progress: 100,
            filePath: result.filePath,
            endTime: new Date(),
          })
          console.log(`导出成功: ${pendingTask.title}`)
        } else {
          // 导出失败 - 标记为失败但继续处理下一个任务
          get().updateTask(pendingTask.id, {
            status: 'failed',
            error: result.error || '导出失败',
            endTime: new Date(),
          })
          console.log(`导出失败: ${pendingTask.title} - ${result.error}，继续处理下一个任务`)
        }
      } catch (error) {
        // 发生异常 - 标记为失败但继续处理下一个任务
        console.error(`导出任务执行失败: ${pendingTask.title}`, error)
        get().updateTask(pendingTask.id, {
          status: 'failed',
          error: error instanceof Error ? error.message : '未知错误',
          endTime: new Date(),
        })
        console.log(`任务 ${pendingTask.title} 失败，继续处理下一个任务`)
      }

      // 延迟一下，避免API调用过于频繁
      await new Promise((resolve) => setTimeout(resolve, 500))
    }

    console.log('所有任务处理完成')
    set({ isProcessing: false })
  },

  // 检查文档是否在队列中
  isDocumentInQueue: (docUuid: string) => {
    const state = get()
    return state.tasks.some(
      (task) =>
        task.docInfo.uuid === docUuid && (task.status === 'pending' || task.status === 'exporting')
    )
  },

  // 获取所有在队列中的文档UUID
  getDocumentsInQueue: () => {
    const state = get()
    return state.tasks
      .filter((task) => task.status === 'pending' || task.status === 'exporting')
      .map((task) => task.docInfo.uuid)
  },

  // 获取单个文档的任务状态
  getDocumentTaskStatus: (docUuid: string) => {
    const state = get()
    const task = state.tasks.find((task) => task.docInfo.uuid === docUuid)
    return task ? task.status : null
  },

  // 清空所有任务
  clearAllTasks: () => {
    set({ tasks: [], isProcessing: false })
  },
}))
