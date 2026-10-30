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
rows=[];counts={};recent=[];checkpoints=[];wood_sites={};curved_calls=[];status='running';initial=None

def ret(c):
 r=c.reg_read(UC_X86_REG_RSP);c.reg_write(UC_X86_REG_RIP,struct.unpack('<Q',c.mem_read(r,8))[0]);c.reg_write(UC_X86_REG_RSP,r+8)
def hook(c,pc,size,_):
 global heap,status
 recent.append(pc);del recent[:-35]
 if pc in (0x140d4d740,0x141246fb0,0x1412218a0,0x140f86cd0,0x140a1c2c0):
  key={0x140d4d740:'tls_rng_accessor_executed',0x141246fb0:'terrain_sampler_executed',0x1412218a0:'overlap_windows_executed',0x140f86cd0:'holes_no_duplicates_executed',0x140a1c2c0:'decorator_storage_get_executed'}[pc];counts[key]=counts.get(key,0)+1
 if pc==0x14121c505:
  checkpoints.append({'site':'0x14121c505','begin':num(c.reg_read(UC_X86_REG_XMM1)),'end':num(c.reg_read(UC_X86_REG_XMM2))})
 if pc==0x14121cc71:
  ptr=c.reg_read(UC_X86_REG_RAX);n=c.reg_read(UC_X86_REG_RDX)
  vals=[list(struct.unpack('<IIB3x',c.mem_read(ptr+12*i,12))) for i in range(n)] if n<10000 else []
  checkpoints.append({'site':'0x14121cc71','ptr':hex(ptr),'len':n,'interval_words':vals})
 if pc in (0x14098f690,0x14098f800):
  sp0=c.reg_read(UC_X86_REG_RSP);src=c.reg_read(UC_X86_REG_RDX)
  curved_calls.append({'function':hex(pc),'position_words':list(struct.unpack('<3I',c.mem_read(src,12))),'xmm2':c.reg_read(UC_X86_REG_XMM2)&0xffffffff,'xmm3':c.reg_read(UC_X86_REG_XMM3)&0xffffffff,'stack_words':[struct.unpack('<I',c.mem_read(sp0+off,4))[0] for off in (0x28,0x30,0x38,0x40)]})
 if pc==retaddr:status='returned';c.emu_stop();return
 if pc in bridges:
  n=bridges[pc];vals=[num(c.reg_read(UC_X86_REG_XMM0))];
  if n in ('powf','fmodf'):vals.append(num(c.reg_read(UC_X86_REG_XMM1)))
  c.reg_write(UC_X86_REG_XMM0,bits(getattr(lib,n)(*vals)));counts[n]=counts.get(n,0)+1;ret(c)
 elif pc==0x141209c40: c.reg_write(UC_X86_REG_RAX,state);counts['ssbo_lookup']=counts.get('ssbo_lookup',0)+1;ret(c)
 elif pc==0x30000100:c.reg_write(UC_X86_REG_RAX,ssbo);counts['ssbo_downcast']=counts.get('ssbo_downcast',0)+1;ret(c)
 elif pc in (0x140c81c70,0x140c81e10,0x140613c20,0x140d65810,0x14120c9d0):counts[hex(pc)]=counts.get(hex(pc),0)+1;ret(c)
 elif pc==0x14120c430:site=hex(struct.unpack('<Q',c.mem_read(c.reg_read(UC_X86_REG_RSP),8))[0]);wood_sites[site]=wood_sites.get(site,0)+1;rows.append(bytes(c.mem_read(c.reg_read(UC_X86_REG_RDX),64)).hex());counts['wood_push']=counts.get('wood_push',0)+1;c.reg_write(UC_X86_REG_RAX,0);ret(c)
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
# 原 Circle2d::as_curve2 先构建合法闭合多边形及原参数表，不桥业务。
shape=struct.pack('<I4f',0,0.,0.,2.,0.)+bytes(8)
w(0x3002d000,struct.pack('<4f',0.,0.,2.,0.));q(sp,retaddr)
for reg,val in [(UC_X86_REG_RSP,sp),(UC_X86_REG_RCX,0x3002e000),(UC_X86_REG_RDX,0x3002d000),(UC_X86_REG_XMM2,bits(0.9)),(UC_X86_REG_MXCSR,0x1f80)]:u.reg_write(reg,val)
try:u.emu_start(0x140c97cf0,retaddr+1,count=3000000)
except Exception as e:raise RuntimeError('circle curve setup '+str(e)+'@'+hex(u.reg_read(UC_X86_REG_RIP)))
if status!='returned':raise RuntimeError('circle curve setup '+status)
status='running'
raw_curve=bytes(u.mem_read(0x3002e000,56)); cap,ptr,n,capu,ptru,nu,lengthbits,pad=struct.unpack('<6QII',raw_curve)
if not (2<=n==nu<=20):raise RuntimeError('unexpected original circle point count '+str(n))
points=[struct.unpack('<2f',u.mem_read(ptr+8*i,8)) for i in range(n)];us=list(struct.unpack('<'+'f'*n,u.mem_read(ptru,4*n)));length=num(lengthbits)
w(wall+0x258,shape)
w(source,b''.join(struct.pack('<2f',*p) for p in points));w(source+0x100,struct.pack('<'+'f'*n,*us));w(source+0x200,b''.join(struct.pack('<3f',x,0.,z) for x,z in points));w(source+0x300,struct.pack('<'+'f'*n,*us));w(source+0x400,struct.pack('<'+'f'*n,*([2.5]*n)));w(source+0x500,struct.pack('<'+'f'*n,*([0.]*n)));w(source+0x600,struct.pack('<'+'f'*n,*([0.]*n)));w(source+0x700,struct.pack('<'+'f'*n,*([0.]*n)));w(source+0x800,b''.join(struct.pack('<3f',x,0.,z) for x,z in points));w(source+0x900,struct.pack('<'+'f'*n,*us))
for off,val in [(0,n),(8,source),(0x10,n),(0x18,n),(0x20,source+0x100),(0x28,n),(0x38,n),(0x40,source+0x800),(0x48,n),(0x50,n),(0x58,source+0x900),(0x60,n),(0x88,n),(0x90,source+0x200),(0x98,n),(0xa0,n),(0xa8,source+0x300),(0xb0,n),(0xc0,n),(0xc8,source+0x600),(0xd0,n),(0xd8,n),(0xe0,source+0x400),(0xe8,n),(0xf0,n),(0xf8,source+0x500),(0x100,n),(0x180,owner)]:q(wall+off,val)
for off,val in [(0x30,length),(0x68,length),(0xb8,length),(0x10c,2.5),(0x114,2.5),(0x280,0.),(0x284,2.5),(0x288,2.5),(0x28c,length)]:f(wall+off,val)
w(wall+0x108,struct.pack('<I',1));w(wall+0x110,struct.pack('<I',1));w(roof+8,shape);f(roof+0x2c,0.5);f(roof+0x30,0.0);f(roof+0x34,1.);f(roof+0x4c,2.5)
q(hole,0);q(hole+8,4);q(hole+16,0);q(deco+0x18,0);q(terrain+8,grid);q(terrain+0x10,512*512);f(terrain+0x50,0.);f(terrain+0x54,0.)
q(owner+8,0x3001b000);q(owner+0x10,0);q(ssbo+0x28,1);q(state,0x30000100);q(state+8,vt);q(vt+0x18,0x30000100);q(state+0x10,0x142c1a968);q(state+0x18,0x3d)
q(0x3001f000+0x58,0x3001f400);q(0x3001f400,0x3001f800);w(base+0x363acb0,struct.pack("<I",0));w(0x3001f800+0x1e8,struct.pack("<QQQ",1,0x123456789abcdef0,0xfedcba9876543210));q(base+0x362c098,0x3001c000);q(base+0x362c308,0x3001c008);q(curved,0);q(curved+8,4);q(curved+16,0)
q(sp,retaddr)
for off,val in [(0x28,terrain),(0x30,0x3001d000),(0x38,source+0x500),(0x40,n),(0x48,source+0x700),(0x50,n),(0x58,0x3001e000),(0x60,roof),(0x68,hole),(0x70,curved)]:q(sp+off,val)
for reg,val in [(UC_X86_REG_RSP,sp),(UC_X86_REG_RCX,wall),(UC_X86_REG_RDX,123),(UC_X86_REG_R8,deco),(UC_X86_REG_R9,derived),(UC_X86_REG_MXCSR,0x1f80),(UC_X86_REG_GS_BASE,0x3001f000)]:u.reg_write(reg,val)
initial={"public_wall_672":bytes(u.mem_read(wall,672)).hex(),"roof_88":bytes(u.mem_read(roof,88)).hex(),"finalized_holes_24":bytes(u.mem_read(hole,24)).hex(),"decorator_storage_256":bytes(u.mem_read(deco,256)).hex(),"derived_view_16":bytes(u.mem_read(derived,16)).hex(),"terrain_header_88":bytes(u.mem_read(terrain,88)).hex(),"source_2560":bytes(u.mem_read(source,0xa00)).hex(),"initial_stack_120":bytes(u.mem_read(sp,0x78)).hex(),"allocation_owner_32":bytes(u.mem_read(owner,32)).hex(),"ssbo_storage_96":bytes(u.mem_read(ssbo,96)).hex(),"ssbo_fake_state_32":bytes(u.mem_read(state,32)).hex(),"ssbo_fake_vtable_32":bytes(u.mem_read(vt,32)).hex(),"curved_vec_24":bytes(u.mem_read(curved,24)).hex(),"terrain_grid_sha256":hashlib.sha256(bytes(u.mem_read(grid,512*512*2))).hexdigest()}
try:u.emu_start(0x141218e30,retaddr+1,count=30000000)
except Exception as e:status='fault:'+str(e)+'@'+hex(u.reg_read(UC_X86_REG_RIP))
if status=='running':status='instruction_budget@'+hex(u.reg_read(UC_X86_REG_RIP))
cap,ptr,length=struct.unpack('<QQQ',bytes(u.mem_read(curved,24)));curvebytes=''
if length and length<=10000:
 try:curvebytes=bytes(u.mem_read(ptr,length*64)).hex()
 except Exception as e:curvebytes='unreadable:'+str(e)
