"""Fetch original 1.21.5 texture pixels and extract the visible model UV faces."""
from concurrent.futures import ThreadPoolExecutor
import hashlib
import json
from pathlib import Path
import urllib.request
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
DEST = ROOT / 'tools/reader-templates/minecraft/assets'
OUT = ROOT / 'output/reader-minecraft-live-artwork'
BASE = 'https://assets.mcasset.cloud/1.21.5/assets/minecraft/textures/'
FILES = {'sun': 'environment/sun.png', 'moon-phases': 'environment/moon_phases.png',
         'skeleton': 'entity/skeleton/skeleton.png', 'lantern': 'block/lantern.png',
         'red-mushroom': 'block/red_mushroom.png', 'poppy': 'block/poppy.png',
         'amethyst': 'block/amethyst_cluster.png', 'short-grass': 'block/short_grass.png'}

def fetch(item):
    name, relative = item
    path = DEST / (name + '.png')
    data = path.read_bytes() if path.exists() else b''
    for attempt in range(3):
        if data.startswith(b'\x89PNG'):
            break
        try:
            request = urllib.request.Request(BASE + relative, headers={'User-Agent': 'Reader-template-artwork/1.0'})
            with urllib.request.urlopen(request, timeout=20) as response:
                data = response.read(2_000_000)
        except Exception:
            if attempt == 2:
                raise
    assert data.startswith(b'\x89PNG'), relative
    path.write_bytes(data)
    return dict(name=name, url=BASE+relative, sha256=hashlib.sha256(data).hexdigest(), bytes=len(data))

OUT.mkdir(parents=True, exist_ok=True)
with ThreadPoolExecutor(max_workers=4) as pool:
    records = list(pool.map(fetch, FILES.items()))

for kind in ('creeper', 'skeleton'):
    skin = Image.open(DEST / (kind + '.png')).convert('RGBA')
    faces = {'head-front': (8, 8, 16, 16), 'head-top': (8, 0, 16, 8), 'head-side': (0, 8, 8, 16),
             'torso': (20, 20, 28, 32), 'leg': (4, 20, 8 if kind == 'creeper' else 6, 26 if kind == 'creeper' else 32)}
    if kind == 'skeleton':
        faces['arm'] = (44, 20, 46, 32)
    for name, box in faces.items():
        skin.crop(box).save(DEST / (kind + '-' + name + '.png'))

# Sky textures use black as the blend key in the game. Preserve their pixel
# colors and convert that key to alpha for compositing over the reader's sky.
for name, box in [('sun', None), ('moon', (8, 8, 24, 24))]:
    source = Image.open(DEST / ('moon-phases.png' if name == 'moon' else 'sun.png')).convert('RGBA')
    if box:
        source = source.crop(box)
    pixels = []
    for r, g, b, a in source.get_flattened_data():
        light = max(r, g, b)
        pixels.append((round(r * 255 / light), round(g * 255 / light), round(b * 255 / light), round(a * light / 255))
                      if light else (0, 0, 0, 0))
    source.putdata(pixels)
    source.save(DEST / (name + '-disc.png'))
(OUT / 'sources.json').write_text(json.dumps(records, indent=2) + '\n', encoding='utf-8')
print(json.dumps(records))
