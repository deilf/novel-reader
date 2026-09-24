// Legado 书源规则引擎（TS 重写）
//
// 支持语法子集（覆盖主流书源）：
// - 模板 {{key}} / {{page}} / {{bookUrl}} 等 URL 变量
// - {{$.css("选择器@属性")}} / {{$.xpath("...")}} / {{$.regex("...")}} / {{$.json("$.path")}}
// - {{js.代码}} / {{$.js(代码)}} / {{java.表达式}}
// - 前缀规则 @js: / @css: / @xpath: / @regex: / @json: / @get:
// - 多规则拼接 ||（取第一个非空）

import type { BookSource } from './types'

/** 求值作用域 */
export interface RuleScope {
  /** 当前页面 HTML（未解析字符串） */
  html: string
  /** 解析后的文档（懒缓存） */
  doc: Document | null
  /** 当前响应 JSON（若页面是 JSON） */
  json: any
  /** 当前作用域元素列表（如书列表的 item） */
  elements: Element[]
  /** 相对 URL 解析基准 */
  baseUrl: string
  /** URL 模板变量 */
  vars: Record<string, string>
  /** 当前书源 */
  source: BookSource
  /** jsLib 公共脚本代码（可选） */
  jsLibCode?: string
}

/** 全局变量存储（put/get） */
const globalVars: Record<string, string> = {}

// ---------------------------------------------------------------- URL 工具

/** 相对 URL 解析为绝对 URL */
export function resolveUrl(url: string, baseUrl: string): string {
  if (!url) return url
  if (/^https?:\/\//i.test(url)) return url
  try {
    return new URL(url, baseUrl).href
  } catch {
    return url
  }
}

/** URL 模板变量替换 + 编码 */
export function buildUrl(template: string, vars: Record<string, string>): string {
  let out = template.replace(/\{\{([\s\S]*?)\}\}/g, (_, name: string) => {
    const key = name.trim()
    if (key in vars) return vars[key]
    return ''
  })
  // 相对路径补全
  if (vars.baseUrl) out = resolveUrl(out, vars.baseUrl)
  // 编码非 ASCII（保留 URL 结构字符与已编码序列）
  try {
    const u = new URL(out)
    return u.href
  } catch {
    return encodeURI(out)
  }
}

// ---------------------------------------------------------------- 基础解析

function parseDoc(html: string): Document {
  const parser = new DOMParser()
  return parser.parseFromString(html, 'text/html')
}

/** 从 html 或元素作用域中取 css 匹配元素（空选择器时返回作用域元素自身） */
function cssSelect(scope: RuleScope, selector: string): Element[] {
  let nodes: Element[] = []
  if (selector.trim() === '') {
    return scope.elements.slice()
  }
  if (scope.elements.length > 0) {
    for (const el of scope.elements) {
      try {
        nodes = nodes.concat(Array.from(el.querySelectorAll(selector)))
      } catch { /* 选择器无效忽略 */ }
    }
  } else {
    let doc = scope.doc
    if (!doc) {
      doc = parseDoc(scope.html)
      scope.doc = doc
    }
    try {
      nodes = Array.from(doc.querySelectorAll(selector))
    } catch { /* 忽略 */ }
  }
  return nodes
}

/** XPath 求值（作用域元素相对或文档级） */
function xpathSelect(scope: RuleScope, expr: string): string[] {
  const results: string[] = []
  const collect = (node: Node) => {
    try {
      const snapshot = document.evaluate(
        expr,
        node,
        null,
        XPathResult.ORDERED_NODE_SNAPSHOT_TYPE,
        null,
      )
      for (let i = 0; i < snapshot.snapshotLength; i++) {
        const item = snapshot.snapshotItem(i)
        if (item instanceof Attr) results.push(item.value)
        else if (item instanceof Node) {
          results.push((item as Element).textContent?.trim() ?? '')
        }
      }
    } catch { /* 忽略无效 XPath */ }
  }
  if (scope.elements.length > 0) {
    for (const el of scope.elements) collect(el)
  } else {
    let doc = scope.doc
    if (!doc) {
      doc = parseDoc(scope.html)
      scope.doc = doc
    }
    collect(doc)
  }
  return results
}

/** JSONPath 子集：$.a.b、$[0]、$..key、$.a[*].b */
export function jsonPath(data: any, path: string): any {
  if (path === '$') return data
  const expr = path.replace(/^\$\.?/, '')
  if (expr === '') return data

  const tokens = expr
    .split(/\./)
    .flatMap((seg) => {
      const bracket = seg.match(/^([^\[]*)((\[[^\]]*\])*)$/)
      if (!bracket) return [seg]
      const parts: string[] = []
      if (bracket[1]) parts.push(bracket[1])
      const idxRe = /\[([^\]]*)\]/g
      let m: RegExpExecArray | null
      while ((m = idxRe.exec(bracket[2]))) parts.push(m[1])
      return parts
    })
    .filter((t) => t !== '')

  const evalTokens = (node: any, idx: number): any => {
    if (idx >= tokens.length) return node
    const token = tokens[idx]
    if (token === '*') {
      if (Array.isArray(node)) {
        return node.flatMap((item) => evalTokens(item, idx + 1))
      }
      if (node && typeof node === 'object') {
        return Object.values(node).flatMap((item) => evalTokens(item, idx + 1))
      }
      return undefined
    }
    if (/^\d+$/.test(token)) {
      if (Array.isArray(node)) return evalTokens(node[Number(token)], idx + 1)
      return undefined
    }
    if (token === '..') {
      // 递归搜索：返回所有匹配后续路径的值
      const rest = tokens.slice(idx + 1)
      const found: any[] = []
      // 简化实现：.. 视为递归查找键
      const collect = (n: any, depth: number): any => {
        if (n === null || n === undefined || depth > 8) return undefined
        if (Array.isArray(n)) {
          for (const item of n) {
            const r = collect(item, depth + 1)
            if (r !== undefined) found.push(r)
          }
          return undefined
        }
        if (typeof n === 'object') {
          for (const [k, v] of Object.entries(n)) {
            if (k === rest[0]) {
              const r = evalTokens(v, 1)
              if (r !== undefined) found.push(r)
            }
            const r = collect(v, depth + 1)
            if (r !== undefined) found.push(r)
          }
          return undefined
        }
        return undefined
      }
      collect(node, 0)
      return found.length === 1 ? found[0] : found
    }
    if (node && typeof node === 'object' && token in node) {
      return evalTokens(node[token], idx + 1)
    }
    // 大小写容错（Legado 书源常见）
    if (node && typeof node === 'object') {
      const lower = Object.keys(node).find((k) => k.toLowerCase() === token.toLowerCase())
      if (lower !== undefined) return evalTokens(node[lower], idx + 1)
    }
    return undefined
  }

  return evalTokens(data, 0)
}

