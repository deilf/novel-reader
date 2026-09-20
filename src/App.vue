<template>
  <div id="app" :class="{ 'dark-theme': isDarkTheme }">
    <!-- 搜索页面 -->
    <div v-if="currentView === 'search'" class="search-view">
      <header class="app-header">
        <h1 class="app-title">小说阅读器</h1>
      </header>

      <div class="search-container">
        <div class="website-input">
          <input
            v-model="websiteUrl"
            type="url"
            placeholder="输入小说网站URL，如 https://example.com"
            class="url-input"
            @keyup.enter="handleSearch"
          />
          <button @click="handleSearch" class="search-btn" :disabled="isSearching">
            {{ isSearching ? '搜索中...' : '搜索' }}
          </button>
        </div>

        <div class="keyword-input">
          <input
            v-model="searchKeyword"
            type="text"
            placeholder="输入小说名称或作者"
            class="keyword-input-field"
            @keyup.enter="handleSearch"
          />
        </div>
      </div>

      <div v-if="searchResults.length > 0" class="results-container">
        <div class="results-header">
          <span>找到 {{ searchResults.length }} 个结果</span>
          <button @click="clearResults" class="clear-btn">清除</button>
        </div>
        <div class="novel-list">
          <div
            v-for="novel in searchResults"
            :key="novel.url"
            class="novel-card"
            @click="openNovel(novel)"
          >
            <img
              v-if="novel.cover_url"
              :src="novel.cover_url"
              :alt="novel.title"
              class="novel-cover"
              @error="handleImageError"
            />
            <div v-else class="novel-cover placeholder">
              <span>暂无封面</span>
            </div>
            <div class="novel-info">
              <h3 class="novel-title">{{ novel.title }}</h3>
              <p class="novel-author">{{ novel.author }}</p>
              <p class="novel-chapter">{{ novel.latest_chapter }}</p>
              <p class="novel-source">来源: {{ novel.source }}</p>
            </div>
          </div>
        </div>
      </div>

      <div v-if="error" class="error-message">
        <p>{{ error }}</p>
        <button @click="error = ''" class="dismiss-btn">关闭</button>
      </div>
    </div>

    <!-- 小说详情页面 -->
    <div v-else-if="currentView === 'novel'" class="novel-view">
      <header class="app-header">
        <button @click="goBack" class="back-btn">返回</button>
        <h1 class="page-title">{{ novelDetails?.title || '小说详情' }}</h1>
        <div class="header-actions">
          <button @click="toggleCoverSource" class="action-btn" title="切换封面">
            <span>📷</span>
          </button>
        </div>
      </header>

      <div class="novel-detail" v-if="novelDetails">
        <div class="detail-cover">
          <img
            v-if="currentCoverUrl"
            :src="currentCoverUrl"
            :alt="novelDetails.title"
            class="detail-cover-img"
          />
          <div v-else class="detail-cover placeholder">
            <span>暂无封面</span>
          </div>
        </div>

        <div class="detail-info">
          <h2 class="detail-title">{{ novelDetails.title }}</h2>
          <p class="detail-author">作者: {{ novelDetails.author }}</p>
          <p class="detail-desc">{{ novelDetails.description || '暂无简介' }}</p>
        </div>

        <div class="chapter-list-container">
          <h3 class="section-title">目录 ({{ chapters.length }} 章)</h3>
          <div class="chapter-list">
            <div
              v-for="chapter in chapters"
              :key="chapter.url"
              class="chapter-item"
              @click="readChapter(chapter)"
            >
              <span class="chapter-title">{{ chapter.title }}</span>
              <span class="chapter-arrow">›</span>
            </div>
          </div>
        </div>
      </div>

      <div v-else class="loading">
        <p>加载中...</p>
      </div>
    </div>

    <!-- 阅读页面 -->
    <div v-else-if="currentView === 'reader'" class="reader-view">
      <header class="reader-header">
        <button @click="closeReader" class="back-btn">返回</button>
        <h1 class="reader-title">{{ currentChapter?.title || '阅读' }}</h1>
        <button @click="showSettings = true" class="action-btn" title="设置">
          <span>⚙️</span>
        </button>
      </header>

      <div class="reader-content" v-if="currentChapter">
        <div class="chapter-container" v-html="currentChapter.content"></div>
      </div>

      <div class="reader-footer">
        <button @click="prevChapter" class="nav-btn" :disabled="!hasPrevChapter">
          上一章
        </button>
        <button @click="showChapterList = true" class="nav-btn">
          目录
        </button>
        <button @click="nextChapter" class="nav-btn" :disabled="!hasNextChapter">
          下一章
        </button>
      </div>
    </div>

    <!-- 设置面板 -->
    <div v-if="showSettings" class="settings-overlay" @click.self="showSettings = false">
      <div class="settings-panel">
        <h2 class="settings-title">阅读设置</h2>

        <div class="setting-item">
          <label>字体大小</label>
          <div class="setting-control">
            <button @click="decreaseFontSize">-</button>
            <span>{{ formatConfig.fontSize }}px</span>
            <button @click="increaseFontSize">+</button>
          </div>
        </div>

        <div class="setting-item">
          <label>行高</label>
          <div class="setting-control">
            <input
              type="range"
              v-model.number="formatConfig.lineHeight"
              min="1.2"
              max="2.5"
              step="0.1"
            />
            <span>{{ formatConfig.lineHeight.toFixed(1) }}</span>
          </div>
        </div>

        <div class="setting-item">
          <label>段间距</label>
          <div class="setting-control">
            <input
              type="range"
              v-model.number="formatConfig.paragraphSpacing"
              min="0.5"
              max="3"
              step="0.5"
            />
            <span>{{ formatConfig.paragraphSpacing }}em</span>
          </div>
        </div>

        <div class="setting-item">
          <label>首行缩进</label>
          <label class="switch">
            <input type="checkbox" v-model="formatConfig.textIndent" />
            <span class="slider"></span>
          </label>
        </div>

        <div class="setting-item">
          <label>主题</label>
          <div class="theme-options">
            <button
              v-for="theme in themes"
              :key="theme.value"
              :class="{ active: currentTheme === theme.value }"
              @click="setTheme(theme.value)"
              class="theme-btn"
            >
              {{ theme.label }}
            </button>
          </div>
        </div>

        <button @click="showSettings = false" class="close-settings-btn">
          关闭
        </button>
      </div>
    </div>

    <!-- 章节列表弹窗 -->
    <div v-if="showChapterList" class="chapter-list-overlay" @click.self="showChapterList = false">
      <div class="chapter-list-panel">
        <div class="panel-header">
          <h2>目录</h2>
          <button @click="showChapterList = false" class="close-btn">×</button>
        </div>
        <div class="panel-content">
          <div
            v-for="chapter in chapters"
            :key="chapter.url"
            class="chapter-item"
            :class="{ active: chapter.url === currentChapterUrl }"
            @click="selectChapter(chapter)"
          >
            <span class="chapter-index">{{ chapter.index + 1 }}</span>
            <span class="chapter-title">{{ chapter.title }}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- 封面来源切换 -->
    <div v-if="showCoverSwitcher" class="cover-switcher-overlay" @click.self="showCoverSwitcher = false">
      <div class="cover-switcher-panel">
        <h2>选择封面来源</h2>
        <div class="cover-sources">
          <div
            v-for="source in coverSources"
            :key="source.value"
            class="source-option"
            :class="{ active: coverSource === source.value }"
            @click="changeCoverSource(source.value)"
          >
            <span class="source-icon">{{ source.icon }}</span>
            <span class="source-name">{{ source.label }}</span>
          </div>
        </div>
        <button @click="showCoverSwitcher = false" class="close-btn">关闭</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// 类型定义
