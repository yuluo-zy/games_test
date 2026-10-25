"""Original SheepControl runtime and real snapshot apply/capture.
32 cases have no Egui context. Another48 execute original picking after a
generic Egui-show bridge that does not invoke the GUI content callback.
Those48 also bridge std formatting and the independently restored UiState
registration boundary with stable fixture IDs; all Sheep control branches,
actual Query iteration and original capture/apply execute without substitution.
"""
from pathlib import Path
import json,hashlib,struct
import pefile
from unicorn import *
from unicorn.x86_const import *
OUT=Path(__file__).resolve().parent
raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;image=pe.get_memory_mapped_image()
c=Uc(UC_ARCH_X86,UC_MODE_64);c.mem_map(base,(len(image)+4095)&~4095);c.mem_write(base,image)
D,H,S,STOP=0x30000000,0x40000000,0x50000000,0x60000000
for at,n in [(D,0x40000),(H,0x400000),(S,0x20000),(STOP,0x1000)]:c.mem_map(at,n)
def put(a,f,*x):c.mem_write(a,struct.pack(f,*x))
def u64(a):return struct.unpack('<Q',c.mem_read(a,8))[0]
def ret(v=None):
    if v is not None:c.reg_write(UC_X86_REG_RAX,v)
    sp=c.reg_read(UC_X86_REG_RSP);c.reg_write(UC_X86_REG_RSP,sp+8);c.reg_write(UC_X86_REG_RIP,u64(sp))
heap=H;key=False;log=[];widget_index=0
def alloc(n):
    global heap
    a=heap;heap+=max(256,(n+15)&~15);return a
def hook(cpu,pc,n,_):
    global widget_index
    if pc==STOP:cpu.emu_stop();return
    if pc==0x140613c10:ret(alloc(cpu.reg_read(UC_X86_REG_RCX)))
    elif pc==0x140613c20:
        log.append({'free_bytes':cpu.reg_read(UC_X86_REG_RDX)});ret()
    elif pc in [0x142923c00,0x142923c10]:
        d,s,n=[cpu.reg_read(r) for r in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8]]
        if n:cpu.mem_write(d,bytes(cpu.mem_read(s,n)))
        ret(d)
    elif pc==0x14089aef0:
        log.append({'key_native_tag':struct.unpack('<I',cpu.mem_read(cpu.reg_read(UC_X86_REG_RDX),4))[0]});ret(int(key))
    elif pc==0x140adb2f0:
        cpu.mem_write(cpu.reg_read(UC_X86_REG_RCX),bytes(0x188));ret()
    elif pc==0x140adb610:
        # Egui's host does not invoke its UI-content callback in these cases.
        # No SheepControl GUI callback or game helper is substituted.
        dest=cpu.reg_read(UC_X86_REG_RCX);cpu.mem_write(dest,bytes(0x48));put(dest+0x40,'<I',2);ret()
    elif pc==0x140d79f90:
        # std formatting bridge for the original static prefix + u64 ID.
        dest,args=cpu.reg_read(UC_X86_REG_RCX),cpu.reg_read(UC_X86_REG_RDX)
        pieces=u64(args);p,n=struct.unpack('<QQ',cpu.mem_read(pieces,16));arg=u64(u64(args+16))
        text=bytes(cpu.mem_read(p,n))+str(u64(arg)).encode();buffer=alloc(len(text));cpu.mem_write(buffer,text);put(dest,'<3Q',len(text),buffer,len(text));ret(dest)
    elif pc in [0x1409231f0,0x1409236d0]:
        ui=cpu.reg_read(UC_X86_REG_RCX)
        params=cpu.reg_read(UC_X86_REG_R8 if pc==0x1409231f0 else UC_X86_REG_R9)
        widget_id=100+widget_index if pc==0x1409231f0 else 999
        widget_index+=1
        log.append({'widget':widget_id,'volume':bytes(cpu.mem_read(params+0x88,32)).hex(),'extra':bytes(cpu.mem_read(params+0x78,16)).hex(),'flags':bytes(cpu.mem_read(params+0xa8,2)).hex()})
        ret(ui);cpu.reg_write(UC_X86_REG_RDX,widget_id)
    elif pc==0x1428d9760:raise AssertionError('original unique query panic')
c.hook_add(UC_HOOK_CODE,hook)

def dense(query,state,world,data,strides,count):
    put(query,'<3Q',state,world,12345)
    put(state+0xf8,'<QQB',state+0x300,1,1);put(state+0x300,'<I',0)
    put(world+0x1a8,'<Q',world+0x400)
    table=world+0x400;columns=world+0x800;mapping=world+0xc00
    put(table+0x10,'<QQ',count,columns);put(table+0x38,'<Q',mapping)
    for i,(address,stride) in enumerate(zip(data,strides)):
        put(state+0x110+i*8,'<Q',i);put(mapping+i*8,'<Q',(~i)&((1<<64)-1))
        put(columns+i*0x30+0x10,'<Q',address);put(columns+i*0x30+0x28,'<Q',world+0x1000+i*0x100)

