import React, { useMemo, useState, useCallback } from 'react'
import { useExportStore } from '../stores/exportStore'
import { useMessage } from '../hooks/useMessage'
import type { ExportTask } from '../components/ExportQueuePanel'

// 使用全局的 DocItem 类型，删除本地接口定义

interface TreeNode extends DocItem {
  children: TreeNode[]
  level: number
}

interface BookManageDrawerProps {
  isOpen: boolean
  onClose: () => void
  bookName: string
  bookSlug: string // 添加知识库slug
  docs: DocItem[]
}

// 递归渲染树形节点的组件 - 提取到外部，避免死循环
const TreeNodeComponent: React.FC<{
  node: TreeNode
  expandedNodes: Set<string>
  setExpandedNodes: React.Dispatch<React.SetStateAction<Set<string>>>
  onExportDocument: (doc: DocItem) => void
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
        // console.log(`Debug: Collecting descendant at depth ${depth}:`, {
        //   title: currentNode.title,
        //   uuid: currentNode.uuid,
        //   children_count: currentNode.children.length,
        // })
        nodesToCollapse.add(currentNode.uuid)
        currentNode.children.forEach((child) => collectDescendants(child, depth + 1))
      }
      collectDescendants(node)

      setExpandedNodes((prev) => {
        // const prevArray = Array.from(prev)
        const newSet = new Set(prev)
        nodesToCollapse.forEach((uuid) => newSet.delete(uuid))
        // const newArray = Array.from(newSet)

        // console.log('Debug: State transition', {
        //   before: prevArray,
        //   toCollapse: Array.from(nodesToCollapse),
        //   after: newArray,
        // })

        return newSet
      })

      // console.log('Debug: Collapsed node and descendants', {
      //   title: node.title,
      //   collapsedNodes: Array.from(nodesToCollapse),
      //   totalCollapsed: nodesToCollapse.size,
      // })
    } else {
      // 展开时，只展开当前节点
      setExpandedNodes((prev) => new Set([...prev, node.uuid]))
      // console.log('Debug: Expanded node', {
      //   title: node.title,
      //   uuid: node.uuid,
      // })
    }
  }, [isExpanded, node.uuid, node.title, node.children, setExpandedNodes])

  // 调试信息 - 简化调试输出
  // console.log('Debug: TreeNode render', {
  //   title: node.title,
  //   child_uuid: node.child_uuid,
  //   children_count: node.children.length,
  //   hasChildren,
  //   isExpanded,
  //   type: node.type,
  // })

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

        {/* 标签 */}
        <div className="flex items-center space-x-2 mr-3">
          {/* 类型标签 */}
          {/* <span
            className={`inline-flex items-center px-2 py-1 rounded-full text-xs font-medium ${
              isDirectory ? 'bg-blue-100 text-blue-800' : 'bg-gray-100 text-gray-700'
            }`}
          >
            {isDirectory ? '目录' : '文档'}
          </span> */}

          {/* 可见性标签 */}
          {/* {node.visible === 1 && (
            <span className="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-green-100 text-green-800">
              可见
            </span>
          )} */}

          {/* 层级标签 */}
          {/* <span className="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-gray-100 text-gray-600">
            层级 {node.level}
          </span> */}
        </div>

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

  // 构建树形结构
  const treeData = useMemo(() => {
    if (!docs || docs.length === 0) return []

    // 创建文档映射
    const docMap = new Map<string, TreeNode>()
    const rootNodes: TreeNode[] = []

    // 初始化所有节点，优先使用原始 level 字段
    docs.forEach((doc) => {
      docMap.set(doc.uuid, {
        ...doc,
        children: [],
        level: (doc.level as number) || 0,
      })
    })

    // 构建父子关系
    docs.forEach((doc) => {
      const node = docMap.get(doc.uuid)!
      if (doc.parent_uuid && doc.parent_uuid !== '' && docMap.has(doc.parent_uuid)) {
        const parent = docMap.get(doc.parent_uuid)!
        parent.children.push(node)
        // 如果没有原始 level，则计算层级
        if (node.level === 0) {
          node.level = parent.level + 1
        }
      } else {
        rootNodes.push(node)
      }
    })

    // 为了调试，也根据child_uuid构建反向关系
    docs.forEach((doc) => {
      if (doc.child_uuid && doc.child_uuid !== '') {
        const parentNode = docMap.get(doc.uuid)!
        const childNode = docMap.get(doc.child_uuid)
        if (childNode && !parentNode.children.includes(childNode)) {
          // console.log('Debug: Adding child via child_uuid', {
          //   parent: parentNode.title,
          //   child: childNode.title,
          // })
          parentNode.children.push(childNode)
        }
      }
    })

    // 按原始顺序排序，保持语雀的文档顺序
    const sortByOrder = (nodes: TreeNode[]) => {
      nodes.sort((a, b) => {
        // 首先按 level 排序
        if (a.level !== b.level) {
          return a.level - b.level
        }
        // 然后按在原始数组中的位置排序
        const aIndex = docs.findIndex((d) => d.uuid === a.uuid)
        const bIndex = docs.findIndex((d) => d.uuid === b.uuid)
        return aIndex - bIndex
      })

      // 递归排序子节点
      nodes.forEach((node) => {
        if (node.children.length > 0) {
          sortByOrder(node.children)
        }
      })
    }

    sortByOrder(rootNodes)
    // console.log('Debug: Tree structure built', {
    //   totalDocs: docs.length,
    //   rootNodes: rootNodes.length,
    //   allNodes: Array.from(docMap.values()).map((node) => ({
    //     title: node.title,
    //     uuid: node.uuid,
    //     parent_uuid: node.parent_uuid,
    //     child_uuid: node.child_uuid,
    //     children: node.children.length,
    //     type: node.type,
    //   })),
    // })

    return rootNodes
  }, [docs])

  // 全局展开状态管理
  const [expandedNodes, setExpandedNodes] = useState<Set<string>>(new Set())

  // 当树形数据变化时，更新默认展开状态 - 使用 useCallback 优化
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
  React.useEffect(() => {
    if (treeData.length > 0 && initialExpandedSet.size > 0) {
      setExpandedNodes(initialExpandedSet)
      // console.log('Debug: Initial expanded nodes', Array.from(initialExpandedSet))
    }
  }, [initialExpandedSet])

  if (!isOpen) return null

  // 处理单个文档导出
  const handleExportDocument = (doc: DocItem) => {
    console.log('=== handleExportDocument 被调用 ===')
    console.log('文档信息:', {
      title: doc.title,
      type: doc.type,
      uuid: doc.uuid,
      slug: doc.slug,
      bookSlug: bookSlug,
    })

    if (doc.type === 'DOC') {
      console.log('文档类型是DOC，开始添加导出任务')
      console.log('文档原始信息', doc)
      const task: ExportTask = {
        id: `${doc.uuid}-${Date.now()}`,
        title: doc.title,
        status: 'pending',
        progress: 0,
        startTime: new Date(),
        docInfo: {
          bookSlug: bookSlug, // 知识库的slug
          title: doc.title,
          type: doc.type,
          uuid: doc.uuid,
          slug: doc.slug || doc.uuid, // 如果没有slug，使用uuid作为fallback
          url: doc.url,
        },
      }
      addTask(task)
      showSuccess(`已添加导出任务: ${doc.title}`)
    } else {
      console.log('文档类型不是DOC，跳过导出:', doc.type)
      showWarning(`跳过导出: ${doc.title} (不是文档类型)`)
    }
  }

  // 处理批量导出
  const handleBatchExport = () => {
    const documentsToExport = docs.filter((doc) => doc.type === 'DOC')
    if (documentsToExport.length > 0) {
      documentsToExport.forEach((doc) => {
        const task: ExportTask = {
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
        }
        addTask(task)
      })
      showSuccess(`已添加 ${documentsToExport.length} 个导出任务`)
    } else {
      showWarning('没有可导出的文档')
    }
  }

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
                <h3 className="text-sm font-medium text-gray-700">文档列表 ({docs.length})</h3>
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
              <span>共 {docs.length} 个文档</span>
              <div className="flex space-x-2">
                <button
                  onClick={handleBatchExport}
                  disabled={
                    getDocumentsInQueue().length === docs.filter((doc) => doc.type === 'DOC').length
                  }
                  className={`px-4 py-2 text-sm font-medium rounded-md transition-colors ${
                    getDocumentsInQueue().length === docs.filter((doc) => doc.type === 'DOC').length
                      ? 'text-gray-400 bg-gray-300 cursor-not-allowed'
                      : 'text-white bg-blue-500 hover:bg-blue-600'
                  }`}
                  title={
                    getDocumentsInQueue().length === docs.filter((doc) => doc.type === 'DOC').length
                      ? '所有文档已在导出队列中'
                      : '导出所有文档'
                  }
                >
                  {getDocumentsInQueue().length === docs.filter((doc) => doc.type === 'DOC').length
                    ? '已全部添加'
                    : '导出全部'}
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
