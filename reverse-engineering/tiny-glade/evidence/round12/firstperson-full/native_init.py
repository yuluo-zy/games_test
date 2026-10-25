"""Original instantiate_loop plus original iterator, disabled-studio branch.
Only allocator/Vec growth/CRT copies bridged; verifies actual count and50ms.
"""
from native_oracle import cpu,D,H,S,STOP,put,u64,ret,OUT
from unicorn import UC_HOOK_CODE
from unicorn.x86_const import *
import json
heap=H
def alloc(n):
    global heap
    result=heap;heap+=max((n+15)&~15,256)
    return result
def bridge(c,pc,size,user):
    if pc==0x140613c10:ret(alloc(c.reg_read(UC_X86_REG_RCX)))
    elif pc in [0x142923c00,0x142923c10]:
        dest,source,count=[c.reg_read(r) for r in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8]]
        if count:c.mem_write(dest,bytes(c.mem_read(source,count)))
        ret(dest)
    elif pc==0x1428a9a60:
        vec=c.reg_read(UC_X86_REG_RCX);count=u64(vec+16)
        capacity=max(u64(vec)*2,count+1);address=alloc(capacity*48)
        if count:c.mem_write(address,bytes(c.mem_read(u64(vec+8),count*48)))
        put(vec,'<QQ',capacity,address);ret()
cpu.hook_add(UC_HOOK_CODE,bridge)
results=[]
for count in [0,1,3,7]:
    heap=H;cpu.mem_write(D,bytes(0x18000));cpu.mem_write(S,bytes(0x20000))
    output,core,desc=D+0x1000,D+0x2000,D+0x3000
    put(core,'<Q',0)
    put(desc,'<3QI',0x142ad8e28,25,count,50)
    sp=S+0x1f008;put(sp,'<Q',STOP)
    for r,v in [(UC_X86_REG_RCX,output),(UC_X86_REG_RDX,core),(UC_X86_REG_R8,desc),(UC_X86_REG_RSP,sp)]:cpu.reg_write(r,v)
    cpu.emu_start(0x140a59220,STOP,count=100000)
    assert cpu.reg_read(UC_X86_REG_RIP)==STOP
    observed=u64(output+16)
    threshold=int.from_bytes(cpu.mem_read(output+0x30,4),'little')
    assert observed==count+1,(count,observed)
    assert threshold==50
    results.append({'descriptor_count':count,'actual_instances':observed,'threshold_ms':threshold,'flags':list(cpu.mem_read(output+0x34,3))})
(OUT/'native-loop-init-fixtures.json').write_text(json.dumps(results,indent=2)+'\n',encoding='utf-8')
print(json.dumps({'native_loop_init_cases':len(results),'results':results}))
