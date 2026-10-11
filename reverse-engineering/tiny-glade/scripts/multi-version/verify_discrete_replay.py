"""Differentially execute B create/delete/direction/flat-style replay branches."""
import itertools, json, random, struct, subprocess
from verify_parameters import cpu, DATA, STACK, ROOT, baseline, bits
from verify_replay import HISTORY
from verification_provenance import candidate_provenance
from unicorn.x86_const import *

def execute(state, payload, delta_time):
    cpu.mem_write(DATA,state)
    cpu.mem_write(HISTORY,bytes(16)+payload)
    queues=[DATA+0x8000+i*0x1000 for i in range(5)]
    writers=[DATA+0xd000+i*0x40 for i in range(5)]
    ticks=[DATA+0xe000+i*4 for i in range(5)]
    for queue,writer,tick in zip(queues,writers,ticks):
        cpu.mem_write(queue,bytes(0x400))
        cpu.mem_write(queue+0x20,struct.pack('<QQQQQ',16,queue+0x100,0,0,0))
        cpu.mem_write(queue+0x100,bytes([0xa5])*64)
        cpu.mem_write(writer,struct.pack('<QQQII',queue,0,tick,0,0x12345678))
        cpu.mem_write(tick,bytes(4))
    sp=STACK+0x1000;cpu.mem_write(sp,bytes(0x100));cpu.mem_write(sp,struct.pack('<Q',STACK))
    for offset,writer in zip([0x28,0x30,0x38],writers[2:]):
        cpu.mem_write(sp+offset,struct.pack('<Q',writer))
    cpu.reg_write(UC_X86_REG_RSP,sp)
    cpu.reg_write(UC_X86_REG_RCX,DATA)
    cpu.reg_write(UC_X86_REG_R8,writers[0]);cpu.reg_write(UC_X86_REG_R9,writers[1])
    cpu.reg_write(UC_X86_REG_XMM1,delta_time);cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
    cpu.emu_start(0x1421b48e0,STACK,count=3000)
    assert cpu.reg_read(UC_X86_REG_RIP)==STACK
    result=bytes(cpu.mem_read(DATA,24)).hex()
    emitted=[]
    for tag,queue,tick in zip(range(5),queues,ticks):
        count=struct.unpack('<Q',cpu.mem_read(queue+0x30,8))[0]
        sequence=struct.unpack('<Q',cpu.mem_read(queue+0x40,8))[0]
        assert count in [0,1] and sequence==count
        expected_tick=0x12345678 if count else 0
        assert struct.unpack('<I',cpu.mem_read(tick,4))[0]==expected_tick
        if count:
            end={0:61,1:17,3:17,4:19}[tag]
            emitted.append(f'{tag} '+bytes(cpu.mem_read(queue+0x108,end-8)).hex())
    assert len(emitted)<=1
    return result+' '+(emitted[0] if emitted else '- -')

def main():
    rng=random.Random(20261011);cases=[]
    for tag,mode,undo,duration,elapsed,delta,finished in itertools.product(
            [0,1,3,4],[0,1,2,3],[0,1],[0.,1.,-1.,.5],
            [-.5,-0.,.25,.5,1.,float('inf'),float('nan')],[0.,.1,.5,-.5],[0,1]):
        state=struct.pack('<IfQfBBxx',mode,duration,HISTORY,elapsed,undo,finished)
        payload=bytearray(rng.randbytes(64));payload[0]=tag;payload[1]=rng.randrange(2);payload[2]=rng.randrange(2)
        cases.append((state,bytes(payload),bits(delta)))
    lines=['discrete '+state.hex()+' '+payload.hex()+f' {dt:08x}' for state,payload,dt in cases]
    process=subprocess.run([str(ROOT/'reconstruction/target/release/parameter-probe')],
        input='\n'.join(lines)+'\n',capture_output=True,text=True,check=True)
    actual=process.stdout.splitlines();assert len(actual)==len(cases)
    failures=[]
    for i,((state,payload,dt),result) in enumerate(zip(cases,actual)):
        original=execute(state,payload,dt)
        if original!=result:failures.append({'index':i,'input':lines[i],'original':original,'rust':result})
    report={'ok':not failures,'cases':len(cases),'source_sha256':baseline['binary']['sha256'],
        'scope':'Complete B dispatch for tags 0/1/3/4: exact replay state24, produced queue and meaningful game event bytes; preallocated queues',
        'oracle_queue_metadata_checks':'Message count, sequence and change tick checked for each of five distinct original writers',
        'excluded':['Presence JSON mapping and missing field defaults','ECS create/delete/switch/style consumers','History transactions/cursor/snapshots','Queue growth and allocation failures','SSE exception flags','A/C machine code'],
        'failure_count':len(failures),'failures':failures[:10],**candidate_provenance()}
    (ROOT/'evidence/multi-version/B-discrete-replay-verification.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n')
    print(json.dumps({'ok':report['ok'],'cases':len(cases),'failure_count':len(failures)}))
    if failures:print(json.dumps(failures[:2]));raise SystemExit(1)

if __name__=='__main__':main()
