// 替换净化规则应用（正文/目录）

import type { ReplaceRule } from './store'

/** 对文本应用替换规则（仅 content/global 范围） */
export function applyReplaceRules(content: string, rules: ReplaceRule[]): string {
  let out = content
  for (const r of rules) {
    if (!r.enabled) continue
    if (r.scope !== 'content' && r.scope !== 'global') continue
    if (!r.replaceRegex) continue
    try {
      if (r.isRegex) {
        out = out.replace(new RegExp(r.replaceRegex, 'g'), r.replacement ?? '')
      } else {
        out = out.split(r.replaceRegex).join(r.replacement ?? '')
      }
    } catch {
      // 无效正则跳过
    }
  }
  return out
}

/** 对目录文本应用替换规则（toc/global 范围） */
export function applyReplaceToToc(text: string, rules: ReplaceRule[]): string {
  let out = text
  for (const r of rules) {
    if (!r.enabled) continue
    if (r.scope !== 'toc' && r.scope !== 'global') continue
    if (!r.replaceRegex) continue
    try {
      if (r.isRegex) {
        out = out.replace(new RegExp(r.replaceRegex, 'g'), r.replacement ?? '')
      } else {
        out = out.split(r.replaceRegex).join(r.replacement ?? '')
      }
    } catch {
      // 忽略
    }
  }
  return out
}
