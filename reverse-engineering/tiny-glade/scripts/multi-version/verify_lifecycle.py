"""Original game ranges with explicit dependency API boundaries, no ECS replica."""
import itertools,json,random,struct,subprocess
from verify_parameters import cpu,DATA,STACK,ROOT,baseline,bits
from verify_building_state import reset
from verification_provenance import candidate_provenance
from unicorn import UC_HOOK_CODE
from unicorn.x86_const import *

active=None; action=None; previous=0; emitted=[]
def ret(c):
    sp=c.reg_read(UC_X86_REG_RSP); target=struct.unpack('<Q',c.mem_read(sp,8))[0]
    c.reg_write(UC_X86_REG_RSP,sp+8);c.reg_write(UC_X86_REG_RIP,target)
def boundary(c,pc,size,user):
    global action
    if active=='animation' and pc==0x1421a2e8f:c.emu_stop()
    elif active=='association':
        if pc==0x140eec970:
            action='insert';emitted.append((c.reg_read(UC_X86_REG_RDX),c.reg_read(UC_X86_REG_R8)))
            c.reg_write(UC_X86_REG_RAX,previous);ret(c)
        elif pc in [0x1428d9430,0x1428d9390]:action='panic';c.emu_stop()
        elif pc==0x140a95116:c.emu_stop()
    elif active=='despawn':
        if pc==0x140d14770:
            command=c.reg_read(UC_X86_REG_RCX)
            emitted.append(struct.unpack('<Q',c.mem_read(command+32,8))[0]);ret(c)
        elif pc==0x14062f94b:c.emu_stop()
    elif active=='publication' and pc in [0x140a8d505,0x140a8d8b7]:c.emu_stop()
    elif active=='mark' and pc in [0x14062f776,0x14062f5d6]:c.emu_stop()
    elif active=='projection' and pc==0x140a8db97:c.emu_stop()
cpu.hook_add(UC_HOOK_CODE,boundary)

def call(address,output=None,arg=None,xmm1=0):
    reset();sp=STACK+0x8000;cpu.mem_write(sp,struct.pack('<Q',STACK))
    cpu.reg_write(UC_X86_REG_RSP,sp)
    if output is not None:cpu.mem_write(DATA,output);cpu.reg_write(UC_X86_REG_RCX,DATA)
    if arg is not None:cpu.reg_write(UC_X86_REG_RCX,arg)
    cpu.reg_write(UC_X86_REG_XMM1,xmm1)
    cpu.emu_start(address,STACK,count=1000);assert cpu.reg_read(UC_X86_REG_RIP)==STACK

def tile(raw,height):
    global active
    active='tile';call(0x140939390,raw,xmm1=height)
    return bytes(cpu.mem_read(DATA,24)).hex()

def animation(raw,roof,marked,dt):
    global active
    active='animation';reset();sp=STACK+0x8000
    cpu.mem_write(DATA,roof);cpu.mem_write(DATA+0x100,raw);cpu.mem_write(DATA+0x200,struct.pack('<I',0xaaaaaaaa))
    cpu.mem_write(DATA+0x300,bytes(0x40));cpu.mem_write(DATA+0x328,struct.pack('<I',dt))
    cpu.mem_write(sp,bytes(0x100));cpu.mem_write(sp+0x58,struct.pack('<Q',DATA+0x300));cpu.mem_write(sp+0x3c,struct.pack('<I',0x12345678))
    for reg,value in [(UC_X86_REG_RSP,sp),(UC_X86_REG_RCX,DATA),(UC_X86_REG_RDI,DATA+0x100),
            (UC_X86_REG_RBX,DATA+0x200),(UC_X86_REG_R12,marked)]:cpu.reg_write(reg,value)
    for reg,value in zip(range(UC_X86_REG_XMM6,UC_X86_REG_XMM13+1),[0xc0800000,0xc1200000,0,0x3f800000,0x40000000,0x4036db6e,0x40555555,0x3f7d70a4]):cpu.reg_write(reg,value)
    cpu.emu_start(0x1421a3102,0,count=3000);assert cpu.reg_read(UC_X86_REG_RIP)==0x1421a2e8f
    updated=bytes(cpu.mem_read(DATA+0x100,8));tick=struct.unpack('<I',cpu.mem_read(DATA+0x200,4))[0]
    active='animated';call(0x1408dbc20,arg=DATA+0x100);animated=cpu.reg_read(UC_X86_REG_RAX)&1
    return f'{updated.hex()} {tick:08x} {animated} {despawn(updated,marked)}'

