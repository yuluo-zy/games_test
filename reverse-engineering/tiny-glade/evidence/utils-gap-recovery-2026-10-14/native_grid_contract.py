"""Execute B's RasterBufferRef<512> sampling entry without game/library hooks."""
from pathlib import Path
import hashlib,json,random,struct,sys
sys.path.insert(0,'D:/game/tools/reverse-runtime/utils-gap-20261014/python-deps')
OUT=Path(__file__).parent
sys.path.insert(0,str(OUT.parent/'shared-foundation-2026-10-13/utils-base'))
import native_spline_rdp as o
from capstone import Cs,CS_ARCH_X86,CS_MODE_64
from unicorn.x86_const import *
md=Cs(CS_ARCH_X86,CS_MODE_64)
data=o.pe.get_data(0xa14df0,835)
(OUT/'grid.asm.txt').write_text('\n'.join(f'{i.address:#x}: {i.mnemonic} {i.op_str}' for i in md.disasm(data,0x140a14df0))+'\n',encoding='utf8')
buffer=o.HEAP+0x10000
flag_pointer=struct.unpack('<Q',o.u.mem_read(0x14362c308,8))[0]
assert flag_pointer == 0x143638fac
# Asymmetric, non-linear byte field catches axis swap, clamping and lerp order.
values=bytes((r*17+c*31+(r*c)%251)%256 for r in range(512) for c in range(512))
rng=random.Random(141015)
inputs=[(0.,0.,130.,130.),(-65.,-65.,130.,130.),(65.,65.,130.,130.),
        (-100.,100.,130.,130.),(20.,-31.,130.,90.),(float('nan'),11.,130.,130.),
        (float('inf'),float('-inf'),130.,130.),(1.,-1.,0.,0.),(0.,0.,0.,0.),
        (-0.,0.,-130.,130.)]
inputs += [(rng.uniform(-100,100),rng.uniform(-100,100),130.,rng.choice([90.,130.,260.])) for _ in range(40)]
rows=[]
for mode in [0,1]:
    for x,z,ex,ez in inputs:
        sp=o.reset();o.u.mem_write(buffer,values)
        o.u.mem_write(o.INPUT,struct.pack('<QQ',buffer,len(values)))
        o.u.mem_write(flag_pointer,bytes([mode]))
        o.u.reg_write(UC_X86_REG_RCX,o.INPUT)
        for reg,v in [(UC_X86_REG_XMM1,x),(UC_X86_REG_XMM2,z),(UC_X86_REG_XMM3,ex)]:
            o.u.reg_write(reg,o.bits(v))
        o.u.mem_write(sp+0x28,o.pack([ez]))
        o.run(0x140a14df0)
        rows.append({'mode':mode,'input':[o.bits(v) for v in [x,z,ex,ez]],'result':o.u.reg_read(UC_X86_REG_XMM0)&0xffffffff})
(OUT/'grid-native-fixtures.json').write_text(json.dumps({'original_sha256':hashlib.sha256(o.raw).hexdigest(),'flag_pointer_slot':'0x14362c308','flag_byte':hex(flag_pointer),'values_sha256':hashlib.sha256(values).hexdigest(),'pattern':'(row*17+col*31+(row*col)%251)%256','rows':rows,'scope':'B 512x512 byte-grid entry; actual pointed-to branch byte supplied as input. No flag producer, terrain lookup or full game integration claim.'},indent=2)+'\n',encoding='utf8')
print('grid rows',len(rows))
