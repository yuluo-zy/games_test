import struct
path = '/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne'
with open(path, 'rb') as f:
    data = f.read()
def read_cstr(va):
    fo = 0x2923600 + (va - 0x142925000)
    end = fo
    while end < len(data) and data[end] != 0 and 32 <= data[end] < 127:
        end += 1
    return data[fo:end].decode('ascii', errors='replace')
# FirstPerson string region 0x142ad8eb0
for start in range(0x142ad8ea0, 0x142ad8ee0, 8):
    s = read_cstr(start)
    if s and 'crate' not in s:
        print(f'{hex(start)}: {s!r}')
