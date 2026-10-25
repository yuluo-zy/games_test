from pathlib import Path
import importlib.util
import hashlib
import json
import itertools
import subprocess
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import *

OUT=Path(__file__).resolve().parent
ROOT=Path('D:/game/reverse-engineering/tiny-glade')
spec=importlib.util.spec_from_file_location("barrier_base",OUT/"pe_fixture.py")
b=importlib.util.module_from_spec(spec);spec.loader.exec_module(b)
# Reuse identity and fixture writer, but deliberately install no base hooks.
# In particular the earlier barrier oracle skipped private debug helpers.
CPU=Uc(UC_ARCH_X86,UC_MODE_64)
CPU.mem_map(b.BASE,(b.SIZE+4095)&~4095)
for section in range(b.struct.unpack_from("<H",b.RAW,b.PE+6)[0]):
    header=b.OPTIONAL+b.struct.unpack_from("<H",b.RAW,b.PE+20)[0]+section*40
    _,va,size,offset=b.struct.unpack_from("<IIII",b.RAW,header+8)
    CPU.mem_write(b.BASE+va,b.RAW[offset:offset+size])
CPU.mem_map(b.STACK,0x20000)
CPU.mem_map(b.STOP,4096)
CPU.mem_map(b.MEMORY,0x20000)
b.CPU=CPU
EXTRA=0x60000000
CPU.mem_map(EXTRA,0x1000000)
PAYLOAD=EXTRA;COMMANDS=EXTRA+0x1000;QUEUES=EXTRA+0x3000;RECEIPT=EXTRA+0x5000;ALLOC=EXTRA+0x10000
VK_SUBMIT=b.STOP+0x100;VK_COUNTER=b.STOP+0x200;END_FLUSH=b.STOP+0x300;BARRIER_FLUSH=b.STOP+0x400
STATE={}

def alloc(length):
    pointer=STATE["alloc"];STATE["alloc"]+=((length+15)&~15)+16
    assert STATE["alloc"]<EXTRA+0x1000000
    CPU.mem_write(pointer,bytes(length));return pointer

def vector(pointer,count,width):
    return list(b.read(pointer,"<"+({4:"I",8:"Q"}[width])*count)) if count else []

def hook(cpu,address,size,user):
    STATE["visited"].add(hex(address))
    STATE["last"].append(hex(address));STATE["last"]=STATE["last"][-20:]
    if address in (0x1428D9760,0x1428CA88C,0x1428D95D4,0x1428D9430,0x1428D9310,0x1428D9390,0x1428D9518,0x1428D8FE3):
        STATE["panic"]=hex(address);cpu.emu_stop()
    elif address==0x140613C10:
        b.ret(alloc(cpu.reg_read(UC_X86_REG_RCX)))
    elif address==0x140613C30:
        old=cpu.reg_read(UC_X86_REG_RCX);oldsize=cpu.reg_read(UC_X86_REG_RDX);length=cpu.reg_read(UC_X86_REG_R9)
        pointer=alloc(length);CPU.mem_write(pointer,bytes(CPU.mem_read(old,min(oldsize,length))));b.ret(pointer)
    elif address==0x140613C20:
        b.ret()
    elif address in (0x142923C00,0x142923C10):
        dest=cpu.reg_read(UC_X86_REG_RCX);source=cpu.reg_read(UC_X86_REG_RDX);length=cpu.reg_read(UC_X86_REG_R8)
        CPU.mem_write(dest,bytes(CPU.mem_read(source,length)));b.ret(dest)
    elif address in (0x140C81C70,0x140C81E10,0x1418C1380,0x140C84010):
        b.ret()
    elif address==0x140C16A00:
        b.ret(0)
    elif address==VK_SUBMIT:
        count=cpu.reg_read(UC_X86_REG_RDX)&0xffffffff;pointer=cpu.reg_read(UC_X86_REG_R8)
        captures=[]
        for i in range(count):
            p=pointer+i*72
            stype,_,nextptr,wcount=b.read(p,"<IIQI")
            wp,sp=b.read(p+24,"<QQ");ccount=b.read(p+40,"<I")[0];cp=b.read(p+48,"<Q")[0];scount=b.read(p+56,"<I")[0];sg=b.read(p+64,"<Q")[0]
            timeline=None
            if nextptr:
                nt=b.read(nextptr,"<I")[0];nw=b.read(nextptr+16,"<I")[0];nwp=b.read(nextptr+24,"<Q")[0];ns=b.read(nextptr+32,"<I")[0];nsp=b.read(nextptr+40,"<Q")[0]
                timeline={"stype":nt,"wait_values":vector(nwp,nw,8),"signal_values":vector(nsp,ns,8)}
            captures.append({"stype":stype,"wait_semaphores":vector(wp,wcount,8),"wait_stages":vector(sp,wcount,4),"commands":vector(cp,ccount,8),"signal_semaphores":vector(sg,scount,8),"timeline":timeline})
        STATE["backend"]={"queue":cpu.reg_read(UC_X86_REG_RCX),"fence":cpu.reg_read(UC_X86_REG_R9),"submits":captures}
        STATE["events"].append("backend");b.ret(STATE["error"])
    elif address==VK_COUNTER:
        b.write(cpu.reg_read(UC_X86_REG_R8),"<Q",0);b.ret(0)
    elif address in (END_FLUSH,BARRIER_FLUSH):
        if address==END_FLUSH:STATE["events"].append("flush-end")
        b.ret(0)
    elif address==0x140BF0860:
        b.write(cpu.reg_read(UC_X86_REG_RCX),"<Q",11 if STATE["converted"] else 9);STATE["events"].append("convert-error");b.ret()
    elif address==0x1418C3B80:
        STATE["events"].append("register-range")
    elif address==0x140C0630D:
        STATE["events"].append("post-submit")

