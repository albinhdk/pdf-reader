<template>
  <div class="pdf-viewer" :class="{ 'dark-mode': isDarkMode }" @mousemove="onMouseMove">
    <!-- 工具栏 -->
    <div class="toolbar" :class="{ 'toolbar-hidden': !showToolbar }" @mouseenter="onToolbarMouseEnter" @mouseleave="onToolbarMouseLeave">
      <div class="toolbar-left">
        <button @click="openFile" class="btn btn-primary">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
            <polyline points="14,2 14,8 20,8"></polyline>
          </svg>
          打开PDF
        </button>
        <button v-if="pdfDocument" @click="closeCurrentPdf" class="btn btn-secondary">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M18 6L6 18"></path>
            <path d="M6 6l12 12"></path>
          </svg>
          关闭
        </button>
        <span v-if="fileName" class="file-name">{{ displayFileName }}</span>
      </div>
      
      <div class="toolbar-center" v-if="pdfDocument">
        <button @click="previousPage" :disabled="currentPage <= 1" class="btn btn-icon">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="15,18 9,12 15,6"></polyline>
          </svg>
        </button>
        
        <div class="page-info">
          <input 
            v-model.number="pageInput" 
            @keyup.enter="goToPage"
            @blur="goToPage"
            type="number" 
            :min="1" 
            :max="totalPages"
            class="page-input"
          />
          <span>/ {{ totalPages }}</span>
        </div>
        
        <button @click="nextPage" :disabled="currentPage >= totalPages" class="btn btn-icon">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="9,18 15,12 9,6"></polyline>
          </svg>
        </button>
      </div>
      
      <div class="toolbar-right" v-if="pdfDocument">
        <!-- 夜间模式切换 -->
        <button @click="toggleDarkMode" class="btn btn-icon" :class="{ active: isDarkMode }" title="夜间模式">
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
        
        <!-- 缩略图预览 -->
        <button @click="toggleThumbnails" class="btn btn-icon" :class="{ active: showThumbnails }" title="缩略图">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="3" y="3" width="7" height="7"></rect>
            <rect x="14" y="3" width="7" height="7"></rect>
            <rect x="14" y="14" width="7" height="7"></rect>
            <rect x="3" y="14" width="7" height="7"></rect>
          </svg>
        </button>
        
        <!-- 目录/大纲 -->
        <button @click="toggleOutline" class="btn btn-icon" :class="{ active: showOutline }" title="目录大纲">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="8" y1="6" x2="21" y2="6"></line>
            <line x1="8" y1="12" x2="21" y2="12"></line>
            <line x1="8" y1="18" x2="21" y2="18"></line>
            <line x1="3" y1="6" x2="3.01" y2="6"></line>
            <line x1="3" y1="12" x2="3.01" y2="12"></line>
            <line x1="3" y1="18" x2="3.01" y2="18"></line>
          </svg>
        </button>
        
        <!-- 书签功能 -->
        <button @click="toggleBookmark" class="btn btn-icon" :class="{ active: isCurrentPageBookmarked }" title="书签">
          <svg v-if="!isCurrentPageBookmarked" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M19 21l-7-5-7 5V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2z"></path>
          </svg>
          <svg v-else width="16" height="16" viewBox="0 0 24 24" fill="currentColor" stroke="currentColor" stroke-width="2">
            <path d="M19 21l-7-5-7 5V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2z"></path>
          </svg>
        </button>
        
        <!-- 书签列表 -->
        <button @click="toggleBookmarkList" class="btn btn-icon" :class="{ active: showBookmarkList }" title="书签列表">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M19 21l-7-5-7 5V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2z"></path>
            <line x1="9" y1="9" x2="15" y2="9"></line>
            <line x1="9" y1="13" x2="15" y2="13"></line>
          </svg>
        </button>
        
        <!-- 搜索功能 -->
        <div class="search-container">
          <button @click="toggleSearch" class="btn btn-icon" :class="{ active: showSearch }">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="11" cy="11" r="8"></circle>
              <path d="M21 21l-4.35-4.35"></path>
            </svg>
          </button>
          
          <div v-if="showSearch" class="search-box">
            <input 
              ref="searchInput"
              v-model="searchQuery" 
              @keyup.enter="searchNext"
              @input="performSearch"
              type="text" 
              placeholder="搜索文本..."
              class="search-input"
            />
            <div v-if="searchResults.length > 0" class="search-info">
              {{ currentSearchIndex + 1 }} / {{ searchResults.length }}
            </div>
            <button @click="searchPrevious" :disabled="searchResults.length === 0" class="btn btn-icon btn-small">
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <polyline points="15,18 9,12 15,6"></polyline>
              </svg>
            </button>
            <button @click="searchNext" :disabled="searchResults.length === 0" class="btn btn-icon btn-small">
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <polyline points="9,18 15,12 9,6"></polyline>
              </svg>
            </button>
          </div>
          
          <!-- 搜索结果面板 -->
          <div v-if="showSearch && searchResults.length > 0" class="search-results-panel">
            <div 
              v-for="(result, index) in searchResults.slice(0, 10)" 
              :key="`${result.pageNum}-${result.textIndex}`"
              @click="goToSearchResult(index)"
              class="search-result-item"
              :class="{ active: index === currentSearchIndex }"
            >
              <div class="search-result-page">第 {{ result.pageNum }} 页</div>
              <div class="search-result-text" v-html="highlightSearchText(result.text, searchQuery)"></div>
            </div>
            <div v-if="searchResults.length > 10" class="search-result-item" style="text-align: center; color: #999; font-style: italic;">
              还有 {{ searchResults.length - 10 }} 个结果...
            </div>
          </div>
        </div>
        
        <button @click="zoomOut" class="btn btn-icon">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="11" cy="11" r="8"></circle>
            <path d="M21 21l-4.35-4.35"></path>
            <line x1="8" y1="11" x2="14" y2="11"></line>
          </svg>
        </button>
        
        <span class="zoom-level">{{ Math.round(scale * 100) }}%</span>
        
        <button @click="zoomIn" class="btn btn-icon">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="11" cy="11" r="8"></circle>
            <path d="M21 21l-4.35-4.35"></path>
            <line x1="11" y1="8" x2="11" y2="14"></line>
            <line x1="8" y1="11" x2="14" y2="11"></line>
          </svg>
        </button>
        
        <button @click="fitToWidth" class="btn btn-secondary">适应宽度</button>
        <button @click="fitToPage" class="btn btn-secondary">适应页面</button>
      </div>
    </div>
    
    <!-- 侧边栏面板 -->
    <div class="sidebar" v-if="showThumbnails || showOutline || showBookmarkList" :class="{ 'dark-mode': isDarkMode }">
      <!-- 缩略图面板 -->
      <div v-if="showThumbnails" class="sidebar-panel thumbnails-panel">
        <div class="panel-header">
          <h3>缩略图</h3>
          <button @click="showThumbnails = false" class="btn btn-icon btn-small">
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        </div>
        <div class="thumbnails-container">
          <div 
            v-for="pageNum in totalPages" 
            :key="pageNum"
            @click="goToPageFromThumbnail(pageNum)"
            class="thumbnail-item"
            :class="{ active: pageNum === currentPage }"
          >
            <canvas 
              :ref="el => setThumbnailRef(el, pageNum)"
              class="thumbnail-canvas"
            ></canvas>
            <div class="thumbnail-page-num">{{ pageNum }}</div>
          </div>
        </div>
      </div>
      
      <!-- 目录大纲面板 -->
      <div v-if="showOutline" class="sidebar-panel outline-panel">
        <div class="panel-header">
          <h3>目录大纲</h3>
          <button @click="showOutline = false" class="btn btn-icon btn-small">
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        </div>
        <div class="outline-container">
          <div v-if="outline.length === 0" class="empty-outline">
            <p>此PDF文档没有目录信息</p>
          </div>
          <div v-else>
            <div 
              v-for="(item, index) in outline" 
              :key="index"
              @click="goToOutlineItem(item)"
              class="outline-item"
              :style="{ paddingLeft: (item.level * 16 + 12) + 'px' }"
            >
              <span class="outline-title">{{ item.title }}</span>
              <span class="outline-page">{{ item.pageNum }}</span>
            </div>
          </div>
        </div>
      </div>
      
      <!-- 书签列表面板 -->
      <div v-if="showBookmarkList" class="sidebar-panel bookmarks-panel">
        <div class="panel-header">
          <h3>书签列表</h3>
          <button @click="showBookmarkList = false" class="btn btn-icon btn-small">
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        </div>
        <div class="bookmarks-container">
          <div v-if="bookmarks.length === 0" class="empty-bookmarks">
            <p>还没有添加任何书签</p>
            <p>点击工具栏中的书签按钮添加当前页面</p>
          </div>
          <div v-else>
            <div 
              v-for="(bookmark, index) in bookmarks" 
              :key="index"
              class="bookmark-item"
              :class="{ active: bookmark.pageNum === currentPage }"
            >
              <div @click="goToBookmark(bookmark)" class="bookmark-content">
                <div class="bookmark-title">{{ bookmark.title }}</div>
                <div class="bookmark-page">第 {{ bookmark.pageNum }} 页</div>
                <div class="bookmark-date">{{ formatDate(bookmark.createdAt) }}</div>
              </div>
              <button @click="removeBookmark(index)" class="btn btn-icon btn-small bookmark-delete">
                <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <polyline points="3,6 5,6 21,6"></polyline>
                  <path d="M19,6v14a2,2 0 0,1 -2,2H7a2,2 0 0,1 -2,-2V6m3,0V4a2,2 0 0,1 2,-2h4a2,2 0 0,1 2,2v2"></path>
                </svg>
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
    
    <!-- PDF内容区域 -->
    <div class="pdf-content" ref="pdfContainer" :class="{ 'dark-mode': isDarkMode, 'with-sidebar': showThumbnails || showOutline || showBookmarkList }">
      <!-- 文档载入状态 -->
      <div v-if="loading && !pdfDocument" class="loading-state">
        <div class="loading-spinner"></div>
        <h3>正在载入PDF文档...</h3>
        <p>请稍候，正在解析文件内容</p>
        <div class="loading-tips">
          <p class="tip-text">💡 加载大文件可能需要较长时间，请耐心等待</p>
          <p class="tip-text">📋 加载时间取决于文件大小和系统性能</p>
        </div>
      </div>
      
      <!-- 空状态 -->
      <div v-else-if="!pdfDocument" class="empty-state">
        <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1">
          <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
          <polyline points="14,2 14,8 20,8"></polyline>
        </svg>
        <h3>选择一个PDF文件开始阅读</h3>
        <p>点击上方的"打开PDF"按钮选择文件</p>
      </div>
      
      <!-- PDF页面容器 -->
      <div v-else class="pdf-page-container">
        <canvas 
          ref="pdfCanvas" 
          class="pdf-canvas"
        ></canvas>
        
        <!-- 页面渲染载入状态 -->
        <div v-if="loading" class="loading-overlay">
          <div class="loading-spinner"></div>
          <p>正在渲染页面...</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, nextTick, toRaw, markRaw, onUnmounted, computed, type ComponentPublicInstance } from 'vue'
