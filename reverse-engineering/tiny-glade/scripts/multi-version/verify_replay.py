"""Execute the original complete B dispatch into VisualEdit with prepared queues."""
from pathlib import Path
import hashlib, itertools, json, random, struct, subprocess
from verify_parameters import cpu, DATA, STACK, ROOT, baseline, bits
from unicorn.x86_const import *
from verification_provenance import candidate_provenance

HISTORY, QUEUE, WRITER, TICK = DATA+0x100, DATA+0x4000, DATA+0x5000, DATA+0x5100

def execute(state, payload, delta_time):
    cpu.mem_write(DATA,state)
    cpu.mem_write(HISTORY,bytes(16)+payload)
    cpu.mem_write(QUEUE,bytes(0x400))
    cpu.mem_write(QUEUE+0x20,struct.pack('<QQQQQ',16,QUEUE+0x100,0,0,0))
    cpu.mem_write(QUEUE+0x100,bytes([0xa5])*64)
    cpu.mem_write(WRITER,struct.pack('<QQQII',QUEUE,0,TICK,0,0x12345678))
    cpu.mem_write(TICK,bytes(4))
    sp=STACK+0x1000
    cpu.mem_write(sp,bytes(0x100))
    cpu.mem_write(sp,struct.pack('<Q',STACK))
    for offset in [0x28,0x30,0x38]:cpu.mem_write(sp+offset,struct.pack('<Q',WRITER))
    cpu.reg_write(UC_X86_REG_RSP,sp)
    cpu.reg_write(UC_X86_REG_RCX,DATA)
    cpu.reg_write(UC_X86_REG_R8,WRITER)
    cpu.reg_write(UC_X86_REG_R9,WRITER)
    cpu.reg_write(UC_X86_REG_XMM1,delta_time)
    cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
    cpu.emu_start(0x1421b48e0,STACK,count=3000)
    assert cpu.reg_read(UC_X86_REG_RIP)==STACK
    assert struct.unpack('<Q',cpu.mem_read(QUEUE+0x30,8))[0]==1
    return bytes(cpu.mem_read(DATA,24)).hex()+' '+bytes(cpu.mem_read(QUEUE+0x100,53)).hex()

def main():
    rng=random.Random(20261010)
    cases=[]
    for mode,undo,duration,elapsed,delta in itertools.product(
            [0,1,2,3],[0,1],[0.,1.,-1.,0.5],[-0.5,0.,0.25,0.5,1.,1.5],[0.,0.1,0.5]):
        state=bytearray(24)
        struct.pack_into('<IfQfBB',state,0,mode,duration,HISTORY,elapsed,undo,0)
        payload=bytearray(64);payload[0]=2
        for offset in [4,8,12,16,20,24,28,32,48,52,56,60]:
            struct.pack_into('<f',payload,offset,rng.uniform(-2,2))
        struct.pack_into('<Q',payload,40,rng.getrandbits(64))
        cases.append((bytes(state),bytes(payload),bits(delta)))
    for _ in range(1024):
        state=bytearray(24)
        struct.pack_into('<IfQfBB',state,0,rng.randrange(4),rng.uniform(0.1,2),HISTORY,
                         rng.uniform(-0.1,3),rng.randrange(2),rng.randrange(2))
        payload=bytearray(64);payload[0]=2
        for offset in [4,8,12,16,20,24,28,32,48,52,56,60]:
            struct.pack_into('<f',payload,offset,rng.uniform(-2,2))
        struct.pack_into('<Q',payload,40,rng.getrandbits(64))
        cases.append((bytes(state),bytes(payload),bits(rng.uniform(-0.1,0.3))))
    inputs=['replay '+state.hex()+' '+payload.hex()+f' {dt:08x}' for state,payload,dt in cases]
    process=subprocess.run([str(ROOT/'reconstruction/target/release/parameter-probe')],
        input='\n'.join(inputs)+'\n',capture_output=True,text=True,check=True)
    actual=process.stdout.splitlines()
    assert len(actual)==len(cases)
    failures=[]
    for i,((state,payload,dt),result) in enumerate(zip(cases,actual)):
        expected=execute(state,payload,dt)
        if expected!=result:
            failures.append({'index':i,'input':inputs[i],'original':expected,'rust':result})
    report={'ok':not failures,'cases':len(cases),'failure_count':len(failures),'failures':failures[:10],
        'source_sha256':baseline['binary']['sha256'],
        'scope':'Complete B RoofReplayCmd::process dispatch for resolved VisualEdit; exact state24 and 53 meaningful edit-event bytes',
        'excluded':['Other RoofEdit variants','History cursor/transaction/snapshot semantics','EventWriter allocation failures','NaN/infinity parameter payloads','Original ECS consumers and scheduling','A/C replay code'],
        'source':{'reconstruction/roof/replay.rs':hashlib.sha256((ROOT/'reconstruction/roof/replay.rs').read_bytes()).hexdigest()},
        'cargo_lock_sha256':hashlib.sha256((ROOT/'reconstruction/Cargo.lock').read_bytes()).hexdigest()}
    report.update(candidate_provenance())
    (ROOT/'evidence/multi-version/B-replay-verification.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n')
    print(json.dumps({'ok':report['ok'],'cases':len(cases),'failures':len(failures)}))
    if failures:print(json.dumps(failures[:2]));raise SystemExit(1)

if __name__=='__main__':main()
