<template>
  <div id="app" :class="{ 'dark-theme': isDarkTheme }">
    <!-- 搜索页面 -->
    <div v-if="currentView === 'search'" class="search-view">
      <!-- 顶部标题栏 -->
      <header class="app-header">
        <h1 class="app-title">📚 小说阅读器</h1>
      </header>

      <!-- 搜索区域 - 磁贴式输入框 -->
      <div class="search-container">
        <div class="search-box">
          <div class="input-tile">
            <span class="tile-icon">🌐</span>
            <input
              v-model="websiteUrl"
              type="url"
              placeholder="输入小说网站URL"
              class="url-input"
              @keyup.enter="handleSearch"
            />
          </div>
          <div class="input-tile keyword-tile">
            <span class="tile-icon">🔍</span>
            <input
              v-model="searchKeyword"
              type="text"
              placeholder="输入小说名称或作者"
              class="keyword-input-field"
              @keyup.enter="handleSearch"
            />
          </div>
          <button @click="handleSearch" class="search-tile-btn" :disabled="isSearching">
            <span v-if="isSearching" class="loading-spinner">⏳</span>
            <span v-else>🚀 搜索</span>
          </button>
        </div>
      </div>

      <!-- 搜索结果 - 磁贴网格布局 -->
      <div v-if="searchResults.length > 0" class="results-container">
        <div class="results-header">
          <span class="result-count">找到 {{ searchResults.length }} 个小说</span>
          <button @click="clearResults" class="clear-btn">🗑️ 清空</button>
        </div>

        <!-- 磁贴网格 -->
        <div class="novel-grid">
          <div
            v-for="novel in searchResults"
            :key="novel.url"
            class="novel-tile"
            @click="openNovel(novel)"
          >
            <!-- 封面图片 -->
            <div class="tile-cover">
              <img
                v-if="novel.cover_url"
                :src="novel.cover_url"
                :alt="novel.title"
                class="cover-image"
                @error="handleImageError"
              />
              <div v-else class="cover-placeholder">
                <span class="placeholder-icon">📖</span>
                <span class="placeholder-text">暂无封面</span>
              </div>
              <div class="tile-overlay">
                <span class="read-btn">📖 开始阅读</span>
              </div>
            </div>

            <!-- 小说信息 -->
            <div class="tile-info">
              <h3 class="tile-title">{{ novel.title }}</h3>
              <p class="tile-author">👤 {{ novel.author }}</p>
              <p class="tile-chapter">📑 {{ novel.latest_chapter }}</p>
              <div class="tile-footer">
                <span class="tile-source">🏷️ {{ novel.source }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 空状态 -->
      <div v-if="searchResults.length === 0 && !isSearching" class="empty-state">
        <div class="empty-icon">📚</div>
        <p class="empty-text">输入网站和关键词开始搜索小说</p>
        <p class="empty-hint">推荐网站：笔趣阁、起点中文网等</p>
      </div>

      <!-- 错误消息 -->
      <div v-if="error" class="error-message">
        <span>⚠️ {{ error }}</span>
        <button @click="error = ''" class="dismiss-btn">关闭</button>
      </div>
    </div>

    <!-- 小说详情页面 -->
    <div v-else-if="currentView === 'novel'" class="novel-view">
      <header class="app-header">
        <button @click="goBack" class="back-btn">← 返回</button>
        <h1 class="page-title">{{ novelDetails?.title || '小说详情' }}</h1>
        <button @click="toggleCoverSource" class="action-btn" title="切换封面">
          🖼️
        </button>
      </header>

      <div class="novel-detail" v-if="novelDetails">
        <!-- 大封面展示 -->
        <div class="detail-hero">
          <div class="hero-cover">
            <img
              v-if="currentCoverUrl"
              :src="currentCoverUrl"
              :alt="novelDetails.title"
              class="hero-cover-img"
            />
            <div v-else class="hero-cover-placeholder">
              <span>📖</span>
            </div>
          </div>
          <div class="hero-info">
            <h2 class="detail-title">{{ novelDetails.title }}</h2>
            <p class="detail-author">👤 {{ novelDetails.author }}</p>
            <p class="detail-desc">{{ novelDetails.description || '暂无简介' }}</p>
            <div class="detail-actions">
              <button class="action-tile" @click="readFirstChapter">
                📖 开始阅读
              </button>
              <button class="action-tile secondary" @click="toggleCoverSource">
                🖼️ 换封面
              </button>
            </div>
          </div>
        </div>

        <!-- 章节列表 -->
        <div class="chapter-section">
          <h3 class="section-title">📚 目录 ({{ chapters.length }} 章)</h3>
          <div class="chapter-grid">
            <div
              v-for="chapter in chapters"
              :key="chapter.url"
              class="chapter-tile"
              @click="readChapter(chapter)"
            >
              <span class="chapter-num">{{ chapter.index + 1 }}</span>
              <span class="chapter-title">{{ chapter.title }}</span>
            </div>
          </div>
        </div>
      </div>

      <div v-else class="loading">
        <div class="loading-spinner">⏳</div>
        <p>加载中...</p>
      </div>
    </div>

    <!-- 阅读页面 -->
    <div v-else-if="currentView === 'reader'" class="reader-view">
      <header class="reader-header">
        <button @click="closeReader" class="back-btn">← 返回</button>
        <h1 class="reader-title">{{ currentChapter?.title || '阅读' }}</h1>
        <button @click="showSettings = true" class="action-btn" title="设置">
          ⚙️
        </button>
      </header>

      <div class="reader-content" v-if="currentChapter">
        <div class="chapter-container" v-html="currentChapter.content"></div>
      </div>

      <div class="reader-footer">
        <button @click="prevChapter" class="nav-btn" :disabled="!hasPrevChapter">
          ◀ 上一章
        </button>
        <button @click="showChapterList = true" class="nav-btn highlight">
          📖 目录
        </button>
        <button @click="nextChapter" class="nav-btn" :disabled="!hasNextChapter">
          下一章 ▶
        </button>
      </div>
    </div>

    <!-- 设置面板 -->
    <div v-if="showSettings" class="settings-overlay" @click.self="showSettings = false">
      <div class="settings-panel">
        <h2 class="settings-title">⚙️ 阅读设置</h2>

        <div class="setting-item">
          <label>🔤 字体大小</label>
          <div class="setting-control">
            <button @click="decreaseFontSize" class="control-btn">-</button>
            <span class="setting-value">{{ formatConfig.font_size }}px</span>
            <button @click="increaseFontSize" class="control-btn">+</button>
          </div>
        </div>

        <div class="setting-item">
          <label>📏 行高</label>
          <div class="setting-control">
            <input
              type="range"
              v-model.number="formatConfig.line_height"
              min="1.2"
              max="2.5"
              step="0.1"
              class="range-slider"
            />
            <span class="setting-value">{{ formatConfig.line_height.toFixed(1) }}</span>
          </div>
        </div>

        <div class="setting-item">
          <label>📐 段间距</label>
          <div class="setting-control">
            <input
              type="range"
              v-model.number="formatConfig.paragraph_spacing"
              min="0.5"
              max="3"
              step="0.5"
              class="range-slider"
            />
            <span class="setting-value">{{ formatConfig.paragraph_spacing }}em</span>
          </div>
        </div>

        <div class="setting-item">
          <label>📝 首行缩进</label>
          <label class="switch">
            <input type="checkbox" v-model="formatConfig.text_indent" />
            <span class="slider"></span>
          </label>
        </div>

        <div class="setting-item">
          <label>🎨 主题</label>
          <div class="theme-options">
            <button
              v-for="theme in themes"
              :key="theme.value"
              :class="{ active: currentTheme === theme.value }"
              @click="setTheme(theme.value)"
              class="theme-btn"
            >
              {{ theme.icon }} {{ theme.label }}
            </button>
          </div>
        </div>

        <button @click="showSettings = false" class="close-settings-btn">
          ✓ 关闭
        </button>
      </div>
    </div>

    <!-- 章节列表弹窗 -->
    <div v-if="showChapterList" class="chapter-list-overlay" @click.self="showChapterList = false">
      <div class="chapter-list-panel">
        <div class="panel-header">
          <h2>📖 目录</h2>
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
        <h2>🖼️ 选择封面来源</h2>
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
        <button @click="showCoverSwitcher = false" class="close-btn-secondary">关闭</button>
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
  { value: 'light', label: '白天', icon: '☀️' },
  { value: 'dark', label: '夜间', icon: '🌙' },
  { value: 'sepia', label: '护眼', icon: '📜' }
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

const readFirstChapter = () => {
  if (chapters.value.length > 0) {
    readChapter(chapters.value[0])
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
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: #333;
  overflow-x: hidden;
}

#app.dark-theme {
  background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%);
  color: #ccc;
}

