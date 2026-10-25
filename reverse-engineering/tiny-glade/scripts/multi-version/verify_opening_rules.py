"""Execute B complete raw-hole production blocks, after resolved wall placement
and asset-collision lookup. Do not inject precomputed hole output or geometry."""
import hashlib,json,random,struct,subprocess
from verify_parameters import cpu,DATA,STACK,ROOT,baseline,bits
from verify_building_state import reset
from unicorn import UC_HOOK_CODE
from unicorn.x86_const import *

active=False
def ret(c,value=None):
    if value is not None:c.reg_write(UC_X86_REG_RAX,value)
    sp=c.reg_read(UC_X86_REG_RSP);pc=struct.unpack('<Q',c.mem_read(sp,8))[0]
    c.reg_write(UC_X86_REG_RSP,sp+8);c.reg_write(UC_X86_REG_RIP,pc)
def hook(c,pc,size,user):
    if not active:return
    if pc==0x140833260:
        # DecoratorInfo::dst_as_wall is an already-resolved enum/accessor boundary.
        ret(c,DATA+0x1000)
    elif pc==0x140613c10:ret(c,DATA+0x5000)
    elif pc==0x140613c20:ret(c)
    elif pc in [0x140563600,0x1428d9390,0x1428d9430]:
        raise RuntimeError('unexpected capacity/panic path '+hex(pc))
cpu.hook_add(UC_HOOK_CODE,hook)
RANGES={'cw':(0x1407ed546,0x1407ed659),'gw':(0x1407ea272,0x1407ea4e4),
        'cd':(0x1407f4adb,0x1407f4bba),'gd':(0x1407f16c4,0x1407f1969)}
def original(family,size,tag,values):
    global active
    reset();cpu.mem_write(DATA,bytes(0x7000));frame=STACK+0x8000
    cpu.mem_write(frame-0x100,bytes(0x500));cpu.reg_write(UC_X86_REG_RBP,frame);cpu.reg_write(UC_X86_REG_RSP,STACK+0x7000)
    x,y,width,depth,bottom,top=values
    cpu.mem_write(DATA+0x1000,struct.pack('<2f',x,y));cpu.mem_write(DATA+0x2000,bytes([0xa5])*60)
    # DecoratorBlueprint holes vector: capacity, data, len; capacity excludes the
    # standard Vec growth path without altering the business record generation.
    cpu.mem_write(DATA+0xc8,struct.pack('<QQQ',8,DATA+0x2000,0))
    derived_end=DATA+0x3500;wall_end=DATA+0x4000
    cpu.mem_write(derived_end-4,bytes([size]));cpu.mem_write(wall_end-0x48,struct.pack('<I',tag))
    if family=='cw':
        for off,v in [(0x200,wall_end),(0x298,derived_end),(0x2a8,DATA)]:cpu.mem_write(frame+off,struct.pack('<Q',v))
        cpu.mem_write(frame+0xd0,struct.pack('<f',width));cpu.mem_write(frame+0xd8,struct.pack('<2f',bottom,top))
    elif family=='gw':
        cpu.mem_write(frame+0x1b8,struct.pack('<Q',derived_end));cpu.reg_write(UC_X86_REG_R15,DATA)
    elif family=='cd':
        # Successful collision table entry remains the original producer's input;
        # no generic hash probing is executed or reconstructed.
        collision_end=DATA+0x4500;cpu.reg_write(UC_X86_REG_RCX,collision_end)
        cpu.reg_write(UC_X86_REG_RDI,0);cpu.mem_write(collision_end-0x10,struct.pack('<2f',width,depth));cpu.mem_write(collision_end-8,struct.pack('<2f',bottom,top))
        cpu.mem_write(frame+0x170,struct.pack('<Q',derived_end));cpu.mem_write(frame+0x1c8,struct.pack('<Q',DATA))
    else:
        cpu.mem_write(frame+0x1c0,struct.pack('<Q',derived_end));cpu.mem_write(frame+0x1b8,struct.pack('<Q',DATA));cpu.mem_write(frame+0x188,struct.pack('<Q',DATA))
    start,end=RANGES[family];active=True
    try:cpu.emu_start(start,end,count=5000)
    finally:active=False
    assert cpu.reg_read(UC_X86_REG_RIP)==end
    count=struct.unpack('<Q',cpu.mem_read(DATA+0xd8,8))[0]
    assert count==(3 if family in ['gw','gd'] else 1)
    result=[]
    for i in range(count):
        a,b,w,h,kind=struct.unpack('<4IB',cpu.mem_read(DATA+0x2000+20*i,17))
        result.append(f'{a:08x},{b:08x},{w:08x},{h:08x},{kind}')
        assert bytes(cpu.mem_read(DATA+0x2000+20*i+17,3))==bytes([0xa5])*3
    return ';'.join(result)
