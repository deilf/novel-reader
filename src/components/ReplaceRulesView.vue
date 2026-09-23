<template>
  <div class="replace-view">
    <div class="rv-header">
      <h2>替换净化</h2>
      <span class="rv-count">{{ rules.length }} 条规则</span>
    </div>

    <!-- 新增规则 -->
    <div class="rule-form">
      <input v-model="form.name" placeholder="规则名称（如：去广告段落）" />
      <div class="form-row">
        <input v-model="form.replaceRegex" placeholder="匹配文本（支持正则）" />
        <input v-model="form.replacement" placeholder="替换为（留空=删除）" />
      </div>
      <div class="form-options">
        <label><input type="checkbox" v-model="form.isRegex" /> 正则</label>
        <label>范围
          <select v-model="form.scope">
            <option value="content">正文</option>
            <option value="toc">目录</option>
            <option value="global">全部</option>
          </select>
        </label>
        <button class="btn primary" :disabled="!form.name || !form.replaceRegex" @click="addRule">添加</button>
      </div>
    </div>

    <div v-if="rules.length === 0" class="empty-tip">
      还没有替换规则。可以添加"正文里去掉推荐语/广告段落"之类的净化规则。
    </div>

    <div v-else class="rule-list">
      <div v-for="r in rules" :key="r.name" class="rule-item">
        <div class="rule-main">
          <div class="rule-name">
            <span class="dot" :class="{ on: r.enabled }"></span>
            {{ r.name }}
            <span class="rule-tag">{{ r.isRegex ? '正则' : '文本' }} · {{ r.scope }}</span>
          </div>
          <div class="rule-detail">{{ r.replaceRegex }} → {{ r.replacement || '（删除）' }}</div>
        </div>
        <div class="rule-actions">
          <button class="btn small" :class="r.enabled ? 'on' : ''" @click="toggle(r)">{{ r.enabled ? '启用' : '停用' }}</button>
          <button class="btn small danger" @click="remove(r)">删除</button>
        </div>
      </div>
    </div>

    <div v-if="rules.length > 0" class="rv-footer">
      <button class="btn danger" @click="clearAll">清空全部</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, reactive, ref } from 'vue'
import type { ReplaceRule } from '../lib/bookSource/store'
import {
  clearReplaceRules,
  deleteReplaceRule,
  importReplaceRules,
  listReplaceRules,
  toggleReplaceRule,
} from '../lib/bookSource/store'

const rules = ref<ReplaceRule[]>([])
const form = reactive({
  name: '',
  replaceRegex: '',
  replacement: '',
  isRegex: true,
  scope: 'content',
})

onMounted(load)

async function load() {
  rules.value = await listReplaceRules()
}

async function addRule() {
  await importReplaceRules([
    {
      name: form.name.trim(),
      enabled: true,
      isRegex: form.isRegex,
      replaceRegex: form.replaceRegex,
      replacement: form.replacement,
      scope: form.scope,
    },
  ])
  form.name = ''
  form.replaceRegex = ''
  form.replacement = ''
  await load()
}

async function toggle(r: ReplaceRule) {
  await toggleReplaceRule(r.name)
  await load()
}

async function remove(r: ReplaceRule) {
  await deleteReplaceRule(r.name)
  await load()
}

async function clearAll() {
  await clearReplaceRules()
  await load()
}
</script>

<style scoped>
.replace-view {
  padding: 16px;
  max-width: 720px;
  margin: 0 auto;
  width: 100%;
  display: flex;
  flex-direction: column;
  gap: 14px;
}
.rv-header { display: flex; align-items: baseline; gap: 10px; }
.rv-header h2 { font-size: 20px; }
.rv-count { font-size: 13px; color: #666; }
.rule-form {
  border: 1px solid #eee;
  border-radius: 10px;
  padding: 12px;
  background: #fff;
  display: flex;
  flex-direction: column;
  gap: 8px;
}
.rule-form input {
  padding: 8px 10px;
  border: 1px solid #ddd;
  border-radius: 6px;
  font-size: 13px;
  width: 100%;
  box-sizing: border-box;
}
.form-row { display: flex; gap: 8px; }
.form-options { display: flex; align-items: center; gap: 12px; font-size: 13px; color: #555; }
.form-options select { padding: 4px 6px; border-radius: 4px; border: 1px solid #ddd; }
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
.btn.danger { color: #d4380d; border-color: #ffa39e; }
.btn.small { padding: 3px 10px; font-size: 12px; }
.btn.small.on { background: #f6ffed; color: #52c41a; border-color: #b7eb8f; }
.empty-tip { text-align: center; color: #999; padding: 30px 10px; font-size: 14px; border: 1px dashed #ddd; border-radius: 8px; line-height: 1.8; }
.rule-list { display: flex; flex-direction: column; gap: 8px; }
.rule-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 10px;
  padding: 10px 12px;
  border: 1px solid #eee;
  border-radius: 8px;
  background: #fff;
}
.rule-main { min-width: 0; flex: 1; }
.rule-name { font-size: 14px; font-weight: 600; display: flex; align-items: center; gap: 6px; flex-wrap: wrap; }
.dot { width: 8px; height: 8px; border-radius: 50%; background: #ccc; display: inline-block; }
.dot.on { background: #52c41a; }
.rule-tag { font-size: 11px; color: #888; background: #f5f5f5; padding: 1px 6px; border-radius: 8px; }
.rule-detail { font-size: 12px; color: #999; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; margin-top: 2px; }
.rule-actions { display: flex; gap: 6px; flex-shrink: 0; }
.rv-footer { display: flex; justify-content: flex-end; }
</style>
