"""Joint selected original begin/bucket-wrapper/track/drop/query/reclaim entries.
Selected entry bodies are unchanged; intercepted callees and manually produced
queue success / waste attachment effects remain explicit gaps. This is a CPU
composition oracle, not an original full submit-to-destruction chain.
No database or game process is opened.
"""
from pathlib import Path
import gzip,hashlib,itertools,json,struct,subprocess,sys
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *

OUT=Path(__file__).resolve().parent;ROOT=OUT.parents[3]
SOURCE=ROOT/'reconstruction/render/work-packages/sequence-runtime-r8'
TARGET=ROOT/'workspace/input/tiny-glade.exe'
SHA='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
RAW=TARGET.read_bytes();assert hashlib.sha256(RAW).hexdigest()==SHA
header=struct.unpack_from('<I',RAW,60)[0];optional=header+24
image_base=struct.unpack_from('<Q',RAW,optional+24)[0]
cpu=Uc(UC_ARCH_X86,UC_MODE_64);size=struct.unpack_from('<I',RAW,optional+56)[0];cpu.mem_map(image_base,(size+4095)&~4095)
for i in range(struct.unpack_from('<H',RAW,header+6)[0]):
    h=optional+struct.unpack_from('<H',RAW,header+20)[0]+40*i
    _,address,length,offset=struct.unpack_from('<IIII',RAW,h+8)
    if length:cpu.mem_write(image_base+address,RAW[offset:offset+length])
STACK=0x20000000;STOP=0x30000000;MEMORY=0x40000000
cpu.mem_map(STACK,0x20000);cpu.mem_map(STOP,0x1000);cpu.mem_map(MEMORY,0x40000)
DEVICE=MEMORY;INNER=MEMORY+0x2000;COUNTERS=MEMORY+0x3000;COMMANDS=MEMORY+0x4000
QUEUES=[MEMORY+0x6000,MEMORY+0x8000];PENDINGS=[MEMORY+0xa000,MEMORY+0xc000];TIMELINES=[MEMORY+0xe000,MEMORY+0xf000];MARKERS=[MEMORY+0x10000,MEMORY+0x11000]
POOL_DESC=MEMORY+0x12000;POOL_DATA=MEMORY+0x13000;CONTROL=MEMORY+0x18000;HANDLES=MEMORY+0x1a000;WASTE=MEMORY+0x1c000;WASTE_DATA=MEMORY+0x1d000
state={}
def read(a,fmt):return struct.unpack(fmt,bytes(cpu.mem_read(a,struct.calcsize(fmt))))
def write(a,fmt,*v):cpu.mem_write(a,struct.pack(fmt,*v))
def ret(value=None):
    p=cpu.reg_read(UC_X86_REG_RSP);address=read(p,'<Q')[0];cpu.reg_write(UC_X86_REG_RSP,p+8);cpu.reg_write(UC_X86_REG_RIP,address)
    if value is not None:cpu.reg_write(UC_X86_REG_RAX,value)
def hook(machine,address,length,user):
    state['visited'].add(hex(address))
    if address==STOP:machine.emu_stop();return
    if address==STOP+0x100:ret(int(state['fail']==1));return
    if address==STOP+0x200:ret(int(state['fail']==2));return
    if address==STOP+0x300:ret();return
    if address==STOP+0x400:
        write(machine.reg_read(UC_X86_REG_R8),'<Q',state['query']);ret(0);return
    if address in [0x140c16a00]:ret(0);return
    if address in [0x1418c1380,0x140c81c70,0x140c81e10,0x140bdfcf0,0x140bdef60]:ret();return
    if address==0x1418adf90:
        sequence=machine.reg_read(UC_X86_REG_R8);present=sequence in state['buckets'];state['buckets'][sequence]=[]
        state['events'].append(f'insert:{sequence}:{int(present)}');write(machine.reg_read(UC_X86_REG_RCX),'<Q',int(present));ret();return
    if address==0x140bed31d:
        command=machine.reg_read(UC_X86_REG_RBX);tag,seq,q,aux=read(command,'<QQII');handle=read(command+0x48,'<Q')[0]
        state['events'].append(f'begin:{handle}:{seq}:{q}:{aux}')
    if address==0x1418c4100:
        sequence=machine.reg_read(UC_X86_REG_R8)
        if sequence not in state['buckets']:state['panic']='MissingBucket';machine.emu_stop();return
        state['draining']=state['buckets'].pop(sequence);state['events'].append(f'remove:{sequence}');ret();return
    if address==0x140bf92f0:
        for resource in state['draining']:state['events'].append(f'dispose:{resource}')
        ret();return
    if address==0x1418c3f40:
        # Public VecDeque::resize semantics only; the private offset/marker branch runs.
        q=machine.reg_read(UC_X86_REG_RCX);requested=machine.reg_read(UC_X86_REG_RDX);cap,pointer,head,length=read(q,'<QQQQ')
        assert requested<cap
        for i in range(length,requested):write(pointer+8*((head+i)%cap),'<Q',0)
        write(q+0x18,'<Q',requested);ret();return
    if address==0x140bf1006:
        vector=machine.reg_read(UC_X86_REG_RAX);index=machine.reg_read(UC_X86_REG_RDX);sequence=read(vector+96*index+8,'<Q')[0];state['events'].append(f'pop:{sequence}')
    if address==0x140bf1926:
        vector=machine.reg_read(UC_X86_REG_RBX);pointer,length=read(vector+8,'<QQ');sequence=read(pointer+(length-1)*88+8,'<Q')[0];state['events'].append(f'pool:{sequence}')
    if address in [0x1428d9430,0x1428d9390,0x1428d9760,0x1428d9518,0x1428ca851,0x1428d95d4,0x1428ca88c]:
        if state['operation']=='begin':
            state['panic']={1:'BackendReset',2:'BackendBegin',3:'InvalidBeginState',4:'CounterBounds'}.get(state['fail'],'DuplicateBucket')
        elif address==0x1428ca851:state['panic']='DuplicateReadyMarker'
        elif state['fail']==5:state['panic']='InvalidPendingState'
        elif state['fail']==6:state['panic']='QueueMismatch'
        elif state['fail']==9:state['panic']='PoolFamilyBounds'
        else:state['panic']='SequenceAlreadyDisposed'
        state['panic_address']=hex(address);machine.emu_stop();return
