import struct
path = '/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne'
with open(path, 'rb') as f:
    data = f.read()
def read(va, length):
    fo = 0x2923600 + (va - 0x142925000)
    return data[fo:fo+length]
def show(va, length):
    b = read(va, length)
    # print ascii + hex
    for i in range(0, len(b), 16):
        chunk = b[i:i+16]
        hexs = ' '.join(f'{x:02x}' for x in chunk)
        asci = ''.join(chr(x) if 32 <= x < 127 else '.' for x in chunk)
        print(f'{hex(va+i)}: {hexs:48s} {asci}')
# read 0x142ad8e00..0x142ad8eb0
show(0x142ad8e00, 0xb8)
