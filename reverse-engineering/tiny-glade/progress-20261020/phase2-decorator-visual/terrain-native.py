import hashlib,json,math,random,struct
from pathlib import Path
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
HERE=Path(__file__).parent
blob=Path('D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe').read_bytes()
assert hashlib.sha256(blob).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=blob,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64)
for address in [0x140816000,0x140257000,0x142925000,0x1429cd000,0x14295a000,0x142a82000]:
    u.mem_map(address,4096);u.mem_write(address,pe.get_data(address-base,4096))
u.mem_map(0x800000,0x20000)
def bits(x):return struct.unpack('<I',struct.pack('<f',x))[0]
def f32(x):return struct.unpack('<f',struct.pack('<f',x))[0]
def scalar(reg):return int(u.reg_read(reg))&0xffffffff
state={}
def hook(uc,address,size,data):
    if address==0x140257ba0:
        state['coordinates'].extend([scalar(UC_X86_REG_XMM1),scalar(UC_X86_REG_XMM2)])
        uc.reg_write(UC_X86_REG_XMM0,state['heights'][len(state['coordinates'])//2-1])
        rsp=uc.reg_read(UC_X86_REG_RSP)
        uc.reg_write(UC_X86_REG_RIP,struct.unpack('<Q',uc.mem_read(rsp,8))[0]);uc.reg_write(UC_X86_REG_RSP,rsp+8)
u.hook_add(UC_HOOK_CODE,hook)
rng=random.Random(20261021);cases=[];axes=[]
for i in range(2400):
    angle=rng.uniform(-math.pi,math.pi)
    q=[0.,f32(math.sin(angle*.5)),0.,f32(math.cos(angle*.5))]
    u.reg_write(UC_X86_REG_XMM15,int.from_bytes(struct.pack('<4f',*q),'little'))
    u.reg_write(UC_X86_REG_XMM7,int.from_bytes(struct.pack('<4f',0.,q[1],0.,0.),'little'))
    u.reg_write(UC_X86_REG_XMM3,0)
    u.emu_start(0x140816282,0x140816338,count=5000)
    axes.append([*[bits(x) for x in q],scalar(UC_X86_REG_XMM1),scalar(UC_X86_REG_XMM2)])
    fields=[f32(math.cos(angle)*rng.uniform(.1,4)),f32(math.sin(angle)*rng.uniform(.1,4)),
            f32(rng.uniform(-20,20)),f32(rng.uniform(-20,20)),
            f32(rng.uniform(0,6)),f32(rng.uniform(0,6)),f32(rng.uniform(-3,3)),f32(rng.uniform(-3,3))]
    heights=[bits(f32(rng.uniform(-3,5))) for _ in range(4)]
    if i<200:
        heights=[bits(v) for v in rng.choices([0.,-0.,.25,.5,1.,-1.,float('inf'),-float('inf')],k=4)]
        if i%3==0:heights[i%4]=0x7fc12345
    state.clear();state.update(coordinates=[],heights=heights)
    rbp=0x810000;u.mem_write(rbp+0x190,struct.pack('<8f',*fields))
    u.mem_write(0x808000,b'\0'*0x100)
    for reg,val in [(UC_X86_REG_RBP,rbp),(UC_X86_REG_RBX,0x808000),(UC_X86_REG_R13,rbp+0x148),(UC_X86_REG_RSP,0x81f000)]:u.reg_write(reg,val)
    u.emu_start(0x140816362,0x140816653,count=5000)
    output=struct.unpack('<I',u.mem_read(rbp+0x17c,4))[0]
    cases.append([*[bits(x) for x in fields],*heights,output,*state['coordinates']])
(HERE/'terrain-native.txt').write_text('\n'.join(' '.join(map(str,c)) for c in cases))
(HERE/'terrain-axis-native.txt').write_text('\n'.join(' '.join(map(str,c)) for c in axes))
body=pe.get_data(0x816362,0x140816653-0x140816362)
(HERE/'terrain-native-manifest.json').write_text(json.dumps({'binary_sha256':hashlib.sha256(blob).hexdigest(),'start':'140816362','end_exclusive':'140816653','code_sha256':hashlib.sha256(body).hexdigest(),'cases':len(cases),'hooked':['140257ba0: terrain heights supplied; sampler not verified'],'scope':'Original full four-coordinate sampling and support-height block. Bounds orientation/translation and whole cache ECS are outside this native slice.'},indent=2))
print('Native terrain support cases:',len(cases))