import { exists } from '@tauri-apps/plugin-fs'
import * as pdfjsLib from 'pdfjs-dist'
import { useRouter } from 'vue-router'

// PDF.js worker 配置 - 按照官方文档标准配置
pdfjsLib.GlobalWorkerOptions.workerSrc = 'https://unpkg.com/pdfjs-dist@5.3.31/build/pdf.worker.min.mjs'

// 获取路由实例
const router = useRouter()

// 响应式数据
const pdfDocument = ref<any>(null)
const currentPage = ref(1)
const totalPages = ref(0)
const scale = ref(1.0)
const fileName = ref('')
const loading = ref(false)
const pageInput = ref(1)

// 搜索相关数据
const showSearch = ref(false)
const searchQuery = ref('')
const searchResults = ref<Array<{pageNum: number, textIndex: number, text: string}>>([]) 

// 夜间模式
const isDarkMode = ref(false)

// 侧边栏面板控制
const showThumbnails = ref(false)
const showOutline = ref(false)
const showBookmarkList = ref(false)

// 缩略图相关
const thumbnailRefs = ref<Map<number, HTMLCanvasElement>>(new Map())
const thumbnailScale = 0.2 // 缩略图缩放比例
const thumbnailsRendered = ref<Set<number>>(new Set())

// 目录大纲
const outline = ref<Array<{title: string, pageNum: number, level: number, dest?: any}>>([]) 

// 书签相关
const bookmarks = ref<Array<{title: string, pageNum: number, createdAt: Date}>>([]) 
const isCurrentPageBookmarked = computed(() => {
  return bookmarks.value.some(bookmark => bookmark.pageNum === currentPage.value)
})
const currentSearchIndex = ref(-1)

// 显示文件名（从完整路径中提取）
const displayFileName = computed(() => {
  if (!fileName.value) return ''
  return fileName.value.split(/[\\/]/).pop() || fileName.value
})
const pageTextCache = ref<Map<number, string>>(new Map())

// 工具栏自动隐藏相关
const showToolbar = ref(true)
const toolbarHideTimer = ref<number | null>(null)
const isMouseOverToolbar = ref(false)

// DOM引用
const pdfCanvas = ref<HTMLCanvasElement>()
const pdfContainer = ref<HTMLDivElement>()
const searchInput = ref<HTMLInputElement>()

// 打开文件
const openFile = async () => {
  try {
    // 使用Tauri的文件对话框
    const { invoke } = await import('@tauri-apps/api/core')
    const filePath = await invoke('open_file_dialog')
    
    if (filePath && typeof filePath === 'string') {
      await loadPdf(filePath)
    }
  } catch (error) {
    console.error('打开文件失败:', error)
    // 如果Tauri方法失败，使用HTML5文件输入作为备选
    const input = document.createElement('input')
    input.type = 'file'
    input.accept = '.pdf'
    input.onchange = async (e) => {
      const file = (e.target as HTMLInputElement).files?.[0]
      if (file) {
        fileName.value = file.name
        const arrayBuffer = await file.arrayBuffer()
        await loadPdfFromBuffer(arrayBuffer)
      }
    }
    input.click()
  }
}

