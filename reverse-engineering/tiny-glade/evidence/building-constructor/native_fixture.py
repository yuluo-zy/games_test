"""Research harness: original full B WallConstructor business body, explicit inputs.
Not an accepted verifier until output, bridges and input provenance are closed.
"""
import sys
if any(v in sys.argv for v in ['--corners-angle','--skirts-height','--corners-seed','--corners-full']):
 # Reuse the session's original-PE loader; no author geometry is substituted.
 from pathlib import Path
 exec(Path('D:/game/tiny-glade/work/m01-scratch/rotate_fragment_fixture.py').read_text().split('cases=')[0])
 from unicorn import UC_HOOK_CODE
 import math,random
 if '--corners-full' in sys.argv:
  import ctypes
  u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(size+4095)&~4095);u.mem_write(base,bytes(image));u.mem_map(0x20000000,0x200000);u.mem_map(0x30000000,0x200000);u.mem_map(0x40000000,0x1000000);u.mem_map(0x50000000,4096)
  data=0x20000000;sp=0x30100008;heap_next=[0x40000000];last=[];bridges={};public_reads=set();trace=[];pending_samples=[]
  def allocate(n):
   ptr=heap_next[0];heap_next[0]+=max(4096,(n+15)&~15);assert heap_next[0]<0x41000000;return ptr
  def return_to_caller(v=None):
   if v is not None:u.reg_write(UC_X86_REG_RAX,v)
   p=u.reg_read(UC_X86_REG_RSP);ra=struct.unpack('<Q',u.mem_read(p,8))[0];u.reg_write(UC_X86_REG_RSP,p+8);u.reg_write(UC_X86_REG_RIP,ra)
  crt=ctypes.CDLL('ucrtbase.dll');crt.ceilf.argtypes=[ctypes.c_float];crt.ceilf.restype=ctypes.c_float;crt.fmodf.argtypes=[ctypes.c_float,ctypes.c_float];crt.fmodf.restype=ctypes.c_float
  def bridge(uc,pc,n,unused):
   last.append(hex(pc))
   if len(last)>80:last.pop(0)
   if pending_samples and pc==pending_samples[-1][0]:
    _,ptr,u_bits=pending_samples.pop();trace.append({'triangle_sampler_u_bits':u_bits,'height_triple_bits':bytes(uc.mem_read(ptr,12)).hex()})
   if pc==0x141217a20:
    rsp=uc.reg_read(UC_X86_REG_RSP);ra=struct.unpack('<Q',uc.mem_read(rsp,8))[0];pending_samples.append((ra,uc.reg_read(UC_X86_REG_RCX),f'{uc.reg_read(UC_X86_REG_XMM2)&0xffffffff:08x}'))
   elif pc==0x141215b72:trace.append({'corner_index':uc.reg_read(UC_X86_REG_R15),'corner_u_bits':bytes(uc.mem_read(uc.reg_read(UC_X86_REG_RBP)+0x24c,4)).hex()})
   elif pc==0x141215c2b:trace.append({'corner_seed':f'{uc.reg_read(UC_X86_REG_R15):016x}'})
   elif pc==0x1412411c0:
    q=uc.reg_read(UC_X86_REG_RDX);cap,ptr,head,count=struct.unpack('<4Q',uc.mem_read(q,32));assert count<=cap and count<10000
    trace.append({'resolve_hole_aabb_bits':bytes(uc.mem_read(uc.reg_read(UC_X86_REG_RCX),16)).hex(),'input_proposals40':[bytes(uc.mem_read(ptr+((head+i)%cap)*40,40)).hex() for i in range(count)]})
   elif pc==0x141216c9d:trace.append({'top_clip_compare_bits':[f'{uc.reg_read(r)&0xffffffff:08x}' for r in [UC_X86_REG_XMM0,UC_X86_REG_XMM15,UC_X86_REG_XMM12]]})
   elif pc==0x141216d21:trace.append({'emitter_flags':uc.reg_read(UC_X86_REG_RAX)&0xffffffff,'resolved_aabb_bits':bytes(uc.mem_read(uc.reg_read(UC_X86_REG_RBP)+0x290,16)).hex()})
   if pc==0x140c16a00:bridges['profiling_disabled']=True;return_to_caller(0)
   elif pc in [0x140613c10,0x140613c40]:
    n=uc.reg_read(UC_X86_REG_RCX);ptr=allocate(n)
    if pc==0x140613c40:uc.mem_write(ptr,bytes(n))
    bridges['allocator']=True;return_to_caller(ptr)
   elif pc in [0x140613c20,0x14289aa40]:bridges['deallocator']=True;return_to_caller()
   elif pc==0x140613c30:
    old,oldn,newn=(uc.reg_read(r) for r in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R9]);ptr=allocate(newn);uc.mem_write(ptr,bytes(uc.mem_read(old,min(oldn,newn))));bridges['reallocator']=True;return_to_caller(ptr)
   elif pc in [0x142923c00,0x142923c10]:
    dst,src,n=(uc.reg_read(r) for r in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8]);uc.mem_write(dst,bytes(uc.mem_read(src,n)));bridges['memcpy_memmove']=True;return_to_caller(dst)
   elif pc==0x142923c20:
    dst,v,n=(uc.reg_read(r) for r in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8]);uc.mem_write(dst,bytes([v&255])*n);bridges['memset']=True;return_to_caller(dst)
   elif pc==0x142923e60:
    f=struct.unpack('<f',struct.pack('<I',uc.reg_read(UC_X86_REG_XMM0)&0xffffffff))[0];result=crt.ceilf(f);uc.reg_write(UC_X86_REG_XMM0,vec(result,0,0,0));bridges['host_ucrt_ceilf']=True;return_to_caller()
   elif pc==0x142923f10:
    f=[struct.unpack('<f',struct.pack('<I',uc.reg_read(r)&0xffffffff))[0] for r in [UC_X86_REG_XMM0,UC_X86_REG_XMM1]];result=crt.fmodf(*f);uc.reg_write(UC_X86_REG_XMM0,vec(result,0,0,0));bridges['host_ucrt_fmodf']=True;return_to_caller()
   elif 0x1428d8f00<=pc<=0x1428f1800:raise RuntimeError('original panic '+hex(pc))
  u.hook_add(UC_HOOK_CODE,bridge)
  from unicorn import UC_HOOK_MEM_READ
  def reads(uc,access,address,n,value,unused):
   if data<=address<data+0x2a0:public_reads.add((address-data,n))
  u.hook_add(UC_HOOK_MEM_READ,reads)
  points=[[-2.,-2.],[2.,-2.],[2.,2.],[-2.,2.],[-2.,-2.]];us=[0.,.25,.5,.75,1.];n=len(points)
  u.mem_write(data,struct.pack('<QQQQQQf',n,data+0x1000,n,n,data+0x2000,n,16.));u.mem_write(data+0x1000,b''.join(struct.pack('<2f',*p) for p in points));u.mem_write(data+0x2000,struct.pack('<5f',*us))
  u.mem_write(data+0x88,struct.pack('<QQQQQQf',n,data+0x4000,n,n,data+0x5000,n,16.));u.mem_write(data+0x4000,b''.join(struct.pack('<3f',p[0],0.,p[1]) for p in points));u.mem_write(data+0x5000,struct.pack('<5f',*us))
  third=[1.]*n if '--support1' in sys.argv else [0.]*n
  u.mem_write(data+0xf0,struct.pack('<QQQ',n,data+0x6000,n));u.mem_write(data+0x6000,struct.pack('<5f',*third));u.mem_write(data+0x108,struct.pack('<If',1,3.));u.mem_write(data+0x258,struct.pack('<I6f',1,1.,0.,0.,0.,4.,4.));u.mem_write(data+0x280,struct.pack('<4f',0.,3.,0.,16.))
  top_tag=1 if '--top1' in sys.argv else 2;arches=[3.,5.,3.,5.,3.] if top_tag==1 else [3.]*n;top_values=[3.,0.,.5,.5,1.] if top_tag==1 else [0.]*5
  if '--top-steep' in sys.argv:assert top_tag==1;arches=[1.,10.,1.,10.,1.]
  u.mem_write(data+0x108,struct.pack('<If',1,max(arches)));u.mem_write(data+0x3000,struct.pack('<5f',*[0.]*n));u.mem_write(data+0x7000,struct.pack('<5f',*arches));u.mem_write(data+0x8000,struct.pack('<QQQ',10000,data+0x10000,0));u.mem_write(data+0x9000,struct.pack('<QQQQQ',0,4,0,0,0));u.mem_write(data+0xa000,struct.pack('<5fBxxx',*top_values,top_tag))
  input_holes=[]
  if '--hole' in sys.argv:
   input_holes=[(0.,1.5,1.2,2.,0)];u.mem_write(data+0xb000,struct.pack('<QQQ',1,data+0xb100,1));u.mem_write(data+0xb100,struct.pack('<4fBxxx',*input_holes[0]));u.mem_write(sp,struct.pack('<Q',0x50000000));u.reg_write(UC_X86_REG_RSP,sp);u.reg_write(UC_X86_REG_RCX,data+0x9000);u.reg_write(UC_X86_REG_RDX,data+0xb000);u.emu_start(0x14123fe30,0x50000000,count=100000);assert u.reg_read(UC_X86_REG_RIP)==0x50000000
  u.mem_write(sp,struct.pack('<Q',0x50000000));u.reg_write(UC_X86_REG_RSP,sp);u.reg_write(UC_X86_REG_RCX,data);u.reg_write(UC_X86_REG_RDX,27);u.reg_write(UC_X86_REG_R8,data+0x3000);u.reg_write(UC_X86_REG_R9,n)
  for off,v in {0x28:data+0x7000,0x30:n,0x38:data+0x8000,0x40:data+0x9000,0x48:2,0x50:data+0xa000,0x58:0}.items():u.mem_write(sp+off,struct.pack('<Q',v))
  try:
   u.emu_start(0x141215540,0x50000000,count=10000000);assert u.reg_read(UC_X86_REG_RIP)==0x50000000
   cap,ptr,count=struct.unpack('<3Q',u.mem_read(data+0x8000,24));records=[bytes(u.mem_read(ptr+i*96,96)).hex() for i in range(count)];hcap,hptr,hcount=struct.unpack('<3Q',u.mem_read(data+0x9000,24));holes=[bytes(u.mem_read(hptr+i*20,20)).hex() for i in range(hcount)]
   out={'scope':'Original complete add_wall_corners141215540 with explicit rectangle/published fields; original geometry/hole algorithms retained; listed allocator/CRT/profiling bridges only. Input nonempty Wip holes initialized through original InProgressHoleStorage::extend. Public read offsets and observational intermediate trace are retained to audit inputs. Trace observers do not substitute algorithms. Not complete construct_walls system','pe_sha256':hashlib.sha256(raw).hexdigest(),'points':points,'points_u':us,'length':16.,'prepared_y':[0.]*n,'arch_heights':arches,'third_heights':third,'construction_kind':2,'top_tag':top_tag,'top_values':top_values,'input_holes':input_holes,'bridges':bridges,'public_reads':sorted(public_reads),'trace':trace,'records':records,'raw_holes':holes}
   filename='B-corners-full'+('-top1' if top_tag==1 else '')+('-support1' if '--support1' in sys.argv else '')+('-steep' if '--top-steep' in sys.argv else '')+('-hole' if '--hole' in sys.argv else '')+'-fixture.json'
   Path('D:/game/reverse-engineering/tiny-glade/evidence/building-structures/'+filename).write_text(json.dumps(out,indent=2,allow_nan=False));print(json.dumps({'records':len(records),'holes':len(holes),'bridges':bridges,'public_reads':sorted(public_reads)}));raise SystemExit
  except Exception as error:
   out={'error':str(error),'rip':hex(u.reg_read(UC_X86_REG_RIP)),'last':last,'public_reads':sorted(public_reads),'bridges':bridges};Path('D:/game/reverse-engineering/tiny-glade/evidence/building-structures/B-corners-full-first-failure.json').write_text(json.dumps(out,indent=2));raise
 if '--corners-seed' in sys.argv:
  def seed_fragment(index,corner_u,holes):
   u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(size+4095)&~4095);u.mem_write(base,bytes(image));u.mem_map(0x100000,0x4000);u.reg_write(UC_X86_REG_RAX,0x100000);u.reg_write(UC_X86_REG_RDX,len(holes));u.reg_write(UC_X86_REG_R15,index);u.reg_write(UC_X86_REG_RBP,0x102000);u.reg_write(UC_X86_REG_XMM12,0x5f000000);u.reg_write(UC_X86_REG_XMM14,0);u.mem_write(0x10224c,struct.pack('<f',corner_u))
   for i,(x,y,kind) in enumerate(holes):u.mem_write(0x100000+i*20,struct.pack('<4fBxxx',x,y,1.,1.,kind))
   u.emu_start(0x141215b72,0x141215c2b,count=10000)
   return f'{u.reg_read(UC_X86_REG_R15):016x}'
  samples=[('empty',4,10.,[]),('one',4,10.,[(10.,2.9,0)]),('kind2',4,10.,[(10.,2.9,2)]),('near-and-far',4,10.,[(10.009,2.9,1),(10.02,8.,2),(9.999,3.9,4)]),('negative',4,10.,[(10.,-3.,0)]),('nan-height',4,10.,[(10.,math.nan,0)]),('infinite-height',4,10.,[(10.,math.inf,0)])]
  def hb(holes):return [[*(f'{struct.unpack("<I",struct.pack("<f",v))[0]:08x}' for v in [x,y,1.,1.]),kind] for x,y,kind in holes]
  out={'scope':'Original corner hole-sensitive seed span141215b72..5c24, raw20B holes returned by InProgressHoleStorage::holes; no bridges/substitutions. Input R15 is selected corner index; height conversion and wrapping addition are original instructions. Not full corner emitter','pe_sha256':hashlib.sha256(raw).hexdigest(),'cases':[{'name':n,'corner_index':idx,'corner_u':cu,'raw_holes_bits':hb(h),'seed_bits':seed_fragment(idx,cu,h)} for n,idx,cu,h in samples]}
  Path('D:/game/reverse-engineering/tiny-glade/evidence/building-structures/B-corners-seed-fragment-fixture.json').write_text(json.dumps(out,indent=2,allow_nan=False));print(json.dumps(out,indent=2));raise SystemExit
 if '--skirts-height' in sys.argv:
  def native_skirts_height(bits):
   u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(size+4095)&~4095);u.mem_write(base,bytes(image));u.mem_map(0x100000,0x20000);u.reg_write(UC_X86_REG_RSP,0x118008);u.mem_write(0x118008,struct.pack('<Q',0x100200));u.reg_write(UC_X86_REG_RCX,0x100000);u.reg_write(UC_X86_REG_XMM1,bits)
   def capacity_bridge(uc,address,n,data):
    if address==0x140650050:
     # Generic Vec<f32> growth only: preserve len/data, own a disjoint 64-slot buffer.
     owner=uc.reg_read(UC_X86_REG_RCX);cap,ptr,count=struct.unpack('<3Q',uc.mem_read(owner,24));assert count<64
     if count:uc.mem_write(0x101000,bytes(uc.mem_read(ptr,count*4)))
     uc.mem_write(owner,struct.pack('<QQQ',64,0x101000,count));sp=uc.reg_read(UC_X86_REG_RSP);ret=struct.unpack('<Q',uc.mem_read(sp,8))[0];uc.reg_write(UC_X86_REG_RSP,sp+8);uc.reg_write(UC_X86_REG_RIP,ret)
   u.hook_add(UC_HOOK_CODE,capacity_bridge);u.emu_start(0x140b34f80,0x100200,count=20000);assert u.reg_read(UC_X86_REG_RIP)==0x100200
   cap,ptr,count=struct.unpack('<3Q',u.mem_read(0x100000,24));return [f'{x:08x}' for x in struct.unpack('<'+'I'*count,u.mem_read(ptr,count*4))] if count else []
  heights=[0.,-0.,-1.,5.,5.9,9.4,12.9,20.,40.]
  height_bits=[struct.unpack('<I',struct.pack('<f',v))[0] for v in heights]+[0x40bccccb,0x40bccccd,0x7fc12345,0xffc12345]
  out={'scope':'Original complete split_height_into_skirts140b34f80; only generic Vec<f32> capacity growth140650050 bridged, capacity/pointers excluded from comparison; no skirt geometry/emitter or full add_skirts execution','pe_sha256':hashlib.sha256(raw).hexdigest(),'cases':[{'height_bits':f'{b:08x}','output_bits':native_skirts_height(b)} for b in height_bits]}
  Path('D:/game/reverse-engineering/tiny-glade/evidence/building-structures/B-skirts-height-fixture.json').write_text(json.dumps(out,indent=2,allow_nan=False));print(json.dumps(out,indent=2));raise SystemExit
 def corner_fragment(previous,current,next_point):
  u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(size+4095)&~4095);u.mem_write(base,bytes(image));u.mem_map(0x100000,4096)
  for ptr,p in zip([0x100000,0x100100,0x100200],[previous,current,next_point]):u.mem_write(ptr,struct.pack('<2f',*p))
  u.reg_write(UC_X86_REG_RSI,0x100000);u.reg_write(UC_X86_REG_RAX,0x100100);u.reg_write(UC_X86_REG_R13,0x100200)
  u.reg_write(UC_X86_REG_XMM6,0xbe31d0d5);u.reg_write(UC_X86_REG_XMM7,0x3e31d0d7)
  u.emu_start(0x141215799,0x1412157e2)
  cosine=u.reg_read(UC_X86_REG_XMM0)&0xffffffff
  def stop_branch(uc,address,n,data):
   if address in [0x1412157fb,0x1412158c2]:uc.emu_stop()
  u.hook_add(UC_HOOK_CODE,stop_branch);u.emu_start(0x1412157e2,0x1412158c3,count=100)
  target=u.reg_read(UC_X86_REG_RIP);assert target in [0x1412157fb,0x1412158c2]
  return {'cosine_bits':f'{cosine:08x}','selected':target==0x1412158c2,'branch_target':hex(target)}
 cases=[('right',(-1.,0.),(0.,0.),(0.,1.)),('straight',(-1.,0.),(0.,0.),(1.,0.)),('repeat',(0.,0.),(0.,0.),(0.,1.))]
 for threshold in [0xbe31d0d5,0x3e31d0d7]:
  for offset in range(-6,7):
   c=struct.unpack('<f',struct.pack('<I',threshold+offset))[0];z=struct.unpack('<f',struct.pack('<f',math.sqrt(1-c*c)))[0]
   cases.append((f'threshold-{threshold:08x}-{offset}',(-1.,0.),(0.,0.),(c,z)))
 rng=random.Random(20261028)
 for i in range(24):cases.append((f'random-{i}',tuple(rng.uniform(-10,10) for _ in range(2)),tuple(rng.uniform(-10,10) for _ in range(2)),tuple(rng.uniform(-10,10) for _ in range(2))))
 out={'scope':'Original B incoming/outgoing corner cosine and threshold branch 141215799..7e2,1412157e2..8c2; no hooks substitute algorithms; hook stops at original selected/rejected labels; not full corner emitter','pe_sha256':hashlib.sha256(raw).hexdigest(),'cases':[{'name':n,'previous':p,'current':c,'next':q,**corner_fragment(p,c,q)} for n,p,c,q in cases]}
 if '--current-corner-probe' in sys.argv:
  import subprocess
  rows='\n'.join(' '.join(str(v) for p in [c['previous'],c['current'],c['next']] for v in p) for c in out['cases'])+'\n'
  actual=subprocess.run(['D:/game/tiny-glade/work/m01-scratch/m06_corner_probe.exe'],input=rows,text=True,capture_output=True,check=True).stdout.splitlines();assert len(actual)==len(cases)
  for case,value in zip(out['cases'],actual):case['current_rust_selected']=value=='true'
  source=Path('D:/game/tiny-glade/crates/systems/wall-constructor/src/utils/wall_corners.rs')
  out['current_rust_source_sha256']=hashlib.sha256(source.read_bytes()).hexdigest();out['current_rust_difference_count']=sum(c['selected']!=c['current_rust_selected'] for c in out['cases'])
 Path('D:/game/reverse-engineering/tiny-glade/evidence/building-structures/B-corners-angle-fragment-fixture.json').write_text(json.dumps(out,indent=2,allow_nan=False));print(json.dumps({'cases':len(cases),'selected':sum(c['selected'] for c in out['cases']),'current_rust_difference_count':out.get('current_rust_difference_count')}));raise SystemExit
