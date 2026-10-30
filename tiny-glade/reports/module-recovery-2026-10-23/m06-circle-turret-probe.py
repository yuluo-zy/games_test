import hashlib,json,struct,ctypes,sys
from pathlib import Path
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
root=Path('tiny-glade/reports/module-recovery-2026-10-23')
raw=Path('B版本内容/tiny-glade.rne').read_bytes(); sha=hashlib.sha256(raw).hexdigest(); assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True); base=p.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64); u.mem_map(base,(p.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);u.mem_write(base,p.get_memory_mapped_image());u.mem_map(0x30000000,0x200000);u.mem_map(0x40000000,0x4000000)
sp=0x30100008;retaddr=0x30000000;wall=0x30010000;roof=0x30011000;hole=0x30012000;deco=0x30013000;derived=0x30014000;terrain=0x30015000;owner=0x30016000;ssbo=0x30017000;state=0x30018000;vt=0x30019000;curved=0x3001a000;source=0x30020000;grid=0x30040000;heap=0x40000000
w=lambda a,b:u.mem_write(a,b); q=lambda a,x:w(a,struct.pack('<Q',x)); f=lambda a,x:w(a,struct.pack('<f',x));
bits=lambda v:struct.unpack('<I',struct.pack('<f',v))[0]; num=lambda v:struct.unpack('<f',struct.pack('<I',v&0xffffffff))[0]
lib=ctypes.CDLL('/usr/lib/libSystem.B.dylib')
for n,k in [('powf',2),('fmodf',2),('roundf',1),('floorf',1),('ceilf',1),('sinf',1),('cosf',1),('expf',1)]:
 x=getattr(lib,n);x.argtypes=[ctypes.c_float]*k;x.restype=ctypes.c_float
bridges={0x142923f70:'powf',0x142923f10:'fmodf',0x142923fa0:'roundf',0x142923ef0:'floorf',0x142923e60:'ceilf',0x142923fd0:'sinf',0x142923e80:'cosf',0x142923ec0:'expf'}
rows=[];counts={};recent=[];status='running';initial=None

def ret(c):
 r=c.reg_read(UC_X86_REG_RSP);c.reg_write(UC_X86_REG_RIP,struct.unpack('<Q',c.mem_read(r,8))[0]);c.reg_write(UC_X86_REG_RSP,r+8)
def hook(c,pc,size,_):
 global heap,status
 recent.append(pc);del recent[:-35]
 if pc in (0x140d4d740,0x141246fb0,0x1412218a0,0x140f86cd0,0x140a1c2c0):
  key={0x140d4d740:'tls_rng_accessor_executed',0x141246fb0:'terrain_sampler_executed',0x1412218a0:'overlap_windows_executed',0x140f86cd0:'holes_no_duplicates_executed',0x140a1c2c0:'decorator_storage_get_executed'}[pc];counts[key]=counts.get(key,0)+1
 if pc==retaddr:status='returned';c.emu_stop();return
 if pc in bridges:
  n=bridges[pc];vals=[num(c.reg_read(UC_X86_REG_XMM0))];
  if n in ('powf','fmodf'):vals.append(num(c.reg_read(UC_X86_REG_XMM1)))
  c.reg_write(UC_X86_REG_XMM0,bits(getattr(lib,n)(*vals)));counts[n]=counts.get(n,0)+1;ret(c)
 elif pc==0x140c16a00:c.reg_write(UC_X86_REG_RAX,0);ret(c)
 elif pc==0x140a91ef0:c.reg_write(UC_X86_REG_RAX,0x30030000);counts['public_lookup_bridge']=counts.get('public_lookup_bridge',0)+1;ret(c)
 elif pc==0x14120c120:rows.append(bytes(c.mem_read(c.reg_read(UC_X86_REG_RDX),96)).hex());counts['brick_push']=counts.get('brick_push',0)+1;c.reg_write(UC_X86_REG_RAX,0);ret(c)
 elif pc==0x141209c40: c.reg_write(UC_X86_REG_RAX,state);counts['ssbo_lookup']=counts.get('ssbo_lookup',0)+1;ret(c)
 elif pc==0x30000100:c.reg_write(UC_X86_REG_RAX,ssbo);counts['ssbo_downcast']=counts.get('ssbo_downcast',0)+1;ret(c)
 elif pc in (0x140c81c70,0x140c81e10,0x140613c20,0x140d65810,0x14120c9d0):counts[hex(pc)]=counts.get(hex(pc),0)+1;ret(c)
 elif pc==0x14120c430:rows.append(bytes(c.mem_read(c.reg_read(UC_X86_REG_RDX),64)).hex());counts['wood_push']=counts.get('wood_push',0)+1;c.reg_write(UC_X86_REG_RAX,0);ret(c)
 elif pc in (0x140613c10,0x140d657b0):
  n=c.reg_read(UC_X86_REG_RCX);a=c.reg_read(UC_X86_REG_RDX);heap=(heap+a-1)&-a;c.reg_write(UC_X86_REG_RAX,heap);heap+=n;counts['alloc']=counts.get('alloc',0)+1;ret(c)
 elif pc==0x140d75fb0:
  n=c.reg_read(UC_X86_REG_R8);heap=(heap+15)&-16;c.reg_write(UC_X86_REG_RAX,heap);heap+=n;counts['win_heap_alloc']=counts.get('win_heap_alloc',0)+1;ret(c)
 elif pc==0x140d65850:
  old=c.reg_read(UC_X86_REG_RCX);oldsize=c.reg_read(UC_X86_REG_RDX);a=c.reg_read(UC_X86_REG_R8);n=c.reg_read(UC_X86_REG_R9);heap=(heap+a-1)&-a;c.mem_write(heap,bytes(c.mem_read(old,min(n,oldsize))));c.reg_write(UC_X86_REG_RAX,heap);heap+=n;counts['realloc']=counts.get('realloc',0)+1;ret(c)
 elif pc in (0x142923c00,0x142923c10):
  d=c.reg_read(UC_X86_REG_RCX);s=c.reg_read(UC_X86_REG_RDX);n=c.reg_read(UC_X86_REG_R8);c.mem_write(d,bytes(c.mem_read(s,n)));c.reg_write(UC_X86_REG_RAX,d);counts['memcpy_or_memmove']=counts.get('memcpy_or_memmove',0)+1;ret(c)
 elif pc==0x142923bf0:
  a=c.reg_read(UC_X86_REG_RCX);b=c.reg_read(UC_X86_REG_RDX);n=c.reg_read(UC_X86_REG_R8);x=bytes(c.mem_read(a,n));y=bytes(c.mem_read(b,n));c.reg_write(UC_X86_REG_RAX,0 if x==y else (1 if x>y else 0xffffffff));counts['memcmp']=counts.get('memcmp',0)+1;ret(c)
 elif pc==0x142923c20:
  d=c.reg_read(UC_X86_REG_RCX);v=c.reg_read(UC_X86_REG_RDX)&255;n=c.reg_read(UC_X86_REG_R8);c.mem_write(d,bytes([v])*n);c.reg_write(UC_X86_REG_RAX,d);counts['memset']=counts.get('memset',0)+1;ret(c)
u.hook_add(UC_HOOK_CODE,hook)
# 原圆塔入口；邻墙查找只桥一条显式记录，用于限定原几何路径。
case=int(sys.argv[1]) if len(sys.argv)>1 else 0
w(wall+0x280,struct.pack('<f',4.));w(wall+0x299,bytes([2 if case else 0]));q(wall+0x150,owner)
w(roof,struct.pack('<4f',0.,0.,2.5,0.));w(0x30030000+0x258,struct.pack('<I6f',1,1.,0.,0.,0.,4.,2.))
segment=0x30020000;segvec=0x30021000
# OverhangSegment: curve Vec2/参数、长度、WallId和flag；flag1保留圆心。
q(segment,2);q(segment+8,0x30022000);q(segment+16,2);q(segment+24,2);q(segment+32,0x30022100);q(segment+40,2);f(segment+48,4.);q(segment+56,456);f(segment+64,0.);f(segment+68,1.);w(segment+72,b'\x01')
w(0x30022000,struct.pack('<4f',0.,0.,4.,0.));w(0x30022100,struct.pack('<2f',0.,1.));q(segvec,1);q(segvec+8,segment);q(segvec+16,1)
q(owner+8,0x3001b000);q(owner+0x10,0);q(ssbo+0x28,1);q(state,0x30000100);q(state+8,vt);q(vt+0x18,0x30000100);q(state+0x10,0x142c1a9a5);q(state+0x18,0x3a)
# EventWriter 的真实 ResMut 指针和空 events_b 容器，不桥事件写入。
eventwriter=0x30024000;events=0x30025000;tick=0x30026000;q(eventwriter,events);q(eventwriter+16,tick);w(eventwriter+28,struct.pack('<I',9));q(events+0x20,4);q(events+0x28,0x30027000);q(events+0x30,0);q(events+0x40,0)
q(sp,retaddr)
for off,val in [(0x28,0x3001d000),(0x30,segvec),(0x38,0x3001e000),(0x40,eventwriter)]:q(sp+off,val)
for reg,val in [(UC_X86_REG_RSP,sp),(UC_X86_REG_RCX,123),(UC_X86_REG_RDX,roof),(UC_X86_REG_R8,wall),(UC_X86_REG_R9,0x30031000),(UC_X86_REG_MXCSR,0x1f80)]:u.reg_write(reg,val)
try:u.emu_start(0x14124cc30,retaddr+1,count=30000000)
except Exception as e:status='fault:'+str(e)+'@'+hex(u.reg_read(UC_X86_REG_RIP))
if status=='running':status='instruction_budget@'+hex(u.reg_read(UC_X86_REG_RIP))
length=struct.unpack('<Q',u.mem_read(events+0x30,8))[0];cur=[]
if length:
 event=bytes(u.mem_read(0x30027000,40));cap,ptr,n,wallid,eid=struct.unpack('<5Q',event)
 if n<10000:cur=[bytes(u.mem_read(ptr+64*i,64)).hex() for i in range(n)]
result={'status':status,'case':case,'rne_sha256':sha,'input':{'wall_id':123,'circle':[0.,0.,2.5,0.],'elevation':4.,'style':2 if case else 0,'neighbor_id':456,'flag':1,'neighbor_shape':'rectangle at origin'},'bridges':counts,'brick_96b':rows,'events':length,'curved_64b':cur,'recent':list(map(hex,recent)),'scope':'原入口静态样本；PublicWalls lookup桥接，math/allocator/SSBO桥接；非真实World完成验收'}
Path(f'/private/tmp/m06-circle-turret-probe-{case}.json').write_text(json.dumps(result,indent=2)+'\n');print(status,'bricks',len(rows),'events',length,'curved',len(cur),'counts',counts,'recent',list(map(hex,recent[-10:])))
if status!='returned':sys.exit(1)
