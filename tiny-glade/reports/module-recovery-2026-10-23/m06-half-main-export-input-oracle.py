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
rows=[];counts={};recent=[];checkpoints=[];wood_sites={};status='running';initial=None

def ret(c):
 r=c.reg_read(UC_X86_REG_RSP);c.reg_write(UC_X86_REG_RIP,struct.unpack('<Q',c.mem_read(r,8))[0]);c.reg_write(UC_X86_REG_RSP,r+8)
def hook(c,pc,size,_):
 global heap,status
 recent.append(pc);del recent[:-35]
 if pc in (0x140d4d740,0x141246fb0,0x1412218a0,0x140f86cd0,0x140a1c2c0):
  key={0x140d4d740:'tls_rng_accessor_executed',0x141246fb0:'terrain_sampler_executed',0x1412218a0:'overlap_windows_executed',0x140f86cd0:'holes_no_duplicates_executed',0x140a1c2c0:'decorator_storage_get_executed'}[pc];counts[key]=counts.get(key,0)+1
 if pc==0x141218f08:
  rb=c.reg_read(UC_X86_REG_RBP);checkpoints.append({'site':'0x141218f08','gable_option_words':list(struct.unpack('<3Q',c.mem_read(rb+0x208,24)))})
 if pc==0x14121b9f0:
  rb=c.reg_read(UC_X86_REG_RBP);cap,ptr,n=struct.unpack('<3Q',c.mem_read(rb+0x3b0,24));vals=list(struct.unpack('<'+'I'*n,c.mem_read(ptr,4*n))) if n<10000 else [];checkpoints.append({'site':'0x14121b9f0','vec':[cap,hex(ptr),n],'words':vals})
 if pc==0x14121c326:
  rb=c.reg_read(UC_X86_REG_RBP);ptr=struct.unpack('<Q',c.mem_read(rb+0x480,8))[0];n=c.reg_read(UC_X86_REG_RDX);vals=list(struct.unpack('<'+'I'*n,c.mem_read(ptr,4*n))) if n<10000 else [];checkpoints.append({'site':'0x14121c326','ptr':hex(ptr),'len':n,'words':vals})
 if pc==0x14121c505:checkpoints.append({'site':'0x14121c505','begin':num(c.reg_read(UC_X86_REG_XMM1)),'end':num(c.reg_read(UC_X86_REG_XMM2))})
 if pc==0x14121cc71:
  ptr=c.reg_read(UC_X86_REG_RAX);n=c.reg_read(UC_X86_REG_RDX);vals=[list(struct.unpack('<IIB3x',c.mem_read(ptr+12*i,12))) for i in range(n)] if n<10000 else [];checkpoints.append({'site':'0x14121cc71','len':n,'interval_words':vals})
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
# Original PWS ABI projected from an explicit production export. All f32 words remain raw bits.
input_path=Path(sys.argv[1]); doc=json.loads(input_path.read_text()); data=doc['pws']; wallid=int(doc['wall_id']);
def words(seq): return struct.pack('<'+'I'*len(seq),*seq)
def raw_at(addr,value):w(addr,struct.pack('<I',value))
def arr(name,baseoff,stride):
 a=data[name]; assert len(a)*stride<=0x1000; flat=[v for row in a for v in row] if stride>4 else a
 assert len(flat)*4==len(a)*stride; w(source+baseoff,words(flat));return len(a)
n0=arr('curve2_points',0,8);nu0=arr('curve2_u',0x1000,4);n1=arr('prepared_positions',0x2000,12);nu1=arr('prepared_u',0x3000,4);nh=arr('wall_heights',0x4000,4);ns=arr('segment_heights',0x5000,4);nt=arr('aux_heights',0x6000,4);ny=arr('prepared_y',0x7000,4);n2=arr('secondary_positions',0x8000,12);nu2=arr('secondary_u',0x9000,4)
for off,val in [(0,n0),(8,source),(0x10,n0),(0x18,nu0),(0x20,source+0x1000),(0x28,nu0),(0x38,n2),(0x40,source+0x8000),(0x48,n2),(0x50,nu2),(0x58,source+0x9000),(0x60,nu2),(0x88,n1),(0x90,source+0x2000),(0x98,n1),(0xa0,nu1),(0xa8,source+0x3000),(0xb0,nu1),(0xc0,nt),(0xc8,source+0x6000),(0xd0,nt),(0xd8,nh),(0xe0,source+0x4000),(0xe8,nh),(0xf0,ns),(0xf8,source+0x5000),(0x100,ns),(0x180,owner)]:q(wall+off,val)
for off,k in [(0x30,'curve2_length'),(0x68,'secondary_length'),(0xb8,'prepared_length'),(0x280,'elevation'),(0x284,'constrained_extra'),(0x288,'height'),(0x28c,'wall_length')]:raw_at(wall+off,data[k])
for flag,value,k in [(0x108,0x10c,'max_y'),(0x110,0x114,'max_y_gable')]:
 v=data[k];w(wall+flag,struct.pack('<I',int(v is not None)));raw_at(wall+value,0 if v is None else v)
