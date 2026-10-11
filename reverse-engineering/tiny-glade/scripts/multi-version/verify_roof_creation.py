"""Execute B create_roof's resolved game initialization, before ECS spawn."""
import itertools, json, random, struct, subprocess
import verify_building_state as state
from verify_parameters import cpu, DATA, STACK, ROOT, baseline
from verification_provenance import candidate_provenance
from unicorn import UC_HOOK_CODE
from unicorn.x86_const import *

def boundary(c,pc,size,user):
    if pc==0x1421ab26a:c.emu_stop()
    elif pc==0x1421ab551:state.disposition='panic';c.emu_stop()
    elif pc==0x140562630:raise RuntimeError('Unexpected dependency queue allocation')
cpu.hook_add(UC_HOOK_CODE,boundary)

def execute(roof,record,wall):
    state.reset()
    w,e,q,t=[DATA+offset for offset in [0x1000,0x4000,0x5000,0x6000]]
    cpu.mem_write(w,wall);cpu.mem_write(e,record)
    cpu.mem_write(q,bytes(0x400));cpu.mem_write(q+0x20,struct.pack('<QQQQQ',16,q+0x100,0,0,0))
    cpu.mem_write(q+0x100,bytes([0xa5])*32);cpu.mem_write(t,bytes(4))
    sp=STACK+0x8000;frame=sp+0x80;cpu.mem_write(sp,bytes(0x400));cpu.mem_write(frame+0xe8,roof)
    for offset,value in [(0x250,t),(0x268,q+0x20),(0x290,q)]:cpu.mem_write(frame+offset,struct.pack('<Q',value))
    cpu.mem_write(frame+0x2a0,struct.pack('<I',0x12345678))
    for reg,value in [(UC_X86_REG_RAX,w),(UC_X86_REG_RBX,w),(UC_X86_REG_R12,e),
            (UC_X86_REG_RSP,sp),(UC_X86_REG_RBP,frame)]:cpu.reg_write(reg,value)
    cpu.emu_start(0x1421ab0f6,0,count=3000)
    if state.disposition:return state.disposition
    assert cpu.reg_read(UC_X86_REG_RIP)==0x1421ab26a
    count=struct.unpack('<Q',cpu.mem_read(q+0x30,8))[0]
    assert count<=1 and struct.unpack('<Q',cpu.mem_read(q+0x40,8))[0]==count
    assert struct.unpack('<I',cpu.mem_read(t,4))[0]==(0x12345678 if count else 0)
    wall_id=struct.unpack('<Q',cpu.mem_read(q+0x108,8))[0] if count else None
    # Roof::ty returns bool in AL; the caller copies EAX to EBX but upper bits
    # are unspecified register residue, not part of the game bool result.
    return f'{bytes(cpu.mem_read(frame+0xe8,88)).hex()} {cpu.reg_read(UC_X86_REG_EBX)&255} '+(f'{wall_id:016x}' if count else '-')

def main():
    rng=random.Random(20261012);cases=[]
    variants=itertools.product([0,1,2],[0,1,2,3],[0,1],[-1.,0.,.1,.10001,.5,1.,float('nan')],[0.,1.])
    for tag,style,cache,height,ridge in variants:
        wall=bytearray(0x2a0);record=bytearray(rng.randbytes(64))
        struct.pack_into('<Q',wall,0x10,3);struct.pack_into('<f',wall,0x30,2.)
        wall[0x108]=cache;struct.pack_into('<f',wall,0x10c,7.5)
        struct.pack_into('<I6f',wall,0x258,tag,1.,0.,2.,3.,4.,5.);wall[0x299]=style
        struct.pack_into('<6f',record,32,.2,.3,.5,height,ridge,.25);record[56]=rng.randrange(2)
        cases.append((rng.randbytes(88),bytes(record),bytes(wall)))
    boundary_count=len(cases)
    for _ in range(512):
        wall=bytearray(0x2a0);record=bytearray(rng.randbytes(64))
        struct.pack_into('<Q',wall,0x10,3);struct.pack_into('<f',wall,0x30,2.)
        wall[0x108]=1;struct.pack_into('<f',wall,0x10c,rng.uniform(-4,14))
        struct.pack_into('<I6f',wall,0x258,rng.randrange(2),*[rng.uniform(-4,4) for _ in range(6)]);wall[0x299]=rng.randrange(4)
        struct.pack_into('<6f',record,32,*[rng.uniform(0,1) for _ in range(6)]);record[56]=rng.randrange(2)
        cases.append((rng.randbytes(88),bytes(record),bytes(wall)))
    inputs=['create '+roof.hex()+' '+record.hex()+' '+wall.hex() for roof,record,wall in cases]
    expected=[execute(*case) for case in cases]
    result=subprocess.run([str(ROOT/'reconstruction/target/release/building-probe')],
        input='\n'.join(inputs)+'\n',capture_output=True,text=True,check=True)
    actual=result.stdout.splitlines();assert len(actual)==len(expected)
    failures=[{'index':i,'input':inputs[i],'original':want,'rust':got} for i,(want,got) in enumerate(zip(expected,actual)) if want!=got]
    report={'ok':not failures,'source_sha256':baseline['binary']['sha256'],'cases':len(cases),
        'case_groups':{'create_boundaries':boundary_count,'create_random':512},
        'scope':'Resolved B create event/public-wall to exact Roof88, roof type and gable wall-ID notification; missing cache/unsupported wall shape panic disposition',
        'original_range':'0x1421ab0f6 to 0x1421ab26a before subsequent component constructors and ECS spawn',
        'game_update_stubs':[],'dependency_implementations_reconstructed':[],
        'excluded':['PublicWalls hash probing and successful-read guards (separately verified)','ECS spawn/registration and deferred Commands',
            'RoofTileSimulation/TriggerVolume/Animation/drawable initialization and subsequent sound events',
            'Missing JSON presence/rim defaults','Queue allocation failures and SSE flags','A/C behavior'],
        'failure_count':len(failures),'failures':failures[:10],**candidate_provenance()}
    (ROOT/'evidence/building-state/B-creation-verification.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n')
    print(json.dumps({'ok':report['ok'],'cases':len(cases),'failure_count':len(failures)}))
    if failures:print(json.dumps(failures[:2]));raise SystemExit(1)

if __name__=='__main__':main()
