// 简繁转换（opencc-js 懒加载）

// 动态导入避免打包体积
let s2tConverter: any = null
let t2sConverter: any = null

async function getS2T(): Promise<(t: string) => string> {
  if (!s2tConverter) {
    const mod: any = await import('opencc-js')
    s2tConverter = new mod.Converter({ from: 'cn', to: 'tw' })
  }
  return s2tConverter
}

async function getT2S(): Promise<(t: string) => string> {
  if (!t2sConverter) {
    const mod: any = await import('opencc-js')
    t2sConverter = new mod.Converter({ from: 'tw', to: 'cn' })
  }
  return t2sConverter
}

/** 简体 → 繁体 */
export async function toTraditional(text: string): Promise<string> {
  const conv = await getS2T()
  return conv(text)
}

/** 繁体 → 简体 */
export async function toSimplified(text: string): Promise<string> {
  const conv = await getT2S()
  return conv(text)
}