import json,struct,ctypes,ctypes.util,math,hashlib
from pathlib import Path
from unicorn import *
from unicorn.x86_const import *
ROOT=Path(__file__).resolve().parents[2]
b=json.loads((ROOT/'tracking/baselines.json').read_text())['baselines']['B'];raw=Path(b['binary']['path']).read_bytes();assert hashlib.sha256(raw).hexdigest()==b['binary']['sha256'];base=b['pe']['image_base'];cpu=Uc(UC_ARCH_X86,UC_MODE_64);end=max(s['rva']+max(s['raw_size'],s['virtual_size']) for s in b['pe']['sections']);cpu.mem_map(base,(end+4095)&~4095)
for s in b['pe']['sections']:
 if s['raw_size']:cpu.mem_write(base+s['rva'],raw[s['raw_pointer']:s['raw_pointer']+s['raw_size']])
data,stack,heap,stop=0x20000000,0x30000000,0x40000000,0x50000000
cpu.mem_map(data,0x100000);cpu.mem_map(stack,0x200000);cpu.mem_map(heap,0x1000000);cpu.mem_map(stop,0x1000);allocation=[heap];last=[]
def u64(p):return struct.unpack('<Q',cpu.mem_read(p,8))[0]
def wf(p,v):cpu.mem_write(p,struct.pack('<f',v))
def bits(f):return struct.unpack('<I',struct.pack('<f',f))[0]
def fl(v):return struct.unpack('<f',struct.pack('<I',v&0xffffffff))[0]
def ret(v=None):
 if v is not None:cpu.reg_write(UC_X86_REG_RAX,v)
 sp=cpu.reg_read(UC_X86_REG_RSP);ra=u64(sp);cpu.reg_write(UC_X86_REG_RSP,sp+8);cpu.reg_write(UC_X86_REG_RIP,ra)