interface SearchResult {
  title: string
  author: string
  url: string
  cover_url: string
  description: string
  latest_chapter: string
  source: string
}

interface Chapter {
  title: string
  url: string
  index: number
  word_count?: number
}

interface FormattedChapter {
  title: string
  content: string
  source_url: string
  word_count: number
}

interface FormatConfig {
  font_family: string
  font_size: number
  line_height: number
  paragraph_spacing: number
  text_align: string
  text_indent: boolean
  margin_horizontal: number
  margin_vertical: number
  background_color: string
  text_color: string
}

// 响应式数据
const websiteUrl = ref('')
const searchKeyword = ref('')
const searchResults = ref<SearchResult[]>([])
const isSearching = ref(false)
const error = ref('')
const currentView = ref('search')

const novelDetails = ref<{
  title: string
  author: string
  cover_url: string
  description: string
  chapters: Chapter[]
} | null>(null)
const chapters = ref<Chapter[]>([])
const currentCoverUrl = ref('')
const currentChapterUrl = ref('')
const currentChapter = ref<FormattedChapter | null>(null)
const currentChapterIndex = ref(0)

const showSettings = ref(false)
const showChapterList = ref(false)
const showCoverSwitcher = ref(false)
const isDarkTheme = ref(false)
const currentTheme = ref('light')
const coverSource = ref('original')

