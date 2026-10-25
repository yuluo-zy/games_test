from pathlib import Path
import gzip, hashlib, json, random, struct, subprocess, sys, tomllib
sys.path.insert(0,'C:/Users/liyu/AppData/Local/Packages/PythonSoftwareFoundation.Python.3.12_qbz5n2kfra8p0/LocalCache/local-packages/Python312/site-packages')
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import *
root=Path('D:/game/reverse-engineering/tiny-glade')
out=root/'evidence/render-recovery/parallel-batch-06/descriptor-binding'
crate=root/'reconstruction/render/work-packages/descriptor-binding-r8'
raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
digest=hashlib.sha256(raw).hexdigest();assert digest=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
h=struct.unpack_from('<I',raw,60)[0];o=h+24;base=struct.unpack_from('<Q',raw,o+24)[0]
cpu=Uc(UC_ARCH_X86,UC_MODE_64);cpu.mem_map(base,(struct.unpack_from('<I',raw,o+56)[0]+4095)&~4095)
sections=[]
for i in range(struct.unpack_from('<H',raw,h+6)[0]):
    s=o+struct.unpack_from('<H',raw,h+20)[0]+i*40
    vs,va,sz,p=struct.unpack_from('<IIII',raw,s+8);sections.append((vs,va,sz,p));cpu.mem_write(base+va,raw[p:p+sz])
data=0x20000000;stop=data+0x100;layout_api=data+0x110;update_api=data+0x120;bind_api=data+0x130
device=data+0x1000;properties=data+0x3000;resultp=data+0x4000;iterator=data+0x5000;bucket_base=data+0x8000;ctrl=data+0x9000
command=data+0xa000;pipeline=data+0xb000;set_states=data+0xc000;arena=data+0xe000;chunk=data+0xe100;offset_vec=data+0xf000;offset_data=data+0xf100
resources=data+0x10000;strings=data+0x20000;heap_start=data+0x40000;stack=data+0x3f008
cpu.mem_map(data,0x200000);state={}
def q(a):return struct.unpack('<Q',cpu.mem_read(a,8))[0]
def u(a):return struct.unpack('<I',cpu.mem_read(a,4))[0]
def s32(a):return struct.unpack('<i',cpu.mem_read(a,4))[0]
def ret(v=0):
    rsp=cpu.reg_read(UC_X86_REG_RSP);cpu.reg_write(UC_X86_REG_RAX,v&((1<<64)-1));cpu.reg_write(UC_X86_REG_RIP,q(rsp));cpu.reg_write(UC_X86_REG_RSP,rsp+8)
def allocate(n,alignment=8):
    p=(state['heap']+alignment-1)&-alignment;state['heap']=p+n;assert state['heap']<data+0x200000;cpu.mem_write(p,bytes(n));return p
def image(a):return dict(sampler=q(a),image_view=q(a+8),image_layout=s32(a+16))
def buffer(a):return dict(buffer=q(a),offset=q(a+8),range=q(a+16))
def capture_write(a):
    assert u(a)==35 and q(a+8)==0 and q(a+56)==0
    n=u(a+32);ip=q(a+40);bp=q(a+48)
    return dict(dst_set=q(a+16),dst_binding=u(a+24),dst_array_element=u(a+28),descriptor_count=n,descriptor_type=s32(a+36),images=[image(ip+24*i) for i in range(n)] if ip else [],buffers=[buffer(bp+24*i) for i in range(n)] if bp else [])
