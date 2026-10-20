import hashlib,json,random,struct
from pathlib import Path
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
HERE=Path(__file__).parent
blob=Path('D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe').read_bytes()
assert hashlib.sha256(blob).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=blob,fast_load=True);u=Uc(UC_ARCH_X86,UC_MODE_64)
for a in [0x140815000,0x140a1a000,0x140c9d000,0x14293e000]:
    u.mem_map(a,4096);u.mem_write(a,pe.get_data(a-0x140000000,4096))
u.mem_map(0x800000,0x20000);state={}
def hook(uc,address,size,data):
    if address==0x140a1af40:
        state['ids'].append(uc.reg_read(UC_X86_REG_EDX))
        uc.reg_write(UC_X86_REG_RAX,0x808000 if state['old'] else 0)
        rsp=uc.reg_read(UC_X86_REG_RSP);uc.reg_write(UC_X86_REG_RIP,struct.unpack('<Q',uc.mem_read(rsp,8))[0]);uc.reg_write(UC_X86_REG_RSP,rsp+8)
u.hook_add(UC_HOOK_CODE,hook)
def bits(x):return struct.unpack('<I',struct.pack('<f',x))[0]
rng=random.Random(20261023);rows=[]
for i in range(512):
    old=i%2;pose=[bits(rng.uniform(-10,10)) for _ in range(10)]
    if i<40:pose[i%10]=[0,0x80000000][i%2]
    state.clear();state.update(old=old,ids=[])
    rbp=0x810000
    u.mem_write(0x808000,struct.pack('<10I',*pose)+b'\0'*8)
    u.mem_write(rbp+0xd8,struct.pack('<Q',0x809000));u.mem_write(rbp+0x1c0,struct.pack('<I',92))
    u.reg_write(UC_X86_REG_RBP,rbp);u.reg_write(UC_X86_REG_RSP,0x81f000);u.reg_write(UC_X86_REG_XMM10,bits(-100.))
    u.emu_start(0x140815c00,0x140815cb2,count=1000)
    output=list(struct.unpack('<4I',int(u.reg_read(UC_X86_REG_XMM0)).to_bytes(16,'little')))
    output += [u.reg_read(UC_X86_REG_R14D),int(u.reg_read(UC_X86_REG_XMM8))&0xffffffff,u.reg_read(UC_X86_REG_R13D)]
    output += list(struct.unpack('<3I',u.mem_read(rbp+0x148,12)))
    assert state['ids']==[92]
    rows.append([old,*pose,*output])
fixture=Path('D:/game/tiny-glade/crates/systems/decorator/tests/fixtures/roof_uv_fallback_native.txt');fixture.parent.mkdir(exist_ok=True)
fixture.write_text('\n'.join(' '.join(map(str,r)) for r in rows))
(HERE/'roof-fallback-native-manifest.json').write_text(json.dumps({'binary_sha256':hashlib.sha256(blob).hexdigest(),'start':'140815c00','end_exclusive':'140815cb2','code_sha256':hashlib.sha256(pe.get_data(0x815c00,0xb2)).hexdigest(),'cases':len(rows),'fixture_sha256':hashlib.sha256(fixture.read_bytes()).hexdigest(),'scope':'Original previous-lookup miss/hit transform fallback and real original Transform::default. Hash lookup intercepted with explicit previous data; original cache insertion finite guards are checked by actual ECS separately.'},indent=2))
print('native roof UV fallback cases',len(rows))
