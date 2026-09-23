<template>
  <div class="discover-view">
    <!-- 搜索栏 -->
    <div class="search-bar">
      <input
        v-model="keyword"
        type="text"
        placeholder="输入书名或作者，回车搜索"
        @keyup.enter="doSearch"
      />
      <button class="btn primary" :disabled="!keyword.trim() || searching" @click="doSearch">
        {{ searching ? '搜索中...' : '搜索' }}
      </button>
    </div>

    <div v-if="states.length > 0" class="src-states">
      <span
        v-for="s in states"
        :key="s.source.bookSourceUrl"
        class="state-chip"
        :class="s.error ? 'fail' : 'ok'"
        :title="s.error || undefined"
      >{{ s.source.bookSourceName }}{{ s.error ? ' ✗' : ' ✓' }}</span>
    </div>

    <!-- 结果 -->
    <div v-if="books.length > 0" class="results">
      <div class="results-header">
        <span class="count">找到 {{ books.length }} 本（来自 {{ sourceNames.length }} 个书源）</span>
        <span class="src-names">{{ sourceNames.join('、') }}</span>
      </div>
      <div class="book-grid">
        <div v-for="b in books" :key="b.sourceUrl + b.bookUrl" class="book-card" @click="open(b)">
          <div class="cover">
            <img v-if="b.coverUrl" :src="b.coverUrl" loading="lazy" @error="onImgError" alt="" />
            <span v-else class="no-cover">📖</span>
          </div>
          <div class="book-meta">
            <div class="book-name">{{ b.bookName }}</div>
            <div class="book-author">{{ b.author || '未知作者' }}</div>
            <div class="book-source">{{ b.source }}</div>
            <div class="book-intro" v-if="b.intro">{{ b.intro }}</div>
          </div>
        </div>
      </div>
    </div>

    <div v-else-if="searched && !searching" class="no-result">
      没有找到结果，试试更换关键词，或检查书源是否可用。
    </div>

    <div v-if="searching" class="searching-tip">正在并发搜索 {{ enabledSources.length }} 个书源...</div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import type { BookItem, BookSource } from '../lib/bookSource/types'
import { searchAll, type SourceState } from '../lib/bookSource/runner'

const props = defineProps<{ sources: BookSource[] }>()
const emit = defineEmits<{ (e: 'open', book: BookItem): void }>()

const keyword = ref('')
const searching = ref(false)
const searched = ref(false)
const books = ref<BookItem[]>([])
const states = ref<SourceState[]>([])

const enabledSources = computed(() => props.sources.filter((s) => s.enabled))
const sourceNames = computed(() => {
  const names = new Set(books.value.map((b) => b.source))
  return Array.from(names)
})

async function doSearch() {
  const key = keyword.value.trim()
  if (!key || searching.value) return
  searching.value = true
  searched.value = false
  states.value = []
  try {
    const result = await searchAll(enabledSources.value, key)
    books.value = result.books
    states.value = result.states
  } catch (e) {
    states.value = [{ source: { bookSourceUrl: '', bookSourceName: '系统' }, error: String(e) }]
  } finally {
    searching.value = false
    searched.value = true
  }
}

function open(book: BookItem) {
  emit('open', book)
}

function onImgError(e: Event) {
  const img = e.target as HTMLImageElement
  img.style.display = 'none'
}
</script>

<style scoped>
.discover-view {
  display: flex;
  flex-direction: column;
  gap: 12px;
  padding: 16px;
  max-width: 760px;
  margin: 0 auto;
  width: 100%;
}
.search-bar {
  display: flex;
  gap: 8px;
}
.search-bar input {
  flex: 1;
  padding: 10px 12px;
  border: 1px solid #ddd;
  border-radius: 8px;
  font-size: 15px;
}
.btn.primary {
  padding: 10px 18px;
  border-radius: 8px;
  border: 1px solid #1677ff;
  background: #1677ff;
  color: #fff;
  font-size: 14px;
  cursor: pointer;
}
.btn.primary:disabled { opacity: 0.5; cursor: not-allowed; }
.src-states { display: flex; flex-wrap: wrap; gap: 6px; }
.state-chip {
  font-size: 11px;
  padding: 2px 8px;
  border-radius: 10px;
  border: 1px solid;
}
.state-chip.ok { color: #52c41a; border-color: #b7eb8f; background: #f6ffed; }
.state-chip.fail { color: #d4380d; border-color: #ffa39e; background: #fff1f0; }
.searching-tip { font-size: 13px; color: #888; text-align: center; }
.results-header { display: flex; flex-direction: column; gap: 2px; }
.count { font-size: 13px; color: #666; }
.src-names { font-size: 11px; color: #aaa; }
.book-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
  gap: 10px;
}
.book-card {
  border: 1px solid #eee;
  border-radius: 10px;
  overflow: hidden;
  background: #fff;
  cursor: pointer;
  display: flex;
  flex-direction: column;
}
.book-card:hover { border-color: #1677ff; }
.cover {
  height: 180px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #f0f0f0;
  overflow: hidden;
}
.cover img { width: 100%; height: 100%; object-fit: cover; }
.no-cover { font-size: 40px; }
.book-meta { padding: 8px 10px; display: flex; flex-direction: column; gap: 2px; }
.book-name { font-size: 14px; font-weight: 600; }
.book-author { font-size: 12px; color: #666; }
.book-source { font-size: 11px; color: #999; }
.book-intro {
  font-size: 12px;
  color: #777;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
  margin-top: 4px;
}
.no-result { text-align: center; color: #999; padding: 30px 0; font-size: 14px; }
</style>
