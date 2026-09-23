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
