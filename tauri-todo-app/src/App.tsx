import { useEffect, useRef, useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { exit } from '@tauri-apps/api/process'
import './App.css'
import { Button, Form, Input, TimePicker, message, DatePicker, List, Typography } from 'antd'
import dayjs from 'dayjs'
import { PlusOutlined } from '@ant-design/icons'

const { RangePicker } = DatePicker

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
      return
    }

    const item = {
      taskName: taskName,
      deadline: [
        dayjs(deadline[0]).format('YYYY-MM-DD HH:mm:ss'),
        dayjs(deadline[1]).format('YYYY-MM-DD HH:mm:ss'),
      ],
    }

    console.log(item)
    await invoke('add_task', item)
  }

  const data = [
    'Racing car sprays burning fuel into crowd.',
    'Japanese princess to wed commoner.',
    'Australian walks 100km after outback crash.',
    'Man charged over missing wedding girl.',
    'Los Angeles battles huge wildfires.',
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
        <List
          header={<div>待办清单</div>}
          footer={null}
          bordered
          dataSource={data}
          renderItem={(item) => (
            <List.Item>
              <Typography.Text mark>[ITEM]</Typography.Text> {item}
            </List.Item>
          )}
        />
      </div>
    </div>
  )
}

export default App
