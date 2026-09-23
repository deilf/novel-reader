// Rust 端 http_fetch 命令的封装（支持依赖注入以便测试）

import { invoke } from '@tauri-apps/api/core'
import type { HttpRequest, HttpResponse } from './types'

export type HttpFetchFn = (request: HttpRequest) => Promise<HttpResponse>

let httpFetchImpl: HttpFetchFn = async (request) => {
  try {
    return await invoke<HttpResponse>('http_fetch', { request })
  } catch (e) {
    throw new Error(String(e))
  }
}

/** 覆盖底层 HTTP 实现（测试用） */
export function setHttpFetchImpl(fn: HttpFetchFn) {
  httpFetchImpl = fn
}

export async function httpFetch(request: HttpRequest): Promise<HttpResponse> {
  return httpFetchImpl(request)
}
