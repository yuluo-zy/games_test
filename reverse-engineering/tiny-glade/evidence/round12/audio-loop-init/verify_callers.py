"""Bounded original initializer call-site constant dataflow (static only)."""
from pathlib import Path
import hashlib,json
import pefile
from capstone import Cs,CS_ARCH_X86,CS_MODE_64
from capstone.x86_const import *
ROOT=Path(__file__).resolve().parent
repo=ROOT.parents[4]
raw=(repo/'ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;image=pe.get_memory_mapped_image()
md=Cs(CS_ARCH_X86,CS_MODE_64);md.detail=True
def regname(i,r):
    name=i.reg_name(r)
    aliases={'eax':'rax','ecx':'rcx','edx':'rdx','esi':'rsi','edi':'rdi','r8d':'r8','r9d':'r9','r10d':'r10','r11d':'r11'}
    return aliases.get(name,name)
functions=[]
for line in (repo/'reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv').read_text(encoding='utf-8').splitlines():
    if len(line)<650 and '::systems::audio::' in line and 'init_system' in line:
        a,n,_,_,name=line.split('\t',4);functions.append((int(a,16),int(n),name))
calls=[];spans=[]
for va,size,name in functions:
    data=image[va-base:va-base+size]
    regs={};mem={}
    for i in md.disasm(data,va):
        op=i.operands
        if i.mnemonic=='call':
            if i.op_str=='0x140a59220':
                desc=regs.get('r8')
                row={'function':name,'call_va':hex(i.address),'count':None,'threshold_ms':None,'name':None}
                if isinstance(desc,tuple) and desc[0]=='rbp':
                    offset=desc[1]
                    row['count']=mem.get(offset+0x10)
                    row['threshold_ms']=mem.get(offset+0x18)
                    ptr,n=mem.get(offset),mem.get(offset+8)
                    if isinstance(ptr,int) and isinstance(n,int) and base<=ptr<base+len(image) and n<300:
                        row['name']=image[ptr-base:ptr-base+n].decode('utf-8')
                calls.append(row)
                out=regs.get('rcx')
                if isinstance(out,tuple) and out[0]=='rbp':
                    for at in range(out[1],out[1]+56):mem.pop(at,None)
            for register in ['rax','rcx','rdx','r8','r9','r10','r11']:regs.pop(register,None)
        elif i.mnemonic in ['mov','lea'] and len(op)==2:
            dest,src=op
            value=None
            if src.type==X86_OP_IMM:value=src.imm
            elif src.type==X86_OP_REG:value=regs.get(regname(i,src.reg))
            elif src.type==X86_OP_MEM:
                if i.mnemonic=='lea':
                    if src.mem.base==X86_REG_RIP:value=i.address+i.size+src.mem.disp
                    elif src.mem.base==X86_REG_RBP and src.mem.index==0:value=('rbp',src.mem.disp)
                elif src.mem.base==X86_REG_RBP and src.mem.index==0:value=mem.get(src.mem.disp)
            if dest.type==X86_OP_REG:
                r=regname(i,dest.reg)
                if value is None:regs.pop(r,None)
                else:regs[r]=value
            elif dest.type==X86_OP_MEM and dest.mem.base==X86_REG_RBP and dest.mem.index==0:
                if value is None:mem.pop(dest.mem.disp,None)
                else:mem[dest.mem.disp]=value
        elif i.mnemonic=='xor' and len(op)==2 and op[0].type==X86_OP_REG and op[1].type==X86_OP_REG and op[0].reg==op[1].reg:
            regs[regname(i,op[0].reg)]=0
    if any(row['function']==name for row in calls):spans.append({'name':name,'va':hex(va),'size':size,'sha256':hashlib.sha256(data).hexdigest()})
unknown=[row for row in calls if row['count'] is None or row['threshold_ms'] is None or row['name'] is None]
assert not unknown,unknown
assert {row['count'] for row in calls}=={3}
assert {row['threshold_ms'] for row in calls}=={50}
(ROOT/'native-caller-descriptors.json').write_text(json.dumps({'original_sha256':sha,'method':'static original constant stores/lea to actual call descriptor; not dynamic initializer execution','calls':calls,'spans':spans,'not_in_direct_calls':'WaterAmbience inlines range0..4 and50ms, FirstPerson and Sheep init inlined in Bevy wrappers; separately verified'},indent=2)+'\n',encoding='utf-8')
print(json.dumps({'verified_direct_call_sites':len(calls),'initializer_functions':len(spans),'descriptor_counts':[3],'threshold_ms':[50],'unknown':len(unknown)}))