def despawn(raw,marked):
    global active
    active='despawn';emitted.clear();reset();sp=STACK+0x8000
    cpu.mem_write(DATA+0x100,raw);cpu.mem_write(DATA+0x400,struct.pack('<Q',0x100000029));cpu.mem_write(sp,bytes(0x100))
    for reg,value in [(UC_X86_REG_RSP,sp),(UC_X86_REG_R10,marked),(UC_X86_REG_R9,DATA+0x700),
            (UC_X86_REG_R11,DATA+0x100),(UC_X86_REG_RSI,DATA+0x400),(UC_X86_REG_RCX,sp+0x58)]:cpu.reg_write(reg,value)
    cpu.emu_start(0x14062fa15,0,count=3000);assert cpu.reg_read(UC_X86_REG_RIP)==0x14062f94b
    assert emitted in [[],[0x100000029]]
    return len(emitted)

def association(wall,marked,phase,has_previous):
    global active,action,previous
    active='association';action='keep';previous=0x100000011 if has_previous else 0;emitted.clear();reset()
    cpu.mem_write(DATA,bytes(0x500))
    if wall is not None:cpu.mem_write(DATA+8,wall)
    sp=STACK+0x8000;frame=sp+0x80;cpu.mem_write(sp,bytes(0x400));cpu.mem_write(DATA+0x1000,struct.pack('<I',phase))
    cpu.mem_write(DATA+0x1100,struct.pack('<Q',0x1234567890123456))
    q=DATA+0x2000;tick=DATA+0x3000
    cpu.mem_write(q,bytes(0x300));cpu.mem_write(q+0x20,struct.pack('<QQQQQ',16,q+0x100,0,0,0));cpu.mem_write(tick,bytes(4))
    for offset,value in [(-0x28,DATA+0x1000),(-0x20,tick),(0xc8,q),(0x108,0x100000029)]:cpu.mem_write(frame+offset,struct.pack('<Q',value))
    cpu.mem_write(frame+0x120,bytes([marked]));cpu.mem_write(frame+0xbc,struct.pack('<I',0x12345678))
    for reg,value in [(UC_X86_REG_RSP,sp),(UC_X86_REG_RBP,frame),(UC_X86_REG_RCX,DATA+0x2a8),
            (UC_X86_REG_R15,0x1234567890123456),(UC_X86_REG_RBX,DATA+0x1100)]:cpu.reg_write(reg,value)
    cpu.emu_start(0x140a95430 if wall is not None else 0x140a95490,0,count=1000)
    if cpu.reg_read(UC_X86_REG_RIP)!=0x140a95116:action='panic'
    count=struct.unpack('<Q',cpu.mem_read(q+0x30,8))[0]
    if count:
        assert count==1 and struct.unpack('<Q',cpu.mem_read(q+0x108,8))[0]==0x1234567890123456
        action='delete'
    if emitted:assert emitted==[(0x1234567890123456,0x100000029)]
    return action

def publication(values,extra,special):
    global active
    active='publication';reset();sp=STACK+0x8000;frame=sp+0x80
    relative=b''.join(struct.pack('<f',v[0]) for v in values)
    points=b''.join(struct.pack('<fff',0,v[1],0) for v in values)
    cpu.mem_write(DATA+0x100,relative or bytes(4));cpu.mem_write(DATA+0x1000,points or bytes(12));cpu.mem_write(sp,bytes(0x1000))
    cpu.mem_write(frame+0xd08,struct.pack('<Q',DATA+0x1000))
    for reg,value in [(UC_X86_REG_RSP,sp),(UC_X86_REG_RBP,frame),(UC_X86_REG_RSI,len(values)),
            (UC_X86_REG_RBX,DATA+0x100),(UC_X86_REG_XMM10,bits(24)),(UC_X86_REG_XMM11,bits(100))]:cpu.reg_write(reg,value)
    cpu.emu_start(0x140a8d2f9,0,count=10000)
    assert cpu.reg_read(UC_X86_REG_RIP)==0x140a8d505
    outputs=[cpu.reg_read(reg)&0xffffffff for reg in [UC_X86_REG_XMM9,UC_X86_REG_XMM10,UC_X86_REG_XMM13]]
    cpu.mem_write(frame+0xc98,bytes([special]));cpu.mem_write(frame+0xb88,bytes(8))
    cpu.mem_write(DATA+0x5000,bytes(128));cpu.mem_write(frame+0xd40,struct.pack('<Q',DATA+0x5040))
    for reg,value in [(UC_X86_REG_XMM15,bits(.3)),(UC_X86_REG_XMM6,bits(.5)),
            (UC_X86_REG_XMM14,bits(2.5)),(UC_X86_REG_XMM12,extra)]:cpu.reg_write(reg,value)
    cpu.emu_start(0x140a8d7a4,0,count=1000);assert cpu.reg_read(UC_X86_REG_RIP)==0x140a8d8b7
    outputs.append(cpu.reg_read(UC_X86_REG_XMM2)&0xffffffff)
    return ' '.join(f'{v:08x}' for v in outputs)

