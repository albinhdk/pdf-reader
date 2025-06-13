# PDF阅读器 (PDF Reader)

一个基于Tauri、Vue 3和PDF.js构建的现代化PDF阅读器应用，提供丰富的功能和优秀的用户体验。

## 功能特性

### 核心功能

- **PDF文件打开和显示**：支持从本地打开PDF文件并高质量渲染
- **页面导航**：支持前进、后退、跳转到指定页面
- **缩放控制**：支持放大、缩小、适应宽度、适应页面
- **全文搜索**：支持在整个PDF文档中搜索文本，并高亮显示结果

### 用户体验优化

#### 🌙 夜间模式
- **功能**：一键切换深色主题，保护眼睛
- **操作**：点击工具栏月亮图标或使用快捷键 `Ctrl+D`
- **特色**：自动保存设置，下次打开时记住偏好

#### 📑 缩略图预览
- **功能**：侧边栏显示所有页面的缩略图
- **操作**：点击工具栏网格图标或使用快捷键 `Ctrl+T`
- **特色**：智能渲染前10页，点击缩略图快速跳转

#### 📋 目录/大纲显示
- **功能**：自动提取PDF文档的目录结构
- **操作**：点击工具栏列表图标或使用快捷键 `Ctrl+O`
- **特色**：层级显示，支持多级目录导航

#### 🔖 书签功能
- **功能**：收藏重要页面，建立个人阅读记录
- **操作**：
  - 添加书签：点击书签图标或使用快捷键 `Ctrl+B`
  - 查看书签：点击书签列表图标
- **特色**：自动保存到本地，显示创建时间

## 快捷键

| 功能 | 快捷键 | 说明 |
|------|--------|------|
| 夜间模式 | `Ctrl+D` | 切换深色/浅色主题 |
| 缩略图 | `Ctrl+T` | 显示/隐藏缩略图面板 |
| 目录大纲 | `Ctrl+O` | 显示/隐藏目录面板 |
| 书签 | `Ctrl+B` | 添加/移除当前页书签 |
| 搜索 | `Ctrl+F` | 打开搜索功能 |
| 关闭面板 | `Escape` | 关闭当前打开的面板 |
| 上一页 | `←` 或 `↑` 或 `PageUp` | 导航到上一页 |
| 下一页 | `→` 或 `↓` 或 `PageDown` | 导航到下一页 |
| 首页 | `Home` | 跳转到第一页 |
| 末页 | `End` | 跳转到最后一页 |
| 滚动 | `Space` | 向下滚动一屏 |

## 开发信息

This template should help get you started developing with Vue 3 and TypeScript in Vite. The template uses Vue 3 `<script setup>` SFCs, check out the [script setup docs](https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup) to learn more.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Type Support For `.vue` Imports in TS

Since TypeScript cannot handle type information for `.vue` imports, they are shimmed to be a generic Vue component type by default. In most cases this is fine if you don't really care about component prop types outside of templates. However, if you wish to get actual prop types in `.vue` imports (for example to get props validation when using manual `h(...)` calls), you can enable Volar's Take Over mode by following these steps:

1. Run `Extensions: Show Built-in Extensions` from VS Code's command palette, look for `TypeScript and JavaScript Language Features`, then right click and select `Disable (Workspace)`. By default, Take Over mode will enable itself if the default TypeScript extension is disabled.
2. Reload the VS Code window by running `Developer: Reload Window` from the command palette.

You can learn more about Take Over mode [here](https://github.com/johnsoncodehk/volar/discussions/471).