cpu.hook_add(UC_HOOK_CODE,hook)
def call(entry,rcx,rdx=0,r8=0,r9=0):
    stack=STACK+0x10008;write(stack,'<Q',STOP)
    for reg,value in [(UC_X86_REG_RSP,stack),(UC_X86_REG_RCX,rcx),(UC_X86_REG_RDX,rdx),(UC_X86_REG_R8,r8),(UC_X86_REG_R9,r9)]:cpu.reg_write(reg,value)
    try:cpu.emu_start(entry,STOP,count=500000)
    except Exception as e:raise RuntimeError(f'entry={entry:#x} rip={cpu.reg_read(UC_X86_REG_RIP):#x} {e}') from e
    assert state['panic']!='ok' or cpu.reg_read(UC_X86_REG_RIP)==STOP,hex(cpu.reg_read(UC_X86_REG_RIP))
def setup(fail):
    cpu.mem_write(MEMORY,bytes(0x40000));cpu.mem_write(STACK,bytes(0x20000))
    state.clear();state.update(fail=fail,panic='ok',panic_address=None,events=[],buckets={},draining=[],visited=set(),operation='begin',query=0)
    write(DEVICE+0x658,'<Q',INNER);write(DEVICE+0x668,'<QQ',COUNTERS,2);write(COUNTERS,'<QQ',1,1)
    for offset,function in [(0x2d0,STOP+0x100),(0x2c0,STOP+0x200),(0x320,STOP+0x300),(0x478,STOP+0x400)]:write(DEVICE+offset,'<Q',function)
    write(DEVICE+0x9c0,'<QQ',POOL_DESC,1);write(POOL_DESC,'<QQQ',64,POOL_DATA,0)
    write(WASTE+8,'<QQQ',64,WASTE_DATA,0)
    for i,q in enumerate(QUEUES):
        write(q,'<QQQQ',128,TIMELINES[i],0,0);write(q+0x20,'<QQQQ',64,PENDINGS[i],0,0)
        write(q+0x40,'<QQQQQ',128,MARKERS[i],0,0,0);write(q+0x68,'<II',i,0)
def collect(resources):
    for id in resources:
        sequence=read(CONTROL+id*256,'<Q')[0]
        if sequence in state['buckets']:state['buckets'][sequence].append(id);state['events'].append(f'attach:{id}:{sequence}')
        else:state['events'].append(f'dispose:{id}')
    write(WASTE+0x18,'<Q',0)
