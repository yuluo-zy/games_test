"""Bounded B original/analysis byte comparison; no database mutation."""
import hashlib, json, struct
from pathlib import Path

ROOT = Path('D:/game')
OUT = Path(__file__).parent
TARGETS = [(0x140b383cc, 0x65), (0x142268dc0, 0x33e),
           (0x140840610, 0xf30), (0x140841b00, 0x1fc)]
paths = [ROOT / 'ljxsj_92385/Tiny Glade/tiny-glade.rne',
         ROOT / 'reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe']

def read_ranges(path):
    data = path.read_bytes()
    pe = struct.unpack_from('<I', data, 0x3c)[0]
    count = struct.unpack_from('<H', data, pe + 6)[0]
    optional = struct.unpack_from('<H', data, pe + 20)[0]
    ranges = []
    for va, size in TARGETS:
        rva = va - 0x140000000
        for i in range(count):
            o = pe + 24 + optional + i * 40
            _, start, length, raw = struct.unpack_from('<IIII', data, o + 8)
            if start <= rva and rva + size <= start + length:
                part = data[raw + rva - start:raw + rva - start + size]
                ranges.append(dict(va=hex(va), length=size, sha256=hashlib.sha256(part).hexdigest(), hex=part.hex()))
                break
        else:
            raise RuntimeError(f'Unmapped {va:x}')
    return dict(path=str(path), sha256=hashlib.sha256(data).hexdigest(), ranges=ranges)

files = [read_ranges(p) for p in paths]
report = dict(files=files, ranges_match=files[0]['ranges'] == files[1]['ranges'])
(OUT / 'original-ranges.json').write_text(json.dumps(report, indent=2), encoding='utf-8')
print(json.dumps(dict(ranges_match=report['ranges_match'], files=[{k:v for k,v in f.items() if k != 'ranges'} for f in files])))
