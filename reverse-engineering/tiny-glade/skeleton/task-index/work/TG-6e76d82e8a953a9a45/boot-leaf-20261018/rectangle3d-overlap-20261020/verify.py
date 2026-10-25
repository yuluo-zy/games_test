from pathlib import Path
import gzip
import hashlib
import json
import struct

INDEX = Path('/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index')
WORK = INDEX / 'work'
HERE = Path(__file__).resolve().parent
OVERLAP = WORK / 'TG-e3145687c65378b3d0/rhapsody-correctness-20261019/calculate-overlap-20261020'
DERIVE = WORK / 'TG-a790a7eb5fc820c629/rhapsody-correctness-20261019/candidate-derive-20261020/exact-kernel-addendum-v1'
blob = Path('/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
assert hashlib.sha256(blob).hexdigest() == 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
nt = struct.unpack_from('<I', blob, 60)[0]
table = nt + 24 + struct.unpack_from('<H', blob, nt + 20)[0]
sections = [struct.unpack_from('<IIII', blob, table + i * 40 + 8)
            for i in range(struct.unpack_from('<H', blob, nt + 6)[0])]


def raw(va, size):
    rva = va - 0x140000000
    for vsize, first, rsize, offset in sections:
        if first <= rva and rva + size <= first + max(vsize, rsize):
            return blob[offset + rva - first:offset + rva - first + size]
    raise AssertionError(hex(va))


items = [
    (OVERLAP / 'rectangle-native-pack/original-rectangle3d-constructor-full.json', 768, 252),
    (OVERLAP / 'rectangle-native-pack/original-rectangle3d-overlap-full.json', 768, 1328),
    (DERIVE / 'native-pack/half-rectangle-closest-edge-full.json', 768, 30296),
    (DERIVE / 'closest-native-pack/rectangle-closest-common-full.json', 768, 28372),
    (DERIVE / 'native-pack/half-mapped-nearest-full.json', 1506, 20496),
]
checks = []
for path, module, record in items:
    obj = json.loads(path.read_text())
    cursor = int(obj['range']['start_va'], 16)
    start = cursor
    for instruction in obj['instructions']:
        data = bytes.fromhex(instruction['bytes'])
        assert int(instruction['va'], 16) == cursor
        assert len(data) == instruction['size'] and raw(cursor, len(data)) == data
        assert not instruction['crosses_requested_stop']
        cursor += len(data)
    assert cursor == int(obj['range']['stop_va_exclusive'], 16)
    for line in gzip.open(INDEX.parent / f'_pdb/B/modules/{module:05}.jsonl.gz', 'rt'):
        proc = json.loads(line)
        if proc.get('record_offset') == record:
            assert proc['class'] == 'procedure'
            assert proc['section_offset'] + 0x1000 == start - 0x140000000
            assert proc['code_size'] == cursor - start
            break
    else:
        raise AssertionError('missing procedure')
    checks.append({'path': str(path), 'sha256': hashlib.sha256(path.read_bytes()).hexdigest(),
                   'procedure_id': f'{module}:{record}', 'va': hex(start), 'length': cursor - start,
                   'instructions': len(obj['instructions']), 'full_continuous_pe_and_pdb_match': True,
                   'native_sha256': hashlib.sha256(raw(start, cursor - start)).hexdigest()})
assert raw(0x140c9acb0, 5) == bytes.fromhex('e91bfdffff')
assert 0x140c9acb0 + 5 + struct.unpack('<i', raw(0x140c9acb1, 4))[0] == 0x140c9a9d0
constants = [{'va': hex(a), 'bytes': raw(a, 4).hex(), 'f32': struct.unpack('<f', raw(a, 4))[0]}
             for a in [0x14295b630, 0x142a7baa0]]
result = {'schema_version': 1, 'original_target_executed': False, 'checks': checks,
          'exact_tail_jump_target': '0x140c9a9d0', 'constants': constants}
for directory in [HERE, DERIVE]:
    (directory / 'native-range-verification.json').write_text(json.dumps(result, indent=2) + '\n')
print(json.dumps({'sha256': hashlib.sha256((HERE / 'native-range-verification.json').read_bytes()).hexdigest(),
                  'full_functions': len(checks)}))
