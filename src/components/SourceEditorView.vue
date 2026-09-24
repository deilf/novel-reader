<template>
  <div class="source-editor">
    <div class="se-header">
      <button class="back-btn" @click="$emit('back')">← 返回</button>
      <h3>{{ isNew ? '新增书源' : '编辑书源' }}</h3>
      <button class="btn primary" :disabled="saving" @click="save">
        {{ saving ? '保存中...' : '保存' }}
      </button>
    </div>

    <!-- 基本信息 -->
    <div class="section">
      <div class="section-title">基本信息</div>
      <div class="form-grid">
        <label class="field">
          <span>书源名称 *</span>
          <input v-model="form.bookSourceName" placeholder="如：起点中文网" />
        </label>
        <label class="field">
          <span>书源 URL *</span>
          <input v-model="form.bookSourceUrl" placeholder="如：https://www.qidian.com" />
        </label>
        <label class="field">
          <span>分组</span>
          <input v-model="form.bookSourceGroup" placeholder="如：综合 / 轻小说" />
        </label>
        <label class="field">
          <span>备注</span>
          <input v-model="form.bookSourceComment" placeholder="说明文字" />
        </label>
        <label class="field wide">
          <span>请求头（JSON）</span>
          <textarea v-model="form.header" rows="2" placeholder='{"User-Agent": "..."}'></textarea>
        </label>
      </div>
    </div>

    <!-- 搜索规则 -->
    <div class="section">
      <div class="section-title">搜索规则 ruleSearch</div>
      <div class="form-grid">
        <label class="field wide">
          <span>搜索 URL（searchUrl）</span>
          <input v-model="form.searchUrl" placeholder="如：https://www.example.com/search?q={{key}}" />
        </label>
        <label class="field"><span>bookList</span><input v-model="form.ruleSearch.bookList" /></label>
        <label class="field"><span>name</span><input v-model="form.ruleSearch.name" /></label>
        <label class="field"><span>author</span><input v-model="form.ruleSearch.author" /></label>
        <label class="field"><span>coverUrl</span><input v-model="form.ruleSearch.coverUrl" /></label>
        <label class="field"><span>bookUrl</span><input v-model="form.ruleSearch.bookUrl" /></label>
        <label class="field"><span>intro</span><input v-model="form.ruleSearch.intro" /></label>
        <label class="field"><span>kind</span><input v-model="form.ruleSearch.kind" /></label>
        <label class="field"><span>lastChapter</span><input v-model="form.ruleSearch.lastChapter" /></label>
      </div>
    </div>

    <!-- 目录规则 -->
    <div class="section">
      <div class="section-title">目录规则 ruleToc</div>
      <div class="form-grid">
        <label class="field"><span>chapterList</span><input v-model="form.ruleToc.chapterList" /></label>
        <label class="field"><span>chapterName</span><input v-model="form.ruleToc.chapterName" /></label>
        <label class="field"><span>chapterUrl</span><input v-model="form.ruleToc.chapterUrl" /></label>
        <label class="field"><span>nextTocUrl</span><input v-model="form.ruleToc.nextTocUrl" /></label>
      </div>
    </div>

    <!-- 正文规则 -->
    <div class="section">
      <div class="section-title">正文规则 ruleContent</div>
      <div class="form-grid">
        <label class="field wide"><span>content</span><textarea v-model="form.ruleContent.content" rows="2"></textarea></label>
        <label class="field wide"><span>nextContent</span><input v-model="form.ruleContent.nextContent" /></label>
      </div>
    </div>

    <!-- 测试区 -->
    <div class="section">
      <div class="section-title">规则测试</div>
      <div class="test-row">
        <input v-model="testKeyword" placeholder="输入关键词搜索测试" @keydown.enter="runTest" />
        <button class="btn" :disabled="testing || !testKeyword.trim()" @click="runTest">
          {{ testing ? '测试中...' : '测试搜索' }}
        </button>
      </div>
      <div v-if="testError" class="test-error">{{ testError }}</div>
      <div v-if="testResults.length > 0" class="test-results">
        <div v-for="(r, i) in testResults" :key="i" class="test-item">
          <span class="t-name">{{ r.bookName }}</span>
          <span class="t-author" v-if="r.author">{{ r.author }}</span>
          <span class="t-last" v-if="r.lastChapter">{{ r.lastChapter }}</span>
        </div>
      </div>
    </div>

    <div v-if="saveError" class="save-error">{{ saveError }}</div>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref } from 'vue'
import type { BookSource } from '../lib/bookSource/types'
import { searchBySource } from '../lib/bookSource/runner'
import { updateBookSource } from '../lib/bookSource/store'

const props = defineProps<{ source: BookSource | null }>()
const emit = defineEmits<{ (e: 'back'): void; (e: 'saved'): void }>()

