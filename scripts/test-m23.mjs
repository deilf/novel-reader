// M2/M3 补充测试：替换净化规则 + 简繁转换 + 本地 TXT 解析逻辑
// 运行：node scripts/test-m23.mjs（使用 tsx）

import { parseHTML } from 'linkedom'
import { applyReplaceRules, applyReplaceToToc } from '../src/lib/bookSource/replace.ts'

// ---- 浏览器环境模拟 ----
globalThis.DOMParser = class {
  parseFromString(html, _type) {
    return parseHTML(html).document
  }
}

let failures = 0
function assert(cond, msg) {
  if (cond) console.log(`  ✓ ${msg}`)
  else {
    failures++
    console.error(`  ✗ ${msg}`)
  }
}

async function main() {
  console.log('[1] 替换净化规则')
  const rules = [
    { name: '去推荐语', enabled: true, isRegex: true, replaceRegex: '本章推荐.*\\n', replacement: '', scope: 'content' },
    { name: '去广告', enabled: true, isRegex: false, replaceRegex: '【广告】', replacement: '', scope: 'content' },
    { name: '替换称呼', enabled: true, isRegex: false, replaceRegex: '主角', replacement: '小明', scope: 'content' },
    { name: '停用规则', enabled: false, isRegex: true, replaceRegex: '不该出现', replacement: '', scope: 'content' },
    { name: '目录净化', enabled: true, isRegex: true, replaceRegex: '（番外）', replacement: '', scope: 'toc' },
  ]
  const raw = '本章推荐：点我加入书友群\n【广告】精彩广告\n主角说：你好\n不该出现这句话\n正文继续'
  const cleaned = applyReplaceRules(raw, rules)
  assert(!cleaned.includes('本章推荐'), '正则净化删除推荐语')
  assert(!cleaned.includes('【广告】'), '文本净化删除广告')
  assert(cleaned.includes('小明说'), '文本替换生效')
  assert(cleaned.includes('不该出现'), '停用规则不生效')
  const toc = applyReplaceToToc('第一章（番外）开始', rules)
  assert(toc === '第一章开始', `目录净化生效（实际 ${toc}）`)

  console.log('[2] 简繁转换（opencc-js）')
  const { toTraditional } = await import('../src/lib/bookSource/traditional.ts')
  const tw = await toTraditional('小说阅读器，万古长青')
  assert(tw.includes('萬古長青'), `简→繁正确（实际 ${tw}）`)
  const { toSimplified } = await import('../src/lib/bookSource/traditional.ts')
  const cn = await toSimplified('萬古長青')
  assert(cn.includes('万古长青'), `繁→简正确（实际 ${cn}）`)

  console.log('[3] Rust 端逻辑（通过 npm run test:engine 已覆盖引擎）')
  console.log('    TXT/EPUB 解析在 Rust 端实现，由 CI 交叉编译验证 + 手动冒烟')

  console.log('\n' + (failures === 0 ? '✅ 全部测试通过' : `❌ ${failures} 项失败`))
  process.exit(failures === 0 ? 0 : 1)
}

main().catch((e) => {
  console.error('测试执行异常：', e)
  process.exit(1)
})
