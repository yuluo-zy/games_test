"""Validate only resolved edit_roof rules with the untouched original suffix.
World/query resolution is a supplied boundary, not a game/ECS implementation.
Original stores into preallocated message buffers execute as shipped; no writer,
is_gable, pivot or core math stub is substituted. CRT ceilf is host API bridge.
"""
from pathlib import Path
import struct,json,hashlib,ctypes,random,itertools,sys,subprocess
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE,UC_HOOK_MEM_WRITE
from unicorn.x86_const import *
ROOT=Path('D:/game/reverse-engineering/tiny-glade');sys.path.insert(0,str(ROOT/'scripts'))
from dependency_build import build_runner
raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes();sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=struct.unpack_from('<I',raw,60)[0];opt=pe+24;size=struct.unpack_from('<I',raw,opt+56)[0];base=struct.unpack_from('<Q',raw,opt+24)[0]
cpu=Uc(UC_ARCH_X86,UC_MODE_64);cpu.mem_map(base,(size+4095)&~4095)
for i in range(struct.unpack_from('<H',raw,pe+6)[0]):
    vs,va,rs,ro=struct.unpack_from('<IIII',raw,opt+struct.unpack_from('<H',raw,pe+20)[0]+40*i+8);cpu.mem_write(base+va,raw[ro:ro+rs])
DATA,STACK=0x20000000,0x30000000;cpu.mem_map(DATA,0x20000);cpu.mem_map(STACK,0x20000)
crt=ctypes.CDLL('ucrtbase.dll');ceilf=crt.ceilf;ceilf.argtypes=[ctypes.c_float];ceilf.restype=ctypes.c_float
def bits(v):return struct.unpack('<I',struct.pack('<f',v))[0]
def fl(v):return struct.unpack('<f',struct.pack('<I',v))[0]
def xv(reg):return cpu.reg_read(reg)&0xffffffff
snapshot={};marked=0;bridge_calls=0
def hook(c,pc,sz,user):
    global bridge_calls
    if pc==0x142923e60:
        bridge_calls+=1;c.reg_write(UC_X86_REG_XMM0,bits(ceilf(fl(xv(UC_X86_REG_XMM0)))))
        sp=c.reg_read(UC_X86_REG_RSP);ret=struct.unpack('<Q',c.mem_read(sp,8))[0];c.reg_write(UC_X86_REG_RSP,sp+8);c.reg_write(UC_X86_REG_RIP,ret)
    elif pc==0x1421abe48:
        snapshot.update(changes=[xv(UC_X86_REG_XMM7),xv(UC_X86_REG_XMM6),xv(UC_X86_REG_XMM9),xv(UC_X86_REG_XMM8),xv(UC_X86_REG_XMM10)],old=xv(UC_X86_REG_XMM13),new=xv(UC_X86_REG_XMM15),before=c.reg_read(UC_X86_REG_R12)&255,after=c.reg_read(UC_X86_REG_RDI)&255)
    elif pc==0x1421ab9fc:c.emu_stop()
    elif pc in [0x140562630,0x140562ab0,0x140562c30,0x1428d9390,0x1428d9518,0x1428d9430,0x1428d9760]:raise RuntimeError('Unexpected ECS allocation/panic boundary '+hex(pc))
def memwrite(c,access,addr,size,value,user):
    global marked
    o=addr-DATA
    if o in [0x2c,0x30,0x34,0x38,0x24,0x28]:marked|={0x2c:1,0x30:2,0x34:4,0x38:8,0x24:16,0x28:16}[o]
cpu.hook_add(UC_HOOK_CODE,hook);cpu.hook_add(UC_HOOK_MEM_WRITE,memwrite)
def setf(b,o,v):struct.pack_into('<f',b,o,v)
def base_roof():
    b=bytearray(88);struct.pack_into('<Q',b,0,0x0000000100000029);struct.pack_into('<I',b,8,1)
    for o,v in [(0x14,2.),(0x18,-3.),(0x24,.2),(0x28,.3),(0x2c,.5),(0x30,.4),(0x34,.7),(0x38,.2),(0x4c,3.)]:setf(b,o,v)
    return b
def base_event(modes):
    b=bytearray(64)
    for o,m,v in [(0,modes[0],.25),(8,modes[1],.3),(16,modes[2],1.),(24,modes[3],.6),(40,modes[4],.8)]:struct.pack_into('<I',b,o,m);setf(b,o+4,v)
    setf(b,48,-.5);struct.pack_into('<Q',b,32,0x1234567);b[52]=1;return b