def hook(cpu,addr,size,user):
    if addr==0x140613c10:ret(allocate(cpu.reg_read(UC_X86_REG_RCX),cpu.reg_read(UC_X86_REG_RDX)))
    elif addr==0x140613c20:ret()
    elif addr==0x142923c20:
        p=cpu.reg_read(UC_X86_REG_RCX);cpu.mem_write(p,bytes([cpu.reg_read(UC_X86_REG_RDX)&255])*cpu.reg_read(UC_X86_REG_R8));ret(p)
    elif addr==0x140bfc5b0:ret(cpu.reg_read(UC_X86_REG_RDX))
    elif addr==layout_api:
        a=cpu.reg_read(UC_X86_REG_RDX);assert u(a)==32
        n=u(a+20);bp=q(a+24);ep=q(a+8);assert u(ep)==1000161000 and u(ep+16)==n
        fp=q(ep+24);bindings=[]
        for i in range(n):
            p=bp+i*24;sp=q(p+16)
            bindings.append(dict(binding=u(p),descriptor_type=s32(p+4),descriptor_count=u(p+8),stage_flags=u(p+12),immutable_sampler=q(sp) if sp else None))
        state['layout']=dict(flags=u(a+16),binding_flags=[u(fp+4*i) for i in range(n)],bindings=bindings)
        cpu.mem_write(cpu.reg_read(UC_X86_REG_R9),struct.pack('<Q',0xabc));ret()
    elif addr==0x1428d9390:state['error']='ZeroDescriptorCount';cpu.emu_stop()
    elif addr==0x1428d9518 and state.get('mode')=='sparse-bind':
        state['error']={'SlotOutOfBounds':{'slot':cpu.reg_read(UC_X86_REG_RCX)&255,'length':cpu.reg_read(UC_X86_REG_RDX)}};cpu.emu_stop()
    elif addr==0x140d4a3b0:
        a=cpu.reg_read(UC_X86_REG_RCX);cpu.mem_write(a,struct.pack('<QQQ',0,0,chunk));cpu.mem_write(0x14362c3e0,struct.pack('<Q',chunk));state['arena']=a;ret()
    elif addr in [0x1417c69c0,0x1417c6af0]:
        a=cpu.reg_read(UC_X86_REG_RCX);capacity=max(64,cpu.reg_read(UC_X86_REG_RDX));stride=4 if addr==0x1417c69c0 else 64
        p=allocate(capacity*stride);cpu.mem_write(a,struct.pack('<QQQQ',p,state['arena'],capacity,0));ret(a)
    elif addr==0x140be5a40:state['allocation_calls']+=1;ret(state['context']['descriptor_set'])
    elif addr==0x1405b0ed0:ret(0) # Public hash boundary; synthetic table controls probe placement only.
    elif addr==update_api:
        n=cpu.reg_read(UC_X86_REG_RDX)&0xffffffff;p=cpu.reg_read(UC_X86_REG_R8)
        assert cpu.reg_read(UC_X86_REG_R9)==0
        state['events'].append(dict(event='update',writes=[capture_write(p+i*64) for i in range(n)]));ret()
    elif addr==bind_api:
        rsp=cpu.reg_read(UC_X86_REG_RSP);n=u(rsp+40);sp=q(rsp+48);dn=u(rsp+56);dp=q(rsp+64)
        state['events'].append(dict(event='bind',command_buffer=cpu.reg_read(UC_X86_REG_RCX),bind_point=cpu.reg_read(UC_X86_REG_RDX)&0xffffffff,pipeline_layout=cpu.reg_read(UC_X86_REG_R8),first_set=cpu.reg_read(UC_X86_REG_R9)&0xffffffff,sets=[q(sp+8*i) for i in range(n)],offsets=[u(dp+4*i) for i in range(dn)]));ret()
cpu.hook_add(UC_HOOK_CODE,hook)
def reset():
    state.clear();state.update(heap=heap_start,events=[],allocation_calls=0)
    cpu.mem_write(data+0x1000,bytes(0x3e000));cpu.mem_write(stack,struct.pack('<Q',stop));cpu.reg_write(UC_X86_REG_RSP,stack)
    cpu.mem_write(device+0x640,struct.pack('<Q',99));cpu.mem_write(device+0x658,struct.pack('<Q',properties));cpu.mem_write(device+0x230,struct.pack('<Q',layout_api))
    # Original write constructor's bump allocator is supplied a valid nonempty chunk.
    cpu.mem_write(chunk,struct.pack('<QQQQQ',heap_start+0x10000,0,0,0,heap_start+0x20000));cpu.mem_write(arena,struct.pack('<QQQ',0,0,chunk))
    cpu.mem_write(offset_vec,struct.pack('<QQQQ',offset_data,arena,64,0))
