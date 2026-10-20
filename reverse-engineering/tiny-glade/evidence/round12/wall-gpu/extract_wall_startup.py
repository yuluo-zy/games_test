import pathlib,struct,json
p=pathlib.Path('ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes();pe=struct.unpack_from('<I',p,60)[0];sh=pe+24+struct.unpack_from('<H',p,pe+20)[0]
sections=[struct.unpack_from('<4I',p,sh+i*40+8) for i in range(struct.unpack_from('<H',p,pe+6)[0])]
def read(va,n):
 r=va-0x140000000
 for vs,vr,rs,ro in sections:
  if vr<=r<vr+rs:return p[ro+r-vr:ro+r-vr+n]
 raise ValueError(hex(va))
out={}
for a,n in [(0x142ad9a40,72),(0x142ac64c0,32),(0x14293e0c0,16),(0x142ada260,180)]:
 b=read(a,n);out[hex(a)]={'hex':b.hex(),'u32':list(struct.unpack('<'+'I'*(n//4),b[:n//4*4])),'ascii':b.decode('ascii','replace')}
pathlib.Path('reverse-engineering/tiny-glade/evidence/round12/wall-gpu/wall-startup-constants.json').write_text(json.dumps(out,indent=2));print(json.dumps(out,indent=2))
