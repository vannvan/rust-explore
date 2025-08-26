import React, { useMemo, useState, useCallback, useEffect } from 'react'
import { useExportStore } from '../stores/exportStore'
import { useMessage } from '../hooks/useMessage'
import type { ExportTask } from '../components/ExportQueuePanel'

// 使用全局的 DocItem 类型，删除本地接口定义

interface TreeNode extends DocItem {
  children: TreeNode[]
  level: number
  docFullPath: string
}

interface BookManageDrawerProps {
  isOpen: boolean
  onClose: () => void
  bookName: string
  bookSlug: string // 添加知识库slug
  docs: TreeNode[] // 现在传入的是已经构建好的树形结构
}

// 递归渲染树形节点的组件 - 提取到外部，避免死循环
const TreeNodeComponent: React.FC<{
  node: TreeNode
  expandedNodes: Set<string>
  setExpandedNodes: React.Dispatch<React.SetStateAction<Set<string>>>
  onExportDocument: (doc: TreeNode) => void
  isDocumentInQueue: (docUuid: string) => boolean
  getDocumentTaskStatus: (docUuid: string) => ExportTask['status'] | null
}> = ({
  node,
  expandedNodes,
  setExpandedNodes,
  onExportDocument,
  isDocumentInQueue,
  getDocumentTaskStatus,
}) => {
  // 使用传入的展开状态管理
  const isExpanded = expandedNodes.has(node.uuid)
  const hasChildren = node.children.length > 0
  const isDirectory = node.type === 'TITLE'
  const isDocument = node.type === 'DOC'

  // 处理展开/折叠逻辑
  const handleToggleExpand = useCallback(() => {
    if (isExpanded) {
      // 折叠时，级联折叠所有子孙节点
      const nodesToCollapse = new Set<string>()
      const collectDescendants = (currentNode: TreeNode, depth = 0) => {
        nodesToCollapse.add(currentNode.uuid)
        currentNode.children.forEach((child) => collectDescendants(child, depth + 1))
      }
      collectDescendants(node)

      setExpandedNodes((prev) => {
        const newSet = new Set(prev)
        nodesToCollapse.forEach((uuid) => newSet.delete(uuid))
        return newSet
      })
    } else {
      // 展开时，只展开当前节点
      setExpandedNodes((prev) => new Set([...prev, node.uuid]))
    }
  }, [isExpanded, node.uuid, node.children, setExpandedNodes])

  return (
    <div className="relative">
      {/* 节点内容 */}
      <div
        className={`flex items-center py-2 hover:bg-gray-50 transition-colors ${
          isDirectory ? 'font-medium' : ''
        }`}
        style={{ paddingLeft: `${node.level * 12}px` }}
      >
        {/* 展开/折叠按钮 */}
        {hasChildren && (
          <button
            onClick={handleToggleExpand}
            className="flex items-center justify-center w-4 h-4 mr-2 text-gray-500 hover:text-gray-700 transition-colors"
          >
            <svg
              className={`w-3 h-3 transition-transform ${isExpanded ? 'rotate-90' : ''}`}
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
            </svg>
          </button>
        )}

        {/* 占位符（当没有展开按钮时） */}
        {!hasChildren && <div className="w-4 mr-2"></div>}

        {/* 图标 */}
        <span className="mr-2 text-sm">{isDirectory ? '📁' : '📄'}</span>

        {/* 标题 */}
        <span
          className={`flex-1 text-sm truncate ${
            isDirectory ? 'text-blue-700 font-medium' : 'text-gray-900'
          }`}
        >
          {node.title}
        </span>

        {/* 导出按钮 */}
        {isDocument && (
          <div className="flex items-center space-x-2">
            {/* 状态指示器 */}
            <div className="flex items-center space-x-1">
              {(() => {
                const status = getDocumentTaskStatus(node.uuid)
                switch (status) {
                  case 'pending':
                    return (
                      <div className="flex items-center space-x-1 text-xs text-gray-500">
                        <div className="w-2 h-2 bg-gray-400 rounded-full animate-pulse"></div>
                        <span>等待中</span>
                      </div>
                    )
                  case 'exporting':
                    return (
                      <div className="flex items-center space-x-1 text-xs text-blue-500">
                        <div className="w-2 h-2 bg-blue-500 rounded-full animate-pulse"></div>
                        <span>导出中</span>
                      </div>
                    )
                  case 'completed':
                    return (
                      <div className="flex items-center space-x-1 text-xs text-green-500">
                        <div className="w-2 h-2 bg-green-500 rounded-full"></div>
                        <span>已完成</span>
                      </div>
                    )
                  case 'failed':
                    return (
                      <div className="flex items-center space-x-1 text-xs text-red-500">
                        <div className="w-2 h-2 bg-red-500 rounded-full"></div>
                        <span>失败</span>
                      </div>
                    )
                  default:
                    return null
                }
              })()}
            </div>

            {/* 导出按钮 */}
            <button
              className={`px-3 py-1 text-xs font-medium rounded-md transition-colors ${
                isDocumentInQueue(node.uuid)
                  ? 'text-gray-400 bg-gray-100 cursor-not-allowed'
                  : 'text-blue-600 bg-blue-50 hover:bg-blue-100'
              }`}
              onClick={() => {
                if (!isDocumentInQueue(node.uuid)) {
                  onExportDocument(node)
                }
              }}
              disabled={isDocumentInQueue(node.uuid)}
              title={isDocumentInQueue(node.uuid) ? '文档已在导出队列中' : '导出文档'}
            >
              {isDocumentInQueue(node.uuid) ? '已添加' : '导出'}
            </button>
          </div>
        )}
      </div>

      {/* 递归渲染子节点 */}
      {hasChildren && isExpanded && (
        <div>
          {node.children.map((child) => (
            <TreeNodeComponent
              key={child.uuid}
              node={child}
              expandedNodes={expandedNodes}
              setExpandedNodes={setExpandedNodes}
              onExportDocument={onExportDocument}
              isDocumentInQueue={isDocumentInQueue}
              getDocumentTaskStatus={getDocumentTaskStatus}
            />
          ))}
        </div>
      )}
    </div>
  )
}