def execute(roof,event):
    global marked
    snapshot.clear();marked=0
    sp=STACK+0x8000;cpu.mem_write(DATA,bytes(roof));cpu.mem_write(DATA+0x100,bytes(event));cpu.mem_write(DATA+0x900,struct.pack('<I',0x55555555));cpu.mem_write(sp,bytes(0x200))
    for reg in [UC_X86_REG_RAX,UC_X86_REG_RBX,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_RDI,UC_X86_REG_RSI,UC_X86_REG_RBP,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,UC_X86_REG_R13,UC_X86_REG_R14,UC_X86_REG_R15]:cpu.reg_write(reg,0)
    for reg in range(UC_X86_REG_XMM0,UC_X86_REG_XMM15+1):cpu.reg_write(reg,0)
    for reg,val in [(UC_X86_REG_R15,DATA),(UC_X86_REG_RBX,DATA+0x100),(UC_X86_REG_RSI,DATA+0x900),(UC_X86_REG_RCX,DATA+0x900),(UC_X86_REG_R12,DATA+0x6000),(UC_X86_REG_R13,DATA+0x6100),(UC_X86_REG_R14,DATA+0x120),(UC_X86_REG_R10,0x100000005),(UC_X86_REG_RSP,sp),(UC_X86_REG_XMM12,bits(.1)),(UC_X86_REG_XMM14,bits(1.))]:cpu.reg_write(reg,val)
    def slot(o,v):cpu.mem_write(sp+o,struct.pack('<Q',v))
    def slot32(o,v):cpu.mem_write(sp+o,struct.pack('<I',v))
    slot32(0x28,0x12345678);slot32(0x2c,17);slot32(0x3c,18);slot32(0x94,19);slot32(0xa4,20)
    slot(0xb8,DATA+0x6100)
    qs=[DATA+0x2000,DATA+0x3000,DATA+0x4000,DATA+0x5000]
    for i,q in enumerate(qs):
        cpu.mem_write(q,bytes(0x50));cpu.mem_write(q+0x100,bytes(0x200));cpu.mem_write(q+0x20,struct.pack('<QQQQQ',16,q+0x100,0,0,0))
    rebuild,feedback,geometry,height=qs
    for o,v in [(0xb0,rebuild),(0xd8,rebuild+0x20),(0xf0,rebuild+0x80),(0x30,feedback),(0x70,feedback+0x20),(0x78,feedback+0x80),(0xc0,geometry),(0xe8,geometry+0x20),(0x110,geometry+0x80),(0x80,height),(0xe0,height+0x20),(0xa8,height+0x80)]:slot(o,v)
    cpu.reg_write(UC_X86_REG_MXCSR,0x1f80);cpu.emu_start(0x1421abb2c,0,count=20000)
    assert cpu.reg_read(UC_X86_REG_RIP)==0x1421ab9fc
    def n(q):return struct.unpack('<Q',cpu.mem_read(q+0x30,8))[0]
    output=bytes(cpu.mem_read(DATA,88)).hex();tick=struct.unpack('<I',cpu.mem_read(DATA+0x900,4))[0]
    assert n(rebuild)<=1 and n(geometry)==1 and n(height)<=1
    crossing=0
    if n(height):crossing=2 if cpu.mem_read(height+0x100+25,1)[0] else 1
    result=f'{output} {tick:08x} {marked} '+' '.join(f'{b:08x}' for b in snapshot['changes']+[snapshot['old'],snapshot['new']])+f' {snapshot["before"]} {snapshot["after"]} {n(rebuild)} {n(geometry)} {crossing}'
    for i in range(n(feedback)):
        row=bytes(cpu.mem_read(feedback+0x100+i*40,40));tag=row[8]
        values=[]
        if tag==2:values=[row[9]]+list(struct.unpack_from('<3I',row,12))
        if tag==3:values=list(struct.unpack_from('<2I',row,12))
        assert tag in [2,3,10,11]
        values+=list(struct.unpack_from('<3I',row,24));result+=' |'+str(tag)+''.join(f' {b:08x}' for b in values)
    return result
