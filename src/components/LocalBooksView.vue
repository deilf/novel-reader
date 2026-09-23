<template>
  <div class="local-view">
    <div class="lv-header">
      <h2>本地书籍</h2>
      <button class="btn primary" :disabled="importing" @click="importBook">
        {{ importing ? '导入中...' : '导入 TXT / EPUB' }}
      </button>
    </div>

    <div v-if="importMsg" class="import-msg" :class="{ error: importError }">{{ importMsg }}</div>

    <div v-if="books.length === 0" class="empty-tip">
      还没有本地书籍。<br />
      点击右上角导入 TXT 或 EPUB 文件。
    </div>

    <div v-else class="book-list">
      <div v-for="b in books" :key="b.id" class="book-item" @click="openBook(b)">
        <div class="meta">
          <div class="name">{{ b.title }}</div>
          <div class="sub">{{ b.format.toUpperCase() }} · {{ b.chapter_count }} 章<template v-if="b.author"> · {{ b.author }}</template></div>
        </div>
        <button class="remove-btn" @click.stop="remove(b)">✕</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import type { LocalBook } from '../lib/bookSource/store'
import { importLocalBook, listLocalBooks, removeLocalBook } from '../lib/bookSource/store'

const emit = defineEmits<{ (e: 'open', book: LocalBook): void }>()

const books = ref<LocalBook[]>([])
const importing = ref(false)
const importMsg = ref('')
const importError = ref(false)

onMounted(load)

async function load() {
  books.value = await listLocalBooks()
}

async function importBook() {
  try {
    importing.value = true
    importError.value = false
    const path = await open({
      multiple: false,
      filters: [
        { name: '书籍文件', extensions: ['txt', 'epub'] },
      ],
    })
    if (typeof path !== 'string' || !path) return
    const book = await importLocalBook(path)
    importMsg.value = `导入成功：${book.title}（${book.chapter_count} 章）`
    await load()
  } catch (e) {
    importError.value = true
    importMsg.value = `导入失败：${e}`
  } finally {
    importing.value = false
  }
}

async function remove(b: LocalBook) {
  await removeLocalBook(b.id)
  await load()
}

function openBook(b: LocalBook) {
  emit('open', b)
}
</script>

<style scoped>
.local-view {
  padding: 16px;
  max-width: 720px;
  margin: 0 auto;
  width: 100%;
  display: flex;
  flex-direction: column;
  gap: 14px;
}
.lv-header { display: flex; align-items: center; justify-content: space-between; gap: 10px; }
.lv-header h2 { font-size: 20px; }
.btn.primary {
  padding: 8px 16px;
  border-radius: 8px;
  border: 1px solid #1677ff;
  background: #1677ff;
  color: #fff;
  font-size: 13px;
  cursor: pointer;
}
.btn.primary:disabled { opacity: 0.5; }
.import-msg { font-size: 13px; color: #52c41a; }
.import-msg.error { color: #d4380d; }
.empty-tip { text-align: center; color: #999; padding: 40px 10px; font-size: 14px; border: 1px dashed #ddd; border-radius: 8px; line-height: 1.8; }
.book-list { display: flex; flex-direction: column; gap: 8px; }
.book-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 10px;
  padding: 12px;
  border: 1px solid #eee;
  border-radius: 8px;
  background: #fff;
  cursor: pointer;
}
.book-item:hover { border-color: #1677ff; }
.meta { min-width: 0; flex: 1; }
.name { font-size: 15px; font-weight: 600; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.sub { font-size: 12px; color: #888; margin-top: 2px; }
.remove-btn {
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
