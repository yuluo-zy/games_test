import struct, re
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
            return raw_ptr + (va - sva)
    return None
def read_cstr(va):
    fo = file_offset(va)
    if fo is None: return None
    end = fo
    while end < len(data) and data[end] != 0 and 32 <= data[end] < 127:
        end += 1
    return data[fo:end].decode('ascii', errors='replace')
def scan(va, length):
    fo = file_offset(va)
    if fo is None:
        print(f'{hex(va)}: no mapping'); return
    code = data[fo:fo+length]
    md = Cs(CS_ARCH_X86, CS_MODE_64)
    print(f'===== {hex(va)} =====')
    for ins in md.disasm(code, va):
        op = ins.op_str
        line = f"0x{ins.address:x}  {ins.mnemonic:6s} {op}"
        shown = False
        if '[rip' in op:
            m = re.search(r'\[rip \+ (0x[0-9a-f]+)\]', op)
            if m:
                tgt = ins.address + ins.size + int(m.group(1),16)
                if 0x142000000 <= tgt <= 0x144000000:
                    s = read_cstr(tgt)
                    line += f"  -> {hex(tgt)} {s!r}" if s else f"  -> {hex(tgt)}"
                    shown = True
        if ins.mnemonic == 'call' and ('0x140a5' in op or '0x14143' in op):
            shown = True
        if '0x142' in op and ins.mnemonic in ('movabs','mov'):
            shown = True
        if shown:
            print(line)
    print()
if __name__ == '__main__':
    import sys
    for arg in sys.argv[1:]:
        va, ln = arg.split(':')
        scan(int(va,16), int(ln))