// 从文件路径加载PDF
const loadPdf = async (filePath: string) => {
  try {
    loading.value = true
    console.log('开始加载PDF文件，原始路径:', filePath)
    
    // 保存完整文件路径用于历史记录
    fileName.value = filePath
    console.log('保存的完整文件路径:', fileName.value)
    
    // 检查文件是否存在
    try {
      const fileExists = await exists(filePath)
      console.log('文件是否存在检查结果:', fileExists, '路径:', filePath)
      
      if (!fileExists) {
        throw new Error(`文件不存在: ${filePath}`)
      }
    } catch (fsError) {
      console.error('文件存在性检查失败:', fsError)
      // 即使文件检查失败，也尝试继续加载文件
      console.log('尝试继续加载文件，忽略文件存在性检查错误')
    }
    
    // 使用Tauri读取文件
    const { invoke } = await import('@tauri-apps/api/core')
    console.log('调用Tauri API读取文件...')
    
    try {
      const fileData = await invoke('read_pdf_file', { path: filePath }) as ArrayBuffer
      console.log('文件读取成功，数据大小:', fileData ? fileData.byteLength : 0)
      
      if (!fileData || fileData.byteLength === 0) {
        throw new Error('文件数据为空')
      }
      
      await loadPdfFromBuffer(fileData)
    } catch (invokeError) {
      console.error('Tauri API调用失败:', invokeError)
      throw new Error(`文件读取失败: ${(invokeError as Error).message}`)
    }
  } catch (error) {
    console.error('加载PDF失败:', error)
    console.error('错误详情:', (error as Error).stack)
    const errorMessage = error instanceof Error ? error.message : '未知错误'
    
    // 构建用户友好的错误信息
    let userMessage = `加载PDF文件失败: ${errorMessage}`
    let suggestions = ''
    
    // 根据错误类型提供不同的建议
    if (errorMessage.includes('内存不足')) {
      suggestions = '\n\n建议解决方案：\n• 关闭其他应用程序释放内存\n• 重启应用程序\n• 如果文件过大，可尝试使用PDF压缩工具'
    } else if (errorMessage.includes('较大') || errorMessage.includes('时间')) {
      suggestions = '\n\n大文件处理提示：\n• 文件正在加载中，请耐心等待\n• 加载时间取决于文件大小和系统性能\n• 建议在加载期间不要进行其他操作'
    } else if (errorMessage.includes('文件不存在')) {
      suggestions = '\n\n请检查：\n• 文件路径是否正确\n• 文件是否已被移动或删除\n• 是否有足够的文件访问权限'
    }
    
    // 使用Tauri的对话框插件显示错误
    try {
      const { message } = await import('@tauri-apps/plugin-dialog')
      await message(`${userMessage}${suggestions}`, {
        title: '文件加载错误',
        kind: 'error'
      })
    } catch (dialogError) {
      console.error('显示错误对话框失败:', dialogError)
      // 使用浏览器原生alert作为备选
      alert(`${userMessage}${suggestions}`)
    }
  } finally {
    loading.value = false
  }
}

// 从ArrayBuffer加载PDF - 按照官方Demo方式
const loadPdfFromBuffer = async (buffer: ArrayBuffer) => {
  try {
    loading.value = true
    console.log('开始加载PDF，buffer大小:', buffer.byteLength)
    
    // 检查文件大小限制
    const fileSizeMB = buffer.byteLength / (1024 * 1024)
    console.log('PDF文件大小:', fileSizeMB.toFixed(2), 'MB')
    
    // 对于超大文件给出警告
    if (fileSizeMB > 100) {
      console.warn('警告：文件较大，可能需要较长时间加载或导致内存不足')
    }
    
    // 检查ArrayBuffer大小限制（约2GB在32位系统，8GB在64位系统）
    const maxSize32bit = Math.pow(2, 31) - 1 // 约2GB
    if (buffer.byteLength >= maxSize32bit) {
      throw new Error(`文件过大 (${fileSizeMB.toFixed(2)}MB)，超出系统处理能力。建议使用文件大小小于2GB的PDF文件。`)
    }
    
    // 按照官方Demo的方式创建loadingTask
    const loadingTask = pdfjsLib.getDocument({ 
      data: new Uint8Array(buffer),
      cMapUrl: 'https://unpkg.com/pdfjs-dist@5.3.31/cmaps/',
      cMapPacked: true,
      // 添加大文件优化配置
      maxImageSize: fileSizeMB > 50 ? 16777216 : -1, // 16MB图片限制
      disableFontFace: fileSizeMB > 100, // 大文件禁用字体渲染优化
      useSystemFonts: fileSizeMB > 100
    })
    
    console.log('PDF.js版本:', pdfjsLib.version)
    console.log('LoadingTask创建成功:', loadingTask)
    
    // 使用promise方式获取document
    const pdfDoc = await loadingTask.promise
    console.log('PDF文档加载成功:', pdfDoc)
    console.log('PDF页数:', pdfDoc.numPages)
    // 使用 markRaw 防止 PDF 文档被 Vue 响应式代理包装
    pdfDocument.value = markRaw(pdfDoc)
    
    totalPages.value = pdfDocument.value.numPages
    currentPage.value = 1
    pageInput.value = 1
    
    // 等待DOM更新后再渲染
    await nextTick()
    await renderPage(1)
    
    // 默认适应宽度显示
    await fitToWidth()
  } catch (error) {
    console.error('解析PDF失败:', error)
    console.error('错误详情:', (error as Error).stack)
    
    const errorMessage = error instanceof Error ? error.message : '未知错误'
    let userMessage = 'PDF文件解析失败'
    let suggestions = ''
    
    // 根据错误类型提供具体建议
    if (errorMessage.includes('Invalid PDF') || errorMessage.includes('corrupted')) {
      suggestions = '\n\n可能原因：\n• 文件已损坏或不是有效的PDF文件\n• 文件下载不完整\n• 文件格式不受支持\n\n建议解决方案：\n• 重新下载或获取文件\n• 尝试用其他PDF阅读器打开\n• 检查文件扩展名是否正确'
    } else if (errorMessage.includes('password') || errorMessage.includes('encrypted')) {
      suggestions = '\n\n该PDF文件受密码保护，当前版本暂不支持加密文件。\n\n建议解决方案：\n• 使用其他工具移除密码保护\n• 获取未加密版本的文件'
    } else if (errorMessage.includes('memory') || errorMessage.includes('out of memory') || errorMessage.includes('Invalid array buffer length') || errorMessage.includes('RangeError')) {
      suggestions = '\n\n内存不足或文件过大，无法解析此PDF文件。\n\n建议解决方案：\n• 关闭其他应用程序释放内存\n• 重启应用程序\n• 使用PDF压缩工具减小文件大小\n• 尝试分割大文件为多个小文件\n• 在64位系统上运行以获得更大内存支持'
    } else if (errorMessage.includes('文件过大') || errorMessage.includes('超出系统处理能力')) {
      suggestions = '\n\n文件大小超出系统限制。\n\n建议解决方案：\n• 使用PDF压缩工具减小文件大小\n• 分割PDF为多个较小的文件\n• 在更高配置的设备上打开\n• 使用专业的PDF处理软件'
    } else {
      suggestions = '\n\n请检查：\n• 文件是否为有效的PDF格式\n• 文件是否完整未损坏\n• 文件大小是否过大\n• 尝试重新打开文件'
    }
    
    // 使用更友好的错误提示
    try {
      const { message } = await import('@tauri-apps/plugin-dialog')
      await message(`${userMessage}\n\n错误详情: ${errorMessage}${suggestions}`, {
        title: 'PDF解析错误',
        kind: 'error'
      })
    } catch (dialogError) {
      console.error('显示错误对话框失败:', dialogError)
      alert(`${userMessage}\n\n错误详情: ${errorMessage}${suggestions}`)
    }
  } finally {
    loading.value = false
  }
}