def main():
    rng=random.Random(20261012);inputs=[];expected=[];cases=[]
    for family in RANGES:
        for i in range(768):
            size=i%3+1;tag=(i//3)%4
            vals=[rng.uniform(-100,100),rng.uniform(-20,20),rng.uniform(-.2,10),rng.uniform(0,5),rng.uniform(-5,0),rng.uniform(0,5)]
            if i<24:vals[i%6]=[0.,-0.,.05,.1,.15,1.][i%6]
            vals=list(struct.unpack('<6f',struct.pack('<6f',*vals)))
            inputs.append(f'{family} {size} {tag} '+' '.join(f'{bits(v):08x}' for v in vals))
            expected.append(original(family,size,tag,vals));cases.append({'family':family,'size':size,'shape':tag})
    probe=ROOT/'reconstruction/target/release/opening-probe'
    got=subprocess.run([str(probe)],input='\n'.join(inputs)+'\n',text=True,capture_output=True,check=True).stdout.splitlines();assert len(got)==len(expected)
    failures=[{'case':i,**cases[i],'input':inputs[i],'original':e,'rust':a} for i,(e,a) in enumerate(zip(expected,got)) if e!=a]
    paths=['reconstruction/building/opening_rules.rs','reconstruction/src/bin/opening-probe.rs','reconstruction/building/wall_holes.rs','reconstruction/building/wall_state.rs','reconstruction/Cargo.toml','reconstruction/Cargo.lock']
    report={'ok':not failures,'cases':len(inputs),'failure_count':len(failures),'failures':failures[:10],
        'source_sha256':baseline['binary']['sha256'],'original_ranges':{k:[hex(a),hex(b)] for k,(a,b) in RANGES.items()},
        'candidate_sources':{p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest() for p in paths},
        'candidate_binary_sha256':hashlib.sha256(probe.read_bytes()).hexdigest(),
        'candidate_cargo_lock_sha256':hashlib.sha256((ROOT/'reconstruction/Cargo.lock').read_bytes()).hexdigest(),
        'scope':'Complete resolved B raw-hole production blocks of four wall window/balcony-door families, all 3 size variants and 4 host shape tags; exact f32 record fields and original preserved tail padding.',
        'bridges':['DecoratorInfo::dst_as_wall resolved accessor','std allocator/free with preallocated memory','successful collision map slot supplied for cottage door','Vec capacity supplied; original stores/counts execute'],
        'excluded':['asset collision lookup and mesh metadata extraction','wall-space placement/morph and clip decisions before resolved production','visual and GPU component builders','original scheduler','A/C producer behavior equivalence']}
    out=ROOT/'evidence/building-holes/opening-B-verification.json';out.write_text(json.dumps(report,indent=2)+'\n')
    print(json.dumps({'ok':not failures,'cases':len(inputs),'failures':len(failures)}));
    if failures:print(json.dumps(failures[:3]));raise SystemExit(1)
if __name__=='__main__':main()
