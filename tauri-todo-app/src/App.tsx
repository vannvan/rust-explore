import { BrowserRouter as Router, Routes, Route } from 'react-router-dom'
import { ConfigProvider } from 'antd-mobile'
import zhCN from 'antd-mobile/es/locales/zh-CN'
import './App.css'
import BottomNavigation from './components/BottomNavigation'
import HomePage from './pages/HomePage'
import RecordsPage from './pages/RecordsPage'
import ProfilePage from './pages/ProfilePage'

function App() {
  return (
    <ConfigProvider locale={zhCN}>
      <Router>
        <div className="min-h-screen bg-gradient-to-br from-blue-50 via-white to-purple-50 pb-20">
          <Routes>
            <Route path="/" element={<HomePage />} />
            <Route path="/records" element={<RecordsPage />} />
            <Route path="/profile" element={<ProfilePage />} />
          </Routes>
          <BottomNavigation />
        </div>
      </Router>
    </ConfigProvider>
  )
}

export default App