def alloc(n):
 p=allocation[0];allocation[0]+=max((n+15)&~15,0x1000);assert allocation[0]<heap+0x1000000;return p
lib=ctypes.CDLL(ctypes.util.find_library('m') or ctypes.util.find_library('c'))
functions={0x142923ee0:('floorf',1),0x142923e60:('ceilf',1),0x142923fd0:('sinf',1),0x142924060:('truncf',1),0x142923fa0:('roundf',1)}
for pc,(name,count)in functions.items():f=getattr(lib,name);f.argtypes=[ctypes.c_float]*count;f.restype=ctypes.c_float

def hook(uc,pc,size,user):
 if len(last)>100:last.pop(0)
 last.append(hex(pc))
 if pc==0x140c16a00:ret(0)
 elif pc in [0x140613c10,0x140613c40]:
  n=cpu.reg_read(UC_X86_REG_RCX);p=alloc(n)
  if pc==0x140613c40 and n:cpu.mem_write(p,bytes(n))
  ret(p)
 elif pc in [0x140613c20,0x14289aa40]:ret()
 elif pc==0x140613c30:
  old=cpu.reg_read(UC_X86_REG_RCX);oldn=cpu.reg_read(UC_X86_REG_RDX);newn=cpu.reg_read(UC_X86_REG_R9);p=alloc(newn);cpu.mem_write(p,bytes(cpu.mem_read(old,min(oldn,newn))));ret(p)
 elif pc in [0x142923c00,0x142923c10]:
  dst,src,n=(cpu.reg_read(r) for r in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8]);
  if n:
   try:cpu.mem_write(dst,bytes(cpu.mem_read(src,n)))
   except Exception as e:raise RuntimeError(f'CRT memory bridge pc={pc:x} dst={dst:x} src={src:x} count={n}')from e
  ret(dst)
 elif pc==0x142923c20:
  dst,value,n=(cpu.reg_read(r) for r in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8]);
  if n:cpu.mem_write(dst,bytes([value&255])*n)
  ret(dst)
 elif pc==0x142923bf0:
  a,z,n=(cpu.reg_read(r)for r in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8]);aa=bytes(cpu.mem_read(a,n))if n else b'';zz=bytes(cpu.mem_read(z,n))if n else b'';ret((aa>zz)-(aa<zz))
 elif pc in functions:
  name,count=functions[pc];f=getattr(lib,name);cpu.reg_write(UC_X86_REG_XMM0,bits(f(fl(cpu.reg_read(UC_X86_REG_XMM0)))));ret()
 elif 0x1428d8f00<=pc<=0x1428f1800:raise RuntimeError('panic '+hex(pc))