const isNew = !props.source

function clone(src: BookSource | null): BookSource {
  return src ? JSON.parse(JSON.stringify(src)) : {
    bookSourceUrl: '',
    bookSourceName: '',
    bookSourceGroup: '',
    bookSourceComment: '',
    header: '',
    searchUrl: '',
    enabled: true,
    ruleSearch: {},
    ruleBookInfo: {},
    ruleToc: {},
    ruleContent: {},
  }
}

const _raw = clone(props.source)
const form = reactive({
  ..._raw,
  header: (_raw.header ?? '') as string,
  ruleSearch: _raw.ruleSearch ?? {},
  ruleToc: _raw.ruleToc ?? {},
  ruleContent: _raw.ruleContent ?? {},
  ruleBookInfo: _raw.ruleBookInfo ?? {},
}) as BookSource & {
  header: string
  ruleSearch: NonNullable<BookSource['ruleSearch']>
  ruleToc: NonNullable<BookSource['ruleToc']>
  ruleContent: NonNullable<BookSource['ruleContent']>
  ruleBookInfo: NonNullable<BookSource['ruleBookInfo']>
}

const saving = ref(false)
const saveError = ref('')
const testKeyword = ref('')
const testing = ref(false)
const testError = ref('')
const testResults = ref<{ bookName: string; author?: string; lastChapter?: string }[]>([])

async function save() {
  if (!form.bookSourceName.trim() || !form.bookSourceUrl.trim()) {
    saveError.value = '书源名称和 URL 必填'
    return
  }
  saving.value = true
  saveError.value = ''
  try {
    let header: Record<string, string> | undefined
    if (form.header && form.header.trim()) {
      try {
        header = JSON.parse(form.header)
      } catch {
        saveError.value = '请求头不是合法 JSON'
        saving.value = false
        return
      }
    }
    const payload: BookSource = {
      ...form,
      header: header ? JSON.stringify(header) : undefined,
    }
    await updateBookSource(payload)
    emit('saved')
    emit('back')
  } catch (e) {
    saveError.value = `保存失败：${e}`
  } finally {
    saving.value = false
  }
}

async function runTest() {
  testing.value = true
  testError.value = ''
  testResults.value = []
  try {
    const source = { ...form } as BookSource
    const results = await searchBySource(source, testKeyword.value.trim())
    testResults.value = results.map((r) => ({
      bookName: r.bookName,
      author: r.author,
      lastChapter: r.lastChapter,
    }))
  } catch (e) {
    testError.value = `测试失败：${e}`
  } finally {
    testing.value = false
  }
}
</script>

<style scoped>
.source-editor {
  padding: 14px 16px;
  max-width: 720px;
  margin: 0 auto;
  width: 100%;
  display: flex;
  flex-direction: column;
  gap: 12px;
}
.se-header { display: flex; align-items: center; gap: 10px; }
.se-header h3 { font-size: 17px; flex: 1; }
.back-btn {
  border: 1px solid #ccc;
  background: #fff;
  border-radius: 6px;
  padding: 6px 10px;
  font-size: 13px;
  cursor: pointer;
}
.btn {
  padding: 6px 14px;
  border-radius: 6px;
  border: 1px solid #ccc;
  background: #fff;
  font-size: 13px;
  cursor: pointer;
}
.btn.primary { background: #1677ff; color: #fff; border-color: #1677ff; }
.btn.primary:disabled { opacity: 0.5; }
.section {
  border: 1px solid #eee;
  border-radius: 10px;
  background: #fff;
  padding: 12px;
}
.section-title { font-size: 14px; font-weight: 600; margin-bottom: 10px; color: #333; }
.form-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 10px; }
.field { display: flex; flex-direction: column; gap: 4px; }
.field.wide { grid-column: 1 / -1; }
.field span { font-size: 12px; color: #888; }
.field input, .field textarea {
  padding: 8px 10px;
  border: 1px solid #ddd;
  border-radius: 6px;
  font-size: 13px;
  width: 100%;
  box-sizing: border-box;
  font-family: inherit;
  resize: vertical;
}
.field textarea { min-height: 48px; }
.test-row { display: flex; gap: 8px; }
.test-row input {
  flex: 1;
  padding: 8px 10px;
  border: 1px solid #ddd;
  border-radius: 6px;
  font-size: 13px;
}
.test-error, .save-error { color: #d4380d; font-size: 13px; }
.test-results { display: flex; flex-direction: column; gap: 6px; margin-top: 10px; }
.test-item {
  display: flex;
  gap: 10px;
  align-items: baseline;
  padding: 8px 10px;
  border: 1px solid #f0f0f0;
  border-radius: 6px;
  font-size: 13px;
}
.t-name { font-weight: 600; }
.t-author, .t-last { color: #888; font-size: 12px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
</style>
