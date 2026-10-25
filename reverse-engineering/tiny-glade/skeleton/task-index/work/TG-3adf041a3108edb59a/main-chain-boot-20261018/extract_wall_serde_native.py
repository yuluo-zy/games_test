"""Bounded original-B instructions for save/history owned-wall reconstruction."""
import hashlib
from pathlib import Path
import capstone
import pefile

ROOT = Path('/Users/yuluo/Desktop/game')
OUT = Path(__file__).parent
RAW = (ROOT / 'ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
assert hashlib.sha256(RAW).hexdigest() == 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
PE = pefile.PE(data=RAW, fast_load=True)
BASE = PE.OPTIONAL_HEADER.ImageBase
MD = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
MD.detail = True
symbols = {}
for line in (ROOT / 'games_test/reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv').read_text().splitlines():
    fields = line.split('\t')
    if len(fields) >= 5 and fields[0].startswith('0x'):
        symbols.setdefault(int(fields[0], 16), fields[4])

def read(address, size):
    offset = PE.get_offset_from_rva(address - BASE)
    return RAW[offset:offset + size]

FUNCTIONS = [
    ('wall-style-legacy', 0x14085ebc0, 707),
    ('wall-field-visitor', 0x140861520, 600),
    ('wall-object-visitor', 0x1415bb1c0, 5253),
    ('wall-sequence-visitor', 0x1415b23b0, 1989),
    ('curve2-as3', 0x141285770, 767),
    ('wall-edit-deserializer', 0x141401960, 2390),
]
for name, address, size in FUNCTIONS:
    lines, refs = [], []
    for inst in MD.disasm(read(address, size), address):
        annotation = []
        for operand in inst.operands:
            if operand.type == capstone.x86.X86_OP_MEM and operand.mem.base == capstone.x86.X86_REG_RIP:
                target = inst.address + inst.size + operand.mem.disp
                try:
                    value = read(target, 80)
                except pefile.PEFormatError:
                    continue
                annotation.append(f'{target:x} {value.hex()} {value!r}')
            elif inst.mnemonic == 'call' and operand.type == capstone.x86.X86_OP_IMM:
                annotation.append(symbols.get(operand.imm, f'{operand.imm:x}'))
        line = f'{inst.address:016x}: {inst.mnemonic} {inst.op_str}'
        if annotation:
            line += ' ; ' + ' | '.join(annotation)
            refs.append(line)
        lines.append(line)
    (OUT / (name + '-rne.asm')).write_text('\n'.join(lines) + '\n')
    (OUT / (name + '-references.txt')).write_text('\n'.join(refs) + '\n')
    print(name, hex(address), size, len(lines))
