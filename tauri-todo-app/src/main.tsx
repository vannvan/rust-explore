import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import './style.css'
import zhCN from 'antd/locale/zh_CN'
import { ConfigProvider } from 'antd'

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <ConfigProvider locale={zhCN}>
      <App />
    </ConfigProvider>
  </React.StrictMode>
)
