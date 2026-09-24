"""Prepare the official-site illustrations for the offline, non-official theme.

Requires Pillow. Usage: python prepare-doraemon-artwork.py [Pillow module directory]
"""
from pathlib import Path
from urllib.request import Request, urlopen
from concurrent.futures import ThreadPoolExecutor
from io import BytesIO
import hashlib
import json
import sys

if len(sys.argv) > 1:
    sys.path.insert(0, sys.argv[1])
from PIL import Image

HERE = Path(__file__).resolve().parent / 'doraemon'
ASSETS = HERE / 'assets'
BASE = 'https://dora-world.com/'
SOURCES = {
    'friends.webp': 'assets/images/hd/hd_bg_s01_chara.webp',
    'portrait.webp': 'assets/images/characters/doraemon/doraemon/ico.png',
}

def prepare(entry):
    name, relative = entry
    url = BASE + relative
    raw = urlopen(Request(url, headers={'User-Agent': 'Mozilla/5.0'}), timeout=30).read()
    image = Image.open(BytesIO(raw)).convert('RGBA')
    alpha_max = image.getchannel('A').getextrema()[1]
    processing = 'RGBA conversion and WebP encoding; original colours and composition preserved.'
    if alpha_max and alpha_max < 255:
        image.putalpha(image.getchannel('A').point(lambda a: round(a * 255 / alpha_max)))
        processing += f' Normalized uniform artwork transparency from maximum alpha {alpha_max} to 255.'
    image.thumbnail((910, 460), Image.Resampling.LANCZOS)
    target = ASSETS / name
    image.save(target, 'WEBP', quality=94, method=6)
    return dict(file='assets/' + name, url=url, sourceSha256=hashlib.sha256(raw).hexdigest(),
        sha256=hashlib.sha256(target.read_bytes()).hexdigest(), width=image.width, height=image.height,
        bytes=target.stat().st_size, processing=processing,
        copyright='Fujiko-Pro / Shogakukan and respective rights holders; non-official reader theme.')

def main():
    ASSETS.mkdir(parents=True, exist_ok=True)
    with ThreadPoolExecutor(max_workers=2) as pool:
        records = list(pool.map(prepare, SOURCES.items()))
    (HERE / 'sources.json').write_text(json.dumps(records, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    print(json.dumps(records, ensure_ascii=False, indent=2))

if __name__ == '__main__':
    main()
