import hashlib, json, struct
from pathlib import Path
import pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import *
raw = Path('B版本内容/tiny-glade.rne').read_bytes()
sha = hashlib.sha256(raw).hexdigest()
assert sha == 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p = pefile.PE(data=raw, fast_load=True)
u = Uc(UC_ARCH_X86, UC_MODE_64)
base = p.OPTIONAL_HEADER.ImageBase
u.mem_map(base, (p.OPTIONAL_HEADER.SizeOfImage+4095)&~4095)
u.mem_write(base, p.get_memory_mapped_image())
u.mem_map(0x30000000, 0x20000)
sp=0x30010008
status='unknown'
phase='height'
def ret(c):
 r=c.reg_read(UC_X86_REG_RSP); c.reg_write(UC_X86_REG_RIP, struct.unpack('<Q', c.mem_read(r,8))[0]); c.reg_write(UC_X86_REG_RSP,r+8)
def hook(c,pc,size,_):
 global status
 if pc==0x140c16a00: c.reg_write(UC_X86_REG_RAX,0); ret(c)
 elif phase=='height' and pc==0x1412497ee: status='entered'; c.emu_stop()
 elif phase=='containment' and pc in (0x141249d4a,0x1412498df): status='accepted'; c.emu_stop()
 elif pc==0x30000000: status='skipped'; c.emu_stop()
u.hook_add(UC_HOOK_CODE,hook)
def fbits(x): return struct.unpack('<I',struct.pack('<f',x))[0]
vals=[0,fbits(0.6),fbits(1),fbits(3),fbits(5),0x7f800000,0xff800000,0x7fc12345]
rows=[]
for source,target,maximum in [(s,t,m) for s,t,m in [(fbits(1),fbits(3),fbits(5)),(fbits(1),fbits(1.6),fbits(5)),(fbits(1),fbits(3),fbits(3.6))]+[(fbits(1),x,fbits(5)) for x in vals]+[(fbits(1),fbits(1.6)+d,fbits(5)) for d in (-1,0,1)]+[(fbits(1),fbits(3),fbits(3.6)+d) for d in (-1,0,1)]+[(x,fbits(3),fbits(5)) for x in vals]+[(fbits(1),fbits(3),x) for x in vals]]:
 u.mem_write(0x30001000, bytes(0x6000));u.mem_write(0x30002000,b'\xff'*16)
 u.mem_write(0x30003000,struct.pack('<QQQQ',0x30002000,0,0,0))
 # cache has empty roofs and one (wall id, &PublicWallState) entry.
 u.mem_write(0x30004000,struct.pack('<6Q',0,0x30005000,0,1,0x30005000,1))
 u.mem_write(0x30005000,struct.pack('<QQ',2,0x30006000))
 u.mem_write(0x30006000+0x108,struct.pack('<II',1,maximum))
 u.mem_write(0x30006000+0x280,struct.pack('<I',source))
 u.mem_write(0x30007000+0x280,struct.pack('<I',target))
 u.mem_write(0x30008000,struct.pack('<I6f',0,0.,0.,2.,0.,0.,0.))
 u.mem_write(sp,struct.pack('<Q',0x30000000));u.mem_write(sp+0x28,struct.pack('<Q',0x30009000));u.mem_write(sp+0x30,struct.pack('<Q',0x30003000));u.mem_write(sp+0x38,struct.pack('<Q',0x30004000));u.mem_write(sp+0x40,struct.pack('<Q',0));u.mem_write(sp+0x48,struct.pack('<Q',0))
 for reg,val in [(UC_X86_REG_RSP,sp),(UC_X86_REG_RCX,0x3000a000),(UC_X86_REG_RDX,1),(UC_X86_REG_R8,0x30007000),(UC_X86_REG_R9,0x30008000),(UC_X86_REG_MXCSR,0x1f80)]: u.reg_write(reg,val)
 status='unknown'
 try: u.emu_start(0x141248620,0x30000001,count=200000)
 except Exception as e: status='fault:'+str(e)+'@'+hex(u.reg_read(UC_X86_REG_RIP))
 rows.append({'source_bits':source,'target_bits':target,'max_bits':maximum,'status':status})
phase='containment'
geometry=[]
# shape values: circle center/radius/rotation or rectangle axis/center/size.
cases=[
 (0,[0.,0.,5.,0.,0.,0.],0,[0.,0.,2.,0.,0.,0.]),
 (0,[0.,0.,2.,0.,0.,0.],0,[0.,0.,2.,0.,0.,0.]),
 (0,[0.,0.,5.,0.,0.,0.],0,[6.,0.,1.,0.,0.,0.]),
 (1,[1.,0.,0.,0.,10.,10.],0,[0.,0.,2.,0.,0.,0.]),
 (1,[1.,0.,0.,0.,4.,4.],0,[0.,0.,2.,0.,0.,0.]),
 (0,[0.,0.,5.,0.,0.,0.],1,[1.,0.,0.,0.,2.,2.]),
 (0,[0.,0.,1.,0.,0.,0.],1,[1.,0.,0.,0.,2.,2.]),
 (1,[1.,0.,0.,0.,10.,10.],1,[1.,0.,0.,0.,2.,2.]),
 (1,[1.,0.,0.,0.,2.,2.],1,[1.,0.,5.,0.,2.,2.]),
]
for target_tag,target_shape,source_tag,source_shape in cases:
 u.mem_write(0x30006000+0x108,struct.pack('<II',1,fbits(5.)))
 u.mem_write(0x30006000+0x280,struct.pack('<I',fbits(1.)))
 u.mem_write(0x30007000+0x280,struct.pack('<I',fbits(3.)))
 u.mem_write(0x30006000+0x258,struct.pack('<I6f',source_tag,*source_shape))
 u.mem_write(0x30008000,struct.pack('<I6f',target_tag,*target_shape))
 u.mem_write(sp,struct.pack('<Q',0x30000000))
 for reg,val in [(UC_X86_REG_RSP,sp),(UC_X86_REG_RCX,0x3000a000),(UC_X86_REG_RDX,1),(UC_X86_REG_R8,0x30007000),(UC_X86_REG_R9,0x30008000),(UC_X86_REG_MXCSR,0x1f80)]: u.reg_write(reg,val)
 status='unknown'
 try: u.emu_start(0x141248620,0x30000001,count=200000)
 except Exception as e: status='fault:'+str(e)+'@'+hex(u.reg_read(UC_X86_REG_RIP))
 geometry.append({'target_tag':target_tag,'target_shape':target_shape,'source_tag':source_tag,'source_shape':source_shape,'status':status})
Path('tiny-glade/reports/module-recovery-2026-10-23/m06-finder-height-oracle.json').write_text(json.dumps({'sha256':sha,'scope':'original finder containment height gate, stop at 1412497ee before shape; puffin disabled; original max_y executed','rows':rows,'geometry':geometry},indent=2)+'\n')
print('geometry',len(geometry),[x['status'] for x in geometry]);print('rows',len(rows),'statuses', {x:sum(r['status']==x for r in rows) for x in set(r['status'] for r in rows)})
