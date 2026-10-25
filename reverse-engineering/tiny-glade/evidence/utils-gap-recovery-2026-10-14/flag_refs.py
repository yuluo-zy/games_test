"""Scan original .text RIP displacements for the sampling flag and import slot.

Candidates are confirmed by Capstone RIP operands. A reference is not a write
until its instruction/data flow is checked; Ghidra's missing xrefs are bounded.
"""
from pathlib import Path
import json,sys
sys.path.insert(0,'D:/game/tools/reverse-runtime/utils-gap-20261014/python-deps')
import numpy as np,pefile
from capstone import Cs,CS_ARCH_X86,CS_MODE_64,CS_OP_MEM
from capstone.x86_const import X86_REG_RIP
OUT=Path(__file__).parent;ROOT=OUT.parents[3]
raw=(ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase
section=next(s for s in pe.sections if s.Name.rstrip(b'\0')==b'.text')
data=section.get_data();start=base+section.VirtualAddress
md=Cs(CS_ARCH_X86,CS_MODE_64);md.detail=True
rows={}
for phase in range(4):
    n=(len(data)-phase)//4
    v=np.frombuffer(data,dtype='<i4',count=n,offset=phase)
    ends=v.astype(np.int64)+np.arange(n,dtype=np.int64)*4+phase+4+start
    for target in [0x143638fac,0x14362c308]:
        for trailer in [0,1,4]:
            for index in np.flatnonzero(ends+trailer==target):
                pos=int(index)*4+phase
                for begin in range(max(0,pos-12),pos):
                    for i in md.disasm(data[begin:pos+20],start+begin):
                        if i.address>start+pos:break
                        if any(o.type==CS_OP_MEM and o.mem.base==X86_REG_RIP and i.address+i.size+o.mem.disp==target for o in i.operands):
                            rows[i.address]={'address':hex(i.address),'target':hex(target),'instruction':i.mnemonic+' '+i.op_str}
# A REX-prefixed load also decodes from the following byte as a shorter load.
# Keep the widest candidate ending at the same instruction boundary.
for a in list(rows):
    if a-1 in rows and data[a-start-1] in range(0x40,0x50): del rows[a]
report={'scope':'Original B .text RIP operands only, plus import pointer slot. References confirmed by instruction operands, but no full function-boundary proof. Indirect and computed references outside this boundary remain unresolved.','references':list(rows.values())}
(OUT/'flag-raw-references.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf8')
for r in rows.values():
    a=int(r['address'],16);block=pe.get_data(a-base-16,160)
    (OUT/(r['address']+'-flag.asm.txt')).write_text('\n'.join(f'{i.address:#x}: {i.mnemonic} {i.op_str}' for i in md.disasm(block,a-16))+'\n',encoding='utf8')
print(json.dumps(report,indent=2))
