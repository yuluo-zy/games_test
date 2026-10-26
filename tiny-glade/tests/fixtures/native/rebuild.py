#!/usr/bin/env python3
"""Recover deleted fixed captures and recapture scalar business paths from B RNE.
Run with tools/venv/bin/python. No candidate Rust implementation is invoked.
"""
from pathlib import Path
import hashlib, json, struct, subprocess, itertools
import pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import *
HERE=Path(__file__).resolve().parent
GAME=HERE.parents[3]
TARGET=GAME/'tiny-glade/target/debug/deps'
ARTIFACTS={'audio_integration-3fa5525f7d2180fa': '584523e44e0b18a5c45d30e615a31e97d4412af15232bedc27850e50b0987123', 'decorator_extra_native-782fcfb6f2156f54': '306359c1f6483b378c2c453465b0004343a07eea71e0d5aa9b140c01c1c9e7e5', 'signifier_animation_native-0f0801de00b1b6d8': '0c689b1b8258f1aee6a739df7fa30ef5e7deae6fb2f3e9a7cf207f91d996d7c6', 'toolbar-34d90b3615f127e3': 'fa72fc2d71705f2717518e34f3cd83c6d8a350a7f547c2e280fa8bd8260fab7e'}
CAPTURE_SEEDS={'native-preview-kind.bin': 'e78d3d30c9021904604fbc45e8f4878db5f1134a1d58fd6a924b9868999a52a6', 'native-from-fixtures.bin': 'b6a61beed00166f1091744673b09ec044a97e609cf2ecf79f5965c628b560995', 'native-animation.bin': '1e43e474562c9ea29947da31843a988423c2989385b0ef00d976282a8c1e0fbb', 'photo-tree-full.txt': 'd841289702f1972edc2d60079380ca50382b8624afffc7f51262730ddd8e4357', 'photo-tree-demo.txt': '3f1c2b27093e245ca3d68e6bf9f9c69b73806bf1c7337bb4c7bc84b7b4175a4e'}
records={}
payloads={}
def digest(b): return hashlib.sha256(b).hexdigest()
def save(name,data,**source):
    payloads[name]=data
    records[name]={'bytes':len(data),'sha256':digest(data),**source}
def recover(stem,address,length,name):
    path=TARGET/stem
    # Cargo may replace a test executable after rebuilding. A checked-in fixed
    # capture remains usable only with its previously recorded exact digest.
    blob=path.read_bytes() if path.exists() else b''
    source_read='checked-in fixed capture seed'
    payload=None
    if digest(blob)==ARTIFACTS[stem]:
        off=32
        assert struct.unpack_from('<I',blob)[0]==0xfeedfacf
        for _ in range(struct.unpack_from('<I',blob,16)[0]):
            cmd,size=struct.unpack_from('<II',blob,off)
            if cmd==0x19:
                vm,sz,fo,fs=struct.unpack_from('<QQQQ',blob,off+24)
                if vm<=address and address+length<=vm+fs:
                    payload=blob[fo+address-vm:fo+address-vm+length]
                    source_read='exact retained MachO constant extraction'
                    break
            off+=size
    if payload is None: payload=(HERE/name).read_bytes()
    assert len(payload)==length and digest(payload)==CAPTURE_SEEDS[name], 'fixed capture identity changed'
    save(name,payload,method='historical retained test-artifact capture; not a new native execution',
         source_read=source_read,artifact=str(path.relative_to(GAME)),
         artifact_sha256=ARTIFACTS[stem],vm_address=hex(address))
