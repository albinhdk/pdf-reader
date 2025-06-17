<template>
  <div class="reading-history" :class="{ 'dark-mode': isDarkMode }">
    <div class="history-header">
      <h2>我的阅读历史</h2>
      <div class="header-actions">
        <button @click="toggleDarkMode" class="btn btn-icon" :title="isDarkMode ? '切换到浅色模式' : '切换到夜间模式'">
          <svg v-if="!isDarkMode" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
          </svg>
          <svg v-else width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="5"></circle>
            <line x1="12" y1="1" x2="12" y2="3"></line>
            <line x1="12" y1="21" x2="12" y2="23"></line>
            <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line>
            <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line>
            <line x1="1" y1="12" x2="3" y2="12"></line>
            <line x1="21" y1="12" x2="23" y2="12"></line>
            <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line>
            <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line>
          </svg>
        </button>
        <button @click="openPdfViewer" class="btn btn-primary">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
            <polyline points="14,2 14,8 20,8"></polyline>
          </svg>
          打开新PDF
        </button>
      </div>
    </div>
    
    <div v-if="readingHistory.length === 0" class="empty-history">
      <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1">
        <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
        <polyline points="14,2 14,8 20,8"></polyline>
      </svg>
      <h3>暂无阅读记录</h3>
      <p>点击"打开新PDF"按钮开始阅读</p>
    </div>
    
    <div v-else class="history-list">
      <div 
        v-for="(item, index) in readingHistory" 
        :key="index"
        class="history-item"
        @click="openPdf(item)"
      >
        <div class="book-info">
          <h3 class="book-title">{{ item.title }}</h3>
          <div class="book-path">{{ formatPath(item.filePath) }}</div>
          <div class="book-meta">
            <span class="page-info">页码: {{ item.currentPage }} / {{ item.totalPages }}</span>
            <span class="last-read">上次阅读: {{ formatDate(item.lastRead) }}</span>
          </div>
        </div>
        
        <div class="progress-container">
          <div class="progress-bar">
            <div 
              class="progress-fill" 
              :style="{ width: calculateProgress(item) + '%' }"
              :title="`阅读进度: ${calculateProgress(item)}%`"
            ></div>
          </div>
          <div class="progress-text">{{ calculateProgress(item) }}%</div>
        </div>
        
        <div class="item-actions">
          <button @click.stop="removeHistoryItem(index)" class="btn btn-icon btn-small" title="从历史记录中移除">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <polyline points="3,6 5,6 21,6"></polyline>
              <path d="M19,6v14a2,2 0 0,1 -2,2H7a2,2 0 0,1 -2,-2V6m3,0V4a2,2 0 0,1 2,-2h4a2,2 0 0,1 2,2v2"></path>
            </svg>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'

// 定义阅读历史项的类型
interface ReadingHistoryItem {
  title: string
  filePath: string
  currentPage: number
  totalPages: number
  lastRead: Date
}

// 响应式数据
const readingHistory = ref<ReadingHistoryItem[]>([])
const isDarkMode = ref(false)

// 计算阅读进度百分比
const calculateProgress = (item: ReadingHistoryItem): number => {
  if (item.totalPages === 0) return 0
  return Math.round((item.currentPage / item.totalPages) * 100)
}

// 格式化文件路径，只显示文件名
const formatPath = (path: string): string => {
  return path.split(/[\\/]/).pop() || path
}

// 格式化日期
const formatDate = (date: Date): string => {
  return new Date(date).toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
}

// 从本地存储加载阅读历史
const loadReadingHistory = () => {
  try {
    const saved = localStorage.getItem('pdf-reader-history')
    if (saved) {
      const historyData = JSON.parse(saved)
      readingHistory.value = historyData.map((item: any) => ({
        ...item,
        lastRead: new Date(item.lastRead)
      })).sort((a: ReadingHistoryItem, b: ReadingHistoryItem) => b.lastRead.getTime() - a.lastRead.getTime()) // 按最新时间排序
    }
  } catch (error) {
    console.error('加载阅读历史失败:', error)
  }
}

// 保存阅读历史到本地存储
const saveReadingHistory = () => {
  const historyData = readingHistory.value.map(item => ({
    ...item,
    lastRead: item.lastRead.toISOString()
  }))
  localStorage.setItem('pdf-reader-history', JSON.stringify(historyData))
}

// 移除历史记录项
const removeHistoryItem = (index: number) => {
  readingHistory.value.splice(index, 1)
  saveReadingHistory()
}