const formatConfig = ref<FormatConfig>({
  font_family: '系统默认',
  font_size: 18,
  line_height: 1.8,
  paragraph_spacing: 1.5,
  text_align: 'justify',
  text_indent: true,
  margin_horizontal: 16,
  margin_vertical: 20,
  background_color: '#f5f5f5',
  text_color: '#333333'
})

// 常量
const themes = [
  { value: 'light', label: '白天' },
  { value: 'dark', label: '夜间' },
  { value: 'sepia', label: '护眼' }
]

const coverSources = [
  { value: 'original', label: '原始封面', icon: '🖼️' },
  { value: 'google', label: 'Google图片', icon: '🔍' },
  { value: 'baidu', label: '百度图片', icon: '🌐' }
]

// 计算属性
const hasPrevChapter = computed(() => currentChapterIndex.value > 0)
const hasNextChapter = computed(() => currentChapterIndex.value < chapters.value.length - 1)

// 方法
const handleSearch = async () => {
  if (!websiteUrl.value || !searchKeyword.value) {
    error.value = '请输入网站URL和搜索关键词'
    return
  }

  isSearching.value = true
  error.value = ''

  try {
    const results = await invoke<SearchResult[]>('search_novels', {
      websiteUrl: websiteUrl.value,
      keyword: searchKeyword.value
    })
    searchResults.value = results

    if (results.length === 0) {
      error.value = '未找到相关小说，请尝试其他关键词或网站'
    }
  } catch (e) {
    error.value = `搜索失败: ${e}`
  } finally {
    isSearching.value = false
  }
}

const clearResults = () => {
  searchResults.value = []
  error.value = ''
}

const openNovel = async (novel: SearchResult) => {
  currentView.value = 'novel'
  currentCoverUrl.value = novel.cover_url

  try {
    const [title, author, chapterList, description] = await invoke<[string, string, Chapter[], string]>('get_novel_details', {
      url: novel.url
    })

    novelDetails.value = {
      title,
      author,
      cover_url: novel.cover_url,
      description,
      chapters: chapterList
    }
    chapters.value = chapterList
  } catch (e) {
    error.value = `获取小说详情失败: ${e}`
    currentView.value = 'search'
  }
}

const goBack = () => {
  if (currentView.value === 'reader') {
    closeReader()
  } else {
    currentView.value = 'search'
    novelDetails.value = null
    chapters.value = []
  }
}

const readChapter = async (chapter: Chapter) => {
  currentChapterUrl.value = chapter.url
  currentChapterIndex.value = chapter.index
  currentView.value = 'reader'

  try {
    const chapterData = await invoke<FormattedChapter>('get_chapter_content', {
      url: chapter.url,
      formatConfig: formatConfig.value
    })
    currentChapter.value = chapterData
  } catch (e) {
    error.value = `获取章节内容失败: ${e}`
  }
}

const closeReader = () => {
  currentView.value = 'novel'
  currentChapter.value = null
}

const prevChapter = async () => {
  if (!hasPrevChapter.value) return
  const chapter = chapters.value[currentChapterIndex.value - 1]
  await readChapter(chapter)
}

const nextChapter = async () => {
  if (!hasNextChapter.value) return
  const chapter = chapters.value[currentChapterIndex.value + 1]
  await readChapter(chapter)
}

const selectChapter = (chapter: Chapter) => {
  showChapterList.value = false
  readChapter(chapter)
}

const increaseFontSize = () => {
  if (formatConfig.value.font_size < 32) {
    formatConfig.value.font_size += 2
  }
}

const decreaseFontSize = () => {
  if (formatConfig.value.font_size > 12) {
    formatConfig.value.font_size -= 2
  }
}