result={'status':status,'rne_sha256':sha,'pdb_sha256':'2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911','entry':'0x141218e30','failure_history':['initial VCRUNTIME memmove import unresolved at 142923c10; bridged standard memmove','Windows HeapAlloc import unresolved at 140d75fc1; bridged dependency allocator','Windows GS TLS absent at 140d4d74a; installed explicit initial TLS state and executed original RNG path'],'input':{'wall_circle':list(struct.unpack('<I4f',shape[:20])),'original_circle_curve_struct':raw_curve.hex(),'curve_length':num(lengthbits),'preparation':'original Circle2d::as_curve2(0.9)','curve2_points':points,'curve2_u':us,'prepared_xyz':[[x,0.,z] for x,z in points],'arch':[2.5]*n,'third':[0.]*n,'y':[0.]*n,'roof_flat':(struct.unpack('<f',bytes(u.mem_read(roof+0x30,4)))[0] <= 0.1),'roof_height':struct.unpack('<f',bytes(u.mem_read(roof+0x30,4)))[0],'roof_flag_0x3d':int(u.mem_read(roof+0x3d,1)[0]),'roof_profile':0.5,'roof_floor':2.5,'terrain_u16_all':0,'wall_id':123,'demo':False,'tls_fastrand_initial':[1,'0x123456789abcdef0','0xfedcba9876543210'],'decorator_storage_empty':True,'finalized_holes_empty':True,'stack':{hex(k):hex(v) for k,v in [(0x28,terrain),(0x30,0x3001d000),(0x38,source+0x500),(0x40,n),(0x48,source+0x700),(0x50,n),(0x58,0x3001e000),(0x60,roof),(0x68,hole),(0x70,curved)]}},'input_objects_hex':initial,'ssbo_exhausted_flag':int(u.mem_read(sp-0x6b8+0x4bf,1)[0]),'tls_rng_final_hex':bytes(u.mem_read(0x3001f800+0x1e8,24)).hex(),'bridges_and_observations':counts,'wood_push_return_sites':wood_sites,'checkpoints':checkpoints,'curved_constructor_calls':curved_calls,'wooden_64b':rows,'curved_header':[cap,hex(ptr),length],'curved_bytes':curvebytes,'recent':[hex(x) for x in recent]}
path=Path('/private/tmp/m06-half-main-circle-checkpoint.json');path.write_text(json.dumps(result,indent=2)+'\n');print(status,'wood',len(rows),'curved',length,'bridges',counts,'recent',list(map(hex,recent[-12:])))

if status!='returned':sys.exit(1)
