"""Confirm that analyzed B instructions are unchanged in the retained release."""
import hashlib
import json
import struct
from pathlib import Path

WORK = Path(__file__).resolve().parent
PROJECT = WORK.parents[4]
REPO = PROJECT.parents[2]


class Pe:
    def __init__(self, path):
        self.path = path
        self.data = path.read_bytes()
        pe = struct.unpack_from('<I', self.data, 0x3c)[0]
        count = struct.unpack_from('<H', self.data, pe + 6)[0]
        opt_size = struct.unpack_from('<H', self.data, pe + 20)[0]
        opt = pe + 24
        assert self.data[pe:pe + 4] == b'PE\0\0'
        assert struct.unpack_from('<H', self.data, opt)[0] == 0x20b
        self.base = struct.unpack_from('<Q', self.data, opt + 24)[0]
        self.sections = []
        for index in range(count):
            row = opt + opt_size + index * 40
            _, va, raw_size, raw = struct.unpack_from('<IIII', self.data, row + 8)
            self.sections.append((va, raw_size, raw))

    def span(self, address, size):
        rva = address - self.base
        for va, raw_size, raw in self.sections:
            if va <= rva and rva + size <= va + raw_size:
                return self.data[raw + rva - va:raw + rva - va + size]
        raise ValueError(hex(address))


release = Pe(REPO / 'ljxsj_92385/Tiny Glade/tiny-glade.rne')
analyzed = Pe(REPO / 'ljxsj_92385/Tiny Glade/tiny-glade.exe')
spans = {}
for name in ['schema-direct.json', 'writers-direct.json', 'preplaced-direct.json',
             'remaining-descriptors-direct.json']:
    report = json.loads((WORK / 'evidence' / name).read_text())
    result = report.get('result', report)
    assert not result.get('isError'), name
    decoded = json.loads(result['content'][0]['text'])
    for address, function in decoded['functions'].items():
        assert not function.get('error'), (address, function)
        assert function.get('decompiled_code'), address
        spans[int(address, 16)] = function['size']
spans[0x142279500] = 0x142279ecf - 0x142279500
spans[0x1422689c0] = 1004
rows = []
for address, size in sorted(spans.items()):
    before = release.span(address, size)
    after = analyzed.span(address, size)
    rows.append({'address': hex(address), 'size': size,
                 'release_sha256': hashlib.sha256(before).hexdigest(),
                 'analyzed_sha256': hashlib.sha256(after).hexdigest(),
                 'identical': before == after})
report = {
    'target': 'B',
    'release_sha256': hashlib.sha256(release.data).hexdigest(),
    'analysis_sha256': hashlib.sha256(analyzed.data).hexdigest(),
    'spans': rows,
    'all_identical': all(row['identical'] for row in rows),
    'scope': 'Static byte identity only; no execution or complete-chain equivalence asserted',
}
(WORK / 'evidence/original-span-identity.json').write_text(json.dumps(report, indent=2))
assert report['all_identical']
print(f'{len(rows)} original/analyzed instruction spans identical')
