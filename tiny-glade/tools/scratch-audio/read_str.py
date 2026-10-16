import struct
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
def read_str(va, length=None):
    for sva, vsize, raw_ptr, raw_size, name in sections:
        if sva <= va < sva + max(vsize, raw_size):
            fo = raw_ptr + (va - sva)
            end = fo
            if length is None:
                while end < len(data) and data[end] != 0 and 32 <= data[end] < 127:
                    end += 1
                return data[fo:end].decode('ascii', errors='replace')
            else:
                return data[fo:fo+length].decode('ascii', errors='replace')
    return None
for va in [0x142a98d0, 0x142a9710, 0x142c62943]:
    s = read_str(va)
    print(f'VA {hex(va)}: {s!r}')
