from pathlib import Path
import gzip, hashlib, json, random, struct, subprocess, sys, tomllib
sys.path.insert(0, 'C:/Users/liyu/AppData/Local/Packages/PythonSoftwareFoundation.Python.3.12_qbz5n2kfra8p0/LocalCache/local-packages/Python312/site-packages')
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import *
root = Path('D:/game/reverse-engineering/tiny-glade')
out = root/'evidence/render-recovery/parallel-batch-05/pipeline-descriptors'
crate = root/'reconstruction/render/work-packages/pipeline-descriptors-r7'
raw = Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
digest = hashlib.sha256(raw).hexdigest()
assert digest == 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
h = struct.unpack_from('<I', raw, 60)[0]; o=h+24
base = struct.unpack_from('<Q',raw,o+24)[0]
cpu=Uc(UC_ARCH_X86,UC_MODE_64)
cpu.mem_map(base,(struct.unpack_from('<I',raw,o+56)[0]+4095)&~4095)
for i in range(struct.unpack_from('<H',raw,h+6)[0]):
    s=o+struct.unpack_from('<H',raw,h+20)[0]+40*i
    vs,va,sz,ptr=struct.unpack_from('<IIII',raw,s+8)
    cpu.mem_write(base+va,raw[ptr:ptr+sz])
data=0x20000000; heap=data+0x20000; stack=data+0x1f000
cpu.mem_map(data,0x100000)
stop=data+0x100; api=data+0x110
cpu.mem_write(stop,b'\xc3');cpu.mem_write(api,b'\xc3')
state={}
def q(addr): return struct.unpack('<Q',cpu.mem_read(addr,8))[0]
def u(addr): return struct.unpack('<I',cpu.mem_read(addr,4))[0]
def ret(value=0):
    rsp=cpu.reg_read(UC_X86_REG_RSP)
    cpu.reg_write(UC_X86_REG_RAX,value & ((1<<64)-1))
    cpu.reg_write(UC_X86_REG_RIP,q(rsp))
    cpu.reg_write(UC_X86_REG_RSP,rsp+8)