recover('audio_integration-3fa5525f7d2180fa',0x100122bb9,131072,'native-preview-kind.bin')
recover('decorator_extra_native-782fcfb6f2156f54',0x10007eaa6,576*165,'native-from-fixtures.bin')
recover('signifier_animation_native-0f0801de00b1b6d8',0x1001c0238,1728*28,'native-animation.bin')
recover('toolbar-34d90b3615f127e3',0x100a02415,0x8f7,'photo-tree-full.txt')
recover('toolbar-34d90b3615f127e3',0x100a02d0c,0x544,'photo-tree-demo.txt')
# Original instruction bytes, not the patched executable or candidate logic.
binary=GAME/'B版本内容/tiny-glade.rne';raw=binary.read_bytes()
sha=digest(raw)
assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw); base=pe.OPTIONAL_HEADER.ImageBase
cpu=Uc(UC_ARCH_X86,UC_MODE_64)
cpu.mem_map(base,(pe.OPTIONAL_HEADER.SizeOfImage+4095)&~4095)
cpu.mem_write(base,pe.get_memory_mapped_image())
DATA,STACK,STOP=0x20000000,0x30000000,0x40000000
cpu.mem_map(DATA,0x10000);cpu.mem_map(STACK,0x10000);cpu.mem_map(STOP,0x1000)
SP=STACK+0x8008
cpu.reg_write(UC_X86_REG_MXCSR,0x1f80);ctx=cpu.context_save()
def reg(r,v):cpu.reg_write(r,v)
def put(p,fmt,*v):cpu.mem_write(p,struct.pack('<'+fmt,*v))
def get(p,fmt):return struct.unpack('<'+fmt,cpu.mem_read(p,struct.calcsize('<'+fmt)))[0]
def bits(f):return struct.unpack('<I',struct.pack('<f',f))[0]
def reset():
    cpu.context_restore(ctx);cpu.mem_write(DATA,bytes(0x10000));cpu.mem_write(STACK,bytes(0x1000))
    reg(UC_X86_REG_RSP,SP);put(SP,'Q',STOP)
def run(start,end):
    cpu.emu_start(start,end,count=10000)
    assert cpu.reg_read(UC_X86_REG_RIP)==end,hex(cpu.reg_read(UC_X86_REG_RIP))
def native_source(ranges,**extra):
    return dict(method='original B RNE instructions in Unicorn; resolved component/resource storage',
        binary='B版本内容/tiny-glade.rne',binary_sha256=sha,ranges=ranges,**extra)
# Focus: resolve query membership/storage, execute native optional target branch,
# native easing call and the exact multiply/add sequence. Query iteration excluded.
values=[0.,.1,.25,.5,.75,1.,-.25,1.25,float('nan'),float('inf'),-float('inf')]
scales=[-1.,0.,.25,1.,2.,float('inf')]
customs=[None,(0,.7),(1,.7),(1,float('nan')),(2,2.),(255,-1.),(1,2.)]
out=bytearray()
for slider,reset_marker,custom,select,scale in itertools.product([0,1],[0,1],customs,values,scales):
    reset();run(0x14233bb5e,0x14233bb77)
    reg(UC_X86_REG_RCX,DATA+0x200 if custom else 0)
    reg(UC_X86_REG_RAX,DATA);reg(UC_X86_REG_RSI,DATA+0x100);reg(UC_X86_REG_R13,DATA+0x300)
    put(SP+0x38,'I',reset_marker);put(SP+0x3c,'I',slider)
    put(DATA+4,'f',select);put(DATA+0x110,'f',scale)
    if custom:put(DATA+0x200,'If',*custom)
    run(0x14233bf6e,0x14233bbba)
    kind,value=custom or (0,0.)
    out+=struct.pack('<4BfffI',slider,reset_marker,int(custom is not None),kind,value,select,scale,get(DATA+0x110,'I'))
save('native-focus.bin',out,**native_source(['14233bb5e..14233bb77','14233bf6e..14233bfa0','14233bb80..14233bbba'],cases=1848,bridges=['query resolution only; easing executes original 140c95ef0']))
out=bytearray()
for appear,scale in itertools.product(values,scales):
    reset();reg(UC_X86_REG_R13,DATA);reg(UC_X86_REG_RAX,0);reg(UC_X86_REG_RBX,DATA+0x100)
    reg(UC_X86_REG_RBP,DATA+0x200);reg(UC_X86_REG_RSI,0)
    reg(UC_X86_REG_XMM6,bits(1.));reg(UC_X86_REG_XMM7,0)
    put(DATA,'f',appear);put(DATA+0x110,'f',scale);run(0x14233c0e4,0x14233c11e)
    out+=struct.pack('<ffI',appear,scale,get(DATA+0x110,'I'))
