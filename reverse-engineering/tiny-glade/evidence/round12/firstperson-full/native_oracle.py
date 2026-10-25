"""Complete original1509B footstep function, real original835B raster sampler.
Bridges only FMOD-facing shared audio calls, FrozenMap lookup, mapped view getter
and CRT memcmp; no firstperson game branch or raster sample is substituted.
"""
from pathlib import Path
import hashlib,json,struct,math
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *

OUT=Path(__file__).resolve().parent
raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase
image=pe.get_memory_mapped_image()
cpu=Uc(UC_ARCH_X86,UC_MODE_64);cpu.mem_map(base,(pe.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);cpu.mem_write(base,image)
D,H,S,STOP=0x30000000,0x40000000,0x50000000,0x60000000
for at,n in [(D,0x20000),(H,0x400000),(S,0x20000),(STOP,0x1000)]:cpu.mem_map(at,n)
log=[];samples=[]
def put(at,fmt,*v):cpu.mem_write(at,struct.pack(fmt,*v))
def u64(at):return struct.unpack('<Q',cpu.mem_read(at,8))[0]
def f32bits(f):return struct.unpack('<I',struct.pack('<f',f))[0]
def xmmf(reg):return struct.unpack('<f',struct.pack('<I',cpu.reg_read(reg)&0xffffffff))[0]
def ret(v=None):
    if v is not None:cpu.reg_write(UC_X86_REG_RAX,v)
    sp=cpu.reg_read(UC_X86_REG_RSP);target=u64(sp)
    cpu.reg_write(UC_X86_REG_RSP,sp+8);cpu.reg_write(UC_X86_REG_RIP,target)
def hook(c,pc,size,user):
    if pc==STOP:c.emu_stop();return
    if pc==0x140a53db0:
        ptr,n=c.reg_read(UC_X86_REG_RDX),c.reg_read(UC_X86_REG_R8)
        log.append({'parameter':bytes(c.mem_read(ptr,n)).decode(),'value_bits':c.reg_read(UC_X86_REG_XMM3)&0xffffffff});ret()
    elif pc==0x140969720:log.append({'lookup':'TerrainRockMask'});ret(D+0x1800)
    elif pc==0x142923bf0:
        a,b,n=[c.reg_read(r) for r in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8]]
        ret(0 if bytes(c.mem_read(a,n))==bytes(c.mem_read(b,n)) else 1)
    elif pc==STOP+0x100:ret(D+0x1b00) # real typed mapped slice boundary
    elif pc in [0x140a59430,0x140a596b0]:
        log.append({'loop':'stop' if pc==0x140a59430 else 'start'});ret()
    elif pc==0x140a14df0:
        ptr=u64(c.reg_read(UC_X86_REG_RCX))
        samples.append((ptr-H)//0x80000) # observe original sampler; do not hook return
cpu.hook_add(UC_HOOK_CODE,hook)
def run(mode=3,phase=1,pan=(0.,0.),modifier=0,source=11,channels=(0,0,0,0,0,0),rock=False,centered=False,selector=0,studio=True):
    log.clear();samples.clear();cpu.mem_write(D,bytes(0x18000));cpu.mem_write(S,bytes(0x20000))
    # refs: core,camera,SSBO,paths,state,water,garden,actions in actual order.
    refs=[D+0x100+i*0x40 for i in range(8)]
    core,camera,library,pathrefs,state,water,garden,actions=D+0x1000,D+0x2000,D+0x1400,refs[3],D+0x1600,D+0x6000,D+0x6500,D+0x7000
    for ref,obj in zip(refs,[core,camera,library,0,state,water,garden,actions]):put(ref,'<Q',obj)
    put(core,'<Q',D+0x1200 if studio else 0)
    put(refs[4]+0x10,'<Q',D+0x1700);put(refs[4]+0x1c,'<I',12345)
    # Only current camera mode, ground source and eye position are read.
    cpu.mem_write(camera+0x4d0,bytes([mode]));cpu.mem_write(camera+0x328,bytes([source]));cpu.mem_write(camera+0x344,bytes([modifier]))
    put(camera+0xc0,'<16f',1,0,0,0,0,1,0,0,0,0,1,0,0,1.9,0,1)
    put(actions+0x18,'<I',phase);put(actions+0x150,'<2f',*pan)
    # Original BufferState{object,vtable,name_ptr,name_len} and mapped slice.
    put(D+0x1800,'<4Q',D+0x1a00,D+0x1900,0x142a9d4c1,63)
    put(D+0x1900+0x18,'<Q',STOP+0x100)
    put(D+0x1b00,'<QQ',H+0x350000,0x2000)
    cpu.mem_write(H+0x350000,bytes([255 if rock else 0])*0x8000)
    # Six canonical raster roles: water,path,cobble1,cobble2,cobble3,garden.
    for i,value in enumerate(channels):
        obj=water if i==0 else garden if i==5 else D+0x4000+(i-1)*0x400
        p=H+i*0x80000
        cpu.mem_write(p,bytes([value])*0x40000)
        cpu.mem_write(p+0x40000,bytes([255-value])*0x40000)
        put(obj,'<6Q',0x40000,p,0x40000,0x40000,p+0x40000,0x40000)
        cpu.mem_write(obj+0x298,bytes([selector]))
        if 1<=i<=4:put(pathrefs+(i-1)*0x20,'<Q',obj)
    put(0x14362c308,'<Q',D+0x800);cpu.mem_write(D+0x800,bytes([int(centered)]))
    sp=S+0x1f008;put(sp,'<Q',STOP)
    for i in range(4,8):put(sp+0x28+(i-4)*8,'<Q',refs[i])
    for r in [UC_X86_REG_RAX,UC_X86_REG_RBX,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_RSI,UC_X86_REG_RDI,UC_X86_REG_RBP,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,UC_X86_REG_R13,UC_X86_REG_R14,UC_X86_REG_R15]:cpu.reg_write(r,0)
    for r in range(UC_X86_REG_XMM0,UC_X86_REG_XMM15+1):cpu.reg_write(r,0)
    for r,v in zip([UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_R9],refs):cpu.reg_write(r,v)
    cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
    cpu.emu_start(0x140a889b0,STOP,count=200000)
    assert cpu.reg_read(UC_X86_REG_RIP)==STOP,hex(cpu.reg_read(UC_X86_REG_RIP))
    return {'calls':list(log),'sample_order':list(samples),'state_change_tick':struct.unpack('<I',cpu.mem_read(D+0x1700,4))[0]}

if __name__=='__main__':
    cases=[]
    def add(name,**kw):
        normalized=dict(kw)
        if 'pan' in normalized:normalized['pan_bits']=[f32bits(f) for f in normalized.pop('pan')]
        value=run(**kw);cases.append({'name':name,'inputs':normalized,'result':value})
    for mode in range(6):
        for phase in range(5):
            for pan in [(0.,0.),(1.,0.),(math.nan,0.)]:
                for modifier in [0,1]:add(f'gate-{mode}-{phase}-{pan}-{modifier}',mode=mode,phase=phase,pan=pan,modifier=modifier)
    materials=[('bare',(0,0,0,0,0,0),False),('water',(26,255,255,255,255,255),False),('path',(25,26,0,0,0,255),False),('cobble1',(25,0,26,0,0,255),False),('cobble2',(0,0,0,26,0,255),False),('cobble3',(0,0,0,0,26,255),False),('path-sum',(0,7,7,7,7,0),False),('rock',(0,0,0,0,0,255),True),('garden-equal',(0,0,0,0,0,102),False),('garden-over',(0,0,0,0,0,103),False)]
    for centered in [False,True]:
        for name,ch,rock in materials:add(name+str(centered),channels=ch,rock=rock,centered=centered)
    add('scratch-selected',channels=(255,0,0,0,0,0),selector=1)
    add('nonterrain-source',source=17,channels=(255,255,255,255,255,255))
    add('disabled-studio',studio=False,channels=(0,0,0,0,0,103))
    (OUT/'native-firstperson-fixtures.json').write_text(json.dumps(cases,indent=2)+'\n',encoding='utf-8')
    spans=[{'va':hex(va),'size':size,'sha256':hashlib.sha256(image[va-base:va-base+size]).hexdigest()} for va,size in [(0x140a889b0,1509),(0x140a14df0,835),(0x140c96d40,31)]]
    (OUT/'native-manifest.json').write_text(json.dumps({'original_sha256':hashlib.sha256(raw).hexdigest(),'cases':len(cases),'spans':spans,'bridges':'FMOD-facing set_parameter, shared AudioLoop start/stop, FrozenMap.get, mapped getter and CRT memcmp only; actual raster sampler executes'},indent=2)+'\n',encoding='utf-8')
    print(json.dumps({'complete_original_firstperson_cases':len(cases),'material_examples':[(x['name'],x['result']) for x in cases[-23:]]}))
