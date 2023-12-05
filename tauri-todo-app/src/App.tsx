import { useEffect, useRef, useState } from 'react'
import reactLogo from './assets/react.svg'
import { invoke } from '@tauri-apps/api/tauri'
import './App.css'
import { Button, Form, Input, TimePicker } from 'antd'

function App() {
  const [greetMsg, setGreetMsg] = useState('')
  const [name, setName] = useState('')

  const [form] = Form.useForm()

  useEffect(() => {
    // 获取容器元素
    const container = document.getElementById('container')

    // 添加事件监听器
    container!.addEventListener('contextmenu', function (event) {
      // 阻止默认的右键点击行为
      event.preventDefault()
    })
  }, [])

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke('greet', { name }))
  }

  const onFinish = (values: any) => {
    console.log(values)
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
          <Button
            type="primary"
            htmlType="submit"
            onClick={() => {
              console.log(form.getFieldsValue())
            }}
          >
            添加
          </Button>
        </Form.Item>
      </Form>
    </div>
  )
}

export default App
