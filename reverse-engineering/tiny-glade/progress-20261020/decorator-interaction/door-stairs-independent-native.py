import hashlib,json,math,random,struct
from pathlib import Path
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *

HERE=Path(__file__).parent
exe=Path('D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe')
blob=exe.read_bytes()
assert hashlib.sha256(blob).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=blob,fast_load=True)
base=pe.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64)
mapped=set()
def map_page(address):
    page=address&~4095
    if page in mapped:return
    u.mem_map(page,4096);mapped.add(page)
    if base<=page<base+pe.OPTIONAL_HEADER.SizeOfImage:
        data=pe.get_data(page-base,4096)
        u.mem_write(page,data)
for a in [0x1407f3340,0x140257ba0,0x14067dd80,0x140a14df0,0x140613c10,0x1428d9000,
          0x142a8af30,0x1429cd0c0,0x142a7fa50,0x1429258e0,0x142a7b658,0x1429df16c,
          0x14295a0e8,0x143634c09]:map_page(a)
u.mem_map(0x800000,0x20000);u.mem_map(0x900000,0x10000)
STOP=0x900f00
def f32(x):return struct.unpack('<f',struct.pack('<f',x))[0]
def bits(x):return struct.unpack('<I',struct.pack('<f',x))[0]
def floats(xs):return struct.pack('<'+'f'*len(xs),*xs)
def scalar(reg):return struct.unpack('<f',int(u.reg_read(reg)).to_bytes(16,'little')[:4])[0]
def ret():
    rsp=u.reg_read(UC_X86_REG_RSP)
    ip=struct.unpack('<Q',u.mem_read(rsp,8))[0]
    u.reg_write(UC_X86_REG_RSP,rsp+8);u.reg_write(UC_X86_REG_RIP,ip)
state={}
def hook(uc,address,size,data):
    if address==0x140257ba0:
        state['samples'].append([bits(scalar(UC_X86_REG_XMM1)),bits(scalar(UC_X86_REG_XMM2))])
        index=len(state['samples'])-1
        uc.reg_write(UC_X86_REG_XMM0,bits(state['case']['heights'][index]));ret()
    elif address==0x14067dd80:
        uc.reg_write(UC_X86_REG_RAX,0x902000);ret()
    elif address==0x140a14df0:
        state['path'].append([bits(scalar(UC_X86_REG_XMM1)),bits(scalar(UC_X86_REG_XMM2))])
        uc.reg_write(UC_X86_REG_XMM0,bits(state['case']['coverage']));ret()
    elif address==0x140613c10:
        assert uc.reg_read(UC_X86_REG_RCX)==32 and uc.reg_read(UC_X86_REG_RDX)==4
        uc.reg_write(UC_X86_REG_RAX,0x904000);ret()
    elif address==0x140613c20:
        state['frees'].append([uc.reg_read(UC_X86_REG_RCX),uc.reg_read(UC_X86_REG_RDX),uc.reg_read(UC_X86_REG_R8)]);ret()
    elif address==0x1428d9000:raise AssertionError('unexpected alloc failure')
u.hook_add(UC_HOOK_CODE,hook)
rng=random.Random(20261020)
cases=[]
for i in range(192):
    angle=rng.uniform(-math.pi,math.pi)
    q=[0.,f32(math.sin(angle*.5)),0.,f32(math.cos(angle*.5))]
    if i%11==0:
        q=[f32(rng.uniform(-1,1)) for _ in range(4)]
        norm=math.sqrt(sum(x*x for x in q));q=[f32(x/norm) for x in q]
    case={'q':q,'p':[f32(rng.uniform(-20,20)),f32(rng.uniform(-2,5)),f32(rng.uniform(-20,20))],
          'width':f32(rng.uniform(.1,5)),'bottom':f32(rng.uniform(-2,0)),'top':f32(rng.uniform(.1,3)),
          'heights':[f32(rng.uniform(-3,5)) for _ in range(4)],'coverage':rng.choice([0.,.01,1.,-1.]),'old':bool(i%2)}
    case.update(q=[0.,0.,0.,1.],p=[1.,0.,2.],bottom=-1.,top=1.,width=2.,heights=[2.,.5,.5,.5],coverage=1.,old=bool(i%2))
    # All four terrain NaNs, all signs of infinity/zero, quaternion and width.
    lane=(i//12)%8
    value=[float('nan'),float('inf'),float('-inf'),0.,-0.,-2.,.1,1.5][lane]
    target=i%12
    if target<4: case['heights'][target]=value
    elif target==4: case['coverage']=value
    elif target==5: case['width']=value
    elif target==6: case['bottom']=value
    elif target==7: case['top']=value
    elif target==8: case['p'][1]=value
    elif target==9: case['q']=[0.,value,0.,1.]
    elif target==10: case['p'][0]=value
    else: case['q']=[0.,0.,value,1.]
    q=case['q']
    state.clear();state.update(case=case,samples=[],path=[],frees=[])
    u.mem_write(0x900000,floats(q+case['p']+[1.,1.,1.]))
    u.mem_write(0x901000,floats([1.,0.,0.,0.,case['width'],1.,case['bottom'],case['top']]))
    u.mem_write(0x902000,b'\0'*0x300)
    u.mem_write(0x903000,b'\0'*0x200)
    old=0x905000 if case['old'] else 0
    u.mem_write(0x903120,struct.pack('<Q',old))
    rsp=0x81f000-8
    u.mem_write(rsp,struct.pack('<Q',STOP));u.mem_write(rsp+40,struct.pack('<Q',0x903000))
    for reg,val in [(UC_X86_REG_RSP,rsp),(UC_X86_REG_RCX,0x902800),(UC_X86_REG_RDX,0x902000),
                    (UC_X86_REG_R8,0x900000),(UC_X86_REG_R9,0x901000)]:u.reg_write(reg,val)
    u.emu_start(0x1407f3340,STOP,count=20000)
    ok=bool(u.reg_read(UC_X86_REG_RAX)&255)
    ptr=struct.unpack('<Q',u.mem_read(0x903120,8))[0]
    assert ptr==(0x904000 if ok else old)
    assert state['frees']==([[old,32,4]] if ok and old else [])
    cases.append({'q':[bits(x) for x in q],'p':[bits(x) for x in case['p']],
        'width':bits(case['width']),'bottom':bits(case['bottom']),'top':bits(case['top']),
        'heights':[bits(x) for x in case['heights']],'coverage':bits(case['coverage']),
        'output':list(struct.unpack('<8I',u.mem_read(ptr,32))) if ok else None,
        'terrain_coordinates':state['samples'],'path_coordinates':state['path'],'replaced_old':bool(state['frees'])})
(HERE/'door-stairs-native.json').write_text(json.dumps(cases,separators=(',',':')),encoding='utf-8')
body=pe.get_data(0x7f3340,0x533)
(HERE/'door-stairs-native-manifest.json').write_text(json.dumps({'binary_sha256':hashlib.sha256(blob).hexdigest(),
 'range_start':'0x1407f3340','range_size':len(body),'range_sha256':hashlib.sha256(body).hexdigest(),
 'cases':len(cases),'accepted':sum(x['output'] is not None for x in cases),
 'dependencies_hooked':['terrain bilinear sampler 140257ba0','WorldRaster accessor 14067dd80',
 'path bilinear sampler 140a14df0','alloc/dealloc'],
 'scope':'Original complete native geometry/control flow; actual shared sampler library not executed by this oracle.'},indent=2),encoding='utf-8')
print('native cases',len(cases),'accepted',sum(x['output'] is not None for x in cases))
