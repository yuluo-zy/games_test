import struct, sys
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
path = '/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne'
with open(path, 'rb') as f:
    data = f.read()
e_lfanew = struct.unpack_from('<I', data, 0x3c)[0]
nsec = struct.unpack_from('<H', data, e_lfanew+6)[0]
opt_size = struct.unpack_from('<H', data, e_lfanew+20)[0]
opt = e_lfanew + 24
image_base = struct.unpack_from('<Q', data, opt+24)[0]
sec = opt + opt_size
sections = []
for i in range(nsec):
    name = data[sec:sec+8].rstrip(b'\0').decode()
    vsize = struct.unpack_from('<I', data, sec+8)[0]
    va = struct.unpack_from('<I', data, sec+12)[0]
    raw_size = struct.unpack_from('<I', data, sec+16)[0]
    raw_ptr = struct.unpack_from('<I', data, sec+20)[0]
    sections.append((image_base+va, vsize, raw_ptr, raw_size, name))
    sec += 40
def file_offset(va):
    for sva, vsize, raw_ptr, raw_size, name in sections:
        if sva <= va < sva + max(vsize, raw_size):
            return raw_ptr + (va - sva), name
    return None, None
md = Cs(CS_ARCH_X86, CS_MODE_64)
md.detail = True
def disasm_va(va, length):
    fo, name = file_offset(va)
    if fo is None:
        print(f'{hex(va)}: no mapping'); return
    code = data[fo:fo+length]
    print(f'===== {hex(va)} (section {name}, {hex(fo)}) =====')
    for ins in md.disasm(code, va):
        # annotate data refs
        txt = f"0x{ins.address:x}  {ins.mnemonic:6s} {ins.op_str}"
        print(txt)
    print()
if __name__ == '__main__':
    for arg in sys.argv[1:]:
        va, ln = arg.split(':')
        disasm_va(int(va,16), int(ln))