const setTheme = (theme: string) => {
  currentTheme.value = theme
  isDarkTheme.value = theme === 'dark'

  if (theme === 'dark') {
    formatConfig.value.background_color = '#1a1a1a'
    formatConfig.value.text_color = '#cccccc'
  } else if (theme === 'sepia') {
    formatConfig.value.background_color = '#f4ecd8'
    formatConfig.value.text_color = '#5b4636'
  } else {
    formatConfig.value.background_color = '#f5f5f5'
    formatConfig.value.text_color = '#333333'
  }
}

const toggleCoverSource = () => {
  showCoverSwitcher.value = true
}

const changeCoverSource = async (source: string) => {
  coverSource.value = source

  if (!novelDetails.value) return

  try {
    const newCover = await invoke<string>('change_cover_source', {
      novelTitle: novelDetails.value.title,
      currentCover: currentCoverUrl.value,
      source: source,
      baseUrl: websiteUrl.value
    })
    currentCoverUrl.value = newCover
  } catch (e) {
    error.value = `切换封面失败: ${e}`
  }

  showCoverSwitcher.value = false
}

const handleImageError = (event: Event) => {
  const target = event.target as HTMLImageElement
  target.style.display = 'none'
}

// 监听格式化配置变化
watch(formatConfig, async (newConfig) => {
  if (currentChapter.value) {
    try {
      const chapterData = await invoke<FormattedChapter>('get_chapter_content', {
        url: currentChapterUrl.value,
        formatConfig: newConfig
      })
      currentChapter.value = chapterData
    } catch (e) {
      console.error('重新格式化失败:', e)
    }
  }
}, { deep: true })

// 初始化
onMounted(() => {
  // 从本地存储恢复设置
  const savedTheme = localStorage.getItem('theme')
  if (savedTheme) {
    setTheme(savedTheme)
  }

  const savedCoverSource = localStorage.getItem('coverSource')
  if (savedCoverSource) {
    coverSource.value = savedCoverSource
  }
})
</script>

<style scoped>
/* 基础样式 */
#app {
  height: 100%;
  width: 100%;
  background-color: #f5f5f5;
  color: #333;
  overflow: hidden;
}

#app.dark-theme {
  background-color: #1a1a1a;
  color: #ccc;
}

/* 头部样式 */
.app-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  background-color: #fff;
  border-bottom: 1px solid #e0e0e0;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.dark-theme .app-header {
  background-color: #2a2a2a;
  border-bottom-color: #3a3a3a;
}

.app-title {
  font-size: 18px;
  font-weight: 600;
  color: #333;
}

.dark-theme .app-title {
  color: #fff;
}

.page-title {
  font-size: 16px;
  font-weight: 600;
  flex: 1;
  text-align: center;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.back-btn {
  background: none;
  border: none;
  font-size: 16px;
  color: #333;
  cursor: pointer;
  padding: 4px 8px;
}

.dark-theme .back-btn {
  color: #fff;
}

.header-actions {
  display: flex;
  gap: 8px;
}

.action-btn {
  background: none;
  border: none;
  font-size: 20px;
  cursor: pointer;
  padding: 4px;
}

/* 搜索容器 */
.search-container {
  padding: 16px;
  background-color: #fff;
}

.dark-theme .search-container {
  background-color: #2a2a2a;
}

.website-input {
  display: flex;
  gap: 8px;
  margin-bottom: 12px;
}

.url-input {
  flex: 1;
  padding: 12px;
  border: 1px solid #ddd;
  border-radius: 8px;
  font-size: 14px;
  background-color: #fafafa;
}

.dark-theme .url-input {
  background-color: #3a3a3a;
  border-color: #4a4a4a;
  color: #fff;
}

.search-btn {
  padding: 12px 24px;
  background-color: #4CAF50;
  color: #fff;
  border: none;
  border-radius: 8px;
  font-size: 14px;
  cursor: pointer;
  transition: background-color 0.3s;
}

.search-btn:hover:not(:disabled) {
  background-color: #45a049;
}

.search-btn:disabled {
  background-color: #ccc;
  cursor: not-allowed;
}

.keyword-input {
  margin-top: 8px;
}

.keyword-input-field {
  width: 100%;
  padding: 12px;
  border: 1px solid #ddd;
  border-radius: 8px;
  font-size: 14px;
  background-color: #fafafa;
}

.dark-theme .keyword-input-field {
  background-color: #3a3a3a;
  border-color: #4a4a4a;
  color: #fff;
}

/* 结果容器 */
.results-container {
  padding: 16px;
}

.results-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
  font-size: 14px;
  color: #666;
}

