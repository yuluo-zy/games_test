"""Verify B game rules while excluding hash probing and Bevy entity tables."""
import itertools, json, random, struct, subprocess
from verify_parameters import cpu, DATA, STACK, ROOT, baseline, bits
from verification_provenance import candidate_provenance
from unicorn import UC_HOOK_CODE
from unicorn.x86_const import *

disposition=None
def boundary(c,pc,size,user):
    global disposition
    if pc in [0x1428d9430,0x1428d9390]:
        disposition='panic';c.emu_stop()
    elif pc==0x1421a7c28: c.emu_stop()
    elif pc==0x140562c30: raise RuntimeError('Unexpected dependency queue allocation')
cpu.hook_add(UC_HOOK_CODE,boundary)

def reset():
    global disposition
    disposition=None
    for reg in [UC_X86_REG_RAX,UC_X86_REG_RBX,UC_X86_REG_RCX,UC_X86_REG_RDX,
            UC_X86_REG_RDI,UC_X86_REG_RSI,UC_X86_REG_RBP,UC_X86_REG_R8,
            UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,
            UC_X86_REG_R13,UC_X86_REG_R14,UC_X86_REG_R15]:cpu.reg_write(reg,0)
    for reg in range(UC_X86_REG_XMM0,UC_X86_REG_XMM15+1):cpu.reg_write(reg,0)
    cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)

def guard(count,height):
    reset();cpu.mem_write(DATA,bytes(0x1000))
    # Enter after the dependency's successful key match. Run only game guards.
    cpu.mem_write(DATA+0x18,struct.pack('<Q',count));cpu.mem_write(DATA+0x38,struct.pack('<I',height))
    sp=STACK+0x8000;cpu.mem_write(sp,bytes(0x100));cpu.mem_write(sp+0x48,struct.pack('<Q',STACK))
    cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_RCX,DATA+0x2a8)
    cpu.emu_start(0x140a91fbe,STACK,count=100)
    assert disposition=='panic' or cpu.reg_read(UC_X86_REG_RIP)==STACK
    if not disposition:assert cpu.reg_read(UC_X86_REG_RAX)==DATA+8
    return disposition or 'ok'

def spatial(roof,related,wall,wall_id,entity):
    reset()
    w,r,a,e,q,t= [DATA+offset for offset in [0x1000,0x2000,0x3000,0x4000,0x5000,0x6000]]
    cpu.mem_write(w,wall);cpu.mem_write(r,roof);cpu.mem_write(a,related)
    cpu.mem_write(e,struct.pack('<Q',wall_id));cpu.mem_write(t,struct.pack('<III',0xaaaaaaaa,0xbbbbbbbb,0))
    cpu.mem_write(q,bytes(0x400));cpu.mem_write(q+0x20,struct.pack('<QQQQQ',16,q+0x100,0,0,0))
    cpu.mem_write(q+0x100,bytes([0xa5])*64)
    sp=STACK+0x8000;cpu.mem_write(sp,bytes(0x200))
    for offset,value in [(0x28,0x12345678),(0x2c,0x87654321)]:cpu.mem_write(sp+offset,struct.pack('<I',value))
    for offset,value in [(0x30,q),(0x38,entity),(0x48,t),(0x60,q+0x20),(0x68,t+8),
            (0x78,0),(0x80,t+4),(0x88,a)]:cpu.mem_write(sp+offset,struct.pack('<Q',value))
    for reg,value in [(UC_X86_REG_RAX,w),(UC_X86_REG_R15,r),(UC_X86_REG_RDI,e),
            (UC_X86_REG_RSP,sp),(UC_X86_REG_XMM6,0xbf30a3d7)]:cpu.reg_write(reg,value)
    # Public-wall/ECS resolution is supplied; no game-update calls are stubbed.
    cpu.emu_start(0x1421a7980,0,count=3000)
    if disposition:return disposition
    assert cpu.reg_read(UC_X86_REG_RIP)==0x1421a7c28
    count=struct.unpack('<Q',cpu.mem_read(q+0x30,8))[0]
    assert count<=2 and struct.unpack('<Q',cpu.mem_read(q+0x40,8))[0]==count
    assert struct.unpack('<I',cpu.mem_read(t+8,4))[0]==(0x87654321 if count else 0)
    events=[]
    for i in range(count):
        got_entity,got_wall,kind=struct.unpack('<QQB',cpu.mem_read(q+0x108+i*32,17))
        events.append(f'{got_entity:016x}:{got_wall:016x}:{kind}')
    tick1,tick2=struct.unpack('<II',cpu.mem_read(t,8))
    assert bytes(cpu.mem_read(w,len(wall)))==wall
    return f'{bytes(cpu.mem_read(r,88)).hex()} {bytes(cpu.mem_read(a,48)).hex()} {tick1:08x} {tick2:08x} {count} '+(','.join(events) or '-')

