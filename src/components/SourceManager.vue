<template>
  <div class="source-manager">
    <div class="sm-header">
      <h2>书源管理</h2>
      <span class="sm-count">{{ sources.length }} 个书源 · {{ enabledCount }} 个启用</span>
    </div>

    <!-- 导入区 -->
    <div class="import-box">
      <textarea
        v-model="importText"
        placeholder="粘贴 Legado 导出的书源 JSON（数组或单个对象）..."
        rows="4"
      ></textarea>
      <input v-model="importUrl" placeholder="或粘贴书源分享 URL（https://... 的 JSON）" />
      <div class="import-actions">
        <button class="btn primary" :disabled="(!importText.trim() && !importUrl.trim()) || importing" @click="handleImport">
          {{ importing ? '导入中...' : '导入书源' }}
        </button>
        <button class="btn" @click="exportAll">导出全部</button>
        <button class="btn" @click="importText = ''; importUrl = ''">清空</button>
      </div>
      <div v-if="importMessage" class="import-msg" :class="{ error: importError }">{{ importMessage }}</div>
    </div>

    <!-- 书源列表 -->
    <div class="source-list">
      <div v-if="sources.length === 0" class="empty-tip">
        还没有书源。<br />
        可以从 Legado 阅读 App 导出书源 JSON 粘贴到这里导入。
      </div>
      <div v-for="src in sources" :key="src.bookSourceUrl" class="source-item">
        <div class="src-info">
          <div class="src-name">
            <span class="dot" :class="{ on: src.enabled }"></span>
            {{ src.bookSourceName }}
            <span class="src-group" v-if="src.bookSourceGroup">{{ src.bookSourceGroup }}</span>
          </div>
          <div class="src-url">{{ src.bookSourceUrl }}</div>
          <div class="src-meta" v-if="src.bookSourceComment">{{ src.bookSourceComment }}</div>
        </div>
        <div class="src-actions">
          <button class="btn small" @click="edit(src)">编辑</button>
          <button
            class="btn small"
            :class="src.enabled ? 'on' : ''"
            @click="toggle(src)"
          >{{ src.enabled ? '启用' : '停用' }}</button>
          <button class="btn small danger" @click="remove(src)">删除</button>
        </div>
      </div>
    </div>

    <div v-if="sources.length > 0" class="sm-footer">
      <button class="btn danger" @click="clearAll">清空全部</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import type { BookSource } from '../lib/bookSource/types'
import { clearBookSources, deleteBookSource, importBookSources, toggleBookSource } from '../lib/bookSource/store'

const props = defineProps<{ sources: BookSource[] }>()
const emit = defineEmits<{ (e: 'refresh'): void; (e: 'edit', source: BookSource): void }>()

const importText = ref('')
const importUrl = ref('')
const importing = ref(false)
const importMessage = ref('')
const importError = ref(false)

const enabledCount = computed(() => props.sources.filter((s) => s.enabled).length)

function edit(src: BookSource) {
  emit('edit', src)
}

function exportAll() {
  const json = JSON.stringify(props.sources, null, 2)
  navigator.clipboard?.writeText(json).catch(() => {})
  importMessage.value = `已复制 ${props.sources.length} 个书源的 JSON 到剪贴板`
  importError.value = false
}

function parseImport(text: string): BookSource[] {
  const data = JSON.parse(text)
  if (Array.isArray(data)) return data
  return [data]
}

async function handleImport() {
  try {
    importing.value = true
    // URL 导入：拉取远程 JSON
    if (!importText.value.trim() && importUrl.value.trim()) {
      const { httpFetch } = await import('../lib/bookSource/http')
      const resp = await httpFetch({ url: importUrl.value.trim() })
      if (resp.status >= 400) {
        throw new Error(`HTTP ${resp.status}`)
      }
      importText.value = resp.body
    }
    importError.value = false
    const sources = parseImport(importText.value)
    if (sources.length === 0) {
      importMessage.value = '未解析到书源'
      return
    }
    const added = await importBookSources(sources)
    importMessage.value = `导入完成：新增 ${added} 个书源`
    importText.value = ''
    emit('refresh')
  } catch (e) {
    importError.value = true
    importMessage.value = `导入失败：${e}`
  } finally {
    importing.value = false
  }
}

async function toggle(src: BookSource) {
  await toggleBookSource(src.bookSourceUrl)
  emit('refresh')
}

async function remove(src: BookSource) {
  await deleteBookSource(src.bookSourceUrl)
  emit('refresh')
}

async function clearAll() {
  await clearBookSources()
  emit('refresh')
}
</script>

<style scoped>
.source-manager {
  display: flex;
  flex-direction: column;
  gap: 14px;
  padding: 16px;
  max-width: 720px;
  margin: 0 auto;
  width: 100%;
}
.sm-header {
  display: flex;
  align-items: baseline;
  gap: 10px;
  flex-wrap: wrap;
}
.sm-header h2 { font-size: 20px; }
.sm-count { font-size: 13px; color: #666; }
.import-box input {
  width: 100%;
  border: 1px solid #ddd;
  border-radius: 8px;
  padding: 10px;
  font-size: 13px;
  box-sizing: border-box;
  margin-top: 8px;
}
.import-box textarea {
  width: 100%;
  border: 1px solid #ddd;
  border-radius: 8px;
  padding: 10px;
  font-size: 13px;
  font-family: ui-monospace, monospace;
  resize: vertical;
  box-sizing: border-box;
}
.import-actions { display: flex; gap: 8px; margin-top: 8px; }
.btn {
  padding: 6px 14px;
  border-radius: 6px;
  border: 1px solid #ccc;
  background: #fff;
  font-size: 13px;
  cursor: pointer;
}
.btn.primary { background: #1677ff; color: #fff; border-color: #1677ff; }
.btn.primary:disabled { opacity: 0.5; cursor: not-allowed; }
.btn.danger { background: #fff; color: #d4380d; border-color: #ffa39e; }
.btn.small { padding: 3px 10px; font-size: 12px; }
.btn.small.on { background: #f6ffed; color: #52c41a; border-color: #b7eb8f; }
.import-msg { margin-top: 8px; font-size: 13px; color: #52c41a; }
.import-msg.error { color: #d4380d; }
.source-list { display: flex; flex-direction: column; gap: 8px; }
.empty-tip {
  text-align: center;
  padding: 30px 10px;
  color: #999;
  font-size: 14px;
  line-height: 1.8;
  border: 1px dashed #ddd;
  border-radius: 8px;
}
.source-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 10px;
  padding: 10px 12px;
  border: 1px solid #eee;
  border-radius: 8px;
  background: #fff;
}
.src-info { min-width: 0; flex: 1; }
.src-name { font-size: 14px; font-weight: 600; display: flex; align-items: center; gap: 6px; flex-wrap: wrap; }
.dot { width: 8px; height: 8px; border-radius: 50%; background: #ccc; display: inline-block; }
.dot.on { background: #52c41a; }
.src-group { font-size: 11px; color: #888; background: #f5f5f5; padding: 1px 6px; border-radius: 8px; }
.src-url { font-size: 12px; color: #888; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.src-meta { font-size: 12px; color: #aaa; margin-top: 2px; }
.src-actions { display: flex; gap: 6px; flex-shrink: 0; }
.sm-footer { display: flex; justify-content: flex-end; }
</style>
