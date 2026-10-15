import struct, json
from pathlib import Path
p=Path(r'D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe')
d=p.read_bytes()
pe=struct.unpack_from('<I',d,0x3c)[0]
ns=struct.unpack_from('<H',d,pe+6)[0]
sz=struct.unpack_from('<H',d,pe+20)[0]
secs=[struct.unpack_from('<IIII',d,pe+24+sz+40*i+8) for i in range(ns)]
def read(va,n):
    rva=va-0x140000000
    for vs,v,rs,r in secs:
        if v<=rva<v+vs:return d[r+rva-v:r+rva-v+n]
    raise ValueError(hex(va))
out={}
for va,n in [(0x1429259a0,8),(0x142960690,4),(0x1429258d4,4),(0x142925870,4),(0x142aa4550,16),(0x1429258e0,16),(0x1429df1b0,8),(0x142925904,4)]:
    b=read(va,n)
    out[hex(va)]={'hex':b.hex(),'f32':list(struct.unpack('<'+'f'*(n//4),b))}
text=json.dumps(out,indent=2)
Path(__file__).with_name('trigger-constants.json').write_text(text)
print(text)
