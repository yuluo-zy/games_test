"""原Place数据转Info完整243B及原136B事件写段。容器边界仅抓取实际入参。"""
from pathlib import Path
import hashlib,json,random,struct,pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[3];raw=(root/'B版本内容/tiny-glade.rne').read_bytes()
sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);image=p.get_memory_mapped_image();base=p.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image)
D,S=0x20000000,0x30000000;u.mem_map(D,0x10000);u.mem_map(S,0x10000);ret=D+0xf000
captured={}
def finish_call():
    sp=u.reg_read(UC_X86_REG_RSP);pc=struct.unpack('<Q',u.mem_read(sp,8))[0]
    u.reg_write(UC_X86_REG_RSP,sp+8);u.reg_write(UC_X86_REG_RIP,pc)
def hook(uc,pc,size,data):
    if pc==0x140a1cf30:
        captured['storage']=[u.reg_read(UC_X86_REG_RDX),u.reg_read(UC_X86_REG_R8)]
        u.reg_write(UC_X86_REG_RAX,D+0x1000);u.reg_write(UC_X86_REG_RDX,D+0x2000);finish_call()
    elif pc==0x14155afe0:
        captured['id']=u.reg_read(UC_X86_REG_R8)&0xffffffff
        captured['info']=bytes(u.mem_read(u.reg_read(UC_X86_REG_R9),72));finish_call()
u.hook_add(UC_HOOK_CODE,hook)
rng=random.Random(0x141276e30);rows=[]
for index in range(512):
    tag=index%3;wall=rng.getrandbits(64);id=rng.getrandbits(32);color=rng.getrandbits(32)
    destination=rng.randbytes(40);extra=rng.randbytes(20);position=rng.randbytes(16)
    flag=index%2;ty=index%64;rank=(index//2)%5;state=(index//4)%2;demo=(index//8)%2
    data=struct.pack('<QQ',tag,wall)+destination+extra+struct.pack('<II3B',id,color,flag,ty,rank)+b'\0'
    u.mem_write(D,b'\0'*0x10000);u.mem_write(S,b'\xcc'*0x10000)
    u.mem_write(D+0x3000,data);u.mem_write(0x143634d90,bytes([demo]))
    sp=S+0xe008;u.mem_write(sp,struct.pack('<Q',ret));u.reg_write(UC_X86_REG_RSP,sp)
    u.reg_write(UC_X86_REG_RCX,D);u.reg_write(UC_X86_REG_RDX,D+0x3000);u.reg_write(UC_X86_REG_R8,state)
    captured.clear();u.emu_start(0x140a20c70,ret,count=10000)
    assert captured['storage']==[tag,wall] and captured['id']==id
    info=captured['info']
    # 原apply已将PlaceData拷入自身栈帧；事件写段直接执行，无结果桥接。
    sp=S+0xc000;u.mem_write(S,b'\xcc'*0x10000);u.mem_write(sp+0x40,data);u.mem_write(D+0x4000,position)
    u.reg_write(UC_X86_REG_RSP,sp);u.reg_write(UC_X86_REG_RSI,D+0x4000)
    u.reg_write(UC_X86_REG_R12,tag);u.reg_write(UC_X86_REG_R13,wall);u.reg_write(UC_X86_REG_RBP,id)
    u.reg_write(UC_X86_REG_RBX,state);u.reg_write(UC_X86_REG_RDI,D+0x5000)
    u.emu_start(0x141277260,0x1412772f4,count=10000)
    event=bytes(u.mem_read(sp+0x160,136))
    rows.append(' '.join([str(demo),str(state),data.hex(),position.hex(),info.hex(),event.hex()]))
out=Path(__file__).with_suffix('.txt');out.write_text('\n'.join(rows)+'\n')
Path(__file__).with_suffix('.json').write_text(json.dumps({'binary_sha256':sha,'cases':len(rows),'ranges':['140a20c70..140a20d4a complete successful path','141277260..1412772f4 event writes'],'bridges':['140a1cf30 container get_or_insert returns resolved bucket','14155afe0 container insert captures actual Info and map id; allocation/index internals excluded'],'inactive_event_bytes':'prefill cc; compare only original active ranges','fixture_sha256':hashlib.sha256(out.read_bytes()).hexdigest(),'boundary':'resolved conversion and event encoding; no ECS/archival/history/complete UI claim'},indent=2))
print(len(rows))