def main():
    rng=random.Random(20261011);lines=[];expected=[];groups={}
    heights=[0.,-0.,-1.,1.,float('inf'),float('-inf'),float('nan')]
    for count,height in itertools.product([0,1,2,3,0xffffffffffffffff],heights):
        lines.append(f'guard {count:x} {bits(height):08x}');expected.append(guard(count,bits(height)))
    groups['public_wall_read_guard']=len(lines)
    cases=[]
    for roof_tag,wall_tag,style,height,variant,cache in itertools.product(
            [0,1],[0,1,2,0xffffffff],[0,1,2,3,255],[-1.,0.,.1,.5,float('nan')],
            ['same','changed','nan','zero'],[0,1]):
        roof=bytearray(rng.randbytes(88));related=rng.randbytes(48);wall=bytearray(0x2a0)
        struct.pack_into('<I6f',roof,8,roof_tag,1.,0.,2.,3.,4.,5.)
        struct.pack_into('<f',roof,0x30,height);roof[0x54]=rng.randrange(2)
        shape=bytearray(roof[8:36]);struct.pack_into('<I',shape,0,wall_tag)
        if variant=='changed':struct.pack_into('<f',shape,12,6.)
        elif variant=='nan':struct.pack_into('<I',shape,8,0x7fc01234)
        elif variant=='zero':struct.pack_into('<I',shape,8,0x80000000)
        if roof_tag==wall_tag==0 and variant=='same':shape[20:28]=rng.randbytes(8)
        struct.pack_into('<Q',wall,0x10,3);struct.pack_into('<f',wall,0x30,2.)
        wall[0x108]=cache;struct.pack_into('<f',wall,0x10c,7.5)
        wall[0x258:0x274]=shape;wall[0x299]=style
        cases.append((bytes(roof),related,bytes(wall),rng.getrandbits(64),rng.getrandbits(64)))
    groups['spatial_boundaries']=len(cases)
    for _ in range(1024):
        roof=bytearray(rng.randbytes(88));wall=bytearray(0x2a0)
        struct.pack_into('<I6f',roof,8,rng.randrange(2),*[rng.uniform(-4,4) for _ in range(6)])
        struct.pack_into('<f',roof,0x30,rng.uniform(0,1));roof[0x54]=rng.randrange(2)
        struct.pack_into('<Q',wall,0x10,3);struct.pack_into('<f',wall,0x30,2.)
        wall[0x108]=1;struct.pack_into('<f',wall,0x10c,rng.uniform(-4,14))
        struct.pack_into('<I6f',wall,0x258,rng.randrange(3),*[rng.uniform(-4,4) for _ in range(6)])
        wall[0x299]=rng.randrange(4)
        cases.append((bytes(roof),rng.randbytes(48),bytes(wall),rng.getrandbits(64),rng.getrandbits(64)))
    groups['spatial_random']=1024
    for roof,related,wall,wall_id,entity in cases:
        lines.append(f'spatial {roof.hex()} {related.hex()} {wall.hex()} {wall_id:x} {entity:x}')
        expected.append(spatial(roof,related,wall,wall_id,entity))
    result=subprocess.run([str(ROOT/'reconstruction/target/release/building-probe')],input='\n'.join(lines)+'\n',capture_output=True,text=True,check=True)
    actual=result.stdout.splitlines();assert len(actual)==len(expected)
    failures=[{'index':i,'input':lines[i],'original':want,'rust':got} for i,(want,got) in enumerate(zip(expected,actual)) if want!=got]
    report={'ok':not failures,'source_sha256':baseline['binary']['sha256'],'cases':len(expected),'case_groups':groups,
        'scope':'B public-wall successful-read game guards and complete resolved roof spatial update; exact Roof88/related48/ticks/ordered geometry events and missing-cache panic disposition',
        'original_ranges':['0x140a91fbe successful map match to return/panic','0x1421a7980 resolved wall to 0x1421a7c28'],
        'game_update_stubs':[],'dependency_implementations_reconstructed':[],
        'excluded':['Hash probing/storage and ECS entity/query resolution','Invalid initial Roof shape ABI','Original game field names for count/related component','Queue growth/allocation failures','SSE exception flags','Other building consumers and A/C behavior'],
        'failure_count':len(failures),'failures':failures[:10],**candidate_provenance()}
    out=ROOT/'evidence/building-state';out.mkdir(exist_ok=True)
    (out/'B-spatial-verification.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n')
    print(json.dumps({'ok':report['ok'],'cases':len(expected),'case_groups':groups,'failures':len(failures)}))
    if failures:print(json.dumps(failures[:2]));raise SystemExit(1)

if __name__=='__main__':main()