const BookManageDrawer: React.FC<BookManageDrawerProps> = ({
  isOpen,
  onClose,
  bookName,
  bookSlug,
  docs,
}) => {
  const { addTask, isDocumentInQueue, getDocumentsInQueue, getDocumentTaskStatus } =
    useExportStore()

  const { success: showSuccess, error: showError, warning: showWarning } = useMessage()

  // 直接使用传入的树形结构数据，无需重新构建
  const treeData = useMemo(() => {
    if (!docs || docs.length === 0) return []
    return docs
  }, [docs])

  // 计算文档总数的辅助函数
  const getTotalDocumentCount = useCallback((nodes: TreeNode[]): number => {
    let count = 0
    const countNodes = (nodeList: TreeNode[]) => {
      nodeList.forEach((node) => {
        count++
        if (node.children.length > 0) {
          countNodes(node.children)
        }
      })
    }
    countNodes(nodes)
    return count
  }, [])

  // 获取所有文档节点的辅助函数
  const getAllDocumentNodes = useCallback((nodes: TreeNode[]): TreeNode[] => {
    const documents: TreeNode[] = []
    const collectDocuments = (nodeList: TreeNode[]) => {
      nodeList.forEach((node) => {
        if (node.type === 'DOC') {
          documents.push(node)
        }
        if (node.children.length > 0) {
          collectDocuments(node.children)
        }
      })
    }
    collectDocuments(nodes)
    return documents
  }, [])

  // 全局展开状态管理
  const [expandedNodes, setExpandedNodes] = useState<Set<string>>(new Set())

  // 当树形数据变化时，更新默认展开状态
  const initializeExpandedNodes = useCallback((nodes: TreeNode[]) => {
    const initialExpanded = new Set<string>()
    const initializeExpanded = (nodeList: TreeNode[]) => {
      nodeList.forEach((node) => {
        if (node.children.length > 0) {
          initialExpanded.add(node.uuid)
          initializeExpanded(node.children)
        }
      })
    }
    initializeExpanded(nodes)
    return initialExpanded
  }, [])

  // 使用 useMemo 来避免不必要的重新计算
  const initialExpandedSet = useMemo(() => {
    if (treeData.length > 0) {
      return initializeExpandedNodes(treeData)
    }
    return new Set<string>()
  }, [treeData, initializeExpandedNodes])

  // 只在组件挂载和 treeData 真正变化时初始化展开状态
  useEffect(() => {
    if (treeData.length > 0 && initialExpandedSet.size > 0) {
      setExpandedNodes(initialExpandedSet)
    }
  }, [initialExpandedSet])

  if (!isOpen) return null

  // 创建导出任务的通用方法
  const createExportTask = (doc: TreeNode, isBatch: boolean = false): ExportTask => {
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
        bookSlug: bookSlug,
        title: doc.title,
        type: doc.type,
        uuid: doc.uuid,
        slug: doc.slug || doc.uuid, // 如果没有slug，使用uuid作为fallback
        url: doc.url,
        docFullPath: doc.docFullPath || doc.title, // 直接获取完整路径，如果没有则使用标题作为fallback
      },
    }
  }

  // 验证文档是否可导出的通用方法
  const validateDocumentForExport = (doc: TreeNode): boolean => {
    if (doc.type !== 'DOC') {
      console.log('文档类型不是DOC，跳过导出:', doc.type)
      showWarning(`跳过导出: ${doc.title} (不是文档类型)`)
      return false
    }
    return true
  }

  // 处理单个文档导出
  const handleExportDocument = (doc: TreeNode) => {
    console.log('=== handleExportDocument 被调用 ===')
    console.log('文档信息:', {
      title: doc.title,
      type: doc.type,
      uuid: doc.uuid,
      slug: doc.slug,
      bookSlug: bookSlug,
      docFullPath: doc.docFullPath, // 添加 docFullPath 的日志
    })

    if (validateDocumentForExport(doc)) {
      console.log('文档类型是DOC，开始添加导出任务')
      console.log('文档原始信息', doc)
      console.log('文档完整路径:', doc.docFullPath) // 添加详细日志
      const task = createExportTask(doc, false)
      addTask(task)
      showSuccess(`已添加导出任务: ${doc.title}`)
    }
  }

  // 处理批量导出 - 使用树形结构获取所有文档
  const handleBatchExport = () => {
    const documentsToExport = getAllDocumentNodes(treeData)

    if (documentsToExport.length === 0) {
      showWarning('没有可导出的文档')
      return
    }

    // 过滤出可导出的文档
    const validDocuments = documentsToExport.filter(validateDocumentForExport)

    if (validDocuments.length === 0) {
      showWarning('没有有效的文档可导出')
      return
    }

    // 批量创建导出任务
    validDocuments.forEach((doc) => {
      const task = createExportTask(doc, true)
      addTask(task)
    })

    showSuccess(`已添加 ${validDocuments.length} 个导出任务`)
  }

  // 计算文档总数和可导出文档数
  const totalDocCount = getTotalDocumentCount(treeData)
  const exportableDocCount = getAllDocumentNodes(treeData).length
  const queuedDocCount = getDocumentsInQueue().length

  return (
    <>
      <div className="fixed inset-0 z-modal overflow-hidden">
        {/* 背景遮罩 */}
        <div
          className="absolute inset-0 bg-black bg-opacity-50 transition-opacity"
          onClick={onClose}
        />

        {/* Drawer 内容 */}
        <div className="absolute left-0 top-0 h-full w-96 bg-white shadow-xl transform transition-transform duration-300 ease-in-out flex flex-col">
          {/* 头部 */}
          <div className="flex-shrink-0 flex items-center justify-between p-4 border-b border-gray-200">
            <h2 className="text-lg font-semibold text-gray-900">管理知识库：{bookName}</h2>
            <button
              onClick={onClose}
              className="text-gray-400 hover:text-gray-600 transition-colors"
            >
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M6 18L18 6M6 6l12 12"
                />
              </svg>
            </button>
          </div>

          {/* 文档列表 */}
          <div className="flex-1 overflow-y-auto p-4 min-h-0 scrollbar-thin scrollbar-thumb-gray-300 scrollbar-track-gray-100">
            <div className="mb-4">
              <div className="flex items-center justify-between mb-2">
                <h3 className="text-sm font-medium text-gray-700">文档列表 ({totalDocCount})</h3>
              </div>
            </div>

            {treeData.length === 0 ? (
              <div className="text-center py-8 text-gray-500">暂无文档</div>
            ) : (
              <div className="space-y-1">
                {treeData.map((node) => (
                  <TreeNodeComponent
                    key={node.uuid}
                    node={node}
                    expandedNodes={expandedNodes}
                    setExpandedNodes={setExpandedNodes}
                    onExportDocument={handleExportDocument}
                    isDocumentInQueue={isDocumentInQueue}
                    getDocumentTaskStatus={getDocumentTaskStatus}
                  />
                ))}
              </div>
            )}
          </div>

          {/* 底部操作栏 */}
          <div className="flex-shrink-0 border-t border-gray-200 p-4">
            <div className="flex items-center justify-between text-sm text-gray-600">
              <span>
                共 {totalDocCount} 个文档，{exportableDocCount} 个可导出
              </span>
              <div className="flex space-x-2">
                <button
                  onClick={handleBatchExport}
                  disabled={queuedDocCount === exportableDocCount}
                  className={`px-4 py-2 text-sm font-medium rounded-md transition-colors ${
                    queuedDocCount === exportableDocCount
                      ? 'text-gray-400 bg-gray-300 cursor-not-allowed'
                      : 'text-white bg-blue-500 hover:bg-blue-600'
                  }`}
                  title={
                    queuedDocCount === exportableDocCount
                      ? '所有文档已在导出队列中'
                      : '导出所有文档'
                  }
                >
                  {queuedDocCount === exportableDocCount ? '已全部添加' : '导出全部'}
                </button>
                <button
                  onClick={onClose}
                  className="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-md hover:bg-gray-200 transition-colors"
                >
                  关闭
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </>
  )
}

export default BookManageDrawer
