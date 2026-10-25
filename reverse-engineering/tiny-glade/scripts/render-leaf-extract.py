"""Focused instruction evidence for already-identified renderer leaves."""
from pathlib import Path
import json, hashlib, struct
from capstone import Cs, CS_ARCH_X86, CS_MODE_64

ROOT = Path('D:/game/reverse-engineering/tiny-glade')
OUT = ROOT / 'evidence/render-leaves'
OUT.mkdir(exist_ok=True)
raw = (ROOT / 'workspace/input/tiny-glade.exe').read_bytes()
pe = struct.unpack_from('<I', raw, 60)[0]
opt = pe + 24
base = struct.unpack_from('<Q', raw, opt + 24)[0]
sec = []
for i in range(struct.unpack_from('<H', raw, pe + 6)[0]):
    at = opt + struct.unpack_from('<H', raw, pe + 20)[0] + 40 * i
    vs, va, rs, ro = struct.unpack_from('<IIII', raw, at + 8)
    sec.append((raw[at:at+8].rstrip(b'\0').decode(), vs, va, rs, ro))
def read(va, size):
    for name, vs, s, rs, ro in sec:
        if s <= va-base < s+rs:
            return raw[ro+va-base-s:ro+va-base-s+size]
    raise ValueError(hex(va))
symbols = json.loads((ROOT / 'evidence/render-architecture/public-symbols.json').read_text())
exception_rva, exception_size = struct.unpack_from('<II', raw, opt+112+3*8)
exception_data = read(base+exception_rva, exception_size)
exception_entries = [struct.unpack_from('<III', exception_data, at)
                     for at in range(0, exception_size, 12)]
report = {'source_sha256': hashlib.sha256(raw).hexdigest(), 'functions': []}
cs = Cs(CS_ARCH_X86, CS_MODE_64)
for va, cap in [(0x140bd4b00, 0x80), (0x140bd49d0, 0x124), (0x140c10e10, 0x20)]:
    insns = list(cs.disasm(read(va, cap), va))
    # Returning leaf boundaries stop at RET; the image converter has multiple
    # return blocks, so retain the complete bounded region including panic.
    if va != 0x140bd49d0:
        insns = insns[:next(i+1 for i,x in enumerate(insns) if x.mnemonic == 'ret')]
    f = {'address': hex(va), 'end_exclusive': hex(insns[-1].address+insns[-1].size),
         'range_kind': 'PE exception directory RUNTIME_FUNCTION range' if va == 0x140bd49d0 else 'entry through first RET, straight-line leaf',
         'runtime_function': [{'begin':hex(base+b),'end_exclusive':hex(base+e),'unwind_info':hex(base+u)} for b,e,u in exception_entries if base+b==va],
         'aliases': [s for s in symbols if s['va'] == hex(va)],
         'instructions': [f'{i.address:#x}: {i.bytes.hex():24s} {i.mnemonic} {i.op_str}' for i in insns]}
    (OUT / (hex(va)+'-asm.txt')).write_text('\n'.join(f['instructions'])+'\n')
    report['functions'].append(f)
mcp = json.loads((ROOT/'evidence/render-architecture/mcp-leaf-resource-usage-report.json').read_text())
for row in mcp['results']:
    if row['name'] == 'decompile_function' and 'structuredContent' in row['result']:
        r = row['result']['structuredContent']
        (OUT/(r['address'].lower()+'.c')).write_text(r['decompiled_code'])
(OUT/'native-evidence.json').write_text(json.dumps(report,indent=2))
print(json.dumps({f['address']: f['end_exclusive'] for f in report['functions']}))
