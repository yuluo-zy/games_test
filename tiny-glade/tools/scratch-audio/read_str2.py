import struct
path = '/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne'
with open(path, 'rb') as f:
    data = f.read()
def read_str(va, length):
    # rdata base
    fo = 0x2923600 + (va - 0x142925000)
    return data[fo:fo+length].decode('ascii', errors='replace')
for va, ln in [(0x142aa12d0, 0xf), (0x142aa1590, 16), (0x142aa1590, 32)]:
    try:
        print(f'VA {hex(va)} len {ln}: {read_str(va, ln)!r}')
    except Exception as e:
        print(f'{hex(va)}: err {e}')
