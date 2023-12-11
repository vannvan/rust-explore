import { useEffect, useRef, useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { exit } from '@tauri-apps/api/process'
import './App.css'
import {
  Button,
  Form,
  Input,
  TimePicker,
  message,
  DatePicker,
  List,
  Typography,
  Table,
  Space,
} from 'antd'
import dayjs from 'dayjs'
import { CheckCircleOutlined, DeleteOutlined, PlusOutlined, RedoOutlined } from '@ant-design/icons'

const { RangePicker } = DatePicker

function App() {
  const [form] = Form.useForm()

  const [taskList, setTaskList] = useState<
    {
      taskName: string
      taskStartTime: string
      taskEndTime: string
      finished: boolean
    }[]
  >([])

  useEffect(() => {
    const container = document.getElementById('container')
    container!.addEventListener('contextmenu', function (event) {
      // 阻止默认的右键点击行为
      event.preventDefault()
    })

    invoke('get_tasks').then((res: any) => {
      setTaskList(res)
    })
  }, [])

  const onFinish = (values: any) => {
    handleAddItem(values)
  }

  const handleAddItem = async (values: any) => {
    const { deadline, taskName } = values
    if (!deadline || !taskName) {
      message.error('数据无效')
      return
    }

    const item = {
      taskName: taskName,
      deadline: [
        dayjs(deadline[0]).format('YYYY-MM-DD HH:mm:ss'),
        dayjs(deadline[1]).format('YYYY-MM-DD HH:mm:ss'),
      ],
    }

    let res = await invoke('add_task', item)
    if (res) {
      message.success('添加成功')
      form.resetFields()
      invoke('get_tasks').then((res: any) => {
        console.log(res)
        setTaskList(res)
      })
    } else {
      message.error('添加失败')
    }
  }

  const handleDeleteTask = async (id: number) => {
    let res = await invoke('delete_task', { id: id })
    if (res) {
      message.success('删除成功')
      invoke('get_tasks').then((res: any) => {
        setTaskList(res)
      })
    } else {
      message.error('删除失败')
    }
  }

  const handleUpdateFinishTask = async (id: number, target: boolean) => {
    const res = await invoke('finish_task', { id, target })
    if (res) {
      message.success('操作成功')
      invoke('get_tasks').then((res: any) => {
        setTaskList(res)
      })
    } else {
      message.error('操作失败')
    }
  }

  const columns = [
    {
      title: '任务',
      dataIndex: 'taskName',
      key: 'taskName',
    },
    {
      title: '开始时间',
      dataIndex: 'taskStartTime',
      key: 'taskStartTime',
    },
    {
      title: '结束时间',
      dataIndex: 'taskEndTime',
      key: 'taskEndTime',
    },
    {
      title: '状态',
      dataIndex: 'finished',
      key: 'finished',
      render: (finished: boolean) => (
        <Typography.Text type={finished ? 'success' : 'danger'}>
          {finished ? '已完成' : '未完成'}
        </Typography.Text>
      ),
    },
    {
      title: '操作',
      key: 'action',
      render: (item: any) => (
        <Space>
          {item.finished ? (
            <Button
              type="primary"
              danger
              size="small"
              icon={<RedoOutlined />}
              onClick={() => handleUpdateFinishTask(item.id, false)}
            ></Button>
          ) : (
            <Button
              type="primary"
              size="small"
              icon={<CheckCircleOutlined />}
              onClick={() => handleUpdateFinishTask(item.id, true)}
            ></Button>
          )}

          <Button
            color="primary"
            danger
            size="small"
            icon={<DeleteOutlined />}
            onClick={() => handleDeleteTask(item.id)}
          ></Button>
        </Space>
      ),
    },
  ]

  return (
    <div className="container" id="container">
      <Form layout="horizontal" form={form} onFinish={onFinish}>
        <Form.Item label="事项" name="taskName">
          <Input.TextArea
            maxLength={100}
            style={{ width: 368 }}
            autoSize={{ minRows: 3, maxRows: 5 }}
          />
        </Form.Item>
        <Form.Item label="周期" name="deadline">
          <RangePicker style={{ width: 368 }} showTime={{ format: 'HH:mm' }} />
        </Form.Item>
        <Form.Item>
          <Button type="primary" htmlType="submit" icon={<PlusOutlined rev={undefined} />}>
            添加
          </Button>
        </Form.Item>
      </Form>
      <div style={{ marginTop: 20 }}>
        {/* <List
          header={<div>待办清单</div>}
          footer={null}
          bordered
          dataSource={taskList}
          renderItem={(item, index) => <li style={{ padding: '6px 20px' }}>{item.taskName}</li>}
        /> */}
        <Table dataSource={taskList} columns={columns} style={{ background: 'transparent' }} />
      </div>
    </div>
  )
}

export default App
