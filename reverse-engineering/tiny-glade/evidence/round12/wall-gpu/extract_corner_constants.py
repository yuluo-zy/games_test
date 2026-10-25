from pathlib import Path
import struct,json,re
r=Path('D:/game');data=(r/'ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes();pe=struct.unpack_from('<I',data,60)[0];opt=pe+24;sh=opt+struct.unpack_from('<H',data,pe+20)[0];sections=[struct.unpack_from('<4I',data,sh+i*40+8) for i in range(struct.unpack_from('<H',data,pe+6)[0])]
def raw(va,n):
 for vs,vr,rs,off in sections:
  if vr<=va-0x140000000 and va-0x140000000+n<=vr+rs:return data[off+va-0x140000000-vr:off+va-0x140000000-vr+n]
 raise KeyError(hex(va))
s=(r/'reverse-engineering/tiny-glade/evidence/round12/wall-gpu/corners-original.c').read_text(encoding='utf-8');addresses=sorted(set(int(a,16) for a in re.findall(r'(?:UNK|DAT)_([0-9a-f]{9})',s)))
small={hex(a):{'bits':struct.unpack('<I',raw(a,4))[0],'f32':struct.unpack('<f',raw(a,4))[0]} for a in addresses if a<0x142c1c578 and a>=0x142920000}
p=r/'reverse-engineering/tiny-glade/evidence/round12/wall-gpu/corners-native-constants.json';p.write_text(json.dumps(small,indent=2),encoding='utf-8');print(small)
