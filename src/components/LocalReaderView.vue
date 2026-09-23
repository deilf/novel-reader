<template>
  <div class="local-reader">
    <div class="lr-header">
      <button class="back-btn" @click="$emit('back')">← 返回</button>
      <span class="title">{{ book.title }}</span>
    </div>

    <!-- 章节列表 -->
    <div v-if="!reading" class="chapter-list">
      <div class="chapter-header">目录（{{ chapters.length }} 章）</div>
      <div
        v-for="ch in chapters"
        :key="ch.index"
        class="chapter-item"
        @click="read(ch)"
      >
        <span class="ch-index">{{ ch.index + 1 }}</span>
        <span class="ch-name">{{ ch.title }}</span>
      </div>
    </div>

    <!-- 阅读 -->
    <div v-else class="reading">
      <div class="read-header">
        <button class="back-btn" @click="reading = false">← 目录</button>
        <span class="chapter-title">{{ currentTitle }}</span>
        <span class="loading" v-if="loading">加载中...</span>
        <div class="font-controls">
          <button class="font-btn" @click="fontSize -= 2" :disabled="fontSize <= 12">A-</button>
          <span class="font-size">{{ fontSize }}</span>
          <button class="font-btn" @click="fontSize += 2" :disabled="fontSize >= 28">A+</button>
        </div>
      </div>
      <div v-if="error" class="error-tip">{{ error }}</div>
      <div class="read-body" :style="{ fontSize: fontSize + 'px', lineHeight: 1.9 }">
        <div class="content-text">{{ content }}</div>
      </div>
      <div class="read-footer">
        <button class="nav-btn" :disabled="!prev || loading" @click="go(prev!)">上一章</button>
        <button class="nav-btn" :disabled="!next || loading" @click="go(next!)">下一章</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import type { LocalBook, LocalChapter } from '../lib/bookSource/store'
import { getLocalChapterContent, getLocalChapters } from '../lib/bookSource/store'

const props = defineProps<{ book: LocalBook }>()

const chapters = ref<LocalChapter[]>([])
const reading = ref(false)
const currentTitle = ref('')
const content = ref('')
const loading = ref(false)
const error = ref('')
const fontSize = ref(18)
const currentIndex = ref(0)

const prev = computed(() => chapters.value[currentIndex.value - 1])
const next = computed(() => chapters.value[currentIndex.value + 1])

onMounted(async () => {
  try {
    chapters.value = await getLocalChapters(props.book.id)
  } catch (e) {
    error.value = String(e)
  }
})

async function read(ch: LocalChapter) {
  currentIndex.value = ch.index
  currentTitle.value = ch.title
  reading.value = true
  loading.value = true
  error.value = ''
  content.value = ''
  try {
    content.value = await getLocalChapterContent(props.book.id, ch.index)
  } catch (e) {
    error.value = String(e)
  } finally {
    loading.value = false
  }
}

function go(ch: LocalChapter) {
  read(ch)
}
</script>

<style scoped>
.local-reader {
  display: flex;
  flex-direction: column;
  height: 100%;
  max-width: 760px;
  margin: 0 auto;
  width: 100%;
}
.lr-header {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 14px;
  border-bottom: 1px solid #eee;
  background: #fff;
}
.back-btn {
  border: 1px solid #ccc;
  background: #fff;
  border-radius: 6px;
  padding: 6px 10px;
  font-size: 13px;
  cursor: pointer;
}
.title { font-size: 15px; font-weight: 600; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.chapter-list { overflow-y: auto; flex: 1; }
.chapter-header { padding: 10px 14px; font-size: 14px; font-weight: 600; border-bottom: 1px solid #f0f0f0; background: #fafafa; }
.chapter-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 14px;
  border-bottom: 1px solid #f5f5f5;
  cursor: pointer;
  font-size: 14px;
}
.chapter-item:hover { background: #f6f9ff; }
.ch-index { font-size: 12px; color: #999; width: 28px; flex-shrink: 0; }
.ch-name { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.reading { display: flex; flex-direction: column; flex: 1; overflow: hidden; }
.read-header {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 14px;
  border-bottom: 1px solid #eee;
  background: #fff;
  flex-wrap: wrap;
}
.chapter-title { font-size: 15px; font-weight: 600; flex: 1; min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
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
.read-body {
  flex: 1;
  overflow-y: auto;
  padding: 18px 20px;
  background: #fbfaf6;
  color: #333;
}
.content-text { white-space: pre-wrap; word-break: break-word; }
.read-footer { display: flex; gap: 10px; padding: 10px 14px; border-top: 1px solid #eee; background: #fff; }
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
