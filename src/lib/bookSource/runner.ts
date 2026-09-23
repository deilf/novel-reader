// 书源执行流程：搜索 → 详情+目录 → 正文
//
// 流程与 Legado 一致：
// 1. searchUrl 模板 → 请求 → ruleSearch.bookList 列表 → 子规则提取书名/链接/作者...
// 2. 详情页（detailUrl 或书籍 URL）→ ruleBookInfo + ruleToc 目录
// 3. 正文页（chapterUrl）→ ruleContent.content（含 nextContent 翻页）

import { buildUrl, evalElementList, evalRule, makeScope, parseHeader, resolveUrl } from './engine'
import { httpFetch } from './http'
import type { BookItem, BookInfo, BookSource, Chapter, ChapterContent } from './types'

export interface SourceState {
  source: BookSource
  error?: string
}

/** 请求页面，自动判断 JSON/HTML */
async function fetchPage(source: BookSource, url: string): Promise<{ html: string; json: any; finalUrl: string }> {
  const resp = await httpFetch({
    url,
    headers: parseHeader(source.header),
    timeout_ms: 20_000,
  })
  if (resp.status >= 400) {
    throw new Error(`HTTP ${resp.status}`)
  }
  const body = resp.body.trim()
  let json: any = null
  let html = body
  if (body.startsWith('{') || body.startsWith('[')) {
    try {
      json = JSON.parse(body)
      html = ''
    } catch {
      // 不是 JSON，按 HTML 处理
    }
  }
  return { html, json, finalUrl: resp.final_url || url }
}

/** 判断响应是否 JSON */
function isJsonResponse(json: any): boolean {
  return json !== null
}

// ---------------------------------------------------------------- 搜索

/**
 * 用单个书源搜索
 */
export async function searchBySource(source: BookSource, key: string): Promise<BookItem[]> {
  if (!source.enabled) return []
  if (!source.searchUrl || !source.ruleSearch?.bookList) return []

  const baseUrl = source.bookSourceUrl
  const vars: Record<string, string> = {
    key,
    searchKey: key,
    page: '1',
    baseUrl,
  }

  const url = buildUrl(source.searchUrl, vars)
  const { html, json, finalUrl } = await fetchPage(source, url)
  void html

  // 确定基准 URL（跟随重定向后的页面）
  const base = finalUrl || baseUrl

  if (isJsonResponse(json)) {
    const listRule = source.ruleSearch.bookList || ''
    // JSON 顶层列表：jsonPath 求值
    const { jsonPath } = await import('./engine')
    const items = jsonPath(json, listRule.trim())
    if (!Array.isArray(items)) return []

    const books: BookItem[] = []
    for (const item of items) {
      if (!item || typeof item !== 'object') continue
      const itemScope = makeScope('', item, base, source, vars)
      const bookUrl = evalRule(source.ruleSearch?.bookUrl, itemScope)
      const bookName = evalRule(source.ruleSearch?.name, itemScope)
      if (!bookUrl || !bookName) continue
      books.push({
        bookName,
        bookUrl: resolveUrl(bookUrl, base),
        author: evalRule(source.ruleSearch?.author, itemScope),
        coverUrl: resolveUrl(evalRule(source.ruleSearch?.coverUrl, itemScope), base),
        intro: evalRule(source.ruleSearch?.intro, itemScope),
        kind: evalRule(source.ruleSearch?.kind, itemScope),
        lastChapter: evalRule(source.ruleSearch?.lastChapter, itemScope),
        source: source.bookSourceName,
        sourceUrl: source.bookSourceUrl,
      })
    }
    return books
  }

  const scope = makeScope(html, null, base, source, vars)
  const items = evalElementList(source.ruleSearch.bookList, scope)
  const books: BookItem[] = []
  for (const el of items) {
    const itemScope = makeScope(html, null, base, source, vars)
    itemScope.elements = [el]
    const bookUrl = evalRule(source.ruleSearch?.bookUrl, itemScope)
    const bookName = evalRule(source.ruleSearch?.name, itemScope)
    if (!bookUrl || !bookName) continue
    books.push({
      bookName,
      bookUrl: resolveUrl(bookUrl, base),
      author: evalRule(source.ruleSearch?.author, itemScope),
      coverUrl: resolveUrl(evalRule(source.ruleSearch?.coverUrl, itemScope), base),
      intro: evalRule(source.ruleSearch?.intro, itemScope),
      kind: evalRule(source.ruleSearch?.kind, itemScope),
      lastChapter: evalRule(source.ruleSearch?.lastChapter, itemScope),
      source: source.bookSourceName,
      sourceUrl: source.bookSourceUrl,
    })
  }
  return books
}

/**
 * 多书源并发搜索（互不阻塞，单个失败不影响其它）
 */
export async function searchAll(sources: BookSource[], key: string): Promise<{ books: BookItem[]; states: SourceState[] }> {
  const states: SourceState[] = []
  const results = await Promise.allSettled(
    sources.filter((s) => s.enabled).map(async (s) => {
      try {
        const books = await searchBySource(s, key)
        states.push({ source: s })
        return books
      } catch (e) {
        states.push({ source: s, error: String(e) })
        return []
      }
    }),
  )
  const books = results.flatMap((r) => (r.status === 'fulfilled' ? r.value : []))
  return { books, states }
}

// ---------------------------------------------------------------- 详情 + 目录

/**
 * 获取书籍详情与目录
 */
