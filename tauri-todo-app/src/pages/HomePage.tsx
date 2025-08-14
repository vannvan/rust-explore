import { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import dayjs from 'dayjs'
import { Tag, Plus, Clock, CheckCircle } from 'lucide-react'
import { DatePicker, Button, TextArea, Space, Toast } from 'antd-mobile'

// 事项分类数据
const taskCategories = [
  { id: 1, name: '工作', color: 'bg-blue-500', icon: '💼' },
  { id: 2, name: '学习', color: 'bg-green-500', icon: '📚' },
  { id: 3, name: '生活', color: 'bg-yellow-500', icon: '🏠' },
  { id: 4, name: '运动', color: 'bg-red-500', icon: '🏃' },
  { id: 5, name: '娱乐', color: 'bg-purple-500', icon: '🎮' },
]

interface Task {
  id: number
  taskName: string
  taskStartTime: string
  taskEndTime: string
  finished: boolean
  categoryId?: number
}

const HomePage = () => {
  const [taskName, setTaskName] = useState('')
  const [deadline, setDeadline] = useState<[Date | null, Date | null] | null>(null)
  const [selectedCategory, setSelectedCategory] = useState<number | null>(null)
  const [taskList, setTaskList] = useState<Task[]>([])
  const [showAddForm, setShowAddForm] = useState(false)
  const [showStartDatePicker, setShowStartDatePicker] = useState(false)
  const [showEndDatePicker, setShowEndDatePicker] = useState(false)

  useEffect(() => {
    const container = document.getElementById('container')
    if (container) {
      container.addEventListener('contextmenu', function (event) {
        event.preventDefault()
      })
    }

    invoke('get_tasks').then((res: any) => {
      setTaskList(res || [])
    })
  }, [])

  const handleAddItem = async () => {
    if (!deadline || !taskName.trim()) {
      Toast.show({
        content: '请填写完整的任务信息',
        position: 'center',
      })
      return
    }

    const item = {
      taskName: taskName.trim(),
      deadline: [
        dayjs(deadline[0]).format('YYYY-MM-DD HH:mm:ss'),
        dayjs(deadline[1]).format('YYYY-MM-DD HH:mm:ss'),
      ],
      categoryId: selectedCategory,
    }

    let res = await invoke('add_task', item)
    if (res) {
      Toast.show({
        content: '任务添加成功',
        position: 'center',
        icon: 'success',
      })
      setTaskName('')
      setDeadline(null)
      setSelectedCategory(null)
      setShowAddForm(false)
      invoke('get_tasks').then((res: any) => {
        setTaskList(res || [])
      })
    } else {
      Toast.show({
        content: '任务添加失败',
        position: 'center',
        icon: 'fail',
      })
    }
  }

  const handleDeleteTask = async (id: number) => {
    let res = await invoke('delete_task', { id: id })
    if (res) {
      Toast.show({
        content: '任务删除成功',
        position: 'center',
        icon: 'success',
      })
      invoke('get_tasks').then((res: any) => {
        setTaskList(res || [])
      })
    } else {
      Toast.show({
        content: '任务删除失败',
        position: 'center',
        icon: 'fail',
      })
    }
  }

  const handleUpdateFinishTask = async (id: number, target: boolean) => {
    const res = await invoke('finish_task', { id, target })
    if (res) {
      Toast.show({
        content: target ? '任务已完成' : '任务已重新激活',
        position: 'center',
        icon: 'success',
      })
      invoke('get_tasks').then((res: any) => {
        setTaskList(res || [])
      })
    } else {
      Toast.show({
        content: '操作失败',
        position: 'center',
        icon: 'fail',
      })
    }
  }

  const formatDateTime = (dateTimeStr: string) => {
    return dayjs(dateTimeStr).format('MM-DD HH:mm')
  }

  const getCategoryInfo = (categoryId?: number) => {
    if (!categoryId) return null
    return taskCategories.find((cat) => cat.id === categoryId)
  }

  const pendingTasks = taskList.filter((t) => !t.finished)
  const completedTasks = taskList.filter((t) => t.finished)

  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-50 via-white to-purple-50">
      {/* Header */}
      <div className="bg-white/80 backdrop-blur-md border-b border-gray-200/50 sticky top-0 z-10">
        <div className="max-w-6xl mx-auto px-6 py-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center space-x-3">
              <div className="w-10 h-10 bg-gradient-to-r from-blue-500 to-purple-600 rounded-xl flex items-center justify-center">
                <svg
                  className="w-6 h-6 text-white"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M9 5H7a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"
                  />
                </svg>
              </div>
              <div>
                <h1 className="text-2xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
                  智能待办
                </h1>
                <p className="text-sm text-gray-500">高效管理您的任务</p>
              </div>
            </div>
            <div className="flex items-center space-x-2">
              <div className="px-4 py-2 bg-blue-50 text-blue-600 rounded-lg text-sm font-medium">
                {pendingTasks.length} 个待完成
              </div>
            </div>
          </div>
        </div>
      </div>

      <div className="max-w-6xl mx-auto px-6 py-8">
        {/* Quick Stats */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-8">
          <div className="bg-white/70 backdrop-blur-sm rounded-xl p-4 border border-white/50">
            <div className="flex items-center space-x-3">
              <div className="w-10 h-10 bg-blue-100 rounded-lg flex items-center justify-center">
                <Clock className="w-5 h-5 text-blue-600" />
              </div>
              <div>
                <p className="text-sm text-gray-600">待办事项</p>
                <p className="text-2xl font-bold text-blue-600">{pendingTasks.length}</p>
              </div>
            </div>
          </div>

          <div className="bg-white/70 backdrop-blur-sm rounded-xl p-4 border border-white/50">
            <div className="flex items-center space-x-3">
              <div className="w-10 h-10 bg-green-100 rounded-lg flex items-center justify-center">
                <CheckCircle className="w-5 h-5 text-green-600" />
              </div>
              <div>
                <p className="text-sm text-gray-600">已完成</p>
                <p className="text-2xl font-bold text-green-600">{completedTasks.length}</p>
              </div>
            </div>
          </div>

          <div className="bg-white/70 backdrop-blur-sm rounded-xl p-4 border border-white/50">
            <div className="flex items-center space-x-3">
              <div className="w-10 h-10 bg-purple-100 rounded-lg flex items-center justify-center">
                <Tag className="w-5 h-5 text-purple-600" />
              </div>
              <div>
                <p className="text-sm text-gray-600">分类数量</p>
                <p className="text-2xl font-bold text-purple-600">{taskCategories.length}</p>
              </div>
            </div>
          </div>
        </div>

        {/* Add Task Button */}
        <div className="text-center mb-8">
          <Button
            onClick={() => setShowAddForm(!showAddForm)}
            color="primary"
            fill="solid"
            size="large"
            className="px-8 py-4 rounded-2xl flex items-center justify-center mx-auto space-x-2"
          >
            {/* <Plus className="w-5 h-5" /> */}
            <span>添加新任务</span>
          </Button>
        </div>

        {/* Add Task Form */}
        {showAddForm && (
          <div className="bg-white/70 backdrop-blur-sm rounded-2xl shadow-soft border border-white/50 p-6 mb-8 animate-fade-in">
            <h2 className="text-lg font-semibold text-gray-800 mb-4 flex items-center">
              <Plus className="w-5 h-5 text-blue-500 mr-2" />
              添加新任务
            </h2>

            <div className="space-y-6">
              {/* 任务描述 - 全宽 */}
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">任务描述</label>
                <TextArea
                  value={taskName}
                  onChange={(val) => setTaskName(val)}
                  placeholder="请输入任务描述..."
                  rows={3}
                  maxLength={100}
                  showCount
                  className="w-full"
                />
              </div>

              {/* 时间选择器 - 两列布局 */}
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">开始时间</label>
                  <DatePicker
                    value={deadline?.[0]}
                    onConfirm={(val) => {
                      const newDeadline: [Date | null, Date | null] = deadline
                        ? [val, deadline[1]]
                        : [val, null]
                      setDeadline(newDeadline)
                      setShowStartDatePicker(false)
                    }}
                    onClose={() => setShowStartDatePicker(false)}
                    precision="day"
                    title="选择开始时间"
                    visible={showStartDatePicker}
                  >
                    {(value) => (
                      <div
                        className="w-full px-4 py-3 border border-gray-200 rounded-xl focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200 cursor-pointer hover:border-blue-300"
                        onClick={() => setShowStartDatePicker(true)}
                      >
                        {value ? dayjs(value).format('YYYY-MM-DD HH:mm') : '请选择开始时间'}
                      </div>
                    )}
                  </DatePicker>
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">结束时间</label>
                  <DatePicker
                    value={deadline?.[1]}
                    onConfirm={(val) => {
                      const newDeadline: [Date | null, Date | null] = deadline
                        ? [deadline[0], val]
                        : [null, val]
                      setDeadline(newDeadline)
                      setShowEndDatePicker(false)
                    }}
                    onClose={() => setShowEndDatePicker(false)}
                    precision="day"
                    title="选择结束时间"
                    visible={showEndDatePicker}
                  >
                    {(value) => (
                      <div
                        className="w-full px-4 py-3 border border-gray-200 rounded-xl focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200 cursor-pointer hover:border-blue-300"
                        onClick={() => setShowEndDatePicker(true)}
                      >
                        {value ? dayjs(value).format('YYYY-MM-DD HH:mm') : '请选择结束时间'}
                      </div>
                    )}
                  </DatePicker>
                </div>
              </div>

              {/* 分类选择器 - 全宽 */}
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">选择分类</label>
                <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-5 gap-3">
                  {taskCategories.map((category) => (
                    <button
                      key={category.id}
                      onClick={() =>
                        setSelectedCategory(selectedCategory === category.id ? null : category.id)
                      }
                      className={`p-3 rounded-lg border-2 transition-all duration-200 flex flex-col items-center space-y-2 ${
                        selectedCategory === category.id
                          ? 'border-blue-500 bg-blue-50'
                          : 'border-gray-200 hover:border-gray-300'
                      }`}
                    >
                      <span className="text-2xl">{category.icon}</span>
                      <span className="text-sm font-medium text-center">{category.name}</span>
                    </button>
                  ))}
                </div>
              </div>

              {/* 添加按钮 - 全宽 */}
              <div className="pt-2">
                <Button
                  onClick={handleAddItem}
                  disabled={!taskName.trim() || !deadline}
                  color="primary"
                  fill="solid"
                  size="large"
                  className="w-full"
                >
                  {/* <Plus className="w-5 h-5 mr-2" /> */}
                  添加任务
                </Button>
              </div>
            </div>
          </div>
        )}

        {/* Task List */}
        <div className="bg-white/70 backdrop-blur-sm rounded-2xl shadow-soft border border-white/50 p-6 animate-fade-in">
          <div className="flex items-center justify-between mb-6">
            <h2 className="text-lg font-semibold text-gray-800 flex items-center">
              <CheckCircle className="w-5 h-5 text-green-500 mr-2" />
              任务列表
            </h2>
            <div className="text-sm text-gray-500">共 {taskList.length} 个任务</div>
          </div>

          {taskList.length === 0 ? (
            <div className="text-center py-12">
              <div className="w-16 h-16 bg-gray-100 rounded-full flex items-center justify-center mx-auto mb-4">
                <svg
                  className="w-8 h-8 text-gray-400"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M9 5H7a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"
                  />
                </svg>
              </div>
              <p className="text-gray-500">暂无任务，开始添加您的第一个任务吧！</p>
            </div>
          ) : (
            <div className="space-y-3">
              {taskList.map((task, index) => {
                const category = getCategoryInfo(task.categoryId)
                return (
                  <div
                    key={task.id}
                    className={`group bg-white rounded-xl border border-gray-100 p-4 transition-all duration-200 hover:shadow-md hover:border-gray-200 animate-slide-up`}
                    style={{ animationDelay: `${index * 50}ms` }}
                  >
                    <div className="flex flex-col sm:flex-row sm:items-start sm:justify-between gap-3">
                      <div className="flex-1 min-w-0">
                        <div className="flex items-center space-x-3">
                          <button
                            onClick={() => handleUpdateFinishTask(task.id, !task.finished)}
                            className={`flex-shrink-0 w-6 h-6 rounded-full border-2 flex items-center justify-center transition-all duration-200 ${
                              task.finished
                                ? 'bg-green-500 border-green-500 text-white'
                                : 'border-gray-300 hover:border-green-400 hover:bg-green-50'
                            }`}
                          >
                            {task.finished && (
                              <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                                <path
                                  fillRule="evenodd"
                                  d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
                                  clipRule="evenodd"
                                />
                              </svg>
                            )}
                          </button>

                          <div className="flex-1 min-w-0">
                            <div className="flex flex-col sm:flex-row sm:items-center gap-2 mb-2">
                              <h3
                                className={`text-sm font-medium transition-all duration-200 ${
                                  task.finished ? 'text-gray-500 line-through' : 'text-gray-900'
                                }`}
                              >
                                {task.taskName}
                              </h3>
                              {category && (
                                <span
                                  className={`inline-flex items-center px-2 py-1 rounded-full text-xs font-medium text-white ${category.color} w-fit`}
                                >
                                  {category.icon} {category.name}
                                </span>
                              )}
                            </div>

                            <div className="flex flex-col sm:flex-row sm:items-center gap-2 sm:gap-4 text-xs text-gray-500">
                              <div className="flex items-center">
                                <svg
                                  className="w-3 h-3 mr-1"
                                  fill="none"
                                  stroke="currentColor"
                                  viewBox="0 0 24 24"
                                >
                                  <path
                                    strokeLinecap="round"
                                    strokeLinejoin="round"
                                    strokeWidth={2}
                                    d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"
                                  />
                                </svg>
                                <span className="truncate">
                                  {formatDateTime(task.taskStartTime)}
                                </span>
                              </div>
                              <div className="flex items-center">
                                <svg
                                  className="w-3 h-3 mr-1"
                                  fill="none"
                                  stroke="currentColor"
                                  viewBox="0 0 24 24"
                                >
                                  <path
                                    strokeLinecap="round"
                                    strokeLinejoin="round"
                                    strokeWidth={2}
                                    d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
                                  />
                                </svg>
                                <span className="truncate">{formatDateTime(task.taskEndTime)}</span>
                              </div>
                            </div>
                          </div>
                        </div>
                      </div>

                      <div className="flex items-center justify-end space-x-2 sm:ml-4">
                        {!task.finished && (
                          <Button
                            onClick={() => handleUpdateFinishTask(task.id, true)}
                            fill="none"
                            size="mini"
                            className="p-2 text-green-600 hover:bg-green-50 rounded-lg transition-colors duration-200"
                          >
                            <svg
                              className="w-4 h-4"
                              fill="none"
                              stroke="currentColor"
                              viewBox="0 0 24 24"
                            >
                              <path
                                strokeLinecap="round"
                                strokeLinejoin="round"
                                strokeWidth={2}
                                d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                              />
                            </svg>
                          </Button>
                        )}

                        <Button
                          onClick={() => handleDeleteTask(task.id)}
                          fill="none"
                          size="mini"
                          className="p-2 text-red-500 hover:bg-red-50 rounded-lg transition-colors duration-200"
                        >
                          <svg
                            className="w-4 h-4"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                          >
                            <path
                              strokeLinecap="round"
                              strokeLinejoin="round"
                              strokeWidth={2}
                              d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
                            />
                          </svg>
                        </Button>
                      </div>
                    </div>

                    {task.finished && (
                      <div className="mt-3 pt-3 border-t border-gray-100">
                        <div className="flex items-center text-xs text-green-600">
                          <svg className="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
                            <path
                              fillRule="evenodd"
                              d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                              clipRule="evenodd"
                            />
                          </svg>
                          已完成
                        </div>
                      </div>
                    )}
                  </div>
                )
              })}
            </div>
          )}
        </div>
      </div>
    </div>
  )
}

export default HomePage
