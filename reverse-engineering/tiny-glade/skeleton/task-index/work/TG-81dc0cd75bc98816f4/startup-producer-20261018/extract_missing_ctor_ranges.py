from pathlib import Path
import hashlib
import json
import re
import pefile
from capstone import Cs, CS_ARCH_X86, CS_MODE_64

root = Path('/Users/yuluo/Desktop/game')
binary = root / 'ljxsj_92385/Tiny Glade/tiny-glade.rne'
expected = 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
data = binary.read_bytes()
assert hashlib.sha256(data).hexdigest() == expected
pe = pefile.PE(data=data, fast_load=True)
procs = root / 'games_test/reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv'
wanted = {0x14161a760, 0x1416159e0, 0x141617840, 0x141615bb0}
rows = {}
for line in procs.read_text().splitlines():
    cols = line.split('\t')
    try:
        va = int(cols[0], 0)
    except ValueError:
        continue
    if va in wanted:
        rows[va] = (int(cols[1]), cols[4])
assert set(rows) == wanted
out = Path(__file__).parent / 'extra-constructors'
out.mkdir(exist_ok=True)
cs = Cs(CS_ARCH_X86, CS_MODE_64)
cs.detail = True
manifest = []
for va, (length, name) in sorted(rows.items()):
    code = pe.get_data(va - pe.OPTIONAL_HEADER.ImageBase, length)
    insns = list(cs.disasm(code, va))
    assert sum(i.size for i in insns) == length
    path = out / f'{va:x}.asm'
    path.write_text('\n'.join(f'{i.address:x}: {i.mnemonic} {i.op_str}' for i in insns) + '\n')
    strings = []
    for index, ins in enumerate(insns):
        if ins.mnemonic != 'call' or ins.op_str != '0x1416a2dc0':
            continue
        near = insns[max(0,index-8):index]
        lea = next((i for i in near if i.mnemonic == 'lea' and i.op_str.startswith('r8, [rip')), None)
        size = next((i for i in near if i.mnemonic == 'mov' and i.op_str.startswith('r9d, ')), None)
        if lea and size:
            target = lea.address + lea.size + lea.operands[1].mem.disp
            count = int(size.op_str.split(', ')[1], 0)
            value = pe.get_data(target-pe.OPTIONAL_HEADER.ImageBase,count).decode()
            strings.append({'va':hex(target),'name':value,'length':count})
    manifest.append({'entry':hex(va),'length':length,'symbol':name,'range_sha256':hashlib.sha256(code).hexdigest(),
        'path':str(path),'create_names':strings})
(out/'manifest.json').write_text(json.dumps({'binary':str(binary),'binary_sha256':expected,
    'scope':'Four bounded full-PDB constructor ranges, disassembly only, not execution or behavioral verification',
    'procs_sha256':hashlib.sha256(procs.read_bytes()).hexdigest(),'functions':manifest},indent=2)+'\n')
print(json.dumps(manifest,indent=2))
