"""Bounded original-byte audit of inlined constructors and noloop adaptors.
Static CFG/call-site evidence, not an execution or FMOD integration test.
"""
from pathlib import Path
import hashlib,json
import pefile
from capstone import Cs,CS_ARCH_X86,CS_MODE_64
from capstone.x86_const import *
ROOT=Path(__file__).resolve().parent
repo=ROOT.parents[4]
raw=(repo/'ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
sha=hashlib.sha256(raw).hexdigest()
assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase
image=pe.get_memory_mapped_image();md=Cs(CS_ARCH_X86,CS_MODE_64);md.detail=True
spans=[]
def span(a,b,label):
    data=image[a-base:b-base]
    ins=list(md.disasm(data,a))
    assert sum(i.size for i in ins)==len(data)
    spans.append({'label':label,'va':hex(a),'size':b-a,'sha256':hashlib.sha256(data).hexdigest()})
    (ROOT/(label+'.asm')).write_text('\n'.join(f'{i.address:x}: {i.bytes.hex()} {i.mnemonic} {i.op_str}' for i in ins)+'\n',encoding='utf-8')
    return ins
inlines=[]
for a,b,call,expected,label in [
    (0x140b6a8ff,0x140b6a971,0x140b6a92c,'event:/SFX/Sheep/Pet','sheep-inline-one'),
    (0x1414d9b58,0x1414d9bc7,0x1414d9b82,'event:/SFX/Sheep/Pet','sheep-inline-two'),
    (0x1417e335f,0x1417e33e2,0x1417e3389,'event:/SFX/StairDraw_LoopTrem','stairs-inline'),
]:
    ins=span(a,b,label);before=[i for i in ins if i.address<call]
    assert any(i.address==call and i.mnemonic=='call' and i.op_str=='0x140a59220' for i in ins)
    stores=[i for i in before if i.mnemonic=='mov' and len(i.operands)==2 and i.operands[0].type==X86_OP_MEM and i.operands[1].type==X86_OP_IMM]
    assert stores[0].operands[1].imm==3 and stores[1].operands[1].imm==50
    names=[]
    for i in before:
        if i.mnemonic=='lea' and len(i.operands)==2 and i.operands[1].type==X86_OP_MEM and i.operands[1].mem.base==X86_REG_RIP:
            ptr=i.address+i.size+i.operands[1].mem.disp
            if image[ptr-base:ptr-base+len(expected)]==expected.encode():names.append(hex(ptr))
    assert len(names)==1
    inlines.append({'label':label,'call':hex(call),'count':3,'threshold_ms':50,'name':expected,'name_va':names[0]})

path=span(0x1409d08d0,0x1409d08d0+1701,'path-noloop')
stores=[i for i in path if i.mnemonic=='mov' and len(i.operands)==2 and i.operands[0].type==X86_OP_MEM and i.operands[0].size==1 and i.operands[1].type==X86_OP_IMM and i.operands[1].imm==1]
expected=[(0x1409d0987,0x38),(0x1409d0a1f,0),(0x1409d0ab7,-0x38),(0x1409d0b64,0x190),(0x1409d0c05,0x150)]
for (va,owner),i in zip(expected,stores,strict=True):
    assert i.address==va and i.operands[0].mem.base==X86_REG_RBP and i.operands[0].mem.disp==owner+0x36
water=span(0x1409ebf20,0x1409ebf20+3955,'water-ambience-inline')
assert any(i.address==0x1409ec151 and i.operands[1].type==X86_OP_IMM and i.operands[1].imm==4 for i in water)
assert any(i.address==0x1409ec27c and i.operands[1].imm==50 for i in water)
assert any(i.address==0x1409ec293 and i.operands[1].imm==0 for i in water)
(ROOT/'native-inline-policy.json').write_text(json.dumps({'original_sha256':sha,'method':'static original bytes and actual field stores; not executed','inline_callers':inlines,'path_noloop':[{'va':hex(va),'returned_owner_rbp_offset':owner,'byte_offset':0x36,'value':1} for va,owner in expected],'water_ambience':{'range_start':0,'range_end':4,'threshold_ms':50,'stops_explicitly':False},'spans':spans},indent=2)+'\n',encoding='utf-8')
print(json.dumps({'inline_callers':len(inlines),'path_noloop_stores':len(stores),'water_ambience':1,'static_only':True}))