def native(case):
    count,order,mode,fail,stage,late,resources=case
    setup(fail);commands=[];failed='none';handles=[];qid=1 if mode==1 else 0
    for i in range(count):
        command=COMMANDS+i*88;write(command+0x48,'<Q',100+i)
        if fail==3:write(command,'<Q',2)
        q=1 if (mode==2 and i==1) or fail==4 else qid;aux=99 if fail==4 else 0
        call(0x140bed230,command,DEVICE,q,aux)
        if state['panic']!='ok':failed=':'.join(map(str,read(command,'<QQ')));break
        commands.append(bytes(cpu.mem_read(command,88)))
    if state['panic']=='ok':
        for id in range(resources):
            control=CONTROL+id*256;handle=HANDLES+id*32;handles.append(id)
            write(control,'<QII',0,1,7);write(handle,'<QQIHBB',WASTE,control,7,9,1,0)
            for i in range(count):
                call(0x140bd6a70,handle,COMMANDS+i*88)
                state['events'].append(f'track:{id}:{read(control,"<Q")[0]}:0')
            state['events'].append(f'cpu-retire:{id}')
            if late==0:
                call(0x140be24b0,handle);state['events'].append(f'drop:{id}:{read(control+8,"<I")[0]}')
        if late==0:collect(handles)
        q=QUEUES[qid]
        for i,index in enumerate(order):
            command=bytearray(commands[index-1]);struct.pack_into('<Q',command,0,2 if fail==5 else 3)
            if fail==6:struct.pack_into('<I',command,16,1-qid)
            if fail==9:struct.pack_into('<I',command,0x50,99)
            threshold=41+i;cpu.mem_write(PENDINGS[qid]+i*96,bytes(command)+struct.pack('<Q',threshold));write(TIMELINES[qid]+i*8,'<Q',threshold)
            state['events'].append(f'submit:{read(COMMANDS+(index-1)*88+8,"<Q")[0]}:{threshold}')
        write(q+0x18,'<Q',len(order));write(q+0x38,'<Q',len(order))
        values=[v for i in range(len(order)) for v in [40+i,41+i]] if stage==0 else [40+len(order)]
        state['operation']='reclaim'
        for value in values:
            state['query']=value;state['events'].append(f'complete:{qid}:0:{value}');call(0x140bf0b90,q,DEVICE)
            call(0x140bf0cc0,q,DEVICE)
            if state['panic']!='ok':break
        if late==1 and state['panic']=='ok':
            for id in handles:call(0x140be24b0,HANDLES+id*32);state['events'].append(f'drop:{id}:{read(CONTROL+id*256+8,"<I")[0]}')
            collect(handles)
    def numbers(v):return ','.join(map(str,v))
    queues=[]
    for q in QUEUES:
        cap,pointer,head,length,prefix=read(q+0x40,'<QQQQQ');markers=[read(pointer+8*((head+i)%cap),'<Q')[0] for i in range(length)]
        queues.append(f'{prefix}:{numbers(markers)}:{read(q+0x38,"<Q")[0]}')
    length=read(POOL_DESC+16,'<Q')[0];pool=[read(POOL_DATA+i*88+8,'<Q')[0] for i in range(length)]
    resource_states=[f'{read(CONTROL+i*256,"<Q")[0]}:{read(CONTROL+i*256+8,"<I")[0]}' for i in handles]
    result='|'.join([state['panic'],';'.join(state['events']),numbers(read(COUNTERS,'<QQ')),'/'.join(queues),numbers(sorted(state['buckets'])),numbers(pool),','.join(resource_states),failed])
    return result,{'case':case,'output':result,'panic_address':state['panic_address'],'original_entries_visited':[v for v in ['0x140bed230','0x1418c4260','0x140bd6a70','0x140be24b0','0x140bf0b90','0x140bf0cc0','0x1418c7ae0'] if v in state['visited']],'visited':sorted(state['visited']),'pool_sequences':pool,'remaining_buckets':state['buckets']}
