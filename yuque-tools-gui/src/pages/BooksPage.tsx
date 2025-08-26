import React, { useState, useEffect } from 'react'
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
import {
  ChevronDownIcon,
  ChevronRightIcon,
  FolderIcon,
  DocumentIcon,
} from '@heroicons/react/24/outline'
import { Card, CardContent, CardHeader, CardTitle } from '../components/ui/card'
import { Badge } from '../components/ui/badge'
import BookManageDrawer from '../components/BookManageDrawer'
import ConfirmDialog from '../components/ConfirmDialog'
import { useMessage } from '../hooks/useMessage'
import type { TreeNode, BookItemRaw, BookItem } from '../types/yuque'

interface ExpandedState {
  [bookId: string]: boolean
}

type TabType = 'personal' | 'team'

// 构建树形结构的工具函数
const buildTreeStructure = (docs: DocItem[]): TreeNode[] => {
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
    node.docFullPath = currentPath

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

const BooksPage: React.FC = () => {
  const [activeTab, setActiveTab] = useState<TabType>('personal')
  const [personalBooks, setPersonalBooks] = useState<BookItem[]>([])
  const [teamBooks, setTeamBooks] = useState<BookItem[]>([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [expandedBooks, setExpandedBooks] = useState<ExpandedState>({})
  const { success: showSuccess, error: showError, info: showInfo } = useMessage()
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
        console.log('缓存清除完成')
        showSuccess('缓存清除完成')
      }

      // 并行加载个人和团队知识库
      const [personalResponse, teamResponse] = await Promise.all([
        tauriApi.getPersonalBooks(),
        tauriApi.getTeamBooks(),
      ])

      if (personalResponse.success && personalResponse.data) {
        console.log('个人知识库原始数据:', personalResponse.data)

        // 为每个知识库构建树形结构
        const personalBooksWithTree: BookItem[] = (personalResponse.data as BookItemRaw[]).map(
          (book) => ({
            ...book,
            docs: buildTreeStructure(book.docs),
          })
        ) as BookItem[]

        console.log('个人知识库树形结构:', personalBooksWithTree)
        setPersonalBooks(personalBooksWithTree)
      }

      if (teamResponse.success && teamResponse.data) {
        console.log('团队知识库原始数据:', teamResponse.data)

        // 为每个知识库构建树形结构
        const teamBooksWithTree: BookItem[] = (teamResponse.data as BookItemRaw[]).map((book) => ({
          ...book,
          docs: buildTreeStructure(book.docs),
        })) as BookItem[]

        console.log('团队知识库树形结构:', teamBooksWithTree)
        setTeamBooks(teamBooksWithTree)
      }

      // 检查是否有错误
      if (!personalResponse.success && !teamResponse.success) {
        setError('获取知识库失败')
        showError('获取知识库失败')
      } else {
        showSuccess('知识库数据加载完成')
      }
    } catch (err) {
      setError(`获取知识库失败：${err}`)
    } finally {
      setLoading(false)
    }
  }

  const toggleBookExpansion = (bookId: string) => {
    setExpandedBooks((prev) => ({
      ...prev,
      [bookId]: !prev[bookId],
    }))
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

  const renderDocumentRow = (doc: TreeNode, _bookId: string, level: number = 0) => {
    const isDocument = doc.type === 'DOC'
    const isTitle = doc.type === 'TITLE'

    return (
      <TableRow key={doc.uuid} className="hover:bg-gray-50">
        <TableCell className="pl-6">
          <div className="flex items-center space-x-2" style={{ paddingLeft: `${level * 20}px` }}>
            {isTitle ? (
              <FolderIcon className="h-4 w-4 text-yellow-500" />
            ) : (
              <DocumentIcon className="h-4 w-4 text-blue-500" />
            )}
            <span className={isTitle ? 'font-medium' : ''}>{doc.title}</span>
          </div>
        </TableCell>
        <TableCell>
          <Badge variant={isDocument ? 'default' : 'secondary'}>
            {isDocument ? '文档' : '目录'}
          </Badge>
        </TableCell>
        <TableCell className="text-sm text-gray-500">
          {doc.visible === 1 ? '可见' : '隐藏'}
        </TableCell>
        <TableCell className="text-sm text-gray-500">{doc.uuid}</TableCell>
        <TableCell>
          <Button variant="outline" size="sm">
            查看
          </Button>
        </TableCell>
      </TableRow>
    )
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
    <div className="mx-auto p-2">
      <div className="flex items-center justify-between mb-6">
        <div>
          <p className="text-gray-600 mt-2">
            {getCurrentTabTitle()} - 共 {getCurrentBooks().length} 个知识库
          </p>
        </div>
        <Button onClick={showRefreshConfirm} variant="outline">
          刷新
        </Button>
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
                  <React.Fragment key={book.slug}>
                    {/* 知识库行 */}
                    <TableRow className="bg-gray-50 hover:bg-gray-100">
                      <TableCell>
                        <div className="flex items-center space-x-3">
                          <Button
                            variant="ghost"
                            size="sm"
                            onClick={() => toggleBookExpansion(book.slug)}
                            className="p-1 h-6 w-6"
                          >
                            {expandedBooks[book.slug] ? (
                              <ChevronDownIcon className="h-4 w-4" />
                            ) : (
                              <ChevronRightIcon className="h-4 w-4" />
                            )}
                          </Button>
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
                        <Badge variant="outline">{book.docs.length} 个文档</Badge>
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
                          <Button variant="outline" size="sm">
                            导出
                          </Button>
                        </div>
                      </TableCell>
                    </TableRow>

                    {/* 文档行（展开时显示） */}
                    {expandedBooks[book.slug] && book.docs.length > 0 && (
                      <>{book.docs.map((doc) => renderDocumentRow(doc, book.slug))}</>
                    )}

                    {/* 空文档提示 */}
                    {expandedBooks[book.slug] && book.docs.length === 0 && (
                      <TableRow>
                        <TableCell colSpan={5} className="text-center py-8 text-gray-500">
                          该知识库暂无文档
                        </TableCell>
                      </TableRow>
                    )}
                  </React.Fragment>
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