CPU.hook_add(UC_HOOK_CODE,hook)

def batch(pointer,commands,count,wait=False,signal=False):
    for offset,width,items in [(0,8,[0xA100] if wait else []),(24,4,[0x400] if wait else []),(48,8,[0] if wait else []),(72,8,[0xE100] if signal else []),(96,8,[0] if signal else [])]:
        data=alloc(len(items)*width) if items else width
        if items:b.write(data,"<"+({4:"I",8:"Q"}[width])*len(items),*items)
        b.write(pointer+offset,"<QQQ",len(items),data,len(items))
    b.write(pointer+120,"<QQQ",count,commands,count)

def setup(count=2,kind=0,index=0,error=0,converted=0,wait=0,signal=0,prior=0,flush=0,bad=999,mismatch=0,start=10):
    b.setup([0,0,0,0,256,0,0,4,0,0,1,0]);b.STATE["cut"]=b.STOP
    CPU.mem_write(EXTRA,bytes(0x1000000));STATE.clear();STATE.update(alloc=ALLOC,visited=set(),last=[],panic=None,backend=None,events=[],error=error,converted=converted)
    b.write(b.DEVICE+0x88,"<Q",VK_SUBMIT)
    b.write(b.DEVICE+0x4F0,"<Q",VK_COUNTER)
    b.write(b.DEVICE+0x9F8,"<Q",4)
    b.write(b.INNER,"<Q",EXTRA+0x7000)
    b.write(EXTRA+0x7008,"<QQQ",0,8,0)
    b.write(b.DEVICE+0x7E8,"<QQQ",QUEUES,2,start)
    for qi,q in enumerate([b.DEVICE+0x730,QUEUES,QUEUES+0xb0]):
        b.write(q,"<QQQQ",32,alloc(32*8),0,0)
        b.write(q+0x20,"<QQQQ",32,alloc(32*96),0,0)
        b.write(q+0x78,"<QQQ",0xABCD+qi,0,0xF000+qi)
    b.write(PAYLOAD+0x28,"<QQQ",count,COMMANDS,count)
    b.write(PAYLOAD+0x40,"<Q",0x8000000000000000)
    b.write(PAYLOAD+0xD0,"<Q",b.DEVICE)
    if wait:b.write(PAYLOAD,"<QQI",1,0xA000,0x400)
    if signal:b.write(PAYLOAD+24,"<QQ",1,0xE000)
    if prior:
        batch(PAYLOAD+0x40,EXTRA+0x9000,prior,True,True)
        for i in range(prior):
            b.write(EXTRA+0x9000+i*88,"<QQII",2,80+i,kind,index)
            b.write(EXTRA+0x9000+i*88+0x48,"<Q",0xB000+i)
    if flush:
        b.write(b.DEVICE+0x9F8,"<QQII",1,70,0,0)
        b.write(b.DEVICE+0xA40,"<Q",0xB100)
        b.write(b.DEVICE+0x2C8,"<Q",END_FLUSH)
        b.write(b.DEVICE+0x3E0,"<Q",BARRIER_FLUSH)
    for i in range(count):
        b.write(COMMANDS+i*88,"<QQII",1 if i==bad else 2,100+i,kind ^ (i==0 and mismatch),index)
        b.write(COMMANDS+i*88+0x48,"<Q",0xC000+i)
    pointer=b.STACK+0x10000;b.write(pointer,"<Q",b.STOP);b.write(pointer+0x28,"<Q",PAYLOAD)
    CPU.reg_write(UC_X86_REG_RSP,pointer);CPU.reg_write(UC_X86_REG_RCX,RECEIPT);CPU.reg_write(UC_X86_REG_RDX,b.DEVICE);CPU.reg_write(UC_X86_REG_R8,kind);CPU.reg_write(UC_X86_REG_R9,index)