def native_layout(case):
    reset();entries=case['entries'];n=len(entries);assert n<=32
    mask=(1<<min(n,16))-1
    cpu.mem_write(iterator,struct.pack('<QQQQQ',bucket_base,ctrl,0,mask,n))
    if n>16:cpu.mem_write(ctrl,bytes([0]*(n-16)+[255]*(32-n)))
    cpu.mem_write(properties+0x680,struct.pack('<I',case['limit']));cpu.mem_write(strings,bytes(0x10000))
    for i,b in enumerate(entries):
        p=bucket_base-48*(i+1);name=b['name'].encode();addr=strings+256*i;cpu.mem_write(addr,name+b'\0')
        blob=bytearray(48);struct.pack_into('<I',blob,0,b['index']);struct.pack_into('<QQQII',blob,8,0,b['sampler'] if b['tag']==5 else addr,len(name),int(b['runtime']),b['count']);blob[40]=b['tag'];cpu.mem_write(p,bytes(blob))
    cpu.mem_write(data+0x6000,struct.pack('<Q',data+0x6100));cpu.mem_write(stack+40,struct.pack('<Q',data+0x6000))
    for reg,v in [(UC_X86_REG_RCX,resultp),(UC_X86_REG_RDX,device),(UC_X86_REG_R8,iterator),(UC_X86_REG_R9,case['stage'])]:cpu.reg_write(reg,v)
    cpu.emu_start(0x1418c1a10,stop,count=100000)
    if 'error' in state:return {'Err':state['error']}
    assert cpu.reg_read(UC_X86_REG_RIP)==stop and q(resultp+24)==0xabc
    return {'Ok':state['layout']}
def encode_resource(p,r):
    blob=bytearray(32);struct.pack_into('<II',blob,0,r['tag'],r.get('dynamic_offset',0))
    if r['tag'] in [0,1,4]:
        x=r['images'][0];struct.pack_into('<QQi',blob,8,x['sampler'],x['image_view'],x['image_layout'])
    elif r['tag'] in [2,3]:
        ip=allocate(max(8,len(r['images'])*24));struct.pack_into('<QQI',blob,8,0,ip,len(r['images']))
        for i,x in enumerate(r['images']):cpu.mem_write(ip+i*24,struct.pack('<QQiI',x['sampler'],x['image_view'],x['image_layout'],0))
    else:
        x=r['buffer'];struct.pack_into('<QQQ',blob,8,x['buffer'],x['offset'],x['range'])
    cpu.mem_write(p,bytes(blob))
def native_write(case):
    reset();context=case['context'];w=case['writes'][0]
    encode_resource(resources,w['resource'])
    cpu.mem_write(resultp+0x100,struct.pack('<Q',context['descriptor_set']))
    cpu.mem_write(resultp+0x200,struct.pack('<QQQ',resultp+0x100,arena,offset_vec))
    cpu.mem_write(resultp+0x300,struct.pack('<IIQ',w['binding'],w['array_element'],resources))
    for reg,v in [(UC_X86_REG_RCX,resultp),(UC_X86_REG_RDX,resultp+0x200),(UC_X86_REG_R8,resultp+0x300)]:cpu.reg_write(reg,v)
    cpu.emu_start(0x140b06020,stop,count=2000)
    assert cpu.reg_read(UC_X86_REG_RIP)==stop
    return {'Ok':dict(write=capture_write(resultp),offsets=[u(offset_data+i*4) for i in range(q(offset_vec+24))])}