.dark-theme .results-header {
  color: #999;
}

.clear-btn {
  background: none;
  border: 1px solid #ddd;
  padding: 4px 12px;
  border-radius: 4px;
  cursor: pointer;
  font-size: 12px;
  color: #666;
}

.dark-theme .clear-btn {
  border-color: #4a4a4a;
  color: #999;
}

.novel-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.novel-card {
  display: flex;
  background-color: #fff;
  border-radius: 8px;
  padding: 12px;
  cursor: pointer;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  transition: transform 0.2s, box-shadow 0.2s;
}

.dark-theme .novel-card {
  background-color: #2a2a2a;
}

.novel-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.15);
}

.novel-cover {
  width: 80px;
  height: 110px;
  border-radius: 4px;
  object-fit: cover;
  background-color: #e0e0e0;
  flex-shrink: 0;
}

.dark-theme .novel-cover.placeholder {
  background-color: #3a3a3a;
}

.novel-cover.placeholder {
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 12px;
  color: #999;
}

.novel-info {
  flex: 1;
  margin-left: 12px;
  display: flex;
  flex-direction: column;
  gap: 4px;
  overflow: hidden;
}

.novel-title {
  font-size: 16px;
  font-weight: 600;
  color: #333;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.dark-theme .novel-title {
  color: #fff;
}

.novel-author {
  font-size: 14px;
  color: #666;
}

.dark-theme .novel-author {
  color: #999;
}

.novel-chapter {
  font-size: 12px;
  color: #4CAF50;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.novel-source {
  font-size: 11px;
  color: #999;
  margin-top: auto;
}

/* 错误消息 */
.error-message {
  position: fixed;
  bottom: 80px;
  left: 16px;
  right: 16px;
  background-color: #ff5252;
  color: #fff;
  padding: 16px;
  border-radius: 8px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  z-index: 1000;
}

.dismiss-btn {
  background: rgba(255, 255, 255, 0.2);
  border: none;
  color: #fff;
  padding: 4px 12px;
  border-radius: 4px;
  cursor: pointer;
}

/* 小说详情 */
.novel-detail {
  padding: 16px;
  overflow-y: auto;
  height: calc(100% - 60px);
}

.detail-cover {
  width: 150px;
  height: 200px;
  margin: 0 auto 16px;
  border-radius: 8px;
  overflow: hidden;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
}

.detail-cover-img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.detail-cover.placeholder {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: #e0e0e0;
  font-size: 14px;
  color: #999;
}

.dark-theme .detail-cover.placeholder {
  background-color: #3a3a3a;
}

.detail-info {
  text-align: center;
  margin-bottom: 20px;
}

.detail-title {
  font-size: 20px;
  font-weight: 600;
  margin-bottom: 8px;
  color: #333;
}

.dark-theme .detail-title {
  color: #fff;
}

.detail-author {
  font-size: 14px;
  color: #666;
  margin-bottom: 12px;
}

.dark-theme .detail-author {
  color: #999;
}

.detail-desc {
  font-size: 14px;
  color: #666;
  line-height: 1.6;
  text-align: left;
}

.dark-theme .detail-desc {
  color: #999;
}

.section-title {
  font-size: 16px;
  font-weight: 600;
  margin-bottom: 12px;
  color: #333;
}

.dark-theme .section-title {
  color: #fff;
}

.chapter-list {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.chapter-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px;
  background-color: #fff;
  border-radius: 4px;
  cursor: pointer;
  transition: background-color 0.2s;
}

.dark-theme .chapter-item {
  background-color: #2a2a2a;
}

.chapter-item:hover {
  background-color: #f0f0f0;
}

.dark-theme .chapter-item:hover {
  background-color: #3a3a3a;
}

.chapter-item.active {
  background-color: #4CAF50;
  color: #fff;
}

.dark-theme .chapter-item.active {
  background-color: #4CAF50;
}

.chapter-arrow {
  font-size: 18px;
  color: #999;
}

.dark-theme .chapter-arrow {
  color: #666;
}

/* 阅读器 */
.reader-view {
  display: flex;
  flex-direction: column;
  height: 100%;
  background-color: #f5f5f5;
}

.dark-theme .reader-view {
  background-color: #1a1a1a;
}

.reader-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  background-color: #fff;
  border-bottom: 1px solid #e0e0e0;
  flex-shrink: 0;
}