def execute(*case):
    setup(*case)
    try:CPU.emu_start(0x140C04E20,b.STOP,count=1000000)
    except Exception as e:
        raise RuntimeError(f"RIP={CPU.reg_read(UC_X86_REG_RIP):x} last={STATE['last']}") from e
    queues=[]
    for q in [b.DEVICE+0x730,QUEUES,QUEUES+0xb0]:
        cap,pointer,head,length=b.read(q,"<QQQQ");timelines=[b.read(pointer+((head+i)%cap)*8,"<Q")[0] for i in range(length)]
        cap,pointer,head,length=b.read(q+0x20,"<QQQQ");commands=[{"state":b.read(pointer+((head+i)%cap)*96,"<Q")[0],"sequence":b.read(pointer+((head+i)%cap)*96+8,"<Q")[0],"timeline":b.read(pointer+((head+i)%cap)*96+88,"<Q")[0]} for i in range(length)]
        queues.append({"pending_timeline":timelines,"pending_commands":commands})
    result={"panic":STATE["panic"],"backend":STATE["backend"],"events":STATE["events"],"next_timeline":b.read(b.DEVICE+0x7f8,"<Q")[0],"receipt":b.read(RECEIPT,"<QQ") if CPU.reg_read(UC_X86_REG_RIP)==b.STOP else None,"queues":queues,"visited":sorted(STATE["visited"]),"locks":{"outer":b.read(b.DEVICE+0x718,"<B")[0],"submit":b.read(b.DEVICE+0x800,"<Q")[0]},"oldest_pending":b.read(b.DEVICE+0x720,"<QQ")}
    result["native"]=format_result(result)
    return result

def nums(values):return ",".join(map(str,values))
def format_result(r):
    qs=";".join(nums(q["pending_timeline"])+"/"+",".join(f'{c["state"]}:{c["sequence"]}:{c["timeline"]}' for c in q["pending_commands"]) for q in r["queues"])
    backend="none"
    if r["backend"]:
        ss=[]
        for s in r["backend"]["submits"]:
            t=s["timeline"]
            ss.append("/".join([nums(s["wait_semaphores"]),nums(s["wait_stages"]),nums(s["commands"]),nums(s["signal_semaphores"]),nums(t["wait_values"]),nums(t["signal_values"])]))
        backend=str(r["backend"]["queue"])+":"+";".join(ss)
    receipt=nums(r["receipt"]) if r["receipt"] is not None else "none"
    oldest=str(r["oldest_pending"][1]) if r["oldest_pending"][0] else "none"
    return f'{int(r["panic"] is not None)}|{r["next_timeline"]}|{receipt}|{qs}|{backend}|{",".join(r["events"])}|{r["locks"]["outer"]}:{r["locks"]["submit"]}|{oldest}'

def cases():
    result=[]
    for count,(kind,index),wait,signal,prior,start in itertools.product([0,1,2,4,8],[(0,0),(1,0),(1,1)],[0,1],[0,1],[0,1,2],[1,10,0xffffffff]):
        result.append([count,kind,index,0,0,wait,signal,prior,0,999,0,start])
    for count,wait,signal,prior in itertools.product([0,1,2,4],[0,1],[0,1],[0,2]):
        result.append([count,0,0,0,0,wait,signal,prior,1,999,0,10])
    for count,(kind,index),error,converted,prior in itertools.product([0,1,4],[(0,0),(1,1)],[-4,1],[0,1],[0,2]):
        result.append([count,kind,index,error,converted,1,1,prior,0,999,0,10])
    for count,(kind,index),bad in itertools.product([1,2,4],[(0,0),(1,1)],[0,1,3]):
        if bad<count:result.append([count,kind,index,0,0,0,0,1,0,bad,0,10])
    for count,(kind,index) in itertools.product([0,1,2],[(0,0),(1,1)]):
        result.append([count,kind,index,0,0,0,0,0,0,999,1,10])
    result += [[2,1,3,0,0,0,0,0,0,999,0,10],[0,0,0,0,0,0,0,0,0,999,0,0]]
    return result
