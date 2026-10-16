import struct
path = '/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne'
with open(path, 'rb') as f:
    data = f.read()
def read_f32(va):
    fo = 0x2923600 + (va - 0x142925000)
    return struct.unpack_from('<f', data, fo)[0]
# the value byte at event+0x15 indexes the table at 0x142aa1590
# decode a run of f32s
for i in range(8):
    print(f'table[{i}] = {read_f32(0x142aa1590 + i*4)}')
