<template>
  <div class="detail-view">
    <div class="detail-header">
      <button class="back-btn" @click="$emit('back')">← 返回</button>
      <span class="loading" v-if="loading">加载中...</span>
    </div>

    <div v-if="book" class="book-info">
      <div class="cover">
        <img v-if="book.coverUrl" :src="book.coverUrl" @error="onImgError" alt="" />
        <span v-else class="no-cover">📖</span>
      </div>
      <div class="info">
        <h3>{{ book.bookName }}</h3>
        <div class="meta-line"><span>作者：</span>{{ book.author || '未知' }}</div>
        <div class="meta-line"><span>来源：</span>{{ book.source }}</div>
        <div class="meta-line" v-if="book.kind"><span>分类：</span>{{ book.kind }}</div>
        <div class="meta-line" v-if="book.lastChapter"><span>最新章节：</span>{{ book.lastChapter }}</div>
        <p class="intro" v-if="book.intro">{{ book.intro }}</p>
      </div>
    </div>

    <div v-if="error" class="error-tip">{{ error }}</div>

    <div v-if="chapters.length > 0" class="chapter-list">
      <div class="chapter-header">目录（{{ chapters.length }} 章）</div>
      <div
        v-for="ch in chapters"
        :key="ch.chapterUrl"
        class="chapter-item"
        @click="$emit('read', ch)"
      >
        <span class="ch-index">{{ ch.index + 1 }}</span>
        <span class="ch-name">{{ ch.chapterName }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import type { BookItem, BookSource, Chapter } from '../lib/bookSource/types'
import { getBookInfoAndToc } from '../lib/bookSource/runner'

const props = defineProps<{
  sources: BookSource[]
  book: BookItem
}>()
const loading = ref(true)
const error = ref('')
const book = ref<BookItem & { intro?: string; kind?: string; lastChapter?: string; coverUrl?: string } | null>(null)
const chapters = ref<Chapter[]>([])

onMounted(async () => {
  const source = props.sources.find((s) => s.bookSourceUrl === props.book.sourceUrl)
  if (!source) {
    error.value = '未找到该书源，可能已被删除'
    loading.value = false
    return
  }
  try {
    const info = await getBookInfoAndToc(source, props.book)
    book.value = info
    chapters.value = info.chapters
  } catch (e) {
    error.value = `加载失败：${e}`
  } finally {
    loading.value = false
  }
})

function onImgError(e: Event) {
  ;(e.target as HTMLImageElement).style.display = 'none'
}
</script>

<style scoped>
.detail-view {
  padding: 16px;
  max-width: 720px;
  margin: 0 auto;
  width: 100%;
  display: flex;
  flex-direction: column;
  gap: 14px;
}
.detail-header { display: flex; align-items: center; gap: 10px; }
.back-btn {
  border: 1px solid #ccc;
  background: #fff;
  border-radius: 6px;
  padding: 6px 12px;
  font-size: 13px;
  cursor: pointer;
}
.loading { font-size: 13px; color: #888; }
.book-info { display: flex; gap: 14px; }
.cover {
  width: 120px;
  height: 160px;
  flex-shrink: 0;
  border-radius: 8px;
  overflow: hidden;
  background: #f0f0f0;
  display: flex;
  align-items: center;
  justify-content: center;
}
.cover img { width: 100%; height: 100%; object-fit: cover; }
.no-cover { font-size: 36px; }
.info { flex: 1; min-width: 0; display: flex; flex-direction: column; gap: 4px; }
.info h3 { font-size: 18px; }
.meta-line { font-size: 13px; color: #555; }
.meta-line span { color: #999; }
.intro { font-size: 13px; color: #666; line-height: 1.7; margin-top: 6px; }
.error-tip { color: #d4380d; font-size: 13px; }
.chapter-list {
  border: 1px solid #eee;
  border-radius: 10px;
  background: #fff;
  overflow: hidden;
}
.chapter-header {
  padding: 10px 14px;
  font-size: 14px;
  font-weight: 600;
  border-bottom: 1px solid #f0f0f0;
  background: #fafafa;
}
.chapter-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 9px 14px;
  border-bottom: 1px solid #f5f5f5;
  cursor: pointer;
  font-size: 14px;
}
.chapter-item:last-child { border-bottom: none; }
.chapter-item:hover { background: #f6f9ff; }
.ch-index { font-size: 12px; color: #999; width: 28px; flex-shrink: 0; }
.ch-name { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
</style>
