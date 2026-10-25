"""Independent original retain/sort region execution; frozen fixture inputs."""
from pathlib import Path
import json,struct
from verify_picker import cpu,stack,DATA,put,u64,canonical
from verify_native import stop_pcs
from unicorn.x86_const import *

OUT=Path(__file__).resolve().parent
END=0x140838eec
stop_pcs.add(END)
def run(case):
    cpu.mem_write(DATA,bytes(0x18000))
    cpu.mem_write(stack,bytes(0x10000))
    vec,items,canvas,viewref,camera,depthref,value=[DATA+i*0x1000 for i in range(7)]
    ordinary,canvases=case['ordinary'],case['canvases']
    put(vec,'<3Q',128,items,len(ordinary))
    if ordinary:cpu.mem_write(items,b''.join(bytes.fromhex(h) for h in ordinary))
    for i,(layer,h) in enumerate(canvases):
        layerptr=DATA+0x8000+i*8
        cpu.mem_write(layerptr,bytes([layer]))
        put(canvas+i*64,'<Q',layerptr)
        cpu.mem_write(canvas+i*64+8,bytes.fromhex(h))
    args=case['inputs']
    put(viewref,'<Q',camera)
    put(depthref,'<Q',value)
    put(value+4,'<f',args.get('depth',0))
    put(camera+0x80,'<16f',*args.get('view',[1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1]))
    sp=stack+0x8000;bp=sp+0x80
    put(bp-0x30,'<6f',*args.get('origin',[0,0,0]),*args.get('direction',[0,0,1]))
    for at,v in [(bp+0x20,viewref),(bp+0x140,depthref),(bp+0x68,canvas),(bp+0x60,0)]:put(at,'<Q',v)
    for reg in [UC_X86_REG_RAX,UC_X86_REG_RBX,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_RSI,UC_X86_REG_RDI,UC_X86_REG_RBP,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,UC_X86_REG_R13,UC_X86_REG_R14,UC_X86_REG_R15]:cpu.reg_write(reg,0)
    for reg,v in [(UC_X86_REG_RSP,sp),(UC_X86_REG_RBP,bp),(UC_X86_REG_RBX,vec),(UC_X86_REG_RSI,depthref),(UC_X86_REG_R14,len(canvases)),(UC_X86_REG_R15,DATA+0x9000),(UC_X86_REG_MXCSR,0x1f80),(UC_X86_REG_RFLAGS,0x202)]:cpu.reg_write(reg,v)
    cpu.emu_start(0x140838bfd,END,count=1000000)
    assert cpu.reg_read(UC_X86_REG_RIP)==END
    count=cpu.reg_read(UC_X86_REG_RDX)
    return [canonical(bytes(cpu.mem_read(items+i*56,56)).hex(),False) for i in range(count)]
fixtures=json.loads((OUT.parent/'ui-picker/native-arbitration-fixtures.json').read_text(encoding='utf-8'))
results=[]
for case in fixtures:
    value=run(case)
    expected=[canonical(h,False) for h in case['result']]
    assert value==expected,(case['name'],value,expected)
    results.append({'name':case['name'],'result':value})
(OUT/'native-arbitration-reproduced.json').write_text(json.dumps(results,indent=2)+'\n',encoding='utf-8')
print(json.dumps({'original_arbitration_cases':len(results),'mismatches':0,'private_game_helper_hooks':0,'candidate_executed':False}))
