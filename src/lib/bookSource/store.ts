// 书源存储命令封装（Rust 端 book_sources.json）

import { invoke } from '@tauri-apps/api/core'
import type { BookSource } from './types'

export async function importBookSources(sources: BookSource[]): Promise<number> {
  return invoke<number>('import_book_sources', { sources })
}

export async function listBookSources(): Promise<BookSource[]> {
  return invoke<BookSource[]>('list_book_sources')
}

export async function updateBookSource(source: BookSource): Promise<void> {
  return invoke<void>('update_book_source', { source })
}

export async function deleteBookSource(bookSourceUrl: string): Promise<void> {
  return invoke<void>('delete_book_source', { bookSourceUrl })
}

export async function toggleBookSource(bookSourceUrl: string): Promise<boolean> {
  return invoke<boolean>('toggle_book_source', { bookSourceUrl })
}

export async function clearBookSources(): Promise<void> {
  return invoke<void>('clear_book_sources')
}

// ==================== 书签 ====================

export interface Bookmark {
  novel_url: string
  novel_title: string
  chapter_url: string
  chapter_title: string
  content?: string
  note?: string
  created_at: number
}

export async function addBookmark(bookmark: Bookmark): Promise<boolean> {
  return invoke<boolean>('add_bookmark', { bookmark })
}

export async function listBookmarks(novelUrl: string): Promise<Bookmark[]> {
  return invoke<Bookmark[]>('list_bookmarks', { novelUrl })
}

export async function removeBookmark(novelUrl: string, chapterUrl: string): Promise<void> {
  return invoke<void>('remove_bookmark', { novelUrl, chapterUrl })
}

// ==================== 替换净化规则 ====================

export interface ReplaceRule {
  name: string
  enabled: boolean
  isRegex: boolean
  replaceRegex: string
  replacement: string
  scope: string
}

export async function importReplaceRules(rules: ReplaceRule[]): Promise<number> {
  return invoke<number>('import_replace_rules', { rules })
}

export async function listReplaceRules(): Promise<ReplaceRule[]> {
  return invoke<ReplaceRule[]>('list_replace_rules')
}

export async function updateReplaceRule(rule: ReplaceRule): Promise<void> {
  return invoke<void>('update_replace_rule', { rule })
}

export async function deleteReplaceRule(name: string): Promise<void> {
  return invoke<void>('delete_replace_rule', { name })
}

export async function toggleReplaceRule(name: string): Promise<boolean> {
  return invoke<boolean>('toggle_replace_rule', { name })
}

export async function clearReplaceRules(): Promise<void> {
  return invoke<void>('clear_replace_rules')
}

// ==================== 本地书籍 ====================

export interface LocalBook {
  id: string
  title: string
  author?: string
  format: string
  path: string
  chapter_count: number
  added_at: number
  cover?: string
}

export interface LocalChapter {
  title: string
  index: number
  offset: number
}

export async function importLocalBook(path: string): Promise<LocalBook> {
  return invoke<LocalBook>('import_local_book', { path })
}

export async function listLocalBooks(): Promise<LocalBook[]> {
  return invoke<LocalBook[]>('list_local_books')
}

export async function removeLocalBook(id: string): Promise<void> {
  return invoke<void>('remove_local_book', { id })
}

export async function getLocalChapters(bookId: string): Promise<LocalChapter[]> {
  return invoke<LocalChapter[]>('get_local_chapters', { bookId })
}

export async function getLocalChapterContent(bookId: string, index: number): Promise<string> {
  return invoke<string>('get_local_chapter_content', { bookId, index })
}
