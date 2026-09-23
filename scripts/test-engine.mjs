// 书源引擎端到端测试：本地 mock 小说站 + 真实引擎全链路
// 运行：node scripts/test-engine.mjs
// 覆盖：搜索 → 详情+目录 → 正文（CSS 规则）+ JSON 规则搜索

import { createServer } from 'node:http'
import { parseHTML } from 'linkedom'
import { setHttpFetchImpl } from '../src/lib/bookSource/http.ts'
import { searchBySource, getBookInfoAndToc, getChapterContent } from '../src/lib/bookSource/runner.ts'

const PORT = 18765
const BASE = `http://localhost:${PORT}`

// ---- 浏览器环境模拟 ----
globalThis.DOMParser = class {
  parseFromString(html, _type) {
    return parseHTML(html).document
  }
}
// XPath 在 linkedom 中不支持：stub 返回空（测试不覆盖 XPath 规则）
globalThis.document = {
  evaluate: () => ({ snapshotLength: 0, snapshotItem: () => null }),
}

// ---- mock 小说站 ----
const pages = {
  '/search': (url) => {
    const q = new URL(url, BASE).searchParams.get('q')
    return `<!DOCTYPE html><html><body>
      <div class="book-item"><h2><a class="name" href="/book/101">${q}之第一章</a></h2><span class="author">测试作者</span></div>
      <div class="book-item"><h2><a class="name" href="/book/102">${q}之第二章</a></h2><span class="author">另一位作者</span></div>
    </body></html>`
  },
  '/api/search': (url) => {
    const q = new URL(url, BASE).searchParams.get('q')
    return JSON.stringify({
      code: 0,
      data: {
        books: [
          { name: `${q}之第一章`, author: '测试作者', url: `${BASE}/book/101` },
          { name: `${q}之第二章`, author: '另一位作者', url: `${BASE}/book/102` },
        ],
      },
    })
  },
  '/book/101': () => `<!DOCTYPE html><html><body>
    <div class="info"><h1>测试之第一章</h1><p class="author">测试作者</p><p class="intro">这是一本测试小说的简介内容。</p></div>
    <ul id="toc">
      <li><a class="chapter" href="/chapter/101/1">第一章 开始</a></li>
      <li><a class="chapter" href="/chapter/101/2">第二章 发展</a></li>
      <li><a class="chapter" href="/chapter/101/3">第三章 高潮</a></li>
    </ul>
  </body></html>`,
  '/chapter/101/1': () => `<!DOCTYPE html><html><body><div class="content">
    <p>第一章正文第一段：这是正文内容。</p><p>第一章正文第二段：继续阅读。</p>
  </div></body></html>`,
  '/chapter/101/2': () => `<!DOCTYPE html><html><body><div class="content"><p>第二章正文内容。</p></div></body></html>`,
  '/chapter/101/3': () => `<!DOCTYPE html><html><body><div class="content"><p>第三章正文内容。</p></div></body></html>`,
}

const server = createServer((req, res) => {
  const url = req.url ?? '/'
  const path = url.split('?')[0]
  const handler = pages[path]
  if (!handler) {
    res.writeHead(404)
    res.end('not found')
    return
  }
  const isJson = path.startsWith('/api/')
  res.writeHead(200, {
    'Content-Type': isJson ? 'application/json; charset=utf-8' : 'text/html; charset=utf-8',
  })
  res.end(handler(url))
})

// ---- 用 node fetch 作为引擎的 HTTP 实现 ----
setHttpFetchImpl(async (request) => {
  const resp = await fetch(request.url, {
    method: request.method ?? 'GET',
    headers: request.headers ?? {},
    body: request.body,
  })
  return {
    status: resp.status,
    final_url: resp.url,
    content_type: resp.headers.get('content-type') ?? '',
    body: await resp.text(),
  }
})

function makeSource() {
  return {
    bookSourceUrl: BASE,
    bookSourceName: '测试书源',
    enabled: true,
    searchUrl: `${BASE}/search?q={{key}}`,
    ruleSearch: {
      bookList: '{{$.css("div.book-item")}}',
      name: '{{$.css("a.name@text")}}',
      author: '{{$.css("span.author@text")}}',
      bookUrl: '{{$.css("a.name@href")}}',
    },
    ruleBookInfo: {
      name: '{{$.css("h1@text")}}',
      author: '{{$.css("p.author@text")}}',
      intro: '{{$.css("p.intro@text")}}',
    },
    ruleToc: {
      chapterList: '{{$.css("#toc a.chapter")}}',
      chapterName: '{{$.css("@text")}}',
      chapterUrl: '{{$.css("@href")}}',
    },
    ruleContent: {
      content: '{{$.css("div.content@text")}}',
    },
  }
}

