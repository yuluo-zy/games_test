"""Read-only original PE/PDB checks for this bounded merge/overlap design."""
from pathlib import Path
import gzip
import hashlib
import json
import struct

ROOT = Path('/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton')
WORK = ROOT / 'task-index/work'
WALL = WORK / 'TG-e6891d165c4bc44d47/rhapsody-correctness-20261019/wall-merge-20261020'
ROOF = WORK / 'TG-4f40224d71f85debf9/rhapsody-correctness-20261019/roof-merge-20261020'
OVERLAP = WORK / 'TG-e3145687c65378b3d0/rhapsody-correctness-20261019/calculate-overlap-20261020'
CALLER = WORK / 'TG-60234a3eb4a6f6e4d2/rhapsody-correctness-20261019/snap-balcony-door-20261020/calculate-contract-20261020/caller-roles-addendum-v1/native-pack'
PE = Path('/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')
blob = PE.read_bytes()
assert hashlib.sha256(blob).hexdigest() == 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
nt = struct.unpack_from('<I', blob, 0x3c)[0]
section_count = struct.unpack_from('<H', blob, nt + 6)[0]
optional_size = struct.unpack_from('<H', blob, nt + 20)[0]
optional = nt + 24
image_base = struct.unpack_from('<Q', blob, optional + 24)[0]
sections = []
for i in range(section_count):
    section = optional + optional_size + i * 40
    vsize, rva, raw_size, raw_offset = struct.unpack_from('<IIII', blob, section + 8)
    sections.append((rva, max(vsize, raw_size), raw_offset))


def raw(va, size):
    rva = va - image_base
    for first, length, offset in sections:
        if first <= rva and rva + size <= first + length:
            return blob[offset + rva - first:offset + rva - first + size]
    raise AssertionError(hex(va))


requests = [
    (WALL / 'native-pack/wall-merge-select-full.json', 3064, 264),
    (WALL / 'native-pack/wall-merge-propose-full.json', 3064, 1256),
    (WALL / 'callback-native-pack/merge-wall-intersection-full.json', 3051, 5608),
    (WALL / 'callback-native-pack/merge-wall-predicate-full.json', 3051, 1664),
    (WALL / 'circular-native-pack/merge-wall-circular-predicate-full.json', 3051, 264),
    (ROOF / 'native-pack/roof-merge-select-full.json', 3063, 264),
    (ROOF / 'callback-native-pack/roof-merge-propose-closure-full.json', 3053, 264),
    (OVERLAP / 'native-pack/parent-wall-intersection-full.json', 3051, 9376),
    (OVERLAP / 'predicates-native-pack/parent-wall-circular-predicate-full.json', 3051, 3124),
    (OVERLAP / 'predicates-native-pack/parent-wall-linear-predicate-full.json', 3051, 4336),
    (CALLER / 'roof-intersection-for-calculate-full.json', 3053, 5772),
    (CALLER / 'roof-calculate-callback-full.json', 3053, 2968),
    (OVERLAP / 'rectangle-native-pack/original-rectangle3d-constructor-full.json', 768, 252),
    (OVERLAP / 'rectangle-native-pack/original-rectangle3d-overlap-full.json', 768, 1328),
]
procedures = {}
for _, module, _ in requests:
    if module not in procedures:
        procedures[module] = {
            obj['record_offset']: obj
            for line in gzip.open(ROOT / f'_pdb/B/modules/{module:05}.jsonl.gz', 'rt')
            if (obj := json.loads(line)).get('class') == 'procedure'
        }

checks = []
for path, module, record in requests:
    obj = json.loads(path.read_text())
    assert obj['status'] == 'ok' and not obj['original_target_executed']
    range_ = obj['range']
    start = int(range_['start_va'], 16)
    end = int(range_['stop_va_exclusive'], 16)
    cursor = start
    for instruction in obj['instructions']:
        address = int(instruction['va'], 16)
        encoded = bytes.fromhex(instruction['bytes'])
        assert cursor == address and len(encoded) == instruction['size']
        assert raw(address, len(encoded)) == encoded
        assert not instruction['crosses_requested_stop']
        cursor += len(encoded)
    assert cursor == end and end - start == range_['length']
    proc = procedures[module][record]
    assert proc['code_size'] == range_['length']
    assert proc['section'] == 1
    assert proc['section_offset'] + 0x1000 == start - image_base
    checks.append({
        'path': str(path), 'sha256': hashlib.sha256(path.read_bytes()).hexdigest(),
        'start_va': hex(start), 'length': end - start,
        'instructions': len(obj['instructions']), 'pdb_procedure_id': f'{module}:{record}',
        'native_sha256': hashlib.sha256(raw(start, end - start)).hexdigest(),
        'continuous_full_instruction_pe_match': True, 'pdb_size_and_start_match': True,
        'collector_function_boundary_verified': range_['function_boundary_verified'],
    })

constants = []
for address in [0x142925870, 0x142925950, 0x1429258d8, 0x142925908, 0x142925904, 0x1429cd068]:
    value = raw(address, 4)
    constants.append({'va': hex(address), 'bytes': value.hex(), 'f32': struct.unpack('<f', value)[0]})
vectors = []
for address in [0x142fcbce0, 0x142fcbcf0, 0x142a82400, 0x142abcb30, 0x1429258e0]:
    value = raw(address, 16)
    vectors.append({'va': hex(address), 'bytes': value.hex(), 'f32': struct.unpack('<4f', value)})
value = raw(0x142925970, 4)
constants.append({'va': hex(0x142925970), 'bytes': value.hex(), 'f32': struct.unpack('<f', value)[0]})
table = []
for index in range(4):
    address = 0x142fcbe30 + index * 4
    value = raw(address, 4)
    table.append({'index': index, 'bytes': value.hex(), 'target_va': hex(0x142fcbe30 + struct.unpack('<i', value)[0])})
result = {'schema_version': 1, 'original_target_executed': False,
          'checks': checks, 'constants': constants, 'vectors': vectors,
          'roof_callback_destination_table': table}
for directory in [WALL, ROOF, OVERLAP]:
    (directory / 'native-range-verification.json').write_text(json.dumps(result, indent=2) + '\n')
print(json.dumps({'full_functions': len(checks), 'full_instruction_count': sum(c['instructions'] for c in checks),
                  'verification_sha256': hashlib.sha256((WALL / 'native-range-verification.json').read_bytes()).hexdigest()}))
