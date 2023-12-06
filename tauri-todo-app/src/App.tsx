import { useEffect, useRef, useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { exit } from '@tauri-apps/api/process'
import './App.css'
import { Button, Form, Input, TimePicker, message } from 'antd'
import dayjs from 'dayjs'

function App() {
  const [form] = Form.useForm()

  useEffect(() => {
    const container = document.getElementById('container')
    container!.addEventListener('contextmenu', function (event) {
      // 阻止默认的右键点击行为
      event.preventDefault()
    })
  }, [])

  const onFinish = (values: any) => {
    handleAddItem(values)
  }

  const handleAddItem = async (values: any) => {
    const { deadline, taskName } = values

    if (!deadline || !taskName) {
      message.error('数据无效')
    }

    const item = {
      taskName: taskName,
      deadline: dayjs(deadline).format('YYYY-MM-DD HH:mm:ss'),
    }

    console.log(item)
    await invoke('greet', item)
  }

  return (
    <div className="container" id="container">
      <Form layout="inline" form={form} onFinish={onFinish}>
        <Form.Item label="事项" name="taskName">
          <Input />
        </Form.Item>
        <Form.Item label="截止时间" name="deadline">
          <TimePicker style={{ width: 200 }} />
        </Form.Item>
        <Form.Item>
          <Button type="primary" htmlType="submit">
            添加
          </Button>
        </Form.Item>
      </Form>
    </div>
  )
}

export default App