save('native-appear.bin',out,**native_source(['14233c0e4..14233c11e'],cases=66,bridges=['query resolution; original loop invariant xmm6=1/xmm7=0']))
# Whole undo system, all nine resource wrappers and genuine helper calls.
# Preallocate events/anchors; no allocation, query, sound or easing hooks.
out=bytearray()
for alignment,auto,scenario,icon,progress,delta in itertools.product(range(4),[0,1],range(5),[.5,1.,2.],[0.,.2,.5,.8,1.],[0.,.016,.1,.3,.5,1.]):
    reset();cursor,menu,config,time,window,actions,undo,taffy,events=[DATA+i*0x500 for i in range(9)]
    wrappers=DATA+0x4000;tick=DATA+0x4f00;anchors=DATA+0x6000;eventbuf=DATA+0x7000
    objects=[cursor,menu,config,time,window,actions,undo,taffy,events]
    for i,obj in enumerate(objects):
        put(wrappers+i*0x30,'Q',obj);put(wrappers+i*0x30+0x10,'Q',tick)
        put(wrappers+i*0x30+0x1c,'I',1)
    reg(UC_X86_REG_RCX,wrappers);reg(UC_X86_REG_RDX,wrappers+0x30)
    reg(UC_X86_REG_R8,wrappers+0x60);reg(UC_X86_REG_R9,wrappers+0x90)
    for i in range(4,9):put(SP+0x28+(i-4)*8,'Q',wrappers+i*0x30)
    width,height=1280,720
    # Points hit each corresponding edge or deliberately miss the activation box.
    points=[(width*.5,0.),(float(width),height*.5),(0.,height*.5),(width*.5,float(height))]
    x,y=points[alignment] if scenario!=1 else (width*.5,height*.5)
    menu_tag=3 if scenario==2 else (10 if scenario==4 else 2)
    phase=1 if scenario==3 else 4
    put(cursor,'ff',x,y);put(menu,'II',menu_tag,0);put(config+0xe8,'f',icon)
    put(config+0xf0,'BB',alignment,auto);put(time+0x58,'f',delta)
    put(window,'II',width,height);put(actions,'If',phase,0.)
    node=0x100000001;put(undo,'Q',node);put(undo+0x10,'f',progress)
    put(taffy+0xa0,'QQ',anchors,3)
    for i in range(3):put(anchors+i*40+8,'ffQ',17.,17.,node if i<2 else node+1)
    put(events+0x20,'QQQ',16,eventbuf,0);put(events+0x40,'Q',0)
    run(0x1423449f0,STOP)
    count=get(events+0x30,'Q');assert count<=1
    sound=get(eventbuf,'I') if count else 0xffffffff
    out+=struct.pack('<4BI2I5f4I',alignment,auto,menu_tag,0,phase,width,height,icon,progress,delta,x,y,
                     get(undo+0x10,'I'),get(anchors+8,'I'),get(anchors+12,'I'),sound)
    assert bytes(cpu.mem_read(anchors+40+8,8))==struct.pack('<ff',17.,17.)