cases=[]
for modes in itertools.product([0,1,2],repeat=5):cases.append((base_roof(),base_event(modes),'all-valid-modes'))
edges=[-2.,-0.,0.,.099999994,.1,.10000001,.5,1.,1.5,2.,float('inf'),-float('inf'),float('nan')]
for field,(eo,ro) in enumerate([(0,0x2c),(8,0x30),(16,0x34),(24,0x38)]):
 for mode in [1,2,3]:
  for old,new in itertools.product(edges,repeat=2):
    roof=base_roof();setf(roof,ro,old);event=base_event([0]*5);struct.pack_into('<I',event,eo,mode);setf(event,eo+4,new);cases.append((roof,event,'scalar-boundaries'))
vectors=[(0.,0.),(.6,.8),(2.,0.),(-2.,3.),(1e-30,-1e-30),(1e30,-1e30),(float('inf'),0.),(float('nan'),0.),(0.,float('nan'))]
for mode in [1,2,3]:
 for old,new in itertools.product(vectors,repeat=2):
    roof=base_roof();setf(roof,0x24,old[0]);setf(roof,0x28,old[1]);event=base_event([0]*5);struct.pack_into('<I',event,40,mode);setf(event,44,new[0]);setf(event,48,new[1]);cases.append((roof,event,'tip-boundaries'))
rng=random.Random(20261012)
for _ in range(2000):
    roof=base_roof();event=base_event([rng.randrange(3) for _ in range(5)])
    for o in [0x24,0x28,0x2c,0x30,0x34,0x38]:setf(roof,o,rng.uniform(-2.,2.))
    for o in [4,12,20,28,44,48]:setf(event,o,rng.uniform(-2.,2.))
    cases.append((roof,event,'mixed-random'))
lines=[];expected=[]
for roof,event,kind in cases:
    lines.append(f'{roof.hex()} {event.hex()} 55555555 12345678');expected.append(execute(roof,event))
binary=ROOT/'workspace/source-edit-rules.exe';build_runner(ROOT/'scripts/source-edits/runner.rs',binary)
result=subprocess.run([str(binary)],input='\n'.join(lines)+'\n',capture_output=True,text=True,check=True);actual=result.stdout.splitlines()
fail=[dict(index=i,kind=cases[i][2],input=lines[i],original=a,candidate=b)for i,(a,b)in enumerate(zip(expected,actual))if a!=b]
report=dict(ok=not fail and len(actual)==len(expected),source_sha256=sha,cases=len(cases),case_groups={k:sum(c[2]==k for c in cases)for k in set(c[2]for c in cases)},failure_count=len(fail),failures=fail[:25],
    candidate_sources={p.relative_to(ROOT).as_posix():hashlib.sha256(p.read_bytes()).hexdigest()for p in [ROOT/'reconstruction/roof/edit_rules.rs',ROOT/'scripts/source-edits/runner.rs']},
    candidate_cargo_lock_sha256=hashlib.sha256((ROOT/'reconstruction/Cargo.lock').read_bytes()).hexdigest(),
    oracle='Unicorn original edit_roof resolved suffix 0x1421abb2c through one event completion; original is_gable/pivot/real message stores; preallocated queues; CRT ceilf bridge only',
    comparison='exact entire Roof bytes, changed marker, edit deltas, gable predicates, all numerical feedback payloads/pivots, trigger counts and threshold transition; event padding excluded',
    full_game_executed=False,core_stubs=False,crt_ceil_calls=bridge_calls,
    dependencies=['glam = 0.29.3 (Cargo locked; Vec2/Vec3 and clamp_length_max(1.0) public API reused)','CRT ucrtbase.dll ceilf for original import'],
    provenance=['evidence/scope-priorities/mcp-read.json real RE-MCP call','evidence/scope-priorities/mcp_0x1421ab7c0.c','evidence/scope-priorities/0x1421ab7c0-asm.txt'],
    limitations=['ECS entity resolution, original event reader scheduling and failed query branches excluded','Prepared resolved Roof/Event/change tick and valid preallocated message queues; no allocation failure validation','Not the full Bevy system implementation; no rendering or scene replay','SSE exception flags, uninitialized event padding and source ABI ownership excluded'])
(ROOT/'evidence/source-edits/verification.json').write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding='utf-8')
print(json.dumps(dict(ok=report['ok'],cases=len(cases),failures=len(fail))))
if fail:raise SystemExit(1)