// 渲染页面 - 按照官方Demo方式
const renderPage = async (pageNum: number) => {
  if (!pdfDocument.value || !pdfCanvas.value) {
    console.log('渲染条件不满足:', { pdfDocument: !!pdfDocument.value, pdfCanvas: !!pdfCanvas.value })
    return
  }
  
  try {
    loading.value = true
    console.log('开始渲染页面:', pageNum)
    
    // 使用 toRaw 获取原始 PDF 文档对象，避免代理导致的私有字段访问错误
    const rawPdfDocument = toRaw(pdfDocument.value)
    console.log('获取原始PDF文档:', rawPdfDocument)
    // 按照官方Demo方式获取页面
    const page = await rawPdfDocument.getPage(pageNum)
    console.log('页面获取成功:', page)
    
    const canvas = pdfCanvas.value
    const context = canvas.getContext('2d')
    
    if (!context) {
      console.error('无法获取canvas context')
      return
    }
    
    // 计算视口 - 使用当前缩放比例
    console.log('当前缩放比例:', scale.value)
    const viewport = page.getViewport({ scale: scale.value })
    console.log('视口计算成功:', { width: viewport.width, height: viewport.height })
    
    // 设置canvas尺寸
    canvas.height = viewport.height
    canvas.width = viewport.width
    console.log('Canvas尺寸设置:', { width: canvas.width, height: canvas.height })
    
    // 清除之前的内容
    context.clearRect(0, 0, canvas.width, canvas.height)
    
    // 渲染页面 - 按照官方Demo方式
    const renderContext = {
      canvasContext: context,
      viewport: viewport
    }
    console.log('开始渲染页面到canvas...')
    
    const renderTask = page.render(renderContext)
    await renderTask.promise
    console.log('页面渲染完成')
    
    // 更新阅读历史
    updateReadingHistory(pageNum)
    
    currentPage.value = pageNum
    pageInput.value = pageNum
  } catch (error) {
    console.error('渲染页面失败:', error)
    console.error('错误堆栈:', (error as Error).stack)
    console.error('错误名称:', (error as Error).name)
    console.error('错误消息:', (error as Error).message)
  } finally {
    loading.value = false
  }
}

// 关闭当前PDF文档并返回到阅读历史页面
const closeCurrentPdf = () => {
  // 清理当前PDF文档
  if (pdfDocument.value) {
    pdfDocument.value.destroy()
    pdfDocument.value = null
  }
  
  // 重置状态
  fileName.value = ''
  currentPage.value = 1
  totalPages.value = 0
  pageInput.value = 1
  scale.value = 1.0
  
  // 清除搜索结果
  searchResults.value = []
  searchQuery.value = ''
  showSearch.value = false
  
  // 关闭所有侧边栏
  showThumbnails.value = false
  showOutline.value = false
  showBookmarkList.value = false
  
  // 返回到阅读历史页面
  router.push('/')
}

// 页面导航
const previousPage = async () => {
  if (currentPage.value > 1) {
    await renderPage(currentPage.value - 1)
    scrollToTop()
  }
}

const nextPage = async () => {
  if (currentPage.value < totalPages.value) {
    await renderPage(currentPage.value + 1)
    scrollToTop()
  }
}

const goToPage = async () => {
  const page = Math.max(1, Math.min(pageInput.value, totalPages.value))
  if (page !== currentPage.value) {
    await renderPage(page)
    scrollToTop()
  }
}

// 缩放控制
const zoomIn = async () => {
  scale.value = Math.min(scale.value * 1.2, 3.0)
  if (currentPage.value > 0) {
    await renderPage(currentPage.value)
  }
}

const zoomOut = async () => {
  scale.value = Math.max(scale.value / 1.2, 0.5)
  if (currentPage.value > 0) {
    await renderPage(currentPage.value)
  }
}

const fitToWidth = async () => {
  if (!pdfContainer.value || !pdfCanvas.value || !pdfDocument.value) return
  
  const containerWidth = pdfContainer.value.clientWidth - 40 // 减去padding
  const page = await pdfDocument.value.getPage(currentPage.value)
  const viewport = page.getViewport({ scale: 1.0 })
  
  scale.value = containerWidth / viewport.width
  await renderPage(currentPage.value)
}

const fitToPage = async () => {
  if (!pdfContainer.value || !pdfCanvas.value || !pdfDocument.value) return
  
  const containerWidth = pdfContainer.value.clientWidth - 40
  const containerHeight = pdfContainer.value.clientHeight - 40
  const page = await pdfDocument.value.getPage(currentPage.value)
  const viewport = page.getViewport({ scale: 1.0 })
  
  const scaleX = containerWidth / viewport.width
  const scaleY = containerHeight / viewport.height
  scale.value = Math.min(scaleX, scaleY)
  await renderPage(currentPage.value)
}

// 页面滚动控制
let scrollInterval: number | null = null
let isScrolling = false

const scrollUp = (fast = false) => {
  if (pdfContainer.value) {
    pdfContainer.value.scrollBy({
      top: fast ? -80 : -50,
      behavior: 'auto'
    })
  }
}

const scrollDown = (fast = false) => {
  if (pdfContainer.value) {
    pdfContainer.value.scrollBy({
      top: fast ? 80 : 50,
      behavior: 'auto'
    })
  }
}

// 开始连续滚动
const startContinuousScroll = (direction: 'up' | 'down') => {
  if (isScrolling) return
  
  isScrolling = true
  let scrollCount = 0
  
  const scroll = () => {
    if (!isScrolling) return
    
    scrollCount++
    const isFast = scrollCount > 5 // 5次后加速
    
    if (direction === 'up') {
      scrollUp(isFast)
    } else {
      scrollDown(isFast)
    }
  }
  
  // 立即执行一次
  scroll()
  
  // 设置定时器连续滚动，更高频率
  scrollInterval = window.setInterval(scroll, 50)
}

// 停止连续滚动
const stopContinuousScroll = () => {
  isScrolling = false
  if (scrollInterval) {
    clearInterval(scrollInterval)
    scrollInterval = null
  }
}

// 滚动到页面顶部
const scrollToTop = () => {
  if (pdfContainer.value) {
    pdfContainer.value.scrollTo({
      top: 0,
      behavior: 'smooth'
    })
  }
}

// 空格键滚动一页
const scrollPageDown = () => {
  if (pdfContainer.value) {
    const containerHeight = pdfContainer.value.clientHeight
    // 滚动约90%的容器高度，留一些重叠以便阅读连续性
    const scrollDistance = containerHeight * 0.9
    pdfContainer.value.scrollBy({
      top: scrollDistance,
      behavior: 'smooth'
    })
  }
}

// 搜索功能实现
// 切换搜索界面
const toggleSearch = () => {
  showSearch.value = !showSearch.value
  if (showSearch.value) {
    // 显示搜索框时自动聚焦
    nextTick(() => {
      searchInput.value?.focus()
    })
  } else {
    // 隐藏搜索框时清除搜索结果
    clearSearchResults()
  }
}

// 清除搜索结果
const clearSearchResults = () => {
  searchResults.value = []
  currentSearchIndex.value = -1
  searchQuery.value = ''
}

