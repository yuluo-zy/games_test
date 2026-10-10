"""真实矩形屋顶入口中观察四边 strip 的输入和输出，核心不替换。"""
from pathlib import Path
import importlib.util, json, struct
from unicorn import UC_HOOK_CODE
from unicorn.x86_const import UC_X86_REG_RBP, UC_X86_REG_RSI, UC_X86_REG_RSP, UC_X86_REG_RDX
root=Path('D:/game/reverse-engineering/tiny-glade')
spec=importlib.util.spec_from_file_location('rect_capture',root/'scripts/roof-ridge/capture_rectangular.py')
oracle=importlib.util.module_from_spec(spec);spec.loader.exec_module(oracle)
contexts=[]
edge_end={}
def hook(cpu,pc,size,user):
    if pc==0x1421af1ac and cpu.reg_read(UC_X86_REG_RSI)<4:
        rbp=cpu.reg_read(UC_X86_REG_RBP)
        contexts.append(dict(side=cpu.reg_read(UC_X86_REG_RSI),
          bottom_rectangle=bytes(cpu.mem_read(rbp+0xa8,24)).hex(),
          top_rectangle=bytes(cpu.mem_read(rbp+0x148,24)).hex(),
          orientation_basis=bytes(cpu.mem_read(rbp+0x3f0,16)).hex()))
    elif pc==0x140c9cdc0 and contexts:
        rsp=cpu.reg_read(UC_X86_REG_RSP)
        callsite=struct.unpack('<Q',cpu.mem_read(rsp,8))[0]-5
        if callsite in [0x1421af24b,0x1421af2c5]:
            field='delta_xz' if callsite==0x1421af24b else 'origin_xz'
            contexts[-1][field]=bytes(cpu.mem_read(cpu.reg_read(UC_X86_REG_RDX),8)).hex()
    elif pc==0x1421afbfb:
        rbp=cpu.reg_read(UC_X86_REG_RBP)
        edge_end['rng']=struct.unpack('<Q',cpu.mem_read(rbp+0x3a8,8))[0]
        edge_end['ordinal']=struct.unpack('<I',cpu.mem_read(rbp+0x474,4))[0]
oracle.cpu.hook_add(UC_HOOK_CODE,hook)
def run(shape,roof,flag):
    contexts.clear();edge_end.clear()
    result=oracle.case(shape,roof,flag)
    edges=[t for t,site in zip(result['tile_records'],result.get('tile_callsites',[])) if site=='0x1421afbbc']
    return dict(source_sha256='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03',
      original_entry_completed=result.get('completed',False),shape=shape.hex(),roof=roof.hex(),contexts=[dict(c) for c in contexts],edge_records=edges,edge_end=dict(edge_end),panic=result.get('panic',[]),error=result.get('error'))
if __name__=='__main__':
    report=run(oracle.shape,oracle.roof,0)
    (root/'evidence/rectangular-edge-reference.json').write_text(json.dumps(report,indent=2),encoding='utf-8')
    for context in report['contexts']:
        print(context['side'],[struct.unpack('<6f',bytes.fromhex(context[k])) for k in ['bottom_rectangle','top_rectangle']],struct.unpack('<4f',bytes.fromhex(context['orientation_basis'])))
    print('edge records',len(report['edge_records']))