save('native-auto-hide.bin',out,**native_source(['1423449f0..142344de8'],cases=3600,bridges=['resolved resource wrappers; preallocated event vector/three anchors; no function hooks']))
# Resolved one-entity query metadata. Original generation, archetype mask,
# precedence, tag comparison and drag jump table all execute without hooks.
def sound_setup(mask,tag,entity=1<<32):
    reset();world=DATA;meta=DATA+0x500;frame=DATA+0x1000;queries=DATA+0x2000
    ev=DATA+0x4000;evbuf=DATA+0x4500;event=DATA+0x5000;tick=DATA+0x5100
    put(world+8,'QQ',meta,1);put(meta,'IIIII',1,0,0,0,0)
    offsets=[0x28,0x30,0x18,8,0x10];flags=[8,1,16,2,4]
    for i,(off,flag) in enumerate(zip(offsets,flags)):
        q=queries+i*0x200;put(frame+off,'Q',q);put(q+0x28,'Q',q+0x180)
        put(q+0x38,'Q',1);put(q+0x180,'Q',int(bool(mask&flag)))
    for off in [0x60,0x50,0x40,0x20]:put(frame+off,'Q',world)
    archetypes=DATA+0x6000;columns=DATA+0x6200;indices=DATA+0x6400;tags=DATA+0x6500
    put(world+0x1a8,'Q',archetypes);put(archetypes+0x18,'Q',columns)
    put(archetypes+0x38,'Q',indices);put(indices,'Q',0xffffffffffffffff)
    put(columns+0x10,'Q',tags);put(tags,'B',tag)
    put(frame+0x38,'Q',tick);put(frame+0x58,'I',1)
    put(ev+0x20,'QQQ',16,evbuf,0);put(ev+0x40,'Q',0);put(event+8,'Q',entity)
    reg(UC_X86_REG_RBP,frame);reg(UC_X86_REG_R13,ev);reg(UC_X86_REG_RSI,world)
    return frame,event,ev,evbuf
entries=[(mask,tag,1<<32) for mask in range(32) for tag in range(18)]+[(0,0,2<<32),(0,0,(1<<32)|100000)]
focus=[];click=[]
for mask,tag,entity in entries:
    frame,event,ev,evbuf=sound_setup(mask,tag,entity)
    reg(UC_X86_REG_RDI,event);reg(UC_X86_REG_RCX,0)
    run(0x14233ab6f,0x14233ab20);focus.append(cpu.reg_read(UC_X86_REG_R12))
    frame,event,ev,evbuf=sound_setup(mask,tag,entity);reg(UC_X86_REG_RDX,event)
    run(0x14233ad73,0x14233ad38)
    if get(ev+0x30,'Q'):click.append(get(evbuf,'I'))
# Two loop boundaries (emit vs next event); hook only stops execution, injects nothing.
stop_points={0x14233b130,0x14233b105}
def stop_hook(c,pc,size,user):
    if pc in stop_points:c.emu_stop()
hook=cpu.hook_add(UC_HOOK_CODE,stop_hook)
cache={}
def drag_kind(slider,state):
    key=(slider,state)
    if key not in cache:
        frame,event,ev,evbuf=sound_setup(8 if slider else 0,0)
        put(event+0x10,'B',state);reg(UC_X86_REG_RAX,event)
        cpu.emu_start(0x14233b171,STOP,count=10000)
        pc=cpu.reg_read(UC_X86_REG_RIP);assert pc in stop_points
        cache[key]=get(frame+0x40,'I') if pc==0x14233b105 else None
    return cache[key]
drags=[]
for mask,tag,entity in entries:
    for state in range(256):
        value=drag_kind(bool(mask&8) and entity==1<<32,state)
        if value is not None:drags.append(value)
cpu.hook_del(hook)
result=focus+click+drags
assert len(result)==73915,len(result)
save('native-sounds-output.bin',struct.pack('<'+'I'*len(result),*result),**native_source(
    ['14233ab6f..14233ab20','14233ad73..14233ad38','14233b171..14233b105/14233b130'],
    entities=578,events=len(result),bridges=['resolved query metadata/preallocated event vector','original EventReader iteration replaced by ordered focus/click/drag input list; per-event predicates execute original instructions']))
for name,data in payloads.items():(HERE/name).write_bytes(data)
(HERE/'provenance.json').write_text(json.dumps({'schema':1,'generator_sha256':digest(Path(__file__).read_bytes()),'files':records},ensure_ascii=False,indent=2)+'\n')
print(json.dumps({name:info['bytes'] for name,info in records.items()},indent=2))