// 从PDF页面提取文本
const extractTextFromPage = async (pageNum: number): Promise<string> => {
  if (!pdfDocument.value) return ''
  
  // 检查缓存
  if (pageTextCache.value.has(pageNum)) {
    return pageTextCache.value.get(pageNum) || ''
  }
  
  try {
    const rawPdfDocument = toRaw(pdfDocument.value)
    const page = await rawPdfDocument.getPage(pageNum)
    const textContent = await page.getTextContent()
    
    // 提取文本项并合并
    const text = textContent.items
      .map((item: any) => item.str || '')
      .join(' ')
      .replace(/\s+/g, ' ')
      .trim()
    
    // 缓存文本
    pageTextCache.value.set(pageNum, text)
    return text
  } catch (error) {
    console.error(`提取第${pageNum}页文本失败:`, error)
    return ''
  }
}

// 执行搜索
const performSearch = async () => {
  if (!searchQuery.value.trim() || !pdfDocument.value) {
    searchResults.value = []
    currentSearchIndex.value = -1
    return
  }
  
  const query = searchQuery.value.trim().toLowerCase()
  const results: Array<{pageNum: number, textIndex: number, text: string}> = []
  
  try {
    // 搜索所有页面
    for (let pageNum = 1; pageNum <= totalPages.value; pageNum++) {
      const pageText = await extractTextFromPage(pageNum)
      const lowerPageText = pageText.toLowerCase()
      
      // 查找所有匹配项
      let index = 0
      while ((index = lowerPageText.indexOf(query, index)) !== -1) {
        // 获取匹配文本的上下文（前后各30个字符，但尽量在单词边界截断）
        const contextLength = 30
        let start = Math.max(0, index - contextLength)
        let end = Math.min(pageText.length, index + query.length + contextLength)
        
        // 尝试在单词边界截断
        if (start > 0) {
          const spaceIndex = pageText.lastIndexOf(' ', index)
          if (spaceIndex > start) start = spaceIndex + 1
        }
        
        if (end < pageText.length) {
          const spaceIndex = pageText.indexOf(' ', index + query.length + 10)
          if (spaceIndex !== -1 && spaceIndex < end + 20) end = spaceIndex
        }
        
        let context = pageText.substring(start, end).trim()
        
        // 添加省略号
        if (start > 0) context = '...' + context
        if (end < pageText.length) context = context + '...'
        
        results.push({
          pageNum,
          textIndex: index,
          text: context
        })
        
        index += query.length
      }
    }
    
    searchResults.value = results
    currentSearchIndex.value = results.length > 0 ? 0 : -1
    
    // 如果有搜索结果，跳转到第一个结果
    if (results.length > 0) {
      await goToSearchResult(0)
      await highlightSearchInPdf()
    }
  } catch (error) {
    console.error('搜索失败:', error)
  }
}

// 跳转到搜索结果
const goToSearchResult = async (index: number) => {
  if (index < 0 || index >= searchResults.value.length) return
  
  const result = searchResults.value[index]
  currentSearchIndex.value = index
  
  // 跳转到对应页面
  if (result.pageNum !== currentPage.value) {
    await renderPage(result.pageNum)
  }
  
  // 滚动到页面顶部（简化实现，实际应该滚动到具体位置）
  scrollToTop()
}

// 搜索下一个
const searchNext = async () => {
  if (searchResults.value.length === 0) return
  
  const nextIndex = (currentSearchIndex.value + 1) % searchResults.value.length
  await goToSearchResult(nextIndex)
}

// 搜索上一个
const searchPrevious = async () => {
  if (searchResults.value.length === 0) return
  
  const prevIndex = currentSearchIndex.value <= 0 
    ? searchResults.value.length - 1 
    : currentSearchIndex.value - 1
  await goToSearchResult(prevIndex)
}

// 高亮搜索文本
const highlightSearchText = (text: string, query: string): string => {
  if (!query.trim()) return text
  
  const regex = new RegExp(`(${escapeRegExp(query.trim())})`, 'gi')
  return text.replace(regex, '<span class="search-highlight">$1</span>')
}

// 转义正则表达式特殊字符
const escapeRegExp = (string: string): string => {
  return string.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
}

// 在PDF页面上高亮搜索结果
const highlightSearchInPdf = async () => {
  if (!pdfDocument.value || !searchQuery.value.trim() || searchResults.value.length === 0) {
    return
  }
  
  // 这里可以实现在PDF Canvas上的文本高亮
  // 由于PDF.js的文本层实现比较复杂，这里先实现基础版本
  // 实际项目中可以使用PDF.js的文本层API来实现精确的文本高亮
  console.log('PDF文本高亮功能待完善')
}

// ========== 夜间模式功能 ==========
// 切换夜间模式
const toggleDarkMode = () => {
  isDarkMode.value = !isDarkMode.value
  // 保存到本地存储
  localStorage.setItem('pdf-reader-dark-mode', isDarkMode.value.toString())
}

// ========== 工具栏自动隐藏功能 ==========
// 显示工具栏
const showToolbarTemporarily = () => {
  showToolbar.value = true
  resetHideTimer()
}

// 重置隐藏计时器
const resetHideTimer = () => {
  if (toolbarHideTimer.value) {
    clearTimeout(toolbarHideTimer.value)
  }
  
  // 如果鼠标不在工具栏上，3秒后隐藏工具栏
  if (!isMouseOverToolbar.value) {
    toolbarHideTimer.value = setTimeout(() => {
      showToolbar.value = false
    }, 3000)
  }
}

// 鼠标进入工具栏
const onToolbarMouseEnter = () => {
  isMouseOverToolbar.value = true
  showToolbar.value = true
  if (toolbarHideTimer.value) {
    clearTimeout(toolbarHideTimer.value)
    toolbarHideTimer.value = null
  }
}

// 鼠标离开工具栏
const onToolbarMouseLeave = () => {
  isMouseOverToolbar.value = false
  resetHideTimer()
}

// 鼠标移动时显示工具栏
const onMouseMove = () => {
  showToolbarTemporarily()
}

// ========== 缩略图功能 ==========
// 切换缩略图面板
const toggleThumbnails = async () => {
  showThumbnails.value = !showThumbnails.value
  if (showThumbnails.value && pdfDocument.value) {
    // 延迟渲染缩略图，避免阻塞UI
    await nextTick()
    renderVisibleThumbnails()
  }
}

// 设置缩略图canvas引用
const setThumbnailRef = (el: Element | ComponentPublicInstance | null, pageNum: number) => {
  if (el && 'getContext' in el) {
    thumbnailRefs.value.set(pageNum, el as HTMLCanvasElement)
  }
}

// 渲染可见的缩略图
const renderVisibleThumbnails = async () => {
  if (!pdfDocument.value) return
  
  // 渲染前几页的缩略图
  const pagesToRender = Math.min(10, totalPages.value)
  for (let i = 1; i <= pagesToRender; i++) {
    if (!thumbnailsRendered.value.has(i)) {
      await renderThumbnail(i)
    }
  }
}