shape=bytes.fromhex(data['shape_hex']);assert len(shape)==28;w(wall+0x258,shape)
roof_data=bytes.fromhex(doc['roof_88_hex']);assert len(roof_data)==88;w(roof,roof_data)
q(hole,0);q(hole+8,4);q(hole+16,0);q(deco+0x18,0);q(terrain+8,grid);q(terrain+0x10,512*512);f(terrain+0x50,0.);f(terrain+0x54,0.)
q(owner+8,0x3001b000);q(owner+0x10,0);q(ssbo+0x28,1);q(state,0x30000100);q(state+8,vt);q(vt+0x18,0x30000100);q(state+0x10,0x142c1a968);q(state+0x18,0x3d)
q(0x3001f000+0x58,0x3001f400);q(0x3001f400,0x3001f800);w(base+0x363acb0,struct.pack("<I",0));w(0x3001f800+0x1e8,struct.pack("<QQQ",1,0x123456789abcdef0,0xfedcba9876543210));q(base+0x362c098,0x3001c000);q(base+0x362c308,0x3001c008);q(curved,0);q(curved+8,4);q(curved+16,0)
q(sp,retaddr)
for off,val in [(0x28,terrain),(0x30,0x3001d000),(0x38,source+0x5000),(0x40,ns),(0x48,source+0x7000),(0x50,ny),(0x58,0x3001e000),(0x60,roof),(0x68,hole),(0x70,curved)]:q(sp+off,val)
for reg,val in [(UC_X86_REG_RSP,sp),(UC_X86_REG_RCX,wall),(UC_X86_REG_RDX,wallid),(UC_X86_REG_R8,deco),(UC_X86_REG_R9,derived),(UC_X86_REG_MXCSR,0x1f80),(UC_X86_REG_GS_BASE,0x3001f000)]:u.reg_write(reg,val)
initial={"public_wall_672":bytes(u.mem_read(wall,672)).hex(),"roof_88":bytes(u.mem_read(roof,88)).hex(),"finalized_holes_24":bytes(u.mem_read(hole,24)).hex(),"decorator_storage_256":bytes(u.mem_read(deco,256)).hex(),"derived_view_16":bytes(u.mem_read(derived,16)).hex(),"terrain_header_88":bytes(u.mem_read(terrain,88)).hex(),"source_40960":bytes(u.mem_read(source,0xa000)).hex(),"initial_stack_120":bytes(u.mem_read(sp,0x78)).hex(),"allocation_owner_32":bytes(u.mem_read(owner,32)).hex(),"ssbo_storage_96":bytes(u.mem_read(ssbo,96)).hex(),"ssbo_fake_state_32":bytes(u.mem_read(state,32)).hex(),"ssbo_fake_vtable_32":bytes(u.mem_read(vt,32)).hex(),"curved_vec_24":bytes(u.mem_read(curved,24)).hex(),"terrain_grid_sha256":hashlib.sha256(bytes(u.mem_read(grid,512*512*2))).hexdigest()}
try:u.emu_start(0x141218e30,retaddr+1,count=30000000)
except Exception as e:status='fault:'+str(e)+'@'+hex(u.reg_read(UC_X86_REG_RIP))
if status=='running':status='instruction_budget@'+hex(u.reg_read(UC_X86_REG_RIP))
cap,ptr,length=struct.unpack('<QQQ',bytes(u.mem_read(curved,24)));curvebytes=''
if length and length<=10000:
 try:curvebytes=bytes(u.mem_read(ptr,length*64)).hex()
 except Exception as e:curvebytes='unreadable:'+str(e)
result={'status':status,'rne_sha256':sha,'pdb_sha256':'2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911','entry':'0x141218e30','failure_history':['initial VCRUNTIME memmove import unresolved at 142923c10; bridged standard memmove','Windows HeapAlloc import unresolved at 140d75fc1; bridged dependency allocator','Windows GS TLS absent at 140d4d74a; installed explicit initial TLS state and executed original RNG path'],'input':doc,'input_objects_hex':initial,'ssbo_exhausted_flag':int(u.mem_read(sp-0x6b8+0x4bf,1)[0]),'tls_rng_final_hex':bytes(u.mem_read(0x3001f800+0x1e8,24)).hex(),'bridges_and_observations':counts,'checkpoints':checkpoints,'wood_push_return_sites':wood_sites,'wooden_64b':rows,'curved_header':[cap,hex(ptr),length],'curved_bytes':curvebytes,'recent':[hex(x) for x in recent]}
path=Path(sys.argv[2]);
if path.exists(): raise FileExistsError(path)
path.write_text(json.dumps(result,indent=2)+'\n');print(status,'wood',len(rows),'curved',length,'bridges',counts,'recent',list(map(hex,recent[-12:])))

if status!='returned':sys.exit(1)
