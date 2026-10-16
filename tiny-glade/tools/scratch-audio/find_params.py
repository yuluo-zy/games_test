import struct
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
path = '/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne'
with open(path, 'rb') as f:
    data = f.read()
def read_cstr(va):
    fo = 0x2923600 + (va - 0x142925000)
    end = fo
    while end < len(data) and data[end] != 0 and 32 <= data[end] < 127:
        end += 1
    return data[fo:end].decode('ascii', errors='replace')
def find_setparam_calls(va, length):
    fo = 0x400 + (va - 0x140001000)  # .text section
    code = data[fo:fo+length]
    md = Cs(CS_ARCH_X86, CS_MODE_64)
    insns = list(md.disasm(code, va))
    for i, ins in enumerate(insns):
        if ins.mnemonic == 'call' and '0x140a53db0' in ins.op_str:
            # look back for lea rdx, [rip+...]
            for j in range(i-1, max(0,i-8), -1):
                prev = insns[j]
                if prev.mnemonic == 'lea' and 'rdx,' in prev.op_str and '[rip' in prev.op_str:
                    import re
                    m = re.search(r'\[rip \+ (0x[0-9a-f]+)\]', prev.op_str)
                    if m:
                        tgt = prev.address + prev.size + int(m.group(1),16)
                        s = read_cstr(tgt)
                        print(f"  set_parameter name @ {hex(prev.address)} -> {hex(tgt)} {s!r}")
                        break
            else:
                # also look for the r8d length immediately before
                for j in range(i-1, max(0,i-6), -1):
                    if insns[j].mnemonic == 'mov' and 'r8d' in insns[j].op_str:
                        print(f"  set_parameter r8d: {insns[j].op_str}")
                        break
import sys
find_setparam_calls(int(sys.argv[1],16), int(sys.argv[2],16))
