import sys,struct,json,hashlib
from pathlib import Path
sys.path.insert(0,'D:/game/tiny-glade/work/m01-scratch/python-deps')
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import *
p=Path('D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe');raw=p.read_bytes();pe=struct.unpack_from('<I',raw,60)[0];opt=pe+24;base=struct.unpack_from('<Q',raw,opt+24)[0];size=struct.unpack_from('<I',raw,opt+56)[0];n=struct.unpack_from('<H',raw,pe+6)[0];off=opt+struct.unpack_from('<H',raw,pe+20)[0]; image=bytearray(size); hs=struct.unpack_from('<I',raw,opt+60)[0];image[:hs]=raw[:hs]
for i in range(n):
 vs,va,rs,ro=struct.unpack_from('<IIII',raw,off+40*i+8);image[va:va+rs]=raw[ro:ro+rs]
def vec(*v): return int.from_bytes(struct.pack('<4f',*v),'little')
def go(point,half,newsize,delta):
 u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(size+4095)&~4095);u.mem_write(base,bytes(image));u.mem_map(0x30000000,0x10000);rsp=0x30008000;u.reg_write(UC_X86_REG_RSP,rsp);u.mem_write(rsp+0x70,struct.pack('<2f',*newsize));
 for reg,val in [(UC_X86_REG_XMM7,vec(0,0,0,0)),(UC_X86_REG_XMM8,vec(*half,0,0)),(UC_X86_REG_XMM9,vec(1,0,0,1)),(UC_X86_REG_XMM10,vec(1,0,0,1)),(UC_X86_REG_XMM11,vec(0,0,0,0)),(UC_X86_REG_XMM12,vec(*delta,0,0)),(UC_X86_REG_XMM13,vec(delta[1],delta[1],0,0)),(UC_X86_REG_XMM14,vec(point[1],0,0,0)),(UC_X86_REG_XMM15,vec(point[0],0,0,0))]:u.reg_write(reg,val)
 u.emu_start(0x14081a174,0x14081a230);return [f'{x:08x}' for x in struct.unpack('<4I',u.reg_read(UC_X86_REG_XMM0).to_bytes(16,'little'))[:2]]
cases=[('x-dominant',(2.,1.),(2.,2.),(8.,6.),(1.,2.)),('y-dominant',(1.,2.),(2.,2.),(8.,6.),(1.,2.)),('tie',(1.,1.),(2.,2.),(8.,6.),(1.,2.)),('zero-size',(0.,1.),(0.,2.),(8.,6.),(1.,2.)),('nan',(float('nan'),1.),(2.,2.),(8.,6.),(1.,2.)),('signed-zero',(-0.,0.),(2.,2.),(8.,6.),(0.,0.))]
out={'scope':'Unicorn executes original PE instructions 14081a174..14081a22f; supplied intermediate XMM registers, identity orientation, not full consumer','pe_sha256':hashlib.sha256(raw).hexdigest(),'cases':[{'case':name,'point':str(pt),'half':str(h),'new_size':str(sz),'delta':str(d),'output_xy_bits':go(pt,h,sz,d)} for name,pt,h,sz,d in cases]};print(json.dumps(out,indent=2));Path('D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/wall/m01-rectangle-consumer-fragment-fixture.json').write_text(json.dumps(out,indent=2))