export async function getBookInfoAndToc(source: BookSource, book: BookItem): Promise<BookInfo> {
  const baseUrl = source.bookSourceUrl
  const vars: Record<string, string> = {
    key: book.bookName,
    bookName: book.bookName,
    author: book.author ?? '',
    bookUrl: book.bookUrl,
    baseUrl,
  }

  // 详情页 URL：优先 detailUrl 模板，否则用书籍 URL
  let detailUrl: string
  if (source.searchUrl && source.ruleToc?.chapterList) {
    // 无独立 detailUrl 字段时，目录解析基于书籍页
    detailUrl = book.bookUrl
  } else {
    detailUrl = book.bookUrl
  }
  // Legado 无 detailUrl 字段，目录解析页面即书籍详情页
  void vars

  const { html, json, finalUrl } = await fetchPage(source, detailUrl)
  const base = finalUrl || baseUrl

  const scope = makeScope(html, json, base, source, vars)

  const bookInfo: BookInfo = {
    bookName: evalRule(source.ruleBookInfo?.name, scope) || book.bookName,
    bookUrl: book.bookUrl,
    author: evalRule(source.ruleBookInfo?.author, scope) || book.author || '',
    coverUrl: resolveUrl(evalRule(source.ruleBookInfo?.coverUrl, scope), base) || book.coverUrl || '',
    intro: evalRule(source.ruleBookInfo?.intro, scope) || book.intro || '',
    kind: evalRule(source.ruleBookInfo?.kind, scope) || book.kind || '',
    lastChapter: evalRule(source.ruleBookInfo?.lastChapter, scope) || book.lastChapter || '',
    source: source.bookSourceName,
    sourceUrl: source.bookSourceUrl,
    chapters: [],
  }

  // 目录解析
  const tocRule = source.ruleToc
  if (tocRule?.chapterList) {
    const chapterScope = makeScope(html, json, base, source, vars)
    const chapterEls = evalElementList(tocRule.chapterList, chapterScope)
    const chapters: Chapter[] = []
    for (let i = 0; i < chapterEls.length; i++) {
      const el = chapterEls[i]
      const itemScope = makeScope(html, json, base, source, vars)
      itemScope.elements = [el]
      const chapterUrl = evalRule(tocRule.chapterUrl, itemScope)
      const chapterName = evalRule(tocRule.chapterName, itemScope)
      if (!chapterUrl || !chapterName) continue
      chapters.push({
        chapterName,
        chapterUrl: resolveUrl(chapterUrl, base),
        index: chapters.length,
      })
    }
    bookInfo.chapters = chapters
  }

  return bookInfo
}

// ---------------------------------------------------------------- 正文

/**
 * 获取章节正文（含 nextContent 翻页）
 */
export async function getChapterContent(
  source: BookSource,
  book: BookItem,
  chapter: Chapter,
): Promise<ChapterContent> {
  const baseUrl = source.bookSourceUrl
  const vars: Record<string, string> = {
    bookName: book.bookName,
    author: book.author ?? '',
    bookUrl: book.bookUrl,
    chapterUrl: chapter.chapterUrl,
    chapterName: chapter.chapterName,
    baseUrl,
  }

  const contentRule = source.ruleContent
  if (!contentRule?.content) {
    throw new Error('书源缺少正文规则')
  }

  // 正文页 URL = 章节 URL（Legado 无独立 contentUrl 字段）
  const url = chapter.chapterUrl
  const { html, json, finalUrl } = await fetchPage(source, url)
  const base = finalUrl || baseUrl

  const scope = makeScope(html, json, base, source, vars)
  let content = evalRule(contentRule.content, scope).trim()

  // nextContent 翻页（最多 5 页）
  if (contentRule.nextContent) {
    let nextUrl = evalRule(contentRule.nextContent, scope).trim()
    for (let page = 0; page < 5 && nextUrl && content.length < 3000; page++) {
      try {
        const nextPageUrl = resolveUrl(nextUrl, base)
        const { html: nextHtml, json: nextJson, finalUrl: nextFinal } = await fetchPage(source, nextPageUrl)
        const nextScope = makeScope(nextHtml, nextJson, nextFinal || base, source, vars)
        const nextContent = evalRule(contentRule.content, nextScope).trim()
        if (!nextContent) break
        content += '\n' + nextContent
        nextUrl = evalRule(contentRule.nextContent, nextScope).trim()
      } catch {
        break
      }
    }
  }

  // 移除常见残留标签与脚本
  content = sanitizeContent(content)

  return {
    title: chapter.chapterName,
    content,
    sourceUrl: source.bookSourceUrl,
  }
}

/** 净化正文：去标签（保留文本）、去脚本/样式、压缩空白 */
function sanitizeContent(content: string): string {
  if (!content) return ''
  // 若包含 HTML 标签则提取纯文本
  if (/<[a-zA-Z\/][^>]*>/.test(content)) {
    try {
      const doc = new DOMParser().parseFromString(content, 'text/html')
      doc.querySelectorAll('script, style, iframe, noscript').forEach((n) => n.remove())
      content = doc.body.textContent ?? ''
    } catch {
      content = content.replace(/<[^>]*>/g, '')
    }
  }
  // 解码常见实体
  content = content
    .replace(/&nbsp;/g, ' ')
    .replace(/&lt;/g, '<')
    .replace(/&gt;/g, '>')
    .replace(/&amp;/g, '&')
    .replace(/&quot;/g, '"')
    .replace(/&#(\d+);/g, (_, n) => (Number(n) > 0 ? String.fromCodePoint(Number(n)) : ''))
  // 压缩连续空行
  return content
    .split('\n')
    .map((l) => l.trim())
    .filter((l, i, arr) => !(l === '' && (i === 0 || arr[i - 1] === '')))
    .join('\n')
    .trim()
}