/* 头部样式 */
.app-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(10px);
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  position: sticky;
  top: 0;
  z-index: 100;
}

.dark-theme .app-header {
  background: rgba(30, 30, 50, 0.95);
}

.app-title {
  font-size: 20px;
  font-weight: 700;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
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
  color: #667eea;
  cursor: pointer;
  padding: 8px 12px;
  border-radius: 8px;
  transition: background 0.3s;
}

.back-btn:hover {
  background: rgba(102, 126, 234, 0.1);
}

.header-actions {
  display: flex;
  gap: 8px;
}

.action-btn {
  background: none;
  border: none;
  font-size: 24px;
  cursor: pointer;
  padding: 8px;
  border-radius: 8px;
  transition: all 0.3s;
}

.action-btn:hover {
  background: rgba(102, 126, 234, 0.1);
  transform: scale(1.1);
}

/* 搜索容器 */
.search-container {
  padding: 20px;
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(10px);
  margin: 20px;
  border-radius: 16px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
}

.dark-theme .search-container {
  background: rgba(30, 30, 50, 0.95);
}

.search-box {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.input-tile {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 14px 16px;
  background: #f8f9fa;
  border-radius: 12px;
  border: 2px solid transparent;
  transition: all 0.3s;
}

.input-tile:focus-within {
  border-color: #667eea;
  box-shadow: 0 0 0 4px rgba(102, 126, 234, 0.1);
}

.dark-theme .input-tile {
  background: #2a2a3e;
}

.tile-icon {
  font-size: 20px;
}

.url-input,
.keyword-input-field {
  flex: 1;
  border: none;
  background: none;
  font-size: 15px;
  color: #333;
  outline: none;
}

.dark-theme .url-input,
.dark-theme .keyword-input-field {
  color: #ccc;
}

.url-input::placeholder,
.keyword-input-field::placeholder {
  color: #999;
}

.keyword-tile {
  margin-top: 0;
}

.search-tile-btn {
  padding: 14px 24px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: #fff;
  border: none;
  border-radius: 12px;
  font-size: 16px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
}

.search-tile-btn:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 8px 16px rgba(102, 126, 234, 0.3);
}

