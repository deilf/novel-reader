"""Fetch attributed, pinned anime reference frames and the OFL display font."""
import hashlib
import json
from pathlib import Path
import urllib.request

HERE = Path(__file__).resolve().parent
ASSETS = HERE / 'asuka' / 'assets'
SOURCES = (
    ('asuka-opening.jpg', 'https://wiki.evageeks.org/images/e/ef/OP_C057_asuka.jpg',
     '0795294406910e0be28e0befe695227c68ffe15ba6bbf4daa4af3f191450694e'),
    ('asuka-unit02.png', 'https://wiki.evageeks.org/images/0/0a/2.22_Asuka-Nigouki.png',
     '99eb9a1fc9f12127836e6f19b60dfc78aa533bec95a284ad52805fc7b9de66de'),
    ('BarlowCondensed-Bold.ttf', 'https://raw.githubusercontent.com/google/fonts/main/ofl/barlowcondensed/BarlowCondensed-Bold.ttf',
     'e476562ec9c1e16cf16475895b511f08c804f438cc9a9f80a44ea50a0eeb5b65'),
    ('BarlowCondensed-OFL.txt', 'https://raw.githubusercontent.com/google/fonts/main/ofl/barlowcondensed/OFL.txt',
     '186d750eb496a4c17a76385f82be6aea2ac1cf2de074a811d63786cf374ea73f'),
)


def main():
    ASSETS.mkdir(parents=True, exist_ok=True)
    records = []
    for name, url, expected in SOURCES:
        request = urllib.request.Request(url, headers={'User-Agent': 'Legado-private-reader-theme/1.0'})
        with urllib.request.urlopen(request, timeout=45) as response:
            data = response.read(12 * 1024 * 1024 + 1)
        assert len(data) <= 12 * 1024 * 1024, name
        digest = hashlib.sha256(data).hexdigest()
        assert expected is None or digest == expected, name
        destination = ASSETS / name
        if destination.exists():
            assert destination.read_bytes() == data, 'Preserve reviewed assets: ' + name
        else:
            destination.write_bytes(data)
        records.append(dict(file=name, url=url, sha256=digest, bytes=len(data)))
    manifest = ASSETS.parent / 'sources.json'
    data = json.dumps(records, ensure_ascii=False, indent=2) + '\n'
    if manifest.exists():
        assert manifest.read_text(encoding='utf-8') == data
    else:
        manifest.write_text(data, encoding='utf-8')
    print(json.dumps(records, indent=2))


if __name__ == '__main__':
    main()
