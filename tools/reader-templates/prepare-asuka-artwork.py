"""Optimize the original attributed frames without altering the characters."""
from pathlib import Path
import hashlib
import json
from PIL import Image, ImageOps

ASSETS = Path(__file__).resolve().parent / 'asuka/assets'
records = []
for source, name, size in [('asuka-opening.jpg', 'portrait.webp', (1000, 750)),
                           ('asuka-unit02.png', 'arrival.webp', (896, 504))]:
    image = ImageOps.exif_transpose(Image.open(ASSETS / source)).convert('RGB')
    image.thumbnail(size, Image.Resampling.LANCZOS)
    target = ASSETS / name
    image.save(target, 'WEBP', quality=91, method=6)
    records.append(dict(source=source, sourceSha256=hashlib.sha256((ASSETS / source).read_bytes()).hexdigest(),
        file=name, width=image.width, height=image.height, bytes=target.stat().st_size,
        sha256=hashlib.sha256(target.read_bytes()).hexdigest()))
(ASSETS.parent / 'artwork.json').write_text(json.dumps(records, indent=2) + '\n', encoding='utf-8')
print(json.dumps(records, indent=2))