// 打开PDF阅读器 - 直接调用文件选择对话框
const openPdfViewer = async () => {
  try {
    // 使用Tauri的文件对话框
    const { invoke } = await import('@tauri-apps/api/core')
    const filePath = await invoke('open_file_dialog')
    
    if (filePath) {
      // 将选中的PDF信息存储到sessionStorage
      sessionStorage.setItem('pdf-to-open', JSON.stringify({
        filePath: filePath,
        currentPage: 1
      }))
      
      console.log('已选择文件，准备跳转到阅读器:', filePath)
      
      // 导航到PDF阅读器页面
      window.location.href = '#/viewer'
    }
  } catch (error) {
    console.error('打开文件对话框失败:', error)
    
    // 如果Tauri方法失败，使用HTML5文件输入作为备选
    const input = document.createElement('input')
    input.type = 'file'
    input.accept = '.pdf'
    input.onchange = async (e) => {
      const file = (e.target as HTMLInputElement).files?.[0]
      if (file) {
        // 对于HTML5文件选择，我们需要将文件内容转换为可用的格式
        // 这里我们直接跳转到阅读器，让阅读器处理文件
        sessionStorage.setItem('pdf-file-object', JSON.stringify({
          name: file.name,
          size: file.size,
          type: file.type
        }))
        
        // 将文件对象存储到一个临时的全局变量中
        ;(window as any).tempPdfFile = file
        
        console.log('已选择HTML5文件，准备跳转到阅读器:', file.name)
        window.location.href = '#/viewer'
      }
    }
    input.click()
  }
}

// 打开特定的PDF文件
const openPdf = async (item: ReadingHistoryItem) => {
  console.log('准备打开PDF文件:', item)
  
  try {
    // 确保文件路径格式正确，特别是在Windows系统上
    let normalizedPath = item.filePath
    console.log('规范化前的文件路径:', normalizedPath)
    
    // 使用Tauri API检查文件是否存在
    const { exists } = await import('@tauri-apps/plugin-fs')
    
    // 检查文件是否存在
    const fileExists = await exists(normalizedPath)
    console.log('文件是否存在:', fileExists, '路径:', normalizedPath)
    
    if (!fileExists) {
      throw new Error(`文件不存在: ${normalizedPath}`)
    }
    
    // 文件存在，将选中的PDF信息存储到sessionStorage
    sessionStorage.setItem('pdf-to-open', JSON.stringify({
      filePath: normalizedPath,
      currentPage: item.currentPage
    }))
    
    console.log('已将PDF信息存储到sessionStorage，准备跳转到阅读器')
    
    // 导航到PDF阅读器页面
    window.location.href = '#/viewer'
    
  } catch (error) {
    console.error('打开PDF文件失败:', error)
    console.error('错误详情:', (error as Error).stack)
    
    // 在阅读历史界面显示错误提示，不跳转页面
    try {
      const { message } = await import('@tauri-apps/plugin-dialog')
      await message(`无法打开文件: ${item.filePath}\n\n错误信息: ${(error as Error).message || '未知错误'}`, {
        title: '文件打开错误',
        kind: 'error'
      })
    } catch (dialogError) {
      console.error('显示错误对话框失败:', dialogError)
      // 使用浏览器原生alert作为备选
      alert(`无法打开文件: ${item.filePath}\n\n错误信息: ${(error as Error).message || '未知错误'}`)
    }
    // 不跳转页面，保持在阅读历史界面
  }
}

// 切换夜间模式
const toggleDarkMode = () => {
  isDarkMode.value = !isDarkMode.value
  localStorage.setItem('pdf-reader-dark-mode', isDarkMode.value.toString())
}

// 初始化设置
const initializeSettings = () => {
  // 加载夜间模式设置
  const savedDarkMode = localStorage.getItem('pdf-reader-dark-mode')
  if (savedDarkMode === 'true') {
    isDarkMode.value = true
  }
  
  // 加载阅读历史
  loadReadingHistory()
}

onMounted(() => {
  initializeSettings()
})
</script>

<style scoped>
.reading-history {
  padding: 2rem;
  max-width: 1200px;
  margin: 0 auto;
  min-height: 100vh;
  max-height: 100vh;
  background-color: #f5f5f5;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  color: #333;
  transition: background-color 0.3s, color 0.3s;
}

.reading-history.dark-mode {
  background-color: #1e1e1e;
  color: #e0e0e0;
}