cpu.hook_add(UC_HOOK_CODE,hook)

def call(seed=27,points=None,us=None,length=12.,height=2.,holes=None,closed=True,authority_heights=None,verbose=False):
 allocation[0]=heap;cpu.mem_write(data,bytes(0x100000));cpu.mem_write(stack,bytes(0x200000))
 points=points or [[-2.,-1.],[-2.,1.],[2.,1.],[2.,-1.],[-2.,-1.]];us=us or [0.,1/6,.5,2/3,1.]
 n=len(points);curve=data+0x100;cpu.mem_write(curve,struct.pack('<QQQQQQf',n,data+0x1000,n,n,data+0x2000,n,length))
 cpu.mem_write(data+0x1000,b''.join(struct.pack('<2f',*p) for p in points));cpu.mem_write(data+0x2000,struct.pack('<'+'f'*n,*us))
 cpu.mem_write(data+0x3000,struct.pack('<'+'f'*n,*(authority_heights if authority_heights is not None else [height]*n)));cpu.mem_write(data+0x3100,struct.pack('<'+'f'*n,*[0.]*n));cpu.mem_write(data+0x3200,struct.pack('<'+'f'*n,*[0.]*n));cpu.mem_write(data+0x3300,struct.pack('<Q',1))
 # Original FinalizedWallHoles view: cap/ptr/count at0/8/16 length24 closed28.
 holes=holes or [];cpu.mem_write(data+0x4100,b''.join(struct.pack('<4fBBxx',*a,kind,origin)for a,kind,origin in holes));cpu.mem_write(data+0x4000,struct.pack('<QQQfB',len(holes),data+0x4100,len(holes),length,int(closed)));cpu.mem_write(data,struct.pack('<QQQ',4096,data+0x10000,0));cpu.mem_write(data+0x5000,struct.pack('<QQQ',8192,data+0x30000,0))
 cpu.mem_write(data+0x6000,struct.pack('<5fB',0.,0.,0.,0.,0.,2))
 sp=stack+0x100008;cpu.mem_write(sp,struct.pack('<Q',stop));cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_RCX,data);cpu.reg_write(UC_X86_REG_RDX,curve);cpu.reg_write(UC_X86_REG_R8,23);cpu.reg_write(UC_X86_REG_R9,data+0x3000);cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
 args={0x28:n,0x30:data+0x3100,0x38:n,0x40:data+0x3200,0x48:n,0x50:2,0x58:seed,0x60:data+0x4000,0x68:0,0x70:data+0x5000,0x78:0,0x80:data+0x6000,0x88:0,0x90:0,0x98:0,0xa0:0,0xa8:bits(1.),0xb0:0}
 for o,v in args.items():cpu.mem_write(sp+o,struct.pack('<Q',v))
 try:cpu.emu_start(0x140769760,stop,count=10000000)
 except Exception as e:
  print('FAIL',e,'rip',hex(cpu.reg_read(UC_X86_REG_RIP)),'last',last[-12:]);raise
 count=u64(data+0x5010);records=[bytes(cpu.mem_read(data+0x30000+i*96,96)).hex()for i in range(count)];
 if verbose:print('records',count,'normalized_holes',u64(data+16))
 return records
if __name__=='__main__':call()
