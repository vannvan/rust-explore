import { useRef, useState } from 'react'
import reactLogo from './assets/react.svg'
import { invoke } from '@tauri-apps/api/tauri'
import './App.css'
import { Button, Form, Input, TimePicker } from 'antd'

function App() {
  const [greetMsg, setGreetMsg] = useState('')
  const [name, setName] = useState('')

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke('greet', { name }))
  }

  return (
    <div className="container">
      <Form layout="inline">
        <Form.Item label="事项" name="thingName">
          <Input />
        </Form.Item>
        <Form.Item label="截止时间" name="deadline">
          <TimePicker />
        </Form.Item>
        <Form.Item>
          <Button type="primary">添加</Button>
        </Form.Item>
      </Form>
    </div>
  )
}

export default App