.search-tile-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

/* 结果容器 */
.results-container {
  padding: 0 20px 20px;
}

.results-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.result-count {
  font-size: 14px;
  font-weight: 600;
  color: #fff;
  background: rgba(255, 255, 255, 0.2);
  padding: 8px 16px;
  border-radius: 20px;
  backdrop-filter: blur(10px);
}

.clear-btn {
  background: rgba(255, 255, 255, 0.2);
  border: none;
  color: #fff;
  padding: 8px 16px;
  border-radius: 20px;
  cursor: pointer;
  font-size: 14px;
  backdrop-filter: blur(10px);
  transition: all 0.3s;
}

.clear-btn:hover {
  background: rgba(255, 255, 255, 0.3);
}

/* 磁贴网格 - Pinterest风格 */
.novel-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
  gap: 16px;
}

.novel-tile {
  background: #fff;
  border-radius: 16px;
  overflow: hidden;
  cursor: pointer;
  transition: all 0.3s;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.novel-tile:hover {
  transform: translateY(-8px) scale(1.02);
  box-shadow: 0 12px 24px rgba(0, 0, 0, 0.2);
}

.dark-theme .novel-tile {
  background: rgba(40, 40, 70, 0.9);
}

.tile-cover {
  position: relative;
  width: 100%;
  aspect-ratio: 3/4;
  overflow: hidden;
}

.cover-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
  transition: transform 0.3s;
}

.novel-tile:hover .cover-image {
  transform: scale(1.05);
}

.cover-placeholder {
  width: 100%;
  height: 100%;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 8px;
}

.placeholder-icon {
  font-size: 48px;
}

.placeholder-text {
  font-size: 12px;
  color: rgba(255, 255, 255, 0.8);
}

.tile-overlay {
  position: absolute;
  inset: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  opacity: 0;
  transition: opacity 0.3s;
}

.novel-tile:hover .tile-overlay {
  opacity: 1;
}

.read-btn {
  padding: 10px 20px;
  background: #667eea;
  color: #fff;
  border-radius: 20px;
  font-size: 14px;
  font-weight: 600;
  transform: translateY(20px);
  transition: transform 0.3s;
}

.novel-tile:hover .read-btn {
  transform: translateY(0);
}

.tile-info {
  padding: 12px;
}

.tile-title {
  font-size: 14px;
  font-weight: 600;
  color: #333;
  margin-bottom: 6px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.dark-theme .tile-title {
  color: #fff;
}

.tile-author,
.tile-chapter {
  font-size: 11px;
  color: #666;
  margin-bottom: 4px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.dark-theme .tile-author,
.dark-theme .tile-chapter {
  color: #aaa;
}

.tile-footer {
  margin-top: 8px;
}

.tile-source {
  font-size: 10px;
  color: #667eea;
  background: rgba(102, 126, 234, 0.1);
  padding: 4px 8px;
  border-radius: 10px;
}

/* 空状态 */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
  text-align: center;
}

.empty-icon {
  font-size: 80px;
  margin-bottom: 20px;
  animation: bounce 2s infinite;
}

@keyframes bounce {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-20px); }
}

