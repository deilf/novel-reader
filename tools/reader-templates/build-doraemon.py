"""Build the portable, continuous Doraemon template; no runtime network resources."""
from pathlib import Path
import base64
import json
import re

HERE = Path(__file__).resolve().parent
SOURCE = HERE / 'doraemon'
ROOT = HERE.parents[1]

def expand(source):
    def asset(match):
        path = SOURCE / 'assets' / match[1]
        return 'data:image/webp;base64,' + base64.b64encode(path.read_bytes()).decode('ascii')
    return re.sub(r'\{\{asset:([a-zA-Z0-9_.-]+)\}\}', asset, source)

def main():
    notice = ('/* Non-official Doraemon reader theme. Character illustrations copyright Fujiko-Pro / Shogakukan and respective rights holders.\n'
              'Sources: https://dora-world.com/ and https://dora-world.com/character/doraemon\n'
              'Provenance: tools/reader-templates/doraemon/SOURCES.md and sources.json. */\n')
    template = dict(schemaVersion=2, type='scroll', id='builtin.doraemon_scroll', name='哆啦 A 梦 · 未来口袋',
        description='蓝天、铃铛与四次元口袋组成固定阅读画框，页眉、页脚和两侧装饰保持不动，只有中央矩形正文框连续滚动。官网人物插画离线内置，天色随时间变化；不区分首页与续页，翻页方式固定为滚动。非官方主题。',
        scrollHtml=expand((SOURCE / 'scroll.html').read_text(encoding='utf-8')),
        css=notice + (SOURCE / 'style.css').read_text(encoding='utf-8'),
        javascript=(SOURCE / 'script.js').read_text(encoding='utf-8'))
    assert '{{asset:' not in json.dumps(template)
    output = ROOT / 'app/src/main/assets/epub/templates/builtin.doraemon_scroll.json'
    output.write_text(json.dumps(template, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    print(json.dumps(dict(template=str(output), bytes=output.stat().st_size)))

if __name__ == '__main__':
    main()
