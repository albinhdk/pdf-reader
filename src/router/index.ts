import { createRouter, createWebHashHistory } from 'vue-router'
import { ReadingHistory, PdfViewer } from '../components'

const routes = [
  {
    path: '/',
    name: 'Home',
    component: ReadingHistory
  },
  {
    path: '/viewer',
    name: 'PdfViewer',
    component: PdfViewer
  }
]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

export default router