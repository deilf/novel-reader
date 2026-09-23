// 书源与规则的类型定义（与 Legado 导出的书源 JSON 兼容）

export interface SearchRule {
  bookList?: string
  name?: string
  author?: string
  coverUrl?: string
  bookUrl?: string
  intro?: string
  kind?: string
  lastChapter?: string
}

export interface BookInfoRule {
  name?: string
  author?: string
  coverUrl?: string
  intro?: string
  kind?: string
  lastChapter?: string
  init?: string
}

export interface TocRule {
  chapterList?: string
  chapterName?: string
  chapterUrl?: string
  nextTocUrl?: string
}

export interface ContentRule {
  content?: string
  nextContent?: string
}

/** 书源，字段与 Legado 导出的书源 JSON 保持一致 */
export interface BookSource {
  bookSourceUrl: string
  bookSourceName: string
  bookSourceGroup?: string
  bookSourceType?: number
  bookUrlPattern?: string
  customOrder?: number
  enabled?: boolean
  enabledExplore?: boolean
  jsLib?: string
  enabledCookieJar?: boolean
  concurrentRate?: string
  header?: string
  loginUrl?: string
  loginUi?: string
  loginCheckJs?: string
  coverDecodeJs?: string
  bookSourceComment?: string
  variableComment?: string
  lastUpdateTime?: number
  respondTime?: number
  weight?: number
  exploreUrl?: string
  exploreScreen?: string
  ruleExplore?: SearchRule
  searchUrl?: string
  ruleSearch?: SearchRule
  ruleBookInfo?: BookInfoRule
  ruleToc?: TocRule
  ruleContent?: ContentRule
  ruleReview?: SearchRule
  eventListener?: boolean
  customButton?: boolean
  nextPageLazyLoad?: boolean
  homepageModules?: string
}

/** HTTP 请求（与 Rust 端 HttpRequest 对应） */
export interface HttpRequest {
  url: string
  method?: string
  headers?: Record<string, string>
  body?: string
  timeout_ms?: number
}

/** HTTP 响应（与 Rust 端 HttpResponse 对应） */
export interface HttpResponse {
  status: number
  final_url: string
  content_type: string
  body: string
}

/** 搜索结果（一本书） */
export interface BookItem {
  bookName: string
  bookUrl: string
  author?: string
  coverUrl?: string
  intro?: string
  kind?: string
  lastChapter?: string
  source: string
  sourceUrl: string
}

/** 章节 */
export interface Chapter {
  chapterName: string
  chapterUrl: string
  index: number
  wordCount?: number
}

/** 书籍详情 + 目录 */
export interface BookInfo {
  bookName: string
  bookUrl: string
  author?: string
  coverUrl?: string
  intro?: string
  kind?: string
  lastChapter?: string
  source: string
  sourceUrl: string
  chapters: Chapter[]
}

/** 正文页 */
export interface ChapterContent {
  title: string
  content: string
  sourceUrl: string
}
