"""Collect only complete B PDB procedures; never execute original code or use DBs."""
from pathlib import Path
import gzip, hashlib, json
import capstone, pefile

WORK = Path(__file__).resolve().parent
ROOT = next(p for p in WORK.parents if (p / 'games_test/tiny-glade').is_dir())
OLD = ROOT / 'games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-57a33750ac4a866a76/core-producer-20261018/snapshot-state-deserialize-v1'
BIN = ROOT / 'ljxsj_92385/Tiny Glade/tiny-glade.rne'
pe = pefile.PE(str(BIN), fast_load=True)
imagebase = pe.OPTIONAL_HEADER.ImageBase
md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
md.detail = True
aliases = json.load(gzip.open(ROOT / 'games_test/reverse-engineering/tiny-glade/skeleton/_pdb/B/enrichment/address-aliases.json.gz', 'rt'))
added = [0x1415ae9a0, 0x1415b0ef0, 0x1415ad9a0, 0x1415b3890,
         0x1415d1bb0, 0x1415d3c70]
old_rows = json.loads((OLD / 'native/ranges.json').read_text())
requested = [int(r['va'], 16) for r in old_rows] + added
module_ids = {int(a['procedure_id'].split(':')[0]) for va in requested for a in aliases[str(va-imagebase)]}
procedures = {}
for module in module_ids:
    path = ROOT / f'games_test/reverse-engineering/tiny-glade/skeleton/_pdb/B/modules/{module:05d}.jsonl.gz'
    for line in gzip.open(path, 'rt'):
        p = json.loads(line)
        if p.get('class') == 'procedure' and p.get('section') == 1:
            procedures.setdefault(imagebase + 0x1000 + p['section_offset'], p)
rows = []
for va in requested:
    p = procedures[va]
    raw = pe.get_data(va-imagebase, p['code_size'])
    instructions = list(md.disasm(raw, va))
    assert sum(i.size for i in instructions) == len(raw)
    text = ''.join(f'{i.address:x}: {i.bytes.hex()}  {i.mnemonic} {i.op_str}\n' for i in instructions)
    if va not in added:
        assert (OLD / f'native/{va:x}.bin').read_bytes() == raw
        assert (OLD / f'native/{va:x}.asm').read_text() == text
    # Copies under the correct delegated owner keep the old Dormer container
    # unchanged; the prior-input inventory records the historical provenance.
    prefix = WORK / f'native/{va:x}'
    prefix.with_suffix('.bin').write_bytes(raw)
    prefix.with_suffix('.asm').write_text(text)
    prefix.with_suffix('.pdb.json').write_text(json.dumps(p, indent=2) + '\n')
    calls, refs = [], []
    for i in instructions:
        if i.mnemonic == 'call' and i.operands[0].type == capstone.x86.X86_OP_IMM:
            target = i.operands[0].imm
            calls.append({'site': hex(i.address), 'target': hex(target), 'pdb_aliases': aliases.get(str(target-imagebase), [])})
        for o in i.operands:
            if o.type == capstone.x86.X86_OP_MEM and o.mem.base == capstone.x86.X86_REG_RIP:
                target = i.address + i.size + o.mem.disp
                data = pe.get_data(target-imagebase, 64)
                refs.append({'site': hex(i.address), 'target': hex(target), 'bytes64': data.hex()})
    prefix.with_suffix('.calls.json').write_text(json.dumps(calls, indent=2) + '\n')
    prefix.with_suffix('.refs.json').write_text(json.dumps(refs, indent=2) + '\n')
    rows.append({'va': hex(va), 'bytes': len(raw), 'instructions': len(instructions), 'sha256': hashlib.sha256(raw).hexdigest(), 'module': p['module'], 'record_offset': p['record_offset'], 'name': p['name'], 'scope': 'complete PDB procedure including owned cleanup funclets'})
(WORK / 'native/ranges.json').write_text(json.dumps(rows, indent=2) + '\n')
print(json.dumps({'procedures': len(rows), 'bytes': sum(r['bytes'] for r in rows), 'instructions': sum(r['instructions'] for r in rows), 'native_execution': False}))