// 渲染单个缩略图
const renderThumbnail = async (pageNum: number) => {
  if (!pdfDocument.value || thumbnailsRendered.value.has(pageNum)) return
  
  const canvas = thumbnailRefs.value.get(pageNum)
  if (!canvas) return
  
  try {
    const rawPdfDocument = toRaw(pdfDocument.value)
    const page = await rawPdfDocument.getPage(pageNum)
    const viewport = page.getViewport({ scale: thumbnailScale })
    
    canvas.height = viewport.height
    canvas.width = viewport.width
    
    const context = canvas.getContext('2d')
    if (!context) return
    
    const renderContext = {
      canvasContext: context,
      viewport: viewport
    }
    
    await page.render(renderContext).promise
    thumbnailsRendered.value.add(pageNum)
  } catch (error) {
    console.error(`渲染缩略图失败 (页面 ${pageNum}):`, error)
  }
}

// 从缩略图跳转到页面
const goToPageFromThumbnail = async (pageNum: number) => {
  await renderPage(pageNum)
  scrollToTop()
}

// ========== 目录大纲功能 ==========
// 切换目录大纲面板
const toggleOutline = async () => {
  showOutline.value = !showOutline.value
  if (showOutline.value && pdfDocument.value && outline.value.length === 0) {
    await loadOutline()
  }
}

// 加载PDF目录大纲
const loadOutline = async () => {
  if (!pdfDocument.value) return
  
  try {
    const rawPdfDocument = toRaw(pdfDocument.value)
    const outlineData = await rawPdfDocument.getOutline()
    
    if (outlineData) {
      outline.value = await parseOutline(outlineData, 0)
    }
  } catch (error) {
    console.error('加载目录大纲失败:', error)
  }
}

// 解析目录大纲数据
const parseOutline = async (outlineData: any[], level: number): Promise<any[]> => {
  const result = []
  
  for (const item of outlineData) {
    try {
      let pageNum = 1
      if (item.dest) {
        const rawPdfDocument = toRaw(pdfDocument.value)
        const destArray = Array.isArray(item.dest) ? item.dest : await rawPdfDocument.getDestination(item.dest)
        if (destArray && destArray[0]) {
          const pageRef = destArray[0]
          pageNum = await rawPdfDocument.getPageIndex(pageRef) + 1
        }
      }
      
      result.push({
        title: item.title || '未命名',
        pageNum,
        level,
        dest: item.dest
      })
      
      // 递归处理子项
      if (item.items && item.items.length > 0) {
        const children = await parseOutline(item.items, level + 1)
        result.push(...children)
      }
    } catch (error) {
      console.error('解析目录项失败:', error)
    }
  }
  
  return result
}

// 跳转到目录项
const goToOutlineItem = async (item: any) => {
  await renderPage(item.pageNum)
  scrollToTop()
}

// ========== 书签功能 ==========
// 切换当前页面的书签状态
const toggleBookmark = () => {
  if (isCurrentPageBookmarked.value) {
    // 移除书签
    const index = bookmarks.value.findIndex(bookmark => bookmark.pageNum === currentPage.value)
    if (index !== -1) {
      bookmarks.value.splice(index, 1)
    }
  } else {
    // 添加书签
    const title = `第 ${currentPage.value} 页 - ${fileName.value}`
    bookmarks.value.push({
      title,
      pageNum: currentPage.value,
      createdAt: new Date()
    })
  }
  
  // 保存到本地存储
  saveBookmarks()
}

// 切换书签列表面板
const toggleBookmarkList = () => {
  showBookmarkList.value = !showBookmarkList.value
}

// 跳转到书签
const goToBookmark = async (bookmark: any) => {
  await renderPage(bookmark.pageNum)
  scrollToTop()
}

// 移除书签
const removeBookmark = (index: number) => {
  bookmarks.value.splice(index, 1)
  saveBookmarks()
}

// 保存书签到本地存储
const saveBookmarks = () => {
  const bookmarksData = bookmarks.value.map(bookmark => ({
    ...bookmark,
    createdAt: bookmark.createdAt.toISOString()
  }))
  localStorage.setItem('pdf-reader-bookmarks', JSON.stringify(bookmarksData))
}

// 从本地存储加载书签
const loadBookmarks = () => {
  try {
    const saved = localStorage.getItem('pdf-reader-bookmarks')
    if (saved) {
      const bookmarksData = JSON.parse(saved)
      bookmarks.value = bookmarksData.map((bookmark: any) => ({
        ...bookmark,
        createdAt: new Date(bookmark.createdAt)
      }))
    }
  } catch (error) {
    console.error('加载书签失败:', error)
  }
}

// 格式化日期
const formatDate = (date: Date): string => {
  return date.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
}

// 键盘快捷键
const handleKeydown = (e: KeyboardEvent) => {
  // 如果是在输入框中，不处理快捷键
  if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) {
    return
  }
  
  if (!pdfDocument.value) return
  
  // 防止重复触发
  if (e.repeat && (e.key === 'ArrowUp' || e.key === 'ArrowDown')) {
    return
  }
  
  switch (e.key) {
    case 'ArrowLeft':
      e.preventDefault()
      previousPage()
      break
    case 'ArrowRight':
      e.preventDefault()
      nextPage()
      break
    case 'ArrowUp':
      e.preventDefault()
      startContinuousScroll('up')
      break
    case 'ArrowDown':
      e.preventDefault()
      startContinuousScroll('down')
      break
    case ' ': // 空格键
      e.preventDefault()
      scrollPageDown()
      break
    case '+':
    case '=':
      if (e.ctrlKey) {
        e.preventDefault()
        zoomIn()
      }
      break
    case '-':
      if (e.ctrlKey) {
        e.preventDefault()
        zoomOut()
      }
      break
    case 'f':
    case 'F':
      if (e.ctrlKey) {
        e.preventDefault()
        toggleSearch()
      }
      break
    case 'd':
    case 'D':
      if (e.ctrlKey) {
        e.preventDefault()
        toggleDarkMode()
      }
      break
    case 'b':
    case 'B':
      if (e.ctrlKey) {
        e.preventDefault()
        toggleBookmark()
      }
      break
    case 't':
    case 'T':
      if (e.ctrlKey) {
        e.preventDefault()
        toggleThumbnails()
      }
      break
    case 'o':
    case 'O':
      if (e.ctrlKey) {
        e.preventDefault()
        toggleOutline()
      }
      break
    case 'Escape':
      e.preventDefault()
      if (showSearch.value) {
        showSearch.value = false
        clearSearchResults()
      } else if (showThumbnails.value) {
        showThumbnails.value = false
      } else if (showOutline.value) {
        showOutline.value = false
      } else if (showBookmarkList.value) {
        showBookmarkList.value = false
      }
      break
  }
}

// 键盘释放事件
const handleKeyup = (e: KeyboardEvent) => {
  if (e.key === 'ArrowUp' || e.key === 'ArrowDown') {
    stopContinuousScroll()
  }
}

// 滚轮缩放
const handleWheel = (e: WheelEvent) => {
  if (!pdfDocument.value || !e.ctrlKey) return
  
  e.preventDefault()
  
  if (e.deltaY < 0) {
    // 滚轮向上，放大
    zoomIn()
  } else {
    // 滚轮向下，缩小
    zoomOut()
  }
}

onMounted(() => {
  document.addEventListener('keydown', handleKeydown)
  document.addEventListener('keyup', handleKeyup)
  document.addEventListener('wheel', handleWheel, { passive: false })
  
  // 初始化设置
  initializeSettings()
  
  // 初始化工具栏自动隐藏
  resetHideTimer()
})

