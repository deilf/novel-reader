<template>
  <div id="app">
    <!-- 顶部导航 -->
    <nav class="top-nav">
      <div class="nav-title">📚 小说阅读器</div>
      <div class="nav-tabs">
        <button
          v-for="tab in tabs"
          :key="tab.key"
          class="nav-tab"
          :class="{ active: currentTab === tab.key }"
          @click="switchTab(tab.key)"
        >{{ tab.label }}</button>
      </div>
    </nav>

    <!-- 书源管理 -->
    <SourceManager
      v-if="currentTab === 'sources' && !editingSource"
      :sources="sources"
      @refresh="refreshSources"
      @edit="editingSource = $event"
    />

    <!-- 书源规则编辑器 -->
    <SourceEditorView
      v-else-if="currentTab === 'sources' && editingSource"
      :source="editingSource"
      @back="editingSource = null"
      @saved="refreshSources"
    />

    <!-- 替换净化 -->
    <ReplaceRulesView v-else-if="currentTab === 'replace'" />

    <!-- 书架 -->
    <BookshelfView
      v-else-if="currentTab === 'shelf' && !activeBook && !localReading"
      :sources="sources"
      @open="openFromShelf"
    />

    <!-- 本地书籍 -->
    <LocalBooksView
      v-else-if="currentTab === 'local' && !localReading"
      @open="openLocal"
    />

    <!-- 本地阅读 -->
    <LocalReaderView
      v-else-if="localReading"
      :book="localReading"
      @back="localReading = null"
    />

    <!-- 发现 / 搜索 -->
    <DiscoverView
      v-else-if="currentTab === 'discover' && !activeBook"
      :sources="sources"
      @open="openBook"
    />

    <!-- 详情 + 目录 -->
    <DetailView
      v-else-if="activeBook && !activeChapter"
      :sources="sources"
      :book="activeBook"
      :initial-chapter-url="resumeChapterUrl"
      @back="clearActiveBook"
      @read="openChapter"
    />

    <!-- 阅读页 -->
    <ReaderView
      v-else-if="activeBook && activeChapter"
      :sources="sources"
      :book="activeBook"
      :chapters="chapters"
      :chapter="activeChapter"
      @back="activeChapter = null"
      @jump="activeChapter = $event"
    />
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import SourceManager from './components/SourceManager.vue'
import SourceEditorView from './components/SourceEditorView.vue'
import ReplaceRulesView from './components/ReplaceRulesView.vue'
import BookshelfView from './components/BookshelfView.vue'
import LocalBooksView from './components/LocalBooksView.vue'
import LocalReaderView from './components/LocalReaderView.vue'
import DiscoverView from './components/DiscoverView.vue'
import DetailView from './components/DetailView.vue'
import ReaderView from './components/ReaderView.vue'
import type { BookItem, BookSource, Chapter } from './lib/bookSource/types'
import type { LocalBook } from './lib/bookSource/store'
import { listBookSources } from './lib/bookSource/store'

const tabs = [
  { key: 'discover', label: '发现' },
  { key: 'shelf', label: '书架' },
  { key: 'local', label: '本地书' },
  { key: 'replace', label: '净化' },
  { key: 'sources', label: '书源' },
]

const currentTab = ref('discover')
const sources = ref<BookSource[]>([])

// 阅读流程状态
const activeBook = ref<BookItem | null>(null)
const chapters = ref<Chapter[]>([])
const activeChapter = ref<Chapter | null>(null)
const localReading = ref<LocalBook | null>(null)
const resumeChapterUrl = ref<string | undefined>(undefined)
const editingSource = ref<BookSource | null>(null)

async function refreshSources() {
  sources.value = await listBookSources()
}

async function switchTab(key: string) {
  currentTab.value = key
  // 切换到其它 tab 时收起阅读态
  if (key !== 'discover') {
    activeBook.value = null
    activeChapter.value = null
  }
  if (key !== 'local') {
    localReading.value = null
  }
  if (key !== 'sources') {
    editingSource.value = null
  }
}

function openBook(book: BookItem) {
  activeBook.value = book
  activeChapter.value = null
  chapters.value = []
  resumeChapterUrl.value = undefined
}

function clearActiveBook() {
  activeBook.value = null
  activeChapter.value = null
  resumeChapterUrl.value = undefined
}

function openFromShelf(book: BookItem, resumeUrl?: string) {
  activeBook.value = book
  activeChapter.value = null
  chapters.value = []
  resumeChapterUrl.value = resumeUrl
}

function openChapter(chapter: Chapter) {
  activeChapter.value = chapter
}

function openLocal(book: LocalBook) {
  localReading.value = book
}

onMounted(refreshSources)
</script>

<style scoped>
#app {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
  background: #fff;
}
.top-nav {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 10px;
  padding: calc(var(--safe-top) + 10px) 14px 10px;
  background: #fff;
  border-bottom: 1px solid #eee;
  flex-wrap: wrap;
}
.nav-title { font-size: 17px; font-weight: 700; }
.nav-tabs { display: flex; gap: 4px; flex-wrap: wrap; }
.nav-tab {
  border: 1px solid #ddd;
  background: #fff;
  border-radius: 16px;
  padding: 5px 12px;
  font-size: 13px;
  cursor: pointer;
}
.nav-tab.active {
  background: #1677ff;
  border-color: #1677ff;
  color: #fff;
}
</style>
