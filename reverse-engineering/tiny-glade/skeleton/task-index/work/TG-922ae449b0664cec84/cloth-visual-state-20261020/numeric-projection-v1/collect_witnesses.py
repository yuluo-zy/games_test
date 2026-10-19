from pathlib import Path
import hashlib
import json
import struct
import pefile
import capstone

repo = Path('/Users/yuluo/Desktop/game/games_test')
out = Path(__file__).parent
binary = repo.parent / 'ljxsj_92385/Tiny Glade/tiny-glade.rne'
raw = binary.read_bytes()
sha = lambda data: hashlib.sha256(data).hexdigest()
assert sha(raw) == 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe = pefile.PE(data=raw, fast_load=True)
pe.parse_data_directories(directories=[pefile.DIRECTORY_ENTRY['IMAGE_DIRECTORY_ENTRY_IMPORT']])
base = pe.OPTIONAL_HEADER.ImageBase
imports = {x.address: {'dll': e.dll.decode(), 'name': x.name.decode() if x.name else None,
                       'ordinal': x.ordinal}
           for e in pe.DIRECTORY_ENTRY_IMPORT for x in e.imports}
cs = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
cs.detail = True

def decode(va, size):
    pos = pe.get_offset_from_rva(va - base)
    data = raw[pos:pos + size]
    ins = list(cs.disasm(data, va))
    cursor = va
    for i in ins:
        assert i.address == cursor and bytes(i.bytes) == data[cursor - va:cursor - va + i.size]
        cursor += i.size
    assert cursor == va + size
    return ins

thunks = []
for va in [0x142923e60, 0x142923fa0]:
    i, = decode(va, 6)
    assert i.mnemonic == 'jmp' and i.operands[0].mem.base == capstone.x86.X86_REG_RIP
    ptr = i.address + i.size + i.operands[0].mem.disp
    thunks.append({'va': hex(va), 'bytes_hex': i.bytes.hex(), 'iat_va': hex(ptr),
                   'import': imports[ptr]})

constants = []
for lo, hi in [(0x1407e6ae3, 0x1407e6e86), (0x1407e7121, 0x1407e7129)]:
    for i in decode(lo, hi - lo):
        if i.mnemonic == 'lea':
            continue  # addresses/fat-string metadata are not float constants
        for op in i.operands:
            if op.type != capstone.x86.X86_OP_MEM or op.mem.base != capstone.x86.X86_REG_RIP:
                continue
            va = i.address + i.size + op.mem.disp
            pos = pe.get_offset_from_rva(va - base)
            data = raw[pos:pos + op.size]
            constants.append({'instruction_va': hex(i.address), 'instruction': i.mnemonic + ' ' + i.op_str,
                              'data_va': hex(va), 'bytes_hex': data.hex(), 'bytes': len(data),
                              'sha256': sha(data), 'u32_le': list(struct.unpack('<' + 'I' * (len(data) // 4), data))})

reused = [
 'TG-dbc2e274271d5e5274/cloth-visual-state-20261020/cloth-contract-v1/native-manifest.json',
 'TG-dbc2e274271d5e5274/cloth-visual-state-20261020/cloth-contract-v1/1407e5190-full-B-rne.asm',
 'TG-dbc2e274271d5e5274/cloth-visual-state-20261020/cloth-contract-v1/141293730-full-B-rne.asm',
 'TG-28b3a129d231927d1f/cloth-visual-state-20261020/consumer-primitives-v1/native-manifest-v1.json',
 'TG-28b3a129d231927d1f/cloth-visual-state-20261020/consumer-primitives-v1/1407e1430-full-B-rne.asm',
 'TG-28b3a129d231927d1f/cloth-visual-state-20261020/consumer-primitives-v1/1407e2510-full-B-rne.asm',
 'TG-922ae449b0664cec84/cloth-visual-state-20261020/projection-evidence-v1/PROJECTION_EVIDENCE_V1.json',
 'TG-922ae449b0664cec84/cloth-visual-state-20261020/consumer-contract-v1/CONTRACT_MANIFEST_V2.json',
 'TG-1d8b7844066af51d6b/cloth-visual-state-20261020/cloth-utils-harness-v1/runtime-v1/test-run-v1.json',
]
root = repo / 'reverse-engineering/tiny-glade/skeleton/task-index/work'
ref = [{'path': str(root / x), 'sha256': sha((root / x).read_bytes())} for x in reused]
sources = []
for x in ['crates/utils/src/lib.rs', 'crates/utils/src/rng.rs',
          'crates/systems/decorator/src/decorator_visual/generate_clothline.rs',
          'crates/systems/decorator/src/decorator_visual/flag_connections.rs',
          'crates/country-core/src/systems/collision/world.rs']:
    p = repo / 'tiny-glade' / x
    sources.append({'path': str(p), 'sha256': sha(p.read_bytes())})
result = {'scope': 'B-only additional bounded numerical/cache witnesses, not source implementation approval',
          'binary': str(binary), 'binary_sha256': sha(raw),
          'thunks': thunks, 'constant_reads': constants,
          'reused_complete_original_evidence': ref, 'current_source_observation': sources,
          'original_manifests_modified': False, 'target_or_compiler_executed': False,
          'known_remaining': ['four-endpoint plan exact loop', 'second sound-event exact branch/position',
                              'thread sample expression order before final numeric design']}
p = out / 'NUMERIC_WITNESSES_V1.json'
assert not p.exists()
p.write_text(json.dumps(result, indent=2) + '\n')
print(json.dumps({'path': str(p), 'sha256': sha(p.read_bytes()), 'constant_reads': len(constants)}))