def hook(cpu,addr,size,user):
    if addr==data+0x140:
        p=cpu.reg_read(UC_X86_REG_RDX);n=u(p+20);lp=q(p+24);rn=u(p+32);rp=q(p+40)
        state['compute']=dict(s_type=u(p),flags=u(p+16),layouts=[q(lp+i*8) for i in range(n)],ranges=[list(struct.unpack('<III',cpu.mem_read(rp+i*12,12))) for i in range(rn)])
        cpu.emu_stop()
    elif addr==0x1418c2150 and state.get('mode')=='pool':
        plan=state['input_layout'];p=cpu.reg_read(UC_X86_REG_RCX);bp=state['heap'];state['heap']+=len(plan['bindings'])*24
        for i,b in enumerate(plan['bindings']):cpu.mem_write(bp+i*24,struct.pack('<IIIIQ',b['binding'],b['descriptor_type'],b['descriptor_count'],b['stage_flags'],0))
        cpu.mem_write(p,struct.pack('<QQQQ',len(plan['bindings']),bp,len(plan['bindings']),0xabcdef));ret(p)
    elif addr==0x140562e70 and state.get('mode')=='pool':
        p=cpu.reg_read(UC_X86_REG_RCX);capacity=q(p);pointer=q(p+8);length=q(p+16)
        new_capacity=max(4,capacity*2);bp=state['heap'];state['heap']+=new_capacity*8
        if length:cpu.mem_write(bp,bytes(cpu.mem_read(pointer,length*8)))
        cpu.mem_write(p,struct.pack('<QQQ',new_capacity,bp,length));ret()
    elif addr==data+0x120:
        p=cpu.reg_read(UC_X86_REG_RDX);assert u(p)==33
        state['pool']=dict(flags=u(p+16),max_sets=u(p+20),sizes=[list(struct.unpack('<II',cpu.mem_read(q(p+32)+i*8,8))) for i in range(u(p+24))])
        cpu.mem_write(cpu.reg_read(UC_X86_REG_R9),struct.pack('<Q',0x1234));ret()
    elif addr==data+0x130:
        p=cpu.reg_read(UC_X86_REG_RDX);assert u(p)==34 and q(p+16)==0x1234 and u(p+24)==1 and q(q(p+32))==0xabcdef
        ep=q(p+8);assert u(ep)==1000161003 and u(ep+16)==1
        state['pool']['variable_descriptor_count']=u(q(ep+24))
        cpu.mem_write(cpu.reg_read(UC_X86_REG_R8),struct.pack('<Q',0x112233));ret()
    elif addr==0x1418aead0 and state.get('mode')=='pool':ret(data+0x9000)
    elif addr==0x140bde2a0 and state.get('mode')=='pool':ret()
    elif addr==stop: state['returned']=True;cpu.emu_stop()
    elif addr==0x140613c10:
        n=cpu.reg_read(UC_X86_REG_RCX);alignment=cpu.reg_read(UC_X86_REG_RDX)
        pointer=(state['heap']+alignment-1)&-alignment
        state['heap']=pointer+n
        assert state['heap']<data+0x100000
        cpu.mem_write(pointer,bytes(n));ret(pointer)
    elif addr==0x140613c20: ret()
    elif addr==0x142923c20:
        pointer=cpu.reg_read(UC_X86_REG_RCX);value=cpu.reg_read(UC_X86_REG_RDX)&255;n=cpu.reg_read(UC_X86_REG_R8)
        cpu.mem_write(pointer,bytes([value])*n);ret(pointer)
    elif addr==0x140bfc5b0:
        sampler=cpu.reg_read(UC_X86_REG_RDX)
        state['sampler_calls'].append(sampler);ret(sampler)
    elif addr==api:
        pointer=cpu.reg_read(UC_X86_REG_RDX)
        assert u(pointer)==32
        flags=u(pointer+16);n=u(pointer+20);bp=q(pointer+24);ep=q(pointer+8)
        assert u(ep)==1000161000 and u(ep+16)==n
        fp=q(ep+24)
        bindings=[]
        for i in range(n):
            a=bp+i*24;sp=q(a+16)
            bindings.append(dict(binding=u(a),descriptor_type=u(a+4),descriptor_count=u(a+8),stage_flags=u(a+12),immutable_sampler=q(sp) if sp else None))
        state['plan']=dict(flags=flags,binding_flags=[u(fp+4*i) for i in range(n)],bindings=bindings)
        cpu.mem_write(cpu.reg_read(UC_X86_REG_R9),struct.pack('<Q',0xabcdef))
        ret(state['vk_result'])
    elif addr in [0x1428d9390,0x1428d9760]:
        state['error']='ZeroDescriptorCount' if addr==0x1428d9390 else {'Vulkan':state['vk_result']}
        cpu.emu_stop()
cpu.hook_add(UC_HOOK_CODE,hook)
def native(case,vk_result=0):
    state.clear();state.update(heap=heap,sampler_calls=[],vk_result=vk_result)
    cpu.mem_write(data+0x1000,bytes(0x10000))
    device=data+0x1000;properties=data+0x3000;it=data+0x4000;inp=data+0x5000;strings=data+0x10000;result=data+0x6000;scratch=data+0x7000
    cpu.mem_write(device+0x658,struct.pack('<Q',properties));cpu.mem_write(properties+0x680,struct.pack('<I',case['limit']))
    cpu.mem_write(device+0x640,struct.pack('<Q',99));cpu.mem_write(device+0x230,struct.pack('<Q',api))
    cpu.mem_write(it,struct.pack('<QQQ',inp,inp+40*len(case['bindings']),case['first']))
    cpu.mem_write(scratch,struct.pack('<Q',scratch+16))
    for i,b in enumerate(case['bindings']):
        text=b['name'].encode();addr=strings+i*256;cpu.mem_write(addr,text+b'\0')
        blob=bytearray(40)
        struct.pack_into('<QQQII',blob,0,0,b['sampler'] if b['tag']==5 else addr,len(text),int(b['runtime']),b['count'])
        blob[32]=b['tag'];cpu.mem_write(inp+40*i,bytes(blob))
    cpu.mem_write(stack,struct.pack('<Q',stop));cpu.mem_write(stack+40,struct.pack('<Q',scratch))
    for reg,val in [(UC_X86_REG_RSP,stack),(UC_X86_REG_RCX,result),(UC_X86_REG_RDX,device),(UC_X86_REG_R8,it),(UC_X86_REG_R9,case['stage'])]: cpu.reg_write(reg,val)
    cpu.emu_start(0x1418c2150,stop,count=100000)
    if 'error' in state: return {'Err':state['error']}
    assert cpu.reg_read(UC_X86_REG_RIP)==stop and q(result+24)==0xabcdef,(case,state,hex(cpu.reg_read(UC_X86_REG_RIP)),hex(q(result+24)))
    assert len(state['sampler_calls'])==sum(b['tag']==5 for b in case['bindings'])
    return {'Ok':state['plan']}
