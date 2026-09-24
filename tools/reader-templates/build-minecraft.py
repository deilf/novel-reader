"""Package the offline Minecraft reader theme from reviewable source files."""
import base64
import json
from pathlib import Path
import re

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
SOURCE = HERE / 'minecraft'


def asset(name):
    suffix = '.woff2' if name == 'reader-pixel' else '.png'
    mime = 'font/woff2' if suffix == '.woff2' else 'image/png'
    return 'data:' + mime + ';base64,' + base64.b64encode((SOURCE / 'assets' / (name + suffix)).read_bytes()).decode('ascii')


def landscape():
    """Isometric geometry with the actual block textures, preserved as pixels."""
    names = ['grass-top-tinted', 'grass-side', 'dirt', 'stone', 'oak-log',
             'oak-leaves-tinted', 'water-frame', 'creeper-face', 'creeper-body', 'creeper-foot']
    svg = ['<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 -27 252 178" fill="none" aria-hidden="true"><defs>']
    for name in names:
        svg.append('<image id="mc-tex-' + name + '" href="' + asset(name) + '" width="16" height="16" preserveAspectRatio="none"/>')
    svg.append('</defs>')
    blocks = []
    heights = {}
    for x in range(7):
        for z in range(5):
            if (x, z) in {(0, 0), (6, 0), (6, 4), (0, 4)}:
                continue
            heights[x, z] = 1 + int(x < 2 and z < 3)
    for (x, z), height in heights.items():
        for y in range(height + 1):
            water = y == height and (x, z) in {(3, 1), (4, 1), (4, 2), (5, 2)}
            top = 'water-frame' if water else 'grass-top-tinted' if y == height else 'stone'
            side = 'stone' if water else 'grass-side' if y == height else 'dirt'
            blocks.append((x, z, y, top, side))
    for x, z in [(1, 1), (5, 0)]:
        ground = heights.get((x, z), 1)
        for y in range(ground + 1, ground + 4):
            blocks.append((x, z, y, 'oak-log', 'oak-log'))
        for dx, dz, dy in [(0, 0, 5), (-1, 0, 4), (0, -1, 4), (0, 0, 4), (1, 0, 4), (0, 1, 4), (-1, 1, 3), (0, 1, 3), (1, 1, 3)]:
            blocks.append((x + dx, z + dz, ground + dy, 'oak-leaves-tinted', 'oak-leaves-tinted'))
    for x, z, y, top, side in sorted(blocks, key=lambda b: (b[0] + b[1], b[2], b[0])):
        px, py = 113 + (x - z) * 16, 70 + (x + z) * 8 - y * 15
        svg.append(f'<use href="#mc-tex-{side}" transform="matrix(1 .5 0 .9375 {px-16} {py+8})"/>')
        svg.append(f'<path d="M{px-16} {py+8}l16 8v15l-16-8z" fill="#192b20" opacity=".17"/>')
        svg.append(f'<use href="#mc-tex-{side}" transform="matrix(1 -.5 0 .9375 {px} {py+16})"/>')
        svg.append(f'<path d="M{px} {py+16}l16-8v15l-16 8z" fill="#162923" opacity=".38"/>')
        svg.append(f'<use href="#mc-tex-{top}" transform="matrix(1 .5 -1 .5 {px} {py})"/>')
        if top == 'water-frame':
            svg.append(f'<path d="M{px} {py}l16 8-16 8-16-8z" fill="#248bc5" opacity=".75"/>')
    svg.append('</svg>')
    return ''.join(svg)


def mob(kind):
    bow = ('<span class="mc-bow"><i class="mc-bow-rest"></i><i class="mc-bow-draw-0"></i>'
           '<i class="mc-bow-draw-1"></i><i class="mc-bow-draw-2"></i></span>') if kind == 'skeleton' else ''
    return ('<div class="mc-mob mc-mob-' + kind + '"><i class="mc-mob-shadow"></i>'
            '<div class="mc-mob-rig"><i class="mc-mob-leg mc-limb-back"></i>'
            '<i class="mc-mob-arm mc-limb-back"></i><i class="mc-mob-torso"></i>'
            '<i class="mc-mob-leg mc-limb-front"></i><i class="mc-mob-arm mc-limb-front"></i>'
            '<i class="mc-mob-head"></i>' + bow + '</div></div>')


def expand(text):
    return re.sub(r'\{\{asset:([a-z0-9-]+)\}\}', lambda match: asset(match[1]), text)


def main():
    footer = (SOURCE / 'footer.html').read_text(encoding='utf-8')
    # Keep attribution and the full font license with exported template files,
    # including copies made without this source checkout or the application.
    notice = ('/*\nMinecraft textures © Mojang AB / Microsoft. Non-official reader theme.\n'
              'Textures: https://assets.mcasset.cloud/1.21.5/assets/minecraft/textures/\n'
              'Reader Pixel is an ASCII subset of Press Start 2P, renamed under the OFL.\n\n'
              + (SOURCE / 'PressStart2P-OFL.txt').read_text(encoding='utf-8').replace('*/', '* /')
              + '\n*/\n')
    pages = {}
    for name in ('first', 'other'):
        pages[name] = expand((SOURCE / (name + '.html')).read_text(encoding='utf-8')
                             .replace('{{footer}}', footer).replace('{{landscape}}', landscape())
                             .replace('{{creeper}}', mob('creeper')).replace('{{skeleton}}', mob('skeleton')))
    template = dict(schemaVersion=1, id='builtin.minecraft_live', name='Minecraft · 天光漫游',
        description='随本地时间变化的主世界天空与日月轨迹，原地踏步的苦力怕与定点射箭的矿洞小白，随机弧线箭矢和翻页联动的九格物品栏、手持物品。首篇浮岛、续页矿洞，原版贴图与像素字体全部离线内置。JS 动效随阅读状态暂停，正文保持稳定。非官方 Minecraft 主题。',
        firstPageHtml=pages['first'], otherPageHtml=pages['other'],
        css=notice + expand((SOURCE / 'style.css').read_text(encoding='utf-8') + '\n' + (SOURCE / 'live.css').read_text(encoding='utf-8')),
        javascript=(SOURCE / 'script.js').read_text(encoding='utf-8'))
    assert not re.search(r'\{\{(?:asset:|footer|landscape)', json.dumps(template))
    destination = ROOT / 'app/src/main/assets/epub/templates/builtin.minecraft_live.json'
    destination.write_text(json.dumps(template, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    print(json.dumps({'template': str(destination), 'bytes': destination.stat().st_size}))


if __name__ == '__main__':
    main()