def mark(roof,entity,requests):
    global active
    active='mark';reset();sp=STACK+0x8000;frame=sp+0x80
    cpu.mem_write(DATA,roof);cpu.mem_write(DATA+0x1000,b''.join(struct.pack('<Q',v) for v in requests) or bytes(8))
    cpu.mem_write(sp,bytes(0x300));cpu.mem_write(frame+0x18,struct.pack('<Q',DATA+0x1000));cpu.mem_write(frame+0x20,struct.pack('<Q',len(requests)))
    for reg,value in [(UC_X86_REG_RSP,sp),(UC_X86_REG_RBP,frame),(UC_X86_REG_R15,DATA)]:cpu.reg_write(reg,value)
    cpu.emu_start(0x14062f749,0,count=3000)
    pc=cpu.reg_read(UC_X86_REG_RIP);assert pc in [0x14062f776,0x14062f5d6]
    if pc==0x14062f5d6:return '-'
    wall_id=struct.unpack_from('<Q',roof,0)[0]
    active='pivot';reset();cpu.mem_write(DATA,roof);cpu.mem_write(DATA+0x300,bytes(12))
    cpu.mem_write(sp,struct.pack('<Q',STACK));cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_RCX,DATA+0x300);cpu.reg_write(UC_X86_REG_RDX,DATA)
    cpu.emu_start(0x1408e3220,STACK,count=50);assert cpu.reg_read(UC_X86_REG_RIP)==STACK
    xyz=struct.unpack('<III',cpu.mem_read(DATA+0x300,12))
    return f'{entity:016x} {wall_id:016x} '+' '.join(f'{v:08x}' for v in xyz)+' 1'

PROJECTION_FIELDS=[(0x10,8),(0x30,4),(0x28c,4),(0x258,28),(0x108,4),(0x110,4),(0x274,4),(0x278,4),(0x27c,4),(0x284,4),(0x299,1)]
def projection(seed,shape,count,height,style,metrics):
    global active
    active='projection';reset();sp=STACK+0x8000;frame=sp+0x80;source=DATA+0x5040
    cpu.mem_write(sp,bytes(0x1000));cpu.mem_write(frame+0x500,seed);cpu.mem_write(DATA+0x5000,bytes(128))
    cpu.mem_write(source-8,bytes([style,0]));cpu.mem_write(frame+0xd40,struct.pack('<Q',source))
    cpu.mem_write(frame+0xd08,struct.pack('<Q',count));cpu.mem_write(frame+0xc40,shape[4:28])
    cpu.mem_write(frame+0xc98,bytes([0]));cpu.mem_write(frame+0xb88,bytes(8))
    for reg,value in [(UC_X86_REG_RSP,sp),(UC_X86_REG_RBP,frame),(UC_X86_REG_RAX,struct.unpack_from('<I',shape)[0]),
            (UC_X86_REG_XMM7,height),(UC_X86_REG_XMM0,height),(UC_X86_REG_XMM9,metrics[0]),(UC_X86_REG_XMM10,metrics[1]),
            (UC_X86_REG_XMM13,metrics[2]),(UC_X86_REG_XMM12,metrics[3]),
            (UC_X86_REG_XMM15,bits(.3)),(UC_X86_REG_XMM6,bits(.5)),(UC_X86_REG_XMM14,bits(2.5))]:cpu.reg_write(reg,value)
    cpu.emu_start(0x140a8d7a4,0,count=1000);assert cpu.reg_read(UC_X86_REG_RIP)==0x140a8db97
    result=bytes(cpu.mem_read(frame+0x500,672))
    return ''.join(result[o:o+n].hex() for o,n in PROJECTION_FIELDS)

