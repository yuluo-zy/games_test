import struct, re
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
path = '/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne'
with open(path, 'rb') as f:
    data = f.read()
def read_f32(va):
    fo = 0x2923600 + (va - 0x142925000)
    return struct.unpack_from('<f', data, fo)[0]
def scan(va, length):
    fo = 0x400 + (va - 0x140001000)
    code = data[fo:fo+length]
    md = Cs(CS_ARCH_X86, CS_MODE_64)
    for ins in md.disasm(code, va):
        if 'xmm' in ins.op_str and '[rip' in ins.op_str:
            m = re.search(r'\[rip \+ (0x[0-9a-f]+)\]', ins.op_str)
            if m:
                tgt = ins.address + ins.size + int(m.group(1),16)
                if 0x142000000 <= tgt <= 0x144000000:
                    print(f"0x{ins.address:x}  {ins.mnemonic:6s} {ins.op_str}  -> {hex(tgt)} = {read_f32(tgt)}")
import sys
scan(int(sys.argv[1],16), int(sys.argv[2],16))
