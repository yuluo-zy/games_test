"""Independent fixture runner for unchanged original picker host.

Uses the frozen author's INPUTS/outputs, but independently sets machine state.
No private geometry helper or game branch is hooked; only std allocation/Vec
growth and CRT memory calls. This is an oracle reproducibility check, not a
candidate Rust acceptance claim.
"""
from pathlib import Path
import json,struct
from verify_native import cpu,stack,stop,return_from_call
from unicorn import UC_HOOK_CODE
from unicorn.x86_const import *

OUT=Path(__file__).resolve().parent
DATA,HEAP=0x78000000,0x7a000000
cpu.mem_map(DATA,0x20000)
cpu.mem_map(HEAP,0x200000)
heap=HEAP
def put(at,fmt,*v):cpu.mem_write(at,struct.pack(fmt,*v))
def u64(at):return struct.unpack('<Q',cpu.mem_read(at,8))[0]
def alloc(n):
    global heap
    p=heap;heap+=max((n+15)&~15,256)
    assert heap<HEAP+0x200000
    return p
def bridge(c,pc,size,user):
    if pc==0x140613c10:return_from_call(alloc(c.reg_read(UC_X86_REG_RCX)))
    elif pc in [0x140613c20,0x14289aa40]:return_from_call()
    elif pc in [0x1406319d0,0x1405627b0]:
        owner=c.reg_read(UC_X86_REG_RCX)
        stride=56 if pc==0x1406319d0 else 64
        cap=max(4,u64(owner)*2);address=alloc(cap*stride)
        count=u64(owner+16)
        if count:cpu.mem_write(address,bytes(cpu.mem_read(u64(owner+8),count*stride)))
        put(owner,'<QQ',cap,address)
        return_from_call()
    elif pc in [0x142923c00,0x142923c10]:
        dest,src,count=[c.reg_read(r) for r in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8]]
        if count:cpu.mem_write(dest,bytes(cpu.mem_read(src,count)))
        return_from_call(dest)
cpu.hook_add(UC_HOOK_CODE,bridge)
def canonical(h,canvas):
    b=bytearray.fromhex(h)
    if canvas:b=b[8:]
    if struct.unpack_from('<I',b)[0]==0:b[4:16]=bytes(12)
    b[50:56]=bytes(6)
    return b.hex()
def run(case):
    global heap
    heap=HEAP
    cpu.mem_write(DATA,bytes(0x18000))
    cpu.mem_write(stack,bytes(0x10000))
    out,state,env,ray,rayrefs,screenref,screen,terrainref,terrain,items=[DATA+i*0x100 for i in range(10)]
    args=case['inputs']
    put(ray,'<6f',*args.get('origin',[0,0,0]),*args.get('direction',[0,0,1]))
    put(rayrefs,'<QQ',ray,ray+12)
    put(screenref,'<Q',screen);put(screen,'<2f',*args.get('screen',[0,0]))
    put(terrainref,'<Q',terrain)
    if args.get('terrain') is not None:put(terrain,'<I3f',1,*args['terrain'])
    put(env,'<4Q',ray,rayrefs,screenref,terrainref)
    put(state,'<3Q',env,items,len(case['widgets']))
    for i,widget in enumerate(case['widgets']):
        wp=DATA+0x2000+i*0x200
        cpu.mem_write(wp,bytes.fromhex(widget['bytes']))
        put(items+i*16,'<QQ',widget['id'],wp)
        extra=widget.get('extra')
        if extra:
            arr=DATA+0x8000+i*0x400
            put(wp+0x90,'<3Q',len(extra),arr,len(extra))
            cpu.mem_write(arr,b''.join(struct.pack('<'+'f'*len(v),*v) for v in extra))
    for reg in [UC_X86_REG_RAX,UC_X86_REG_RBX,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_RSI,UC_X86_REG_RDI,UC_X86_REG_RBP,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,UC_X86_REG_R13,UC_X86_REG_R14,UC_X86_REG_R15]:cpu.reg_write(reg,0)
    for reg in range(UC_X86_REG_XMM0,UC_X86_REG_XMM15+1):cpu.reg_write(reg,0)
    sp=stack+0xf008
    put(sp,'<Q',stop)
    for reg,v in [(UC_X86_REG_RSP,sp),(UC_X86_REG_RCX,out),(UC_X86_REG_RDX,state),(UC_X86_REG_MXCSR,0x1f80),(UC_X86_REG_RFLAGS,0x202)]:cpu.reg_write(reg,v)
    cpu.emu_start(0x140ac3f50,stop,count=1000000)
    assert cpu.reg_read(UC_X86_REG_RIP)==stop
    result={}
    for name,offset,stride in [('ordinary',0,56),('canvas',24,64)]:
        data,count=u64(out+offset+8),u64(out+offset+16)
        result[name]=[canonical(bytes(cpu.mem_read(data+i*stride,stride)).hex(),name=='canvas') for i in range(count)]
    return result
fixtures=json.loads((OUT.parent/'ui-picker/native-picker-all-fixtures.json').read_text(encoding='utf-8'))
results=[]
for case in fixtures:
    value=run(case)
    expected={name:[canonical(h,name=='canvas') for h in case['result'][name]] for name in ['ordinary','canvas']}
    assert value==expected,(case['name'],value,expected)
    results.append({'name':case['name'],'result':value})
(OUT/'native-picker-reproduced.json').write_text(json.dumps(results,indent=2)+'\n',encoding='utf-8')
print(json.dumps({'original_picker_cases':len(results),'mismatches':0,'private_game_helper_hooks':0,'candidate_executed':False}))