.empty-text {
  font-size: 18px;
  font-weight: 600;
  color: #fff;
  margin-bottom: 8px;
}

.empty-hint {
  font-size: 14px;
  color: rgba(255, 255, 255, 0.7);
}

/* 错误消息 */
.error-message {
  position: fixed;
  bottom: 80px;
  left: 20px;
  right: 20px;
  background: linear-gradient(135deg, #ff6b6b 0%, #ee5a24 100%);
  color: #fff;
  padding: 16px 20px;
  border-radius: 12px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  z-index: 1000;
  box-shadow: 0 4px 12px rgba(255, 107, 107, 0.3);
}

.dismiss-btn {
  background: rgba(255, 255, 255, 0.2);
  border: none;
  color: #fff;
  padding: 6px 12px;
  border-radius: 6px;
  cursor: pointer;
}

/* 小说详情 */
.novel-detail {
  padding: 20px;
  overflow-y: auto;
}

.detail-hero {
  background: #fff;
  border-radius: 16px;
  overflow: hidden;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.1);
  margin-bottom: 20px;
}

.dark-theme .detail-hero {
  background: rgba(40, 40, 70, 0.9);
}

.hero-cover {
  width: 100%;
  aspect-ratio: 16/9;
  overflow: hidden;
}

.hero-cover-img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.hero-cover-placeholder {
  width: 100%;
  height: 100%;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 80px;
}

.hero-info {
  padding: 20px;
}

.detail-title {
  font-size: 22px;
  font-weight: 700;
  color: #333;
  margin-bottom: 8px;
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
  color: #aaa;
}

.detail-desc {
  font-size: 14px;
  color: #666;
  line-height: 1.6;
  margin-bottom: 16px;
}

.dark-theme .detail-desc {
  color: #aaa;
}

.detail-actions {
  display: flex;
  gap: 12px;
}

.action-tile {
  flex: 1;
  padding: 12px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: #fff;
  border: none;
  border-radius: 12px;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s;
}

.action-tile:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
}

.action-tile.secondary {
  background: #f0f0f0;
  color: #333;
}

.dark-theme .action-tile.secondary {
  background: #2a2a3e;
  color: #fff;
}

/* 章节列表 */
.chapter-section {
  background: #fff;
  border-radius: 16px;
  padding: 20px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.dark-theme .chapter-section {
  background: rgba(40, 40, 70, 0.9);
}

.section-title {
  font-size: 18px;
  font-weight: 700;
  color: #333;
  margin-bottom: 16px;
}

.dark-theme .section-title {
  color: #fff;
}

.chapter-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
  gap: 12px;
  max-height: 400px;
  overflow-y: auto;
}

.chapter-tile {
  padding: 12px;
  background: #f8f9fa;
  border-radius: 10px;
  cursor: pointer;
  transition: all 0.3s;
  display: flex;
  align-items: center;
  gap: 8px;
}

.chapter-tile:hover {
  background: #667eea;
  color: #fff;
  transform: translateX(4px);
}

.dark-theme .chapter-tile {
  background: #2a2a3e;
  color: #fff;
}

.chapter-num {
  width: 28px;
  height: 28px;
  background: #667eea;
  color: #fff;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 12px;
  font-weight: 600;
  flex-shrink: 0;
}

.chapter-tile:hover .chapter-num {
  background: #fff;
  color: #667eea;
}

