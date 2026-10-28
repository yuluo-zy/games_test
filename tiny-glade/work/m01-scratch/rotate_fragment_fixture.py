import sys,struct,json,hashlib
from pathlib import Path
sys.path.insert(0,'D:/game/tiny-glade/work/m01-scratch/python-deps')
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import *
p=Path('D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe');raw=p.read_bytes();pe=struct.unpack_from('<I',raw,60)[0];opt=pe+24;base=struct.unpack_from('<Q',raw,opt+24)[0];size=struct.unpack_from('<I',raw,opt+56)[0];n=struct.unpack_from('<H',raw,pe+6)[0];off=opt+struct.unpack_from('<H',raw,pe+20)[0];image=bytearray(size);hs=struct.unpack_from('<I',raw,opt+60)[0];image[:hs]=raw[:hs]
for i in range(n):
 vs,va,rs,ro=struct.unpack_from('<IIII',raw,off+40*i+8);image[va:va+rs]=raw[ro:ro+rs]
def vec(*v):return int.from_bytes(struct.pack('<4f',*v),'little')
def go(a,b):
 u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(size+4095)&~4095);u.mem_write(base,bytes(image));u.reg_write(UC_X86_REG_XMM0,vec(a[0],b[0],0,0));u.reg_write(UC_X86_REG_XMM3,vec(a[0],b[0],0,0));u.reg_write(UC_X86_REG_XMM2,vec(a[1],b[1],0,0));u.emu_start(0x140ad9211,0x140ad933e);return f'{u.reg_read(UC_X86_REG_XMM2)&0xffffffff:08x}'
cases=[('same',(1.,0.),(1.,0.)),('positive',(1.,0.),(0.,1.)),('negative',(1.,0.),(0.,-1.)),('negative-dot',(1.,0.),(-1.,1.)),('opposite',(1.,0.),(-1.,0.)),('zero-vector',(0.,0.),(1.,0.)),('nan',(float('nan'),0.),(1.,0.)),('negative-zero',(1.,-0.),(1.,-0.))]
out={'scope':'Unicorn original PE instructions 140ad9211..140ad933d, input packed vector registers, not complete consumer','pe_sha256':hashlib.sha256(raw).hexdigest(),'cases':[{'name':n,'a':str(a),'b':str(b),'angle_bits':go(a,b)} for n,a,b in cases]};print(json.dumps(out,indent=2));Path('D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/wall/m01-rotate-fast-acos-fragment-fixture.json').write_text(json.dumps(out,indent=2))