// 清理
const cleanup = () => {
  stopContinuousScroll()
  document.removeEventListener('keydown', handleKeydown)
  document.removeEventListener('keyup', handleKeyup)
  document.removeEventListener('wheel', handleWheel)
  
  // 清理工具栏自动隐藏计时器
  if (toolbarHideTimer.value) {
    clearTimeout(toolbarHideTimer.value)
    toolbarHideTimer.value = null
  }
}

// 初始化设置
const initializeSettings = () => {
  // 加载夜间模式设置
  const savedDarkMode = localStorage.getItem('pdf-reader-dark-mode')
  if (savedDarkMode === 'true') {
    isDarkMode.value = true
  }
  
  // 加载书签
  loadBookmarks()
  
  // 检查是否有从阅读历史页面传递过来的PDF文件信息
  checkPdfToOpen()
}

// 检查是否有需要打开的PDF文件
const checkPdfToOpen = async () => {
  const pdfToOpen = sessionStorage.getItem('pdf-to-open')
  console.log('检查是否有需要打开的PDF文件:', pdfToOpen)
  
  if (pdfToOpen) {
    try {
      const pdfInfo = JSON.parse(pdfToOpen)
      console.log('解析PDF信息成功:', pdfInfo)
      
      if (pdfInfo.filePath) {
        console.log('准备加载PDF文件:', pdfInfo.filePath)
        
        // 检查文件是否存在
        try {
          const fileExists = await exists(pdfInfo.filePath)
          console.log('文件是否存在检查结果:', fileExists, '路径:', pdfInfo.filePath)
          
          if (!fileExists) {
            throw new Error(`文件不存在: ${pdfInfo.filePath}`)
          }
          
          // 文件存在，尝试加载
          await loadPdf(pdfInfo.filePath)
          console.log('PDF加载成功，准备渲染页面')
          
          if (pdfInfo.currentPage && pdfInfo.currentPage <= totalPages.value) {
            await renderPage(pdfInfo.currentPage)
            console.log('渲染到指定页面成功:', pdfInfo.currentPage)
          }
          
          // 默认适应宽度显示
          await fitToWidth()
        } catch (loadError) {
          console.error('加载PDF文件失败:', loadError)
          console.error('错误详情:', (loadError as Error).stack)
          
          // 使用Tauri的对话框插件显示错误
          try {
            const { message } = await import('@tauri-apps/plugin-dialog')
            await message(`无法打开文件: ${pdfInfo.filePath}\n\n错误信息: ${(loadError as Error).message || '未知错误'}`, {
              title: '文件打开错误',
              kind: 'error'
            })
          } catch (dialogError) {
            console.error('显示错误对话框失败:', dialogError)
            alert(`无法打开文件: ${pdfInfo.filePath}\n\n错误信息: ${(loadError as Error).message || '未知错误'}`)
          }
        }
      } else {
        console.error('PDF信息中没有文件路径')
      }
      
      // 清除会话存储中的数据
      sessionStorage.removeItem('pdf-to-open')
    } catch (error) {
      console.error('解析PDF信息失败:', error)
      alert('打开历史PDF失败: ' + ((error as Error).message || '未知错误'))
    }
  }
}

// 更新阅读历史
const updateReadingHistory = (pageNum: number) => {
  if (!pdfDocument.value || !fileName.value) return
  
  try {
    // 从本地存储获取现有的阅读历史
    const saved = localStorage.getItem('pdf-reader-history')
    let history = saved ? JSON.parse(saved) : []
    
    // 查找当前文件是否已在历史记录中
    const filePath = fileName.value
    const existingIndex = history.findIndex((item: any) => item.filePath === filePath)
    
    const historyItem = {
      title: filePath.split(/[\\/]/).pop() || filePath,
      filePath: filePath,
      currentPage: pageNum,
      totalPages: totalPages.value,
      lastRead: new Date().toISOString()
    }
    
    if (existingIndex !== -1) {
      // 更新现有记录
      history[existingIndex] = historyItem
    } else {
      // 添加新记录
      history.unshift(historyItem)
    }
    
    // 限制历史记录数量为20条
    if (history.length > 20) {
      history = history.slice(0, 20)
    }
    
    // 保存到本地存储
    localStorage.setItem('pdf-reader-history', JSON.stringify(history))
  } catch (error) {
    console.error('更新阅读历史失败:', error)
  }
}

// 组件卸载时清理
onUnmounted(cleanup)
</script>

<style scoped>
.pdf-viewer {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: #f5f5f5;
}

.toolbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background: white;
  border-bottom: 1px solid #e0e0e0;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 1000;
  transition: transform 0.3s ease-in-out;
}

.toolbar-hidden {
  transform: translateY(-100%);
}

.toolbar-left,
.toolbar-center,
.toolbar-right {
  display: flex;
  align-items: center;
  gap: 8px;
}

.btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 12px;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s;
}

文档.btn-primary {
  background-color: #0066cc;
  color: white;
}

.btn-secondary {
  background-color: #6c757d;
  color: white;
  margin-left: 8px;
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-primary {
  background: #007acc;
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background: #005a9e;
}

.btn-secondary {
  background: #f0f0f0;
  color: #333;
}

.btn-secondary:hover:not(:disabled) {
  background: #e0e0e0;
}

.btn-icon {
  padding: 8px;
  background: #f8f8f8;
  color: #666;
}

.btn-icon:hover:not(:disabled) {
  background: #e8e8e8;
}

.file-name {
  font-size: 14px;
  color: #666;
  max-width: 200px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.page-info {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 14px;
}

.page-input {
  width: 60px;
  padding: 4px 8px;
  border: 1px solid #ddd;
  border-radius: 4px;
  text-align: center;
  font-size: 14px;
}

.zoom-level {
  font-size: 14px;
  color: #666;
  min-width: 40px;
  text-align: center;
}

.pdf-content {
  flex: 1;
  overflow: auto;
  padding: 20px;
  padding-top: 80px; /* 为固定工具栏留出空间 */
  display: flex;
  justify-content: center;
  align-items: flex-start;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: #999;
  text-align: center;
}

.empty-state svg {
  margin-bottom: 16px;
  opacity: 0.5;
}

.empty-state h3 {
  margin: 0 0 8px 0;
  font-size: 18px;
}

.empty-state p {
  margin: 0;
  font-size: 14px;
}

.loading-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: #666;
  text-align: center;
}

.loading-state h3 {
  margin: 12px 0 8px 0;
  font-size: 18px;
}

.loading-tips {
  margin-top: 16px;
  padding: 12px;
  background: rgba(0, 123, 255, 0.1);
  border-radius: 8px;
  border-left: 3px solid #007bff;
}

.tip-text {
  margin: 4px 0;
  font-size: 13px;
  color: #555;
  line-height: 1.4;
}

.loading-state p {
  margin: 0;
  font-size: 14px;
  color: #666;
}

.pdf-page-container {
  position: relative;
  display: flex;
  justify-content: center;
}

.pdf-canvas {
  border: 1px solid #ddd;
  box-shadow: 0 4px 12px rgba(0,0,0,0.15);
  background: white;
  transform-origin: top center;
}

.loading-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(255,255,255,0.8);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: #666;
}

.loading-spinner {
  width: 32px;
  height: 32px;
  border: 3px solid #f3f3f3;
  border-top: 3px solid #007acc;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin-bottom: 12px;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

/* 搜索功能样式 */
.search-container {
  position: relative;
  display: flex;
  align-items: center;
  gap: 8px;
}

.search-box {
  position: absolute;
  top: 100%;
  right: 0;
  margin-top: 8px;
  background: white;
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0,0,0,0.15);
  padding: 12px;
  width: 450px;
  z-index: 1000;
  display: flex;
  align-items: center;
  gap: 8px;
}