def native_bind(case):
    reset();c=case['context'];state['context']=c;inputs=case['writes'];count=len(inputs)
    cpu.mem_write(device+0x268,struct.pack('<Q',update_api));cpu.mem_write(device+0x328,struct.pack('<Q',bind_api));cpu.mem_write(device+0xab0,struct.pack('<Q',case['counter']))
    cpu.mem_write(command+0x48,struct.pack('<Q',c['command_buffer']))
    cpu.mem_write(pipeline+8,struct.pack('<Q',set_states));cpu.mem_write(pipeline+0x10,struct.pack('<Q',c['set_count']));cpu.mem_write(pipeline+0x20,struct.pack('<Q',c['pipeline_layout']));cpu.mem_write(pipeline+0x30,struct.pack('<I',c['bind_point']))
    keys=c['declared_bindings'];assert len(keys)<=16
    cp=ctrl+0x1000;cpu.mem_write(cp,bytes([0]*len(keys)+[255]*(16-len(keys))))
    for i,key in enumerate(keys):cpu.mem_write(cp-24*(i+1),struct.pack('<I',key)+bytes(20))
    set_state=set_states+(c['set_index'] if c['set_index']<c['set_count'] else 0)*120
    cpu.mem_write(set_state+0x18,struct.pack('<Q',cp));cpu.mem_write(set_state+0x20,struct.pack('<Q',15));cpu.mem_write(set_state+0x30,struct.pack('<Q',len(keys)))
    for i,w in enumerate(inputs):encode_resource(resources+i*32,w['resource'])
    first=case.get('first',0)
    assert [w['binding'] for w in inputs]==[(first+i)&0xffffffff for i in range(count)]
    assert all(w['array_element']==0 for w in inputs)
    cpu.mem_write(iterator,struct.pack('<QQQ',resources,resources+count*32,first))
    cpu.mem_write(stack+40,struct.pack('<Q',count));cpu.mem_write(stack+48,struct.pack('<Q',iterator))
    for reg,v in [(UC_X86_REG_RCX,device),(UC_X86_REG_RDX,command),(UC_X86_REG_R8,pipeline),(UC_X86_REG_R9,c['set_index'])]:cpu.reg_write(reg,v)
    cpu.emu_start(0x1416a5380,stop,count=100000)
    assert cpu.reg_read(UC_X86_REG_RIP)==stop,(case,state,hex(cpu.reg_read(UC_X86_REG_RIP)))
    assert state['allocation_calls']==int(c['set_index']<c['set_count'])
    return {'Ok':dict(counter=q(device+0xab0),events=state['events'])}

def native_sparse_bind(case):
    reset();c=case['context'];state['context']=c;state['mode']='sparse-bind'
    cpu.mem_write(device+0x268,struct.pack('<Q',update_api));cpu.mem_write(device+0x328,struct.pack('<Q',bind_api));cpu.mem_write(device+0xab0,struct.pack('<Q',case['counter']))
    cpu.mem_write(command+0x48,struct.pack('<Q',c['command_buffer']))
    cpu.mem_write(pipeline+8,struct.pack('<Q',set_states));cpu.mem_write(pipeline+0x10,struct.pack('<Q',c['set_count']));cpu.mem_write(pipeline+0x20,struct.pack('<Q',c['pipeline_layout']));cpu.mem_write(pipeline+0x30,struct.pack('<I',c['bind_point']))
    keys=c['declared_bindings'];assert len(keys)<=16
    cp=ctrl+0x1000;cpu.mem_write(cp,bytes([0]*len(keys)+[255]*(16-len(keys))))
    for i,key in enumerate(keys):cpu.mem_write(cp-24*(i+1),struct.pack('<I',key)+bytes(20))
    ss=set_states+(c['set_index'] if c['set_index']<c['set_count'] else 0)*120
    cpu.mem_write(ss+0x18,struct.pack('<Q',cp));cpu.mem_write(ss+0x20,struct.pack('<Q',15));cpu.mem_write(ss+0x30,struct.pack('<Q',len(keys)))
    for i,r in enumerate(case['resources']):encode_resource(resources+i*32,r)
    tuples=data+0x18000
    for i,t in enumerate(case['tuples']):cpu.mem_write(tuples+i*8,struct.pack('<IB3x',t['binding'],t['slot']))
    count=len(case['tuples']);vec=data+0x1a000
    cpu.mem_write(vec,struct.pack('<QQQ',len(case['resources']),resources,len(case['resources'])))
    cpu.mem_write(iterator,struct.pack('<QQQQQQQ',0,0,tuples if count else 0,tuples,count,tuples+count*8,vec))
    cpu.mem_write(stack+40,struct.pack('<Q',count));cpu.mem_write(stack+48,struct.pack('<Q',iterator))
    for reg,v in [(UC_X86_REG_RCX,device),(UC_X86_REG_RDX,command),(UC_X86_REG_R8,pipeline),(UC_X86_REG_R9,c['set_index'])]:cpu.reg_write(reg,v)
    cpu.emu_start(0x1416a6010,stop,count=100000)
    if 'error' in state:return {'Err':state['error']}
    assert cpu.reg_read(UC_X86_REG_RIP)==stop,(case,state,hex(cpu.reg_read(UC_X86_REG_RIP)))
    return {'Ok':dict(counter=q(device+0xab0),events=state['events'])}