/** 正则提取：返回第一个捕获组，无捕获组则返回完整匹配 */
function regexMatch(text: string, pattern: string, flags?: string): string {
  try {
    const re = new RegExp(pattern, flags ?? '')
    const m = re.exec(text)
    if (!m) return ''
    if (m.length > 1) {
      // 返回第一个非 undefined 捕获组
      for (let i = 1; i < m.length; i++) {
        if (m[i] !== undefined && m[i] !== '') return m[i]
      }
      return m[0] ?? ''
    }
    return m[0] ?? ''
  } catch {
    return ''
  }
}

// ---------------------------------------------------------------- 属性提取

/** 从元素提取属性：a@href、div@text、ul@all、p@html、img@src、*@attr:data-x */
function extractAttr(el: Element, attr: string): string {
  switch (attr.toLowerCase()) {
    case 'text':
      return el.textContent?.trim() ?? ''
    case 'textnodes':
      // 直接文本子节点
      return Array.from(el.childNodes)
        .filter((n) => n.nodeType === Node.TEXT_NODE)
        .map((n) => n.textContent ?? '')
        .join('')
        .trim()
    case 'all':
      return el.textContent?.trim() ?? ''
    case 'html':
      return el.innerHTML ?? ''
    case 'href':
      return el.getAttribute('href') ?? ''
    case 'src':
      return el.getAttribute('src') ?? ''
    case 'alt':
      return el.getAttribute('alt') ?? ''
    case 'title':
      return el.getAttribute('title') ?? ''
    case 'id':
      return el.getAttribute('id') ?? ''
    default:
      if (attr.startsWith('attr:')) {
        return el.getAttribute(attr.slice(5)) ?? ''
      }
      return el.getAttribute(attr) ?? ''
  }
}

// ---------------------------------------------------------------- JS 沙箱