def lines(cases):return ''.join(f'{n} {",".join(map(str,order))} {mode} {fail} {stage} {late} {resources}\n' for n,order,mode,fail,stage,late,resources in cases)
def main():
    cases=[]
    for n in [1,2,3,4]:
        for order in itertools.permutations(range(1,n+1)):
            for mode,stage,late,resources in itertools.product([0,1],[0,1],[0,1],[0,1,2]):cases.append([n,list(order),mode,0,stage,late,resources])
    for fail in [1,2,3,4,5,6,9]:
        for mode,stage,late in itertools.product([0,1],[0,1],[0,1]):cases.append([2,[2,1],mode,fail,stage,late,1])
    for mode in [0,1]:
        for order in [[2,2,1],[1,1,2]]:
            for stage,late,resources in itertools.product([0,1],[0,1],[0,1,2]):cases.append([2,order,mode,0,stage,late,resources])
    for stage,late,resources in itertools.product([0,1],[0,1],[0,1,2]):cases.append([2,[1,2],2,0,stage,late,resources])
    if '--smoke' in sys.argv:cases=[[1,[1],0,0,0,0,1],[2,[2,1],0,0,0,0,1],[2,[2,1],1,0,0,0,1],[2,[1,2],2,0,0,0,1],[2,[2,1],0,5,0,0,1]]
    input_text=lines(cases)
    (OUT/'native-inputs.txt').write_bytes(input_text.encode())
    observations=[native(case)[1] for case in cases]
    build=subprocess.run(['cargo','build','--release','--offline','--locked','--manifest-path',str(SOURCE/'Cargo.toml')],capture_output=True,text=True);assert build.returncode==0,build.stderr
    (OUT/'build.log').write_text(build.stdout+build.stderr,encoding='utf8')
    executable=SOURCE/'target/release/tiny-glade-sequence-runtime-r8.exe'
    rust=subprocess.run([str(executable)],input=input_text,capture_output=True,text=True,check=True).stdout.splitlines()
    failures=[{'case':o['case'],'native':o['output'],'candidate':r} for o,r in zip(observations,rust) if o['output']!=r]
    report={'passed':not failures and len(rust)==len(cases),'cases':len(cases),'failure_count':len(failures),'failures':failures[:10],'target_sha256':SHA,'new_original_joint_cases':True,'recounts_old_2840':False,'observations':observations,'limitations':['CPU instruction emulation, no actual Vulkan/GPU','original begin/bucket insert wrapper/handle track/drop/timeline query/reclaim/sequential disposal are unchanged; standard map insert/remove and VecDeque resize are fixture semantics','pending command tag3 and thresholds are injected queue success effects, not full queue_submit ABI validation','CPU resource retire and bucket attachment/destruction callbacks are compositional captured effects; resource destructor implementation not exercised','synthetic order/collision/error fixtures do not prove private caller reachability','source atomic memory order and multithread scheduling unknown']}
    report['evidence_class']='selected-original-entries-with-explicit-private-cutpoints'
    report['coverage']={entry:sum(entry in o['original_entries_visited'] for o in observations) for entry in ['0x140bed230','0x1418c4260','0x140bd6a70','0x140be24b0','0x140bf0b90','0x140bf0cc0','0x1418c7ae0']}
    report['hooks']={
        '0x30000100':'injected Vulkan command reset result',
        '0x30000200':'injected Vulkan command begin result',
        '0x30000300':'injected external command metadata/debug operation',
        '0x30000400':'injected Vulkan get semaphore counter value',
        '0x140c16a00':'fixed false guard; excluded debug/profiling path',
        '0x1418c1380':'private telemetry/event sink suppressed',
        '0x140c81c70':'callee suppressed; identity/side effects outside selected scope',
        '0x140c81e10':'callee suppressed; identity/side effects outside selected scope',
        '0x140bdfcf0':'callee suppressed; identity/side effects outside selected scope',
        '0x140bdef60':'callee suppressed if reached; identity outside selected scope',
        '0x1418adf90':'generic map insert boundary: replace and return occupancy',
        '0x1418c4100':'generic map remove boundary: transfer bucket contents',
        '0x140bf92f0':'private resource destruction callback replaced by captured dispose effects',
        '0x1418c3f40':'public VecDeque resize semantics with fixed capacity',
    }
    paths=[OUT/'verify.py',OUT/'native-inputs.txt',OUT/'update-pending.asm.txt',OUT/'reclaim.asm.txt']
    for package in ['sequence-runtime-r8','device-counters-r6','gpu-lifecycle-r3','record-resources-r7','queue-callers-r7','barrier-record-r5']:
        base=SOURCE.parent/package
        paths.extend(base/name for name in ['Cargo.toml','Cargo.lock'] if (base/name).exists())
        paths.extend(sorted(base.glob('*.rs')))
        paths.extend(sorted((base/'src').rglob('*.rs')))
    report['input_sha256']={str(p.relative_to(ROOT)).replace('\\','/'):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}
    report['executable_sha256']=hashlib.sha256(executable.read_bytes()).hexdigest()
    report['trace_hook_reachability']={address:sum(address in o['visited'] for o in observations) for address in report['hooks']}
    with gzip.open(OUT/'native-observations.jsonl.gz','wt',encoding='utf8') as stream:
        for observation in observations:stream.write(json.dumps(observation,separators=(',',':'))+'\n')
    report['trace_file']='native-observations.jsonl.gz'
    report['trace_sha256']=hashlib.sha256((OUT/'native-observations.jsonl.gz').read_bytes()).hexdigest()
    for observation in observations:
        pcs=observation.pop('visited')
        observation['visited_unique_count']=len(pcs)
        observation['visited_sha256']=hashlib.sha256(json.dumps(pcs,separators=(',',':')).encode()).hexdigest()
    (OUT/'native-validation.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf8')
    print(json.dumps({k:report[k] for k in ['passed','cases','failure_count','failures']}));assert report['passed']
if __name__=='__main__':main()