rng=random.Random(20261013);cases=[];groups=[]
def add(group,case):cases.append(case);groups.append(group)
for tag in range(6):
    for index in [0,3,23,8192,0xffffffff]:
        for runtime in [False,True]:
            for limit in [0,32,33,34,8192,16384]:
                for count in [0,1,2,7,0xffffffff]:
                    add('sparse-layout',dict(op='layout',stage=32,limit=limit,entries=[dict(index=index,tag=tag,name='buffer_dyn' if tag<2 else 'tex',runtime=runtime,count=count,sampler=0x1234)]))
for n in [0,2,3,8,16,17,32]:
    for i in range(25):
        entries=[dict(index=rng.getrandbits(32),tag=rng.randrange(6),name=rng.choice(['u','buffer_dyn']),runtime=bool(rng.randrange(2)),count=rng.choice([1,2,7]),sampler=rng.getrandbits(64)) for _ in range(n)]
        if i==0 and n>1:entries[-1]['index']=entries[0]['index'] # malformed duplicate raw iterator, caller reachability excluded
        add('sparse-layout',dict(op='layout',stage=rng.getrandbits(32),limit=16384,entries=entries))
def context(keys,first_set=0,set_count=4):return dict(command_buffer=0x778899,bind_point=1,pipeline_layout=0x223344,set_index=first_set,set_count=set_count,descriptor_set=0x12345678,declared_bindings=keys)
def resource(tag):
    images=[dict(sampler=rng.getrandbits(64),image_view=rng.getrandbits(64),image_layout=rng.choice([1,5,1000117000])) for _ in range(rng.choice([0,1,2,5]) if tag in [2,3] else 1)] if tag in [0,1,2,3,4] else []
    return dict(tag=tag,images=images,buffer=dict(buffer=rng.getrandbits(64),offset=rng.getrandbits(64),range=rng.getrandbits(64)) if tag in [5,6,8,9] else None,dynamic_offset=rng.getrandbits(32))
for tag in [0,1,2,3,4,5,6,8,9]:
    for i in range(96):
        key=rng.getrandbits(32);w=dict(binding=key,array_element=rng.getrandbits(32),resource=resource(tag))
        add('write-constructor',dict(op='write',writes=[w],context=context([key])))
for count in [0,1,2,4,8]:
    for first in [0,3,0xfffffffe,0xffffffff]:
        for set_index in [0,1,3,4,0xffffffff]:
            for selected in ['all','alternate','none']:
                writes=[dict(binding=(first+i)&0xffffffff,array_element=0,resource=resource(rng.choice([0,1,2,3,4,5,6,8,9]))) for i in range(count)]
                keys=[w['binding'] for i,w in enumerate(writes) if selected=='all' or selected=='alternate' and i%2==0]
                add('update-bind-entry',dict(op='bind',first=first,writes=writes,context=context(keys,set_index),counter=rng.choice([0,7,0xffffffffffffffff])))
