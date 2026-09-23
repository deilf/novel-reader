<template>
  <div class="reader-view">
    <div class="reader-header">
      <button class="back-btn" @click="$emit('back')">← 目录</button>
      <span class="chapter-title">{{ title }}</span>
      <span class="loading" v-if="loading">加载中...</span>
      <div class="font-controls">
        <button class="font-btn" @click="fontSize -= 2" :disabled="fontSize <= 12">A-</button>
        <span class="font-size">{{ fontSize }}</span>
        <button class="font-btn" @click="fontSize += 2" :disabled="fontSize >= 28">A+</button>
      </div>
    </div>

    <div v-if="error" class="error-tip">{{ error }}</div>

    <div class="reader-body" :style="{ fontSize: fontSize + 'px', lineHeight: lineHeight + '' }">
      <div v-if="content" class="content-text">{{ content }}</div>
      <div v-else-if="!loading && !error" class="empty-tip">本章暂无内容</div>
    </div>

    <div class="reader-footer">
      <button class="nav-btn" :disabled="!prevChapter || loading" @click="goTo(prevChapter)">上一章</button>
      <button class="nav-btn" :disabled="!nextChapter || loading" @click="goTo(nextChapter)">下一章</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import type { BookItem, BookSource, Chapter } from '../lib/bookSource/types'
import { getChapterContent } from '../lib/bookSource/runner'

const props = defineProps<{
  sources: BookSource[]
  book: BookItem
  chapters: Chapter[]
  chapter: Chapter
}>()
const emit = defineEmits<{ (e: 'back'): void; (e: 'jump', chapter: Chapter): void }>()

const title = ref('')
const content = ref('')
const loading = ref(true)
const error = ref('')
const fontSize = ref(18)
const lineHeight = ref(1.9)

const idx = computed(() => props.chapters.findIndex((c) => c.chapterUrl === props.chapter.chapterUrl))
const prevChapter = computed(() => (idx.value > 0 ? props.chapters[idx.value - 1] : null))
const nextChapter = computed(() => (idx.value >= 0 && idx.value < props.chapters.length - 1 ? props.chapters[idx.value + 1] : null))

async function load(chapter: Chapter) {
  loading.value = true
  error.value = ''
  content.value = ''
  title.value = chapter.chapterName
  const source = props.sources.find((s) => s.bookSourceUrl === props.book.sourceUrl)
  if (!source) {
    error.value = '未找到该书源'
    loading.value = false
    return
  }
  try {
    const result = await getChapterContent(source, props.book, chapter)
    content.value = result.content
  } catch (e) {
    error.value = `加载失败：${e}`
  } finally {
    loading.value = false
  }
}

onMounted(() => load(props.chapter))

watch(() => props.chapter, (ch) => load(ch))

function goTo(ch: Chapter | null) {
  if (ch) emit('jump', ch)
}
</script>

<style scoped>
.reader-view {
  display: flex;
  flex-direction: column;
  height: 100%;
  max-width: 760px;
  margin: 0 auto;
  width: 100%;
}
.reader-header {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 14px;
  border-bottom: 1px solid #eee;
  background: #fff;
  flex-wrap: wrap;
}
.back-btn {
  border: 1px solid #ccc;
  background: #fff;
  border-radius: 6px;
  padding: 6px 10px;
  font-size: 13px;
  cursor: pointer;
}
.chapter-title {
  font-size: 15px;
  font-weight: 600;
  flex: 1;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.loading { font-size: 12px; color: #999; }
.font-controls { display: flex; align-items: center; gap: 6px; }
.font-btn {
  border: 1px solid #ddd;
  background: #fff;
  border-radius: 5px;
  padding: 3px 8px;
  font-size: 12px;
  cursor: pointer;
}
.font-btn:disabled { opacity: 0.4; }
.font-size { font-size: 12px; color: #666; min-width: 24px; text-align: center; }
.error-tip { color: #d4380d; font-size: 13px; padding: 12px 16px; }
.reader-body {
  flex: 1;
  overflow-y: auto;
  padding: 18px 20px;
  background: #fbfaf6;
  color: #333;
}
.content-text {
  white-space: pre-wrap;
  word-break: break-word;
}
.empty-tip { text-align: center; color: #999; padding: 40px 0; }
.reader-footer {
  display: flex;
  gap: 10px;
  padding: 10px 14px;
  border-top: 1px solid #eee;
  background: #fff;
}
.nav-btn {
  flex: 1;
  padding: 8px 0;
  border: 1px solid #ddd;
  background: #fff;
  border-radius: 6px;
  font-size: 14px;
  cursor: pointer;
}
.nav-btn:disabled { opacity: 0.4; cursor: not-allowed; }
</style>
