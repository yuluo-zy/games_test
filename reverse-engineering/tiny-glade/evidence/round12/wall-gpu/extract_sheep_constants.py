from pathlib import Path
import struct,json
p=Path('ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes();pe=struct.unpack_from('<I',p,60)[0];sh=pe+24+struct.unpack_from('<H',p,pe+20)[0];sections=[struct.unpack_from('<4I',p,sh+i*40+8)for i in range(struct.unpack_from('<H',p,pe+6)[0])]
def read(va,n):
 r=va-0x140000000
 for vs,vr,rs,ro in sections:
  if vr<=r<vr+rs:return p[ro+r-vr:ro+r-vr+n]
for a,n in [(0x142ac6328,120),(0x142ac6a18,920),(0x142ac5d60,12),(0x142925960,8),(0x142abcb6c,4),(0x142a16cb8,4),(0x1429a0788,4),(0x14295a0e8,4)]:
 b=read(a,n);print(hex(a),repr(b) if n>12 else struct.unpack('<'+'f'*(n//4),b))