cases=[]
for tag in range(6):
    for runtime in [False,True]:
        for limit in [0,31,32,33,34,64,8223,8224,8225,0xffffffff]:
            for name in ['', 'a', '_dyn', 'buffer_dyn', 'x_dynx', 'é_dyn']:
                for count in [0,1,2,8192,0xffffffff]:
                    cases.append(dict(first=0xffffffff,stage=0x20,limit=limit,bindings=[dict(tag=tag,runtime=runtime,count=count,name=name,sampler=0x1234)]))
random_source=random.Random(20261012)
for i in range(200):
    bindings=[dict(tag=random_source.randrange(6),runtime=bool(random_source.randrange(2)),count=random_source.choice([0,1,2,7]),name=random_source.choice(['b','b_dyn']),sampler=random_source.getrandbits(64)) for _ in range(random_source.randrange(12))]
    cases.append(dict(first=random_source.getrandbits(64),stage=random_source.getrandbits(32),limit=random_source.choice([0,33,100,16384]),bindings=bindings))
catalog=json.loads((root/'evidence/render-recovery/parallel-batch-04/pipeline-shader-interfaces/catalog.json').read_text())
tags={'StorageBuffer':0,'UniformBuffer':1,'CombinedImageSampler':2,'StorageImage':3,'SampledImage':4,'Sampler':5}
stages={'GLCompute':32,'Vertex':1,'Fragment':16}
catalog_cases=0
for file in catalog['files']:
    for module in file['modules']:
        for entry in module['entries']:
            # Enumeration is the actual persistent-layout entry; original sparse map layout is not claimed.
            bindings=[dict(tag=tags[d['descriptor_type'].split('(')[0]],runtime=d['count']==0,count=d['count'],name=d.get('name') or '',sampler=0x12345678) for d in entry['descriptors']]
            assert max([len(b['name'].encode()) for b in bindings] or [0])<256
            cases.append(dict(first=0,stage=32,limit=16384,bindings=bindings));catalog_cases+=1
native_results=[native(c) for c in cases]
subprocess.run(['cargo','build','--release','--locked','--offline','--manifest-path',str(crate/'Cargo.toml')],check=True)
run=subprocess.run([str(crate/'target/release/tiny-glade-pipeline-descriptors-r7.exe')],input='\n'.join(json.dumps(c) for c in cases)+'\n',text=True,capture_output=True,check=True)
candidate_results=[json.loads(x) for x in run.stdout.splitlines()]
assert len(candidate_results)==len(native_results)
failures=[dict(index=i,input=c,native=a,rust=b) for i,(c,a,b) in enumerate(zip(cases,native_results,candidate_results)) if a!=b]
with gzip.open(out/'native-layout-oracle.jsonl.gz','wt',encoding='utf8') as f:
    for c,n in zip(cases,native_results):f.write(json.dumps(dict(input=c,native=n))+'\n')
error_cases=[]
for error in [-1,-2,-3,-13]:
    c=dict(first=0,stage=32,limit=16384,bindings=[])
    assert native(c,error)=={'Err':{'Vulkan':error}}
    error_cases.append(error)
boundary_inputs=[];boundary_native=[]
for n in [0,1,2,4]:
    for push in [0,4,8,12,16,20,24,0xffffffff]:
        for repeat in range(8):
            layouts=[random_source.getrandbits(64) for _ in range(n)]
            state.clear();device=data+0x1000;frame=data+0x10000;reflection=data+0x5000;lp=data+0x6000
            cpu.mem_write(device+0x210,struct.pack('<Q',data+0x140));cpu.mem_write(device+0x640,struct.pack('<Q',99))
            cpu.mem_write(frame+0xb0,struct.pack('<QQQQQQ',n,lp,n,0,0,0));cpu.mem_write(frame+0x130,struct.pack('<Q',reflection))
            cpu.mem_write(reflection+0x148,struct.pack('<I',push));cpu.mem_write(lp,struct.pack('<'+'Q'*n,*layouts) if n else bytes(8))
            cpu.reg_write(UC_X86_REG_RBP,frame);cpu.reg_write(UC_X86_REG_RDI,device);cpu.reg_write(UC_X86_REG_RSP,stack)
            cpu.emu_start(0x1416a4b99,stop,count=200)
            assert 'compute' in state
            boundary_inputs.append(dict(op='compute',layouts=layouts,push=push));boundary_native.append(state['compute'])