.chapter-title {
  flex: 1;
  font-size: 13px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* 阅读器 */
.reader-view {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.reader-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(10px);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  flex-shrink: 0;
}

.dark-theme .reader-header {
  background: rgba(30, 30, 50, 0.95);
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

.reader-content {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
  padding-bottom: 80px;
}

.reader-footer {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  display: flex;
  justify-content: space-around;
  padding: 12px 20px;
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(10px);
  box-shadow: 0 -2px 8px rgba(0, 0, 0, 0.1);
}

.nav-btn {
  padding: 10px 16px;
  background: #f0f0f0;
  color: #333;
  border: none;
  border-radius: 20px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.3s;
}

.nav-btn:hover:not(:disabled) {
  background: #667eea;
  color: #fff;
}

.nav-btn.highlight {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: #fff;
}

.nav-btn:disabled {
  opacity: 0.5;
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
  background: rgba(0, 0, 0, 0.6);
  backdrop-filter: blur(4px);
  display: flex;
  align-items: flex-end;
  justify-content: center;
  z-index: 2000;
}

.settings-panel,
.chapter-list-panel,
.cover-switcher-panel {
  width: 100%;
  max-height: 85%;
  background: #fff;
  border-radius: 24px 24px 0 0;
  padding: 24px;
  overflow-y: auto;
}

.dark-theme .settings-panel,
.dark-theme .chapter-list-panel,
.dark-theme .cover-switcher-panel {
  background: #1e1e2e;
}

.settings-title,
.panel-header h2,
.cover-switcher-panel h2 {
  font-size: 20px;
  font-weight: 700;
  margin-bottom: 20px;
  text-align: center;
  color: #333;
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
  padding: 16px 0;
  border-bottom: 1px solid #f0f0f0;
}

.dark-theme .setting-item {
  border-bottom-color: #2a2a3e;
}

.setting-item label {
  font-size: 15px;
  color: #333;
  font-weight: 500;
}

.dark-theme .setting-item label {
  color: #fff;
}

.setting-control {
  display: flex;
  align-items: center;
  gap: 12px;
}

.control-btn {
  width: 36px;
  height: 36px;
  border: 2px solid #667eea;
  background: #fff;
  color: #667eea;
  border-radius: 50%;
  cursor: pointer;
  font-size: 18px;
  font-weight: 700;
  transition: all 0.3s;
}

.control-btn:hover {
  background: #667eea;
  color: #fff;
}

.dark-theme .control-btn {
  background: #2a2a3e;
}

.setting-value {
  min-width: 50px;
  text-align: center;
  font-size: 14px;
  font-weight: 600;
  color: #667eea;
}

.range-slider {
  width: 120px;
  accent-color: #667eea;
}

.theme-options {
  display: flex;
  gap: 8px;
}

.theme-btn {
  padding: 8px 14px;
  background: #f0f0f0;
  border: 2px solid transparent;
  border-radius: 20px;
  cursor: pointer;
  font-size: 13px;
  transition: all 0.3s;
}

.dark-theme .theme-btn {
  background: #2a2a3e;
  color: #fff;
}

.theme-btn.active {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: #fff;
  border-color: #667eea;
}

.close-settings-btn {
  width: 100%;
  padding: 14px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: #fff;
  border: none;
  border-radius: 12px;
  cursor: pointer;
  font-size: 16px;
  font-weight: 600;
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
  font-size: 28px;
  cursor: pointer;
  color: #999;
  padding: 4px 8px;
}

.close-btn:hover {
  color: #667eea;
}

.panel-content {
  max-height: 400px;
  overflow-y: auto;
}

.chapter-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 14px;
  background: #f8f9fa;
  border-radius: 10px;
  cursor: pointer;
  margin-bottom: 8px;
  transition: all 0.3s;
}

.dark-theme .chapter-item {
  background: #2a2a3e;
  color: #fff;
}

.chapter-item:hover {
  background: #667eea;
  color: #fff;
}

.chapter-item.active {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: #fff;
}

.chapter-index {
  width: 32px;
  height: 32px;
  background: #667eea;
  color: #fff;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 13px;
  font-weight: 600;
  flex-shrink: 0;
}

.chapter-item.active .chapter-index {
  background: #fff;
  color: #667eea;
}

.chapter-title {
  flex: 1;
  font-size: 14px;
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
  gap: 16px;
  padding: 16px;
  background: #f8f9fa;
  border-radius: 12px;
  cursor: pointer;
  transition: all 0.3s;
  border: 2px solid transparent;
}

.dark-theme .source-option {
  background: #2a2a3e;
  color: #fff;
}

.source-option:hover {
  border-color: #667eea;
}

.source-option.active {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: #fff;
  border-color: #667eea;
}

.source-icon {
  font-size: 28px;
}

.source-name {
  font-size: 15px;
  font-weight: 500;
}

.close-btn-secondary {
  width: 100%;
  padding: 12px;
  background: #f0f0f0;
  color: #333;
  border: none;
  border-radius: 10px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
}

.dark-theme .close-btn-secondary {
  background: #2a2a3e;
  color: #fff;
}

/* 加载状态 */
.loading {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px;
}

.loading-spinner {
  font-size: 48px;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

/* 响应式设计 */
@media (max-width: 480px) {
  .novel-grid {
    grid-template-columns: repeat(2, 1fr);
    gap: 12px;
  }

  .search-container {
    margin: 12px;
    padding: 16px;
  }

  .detail-actions {
    flex-direction: column;
  }

  .chapter-grid {
    grid-template-columns: 1fr;
  }
}

@media (min-width: 768px) {
  .novel-grid {
    grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
  }
}
</style>
