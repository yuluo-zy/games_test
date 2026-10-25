"""Bounded original B instruction evidence; no Ghidra/database mutations."""
import hashlib
import json
import re
import struct
from pathlib import Path
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
from capstone.x86 import X86_OP_IMM, X86_OP_MEM, X86_REG_RIP

ROOT = Path(__file__).parent
SRC = Path('D:/game/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-cfac1f498c427933d2/core-gui-20261015')
BIN = Path('D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe')
data = BIN.read_bytes()
assert hashlib.sha256(data).hexdigest() == 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe = struct.unpack_from('<I', data, 60)[0]
count = struct.unpack_from('<H', data, pe + 6)[0]
sections = pe + 24 + struct.unpack_from('<H', data, pe + 20)[0]
def offset(va):
    for i in range(count):
        vs, rva, rawsize, raw = struct.unpack_from('<IIII', data, sections + i*40 + 8)
        if rva <= va - 0x140000000 < rva + vs:
            return raw + va - 0x140000000 - rva
    raise ValueError(hex(va))

functions = {}
for path in SRC.glob('*.json'):
    obj = json.loads(path.read_text(encoding='utf-8-sig'))
    for content in obj.get('content', []):
        if content.get('type') == 'text':
            functions.update(json.loads(content['text']).get('functions', {}))
for obj in json.loads((SRC/'native/payloads/001-get_functions.json').read_text()):
    functions.update(obj.get('functions', {}))
md = Cs(CS_ARCH_X86, CS_MODE_64)
md.detail = True
for va, f in functions.items():
    addr = int(va, 16)
    lines, focused = [], []
    for ins in md.disasm(data[offset(addr):offset(addr)+f['size']], addr):
        line = f'{ins.address:#x} {ins.mnemonic:8} {ins.op_str}'
        for op in ins.operands:
            if op.type == X86_OP_MEM and op.mem.base == X86_REG_RIP:
                target = ins.address + ins.size + op.mem.disp
                try:
                    raw = data[offset(target):offset(target)+96]
                    text = re.match(rb'[ -~]{4,}', raw)
                    if text:
                        line += f' ; @{target:#x} {text[0].decode()!r}'
                except ValueError:
                    pass
        lines.append(line)
        if '; @' in line or re.search(r'\[r(?!bp|sp)\w+ \+ 0x(?:e8|ec|f[0-9a-f]|10[0-4])\]', line):
            focused.extend(lines[-3:])
    (ROOT/f'{va}.asm').write_text('\n'.join(lines), encoding='utf-8')
    if focused:
        print(f'\n{va} {f["name"]}\n' + '\n'.join(focused))
