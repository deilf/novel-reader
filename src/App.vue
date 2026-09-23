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
    <SourceManager v-if="currentTab === 'sources'" :sources="sources" @refresh="refreshSources" />

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
      @back="activeBook = null"
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
import DiscoverView from './components/DiscoverView.vue'
import DetailView from './components/DetailView.vue'
import ReaderView from './components/ReaderView.vue'
import type { BookItem, BookSource, Chapter } from './lib/bookSource/types'
import { listBookSources } from './lib/bookSource/store'

const tabs = [
  { key: 'discover', label: '发现' },
  { key: 'sources', label: '书源' },
]

const currentTab = ref('discover')
const sources = ref<BookSource[]>([])

// 阅读流程状态
const activeBook = ref<BookItem | null>(null)
const chapters = ref<Chapter[]>([])
const activeChapter = ref<Chapter | null>(null)

async function refreshSources() {
  sources.value = await listBookSources()
}

async function switchTab(key: string) {
  currentTab.value = key
  if (key === 'discover') {
    activeBook.value = null
    activeChapter.value = null
  }
}

function openBook(book: BookItem) {
  activeBook.value = book
  activeChapter.value = null
  chapters.value = []
}

function openChapter(chapter: Chapter) {
  activeChapter.value = chapter
}

onMounted(refreshSources)
</script>

<style scoped>
#app {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
}
.top-nav {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 10px;
  padding: 10px 14px;
  background: #fff;
  border-bottom: 1px solid #eee;
  flex-wrap: wrap;
}
.nav-title { font-size: 17px; font-weight: 700; }
.nav-tabs { display: flex; gap: 6px; }
.nav-tab {
  border: 1px solid #ddd;
  background: #fff;
  border-radius: 16px;
  padding: 5px 16px;
  font-size: 13px;
  cursor: pointer;
}
.nav-tab.active {
  background: #1677ff;
  border-color: #1677ff;
  color: #fff;
}
</style>
