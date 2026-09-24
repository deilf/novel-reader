"""Build the complete, portable Asuka reader template from its reviewed sources."""
import base64
import json
from pathlib import Path
import re

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
SOURCE = HERE / 'asuka'
ORBIT = '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100" fill="none" aria-hidden="true"><path d="M30 3h40l27 27v40L70 97H30L3 70V30Z" stroke="currentColor" stroke-width="1"/><g class="as-orbit-line"><path d="M35 15h30l20 20v30L65 85H35L15 65V35Z" stroke="currentColor" stroke-width="2"/><path d="M39 27h22l12 12v22L61 73H39L27 61V39Z" stroke="currentColor" stroke-width="1"/></g><path d="M50 0v18m0 64v18M0 50h18m64 0h18" stroke="currentColor" stroke-width="2"/><circle cx="50" cy="50" r="3" fill="currentColor"/></svg>'
EMBLEM = '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 36 28" fill="none" aria-hidden="true"><path d="m2 25 9-22h6l9 22h-7l-2-6h-7l-2 6Z" stroke="currentColor" stroke-width="1.5"/><path d="M24 3h10v7H24zm0 11h10v11H24z" fill="currentColor" opacity=".65"/></svg>'


def expand(text):
    def asset(match):
        name = match[1]
        path = SOURCE / 'assets' / name
        mime = 'font/ttf' if path.suffix == '.ttf' else 'image/webp'
        return 'data:' + mime + ';base64,' + base64.b64encode(path.read_bytes()).decode('ascii')
    return re.sub(r'\{\{asset:([a-zA-Z0-9_.-]+)\}\}', asset, text)


def main():
    footer = (SOURCE / 'footer.html').read_text(encoding='utf-8')
    pages = {name: expand((SOURCE / (name + '.html')).read_text(encoding='utf-8')
        .replace('{{footer}}', footer).replace('{{orbit}}', ORBIT).replace('{{emblem}}', EMBLEM))
        for name in ('first', 'other')}
    notice = ('/* Non-official Evangelion reader theme. Character frames © GAINAX / khara and respective rights holders.\n'
        'Sources: https://wiki.evageeks.org/File:OP_C057_asuka.jpg and https://wiki.evageeks.org/File:2.22_Asuka-Nigouki.png\n'
        'Font: Barlow Condensed Bold, Jeremy Tribby, https://github.com/google/fonts/tree/main/ofl/barlowcondensed\n'
        + (SOURCE / 'assets/BarlowCondensed-OFL.txt').read_text(encoding='utf-8').replace('*/', '* /') + '\n*/\n')
    template = dict(schemaVersion=1, id='builtin.asuka_sync', name='明日香 · 赤色同步',
        description='朱红色的明日香海报首页与炭黑驾驶舱续页。原作人物画面、AT 力场仪表、动态扫描光、波形与阅读同步率，搭配暖白书页和细致段落装饰。图片与标题字体全部离线内置。非官方 EVA 主题。',
        firstPageHtml=pages['first'], otherPageHtml=pages['other'],
        css=notice + expand((SOURCE / 'style.css').read_text(encoding='utf-8')),
        javascript=(SOURCE / 'script.js').read_text(encoding='utf-8'))
    assert '{{' not in json.dumps(template, ensure_ascii=False)
    destination = ROOT / 'app/src/main/assets/epub/templates/builtin.asuka_sync.json'
    destination.write_text(json.dumps(template, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    print(json.dumps(dict(template=str(destination), bytes=destination.stat().st_size)))


if __name__ == '__main__':
    main()