function makeJsonSource() {
  return {
    bookSourceUrl: BASE,
    bookSourceName: 'JSON书源',
    enabled: true,
    searchUrl: `${BASE}/api/search?q={{key}}`,
    ruleSearch: {
      bookList: '$.data.books',
      name: '{{$.json("$.name")}}',
      author: '{{$.json("$.author")}}',
      bookUrl: '{{$.json("$.url")}}',
    },
    ruleBookInfo: {},
    ruleToc: {
      chapterList: '{{$.css("#toc a.chapter")}}',
      chapterName: '{{$.css("@text")}}',
      chapterUrl: '{{$.css("@href")}}',
    },
    ruleContent: {
      content: '{{$.css("div.content@text")}}',
    },
  }
}

let failures = 0
function assert(cond, msg) {
  if (cond) {
    console.log(`  ✓ ${msg}`)
  } else {
    failures++
    console.error(`  ✗ ${msg}`)
  }
}

async function main() {
  await new Promise((r) => server.listen(PORT, r))
  console.log('mock 站已启动，开始引擎全链路测试\n')

  // 1. 搜索（CSS）
  console.log('[1] 搜索（CSS 规则）')
  const books = await searchBySource(makeSource(), '测试')
  assert(books.length === 2, `搜索返回 2 本书（实际 ${books.length}）`)
  assert(books[0].bookName === '测试之第一章', `书名解析正确（实际 ${books[0].bookName}）`)
  assert(books[0].author === '测试作者', `作者解析正确（实际 ${books[0].author}）`)
  assert(books[0].bookUrl.startsWith('http://localhost'), `书籍 URL 解析为绝对地址（实际 ${books[0].bookUrl}）`)

  // 2. 搜索（JSON 规则）
  console.log('[2] 搜索（JSON 规则）')
  const jsonBooks = await searchBySource(makeJsonSource(), '测试')
  assert(jsonBooks.length === 2, `JSON 搜索返回 2 本书（实际 ${jsonBooks.length}）`)
  assert(jsonBooks[0].bookName === '测试之第一章', `JSON 书名解析（实际 ${jsonBooks[0].bookName}）`)
  assert(jsonBooks[0].author === '测试作者', `JSON 作者解析（实际 ${jsonBooks[0].author}）`)

  // 3. 详情 + 目录
  console.log('[3] 详情 + 目录')
  const info = await getBookInfoAndToc(makeSource(), books[0])
  assert(info.bookName === '测试之第一章', `详情书名解析（实际 ${info.bookName}）`)
  assert(info.author === '测试作者', `详情作者解析（实际 ${info.author}）`)
  assert(info.intro.includes('简介'), `详情简介解析（实际 ${info.intro}）`)
  assert(info.chapters.length === 3, `目录 3 章（实际 ${info.chapters.length}）`)
  assert(info.chapters[0].chapterName === '第一章 开始', `章节名解析（实际 ${info.chapters[0].chapterName}）`)
  assert(info.chapters[0].chapterUrl.startsWith('http://localhost'), `章节 URL 绝对化（实际 ${info.chapters[0].chapterUrl}）`)

  // 4. 正文
  console.log('[4] 正文')
  const content = await getChapterContent(makeSource(), books[0], info.chapters[0])
  assert(content.content.includes('第一章正文第一段'), `正文解析包含正文段落（实际前 40 字：${content.content.slice(0, 40)}）`)
  assert(content.content.includes('第一章正文第二段'), '正文解析包含全部段落')
  assert(!content.content.includes('<p>'), '正文已去除 HTML 标签')
  assert(content.title === '第一章 开始', '正文标题正确')

  console.log('\n' + (failures === 0 ? '✅ 全部测试通过' : `❌ ${failures} 项失败`))
  server.close()
  process.exit(failures === 0 ? 0 : 1)
}

main().catch((e) => {
  console.error('测试执行异常：', e)
  server.close()
  process.exit(1)
})
