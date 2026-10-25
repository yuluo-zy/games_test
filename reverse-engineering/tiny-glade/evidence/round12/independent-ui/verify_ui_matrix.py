"""Independent original ui_update execution including real get_ui_volume.

No candidate Rust/Python state-transition model is executed. Explicit expected
rows exercise selected native branches, with standard Events storage prepared.
"""
from pathlib import Path
import json, struct
from verify_native import cpu, stack, stop, stop_pcs, return_from_call
from unicorn import UC_HOOK_CODE
from unicorn.x86_const import *

OUT=Path(__file__).resolve().parent
D=0x74000000
cpu.mem_map(D,0x20000)
state,actions=D+0x1000,D+0x1200

def put(at,fmt,*v):cpu.mem_write(at,struct.pack(fmt,*v))
def u64(at):return struct.unpack('<Q',cpu.mem_read(at,8))[0]
def bridge(c,pc,size,user):
    if pc==0x140c16a00:return_from_call(0) # tracing disabled
    elif pc==0x140b79e90:
        put(state+0x80,'<Q',0) # standard HashMap.clear, not a game state branch
        return_from_call()
cpu.hook_add(UC_HOOK_CODE,bridge)

def intersection(widget):
    result=bytearray(56)
    struct.pack_into('<I3fQQ4fBB',result,0,1,0.,1.,0.,widget,0,.5,0.,0.,0.,0,1)
    return bytes(result)

def run(tag,phase,action,hit,focus=(),escape=False,shutdown=False):
    cpu.mem_write(D,bytes(0x18000))
    put(actions,'<If',action,0.)
    refs=[D+0x100+i*0x30 for i in range(8)]
    for i,address in enumerate([state,actions]):
        put(refs[i],'<Q',address)
        put(refs[i]+0x10,'<Q',D+0xb00+i*4)
    events=[]
    for i,values in [(2,list(focus)),(3,[0]*int(escape)),(4,[0]*int(shutdown)),(5,[]),(6,[]),(7,[])]:
        header,buffer,counter=D+0x2000+i*0x100,D+0x6000+i*0x500,D+0xa00+i*8
        put(header,'<9Q',64,buffer,len(values),0,64,buffer+0x200,0,len(values),len(values))
        stride=16 if i in (2,6) else (8 if i in (3,4) else 32)
        for j,value in enumerate(values):
            put(buffer+j*stride,'<Q',j)
            if stride>8:put(buffer+j*stride+8,'<B',value)
        if i<5:put(refs[i],'<QQ',counter,header)
        else:
            put(refs[i],'<Q',header)
            put(refs[i]+0x10,'<Q',D+0xb00+i*4)
            events.append((header,16 if i==6 else 32))
    put(state,'<3Q',8,D+0x3500,int(hit is not None))
    if hit is not None:cpu.mem_write(D+0x3500,intersection(hit))
    put(state+0x18,'<BB',tag,phase)
    put(state+0x20,'<Q',D+0x4000)
    cpu.mem_write(state+0x28,intersection(11))
    # Real original lookup visits both prepared collision buckets and compares
    # actual keys. Arc ownership increments execute in original helper bytes.
    ctrl=D+0x5000
    put(state+0x68,'<4Q',ctrl,3,0,2)
    cpu.mem_write(ctrl,bytes([0,0]+[255]*30))
    for j,key in enumerate([11,22]):
        arc=D+0x4000+j*0x200
        put(ctrl-16*(j+1),'<QQ',key,arc)
        put(arc,'<QQ',100,1)
        for slot in range(4):put(arc+0x28+slot*24,'<6I',101+j*100+slot,55,66,77,88,99)
        put(arc+0x88,'<B',0) # no preview
        put(arc+0x98,'<B',7) # Empty geometry (intersection supplied by previous picker)
    sp=stack+0xf008
    put(sp,'<Q',stop)
    for i in range(4,8):put(sp+0x28+(i-4)*8,'<Q',refs[i])
    for reg,value in zip([UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_R9],refs):cpu.reg_write(reg,value)
    cpu.reg_write(UC_X86_REG_RSP,sp)
    cpu.emu_start(0x140b2de60,stop,count=100000)
    assert cpu.reg_read(UC_X86_REG_RIP)==stop,hex(cpu.reg_read(UC_X86_REG_RIP))
    sounds=[]
    header,stride=events[2]
    for j in range(u64(header+0x30)):
        payload=bytes(cpu.mem_read(u64(header+0x28)+stride*j,24))
        sounds.append(list(struct.unpack('<6I',payload)))
    return {'tag':cpu.mem_read(state+0x18,1)[0],'phase':cpu.mem_read(state+0x19,1)[0],
            'sounds':sounds,'registry_size':u64(state+0x80),'active_arc':u64(state+0x20)-D}

results=[]
# Hand-selected expected native state endpoints; no duplicated Rust algorithm.
for force in ['none','escape','shutdown','focusfalse','focus_true_then_false','focus_false_then_true']:
    kw={}
    if force=='escape':kw['escape']=True
    if force=='shutdown':kw['shutdown']=True
    if force=='focusfalse':kw['focus']=[0]
    if force=='focus_true_then_false':kw['focus']=[1,0]
    if force=='focus_false_then_true':kw['focus']=[0,1]
    forced=force not in ['none','focus_false_then_true']
    for hit in [None,11,22]:
        for oldphase in [3,4]:
            for action in [1,3]: # held and released original digital tags
                value=run(2,oldphase,action,hit,**kw)
                ended=forced or action==3
                expected_phase=int(hit==11) if ended else 4
                assert (value['tag'],value['phase'])==(2,expected_phase),(force,hit,oldphase,action,value)
                expected_sounds=[] if not ended else [[103 if hit!=11 else 104,55,66,77,88,99]]
                assert value['sounds']==expected_sounds,(force,hit,oldphase,action,value)
                assert value['registry_size']==0
                results.append({'force':force,'hit':hit,'old_phase':oldphase,'action':action,'result':value})
# Ordinary real hover and start invoke original game lookup, not its oracle stub.
for tag,phase,action,expected in [(0,2,4,(1,1)),(1,1,0,(2,3)),(2,0,4,(1,1)),(2,1,4,(1,1)),(2,2,1,(2,2))]:
    value=run(tag,phase,action,11)
    assert (value['tag'],value['phase'])==expected,(tag,phase,action,value)
    assert value['registry_size']==0
    results.append({'initial':[tag,phase,action],'result':value})
(OUT/'native-ui-selected-fixtures.json').write_text(json.dumps(results,indent=2)+'\n',encoding='utf-8')
print(json.dumps({'ui_update_selected_cases':len(results),'mismatches':0,'game_get_ui_volume':'executed unchanged','sound_words':6}))