/** 解析书源 header 字段为请求头 */
export function parseHeader(header: string | undefined): Record<string, string> {
  if (!header) return {}
  const out: Record<string, string> = {}
  for (const line of header.split('\n')) {
    const idx = line.indexOf(':')
    if (idx > 0) {
      out[line.slice(0, idx).trim()] = line.slice(idx + 1).trim()
    }
  }
  return out
}

/** 从 html 字符串中提取 css 结果（供 JS 环境 java.css 使用） */
function cssFromHtml(html: string, selector: string, attr?: string): string {
  try {
    const doc = parseDoc(html)
    const nodes = Array.from(doc.querySelectorAll(selector))
    if (nodes.length === 0) return ''
    if (attr) {
      return nodes.map((n) => extractAttr(n, attr)).join('\n')
    }
    return nodes.map((n) => n.textContent?.trim() ?? '').join('\n')
  } catch {
    return ''
  }
}

function regexFromText(text: string, pattern: string, flags?: string): string {
  return regexMatch(text, pattern, flags)
}

function jsonFromStr(jsonText: string, path: string): string {
  try {
    const data = JSON.parse(jsonText)
    const v = jsonPath(data, path)
    if (v === null || v === undefined) return ''
    if (Array.isArray(v)) return v.map((x) => (typeof x === 'object' ? JSON.stringify(x) : String(x))).join('\n')
    if (typeof v === 'object') return JSON.stringify(v)
    return String(v)
  } catch {
    return ''
  }
}

function xpathFromHtml(html: string, expr: string): string {
  try {
    const doc = parseDoc(html)
    const snapshot = document.evaluate(expr, doc, null, XPathResult.ORDERED_NODE_SNAPSHOT_TYPE, null)
    const out: string[] = []
    for (let i = 0; i < snapshot.snapshotLength; i++) {
      const item = snapshot.snapshotItem(i)
      if (item instanceof Attr) out.push(item.value)
      else if (item instanceof Node) out.push((item as Element).textContent?.trim() ?? '')
    }
    return out.join('\n')
  } catch {
    return ''
  }
}