pool_cases=0
for case,result in list(zip(cases,native_results))[::7]:
    if 'Ok' not in result:continue
    plan=result['Ok'];state.clear();state.update(mode='pool',input_layout=plan,heap=heap,sampler_calls=[],vk_result=0)
    device=data+0x1000;properties=data+0x3000;registry=data+0x10000;resultp=data+0x6000
    cpu.mem_write(device+0x658,struct.pack('<Q',properties));cpu.mem_write(properties+0x680,struct.pack('<I',case['limit']))
    cpu.mem_write(device+0x648,struct.pack('<Q',registry));cpu.mem_write(device+0x640,struct.pack('<Q',99))
    cpu.mem_write(device+0x240,struct.pack('<Q',data+0x120));cpu.mem_write(device+0x258,struct.pack('<Q',data+0x130))
    cpu.mem_write(stack,struct.pack('<Q',stop))
    for reg,val in [(UC_X86_REG_RSP,stack),(UC_X86_REG_RCX,resultp),(UC_X86_REG_RDX,device),(UC_X86_REG_R8,data+0x5000),(UC_X86_REG_R9,0)]:cpu.reg_write(reg,val)
    cpu.emu_start(0x140bfef70,stop,count=10000)
    assert cpu.reg_read(UC_X86_REG_RIP)==stop and 'pool' in state
    boundary_inputs.append(dict(op='pool',layout=plan,limit=case['limit']));boundary_native.append(state['pool']);pool_cases+=1
run=subprocess.run([str(crate/'target/release/boundary-oracle.exe')],input='\n'.join(json.dumps(c) for c in boundary_inputs)+'\n',text=True,capture_output=True,check=True)
boundary_candidate=[json.loads(x) for x in run.stdout.splitlines()]
assert len(boundary_candidate)==len(boundary_native)
boundary_failures=[dict(index=i,input=c,native=n,rust=r) for i,(c,n,r) in enumerate(zip(boundary_inputs,boundary_native,boundary_candidate)) if n!=r]
with gzip.open(out/'native-boundary-oracle.jsonl.gz','wt',encoding='utf8') as f:
    for c,n in zip(boundary_inputs,boundary_native):f.write(json.dumps(dict(input=c,native=n))+'\n')
original=tomllib.loads((root/'tracking/original-Cargo.lock').read_text())['package']
candidate=tomllib.loads((crate/'Cargo.lock').read_text())['package']
dependencies=[]
for p in candidate:
    if 'source' not in p:continue
    matches=[o for o in original if all(o.get(k)==p.get(k) for k in ['name','version','source','checksum'])]
    assert len(matches)==1,p
    dependencies.append({k:p[k] for k in ['name','version','source','checksum']})
(out/'dependency-verification.json').write_text(json.dumps(dict(packages=dependencies,matching_original=True),indent=2))
report=dict(binary_sha256=digest,cases=len(cases),catalog_declaration_cases=catalog_cases,original_entry='0x1418c2150',original_return='0x1418c2668',mismatches=len(failures),failures=failures[:10],vulkan_failure_branches=error_cases,source_sha256=hashlib.sha256((crate/'src/lib.rs').read_bytes()).hexdigest(),scope='Complete original persistent-layout CPU entry with external allocation/free/memset/sampler/Vulkan callbacks; captured actual VkDescriptorSetLayoutCreateInfo and return. Panic entry only, unwind excluded. Catalog cases feed declaration semantics through enumeration and do not establish original sparse bindings or real pass association.')
report['boundaries']=dict(cases=len(boundary_inputs),compute_layout_slice_cases=len(boundary_inputs)-pool_cases,compute_slice=['0x1416a4b99','0x1416a4c5a'],persistent_pool_entry_cases=pool_cases,persistent_pool_entry='0x140bfef70',mismatches=len(boundary_failures),failures=boundary_failures[:10],scope='Compute pipeline-layout create-info CPU slice after descriptor layout injection. Persistent pool full entry with injected validated layout output, public Vec growth, external allocator/registry/lifetime callbacks; actual Vulkan pool/allocate create infos captured. No real Vulkan/GPU execution.')
(out/'native-validation.json').write_text(json.dumps(report,indent=2))
print(json.dumps(report,indent=2));assert not failures and not boundary_failures
