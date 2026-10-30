"""原完整roof UV业务控制流；仅隔离外部ColliderSet/Shape服务，不桥接任何浮点运算。"""
from pathlib import Path
import struct, hashlib, json
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[3]
raw=(root/'B版本内容/tiny-glade.rne').read_bytes(); sha=hashlib.sha256(raw).hexdigest()
assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True); image=pe.get_memory_mapped_image();base=pe.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image)
D=0x20000000;u.mem_map(D,0x100000)
P,W,E,C,M,V,STOP,SHAPE=D+0x1000,D+0x10000,D+0x18000,D+0x19000,D+0x20000,D+0x21000,D+0xff000,D+0xfe000
state={};rows=[]
def q(addr,v):u.mem_write(addr,struct.pack('<Q',v))
def f(addr,v):u.mem_write(addr,struct.pack('<'+'f'*len(v),*v))
def ret():
    sp=u.reg_read(UC_X86_REG_RSP);u.reg_write(UC_X86_REG_RIP,struct.unpack('<Q',u.mem_read(sp,8))[0]);u.reg_write(UC_X86_REG_RSP,sp+8)
def hook(uc,addr,size,user):
    if addr==0x140b93940:
        state['physical_lookup']=True;u.reg_write(UC_X86_REG_RAX,0 if state['case']=='physical_missing' else C);ret()
    elif addr==SHAPE:
        state['cast']=list(struct.unpack('<6I',u.mem_read(u.reg_read(UC_X86_REG_R9),24)))
        sp=u.reg_read(UC_X86_REG_RSP);state['max_toi']=struct.unpack('<I',u.mem_read(sp+0x28,4))[0];state['solid']=u.mem_read(sp+0x30,1)[0]
        out=u.reg_read(UC_X86_REG_RCX)
        u.mem_write(out,struct.pack('<II4f',4 if state['case']=='shape_miss' else 2,0,65533.,0.,1.,0.));ret()
    elif addr in (0x1428d8725,0x1428d9330):
        state['panic']='receipt' if addr==0x1428d8725 else 'physical';u.emu_stop()
u.hook_add(UC_HOOK_CODE,hook)
def native_hash(hasher,key):
    q(D+0x90000,key);q(D+0xf0008,STOP)
    for r,v in [(UC_X86_REG_RSP,D+0xf0008),(UC_X86_REG_RCX,hasher),(UC_X86_REG_RDX,D+0x90000)]:u.reg_write(r,v)
    u.emu_start(0x1405b0d70,STOP,count=4096);assert u.reg_read(UC_X86_REG_RIP)==STOP
    return u.reg_read(UC_X86_REG_RAX)
for entry in (0x1408e4090,0x1408e4040):
    for case in ['missing_key','generation_zero','empty_entity_map','missing_entity','empty_slot','physical_missing','stale_receipt','shape_miss','no_uv','even_receipt','success']:
        u.mem_write(P,bytes(0x100));u.mem_write(W,bytes(0x4000));u.mem_write(E,bytes(0x100));u.mem_write(C,bytes(0x100));u.mem_write(M,bytes(0x100));u.mem_write(V,bytes(0x100))
        key,entity=7,0x100000003
        if case=='generation_zero':entity=3
        h1=native_hash(P+0x50,key);h2=native_hash(W+0x3e18,entity)
        # 一槽合法hashbrown表：控制字节使用原native SipHash结果，probe/compare全由原代码执行。
        ctrl1,ctrl2=D+0x3000,D+0x17000
        u.mem_write(ctrl1,bytes([h1>>57])+b'\xff'*15);q(ctrl1-16,key);q(ctrl1-8,entity)
        q(P+0x30,ctrl1);q(P+0x38,0);q(P+0x48,int(case!='missing_key'))
        u.mem_write(ctrl2,bytes([h2>>57])+b'\xff'*15);q(ctrl2-32,entity+(case=='missing_entity'))
        u.mem_write(ctrl2-24,struct.pack('<4IB',1,0,0,0,17 if case=='empty_slot' else 1))
        q(W+0x3df8,ctrl2);q(W+0x3e00,0);q(W+0x3e10,int(case!='empty_entity_map'))
        q(W+8,E);q(W+0x10,1);u.mem_write(E+0x20,struct.pack('<I',3 if case=='stale_receipt' else 1));q(E+0x18,M)
        q(C,0 if case=='even_receipt' else 0x100000000);f(C+0x34,[0,0,0,1,0,0,0])
        # 三角面在y=3，UV完全由真实native converter+barycentric计算。
        q(M+0x10,D+0x22000);q(M+0x18,3);q(M+0x20,D+0x22100);q(M+0x28,1)
        q(M+0x30,D+0x24000);q(M+0x38,V);q(V+0x10,16);q(V+0x38,SHAPE)
        q(M+0x50,0 if case=='no_uv' else D+0x22200);q(M+0x58,3)
        f(D+0x22000,[0,3,0,4,3,0,0,3,4]);u.mem_write(D+0x22100,struct.pack('<3I',0,1,2));f(D+0x22200,[0,0,1,0,0,1])
        state.clear();state['case']=case;state['entry']=hex(entry);q(D+0xf0008,STOP);q(D+0xf0030,W);q(D+0xf0038,P)
        out=D+0x25000;u.mem_write(out,b'\xcd'*32)
        for r,v in [(UC_X86_REG_RSP,D+0xf0008),(UC_X86_REG_RCX,out),(UC_X86_REG_RDX,key),(UC_X86_REG_XMM2,0x3f800000),(UC_X86_REG_XMM3,0x40000000)]:u.reg_write(r,v)
        u.emu_start(entry,STOP,count=20000)
        if 'panic' not in state:assert u.reg_read(UC_X86_REG_RIP)==STOP
        state['output']=bytes(u.mem_read(out,24)).hex(); rows.append(dict(state))
        expect={'missing_key':0,'generation_zero':0,'empty_entity_map':1,'missing_entity':1,'empty_slot':1,'shape_miss':3,'no_uv':4}
        if case in expect:assert u.mem_read(out,2)==bytes([1,expect[case]]),state
        elif case in ('success','even_receipt'):assert u.mem_read(out,1)==b'\0',state
        else:assert state.get('panic')==('physical' if case=='physical_missing' else 'receipt'),state
        if 'cast' in state:
            assert state['cast']==[0x3f800000,0x47800000,0x40000000,0x80000000,0xbf800000,0x80000000],state
            assert state['max_toi']==0x7f7fffff and state['solid']==1,state

meta=dict(binary_sha256=sha,entries=['0x1408e4090','0x1408e4040'],arithmetic_bridges=[],service_bridges=['ColliderSet::get_mut_internal140b93940 returns chosen fixture collider','Shape virtual cast returns chosen fixture intersection; no arithmetic'],cases=rows)
Path(__file__).with_suffix('.json').write_text(json.dumps(meta,indent=2)+'\n');print(json.dumps(dict(cases=len(rows),status='passed',arithmetic_bridges=[])))