# Original membership consumer tests only the key. Deliberately wrong reflected
# class metadata is retained as fixture annotation; no class check is invented.
for tag in [0,1,5,6,8,9]:
    add('reflection-resource-class-mismatch',dict(op='bind',first=23,writes=[dict(binding=23,array_element=0,resource=resource(tag))],context=context([23]),counter=0,declaration_types={'23':'opposite buffer/image class'}))
native=[]
for count in [0,1,2,5,12]:
    for set_index in [0,1,3,4]:
        for mode in ['all','holes','duplicate','badslot']:
            for repeat in range(8):
                rs=[resource(rng.choice([0,1,2,3,4,5,6,8,9])) for _ in range(count)]
                ts=[dict(binding=rng.getrandbits(32),slot=count-i-1) for i in range(count)]
                if mode=='duplicate' and count>1:ts[-1]['binding']=ts[0]['binding']
                if mode=='badslot' and count:ts[-1]['slot']=255
                keys=list(dict.fromkeys(t['binding'] for i,t in enumerate(ts) if mode!='holes' or i%2==0))
                add('compute-sparse-tuple-bind',dict(op='sparse-bind',resources=rs,tuples=ts,context=context(keys,set_index),counter=rng.choice([0,7,0xffffffffffffffff])))
for case in cases:native.append(native_layout(case) if case['op']=='layout' else native_write(case) if case['op']=='write' else native_sparse_bind(case) if case['op']=='sparse-bind' else native_bind(case))
subprocess.run(['cargo','build','--release','--locked','--offline','--manifest-path',str(crate/'Cargo.toml')],check=True)
run=subprocess.run([str(crate/'target/release/tiny-glade-descriptor-binding-r8.exe')],input='\n'.join(json.dumps(c) for c in cases)+'\n',capture_output=True,text=True,check=True)
rust=[json.loads(x) for x in run.stdout.splitlines()];assert len(rust)==len(native)
failures=[dict(index=i,group=g,input=c,native=n,rust=r) for i,(g,c,n,r) in enumerate(zip(groups,cases,native,rust)) if n!=r]
with gzip.open(out/'native-oracle.jsonl.gz','wt',encoding='utf8') as f:
    for g,c,n in zip(groups,cases,native):f.write(json.dumps(dict(group=g,input=c,native=n))+'\n')
original=tomllib.loads((root/'tracking/original-Cargo.lock').read_text(encoding='utf8'))['package'];locked=tomllib.loads((crate/'Cargo.lock').read_text(encoding='utf8'))['package']
dependencies=[]
for p in locked:
    if 'source' not in p:continue
    assert any(all(p.get(k)==o.get(k) for k in ['name','version','source','checksum']) for o in original),p
    dependencies.append({k:p[k] for k in ['name','version','source','checksum']})
(out/'dependency-verification.json').write_text(json.dumps(dict(packages=dependencies,matching_original=True),indent=2),encoding='utf8')
counts={g:groups.count(g) for g in sorted(set(groups))}
report=dict(binary_sha256=digest,cases=len(cases),groups=counts,mismatches=len(failures),failures=failures[:10],source_sha256=hashlib.sha256((crate/'src/lib.rs').read_bytes()).hexdigest(),original_entries={'sparse_layout':'0x1418c1a10','write_constructor':'0x140b06020','update_bind':'0x1416a5380','sparse_tuple_bind':'0x1416a6010'},scope='Original complete CPU entries with mock public hash/allocator/BumpVec/TLS arena, external descriptor-set allocation and actual Vulkan API parameter capture. Sparse tuple bind includes actual first graph-resource tuple mapper; second persistent chained iterator excluded. Raw iterator duplicate layout fixtures are malformed and do not prove caller reachability. Acceleration structures, allocation failures, descriptor validation feature limits, registry lifetimes and real GPU excluded.')
(out/'native-validation.json').write_text(json.dumps(report,indent=2),encoding='utf8');print(json.dumps(report,indent=2));assert not failures
