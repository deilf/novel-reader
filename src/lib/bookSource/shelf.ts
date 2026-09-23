// 书架 / 进度命令封装（复用 Rust storage 命令）

import { invoke } from '@tauri-apps/api/core'

export interface ReadingProgress {
  novel_url: string
  novel_title: string
  chapter_url: string
  chapter_title: string
  chapter_index: number
  scroll_position: number
  last_read_time: string
}

export interface BookshelfNovel {
  title: string
  author?: string
  cover_url?: string
  url: string
  source: string
  added_time?: string
  latest_chapter?: string
  progress?: ReadingProgress
}

export async function getBookshelf(): Promise<BookshelfNovel[]> {
  return invoke<BookshelfNovel[]>('get_bookshelf')
}

export async function saveBookshelf(novels: BookshelfNovel[]): Promise<void> {
  return invoke<void>('save_bookshelf', { novels })
}

export async function addToBookshelf(novel: BookshelfNovel): Promise<void> {
  return invoke<void>('add_to_bookshelf', { novel })
}

export async function removeFromBookshelf(url: string): Promise<void> {
  return invoke<void>('remove_from_bookshelf', { novelUrl: url })
}

export async function saveReadingProgress(progress: ReadingProgress): Promise<void> {
  return invoke<void>('save_reading_progress', { progress })
}

export async function getReadingProgress(novelUrl: string): Promise<ReadingProgress | null> {
  return invoke<ReadingProgress | null>('get_reading_progress', { novelUrl })
}

export async function getAllReadingProgress(): Promise<ReadingProgress[]> {
  return invoke<ReadingProgress[]>('get_all_reading_progress')
}