.history-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
  padding-bottom: 1rem;
  border-bottom: 1px solid #ddd;
}

.dark-mode .history-header {
  border-bottom-color: #444;
}

.header-actions {
  display: flex;
  gap: 1rem;
}

.empty-history {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 4rem 0;
  text-align: center;
  color: #666;
}

.dark-mode .empty-history {
  color: #999;
}

.empty-history svg {
  margin-bottom: 1rem;
  stroke: #999;
}

.dark-mode .empty-history svg {
  stroke: #777;
}

.history-list {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  overflow-y: auto;
  flex: 1;
  padding-right: 0.5rem;
}

.history-list::-webkit-scrollbar {
  width: 8px;
}

.history-list::-webkit-scrollbar-track {
  background: #f1f1f1;
  border-radius: 4px;
}

.history-list::-webkit-scrollbar-thumb {
  background: #c1c1c1;
  border-radius: 4px;
}

.history-list::-webkit-scrollbar-thumb:hover {
  background: #a8a8a8;
}

.dark-mode .history-list::-webkit-scrollbar-track {
  background: #2d2d2d;
}

.dark-mode .history-list::-webkit-scrollbar-thumb {
  background: #555;
}

.dark-mode .history-list::-webkit-scrollbar-thumb:hover {
  background: #777;
}

.history-item {
  display: flex;
  flex-direction: column;
  padding: 1.5rem;
  background-color: #fff;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  cursor: pointer;
  transition: transform 0.2s, box-shadow 0.2s;
  position: relative;
}

.dark-mode .history-item {
  background-color: #2d2d2d;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
}

.history-item:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.dark-mode .history-item:hover {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4);
}

.book-info {
  margin-bottom: 1rem;
}

.book-title {
  font-size: 1.25rem;
  margin-bottom: 0.5rem;
  font-weight: 600;
  color: #333;
}

.dark-mode .book-title {
  color: #e0e0e0;
}

.book-path {
  font-size: 0.875rem;
  color: #666;
  margin-bottom: 0.75rem;
  word-break: break-all;
}

.dark-mode .book-path {
  color: #aaa;
}

.book-meta {
  display: flex;
  justify-content: space-between;
  font-size: 0.875rem;
  color: #666;
}

.dark-mode .book-meta {
  color: #999;
}

.progress-container {
  display: flex;
  align-items: center;
  gap: 1rem;
  margin-bottom: 0.5rem;
}

.progress-bar {
  flex: 1;
  height: 8px;
  background-color: #eee;
  border-radius: 4px;
  overflow: hidden;
}

.dark-mode .progress-bar {
  background-color: #444;
}

.progress-fill {
  height: 100%;
  background-color: #4caf50;
  border-radius: 4px;
  transition: width 0.3s;
}

.progress-text {
  font-size: 0.875rem;
  color: #666;
  min-width: 40px;
  text-align: right;
}

.dark-mode .progress-text {
  color: #999;
}

.item-actions {
  position: absolute;
  top: 1rem;
  right: 1rem;
  opacity: 0;
  transition: opacity 0.2s;
}

.history-item:hover .item-actions {
  opacity: 1;
}

.btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  padding: 0.5rem 1rem;
  border: none;
  border-radius: 4px;
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.2s, color 0.2s;
}

.btn-primary {
  background-color: #007acc;
  color: white;
}

.btn-primary:hover {
  background-color: #0069b3;
}

.dark-mode .btn-primary {
  background-color: #0078d4;
}

.dark-mode .btn-primary:hover {
  background-color: #0086ef;
}

.btn-icon {
  padding: 0.5rem;
  border-radius: 50%;
  background-color: transparent;
  color: #666;
}

.btn-icon:hover {
  background-color: rgba(0, 0, 0, 0.05);
  color: #333;
}

.dark-mode .btn-icon {
  color: #aaa;
}

.dark-mode .btn-icon:hover {
  background-color: rgba(255, 255, 255, 0.1);
  color: #e0e0e0;
}

.btn-small {
  padding: 0.25rem;
}

.btn svg {
  stroke: currentColor;
}

@media (min-width: 768px) {
  .history-item {
    flex-direction: row;
    align-items: center;
    gap: 1.5rem;
  }
  
  .book-info {
    flex: 1;
    margin-bottom: 0;
  }
  
  .progress-container {
    width: 30%;
    margin-bottom: 0;
  }
}
</style>