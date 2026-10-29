"""完整原墙装饰拆分picker：真实FP、rank和投影；仅桥接容器/UI工厂依赖。"""
from pathlib import Path
import struct,json,itertools
source=Path(__file__).with_name('m01_dispatch_start_place_native.py').read_text()
exec(compile(source[:source.index('\nrows=[]')],'identity-bound-pe-setup','exec'))
alloc_cursor=D+0x40000
def allocate(n):
 global alloc_cursor
 out=alloc_cursor;alloc_cursor=(alloc_cursor+max(n,16)+15)&~15;assert alloc_cursor<D+0xf0000;return out
def display_hook(uc,a,size,_):
 rcx,rdx,r8,r9=[r(x)for x in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_R9]]
 if a==0x140833ba0:ret(7)
 elif a==0x14125cf90:
  key=bytes(u.mem_read(rdx,2));assert key==bytes([7,rank]);ret(D+0x20000)
 elif a==0x140d79f90:
  p=allocate(1);u.mem_write(p,b'x');q(rcx,1);q(rcx+8,p);q(rcx+16,1);ret(rcx)
 elif a==0x14089d8b0:
  captures.append({'position_bits':list(struct.unpack('<3I',u.mem_read(r9+16,12))),'vtable':hex(uq(r(UC_X86_REG_RSP)+0x28))})
  for off,val in [(0,r8),(8,rdx),(16,r8),(24,r9),(32,uq(r(UC_X86_REG_RSP)+0x28))]:q(rcx+off,val)
  ret(rcx)
 elif a==0x140b2cf90:ret()
 elif a in [0x140613c10,0x1428d9000]:ret(allocate(rcx))
 elif a==0x140613c20:ret()
 elif a in [0x1428d9310,0x1428d9390,0x1428d9430]:raise RuntimeError('original panic '+hex(a))
u.hook_add(UC_HOOK_CODE,display_hook)
rows=[]
poses=[([0.,0.,0.,1.],[2.,3.,4.],[1.,1.,1.]),([.2,-.3,.1,.92736185],[-3.5,1.25,12.],[1.3,.7,2.]),([0.,.6,0.,.8],[1000.,-3.,4096.],[.5,2.,1.])]
for ty,rank,basepose,visualpose,point in itertools.product([56,58],range(2,6),poses,poses,[[-3.,0.,4.],[2.,3.,4.],[1000.,-3.,4096.],[5.,4.,-2.]]):
 alloc_cursor=D+0x40000;u.mem_write(D,bytes(0x100000));u.mem_write(S,bytes([0xa5])*0x100000)
 cached,hit,info,visual,view=D+0x1000,D+0x2000,D+0x3000,D+0x4000,D+0x5000
 for ptr,pose in [(cached,basepose),(visual,visualpose)]:u.mem_write(ptr,struct.pack('<10f',*pose[0],*pose[1],*pose[2]))
 u.mem_write(hit+24,struct.pack('<3f',*point));u.mem_write(info+0x44,bytes([rank,5,ty,1]))
 q(view,D+0x8000);q(view+8,D+0x9000);u.mem_write(D+0x20030,struct.pack('<4f',rank*1.3+rank*rank*.07,1.,-.2,rank*1.7+rank*rank*.11))
 sp=S+0xf0008;q(sp,STOP);q(sp+0x28,view);q(sp+0x30,42);q(sp+0x38,D+0xa000)
 for reg,value in [(UC_X86_REG_RCX,cached),(UC_X86_REG_RDX,hit),(UC_X86_REG_R8,info),(UC_X86_REG_R9,visual),(UC_X86_REG_RSP,sp)]:u.reg_write(reg,value)
 captures=[];u.emu_start(0x1412782b0,STOP,count=100000);assert r(UC_X86_REG_RIP)==STOP
 assert len(captures)==rank+1,(rank,captures)
 rows.append({'type':ty,'rank':rank,'cached_bits':list(struct.unpack('<10I',u.mem_read(cached,40))),'visual_bits':list(struct.unpack('<10I',u.mem_read(visual,40))),'point_bits':list(struct.unpack('<3I',u.mem_read(hit+24,12))),'selected':r(UC_X86_REG_RAX),'signifiers':captures})
Path(__file__).with_name('m01-split-display-native.json').write_text(json.dumps({'binary_sha256':sha,'entry':'0x1412782b0','bridges':['derived subtype and collision HashMap lookup','std format/allocation/deallocation','SignifierFactory constructor/stream write; captured payload position and original vtable'],'rows':rows},indent=2)+'\n')
lines=['# ty rank cached10_bits visual10_bits point3_bits selected signifier_positions3bits_semicolon']
for row in rows:lines.append('\t'.join(map(str,[row['type'],row['rank'],','.join(map(str,row['cached_bits'])),','.join(map(str,row['visual_bits'])),','.join(map(str,row['point_bits'])),row['selected'],';'.join(','.join(map(str,s['position_bits']))for s in row['signifiers'])])))
Path(__file__).with_name('m01-split-display-native.tsv').write_text('\n'.join(lines)+'\n')
print(json.dumps({'cases':len(rows),'captured_factories':sum(len(r['signifiers'])for r in rows),'vtables':sorted(set(s['vtable']for r in rows for s in r['signifiers']))}))
