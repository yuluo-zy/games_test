import hashlib,json,random,struct
from pathlib import Path
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
HERE=Path(__file__).parent
blob=Path('D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe').read_bytes()
assert hashlib.sha256(blob).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=blob,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64)
for a in [0x140815000,0x140817000,0x142925000,0x142934000,0x142960000,0x1408e4000,0x1411ff000,0x140c9a000]:
    u.mem_map(a,4096);u.mem_write(a,pe.get_data(a-base,4096))
u.mem_map(0x800000,0x20000)
def bits(x):return struct.unpack('<I',struct.pack('<f',x))[0]
def scalar(reg):return int(u.reg_read(reg))&0xffffffff
def ret():
    rsp=u.reg_read(UC_X86_REG_RSP);u.reg_write(UC_X86_REG_RIP,struct.unpack('<Q',u.mem_read(rsp,8))[0]);u.reg_write(UC_X86_REG_RSP,rsp+8)
state={}
def hook(uc,address,size,data):
    if address==0x1408e4750:
        state['query']=list(struct.unpack('<3I',uc.mem_read(uc.reg_read(UC_X86_REG_R8),12)))
        ret()
    elif address==0x1411ff740:
        uc.reg_write(UC_X86_REG_RAX,0x808100);ret()
    elif address==0x140c9a8f0:
        state['xz']=[scalar(UC_X86_REG_XMM1),scalar(UC_X86_REG_XMM2)]
        uc.reg_write(UC_X86_REG_XMM0,state['normal'][0]);uc.reg_write(UC_X86_REG_XMM1,state['normal'][1]);ret()
    elif address==0x140817488:
        state['panic']=1;uc.emu_stop()
u.hook_add(UC_HOOK_CODE,hook)
rng=random.Random(20261022);rows=[]
for i in range(2400):
    rect=i%2;rank=rng.choice([0,1,2,3,4,255])
    p=[bits(rng.uniform(-20,20)) for _ in range(3)];center=[bits(rng.uniform(-10,10)) for _ in range(2)]
    normal=[bits(rng.uniform(-1,1)) for _ in range(2)]
    if i<160:
        p=[bits(1.),bits(2.),bits(3.)];center=[bits(1.),bits(3.)]
        if i%4==1:p[0]=bits(1.005)
        if i%4==2:p[0]=bits(1.01)
        if i%4==3:p[0]=bits(1.02)
    if 160<=i<320:
        specials=[0,0x80000000,0x7fc12345,0x7f800000,0xff800000]
        if i%2:p[i%3]=rng.choice(specials)
        else:center[i%2]=rng.choice(specials)
    state.clear();state.update(normal=normal,query=[],xz=[],panic=0)
    rbp=0x810000;u.mem_write(rbp+0xe0,struct.pack('<Q',0x808000))
    u.mem_write(0x808000,b'\0'*256);u.mem_write(0x808008,bytes([rect]));u.mem_write(0x80800c,struct.pack('<2I',*center))
    u.mem_write(0x809000,b'\0'*128);u.mem_write(0x809040,bytes([rank]))
    for reg,val in [(UC_X86_REG_RBP,rbp),(UC_X86_REG_RSI,0x809044),(UC_X86_REG_R14,p[0]),(UC_X86_REG_R13,p[2]),(UC_X86_REG_XMM8,p[1]),(UC_X86_REG_XMM12,0),(UC_X86_REG_RSP,0x81f000)]:u.reg_write(reg,val)
    u.emu_start(0x140815cf2,0x140815e0f,count=5000)
    result=[scalar(UC_X86_REG_XMM0),scalar(UC_X86_REG_XMM2)] if not state['panic'] else [0,0]
    rows.append([rank,rect,*p,*center,*normal,state['panic'],*result,*(state['query']+state['xz'] if state['query'] else [0]*5)])
(HERE/'roof-normal-native.txt').write_text('\n'.join(' '.join(map(str,r)) for r in rows))
(HERE/'roof-normal-native-manifest.json').write_text(json.dumps({'binary_sha256':hashlib.sha256(blob).hexdigest(),'start':'140815cf2','end_exclusive':'140815e0f','code_sha256':hashlib.sha256(pe.get_data(0x815cf2,0x140815e0f-0x140815cf2)).hexdigest(),'cases':len(rows),'panic_cases':sum(r[9] for r in rows),'scope':'Original rank guard, rectangular query coordinates and circular finite/nonfinite near-center policy; Roof section/rectangle normal are explicit intercepted dependencies, not verified by this slice. Quaternion/yaw and UV fallback are separate.'},indent=2))
print('roof normal native:',len(rows),'panics',sum(r[9] for r in rows))

# Separate original selector and original quaternion-product slices.
for a in [0x1429cd000]:
    u.mem_map(a,4096);u.mem_write(a,pe.get_data(a-base,4096))
def selector_hook(uc,address,size,data):
    if address==0x140815b8b:state['yaw']=1;uc.emu_stop()
u.hook_add(UC_HOOK_CODE,selector_hook)
selectors=[]
for tag in range(256):
    state['yaw']=0;u.mem_write(0x809042,bytes([tag]));u.reg_write(UC_X86_REG_RSI,0x809044)
    u.emu_start(0x140815b6c,0x140815bc0,count=100)
    selectors.append([tag,state['yaw']])
(HERE/'roof-yaw-selector-native.txt').write_text('\n'.join(' '.join(map(str,r)) for r in selectors))
products=[]
for i in range(2400):
    qa=[bits(rng.uniform(-1,1)) for _ in range(4)];qb=[bits(rng.uniform(-1,1)) for _ in range(4)]
    u.reg_write(UC_X86_REG_XMM9,int.from_bytes(struct.pack('<4I',*qa),'little'))
    u.reg_write(UC_X86_REG_XMM0,int.from_bytes(struct.pack('<4I',*qb),'little'))
    u.reg_write(UC_X86_REG_XMM15,int.from_bytes(struct.pack('<4f',-1.,1.,1.,-1.),'little'))
    u.reg_write(UC_X86_REG_RBP,0x810000)
    u.emu_start(0x140815e25,0x140815e84,count=100)
    products.append([*qa,*qb,*struct.unpack('<4I',u.mem_read(0x810190,16))])
(HERE/'roof-product-native.txt').write_text('\n'.join(' '.join(map(str,r)) for r in products))
print('roof selectors',len(selectors),'quaternion products',len(products))
