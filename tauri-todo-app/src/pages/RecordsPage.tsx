import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import {
  Calendar,
  ChevronLeft,
  ChevronRight,
  Tag,
  Plus,
  Edit,
  Trash2,
  CheckCircle,
  Clock,
} from 'lucide-react'
import { Button, Toast } from 'antd-mobile'
import dayjs from 'dayjs'

interface Task {
  id: number
  taskName: string
  taskStartTime: string
  taskEndTime: string
  finished: boolean
  categoryId?: number
}

// 事项分类数据
const taskCategories = [
  { id: 1, name: '工作', color: 'bg-blue-500', icon: '💼' },
  { id: 2, name: '学习', color: 'bg-green-500', icon: '📚' },
  { id: 3, name: '生活', color: 'bg-yellow-500', icon: '🏠' },
  { id: 4, name: '运动', color: 'bg-red-500', icon: '🏃' },
  { id: 5, name: '娱乐', color: 'bg-purple-500', icon: '🎮' },
]

const RecordsPage = () => {
  const [currentDate, setCurrentDate] = useState(dayjs())
  const [taskList, setTaskList] = useState<Task[]>([])
  const [selectedDate, setSelectedDate] = useState<dayjs.Dayjs | null>(null)
  const [showAddTask, setShowAddTask] = useState(false)
  const [editingTask, setEditingTask] = useState<Task | null>(null)

  useEffect(() => {
    invoke('get_tasks').then((res: any) => {
      setTaskList(res || [])
    })
  }, [])

  const getCategoryInfo = (categoryId?: number) => {
    if (!categoryId) return null
    return taskCategories.find((cat) => cat.id === categoryId)
  }

  const getTasksForDate = (date: dayjs.Dayjs) => {
    return taskList.filter((task) => {
      const taskDate = dayjs(task.taskStartTime)
      return taskDate.isSame(date, 'day')
    })
  }

  const getCategoryColor = (categoryId?: number) => {
    if (!categoryId) return 'bg-gray-400'
    const category = getCategoryInfo(categoryId)
    return category ? category.color : 'bg-gray-400'
  }

  const handleUpdateFinishTask = async (id: number, target: boolean) => {
    const res = await invoke('finish_task', { id, target })
    if (res) {
      Toast.show({
        content: target ? '任务已完成' : '任务已重新激活',
        position: 'center',
        icon: 'success',
      })
      // 刷新任务列表
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

  const handleDeleteTask = async (id: number) => {
    let res = await invoke('delete_task', { id: id })
    if (res) {
      Toast.show({
        content: '任务删除成功',
        position: 'center',
        icon: 'success',
      })
      // 刷新任务列表
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

  const handleAddTaskForDate = () => {
    if (!selectedDate) return
    // 这里可以跳转到首页添加任务，或者打开添加任务弹窗
    Toast.show({
      content: '请到首页添加新任务',
      position: 'center',
    })
  }

  const renderCalendar = () => {
    const startOfMonth = currentDate.startOf('month')
    const endOfMonth = currentDate.endOf('month')
    const startDate = startOfMonth.startOf('week')
    const endDate = endOfMonth.endOf('week')

    const days = []
    let dayIndex = 0

    while (dayIndex < 42) {
      // 最多6周，确保覆盖所有情况
      const currentDay = startDate.add(dayIndex, 'day')
      const tasksForDay = getTasksForDate(currentDay)
      const isCurrentMonth = currentDay.month() === currentDate.month()
      const isToday = currentDay.isSame(dayjs(), 'day')
      const isSelected = selectedDate ? currentDay.isSame(selectedDate, 'day') : false

      // 如果超出范围，停止循环
      if (currentDay.isAfter(endDate)) {
        break
      }

      days.push(
        <div
          key={currentDay.format('YYYY-MM-DD')}
          onClick={() => {
            const clickedDate = currentDay.clone()
            setSelectedDate(clickedDate)
          }}
          className={`min-h-[80px] p-2 border cursor-pointer transition-all duration-200 ${
            isSelected
              ? 'bg-blue-100 border-blue-400 shadow-md'
              : isCurrentMonth
              ? 'bg-white border-gray-100 hover:bg-gray-50'
              : 'bg-gray-50 border-gray-100 hover:bg-gray-100'
          } ${isToday ? 'border-blue-500' : ''}`}
        >
          <div className="text-sm font-medium text-gray-900 mb-1">{currentDay.format('D')}</div>

          {/* Task indicators */}
          <div className="space-y-1">
            {tasksForDay.slice(0, 3).map((task, index) => {
              const category = getCategoryInfo(task.categoryId)
              return (
                <div
                  key={task.id}
                  className={`h-2 rounded-full ${getCategoryColor(task.categoryId)} ${
                    task.finished ? 'opacity-50' : ''
                  }`}
                  title={`${task.taskName}${category ? ` (${category.name})` : ''}`}
                />
              )
            })}
            {tasksForDay.length > 3 && (
              <div className="text-xs text-gray-500 text-center">
                +{tasksForDay.length - 3} more
              </div>
            )}
            {tasksForDay.length === 0 && <div className="h-2 rounded-full bg-gray-100"></div>}
          </div>
        </div>
      )

      dayIndex++
    }

    return days
  }

  const selectedDateTasks = selectedDate ? getTasksForDate(selectedDate) : []

  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-50 via-white to-purple-50">
      {/* Header */}
      <div className="bg-white/80 backdrop-blur-md border-b border-gray-200/50 sticky top-0 z-10">
        <div className="max-w-6xl mx-auto px-6 py-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center space-x-3">
              <div className="w-10 h-10 bg-gradient-to-r from-green-500 to-blue-600 rounded-xl flex items-center justify-center">
                <Calendar className="w-6 h-6 text-white" />
              </div>
              <div>
                <h1 className="text-2xl font-bold bg-gradient-to-r from-green-600 to-blue-600 bg-clip-text text-transparent">
                  代办记录
                </h1>
                <p className="text-sm text-gray-500">日历视图查看任务安排</p>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div className="max-w-6xl mx-auto px-6 py-8">
        {/* Monthly Stats */}
        <div className="flex flex-col sm:flex-row gap-4 mb-6">
          <div className="flex-1 bg-white/70 backdrop-blur-sm rounded-xl p-4 border border-white/50">
            <div className="flex items-center space-x-3">
              <div className="w-10 h-10 bg-blue-100 rounded-lg flex items-center justify-center">
                <Calendar className="w-5 h-5 text-blue-600" />
              </div>
              <div>
                <p className="text-sm text-gray-600">本月任务</p>
                <p className="text-2xl font-bold text-blue-600">
                  {
                    taskList.filter((task) => {
                      const taskDate = dayjs(task.taskStartTime)
                      return (
                        taskDate.month() === currentDate.month() &&
                        taskDate.year() === currentDate.year()
                      )
                    }).length
                  }
                </p>
              </div>
            </div>
          </div>

          <div className="flex-1 bg-white/70 backdrop-blur-sm rounded-xl p-4 border border-white/50">
            <div className="flex items-center space-x-3">
              <div className="w-10 h-10 bg-green-100 rounded-lg flex items-center justify-center">
                <CheckCircle className="w-5 h-5 text-green-600" />
              </div>
              <div>
                <p className="text-sm text-gray-600">已完成</p>
                <p className="text-2xl font-bold text-green-600">
                  {
                    taskList.filter((task) => {
                      const taskDate = dayjs(task.taskStartTime)
                      return (
                        task.finished &&
                        taskDate.month() === currentDate.month() &&
                        taskDate.year() === currentDate.year()
                      )
                    }).length
                  }
                </p>
              </div>
            </div>
          </div>

          <div className="flex-1 bg-white/70 backdrop-blur-sm rounded-xl p-4 border border-white/50">
            <div className="flex items-center space-x-3">
              <div className="w-10 h-10 bg-yellow-100 rounded-lg flex items-center justify-center">
                <Clock className="w-5 h-5 text-yellow-600" />
              </div>
              <div>
                <p className="text-sm text-gray-600">待完成</p>
                <p className="text-2xl font-bold text-yellow-600">
                  {
                    taskList.filter((task) => {
                      const taskDate = dayjs(task.taskStartTime)
                      return (
                        !task.finished &&
                        taskDate.month() === currentDate.month() &&
                        taskDate.year() === currentDate.year()
                      )
                    }).length
                  }
                </p>
              </div>
            </div>
          </div>
        </div>

        {/* Calendar Navigation */}
        <div className="bg-white/70 backdrop-blur-sm rounded-2xl shadow-soft border border-white/50 p-6 mb-8">
          <div className="flex items-center justify-between mb-6">
            <button
              onClick={() => setCurrentDate(currentDate.subtract(1, 'month'))}
              className="p-2 hover:bg-gray-100 rounded-lg transition-colors duration-200"
            >
              <ChevronLeft className="w-5 h-5 text-gray-600" />
            </button>

            <h2 className="text-xl font-semibold text-gray-800">
              {currentDate.format('YYYY年 M月')}
            </h2>

            <button
              onClick={() => setCurrentDate(currentDate.add(1, 'month'))}
              className="p-2 hover:bg-gray-100 rounded-lg transition-colors duration-200"
            >
              <ChevronRight className="w-5 h-5 text-gray-600" />
            </button>
          </div>

          {/* Calendar Grid */}
          <div className="grid grid-cols-7 gap-px bg-gray-200 rounded-lg overflow-hidden">
            {/* Weekday headers */}
            {['日', '一', '二', '三', '四', '五', '六'].map((day) => (
              <div key={day} className="bg-gray-100 p-3 text-center">
                <div className="text-sm font-medium text-gray-600">{day}</div>
              </div>
            ))}

            {/* Calendar days */}
            {renderCalendar()}
          </div>
        </div>

        {/* Selected Date Details */}
        {selectedDate && (
          <div className="bg-white/70 backdrop-blur-sm rounded-2xl shadow-soft border border-white/50 p-6">
            <div className="flex items-center justify-between mb-4">
              <div>
                <h3 className="text-lg font-semibold text-gray-800">
                  {selectedDate.format('YYYY年M月D日')} 的任务
                </h3>
                <p className="text-sm text-gray-500 mt-1">{selectedDateTasks.length} 个任务</p>
              </div>
              <Button
                onClick={handleAddTaskForDate}
                color="primary"
                fill="outline"
                size="small"
                className="flex items-center space-x-2"
              >
                {/* <Plus className="w-4 h-4" /> */}
                <span>添加任务</span>
              </Button>
            </div>

            {selectedDateTasks.length === 0 ? (
              <div className="text-center py-8">
                <Calendar className="w-12 h-12 text-gray-300 mx-auto mb-3" />
                <p className="text-gray-500">这一天没有安排任务</p>
              </div>
            ) : (
              <div className="space-y-3">
                {selectedDateTasks.map((task) => {
                  const category = getCategoryInfo(task.categoryId)
                  return (
                    <div
                      key={task.id}
                      className={`p-4 rounded-xl border transition-all duration-200 hover:shadow-md ${
                        task.finished ? 'bg-gray-50 border-gray-200' : 'bg-white border-gray-100'
                      }`}
                    >
                      <div className="flex items-start justify-between">
                        <div className="flex-1">
                          <div className="flex items-center space-x-2 mb-2">
                            <h4
                              className={`font-medium ${
                                task.finished ? 'text-gray-500 line-through' : 'text-gray-900'
                              }`}
                            >
                              {task.taskName}
                            </h4>
                            {category && (
                              <span
                                className={`inline-flex items-center px-2 py-1 rounded-full text-xs font-medium text-white ${category.color}`}
                              >
                                {category.icon} {category.name}
                              </span>
                            )}
                            {task.finished && (
                              <span className="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium text-green-600 bg-green-100">
                                已完成
                              </span>
                            )}
                          </div>

                          <div className="text-sm text-gray-500">
                            <div className="flex items-center space-x-4">
                              <span className="flex items-center">
                                <Clock className="w-3 h-3 mr-1" />
                                {dayjs(task.taskStartTime).format('HH:mm')} -{' '}
                                {dayjs(task.taskEndTime).format('HH:mm')}
                              </span>
                            </div>
                          </div>
                        </div>

                        <div className="flex items-center space-x-2 ml-4">
                          {!task.finished && (
                            <Button
                              onClick={() => handleUpdateFinishTask(task.id, true)}
                              fill="none"
                              size="mini"
                              className="p-2 text-green-600 hover:bg-green-50 rounded-lg"
                            >
                              <CheckCircle className="w-4 h-4" />
                            </Button>
                          )}

                          <Button
                            onClick={() => handleDeleteTask(task.id)}
                            fill="none"
                            size="mini"
                            className="p-2 text-red-500 hover:bg-red-50 rounded-lg"
                          >
                            <Trash2 className="w-4 h-4" />
                          </Button>
                        </div>
                      </div>
                    </div>
                  )
                })}
              </div>
            )}
          </div>
        )}

        {/* Category Legend */}
        <div className="bg-white/70 backdrop-blur-sm rounded-2xl shadow-soft border border-white/50 p-6 mt-8">
          <h3 className="text-lg font-semibold text-gray-800 mb-4 flex items-center">
            <Tag className="w-5 h-5 text-purple-500 mr-2" />
            分类说明
          </h3>
          <div className="grid grid-cols-2 md:grid-cols-5 gap-3">
            {taskCategories.map((category) => (
              <div key={category.id} className="flex items-center space-x-2">
                <div className={`w-3 h-3 rounded-full ${category.color}`}></div>
                <span className="text-sm text-gray-600">{category.name}</span>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  )
}

export default RecordsPage