.search-input {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid #ddd;
  border-radius: 6px;
  font-size: 14px;
  outline: none;
  transition: border-color 0.2s;
}

.search-input:focus {
  border-color: #007acc;
  box-shadow: 0 0 0 2px rgba(0, 122, 204, 0.1);
}

.search-info {
  font-size: 12px;
  color: #666;
  white-space: nowrap;
  min-width: 50px;
  text-align: center;
}

.btn-small {
  padding: 6px;
  min-width: 28px;
  height: 28px;
}

.btn.active {
  background: #007acc;
  color: white;
}

.btn.active:hover {
  background: #005a9e;
}

/* 搜索高亮样式 */
.search-highlight {
  background-color: #ffeb3b;
  color: #000;
  padding: 1px 2px;
  border-radius: 2px;
  font-weight: bold;
}

.search-highlight.current {
  background-color: #ff9800;
  color: white;
}

/* 搜索结果面板 */
.search-results-panel {
  position: absolute;
  top: 100%;
  right: 0;
  margin-top: 4px;
  background: white;
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0,0,0,0.15);
  width: 450px;
  max-height: 300px;
  overflow-y: auto;
  z-index: 999;
}

/* ========== 夜间模式样式 ========== */
.pdf-viewer.dark-mode {
  background-color: #1a1a1a;
  color: #e0e0e0;
}

.pdf-viewer.dark-mode .toolbar {
  background-color: #2d2d2d;
  border-bottom-color: #404040;
}

.pdf-viewer.dark-mode .btn {
  background-color: #404040;
  color: #e0e0e0;
  border-color: #555;
}

.pdf-viewer.dark-mode .btn:hover {
  background-color: #505050;
}

.pdf-viewer.dark-mode .btn.active {
  background-color: #0066cc;
}

.pdf-viewer.dark-mode .pdf-content.dark-mode {
  background-color: #1a1a1a;
}

.pdf-viewer.dark-mode .loading-state,
.pdf-viewer.dark-mode .empty-state {
  color: #e0e0e0;
}

/* ========== 侧边栏样式 ========== */
.sidebar {
  position: fixed;
  left: 0;
  top: 60px;
  width: 300px;
  height: calc(100vh - 60px);
  background: white;
  border-right: 1px solid #ddd;
  z-index: 100;
  overflow-y: auto;
}

.sidebar.dark-mode {
  background: #2d2d2d;
  border-right-color: #404040;
  color: #e0e0e0;
}

.pdf-content.with-sidebar {
  margin-left: 300px;
}

.sidebar-panel {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px;
  border-bottom: 1px solid #eee;
  background: #f8f9fa;
}

.sidebar.dark-mode .panel-header {
  background: #404040;
  border-bottom-color: #555;
}

.panel-header h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
}

/* ========== 缩略图样式 ========== */
.thumbnails-container {
  flex: 1;
  padding: 16px;
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
  gap: 12px;
  overflow-y: auto;
}

.thumbnail-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 8px;
  border: 2px solid transparent;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.thumbnail-item:hover {
  background-color: #f0f0f0;
  border-color: #ddd;
}

.thumbnail-item.active {
  border-color: #0066cc;
  background-color: #e6f3ff;
}

.sidebar.dark-mode .thumbnail-item:hover {
  background-color: #404040;
  border-color: #555;
}

.sidebar.dark-mode .thumbnail-item.active {
  border-color: #0066cc;
  background-color: #1a3d5c;
}

.thumbnail-canvas {
  border: 1px solid #ddd;
  border-radius: 4px;
  max-width: 100%;
  height: auto;
}

.sidebar.dark-mode .thumbnail-canvas {
  border-color: #555;
}

.thumbnail-page-num {
  margin-top: 8px;
  font-size: 12px;
  color: #666;
  font-weight: 500;
}

.sidebar.dark-mode .thumbnail-page-num {
  color: #ccc;
}

/* ========== 目录大纲样式 ========== */
.outline-container {
  flex: 1;
  overflow-y: auto;
}

.empty-outline {
  padding: 32px 16px;
  text-align: center;
  color: #999;
}

.outline-item {
  padding: 12px;
  border-bottom: 1px solid #f0f0f0;
  cursor: pointer;
  display: flex;
  justify-content: space-between;
  align-items: center;
  transition: background-color 0.2s ease;
}

.outline-item:hover {
  background-color: #f8f9fa;
}

.sidebar.dark-mode .outline-item {
  border-bottom-color: #404040;
}

.sidebar.dark-mode .outline-item:hover {
  background-color: #404040;
}

.outline-title {
  flex: 1;
  font-size: 14px;
  line-height: 1.4;
  margin-right: 8px;
}

.outline-page {
  font-size: 12px;
  color: #666;
  font-weight: 500;
}

.sidebar.dark-mode .outline-page {
  color: #ccc;
}

/* ========== 书签样式 ========== */
.bookmarks-container {
  flex: 1;
  overflow-y: auto;
}

.empty-bookmarks {
  padding: 32px 16px;
  text-align: center;
  color: #999;
}

.empty-bookmarks p {
  margin: 8px 0;
  font-size: 14px;
  line-height: 1.4;
}

.bookmark-item {
  display: flex;
  align-items: center;
  padding: 12px 16px;
  border-bottom: 1px solid #f0f0f0;
  transition: background-color 0.2s ease;
}

.bookmark-item:hover {
  background-color: #f8f9fa;
}

.bookmark-item.active {
  background-color: #e6f3ff;
  border-left: 3px solid #0066cc;
}

.sidebar.dark-mode .bookmark-item {
  border-bottom-color: #404040;
}

.sidebar.dark-mode .bookmark-item:hover {
  background-color: #404040;
}

.sidebar.dark-mode .bookmark-item.active {
  background-color: #1a3d5c;
}

.bookmark-content {
  flex: 1;
  cursor: pointer;
}

.bookmark-title {
  font-size: 14px;
  font-weight: 500;
  margin-bottom: 4px;
  line-height: 1.3;
}

.bookmark-page {
  font-size: 12px;
  color: #666;
  margin-bottom: 2px;
}

.bookmark-date {
  font-size: 11px;
  color: #999;
}

.sidebar.dark-mode .bookmark-page {
  color: #ccc;
}

.sidebar.dark-mode .bookmark-date {
  color: #aaa;
}

.bookmark-delete {
  opacity: 0;
  transition: opacity 0.2s ease;
  margin-left: 8px;
}

.bookmark-item:hover .bookmark-delete {
  opacity: 1;
}

.bookmark-delete:hover {
  background-color: #ff4757 !important;
  color: white !important;
}

.search-result-item {
  padding: 12px 16px;
  border-bottom: 1px solid #f0f0f0;
  cursor: pointer;
  transition: background-color 0.2s;
}

.search-result-item:hover {
  background-color: #f8f8f8;
}

.search-result-item.active {
  background-color: #e3f2fd;
}

.search-result-item:last-child {
  border-bottom: none;
}

.search-result-page {
  font-size: 12px;
  color: #666;
  margin-bottom: 4px;
}

.search-result-text {
  font-size: 14px;
  line-height: 1.5;
  word-break: break-word;
  white-space: normal;
}
</style>