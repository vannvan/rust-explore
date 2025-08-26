import React, { useState, useEffect, Fragment } from 'react'
import { tauriApi } from '../services/tauriApi'
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '../components/ui/table'
import { Button } from '../components/ui/button'

import { Card, CardContent, CardHeader, CardTitle } from '../components/ui/card'
import { Badge } from '../components/ui/badge'
import BookManageDrawer from '../components/BookManageDrawer'
import ConfirmDialog from '../components/ConfirmDialog'
import CacheCountdown from '../components/CacheCountdown'
import { useExportStore } from '../stores/exportStore'
import type { TreeNode, BookItemRaw, BookItem } from '../types/yuque'
import type { ExportTask } from '../components/ExportQueuePanel'

type TabType = 'personal' | 'team'

interface BooksPageProps {
  showSuccess: (message: string, duration?: number) => string
  showError: (message: string, duration?: number) => string
  showInfo: (message: string, duration?: number) => string
  showWarning: (message: string, duration?: number) => string
}

// 构建树形结构的工具函数
const buildTreeStructure = (bookName: string, docs: DocItem[]): TreeNode[] => {
  if (!docs || docs.length === 0) return []

  // 创建文档映射
  const docMap = new Map<string, TreeNode>()
  const rootNodes: TreeNode[] = []

  // 初始化所有节点
  docs.forEach((doc) => {
    return docMap.set(doc.uuid, {
      ...doc,
      id: doc.uuid, // 确保 id 是 string 类型
      children: [],
      level:
        typeof doc.level === 'string' ? parseInt(doc.level, 10) || 0 : (doc.level as number) || 0,
      docFullPath: '',
      doc_id: doc.doc_id !== undefined ? String(doc.doc_id) : undefined,
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
        parentNode.children.push(childNode)
      }
    }
  })

  // 计算每个节点的完整路径
  const calculateFullPath = (node: TreeNode, parentPath: string = '') => {
    // 清理文件名，替换不能作为路径的字符
    const cleanTitle = node.title.replace(/[<>:"/\\|?*]/g, '-')

    // 构建当前节点的路径
    const currentPath = parentPath ? `${parentPath}/${cleanTitle}` : cleanTitle

    // 将 bookName 拼在最前面，构建包含知识库的完整文档路径
    node.docFullPath = `${bookName}/${currentPath}`

    // 递归计算子节点的路径
    node.children.forEach((child) => {
      calculateFullPath(child, currentPath)
    })
  }

  // 为所有根节点计算完整路径
  rootNodes.forEach((node) => {
    calculateFullPath(node)
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
  return rootNodes
}

const BooksPage: React.FC<BooksPageProps> = ({ showSuccess, showError, showInfo, showWarning }) => {
  const [activeTab, setActiveTab] = useState<TabType>('personal')
  const [personalBooks, setPersonalBooks] = useState<BookItem[]>([])
  const [teamBooks, setTeamBooks] = useState<BookItem[]>([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const { addTask } = useExportStore()
  // 导出队列现在一直显示在右下角，不需要手动控制显示

  // Drawer 相关状态
  const [drawerOpen, setDrawerOpen] = useState(false)
  const [selectedBook, setSelectedBook] = useState<BookItem | null>(null)

  // 确认弹窗状态
  const [showConfirmDialog, setShowConfirmDialog] = useState(false)

  useEffect(() => {
    loadBooks()
  }, [])

  const loadBooks = async (clearCache: boolean = false) => {
    setLoading(true)
    setError(null)

    try {
      // 如果需要清除缓存，先清除相关缓存
      if (clearCache) {
        console.log('正在清除缓存...')
        showInfo('正在清除缓存...')
        await Promise.all([tauriApi.clearBooksCache(), tauriApi.clearDocsCache()])
        // 清除本地缓存时间记录
        localStorage.removeItem('lastCacheTime')
        console.log('缓存清除完成')
        showSuccess('缓存清除完成')
      }

      // 并行加载个人和团队知识库
      const [personalResponse, teamResponse] = await Promise.all([
        tauriApi.getPersonalBooks(),
        tauriApi.getTeamBooks(),
      ])

      if (personalResponse.success && personalResponse.data) {
        // 为每个知识库构建树形结构
        const personalBooksWithTree: BookItem[] = (personalResponse.data as BookItemRaw[]).map(
          (book) => ({
            ...book,
            docs: buildTreeStructure(book.name, book.docs),
          })
        ) as BookItem[]

        setPersonalBooks(personalBooksWithTree)
      }

      if (teamResponse.success && teamResponse.data) {
        // 为每个知识库构建树形结构
        const teamBooksWithTree: BookItem[] = (teamResponse.data as BookItemRaw[]).map((book) => ({
          ...book,
          docs: buildTreeStructure(book.name, book.docs),
        })) as BookItem[]

        setTeamBooks(teamBooksWithTree)
      }

      // 检查是否有错误
      if (!personalResponse.success && !teamResponse.success) {
        setError('获取知识库失败')
        showError('获取知识库失败')
      } else {
        showSuccess('知识库数据加载完成')
        // 记录缓存时间
        localStorage.setItem('lastCacheTime', Date.now().toString())
      }
    } catch (err) {
      setError(`获取知识库失败：${err}`)
    } finally {
      setLoading(false)
    }
  }

  const getBookIcon = (bookType: string) => {
    return bookType === 'owner' ? '📚' : '🤝'
  }

  const getBookTypeLabel = (bookType: string) => {
    return bookType === 'owner' ? '我的知识库' : '协作知识库'
  }

  const getBookTypeColor = (bookType: string) => {
    return bookType === 'owner' ? 'bg-blue-100 text-blue-800' : 'bg-green-100 text-green-800'
  }

  // 打开知识库管理 Drawer
  const openBookManageDrawer = (book: BookItem) => {
    setSelectedBook(book)
    setDrawerOpen(true)
  }

  // 关闭知识库管理 Drawer
  const closeBookManageDrawer = () => {
    setDrawerOpen(false)
    setSelectedBook(null)
  }

  // 显示刷新确认弹窗
  const showRefreshConfirm = () => {
    setShowConfirmDialog(true)
  }

  // 确认刷新操作
  const handleConfirmRefresh = () => {
    loadBooks(true)
  }

  // 关闭确认弹窗
  const closeConfirmDialog = () => {
    setShowConfirmDialog(false)
  }

  // 获取当前 Tab 的知识库数据
  const getCurrentBooks = () => {
    return activeTab === 'personal' ? personalBooks : teamBooks
  }

  // 获取当前 Tab 的标题
  const getCurrentTabTitle = () => {
    return activeTab === 'personal' ? '个人知识库' : '团队知识库'
  }

  // 获取知识库中文档节点的数量（只统计 type="DOC" 的节点）
  const getDocumentCount = (nodes: TreeNode[]): number => {
    let count = 0

    const countDocuments = (nodeList: TreeNode[]) => {
      nodeList.forEach((node) => {
        if (node.type === 'DOC') {
          count++
        }
        if (node.children.length > 0) {
          countDocuments(node.children)
        }
      })
    }

    countDocuments(nodes)
    return count
  }

  // 获取所有文档节点的辅助函数
  const getAllDocumentNodes = (nodes: TreeNode[]): TreeNode[] => {
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

  // 创建导出任务的通用方法
  const createExportTask = (
    doc: TreeNode,
    bookSlug: string,
    isBatch: boolean = false
  ): ExportTask => {
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

  // 处理知识库导出
  const handleExportBook = (book: BookItem) => {
    console.log('=== handleExportBook 被调用 ===')
    console.log('知识库信息:', {
      name: book.name,
      slug: book.slug,
      docCount: book.docs.length,
    })

    if (!book.docs || book.docs.length === 0) {
      showWarning('该知识库暂无文档可导出')
      return
    }

    // 获取所有文档节点
    const documentsToExport = getAllDocumentNodes(book.docs)

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
      const task = createExportTask(doc, book.slug, true)
      addTask(task)
    })

    showSuccess(`已添加 ${validDocuments.length} 个导出任务到队列`)
  }

  if (loading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto"></div>
          <p className="mt-4 text-gray-600">正在加载知识库...</p>
        </div>
      </div>
    )
  }

  if (error) {
    return (
      <div className="flex items-center justify-center h-64">
        <Card className="w-full max-w-md">
          <CardHeader>
            <CardTitle className="text-red-600">加载失败</CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-gray-600 mb-4">{error}</p>
            <Button onClick={() => loadBooks(false)} className="w-full">
              重试
            </Button>
          </CardContent>
        </Card>
      </div>
    )
  }

  return (
    <div className="mx-auto">
      {/* 缓存倒计时 */}
      <div className="mb-4">
        <CacheCountdown />
      </div>

      <div className="flex items-center justify-between mb-6">
        <div>
          <p className="text-gray-600 mt-2">
            {getCurrentTabTitle()} - 共 {getCurrentBooks().length} 个知识库
          </p>
        </div>
        <div
          className="px-1 py-1 bg-blue-400 text-white rounded-md cursor-pointer hover:bg-blue-500"
          onClick={showRefreshConfirm}
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            strokeWidth={1.5}
            stroke="currentColor"
            className="size-4"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0 3.181 3.183a8.25 8.25 0 0 0 13.803-3.7M4.031 9.865a8.25 8.25 0 0 1 13.803-3.7l3.181 3.182m0-4.991v4.99"
            />
          </svg>
        </div>
      </div>

      {/* Tab 切换 */}
      <div className="mb-6">
        <div className="border-b border-gray-200">
          <nav className="-mb-px flex space-x-8">
            <button
              onClick={() => setActiveTab('personal')}
              className={`py-2 px-1 border-b-2 font-medium text-sm ${
                activeTab === 'personal'
                  ? 'border-blue-500 text-blue-600'
                  : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
              }`}
            >
              个人知识库 ({personalBooks.length})
            </button>
            <button
              onClick={() => setActiveTab('team')}
              className={`py-2 px-1 border-b-2 font-medium text-sm ${
                activeTab === 'team'
                  ? 'border-blue-500 text-blue-600'
                  : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
              }`}
            >
              团队知识库 ({teamBooks.length})
            </button>
          </nav>
        </div>
      </div>

      {getCurrentBooks().length === 0 ? (
        <Card>
          <CardContent className="p-8 text-center">
            <p className="text-gray-500">暂无知识库数据</p>
          </CardContent>
        </Card>
      ) : (
        <Card>
          {/* <CardHeader>
            <CardTitle>知识库列表</CardTitle>
          </CardHeader> */}
          <CardContent>
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead className="w-[400px]">名称</TableHead>
                  <TableHead>类型</TableHead>
                  <TableHead>所有者</TableHead>
                  <TableHead>文档数量</TableHead>
                  <TableHead>操作</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {getCurrentBooks().map((book) => (
                  <Fragment key={book.slug}>
                    {/* 知识库行 */}
                    <TableRow className="bg-gray-50 hover:bg-gray-100">
                      <TableCell>
                        <div className="flex items-center space-x-3">
                          <span className="text-lg">{getBookIcon(book.book_type)}</span>
                          <div>
                            <div className="font-medium">{book.name}</div>
                            <div className="text-sm text-gray-500">
                              /{book.user_login}/{book.slug}
                            </div>
                          </div>
                        </div>
                      </TableCell>
                      <TableCell>
                        <Badge className={getBookTypeColor(book.book_type)}>
                          {getBookTypeLabel(book.book_type)}
                        </Badge>
                      </TableCell>
                      <TableCell>
                        <div className="text-sm">
                          <div className="font-medium">{book.user_name}</div>
                          <div className="text-gray-500">@{book.user_login}</div>
                        </div>
                      </TableCell>
                      <TableCell>
                        <Badge variant="outline">{getDocumentCount(book.docs)} 个文档</Badge>
                      </TableCell>
                      <TableCell>
                        <div className="flex space-x-2">
                          <Button
                            variant="outline"
                            size="sm"
                            onClick={() => openBookManageDrawer(book)}
                          >
                            查看
                          </Button>
                          <Button
                            variant="outline"
                            size="sm"
                            onClick={() => handleExportBook(book)}
                          >
                            导出全部
                          </Button>
                        </div>
                      </TableCell>
                    </TableRow>

                    {/* 文档行（展开时显示） */}
                    {/* {expandedBooks[book.slug] && book.docs.length > 0 && (
                      <>{book.docs.map((doc) => renderDocumentRow(doc, book.slug))}</>
                    )} */}

                    {/* 空文档提示 */}
                    {/* {expandedBooks[book.slug] && book.docs.length === 0 && (
                      <TableRow>
                        <TableCell colSpan={5} className="text-center py-8 text-gray-500">
                          该知识库暂无文档
                        </TableCell>
                      </TableRow>
                    )} */}
                  </Fragment>
                ))}
              </TableBody>
            </Table>
          </CardContent>
        </Card>
      )}

      {/* 知识库管理 Drawer */}
      {selectedBook && (
        <BookManageDrawer
          isOpen={drawerOpen}
          onClose={closeBookManageDrawer}
          bookName={selectedBook.name}
          bookSlug={selectedBook.slug}
          docs={selectedBook.docs}
        />
      )}

      {/* 刷新确认弹窗 */}
      <ConfirmDialog
        isOpen={showConfirmDialog}
        onClose={closeConfirmDialog}
        onConfirm={handleConfirmRefresh}
        title="确认刷新"
        message="该操作会重新获取所有知识库及文档信息，需要一定的时间。当前的缓存数据将被清除，确定要继续吗？"
        confirmText="确认刷新"
        cancelText="取消"
        type="warning"
      />
    </div>
  )
}

export default BooksPage
