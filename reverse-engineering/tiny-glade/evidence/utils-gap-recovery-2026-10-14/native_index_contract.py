"""Original B insertion inline ranges and complete removal with real std code.

Caller frames / raw HashMap storage are supplied. No game outputs are hooked.
Allocation/memory bridges are inherited from the existing B spline oracle.
"""
from pathlib import Path
import hashlib, json, random, struct, sys
sys.path.insert(0,'D:/game/tools/reverse-runtime/utils-gap-20261014/python-deps')
OUT=Path(__file__).parent
sys.path.insert(0,str(OUT.parent/'shared-foundation-2026-10-13/utils-base'))
import native_spline_rdp as o
from unicorn import UC_HOOK_CODE
from unicorn.x86_const import *

state=o.OUTPUT; vector=o.INPUT; ctrl=o.HEAP+0x1000; mask=127
def hash_key(key):
    o.reset(); o.u.mem_write(o.INPUT,bytes(16))
    o.u.mem_write(o.INPUT+0x100,struct.pack('<I',key))
    o.u.reg_write(UC_X86_REG_RCX,o.INPUT);o.u.reg_write(UC_X86_REG_RDX,o.INPUT+0x100)
    o.run(0x1405b0ed0)
    return o.u.reg_read(UC_X86_REG_RAX)

def setup(keys):
    hashes=[hash_key(k) for k in keys]
    o.reset()
    o.u.mem_write(vector,struct.pack('<'+'I'*len(keys),*keys))
    o.u.mem_write(state,struct.pack('<9Q',64,vector,len(keys),ctrl,mask,112-len(keys),len(keys),0,0))
    controls=bytearray([0xff]*144);used=set()
    for i,(k,h) in enumerate(zip(keys,hashes)):
        bucket=h&mask
        while bucket in used: bucket=(bucket+1)&mask
        used.add(bucket);controls[bucket]=h>>57
        o.u.mem_write(ctrl-(bucket+1)*8,struct.pack('<II',k,i))
    controls[128:]=controls[:16];o.u.mem_write(ctrl,bytes(controls))

def snapshot():
    n=struct.unpack('<Q',o.u.mem_read(state+16,8))[0]
    keys=list(struct.unpack('<'+'I'*n,o.u.mem_read(vector,4*n))) if n else []
    mapping={}
    controls=bytes(o.u.mem_read(ctrl,128))
    for i,c in enumerate(controls):
        if c<128:
            k,v=struct.unpack('<II',o.u.mem_read(ctrl-(i+1)*8,8));mapping[str(k)]=v
    return {'keys':keys,'mapping':mapping,'map_len':struct.unpack('<Q',o.u.mem_read(state+48,8))[0]}

stop_at=set();stopped=None
def boundary(uc,address,size,data):
    global stopped
    if address in stop_at: stopped=address;uc.emu_stop()
o.u.hook_add(UC_HOOK_CODE,boundary)

def insert(keys,key,host):
    global stop_at,stopped
    setup(keys);before=snapshot()
    frame=o.STACK+0x10000
    o.u.reg_write(UC_X86_REG_RBP,frame);o.u.reg_write(UC_X86_REG_RSP,frame-0x200)
    o.u.reg_write(UC_X86_REG_XMM6,(1<<128)-1)
    if host=='extend':
        entry,duplicate,success=0x140992350,0x14099233b,0x14099271d
        o.u.reg_write(UC_X86_REG_RBX,state);o.u.reg_write(UC_X86_REG_R13,key)
        o.u.reg_write(UC_X86_REG_RDI,frame-0x20)
        o.u.mem_write(frame-0x20,struct.pack('<I',key))
        o.u.mem_write(frame-0x38,struct.pack('<Q',state+0x38))
        o.u.mem_write(frame-0x28,struct.pack('<Q',state+0x18))
        index_register=UC_X86_REG_R15
    else:
        entry,duplicate,success=0x14156d294,0x14156d280,0x14156d5f6
        o.u.reg_write(UC_X86_REG_R14,state);o.u.reg_write(UC_X86_REG_RDI,key)
        o.u.mem_write(frame,struct.pack('<I',key))
        o.u.mem_write(frame-0x40,struct.pack('<Q',state+0x38))
        o.u.mem_write(frame-0x28,struct.pack('<Q',state+0x18))
        index_register=UC_X86_REG_RBX
    stop_at={duplicate,success};stopped=None
    try: o.u.emu_start(entry,o.STOP,count=500000)
    finally: stop_at=set()
    assert stopped in (duplicate,success),hex(o.u.reg_read(UC_X86_REG_RIP))
    result=None if stopped==duplicate else o.u.reg_read(index_register)&0xffffffff
    after=snapshot()
    expected=keys if key in keys else keys+[key]
    assert after['keys']==expected,(host,key,after)
    assert after['mapping']=={str(k):i for i,k in enumerate(expected)},after
    return {'host':host,'keys':keys,'key':key,'index':result,'before':before,'after':after,'stop_pc':hex(stopped)}

def remove(keys,key,cached):
    setup(keys)
    handle=o.INPUT+0x1000
    o.u.mem_write(handle,struct.pack('<II',key,cached))
    o.u.reg_write(UC_X86_REG_RCX,state);o.u.reg_write(UC_X86_REG_RDX,handle)
    o.run(0x140a19f50)
    result=o.u.reg_read(UC_X86_REG_RDX) if o.u.reg_read(UC_X86_REG_RAX) else None
    updated=struct.unpack('<I',o.u.mem_read(handle+4,4))[0]
    return {'keys':keys,'key':key,'cached':cached,'index':result,'updated_cached':updated,'after':snapshot()}

rng=random.Random(141014)
inserts=[];removes=[]
for n in [0,1,2,3,16,32]:
    keys=rng.sample(range(1,100000),n)
    probes=[0,0xffffffff]+keys[:4]
    for host in ['extend','create']:
        for key in probes: inserts.append(insert(keys,key,host))
    for key in probes:
        for hint in [0,n//2,n+10]: removes.append(remove(keys,key,hint))
report={'original_sha256':hashlib.sha256(o.raw).hexdigest(),'insert':inserts,'remove':removes,'scope':'Resolved B insertion and full removal; real std/hashbrown hashing/probing/insert/remove execute. Preallocated capacity avoids growth. Duplicate stops at original caller retry before next_id increments. No original full producer/GPU runtime claim.'}
(OUT/'index-native-fixtures.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf8')
print('insert',len(inserts),'remove',len(removes))
