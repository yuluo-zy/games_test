"""Read-only original/current input audit; writes only this legal work directory."""
import gzip
import hashlib
import json
import pathlib
import re
import sqlite3
import struct

HERE = pathlib.Path(__file__).resolve().parent
RE = pathlib.Path('/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade')
WS = pathlib.Path('/Users/yuluo/Desktop/game/games_test/tiny-glade')
PE = pathlib.Path('/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')

def sha(path):
    return hashlib.sha256(pathlib.Path(path).read_bytes()).hexdigest()

def save(name, value):
    (HERE / name).write_text(json.dumps(value, ensure_ascii=False, indent=2) + '\n')

pe = PE.read_bytes()
e = struct.unpack_from('<I', pe, 0x3c)[0]
n = struct.unpack_from('<H', pe, e + 6)[0]
opt = struct.unpack_from('<H', pe, e + 20)[0]
sections = []
for i in range(n):
    off = e + 24 + opt + i * 40
    vs, rva, rs, rp = struct.unpack_from('<4I', pe, off + 8)
    sections.append((rva, max(vs, rs), rp))

def mem(va, count):
    rva = va - 0x140000000
    for start, size, offset in sections:
        if start <= rva and rva + count <= start + size:
            return pe[offset + rva - start:offset + rva - start + count]
    raise ValueError(hex(va))

def rows(path):
    out = []
    for line in pathlib.Path(path).read_text().splitlines():
        match = re.match(r'^([0-9a-f]+):\s+((?:[0-9a-f]{2} )*[0-9a-f]{2})\s*\t(.+)$', line)
        if match:
            out.append((int(match[1], 16), bytes.fromhex(match[2]), match[3]))
            continue
        match = re.match(r'^([0-9a-f]+):\s+([0-9a-f]+)\s+(.+)$', line)
        if match:
            out.append((int(match[1], 16), bytes.fromhex(match[2]), match[3]))
    return out

calc = HERE.parent / 'value-consumer-pack/calculate-interaction-intent-full.asm.txt'
place = RE / 'skeleton/task-index/work/TG-f7070bb3446b7a37eb/building-audit-20261018/v2/history-lifecycle-review/insta-producers/ui_place_decorator.asm'
move = HERE / 'native-pack/ui-move-decorator-full.asm.txt'
corner = RE / 'skeleton/task-index/work/TG-719dcf28a173a4b9b1/clutter-startup-20261020/location-result-20261020/corner-native-pack/corner-snap-threshold.asm.txt'
specs = [
    (calc, 0x14127cf70, 8546, 1522, 1440),
    (place, 0x1407a9c10, 15522, 422, 7120),
    (move, 0x1407a4850, 12790, 420, 2336),
    (HERE / 'boundary-callees-pack/calculate-move-dst-full.asm.txt', 0x1412801f0, 512, 1523, 3180),
    (HERE / 'boundary-callees-pack/rectangle-get-point-u-full.asm.txt', 0x140c996d0, 297, 768, 11864),
    (HERE / 'boundary-callees-pack/roof-proposal-option-full.asm.txt', 0x14127cde0, 122, 1522, 552),
    (corner, 0x140811900, 200, 453, 46448),
]
audits = []
for path, start, size, module, offset in specs:
    insns = rows(path)
    pc = start
    binary = bytearray()
    for address, raw, asm in insns:
        assert address == pc, (str(path), hex(pc), hex(address))
        assert raw == mem(address, len(raw)), (str(path), hex(address))
        binary.extend(raw)
        pc += len(raw)
    assert pc == start + size, (str(path), hex(pc), hex(start + size))
    proc = None
    for line in gzip.open(RE / f'skeleton/_pdb/B/modules/{module:05}.jsonl.gz', 'rt'):
        v = json.loads(line)
        if v.get('record_offset') == offset:
            proc = v
            break
    assert proc and proc['code_size'] == size
    audits.append({'path': str(path), 'sha256': sha(path), 'va': hex(start), 'length': size,
                   'instructions': len(insns), 'pe_equal': True, 'continuous_complete_range': True,
                   'native_bytes_sha256': hashlib.sha256(binary).hexdigest(), 'pdb_procedure': proc})
save('native-range-verification.json', {'pe_path': str(PE), 'pe_sha256': sha(PE), 'ranges': audits,
      'scope': 'instruction bytes/ranges/procedure identity only; no target execution or behavioral test'})