.dark-theme .reader-header {
  background-color: #2a2a2a;
  border-bottom-color: #3a3a3a;
}

.reader-title {
  font-size: 16px;
  font-weight: 600;
  flex: 1;
  text-align: center;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.dark-theme .reader-title {
  color: #fff;
}

.reader-content {
  flex: 1;
  overflow-y: auto;
  padding: 16px;
  padding-bottom: 60px;
}

.reader-footer {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  display: flex;
  justify-content: space-around;
  padding: 12px;
  background-color: #fff;
  border-top: 1px solid #e0e0e0;
}

.dark-theme .reader-footer {
  background-color: #2a2a2a;
  border-top-color: #3a3a3a;
}

.nav-btn {
  padding: 8px 16px;
  background-color: #4CAF50;
  color: #fff;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
}

.nav-btn:disabled {
  background-color: #ccc;
  cursor: not-allowed;
}

/* 设置面板 */
.settings-overlay,
.chapter-list-overlay,
.cover-switcher-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: flex-end;
  justify-content: center;
  z-index: 2000;
}

.settings-panel,
.chapter-list-panel,
.cover-switcher-panel {
  width: 100%;
  max-height: 80%;
  background-color: #fff;
  border-radius: 16px 16px 0 0;
  padding: 20px;
  overflow-y: auto;
}

.dark-theme .settings-panel,
.dark-theme .chapter-list-panel,
.dark-theme .cover-switcher-panel {
  background-color: #2a2a2a;
}

.settings-title,
.panel-header h2,
.cover-switcher-panel h2 {
  font-size: 18px;
  font-weight: 600;
  margin-bottom: 20px;
  text-align: center;
}

.dark-theme .settings-title,
.dark-theme .panel-header h2,
.dark-theme .cover-switcher-panel h2 {
  color: #fff;
}

.setting-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
  padding-bottom: 16px;
  border-bottom: 1px solid #e0e0e0;
}

.dark-theme .setting-item {
  border-bottom-color: #3a3a3a;
}

.setting-item label {
  font-size: 14px;
  color: #333;
}

.dark-theme .setting-item label {
  color: #fff;
}

.setting-control {
  display: flex;
  align-items: center;
  gap: 8px;
}

.setting-control button {
  width: 32px;
  height: 32px;
  border: 1px solid #ddd;
  background-color: #fff;
  border-radius: 4px;
  cursor: pointer;
  font-size: 16px;
}

.dark-theme .setting-control button {
  background-color: #3a3a3a;
  border-color: #4a4a4a;
  color: #fff;
}

.setting-control input[type="range"] {
  width: 120px;
}

.theme-options {
  display: flex;
  gap: 8px;
}

.theme-btn {
  padding: 8px 16px;
  border: 1px solid #ddd;
  background-color: #fff;
  border-radius: 4px;
  cursor: pointer;
  font-size: 12px;
}

.dark-theme .theme-btn {
  background-color: #3a3a3a;
  border-color: #4a4a4a;
  color: #fff;
}

.theme-btn.active {
  background-color: #4CAF50;
  color: #fff;
  border-color: #4CAF50;
}

.close-settings-btn {
  width: 100%;
  padding: 12px;
  background-color: #4CAF50;
  color: #fff;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  font-size: 14px;
  margin-top: 20px;
}

/* 章节列表面板 */
.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.close-btn {
  background: none;
  border: none;
  font-size: 24px;
  cursor: pointer;
  color: #666;
}

.dark-theme .close-btn {
  color: #999;
}

.panel-content {
  max-height: 400px;
  overflow-y: auto;
}

/* 封面切换面板 */
.cover-sources {
  display: flex;
  flex-direction: column;
  gap: 12px;
  margin-bottom: 20px;
}

.source-option {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 16px;
  background-color: #f5f5f5;
  border-radius: 8px;
  cursor: pointer;
  transition: background-color 0.2s;
}

.dark-theme .source-option {
  background-color: #3a3a3a;
}

.source-option:hover {
  background-color: #e0e0e0;
}

.dark-theme .source-option:hover {
  background-color: #4a4a4a;
}

.source-option.active {
  background-color: #4CAF50;
  color: #fff;
}

.source-icon {
  font-size: 24px;
}

.source-name {
  font-size: 14px;
}

/* 加载状态 */
.loading {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 200px;
  color: #999;
}
</style>