/** 把 Legado 风格的裸选择器参数（$.css(.n)）转换为合法 JS 字符串 */
function normalizeDollarCalls(code: string): string {
  return code.replace(
    /(\$\.(?:css|xpath|regex|json))\s*\(\s*([^()]*?)\s*\)/g,
    (_m, fn: string, argsRaw: string) => {
      const args = argsRaw.split(',').map((a: string) => a.trim())
      const out = args.map((a: string) =>
        /^[.#][A-Za-z_][\w-]*$/.test(a) ? JSON.stringify(a) : a,
      )
      return `${fn}(${out.join(', ')})`
    },
  )
}

/** 执行 JS 规则代码（new Function 沙箱，注入 Legado 常用对象） */
export function runJs(code: string, scope: RuleScope): string {
  const src = scope.source
  const vars = scope.vars
  code = normalizeDollarCalls(code)

  const java: Record<string, unknown> = {
    baseUrl: scope.baseUrl,
    getDomain: (url: string) => {
      try { return new URL(url).hostname } catch { return '' }
    },
    trim: (s: unknown) => String(s ?? '').trim(),
    join: (arr: unknown[], sep: string) => Array.isArray(arr) ? arr.join(sep ?? '') : '',
    urlEncode: (s: unknown) => encodeURIComponent(String(s ?? '')),
    urlDecode: (s: unknown) => decodeURIComponent(String(s ?? '')),
    nonNull: (s: unknown, def: unknown) => (s === null || s === undefined || s === '' ? def : s),
    css: (html: unknown, selector: string, attr?: string) => cssFromHtml(String(html ?? ''), selector, attr),
    xpath: (html: unknown, expr: string) => xpathFromHtml(String(html ?? ''), expr),
    regex: (text: unknown, pattern: string, flags?: string) => regexFromText(String(text ?? ''), pattern, flags),
    json: (jsonText: unknown, path: string) => jsonFromStr(String(jsonText ?? ''), path),
    get: () => { throw new Error('java.get 同步请求在 M1 暂不支持（需异步化改造）') },
    ajax: () => { throw new Error('java.ajax 同步请求在 M1 暂不支持') },
    base64Encode: (s: unknown) => btoa(unescape(encodeURIComponent(String(s ?? '')))),
    base64Decode: (s: unknown) => decodeURIComponent(escape(atob(String(s ?? '')))),
  }

  // $ 对象：Legado 规则 JS 中常见的选择器风格（$.css / $.xpath / $.regex / $.json）
  // 与 java.* 不同，$ 的方法默认作用于当前页面上下文（scope.html / scope.json）
  const dollar: Record<string, unknown> = {
    css: (selector: string, attr?: string) => evalCss(scope, attr ? `${selector}@${attr}` : selector),
    xpath: (expr: string) => xpathSelect(scope, expr).filter((v) => v).join('\n'),
    regex: (pattern: string, flags?: string) =>
      regexMatch(scope.elements[0]?.textContent ?? scope.html, pattern, flags),
    json: (path: string) => evalJson(scope, path),
    getBaseUrl: (url: string) => {
      try { return new URL(url).origin } catch { return '' }
    },
    trim: java.trim,
    nonNull: java.nonNull,
    urlEncode: java.urlEncode,
    urlDecode: java.urlDecode,
    base64Encode: java.base64Encode,
    base64Decode: java.base64Decode,
  }

  const js: Record<string, unknown> = {
    getBaseUrl: (url: string) => {
      try { return new URL(url).origin } catch { return '' }
    },
    escape: (s: unknown) => encodeURIComponent(String(s ?? '')),
    unescape: (s: unknown) => decodeURIComponent(String(s ?? '')),
    urlEncode: (s: unknown) => encodeURIComponent(String(s ?? '')),
    urlDecode: (s: unknown) => decodeURIComponent(String(s ?? '')),
    base64Encode: java.base64Encode,
    base64Decode: java.base64Decode,
    nonNull: java.nonNull,
    ajax: () => { throw new Error('js.ajax 同步请求在 M1 暂不支持') },
    get: () => { throw new Error('js.get 同步请求在 M1 暂不支持') },
  }

  // put/get 全局变量
  const put = (k: string, v: unknown) => { globalVars[k] = String(v ?? '') }
  const get = (k: string) => globalVars[k] ?? ''

  try {
    // eslint-disable-next-line no-new-func
    const fn = new Function(
      'java', 'js', '$', 'source', 'result', 'baseUrl', 'key', 'page',
      'cookie', 'webView', 'window', 'put', 'get',
      `"use strict";\n${scope.jsLibCode ? scope.jsLibCode + '\n' : ''}return (${code});`,
    )
    const result = fn(
      java, js, dollar, src, scope.elements[0] ?? null, scope.baseUrl,
      vars.key ?? '', vars.page ?? '', '', {}, put, get,
    )
    if (result === null || result === undefined) return ''
    if (Array.isArray(result)) {
      return result.map((x) => (typeof x === 'object' ? JSON.stringify(x) : String(x))).join(',')
    }
    if (typeof result === 'object') return JSON.stringify(result)
    return String(result)
  } catch (e) {
    return ''
  }
}

// ---------------------------------------------------------------- 规则求值

const ATTR_RE = /^(.+?)\s*@([A-Za-z][\w:-]*)$/

/** 求值 CSS 规则字符串（可能带 @属性） */
function evalCss(scope: RuleScope, rule: string): string {
  const parts = rule.split('||')
  for (let part of parts) {
    part = part.trim()
    if (!part) continue
    let selector = part
    let attr: string | undefined
    // 无选择器形式：@text / @href 等，直接作用于当前元素
    if (part.startsWith('@')) {
      const values = scope.elements.map((n) => extractAttr(n, part.slice(1))).filter((v) => v !== '')
      if (values.length > 0) return values.join('\n')
      continue
    }
    const m = part.match(ATTR_RE)
    if (m) {
      selector = m[1].trim()
      attr = m[2]
    }
    const nodes = cssSelect(scope, selector)
    if (nodes.length === 0) continue
    if (attr) {
      const values = nodes.map((n) => extractAttr(n, attr!)).filter((v) => v !== '')
      if (values.length > 0) return values.join('\n')
    } else {
      const values = nodes.map((n) => n.textContent?.trim() ?? '').filter((v) => v !== '')
      if (values.length > 0) return values.join('\n')
    }
  }
  return ''
}

/** 求值 JSON 规则字符串 */
function evalJson(scope: RuleScope, rule: string): string {
  const parts = rule.split('||')
  for (const part of parts) {
    if (!part.trim()) continue
    const v = jsonPath(scope.json, part.trim())
    if (v === null || v === undefined) continue
    if (Array.isArray(v)) {
      const joined = v.map((x) => (typeof x === 'object' ? JSON.stringify(x) : String(x))).join('\n')
      if (joined) return joined
      continue
    }
    if (typeof v === 'object') return JSON.stringify(v)
    if (String(v).trim()) return String(v)
  }
  return ''
}

/** 解析 {{...}} 模板内部表达式 */
function evalTemplateInner(inner: string, scope: RuleScope): string {
  const expr = inner.trim()
  if (!expr) return ''

  // 纯变量
  if (expr in scope.vars) return scope.vars[expr]

  // $.xxx(...) 形式
  const dollar = expr.match(/^\$\.(css|xpath|regex|json|js|java)\((.*)\)$/s)
  if (dollar) {
    const kind = dollar[1]
    const args = dollar[2]
    switch (kind) {
      case 'css': {
        const cssRule = parseQuotedArgs(args)[0] ?? args
        return evalCss(scope, cssRule)
      }
      case 'xpath': {
        const xp = parseQuotedArgs(args)[0] ?? args
        const nodes = xpathSelect(scope, xp)
        return nodes.filter((v) => v).join('\n')
      }
      case 'regex': {
        const [pat, flags] = parseQuotedArgs(args)
        const text = scope.elements[0]?.textContent ?? scope.html
        return regexMatch(text, pat ?? '', flags)
      }
      case 'json': {
        const p = parseQuotedArgs(args)[0] ?? args
        return evalJson(scope, p)
      }
      case 'js':
      case 'java':
        return runJs(args, scope)
    }
    return ''
  }

  // js.xxx / java.xxx 前缀（{{js.代码}}）
  const prefixed = expr.match(/^(js|java)\.([\s\S]+)$/)
  if (prefixed) {
    return runJs(prefixed[2], scope)
  }

  // 未知：原样返回
  return ''
}

/** 解析引号包裹的参数列表（支持双引号/单引号/无引号） */
function parseQuotedArgs(args: string): string[] {
  const out: string[] = []
  const re = /"([^"\\]*(?:\\.[^"\\]*)*)"|'([^'\\]*(?:\\.[^'\\]*)*)'|([^,]+)/g
  let m: RegExpExecArray | null
  while ((m = re.exec(args))) {
    let val = m[1] ?? m[2] ?? m[3] ?? ''
    val = val.trim()
    if (m[1] !== undefined || m[2] !== undefined) {
      // 解转义
      val = val.replace(/\\(["'\\])/g, '$1')
    }
    out.push(val)
  }
  return out
}

/**
 * 求值一条规则字符串。
 *
 * 规则类型：
 * - @js: / @css: / @xpath: / @regex: / @json: / @get: 前缀
 * - {{...}} 模板（可含 || 拼接）
 * - 普通字符串视为常量
 */
export function evalRule(rule: string | undefined, scope: RuleScope): string {
  if (!rule) return ''
  const r = rule.trim()
  if (!r) return ''

  // 前缀规则
  const prefix = r.match(/^@(js|css|xpath|regex|json|get):([\s\S]*)$/)
  if (prefix) {
    const kind = prefix[1]
    const body = prefix[2].trim()
    switch (kind) {
      case 'js':
        return runJs(body, scope)
      case 'css':
        return evalCss(scope, body)
      case 'xpath': {
        const nodes = xpathSelect(scope, body)
        return nodes.filter((v) => v).join('\n')
      }
      case 'regex':
        return regexMatch(scope.elements[0]?.textContent ?? scope.html, body)
      case 'json':
        return evalJson(scope, body)
      case 'get':
        return body // @get: 返回 URL 字符串（不发起请求）
    }
    return ''
  }

  // 模板规则：按 || 在花括号外分割，逐个求值取第一个非空
  const segments = splitOutsideBraces(r)
  for (let seg of segments) {
    seg = seg.trim()
    if (!seg) continue
    if (seg.startsWith('{{') && seg.endsWith('}}')) {
      const inner = seg.slice(2, -2)
      const v = evalTemplateInner(inner, scope)
      if (v && v.trim()) return v
    } else if (seg.includes('{{')) {
      // 混合模板：变量+规则混合（如 前缀{{key}}后缀）
      const out = seg.replace(/\{\{([\s\S]*?)\}\}/g, (_, inner) => evalTemplateInner(inner, scope))
      if (out.trim()) return out
    } else {
      // 常量
      if (seg) return seg
    }
  }
  return ''
}

/** 按 || 分割，但跳过 {{...}} 内部 */
function splitOutsideBraces(rule: string): string[] {
  const out: string[] = []
  let depth = 0
  let cur = ''
  for (let i = 0; i < rule.length; i++) {
    const ch = rule[i]
    if (ch === '{' && rule[i + 1] === '{') { depth++; cur += ch; i++; cur += rule[i]; continue }
    if (ch === '}' && rule[i + 1] === '}' && depth > 0) { depth--; cur += ch; i++; cur += rule[i]; continue }
    if (ch === '|' && rule[i + 1] === '|' && depth === 0) {
      out.push(cur)
      cur = ''
      i++
      continue
    }
    cur += ch
  }
  out.push(cur)
  return out
}

/** 从规则中提取元素列表（顶层列表规则，如 bookList/chapterList） */
export function evalElementList(rule: string | undefined, scope: RuleScope): Element[] {
  if (!rule) return []
  const r = rule.trim()
  if (!r) return []

  const prefix = r.match(/^@(js):([\s\S]*)$/)
  if (prefix) {
    // @js: 顶层列表：执行后按行解析为元素列表
    const out = runJs(prefix[2], scope)
    const lines = out.split('\n').map((l) => l.trim()).filter((l) => l.startsWith('<'))
    if (lines.length > 0) {
      return lines.map((l) => parseDoc(l).body.firstElementChild as Element).filter(Boolean)
    }
    return []
  }

  // 模板形式 {{$.css("...")}} / {{$.xpath("...")}} / {{$.json("...")}} / {{js.代码}}
  if (r.startsWith('{{') && r.endsWith('}}')) {
    const inner = r.slice(2, -2).trim()
    const dollar = inner.match(/^\$\.(css|xpath|json|js|java)\((.*)\)$/s)
    if (dollar) {
      const kind = dollar[1]
      const args = dollar[2]
      switch (kind) {
        case 'css': {
          const cssRule = parseQuotedArgs(args)[0] ?? args
          // css 选择器可能带 @属性（@all 表示返回元素集合）
          const m = cssRule.match(ATTR_RE)
          const selector = m ? m[1].trim() : cssRule.trim()
          return cssSelect(scope, selector)
        }
        case 'xpath': {
          const xp = parseQuotedArgs(args)[0] ?? args
          const out: Element[] = []
          const collect = (node: Node) => {
            try {
              const snapshot = document.evaluate(xp, node, null, XPathResult.ORDERED_NODE_SNAPSHOT_TYPE, null)
              for (let i = 0; i < snapshot.snapshotLength; i++) {
                const item = snapshot.snapshotItem(i)
                if (item instanceof Element) out.push(item)
              }
            } catch { /* 忽略 */ }
          }
          if (scope.elements.length > 0) {
            for (const el of scope.elements) collect(el)
          } else {
            let doc = scope.doc
            if (!doc) { doc = parseDoc(scope.html); scope.doc = doc }
            collect(doc)
          }
          return out
        }
        case 'json': {
          // JSON 列表交给上层（返回空元素，由 JSON 分支处理）
          return []
        }
        case 'js':
        case 'java': {
          // JS 顶层列表：返回数组/HTML 行 → 元素
          const out = runJs(args, scope)
          const lines = out.split('\n').map((l) => l.trim()).filter((l) => l.startsWith('<'))
          return lines.map((l) => parseDoc(l).body.firstElementChild as Element).filter(Boolean)
        }
      }
    }
    const prefixed = inner.match(/^(js|java)\.([\s\S]+)$/)
    if (prefixed) {
      const out = runJs(prefixed[2], scope)
      const lines = out.split('\n').map((l) => l.trim()).filter((l) => l.startsWith('<'))
      return lines.map((l) => parseDoc(l).body.firstElementChild as Element).filter(Boolean)
    }
  }

  // 直接 CSS 选择器
  const m = r.match(ATTR_RE)
  const selector = m ? m[1].trim() : r.trim()
  return cssSelect(scope, selector)
}

/** 创建初始作用域 */
export function makeScope(
  html: string,
  json: any,
  baseUrl: string,
  source: BookSource,
  vars: Record<string, string>,
  jsLibCode?: string,
): RuleScope {
  return {
    html,
    doc: null,
    json,
    elements: [],
    baseUrl,
    vars,
    source,
    jsLibCode,
  }
}