snippets = []
for label, path, lo, hi in [
    ('place-actual-arguments-copy-and-dispatch', place, 0x1407ac43e, 0x1407ac5c8),
    ('place-tag4-and-no-apply', place, 0x1407ac83c, 0x1407ac898),
    ('move-actual-arguments-and-0x60-return', move, 0x1407a6b6b, 0x1407a6c92),
    ('move-tag3-or-tag4-dispatch', move, 0x1407a6d75, 0x1407a6ebd),
    ('move-tag4-no-apply', move, 0x1407a6fd3, 0x1407a7015),
    ('calculate-entry-and-hit-dispatch', calc, 0x14127cfe4, 0x14127d0b3),
    ('calculate-wall-merge-and-shortcut', calc, 0x14127d1fd, 0x14127d3a1),
    ('calculate-wall-accepted-and-rejected', calc, 0x14127e7f7, 0x14127e8fd),
    ('calculate-roof-common-accepted-return', calc, 0x14127e9f5, 0x14127ea5a),
]:
    selected = [(address, raw, asm) for address, raw, asm in rows(path) if lo <= address < hi]
    assert selected[0][0] == lo and selected[-1][0] + len(selected[-1][1]) == hi
    snippets.append({'label': label, 'path': str(path), 'source_sha256': sha(path), 'start': hex(lo),
                     'end_exclusive': hex(hi), 'instructions': len(selected),
                     'native_bytes_sha256': hashlib.sha256(b''.join(x[1] for x in selected)).hexdigest()})
    text = '\n'.join(f'{address:x}: {raw.hex()}  {asm}' for address, raw, asm in selected)
    (HERE / f'{label}.asm.txt').write_text(text + '\n')
save('exact-snippets.json', snippets)

table_specs = [('hit-destination', 0x142c24718, 4), ('move-destination', 0x142c248b0, 4),
               ('rectangle-point-u', 0x142b2ea9c, 5)]
tables = []
for name, address, count in table_specs:
    raw = mem(address, count * 4)
    tables.append({'name': name, 'va': hex(address), 'entry_count': count, 'raw_hex': raw.hex(),
                   'signed_relative_targets': [hex(address + x) for x in struct.unpack('<' + 'i' * count, raw)]})
constants = []
for address in [0x142925904, 0x142925870, 0x142c2415c, 0x1429cd068, 0x1429a0764, 0x142934694, 0x142934698]:
    raw = mem(address, 4)
    constants.append({'va': hex(address), 'raw_hex': raw.hex(), 'u32': hex(struct.unpack('<I', raw)[0]),
                      'f32': struct.unpack('<f', raw)[0]})
save('finite-original-tables.json', {'tables': tables, 'scalar_constants': constants,
      'scope': 'selected exact read-only original B bytes; fourth hit table entry is last, not a fifth case'})

aliases = json.load(gzip.open(RE / 'skeleton/_pdb/B/enrichment/address-aliases.json.gz', 'rt'))
calls = []
for address, raw, asm in rows(calc):
    m = re.match(r'callq\s+0x([0-9a-f]+)', asm)
    if not m:
        continue
    target = int(m[1], 16)
    names = aliases.get(str(target - 0x140000000), [])
    calls.append({'pc': hex(address), 'target': hex(target), 'native_raw': raw.hex(),
                  'alias_count': len(names), 'aliases': names if len(names) <= 20 else [],
                  'folded_identity_unresolved_by_name': len(names) > 20})
save('calculate-physical-calls.json', calls)

paths = [
 'crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs',
 'crates/systems/decorator/src/ui_systems/move_ops.rs',
 'crates/systems/decorator/src/ui_systems/ui_move_decorator.rs',
 'crates/systems/decorator/src/ui_systems/ui_place_decorator.rs',
 'crates/systems/decorator/src/ui_systems/calculate_decorator_dst.rs',
 'crates/systems/decorator/src/ui_systems/calculate_decorator_grab_offset.rs',
 'crates/systems/decorator/src/merge_proposal_wall_decorators.rs',
 'crates/systems/decorator/src/merge_proposal_roof_decorators.rs',
 'crates/systems/decorator/src/collision_detection.rs',
 'crates/systems/decorator/src/derive_decorator_subtype.rs',
 'crates/country-core/src/resources/walls/decorator.rs',
 'crates/country-core/src/resources/walls/decorator_subtype.rs',
 'crates/country-core/src/resources/walls/decorator_storage.rs',
 'crates/country-core/src/resources/walls/public_wall_state.rs',
 'crates/country-core/src/resources/walls/public_walls.rs',
 'crates/country-core/src/systems/wall/shapes/shape_placement.rs',
 'crates/utils/src/curve/mod.rs',
 'crates/utils/src/geometry/rectangle.rs',
 'crates/systems/decorator/src/cull_oob_decorators.rs',
]
c = sqlite3.connect(f'file:{RE}/skeleton/task-index/index.sqlite3?mode=ro', uri=True)
sources = []
for relative in paths:
    path = WS / relative
    tasks = []
    for (task,) in c.execute('select distinct task_id from files where path=?', (relative,)):
        claim = RE / f'skeleton/task-index/claims/{task}.json'
        item = {'task_id': task, 'claim_present': claim.exists()}
        if claim.exists():
            v = json.loads(claim.read_text())
            item.update({'claim_path': str(claim), 'claim_sha256': sha(claim),
                         'owner': v['owner'], 'status': v['status'], 'work_dir': v['work_dir']})
        tasks.append(item)
    sources.append({'path': str(path), 'sha256': sha(path), 'bytes': path.stat().st_size, 'tasks': tasks})
save('current-source-ownership.json', sources)