def main():
    rng=random.Random(20261015);lines=[];expected=[];groups={}
    for _ in range(256):
        raw=rng.randbytes(24);height=rng.getrandbits(32)
        lines.append(f'tile {raw.hex()} {height:08x}');expected.append(tile(raw,height))
    groups['tile_initialization']=256
    special=[0.,-0.,.98,.99,1.,-1.,float('inf'),float('-inf'),float('nan')]
    for first,second,marked,height,dt in itertools.product(special,[0.,1.,float('inf')],[0,1],[0.,.1,.5,float('nan')],[0.,.1,-.1,float('inf')]):
        raw=struct.pack('<ff',first,second);roof=bytearray(88);struct.pack_into('<f',roof,0x30,height)
        lines.append(f'animation {raw.hex()} {roof.hex()} {marked} {bits(dt):08x}');expected.append(animation(raw,bytes(roof),marked,bits(dt)))
    groups['animation_and_deletion']=len(lines)-256
    count_before=len(lines)
    for count,height,shape,marked,phase,prev in itertools.product([1,2],[0.,1.,float('nan')],[0,1,2,3],[0,1],[0,13,14],[0,1]):
        wall=bytearray(672);struct.pack_into('<Q',wall,0x10,count);struct.pack_into('<f',wall,0x30,height);struct.pack_into('<I',wall,0x258,shape)
        lines.append(f'association {wall.hex()} {marked} {phase} {prev}');expected.append(association(bytes(wall),marked,phase,prev))
    groups['association_policy']=len(lines)-count_before
    for marked,phase,prev in itertools.product([0,1],[0,13,14],[0,1]):
        lines.append(f'association none {marked} {phase} {prev}');expected.append(association(None,marked,phase,prev))
    groups['missing_wall_association_policy']=12
    count_before=len(lines)
    for i in range(512):
        values=[(rng.uniform(-4,30),rng.uniform(-4,110)) for _ in range(i%13)]
        if i%11==0:values=[(float('nan'),0.),(-0.,-0.),(0.,float('nan'))]
        extra=bits(special[i%len(special)]);flag=i%2
        lines.append(f'publication {extra:08x} {flag} '+' '.join(f'{bits(a):08x} {bits(b):08x}' for a,b in values));expected.append(publication(values,extra,flag))
    groups['publication_metrics']=len(lines)-count_before
    count_before=len(lines)
    for i in range(256):
        roof=bytearray(rng.randbytes(88));struct.pack_into('<I',roof,8,i%2);entity=0x100000029
        wall_id=struct.unpack_from('<Q',roof,0)[0];requests=[rng.getrandbits(64) for _ in range(i%7)]
        if i%3:requests.extend([wall_id,wall_id])
        lines.append(f'mark {roof.hex()} {entity:x} '+' '.join(f'{v:x}' for v in requests));expected.append(mark(bytes(roof),entity,requests))
    groups['deletion_request_match_and_pivot']=len(lines)-count_before
    projection_indices=[]
    for i in range(128):
        seed=rng.randbytes(672);shape=bytearray(rng.randbytes(28));struct.pack_into('<I',shape,0,i%2)
        count=i%8+2;height=bits(rng.uniform(.5,14));style=i%4
        metrics=[bits(rng.uniform(-4,24)),bits(rng.uniform(-4,100)),bits(rng.uniform(0,14)),bits(rng.uniform(.5,14))]
        projection_indices.append(len(lines))
        lines.append(f'projection {seed.hex()} {shape.hex()} {count:x} {height:08x} {style} '+' '.join(f'{v:08x}' for v in metrics))
        expected.append(projection(seed,bytes(shape),count,height,style,metrics))
    groups['published_field_projection']=128
    process=subprocess.run([str(ROOT/'reconstruction/target/release/lifecycle-probe')],input='\n'.join(lines)+'\n',capture_output=True,text=True,check=True)
    actual=process.stdout.splitlines();assert len(actual)==len(expected)
    for index in projection_indices:
        raw=bytes.fromhex(actual[index]);seed=bytes.fromhex(lines[index].split()[1])
        changed={i for offset,size in PROJECTION_FIELDS for i in range(offset,offset+size)}
        assert all(a==b for i,(a,b) in enumerate(zip(raw,seed)) if i not in changed),'candidate overwrote unconfirmed/library fields'
        actual[index]=''.join(raw[o:o+n].hex() for o,n in PROJECTION_FIELDS)
    failures=[{'index':i,'input':lines[i],'original':want,'rust':got} for i,(want,got) in enumerate(zip(expected,actual)) if want!=got]
    report={'ok':not failures,'source_sha256':baseline['binary']['sha256'],'cases':len(lines),'case_groups':groups,
        'scope':'B game TileSimulation initialization; resolved animation channels/ticks/despawn intent; association guards/actions; publication numeric reductions and extra constraint',
        'dependency_boundaries':['HashMap insert return supplied at 0x140eec970; game duplicate checks run unchanged',
            'EntityCommands::despawn captured as intent at 0x140d14770; library queue implementation not reconstructed'],
        'excluded':['Complete wall publisher ownership/dirty gathering/cache producers','Entire association rebuild/diff function','Actual Bevy spawn/despawn/registration',
            'TriggerVolume element types and library Vec ABI','Drawable/spire/support initializer remainder','A/C behavior','SSE exception flags'],
        'failure_count':len(failures),'failures':failures[:10],**candidate_provenance()}
    (ROOT/'evidence/building-lifecycle/B-verification.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n')
    print(json.dumps({'ok':report['ok'],'case_groups':groups,'failure_count':len(failures)}))
    if failures:print(json.dumps(failures[:2]));raise SystemExit(1)

if __name__=='__main__':main()
