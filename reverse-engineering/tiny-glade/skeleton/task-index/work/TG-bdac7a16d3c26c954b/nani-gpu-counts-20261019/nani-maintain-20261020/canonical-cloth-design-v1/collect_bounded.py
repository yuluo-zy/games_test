"""Read-only B PE/PDB-cache collector; does not execute target or compiler."""
from pathlib import Path
import gzip
import hashlib
import json
import platform
import time
import capstone
import pefile

ROOT = Path('/Users/yuluo/Desktop/game')
REPO = ROOT / 'games_test'
OUT = Path(__file__).parent
BIN = ROOT / 'ljxsj_92385/Tiny Glade/tiny-glade.rne'
PDB = BIN.with_name('tiny_glade.pdb')
EXPECTED_BIN = 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
EXPECTED_PDB = '2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911'

def sha(data):
    return hashlib.sha256(data).hexdigest()

def emit(name, data):
    p = OUT / name
    assert not p.exists(), f'immutable output already exists: {p}'
    p.write_text(json.dumps(data, indent=2, ensure_ascii=False) + '\n')
    return sha(p.read_bytes())

start = time.perf_counter()
raw = BIN.read_bytes()
assert sha(raw) == EXPECTED_BIN
assert sha(PDB.read_bytes()) == EXPECTED_PDB
pe = pefile.PE(data=raw, fast_load=True)
base = pe.OPTIONAL_HEADER.ImageBase
cs = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
cs.detail = True
requests = [(3098, 184028, 0x14207c0a0, 3253),
            (478, 83892, 0x14084cc00, None)]
records = []
for module, offset, va, expected_size in requests:
    cache = REPO / f'reverse-engineering/tiny-glade/skeleton/_pdb/B/modules/{module:05}.jsonl.gz'
    found = []
    with gzip.open(cache, 'rt') as f:
        for line in f:
            rec = json.loads(line)
            if rec.get('record_offset') == offset and rec.get('class') == 'procedure':
                found.append(rec)
    assert len(found) == 1, (module, offset, found)
    rec = found[0]
    size = rec['code_size']
    if expected_size is not None:
        assert size == expected_size
    section = pe.sections[rec['section'] - 1]
    assert base + section.VirtualAddress + rec['section_offset'] == va
    rva = va - base
    file_off = pe.get_offset_from_rva(rva)
    body = raw[file_off:file_off + size]
    instructions = []
    direct = []
    cursor = va
    for inst in cs.disasm(body, va):
        assert inst.address == cursor
        assert bytes(inst.bytes) == body[cursor - va:cursor - va + inst.size]
        instructions.append(f'{inst.address:x}: {inst.bytes.hex()} {inst.mnemonic} {inst.op_str}')
        if inst.mnemonic in ('call', 'jmp') and inst.operands and inst.operands[0].type == capstone.x86.X86_OP_IMM:
            direct.append({'at': hex(inst.address), 'target': hex(inst.operands[0].imm)})
        cursor += inst.size
    assert cursor == va + size, (hex(cursor), hex(va + size))
    asm = OUT / f'{va:x}-full-B-rne.asm'
    assert not asm.exists()
    asm.write_text('\n'.join(instructions) + '\n')
    records.append({'va': hex(va), 'size': size, 'pdb_record': rec,
                    'module_cache': str(cache), 'module_cache_sha256': sha(cache.read_bytes()),
                    'body_sha256': sha(body), 'assembly': asm.name,
                    'assembly_sha256': sha(asm.read_bytes()), 'instruction_count': len(instructions),
                    'decoded_end': hex(cursor), 'continuous_exact_range': True,
                    'every_instruction_matches_original_file': True, 'direct': direct})

paths = [
 'crates/country-core/src/components/drawable.rs',
 'crates/country-core/src/components/mod.rs',
 'crates/country-core/src/components/rendering.rs',
 'crates/country-core/src/components/registry.rs',
 'crates/country-core/src/render/nani/systems.rs',
 'crates/country-core/src/render/nani/instance_updates.rs',
 'crates/country-core/src/asset_libraries/texture_library.rs',
 'crates/systems/decorator/src/utils/visual_entity_builder.rs',
 'crates/systems/decorator/src/decorator_visual/generate_clothline.rs',
 'crates/systems/decorator/src/decorator_visual/flag_connections.rs',
]
snapshots = []
for name in paths:
    src = REPO / 'tiny-glade' / name
    data = src.read_bytes()
    dst = OUT / 'source-snapshots' / name
    dst.parent.mkdir(parents=True, exist_ok=True)
    assert not dst.exists()
    dst.write_bytes(data)
    snapshots.append({'source': str(src), 'sha256': sha(data), 'bytes': len(data),
                      'snapshot': str(dst.relative_to(OUT))})
emit('SOURCE_SNAPSHOT_V1.json', {'scope': 'currently visible sources; no whole-App reachability claim',
                                'sources': snapshots})
emit('NATIVE_EVIDENCE_V1.json', {
 'scope': 'B only; two bounded PDB records and original PE ranges; C parity unverified',
 'backend': 'explicit project-authorized PE/Capstone backend, not LLVM collector output',
 'binary': str(BIN), 'binary_sha256': EXPECTED_BIN, 'pdb': str(PDB), 'pdb_sha256': EXPECTED_PDB,
 'binding': 'existing retained matching B PE/PDB identity; no new author-source/schema inference from debug names',
 'python': platform.python_version(), 'capstone': capstone.__version__, 'pefile': pefile.__version__,
 'target_executed': False, 'compiler_executed': False, 'external_analysis_processes': 0,
 'elapsed_seconds': time.perf_counter() - start, 'functions': records})
print(json.dumps({'complete_original_ranges': len(records), 'bytes': sum(r['size'] for r in records),
                  'source_snapshots': len(snapshots), 'target_executed': False}))
