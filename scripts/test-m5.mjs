// M5 补充测试：jsLib 公共脚本注入 + 缓存
// 运行：node --import tsx scripts/test-m5.mjs

import { parseHTML } from 'linkedom'
import { evalRule, makeScope, runJs } from '../src/lib/bookSource/engine.ts'
import { setHttpFetchImpl } from '../src/lib/bookSource/http.ts'

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

const baseSource = {
  bookSourceUrl: 'https://demo.example.com',
  bookSourceName: '示例',
  enabled: true,
  ruleSearch: {},
  ruleToc: {},
  ruleContent: {},
}

function scope(html, jsLibCode, vars = {}) {
  return makeScope(html, null, 'https://demo.example.com', baseSource, vars, jsLibCode)
}

async function main() {
  console.log('[1] jsLib 函数注入规则 JS')
  const jsLib = `
function wrapTitle(t) { return '【' + t + '】' }
var MAX_LEN = 10
function cut(s) { return s.length > MAX_LEN ? s.slice(0, MAX_LEN) + '...' : s }
`
  const s1 = scope('<div class="b"><a class="n">测试书名</a></div>', jsLib)
  const r1 = evalRule('{{js.wrapTitle($.css(.n))}}', s1)
  assert(r1 === '【测试书名】', `jsLib 函数可用（实际 ${r1}）`)

  const r2 = evalRule('{{js.cut($.css(.n))}}', s1)
  assert(r2 === '测试书名', `jsLib var 可见（实际 ${r2}）`)

  console.log('[2] 无 jsLib 时行为不变')
  const s2 = scope('<div class="n">原样</div>', undefined)
  const r3 = evalRule('{{$.css(.n)}}', s2)
  assert(r3 === '原样', '无 jsLib 规则正常')
  // jsLib 里未定义的函数应返回空（不崩溃）
  const r4 = evalRule('{{js.wrapTitle($.css(.n))}}', s2)
  assert(r4 === '', `未定义函数返回空（实际 "${r4}"）`)

  console.log('[3] window 对象注入不崩溃')
  const jsLib2 = `window.__APP = 'nr'\nfunction getApp() { return window.__APP }`
  const s3 = scope('', jsLib2)
  const r5 = runJs('getApp()', s3)
  assert(r5 === 'nr', `jsLib 可读写注入的 window（实际 ${r5}）`)

  console.log('[4] loadJsLib 缓存（只请求一次）')
  let calls = 0
  setHttpFetchImpl(async (req) => {
    calls++
    return {
      status: 200,
      final_url: req.url,
      content_type: 'text/javascript',
      body: 'function helper() { return 42 }',
    }
  })
  const { loadJsLib } = await import('../src/lib/bookSource/runner.ts')
  const srcWithLib = { ...baseSource, jsLib: 'https://demo.example.com/lib.js' }
  const c1 = await loadJsLib(srcWithLib)
  const c2 = await loadJsLib(srcWithLib)
  assert(c1 === c2 && calls === 1, `缓存生效（请求次数 ${calls}）`)

  console.log('[5] jsLib 请求失败优雅降级')
  setHttpFetchImpl(async () => ({ status: 500, final_url: '', content_type: '', body: '' }))
  const c3 = await loadJsLib({ ...baseSource, jsLib: 'https://demo.example.com/bad.js' })
  assert(c3 === undefined, '加载失败返回 undefined')

  console.log('\n' + (failures === 0 ? '✅ 全部测试通过' : `❌ ${failures} 项失败`))
  process.exit(failures === 0 ? 0 : 1)
}

main().catch((e) => {
  console.error('测试执行异常：', e)
  process.exit(1)
})
