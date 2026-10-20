from pathlib import Path
import hashlib, json, struct
import pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import *

root=Path('D:/game')
out=root/'reverse-engineering/tiny-glade/progress-20261020/central-undo-layout'
out.mkdir(exist_ok=True)
binary=root/'reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe'
pe=pefile.PE(str(binary),fast_load=True)
im=pe.get_memory_mapped_image()
u=Uc(UC_ARCH_X86,UC_MODE_64)
u.mem_map(pe.OPTIONAL_HEADER.ImageBase,(len(im)+4095)&~4095)
u.mem_write(pe.OPTIONAL_HEADER.ImageBase,im)
u.mem_map(0x100000,0x400000)
styles=[]; children=[]; bundles=[]; calls=[]
heap=0x300000
def q(addr): return struct.unpack('<Q',u.mem_read(addr,8))[0]
def ret(value=None):
    sp=u.reg_read(UC_X86_REG_RSP)
    if value is not None: u.reg_write(UC_X86_REG_RAX,value)
    u.reg_write(UC_X86_REG_RIP,q(sp));u.reg_write(UC_X86_REG_RSP,sp+8)
def hook(u,pc,n,data):
    global heap
    rcx=u.reg_read(UC_X86_REG_RCX);rdx=u.reg_read(UC_X86_REG_RDX)
    r8=u.reg_read(UC_X86_REG_R8);sp=u.reg_read(UC_X86_REG_RSP)
    if pc==0x140fe7600:
        styles.append(bytes(u.mem_read(r8,344)))
        u.mem_write(rcx,struct.pack('<III',4,len(styles),1));ret()
    elif pc==0x140fe7790:
        children.append([r8,u.reg_read(UC_X86_REG_R9),q(sp+0x28),q(sp+0x30)])
        u.mem_write(rcx,struct.pack('<I',4));ret()
    elif pc==0x140613c10:
        alloc=(heap+rdx-1)&~(rdx-1);heap=alloc+rcx;ret(alloc)
    elif pc==0x142923c00:
        u.mem_write(rcx,bytes(u.mem_read(rdx,r8)));ret(rcx)
    elif pc==0x140fe0470:
        bundle=bytes(u.mem_read(r8,0x70));bundles.append(bundle.hex())
        u.mem_write(rcx,struct.pack('<QQQQQ',0,0x170000,0,0,0x100000000+len(bundles)))
        ret()
    elif pc==0x1428a58d0:
        raise RuntimeError('unexpected deferred queue reallocation')
    elif pc in [0x1428d9430,0x1428d9760,0x1428d8fe3]:
        raise RuntimeError('native panic')
    if pc in [0x140fe7600,0x140fe7790,0x140613c10,0x142923c00,0x140fe0470]:calls.append(hex(pc))

u.hook_add(UC_HOOK_CODE,hook)
u.mem_write(0x110098,struct.pack('<QQQ',64,0x150000,0))
u.mem_write(0x170000,struct.pack('<QQQ',0x10000,0x180000,0))
u.mem_write(0x20fff8,struct.pack('<Q',0x100000))
u.reg_write(UC_X86_REG_RSP,0x20fff8)
u.reg_write(UC_X86_REG_RCX,0x120000)
u.reg_write(UC_X86_REG_RDX,0x110000)
u.reg_write(UC_X86_REG_R8,0x130000)
u.emu_start(0x140fad870,0x100000,count=10000)
for i,b in enumerate(styles): (out/f'style-{i}.bin').write_bytes(b)
record={'binary_sha256':hashlib.sha256(binary.read_bytes()).hexdigest(),
 'function':['140fad870',2580], 'styles':len(styles),'children':children,
 'bundles':bundles,'layout':bytes(u.mem_read(0x120000,24)).hex(),
 'anchor':bytes(u.mem_read(0x150000,40)).hex(),
 'deferred_commands':bytes(u.mem_read(0x180000,q(0x170010))).hex(),
 'hooks':'allocator/memcpy, real Taffy and Bevy API calls record inputs and return explicit synthetic node/entity IDs; no layout/render/runtime verdict',
 'calls':calls}
(out/'native-undo-setup.json').write_text(json.dumps(record,indent=2))
print(json.dumps(record,indent=2))
