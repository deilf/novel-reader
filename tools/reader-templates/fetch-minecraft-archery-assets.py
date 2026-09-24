"""Fetch the original bow, arrow and target textures for the stationary skeleton."""
from concurrent.futures import ThreadPoolExecutor
import hashlib
import json
from pathlib import Path
import urllib.request

ROOT = Path(__file__).resolve().parents[2]
DEST = ROOT / 'tools/reader-templates/minecraft/assets'
OUT = ROOT / 'output/reader-minecraft-pose-artwork'
BASE = 'https://assets.mcasset.cloud/1.21.5/assets/minecraft/textures/'
FILES = {'bow': 'item/bow.png', 'bow-pulling-0': 'item/bow_pulling_0.png',
         'bow-pulling-1': 'item/bow_pulling_1.png', 'bow-pulling-2': 'item/bow_pulling_2.png',
         'arrow': 'item/arrow.png', 'target': 'block/target_side.png'}


def fetch(item):
    name, relative = item
    path = DEST / (name + '.png')
    data = path.read_bytes() if path.exists() else b''
    for attempt in range(3):
        if data.startswith(b'\x89PNG\r\n\x1a\n'):
            break
        try:
            request = urllib.request.Request(BASE + relative, headers={'User-Agent': 'Reader-template-artwork/1.0'})
            with urllib.request.urlopen(request, timeout=20) as response:
                data = response.read(200_000)
        except Exception:
            if attempt == 2:
                raise
    assert data.startswith(b'\x89PNG\r\n\x1a\n'), relative
    path.write_bytes(data)
    return dict(name=name, url=BASE + relative, sha256=hashlib.sha256(data).hexdigest(), bytes=len(data))


if __name__ == '__main__':
    OUT.mkdir(parents=True, exist_ok=True)
    with ThreadPoolExecutor(max_workers=4) as pool:
        records = list(pool.map(fetch, FILES.items()))
    (OUT / 'sources.json').write_text(json.dumps(records, indent=2) + '\n', encoding='utf-8')
    print(json.dumps(records))
