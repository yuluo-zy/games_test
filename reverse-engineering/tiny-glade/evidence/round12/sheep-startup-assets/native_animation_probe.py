from pathlib import Path
import struct,json,hashlib
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
OUT=Path(__file__).resolve().parent;ROOT=Path('D:/game/ljxsj_92385/Tiny Glade');raw=(ROOT/'tiny-glade.rne').read_bytes();p=pefile.PE(data=raw);base=p.OPTIONAL_HEADER.ImageBase;image=p.get_memory_mapped_image()
assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
c=Uc(UC_ARCH_X86,UC_MODE_64);c.mem_map(base,(p.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);c.mem_write(base,image)
D,S,STOP=0x20000000,0x30000000,0x50000000;c.mem_map(D,0x1000000);c.mem_map(S,0x20000);c.mem_map(STOP,0x1000)
q=lambda a,v:c.mem_write(a,struct.pack('<Q',v&0xffffffffffffffff));u=lambda a:struct.unpack('<Q',c.mem_read(a,8))[0];d=lambda a,v:c.mem_write(a,struct.pack('<I',v))
heap=0;panic=None

def alloc(n):
 global heap
 p=heap;heap=(heap+n+15)&~15;return p

def ret():
 sp=c.reg_read(UC_X86_REG_RSP);pc=u(sp);c.reg_write(UC_X86_REG_RSP,sp+8);c.reg_write(UC_X86_REG_RIP,pc)

def hook(cpu,pc,sz,_):
 global panic
 rcx=cpu.reg_read(UC_X86_REG_RCX);rdx=cpu.reg_read(UC_X86_REG_RDX);r8=cpu.reg_read(UC_X86_REG_R8)
 if pc==0x140a0a5f4:cpu.emu_stop();return
 if pc==0x1405b1a70:cpu.reg_write(UC_X86_REG_RAX,0) # std hasher boundary; fixture ctrl group has h2=0.
 elif pc==0x142923c00:cpu.mem_write(rcx,bytes(cpu.mem_read(rdx,r8)))
 elif pc==0x142923c20:cpu.mem_write(rcx,bytes([rdx&255])*r8)
 elif pc in [0x1428d9310,0x1428d9518,0x1428a8320,0x1428a8340,0x1428a8360,0x1428a8380]:panic={'callee':hex(pc),'index':rcx,'length':rdx};cpu.emu_stop();return
 else:return
 ret()
c.hook_add(UC_HOOK_CODE,hook)

def mesh(pos,norm,missing=None,wrong=None):
 m=alloc(120);table=alloc(16*56+32);ctrl=table+16*56;c.mem_write(ctrl,b'\xff'*32);q(m+0x38,ctrl);q(m+0x40,15);q(m+0x50,2)
 for i,(name,values) in enumerate([('Vertex_Position',pos),('Vertex_Normal',norm)]):
  if name==missing:continue
  cpu=ctrl-56*(i+1);s=name.encode();key=alloc(len(s));c.mem_write(key,s);q(cpu,len(s));q(cpu+8,key);q(cpu+16,len(s));d(cpu+24,2 if name==wrong else 3)
  data=alloc(len(values));c.mem_write(data,values);q(cpu+32,len(values)//12);q(cpu+40,data);q(cpu+48,len(values)//12);c.mem_write(ctrl+i,b'\x00')
 return m

def run(name,vertex_count,frames,delighted,variant=None):
 global heap,panic
 heap=D+0x10000;panic=None;c.mem_write(D,bytes(0x1000000));c.mem_write(S,bytes(0x20000));bp=S+0x10000
 framevec=alloc(30*120)
 for i,(pos,norm) in enumerate(frames):
  missing=wrong=None
  if variant and variant.get('frame')==i:missing=variant.get('missing');wrong=variant.get('wrong')
  ptr=mesh(pos,norm,missing,wrong);c.mem_write(framevec+i*120,bytes(c.mem_read(ptr,120)))
 ptr=mesh(*delighted,missing=variant.get('delighted_missing') if variant else None,wrong=variant.get('delighted_wrong') if variant else None);c.mem_write(bp+0x790,bytes(c.mem_read(ptr,120)))
 out=alloc(max(1,vertex_count)*992)
 q(bp+0x888,30);q(bp+0x890,framevec);q(bp+0x898,30);q(bp+0x8d8,max(1,vertex_count));q(bp+0x8e0,out);q(bp+0x8e8,0)
 for r in [UC_X86_REG_RAX,UC_X86_REG_RBX,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_RSI,UC_X86_REG_RDI,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,UC_X86_REG_R14,UC_X86_REG_R15]:c.reg_write(r,0)
 c.reg_write(UC_X86_REG_RBP,bp);c.reg_write(UC_X86_REG_RSP,bp-0x80);c.reg_write(UC_X86_REG_R13,vertex_count);c.reg_write(UC_X86_REG_MXCSR,0x1f80);c.reg_write(UC_X86_REG_RFLAGS,0x202)
 try:c.emu_start(0x140a09fcf,STOP,count=100000000)
 except Exception as e:raise RuntimeError(f'{name}: {e} pc={c.reg_read(UC_X86_REG_RIP):x}')
 n=u(bp+0x8e8);data=bytes(c.mem_read(out,n*992))
 if not panic:assert c.reg_read(UC_X86_REG_RIP)==0x140a0a5f4
 row={'name':name,'vertex_count':vertex_count,'native_completed_rows':n,'panic':panic,'output_sha256':hashlib.sha256(data).hexdigest(),'variant':variant}
 if not panic:
  expected=b''.join(b''.join(pos[v*12:v*12+12]+bytes(4) for pos,_ in frames)+b''.join(norm[v*12:v*12+12]+bytes(4) for _,norm in frames)+delighted[0][v*12:v*12+12]+bytes(4)+delighted[1][v*12:v*12+12]+bytes(4) for v in range(vertex_count))
  assert data==expected,(name,len(data),len(expected))
 (OUT/(name+'.native.bin')).write_bytes(data)
 return row

def triple(words):return struct.pack('<3I',*words)
def vecs(k):return triple([0x3f800000+k,0x80000000,0x7fc01234])+triple([0xff800000,0x00000000,0x7f800000])
frames=[(vecs(i),vecs(60+i)) for i in range(30)];delighted=(vecs(90),vecs(91));cases=[]
cases.append(run('synthetic-payload-preservation',2,frames,delighted))
for vertex_count in [0,1,2]:
 for frame in [0,29]:
  for attr in ['Vertex_Position','Vertex_Normal']:
   cases.append(run(f'missing-{vertex_count}-{frame}-{attr}',vertex_count,frames,delighted,{'frame':frame,'missing':attr}))
   cases.append(run(f'wrong-type-{vertex_count}-{frame}-{attr}',vertex_count,frames,delighted,{'frame':frame,'wrong':attr}))
for attr in ['Vertex_Position','Vertex_Normal']:
 for vertex_count in [0,1,2]:cases.append(run(f'delighted-missing-{vertex_count}-{attr}',vertex_count,frames,delighted,{'delighted_missing':attr}))
for frame in [0,29]:
 short=frames.copy();short[frame]=(short[frame][0][:12],short[frame][1]);cases.append(run(f'short-frame-{frame}',2,short,delighted))
# Real original assets, all860 vertices through original control flow.
files=[];real=[]
for fn in [str(i)+'.json' for i in range(1,31)]+['delighted.json']:
 path=ROOT/'assets/meshes/sheep_animation'/fn;j=json.loads(path.read_text());real.append(tuple(b''.join(struct.pack('<3f',*v) for v in j[key]['buffer']) for key in ['Vertex_Position','Vertex_Normal']));files.append({'path':str(path.relative_to(ROOT)),'sha256':hashlib.sha256(path.read_bytes()).hexdigest()})
cases.append(run('original-assets-860-vertices',860,real[:30],real[30]))
(OUT/'original-animation-fixtures.json').write_text(json.dumps({'original_sha256':hashlib.sha256(raw).hexdigest(),'native_control_range':['0x140a09fcf','0x140a0a5f4'],'scope':'Original complete vertex-major packing loop, all30 frame position/normal typed lookups and bounds, delighted lookups, array zeroing and992B writes. Std hasher returns fixture hash0; table payloads, CRT memset/memcpy and preallocated Vec capacity are test boundaries, not production implementations. Success output compared against independent packed input bytes.','assets':files,'cases':cases},indent=2))
print({'native_cases':len(cases),'actual_vertices':860,'actual_output_bytes':860*992})
