from pathlib import Path
import struct, json, hashlib, math, random
import pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import *

OUT=Path(__file__).resolve().parent
raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
sha=hashlib.sha256(raw).hexdigest()
assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw); image=pe.get_memory_mapped_image(); base=pe.OPTIONAL_HEADER.ImageBase
cpu=Uc(UC_ARCH_X86,UC_MODE_64)
cpu.mem_map(base,(pe.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);cpu.mem_write(base,image)
DATA,STACK,STOP=0x20000000,0x30000000,0x50000000
cpu.mem_map(DATA,0x100000);cpu.mem_map(STACK,0x20000);cpu.mem_map(STOP,0x1000)
def q(a,v):cpu.mem_write(a,struct.pack('<Q',v))
def f32(v):return struct.unpack('<f',struct.pack('<f',v))[0]
def floor_hook(c,pc,size,_):
    if pc==STOP:c.emu_stop()
    elif pc==0x142923ee0: # Only the existing CRT floorf boundary is modeled.
        value=struct.unpack('<f',struct.pack('<I',c.reg_read(UC_X86_REG_XMM0)&0xffffffff))[0]
        out=value if not math.isfinite(value) else float(math.floor(value))
        c.reg_write(UC_X86_REG_XMM0,struct.unpack('<I',struct.pack('<f',out))[0])
        sp=c.reg_read(UC_X86_REG_RSP);ret=struct.unpack('<Q',c.mem_read(sp,8))[0]
        c.reg_write(UC_X86_REG_RSP,sp+8);c.reg_write(UC_X86_REG_RIP,ret)
cpu.hook_add(UC_HOOK_CODE,floor_hook)
def native(center,radius,occupied):
    cpu.mem_write(DATA,bytes(0x40000));cpu.mem_write(STACK,bytes(0x20000))
    for r in [UC_X86_REG_RAX,UC_X86_REG_RBX,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_RBP,UC_X86_REG_RSI,UC_X86_REG_RDI,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,UC_X86_REG_R13,UC_X86_REG_R14,UC_X86_REG_R15]:cpu.reg_write(r,0)
    for r in range(UC_X86_REG_XMM0,UC_X86_REG_XMM15+1):cpu.reg_write(r,0)
    cells=DATA+0x1000;circle=DATA+0x200;flag=DATA+0x300
    q(DATA,4096);q(DATA+8,cells);q(DATA+16,4096)
    cpu.mem_write(DATA+0x48,struct.pack('<IIff',64,64,130.,130.))
    cpu.mem_write(circle,struct.pack('<ffff',*center,radius,0.));cpu.mem_write(flag,b'\x01')
    for x,z in occupied:q(cells+(z*64+x)*48+16,1)
    sp=STACK+0x10000-8;q(sp,STOP)
    cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_RCX,DATA);cpu.reg_write(UC_X86_REG_RDX,circle);cpu.reg_write(UC_X86_REG_R8,flag)
    cpu.reg_write(UC_X86_REG_MXCSR,0x1f80);cpu.reg_write(UC_X86_REG_RFLAGS,0x202)
    try:cpu.emu_start(0x1425defd0,STOP,count=2000000)
    except Exception as e:raise RuntimeError(f'{e} at {cpu.reg_read(UC_X86_REG_RIP):x} center={center}')
    assert cpu.reg_read(UC_X86_REG_RIP)==STOP
    return bytes(cpu.mem_read(flag,1))==b'\x00'
def candidate(center,radius,occupied):
    # Independent f32 policy projection, compared with untouched full native
    # callback and its geometry/grid helpers. No candidate Rust is executed.
    x,z=map(f32,center);radius=f32(radius)
    lo=[max(f32(v-radius),-65.) for v in (x,z)]
    hi=[min(f32(v+radius),65.) for v in (x,z)]
    lo=[max(math.floor(f32(f32(v+65.)/2.03125)),0) for v in lo]
    hi=[min(math.floor(f32(f32(v+65.)/2.03125)),63) for v in hi]
    expansion=f32(f32(2.03125*.5)*struct.unpack('<f',struct.pack('<I',0x3fb504f3))[0])
    for iz in range(lo[1],hi[1]+1):
        for ix in range(lo[0],hi[0]+1):
            middle=lambda i:f32(f32(f32(f32(i/64.)*65.)-f32(f32(1.-f32(i/64.))*65.))+f32(2.03125*.5))
            dx=f32(middle(ix)-x);dz=f32(middle(iz)-z)
            distance=f32(math.sqrt(f32(f32(dz*dz)+f32(dx*dx))))
            if f32(radius+expansion)>distance and (ix,iz) in occupied:return True
    return False
rows=[]
cases=[((0.,0.),7.,[]),((0.,0.),7.,[(32,32)]),((0.,0.),7.,[(36,32)]),((65.,65.),7.,[(63,63)]),((-65.,-65.),7.,[(0,0)]),((72.,72.),7.,[(63,63)]),((80.,80.),7.,[(63,63)]),((0.,0.),0.,[(32,32)])]
rng=random.Random(20261019)
for _ in range(160):
    center=(rng.uniform(-75,75),rng.uniform(-75,75)); radius=rng.choice([0.,.5,7.,15.])
    near=[(min(63,max(0,int((v+65)/2.03125)))) for v in center]
    occupied=[(min(63,max(0,near[0]+rng.randint(-5,5))),min(63,max(0,near[1]+rng.randint(-5,5)))) for _ in range(rng.randrange(8))]
    cases.append((center,radius,occupied))
for center,radius,occupied in cases:
    center=tuple(map(f32,center));got=native(center,radius,occupied);want=candidate(center,radius,occupied)
    assert got==want,(center,radius,occupied,got,want)
    rows.append({'center':center,'radius':radius,'occupied_cells':occupied,'native_reject':got})
report={'original_sha256':sha,'callback':{'va':'0x1425defd0','bytes':837,'sha256':hashlib.sha256(image[0x1425defd0-base:0x1425defd0-base+837]).hexdigest()},'scope':'Full original callback plus original geometry/grid helpers executed. Only CRT floorf intercepted; all assertions/branches and occupied-cell reads original.','cases':rows}
(OUT/'original-wall-fixtures.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps({'native_cases':len(rows),'rejected':sum(r['native_reject'] for r in rows)}))