def run(snapshot=None,press=False,unfreeze=False,debug=False,show=False,developer=False,context=False,
        picking=0,selected=0,hit=(0,0,0),cursor_hit=True,cursor_position=(7.,99.,8.),delta=(1.,0.,0.)):
    global heap,key,widget_index
    heap=H;key=press;widget_index=0;log.clear();c.mem_write(D,bytes(0x40000));c.mem_write(S,bytes(0x20000))
    c.mem_write(0x143634cd8,bytes([developer]))
    qsheep,qcontrol=D+0x100,D+0x140;control=D+0x1a000
    transforms,sheep,goals=D+0x16000,D+0x17000,D+0x18000
    dense(qsheep,D+0x1000,D+0x2000,[transforms,sheep,goals],[48,104,16],3)
    dense(qcontrol,D+0x5000,D+0x6000,[control],[80],1)
    put(control,'<Q',1<<63);put(control+0x18,'<Q',1<<63);put(control+0x30,'<QQQB',123,456,selected,picking)
    for i in range(3):
        put(transforms+i*48,'<12f',0,0,0,1,10+i,20+i,30+i,0,1,1,1,0)
        put(sheep+i*104+0x20,'<Q',i);put(sheep+i*104+0x30,'<3I',0,0x40000000+i,0)
        put(goals+i*16,'<I3f',1,40+i,50+i,60+i)
    if snapshot is not None:
        p=alloc(len(snapshot)*40)
        for i,row in enumerate(snapshot):c.mem_write(p+i*40,bytes.fromhex(row))
        put(control,'<3Q',len(snapshot),p,len(snapshot))
    cursor,ui,egui,gui,commands,keyboard,reader=[D+0x200+i*0x40 for i in range(7)]
    for ref,ptr in [(cursor,D+0x1b000),(ui,D+0x1b400),(gui,D+0x1b800),(keyboard,D+0x1c000)]:put(ref,'<Q',ptr)
    put(D+0x1b000,'<I7f',int(cursor_hit),*cursor_position,*delta,0.)
    put(D+0x1b400+0x18,'<BB',hit[0],hit[1]);put(D+0x1b400+0x38,'<Q',hit[2])
    if context:put(egui,'<Q',1)
    put(gui+0x10,'<Q',D+0x1d000);put(gui+0x1c,'<I',12345)
    c.mem_write(D+0x1b800+0xf0,bytes([debug]));c.mem_write(D+0x1b800+0xf5,bytes([show]))
    queue=D+0x1e000;put(commands+8,'<Q',queue);put(queue,'<3Q',4096,D+0x20000,0)
    put(reader,'<QQ',D+0x1d100,D+0x1d200)
    if unfreeze:put(D+0x1d200+0x10,'<Q',1);put(D+0x1d200+0x40,'<Q',1)
    args=[D+0x1f000,qsheep,qcontrol,cursor,ui,egui,gui,commands,keyboard,reader]
    sp=S+0x1f008;put(sp,'<Q',STOP)
    for i,arg in enumerate(args[4:]):put(sp+0x28+i*8,'<Q',arg)
    for reg in [UC_X86_REG_RAX,UC_X86_REG_RBX,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_RSI,UC_X86_REG_RDI,UC_X86_REG_RBP,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,UC_X86_REG_R13,UC_X86_REG_R14,UC_X86_REG_R15]:c.reg_write(reg,0)
    for reg,val in zip([UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_R9],args):c.reg_write(reg,val)
    c.reg_write(UC_X86_REG_RSP,sp);c.reg_write(UC_X86_REG_MXCSR,0x1f80)
    try:c.emu_start(0x1416f11a0,STOP,count=1000000)
    except UcError as e:raise RuntimeError((str(e),hex(c.reg_read(UC_X86_REG_RIP))))
    assert c.reg_read(UC_X86_REG_RIP)==STOP,hex(c.reg_read(UC_X86_REG_RIP))
    snap=None if u64(control)==1<<63 else [bytes(c.mem_read(u64(control+8)+i*40,40)).hex() for i in range(u64(control+16))]
    return {'snapshot':snap,'selected':u64(control+0x40),'picking':c.mem_read(control+0x48,1)[0],'transforms':[list(struct.unpack('<3f',c.mem_read(transforms+i*48+16,12))) for i in range(3)],'movement':[bytes(c.mem_read(sheep+i*104+48,12)).hex() for i in range(3)],'goals':[bytes(c.mem_read(goals+i*16,16)).hex() for i in range(3)],'commands_bytes':u64(queue+16),'calls':list(log)}

if __name__=='__main__':
    rows=[]
    for i in range(3):rows.append(struct.pack('<I3fBB2xff3f',i%2,70+i,80+i,90+i,1,1,2+i,3+i,1+i,2+i,3+i).hex())
    cases=[]
    for saved in [None,[],rows[:1],rows]:
        for developer in [False,True]:
            for press in [False,True]:
                for unfreeze in [False,True]:
                    value=run(saved,press,unfreeze,developer=developer)
                    cases.append({'snapshot':saved,'developer':developer,'press':press,'unfreeze':unfreeze,'result':value})
    (OUT/'native-runtime-fixtures.json').write_text(json.dumps(cases,indent=2)+'\n')
    picking=[]
    for mode in [0,1,2]:
        for selected in [0,2,99]:
            for frozen in [False,True]:
                for cursor_hit in [False,True]:
                    args={'picking':mode,'selected':selected,'snapshot':rows if frozen else None,'cursor_hit':cursor_hit,'context':True,'debug':True,'show':True,'hit':(2,1,999)}
                    picking.append({'inputs':args,'result':run(**args)})
    for hit in [(2,4,100),(2,1,101),(2,4,999),(1,4,100)]:
        for delta in [(1.,0.,0.),(.01,0.,0.),(float('nan'),0.,0.)]:
            args={'context':True,'debug':True,'show':True,'hit':hit,'delta':delta}
            result=run(**args)
            args['delta_bits']=[struct.unpack('<I',struct.pack('<f',x))[0] for x in args.pop('delta')]
            picking.append({'inputs':args,'result':result})
    (OUT/'native-picking-fixtures.json').write_text(json.dumps(picking,indent=2)+'\n')
    print(json.dumps({'original_runtime_cases':len(cases),'real_snapshot_apply_capture':True,'gui_executed':False}))
