<template>
  <div class="bookshelf-view">
    <div class="bs-header">
      <h2>书架</h2>
      <span class="bs-count">{{ books.length }} 本</span>
    </div>

    <div v-if="books.length === 0" class="empty-tip">
      书架是空的。<br />
      在小说详情页点击"加入书架"，或从发现页搜索后添加。
    </div>

    <div v-else class="book-list">
      <div v-for="b in books" :key="b.url" class="book-item" @click="openBook(b)">
        <div class="cover">
          <img v-if="b.cover_url" :src="b.cover_url" loading="lazy" @error="onImgError" alt="" />
          <span v-else class="no-cover">📖</span>
        </div>
        <div class="meta">
          <div class="name">{{ b.title }}</div>
          <div class="sub">{{ b.author || '未知作者' }} · {{ b.source }}</div>
          <div class="progress" v-if="b.progress">读到：{{ b.progress.chapter_title }}</div>
        </div>
        <button class="remove-btn" @click.stop="remove(b)">✕</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import type { BookItem, BookSource } from '../lib/bookSource/types'
import type { ReadingProgress } from '../lib/bookSource/shelf'
import { getBookshelf, removeFromBookshelf } from '../lib/bookSource/shelf'

const props = defineProps<{ sources: BookSource[] }>()
const emit = defineEmits<{ (e: 'open', book: BookItem, resumeChapterUrl?: string): void }>()

interface ShelfBook {
  title: string
  author?: string
  cover_url?: string
  url: string
  source: string
  latest_chapter?: string
  progress?: ReadingProgress
}

const books = ref<ShelfBook[]>([])

onMounted(load)

async function load() {
  books.value = await getBookshelf()
}

async function remove(b: ShelfBook) {
  await removeFromBookshelf(b.url)
  await load()
}

function openBook(b: ShelfBook) {
  const source = props.sources.find((s) => s.bookSourceName === b.source)
  const book: BookItem = {
    bookName: b.title,
    bookUrl: b.url,
    author: b.author,
    coverUrl: b.cover_url,
    source: b.source,
    sourceUrl: source?.bookSourceUrl ?? '',
  }
  emit('open', book, b.progress?.chapter_url)
}

function onImgError(e: Event) {
  ;(e.target as HTMLImageElement).style.display = 'none'
}
</script>

<style scoped>
.bookshelf-view {
  padding: 16px;
  max-width: 720px;
  margin: 0 auto;
  width: 100%;
  display: flex;
  flex-direction: column;
  gap: 14px;
}
.bs-header { display: flex; align-items: baseline; gap: 10px; }
.bs-header h2 { font-size: 20px; }
.bs-count { font-size: 13px; color: #666; }
.empty-tip {
  text-align: center;
  padding: 40px 10px;
  color: #999;
  font-size: 14px;
  line-height: 1.8;
  border: 1px dashed #ddd;
  border-radius: 8px;
}
.book-list { display: flex; flex-direction: column; gap: 8px; }
.book-item {
  display: flex;
  gap: 12px;
  align-items: center;
  padding: 10px;
  border: 1px solid #eee;
  border-radius: 10px;
  background: #fff;
  cursor: pointer;
  position: relative;
}
.book-item:hover { border-color: #1677ff; }
.cover {
  width: 56px;
  height: 76px;
  flex-shrink: 0;
  border-radius: 6px;
  overflow: hidden;
  background: #f0f0f0;
  display: flex;
  align-items: center;
  justify-content: center;
}
.cover img { width: 100%; height: 100%; object-fit: cover; }
.no-cover { font-size: 22px; }
.meta { flex: 1; min-width: 0; display: flex; flex-direction: column; gap: 3px; }
.name { font-size: 15px; font-weight: 600; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.sub { font-size: 12px; color: #888; }
.progress { font-size: 12px; color: #1677ff; }
.remove-btn {
  position: absolute;
  top: 6px;
  right: 6px;
  border: none;
  background: #fff;
  color: #bbb;
  font-size: 14px;
  cursor: pointer;
  width: 22px;
  height: 22px;
  border-radius: 50%;
}
.remove-btn:hover { color: #d4380d; background: #fff1f0; }
</style>
