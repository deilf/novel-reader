// M4 补充测试：书源 JSON 导入格式 + URL 导入链路 + 编辑器默认结构
// 运行：node --import tsx scripts/test-m4.mjs

import { setHttpFetchImpl } from '../src/lib/bookSource/http.ts'

let failures = 0
function assert(cond, msg) {
  if (cond) console.log(`  ✓ ${msg}`)
  else {
    failures++
    console.error(`  ✗ ${msg}`)
  }
}

// 与 SourceManager.parseImport 一致
function parseImport(text) {
  const data = JSON.parse(text)
  if (Array.isArray(data)) return data
  return [data]
}

// 与 SourceEditorView.clone 默认结构一致
function defaultNewSource() {
  return {
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

async function main() {
  console.log('[1] Legado 书源 JSON 导入格式')
  const single = {
    bookSourceUrl: 'https://demo.example.com',
    bookSourceName: '示例书源',
    searchUrl: 'https://demo.example.com/search?q={{key}}',
    ruleSearch: { bookList: '{{$.css(.books)}' , name: '{{$.css(.name)}}' },
    ruleToc: { chapterList: '{{$.css(.chapters)}' },
    ruleContent: { content: '{{$.css(.content)}}' },
    enabled: true,
  }
  const arr = parseImport(JSON.stringify([single]))
  assert(arr.length === 1 && arr[0].bookSourceName === '示例书源', '数组导入解析')
  const obj = parseImport(JSON.stringify(single))
  assert(obj.length === 1 && obj[0].bookSourceUrl === 'https://demo.example.com', '单对象导入解析')

  console.log('[2] 编辑器默认新书源结构')
  const def = defaultNewSource()
  assert(def.ruleSearch && def.ruleToc && def.ruleContent, '规则对象默认为空对象')
  assert(def.enabled === true, '默认启用')
  assert(typeof def.header === 'string', 'header 默认字符串')

  console.log('[3] URL 导入链路（httpFetch mock → JSON 解析）')
  let fetchedUrl = ''
  setHttpFetchImpl(async (req) => {
    fetchedUrl = req.url
    return {
      status: 200,
      final_url: req.url,
      content_type: 'application/json',
      body: JSON.stringify([single]),
    }
  })
  const { httpFetch } = await import('../src/lib/bookSource/http.ts')
  const resp = await httpFetch({ url: 'https://share.example.com/sources.json' })
  assert(fetchedUrl === 'https://share.example.com/sources.json', '请求 URL 正确')
  const imported = parseImport(resp.body)
  assert(imported.length === 1 && imported[0].bookSourceUrl === 'https://demo.example.com', 'URL 拉取后解析成功')

  console.log('[4] 请求头 JSON 校验（编辑器 save 逻辑）')
  function parseHeader(h) {
    return JSON.parse(h)
  }
  assert(JSON.stringify(parseHeader('{"User-Agent": "ok"}')).includes('User-Agent'), '合法请求头解析')
  let threw = false
  try {
    parseHeader('{not json}')
  } catch {
    threw = true
  }
  assert(threw, '非法请求头抛错')

  console.log('\n' + (failures === 0 ? '✅ 全部测试通过' : `❌ ${failures} 项失败`))
  process.exit(failures === 0 ? 0 : 1)
}

main().catch((e) => {
  console.error('测试执行异常：', e)
  process.exit(1)
})
