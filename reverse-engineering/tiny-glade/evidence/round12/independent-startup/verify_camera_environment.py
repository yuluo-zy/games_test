"""Original B terrain sampling and complete Normal terrain-constraint oracle.

No geometry, sampling or camera helper is hooked. The runtime sampling feature
pointer is initialized to a fixture-owned bool in emulated data memory.
"""
from pathlib import Path
import hashlib,json,struct,math
import pefile
from capstone import Cs,CS_ARCH_X86,CS_MODE_64
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *

OUT=Path(__file__).resolve().parent
INSTALL=Path('D:/game/ljxsj_92385/Tiny Glade')
raw=(INSTALL/'tiny-glade.rne').read_bytes()
sha=hashlib.sha256(raw).hexdigest()
assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase
mapped=pe.get_memory_mapped_image()
cpu=Uc(UC_ARCH_X86,UC_MODE_64)
cpu.mem_map(base,(pe.OPTIONAL_HEADER.SizeOfImage+4095)&~4095)
cpu.mem_write(base,mapped)
D,S,STOP=0x20000000,0x21000000,0x22000000
cpu.mem_map(D,0x100000);cpu.mem_map(S,0x20000);cpu.mem_map(STOP,0x1000)
cpu.hook_add(UC_HOOK_CODE,lambda c,p,n,u:c.emu_stop() if p==STOP else None)
grid,values,flag,body,context=D+0x1000,D+0x10000,D+0x800,D+0x2000,D+0x3000
cpu.mem_write(values,b''.join(struct.pack('<H',(x*97+z*131+257)&65535) for z in range(512) for x in range(512)))
cpu.mem_write(grid,struct.pack('<3Q',512*512,values,512*512))
cpu.mem_write(grid+0x50,struct.pack('<2f',-2.5,24.0))
cpu.mem_write(context+0x20,struct.pack('<Q',grid))
cpu.mem_write(0x14362c308,struct.pack('<Q',flag))
def bits(f):return struct.unpack('<I',struct.pack('<f',f))[0]
def fl(reg):return struct.unpack('<f',struct.pack('<I',cpu.reg_read(reg)&0xffffffff))[0]
def invoke(entry,regs):
    sp=S+0x1f008
    cpu.mem_write(sp,struct.pack('<Q',STOP))
    for r in [UC_X86_REG_RAX,UC_X86_REG_RBX,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_RSI,UC_X86_REG_RDI,UC_X86_REG_RBP,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,UC_X86_REG_R13,UC_X86_REG_R14,UC_X86_REG_R15]:cpu.reg_write(r,0)
    for r in range(UC_X86_REG_XMM0,UC_X86_REG_XMM15+1):cpu.reg_write(r,0)
    for r,v in regs:cpu.reg_write(r,v)
    cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
    cpu.emu_start(entry,STOP,count=10000)
    assert cpu.reg_read(UC_X86_REG_RIP)==STOP

samples=[];constraints=[]
coords=[(-100.,-100.),(-65.,-65.),(-64.9,64.9),(-30.1927490234375,15.641988754272461),(0.,0.),(-0.,0.),(.125,.375),(1.,2.),(64.9,-64.9),(65.,65.),(100.,100.),(math.nan,0.),(math.inf,-math.inf)]
positions=[(0.,0.,0.),(12.,5.,-8.),(-65.,24.,65.)]
pending=[(0.,0.,0.),(.125,-10.,-.375),(9.,math.nan,12.),(-100.,5.,100.)]
for centered in [False,True]:
    cpu.mem_write(flag,bytes([int(centered)]))
    for x,z in coords:
        invoke(0x1409e9640,[(UC_X86_REG_RCX,grid),(UC_X86_REG_XMM1,bits(x)),(UC_X86_REG_XMM2,bits(z))])
        samples.append({'centered':centered,'x_bits':bits(x),'z_bits':bits(z),'height_bits':cpu.reg_read(UC_X86_REG_XMM0)&0xffffffff})
    for pos in positions:
        for delta in pending:
            for clearance in [0.,2.]:
                cpu.mem_write(body,bytes(0xb8))
                cpu.mem_write(body+0x38,struct.pack('<3f',*pos))
                cpu.mem_write(body+0x44,struct.pack('<3f',*delta))
                cpu.mem_write(body+0x68,struct.pack('<f',clearance))
                invoke(0x140949dc0,[(UC_X86_REG_RCX,body),(UC_X86_REG_RDX,context)])
                constraints.append({'centered':centered,'position_bits':[bits(f) for f in pos],'pending_bits':[bits(f) for f in delta],'clearance_bits':bits(clearance),'result_bits':list(struct.unpack('<3I',cpu.mem_read(body+0x44,12)))})

spans=[]
for name,va,size in [('terrain_sample',0x1409e9640,983),('normal_terrain_constraint',0x140949dc0,166),('walk_entry',0x14090e510,473),('free_from_normal',0x14090e6f0,1404),('glade_grading',0x1408bf9c0,2346),('meadow',0x1408639f0,2611)]:
    data=mapped[va-base:va-base+size]
    spans.append({'name':name,'va':hex(va),'size':size,'sha256':hashlib.sha256(data).hexdigest()})
    if size<1500:
        md=Cs(CS_ARCH_X86,CS_MODE_64)
        (OUT/(name+'.asm')).write_text('\n'.join(f'{i.address:x}: {i.bytes.hex()} {i.mnemonic} {i.op_str}' for i in md.disasm(data,va))+'\n',encoding='utf-8')
(OUT/'native-camera-environment-fixtures.json').write_text(json.dumps({'grid_pattern':'(x*97+z*131+257)&65535','height_range':[-2.5,24.0],'samples':samples,'normal_constraints':constraints},indent=2)+'\n',encoding='utf-8')
(OUT/'native-camera-environment-manifest.json').write_text(json.dumps({'source_sha256':sha,'executed':{'terrain_samples':len(samples),'normal_constraints_including_real_terrain_sampler':len(constraints)},'hooks':'only return stop; no helper substitution','data_initialization':'native feature flag pointer at14362c308 points to fixture bool; both branches executed','spans':spans},indent=2)+'\n',encoding='utf-8')
print(json.dumps({'native_samples':len(samples),'native_normal_constraints':len(constraints),'game_helper_hooks':0}))
