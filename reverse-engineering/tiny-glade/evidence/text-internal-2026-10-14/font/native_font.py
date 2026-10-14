"""Original B CPU font policy entries; explicit allocator/CRT fixtures, no GPU/game."""
import ctypes as c, hashlib, json, struct as s
from pathlib import Path
OUT=Path(__file__).parent
raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=s.unpack_from('<I',raw,60)[0];opt=pe+24;sh=opt+s.unpack_from('<H',raw,pe+20)[0]
size=s.unpack_from('<I',raw,opt+56)[0]
k=c.WinDLL('kernel32',use_last_error=True)
k.VirtualAlloc.argtypes=[c.c_void_p,c.c_size_t,c.c_uint32,c.c_uint32];k.VirtualAlloc.restype=c.c_void_p
k.VirtualFree.argtypes=[c.c_void_p,c.c_size_t,c.c_uint32];k.VirtualFree.restype=c.c_int
base=k.VirtualAlloc(None,size,0x3000,0x40);assert base
for i in range(s.unpack_from('<H',raw,pe+6)[0]):
    _,rva,rs,off=s.unpack_from('<IIII',raw,sh+i*40+8);c.memmove(base+rva,raw[off:off+rs],rs)
crt=c.CDLL('msvcrt.dll')
# Rust layouts in this scope only ask <=8 alignment; msvcrt malloc supplies >=16.
def bridge(rva,proc):c.memmove(base+rva,b'\x48\xb8'+s.pack('<Q',c.cast(proc,c.c_void_p).value)+b'\xff\xe0',12)
bridge(0x613c10,crt.malloc);bridge(0x613c20,crt.free)
bridge(0x2923c00,crt.memcpy);bridge(0x2923c20,crt.memset);bridge(0x2923bf0,crt.memcmp)
def fn(rva,ret,*args):return c.WINFUNCTYPE(ret,*args)(base+rva)
hasher=fn(0x5b1790,c.c_uint64,c.c_void_p,c.c_void_p)
get=fn(0xb91fa0,c.c_void_p,c.c_void_p,c.c_uint8)
visitor=fn(0x1e5d390,c.c_void_p,c.c_void_p,c.c_void_p,c.c_uint64)
new=fn(0xfac1f0,c.c_void_p,c.c_void_p,c.c_float,c.c_uint8,c.c_void_p)
family=fn(0xb90c80,None,c.c_void_p,c.c_uint8)
metrics=fn(0xfac150,None,c.c_void_p,c.c_float)
take=fn(0xb91620,c.c_void_p,c.c_void_p,c.c_void_p)
role_cases=[]
for tag,name in enumerate(['Label','MenuButton','TextEditor','Hand']):
    result=c.create_string_buffer(0x100);text=c.create_string_buffer(name.encode())
    visitor(result,text,len(name));assert result.raw[0]==0x32 and result.raw[1]==tag
    role_cases.append({'name':name,'tag':result.raw[1],'result_discriminant':result.raw[0]})
# Fixture HashMap with original hash function, never an implementation of HashMap.
capacity=16;storage=c.create_string_buffer(capacity*40+32);ctrl=c.addressof(storage)+capacity*40
c.memset(ctrl,255,32)
arc=c.create_string_buffer(0x50);s.pack_into('<QQQQQQ',arc,0,1,1,ctrl,capacity-1,capacity-4,4)
names=[];entries={}
for tag,name,scale in [(0,'FamilyZero',1.),(1,'FamilyOne',2.),(2,'FamilyTwo',0.5),(3,'FamilyThree',3.)]:
    key=c.c_uint8(tag);h=hasher(c.addressof(arc)+0x30,c.byref(key));bucket=h&(capacity-1)
    while c.c_uint8.from_address(ctrl+bucket).value!=255:bucket=(bucket+1)&(capacity-1)
    for i in [bucket,bucket+capacity]:c.c_uint8.from_address(ctrl+i).value=h>>57
    namebuf=c.create_string_buffer(name.encode());names.append(namebuf)
    address=ctrl-(bucket+1)*40
    c.c_uint8.from_address(address).value=tag
    c.memmove(address+8,s.pack('<QQQf',len(name),c.addressof(namebuf),len(name),scale),28)
    assert get(c.addressof(arc)+0x10,tag)==address+8
    entries[tag]={'name':name,'scale':scale,'bucket':bucket}
assert not get(c.addressof(arc)+0x10,4)
def snapshot(obj):
    b=obj.raw
    cap,ptr,length=s.unpack_from('<QQQ',b,0x200)
    return {'role':b[0x255],'metrics_base':list(s.unpack_from('<ff',b,0x240)),
            'metrics_scaled':list(s.unpack_from('<ff',b,0x150)),
            'bounds':list(s.unpack_from('<ff',b,0x238)),
            'max_width_bits':s.unpack_from('<I',b,0x248)[0],'align_tag':b[0x256],
            'attrs_family':c.string_at(ptr,length).decode() if length else '',
            'attrs_weight':s.unpack_from('<H',b,0x22c)[0],
            'buffer_lines':s.unpack_from('<Q',b,0x48)[0],
            'pending_actions':s.unpack_from('<Q',b,0x1b0)[0],
            'arc_strong':s.unpack_from('<Q',arc,0)[0],
            'stored_arc_matches':s.unpack_from('<Q',b,0x230)[0]==c.addressof(arc)}
scenarios=[]
for role in range(4):
    obj=c.create_string_buffer(600)
    new(obj,20.,role,arc)
    snap=snapshot(obj)
    assert snap['role']==role and snap['metrics_scaled']==[20.*entries[role]['scale']]*2
    assert snap['attrs_weight']==400 and snap['buffer_lines']==0
    scenarios.append({'operation':'new<f32>','role':role,'state':snap})
obj=c.create_string_buffer(600);new(obj,20.,0,arc)
# A separately-owned fragment action vector is a fixture, not consumed here.
actions=c.create_string_buffer(32);s.pack_into('<I',actions,0,0x14)
s.pack_into('<QQQ',obj,0x1a0,1,c.addressof(actions),1)
family(obj,1);after=snapshot(obj)
assert after['role']==0 and after['attrs_family']=='FamilyOne' and after['metrics_scaled']==[40.,40.]
assert after['pending_actions']==1
scenarios.append({'operation':'set_family(1) on role0','state':after})
family(obj,0);after=snapshot(obj)
assert after['attrs_family']=='FamilyOne' and after['metrics_scaled']==[40.,40.] and after['role']==0
scenarios.append({'operation':'set_family(0) early return despite attrs1','state':after})
metrics(obj,25.);after=snapshot(obj)
assert after['role']==0 and after['attrs_family']=='FamilyOne' and after['metrics_base']==[25.,25.] and after['metrics_scaled']==[25.,25.]
scenarios.append({'operation':'set_metrics(25) looks up stored role0','state':after})
old_editor=c.create_string_buffer(416);old_bytes=obj.raw[:416]
take(old_editor,obj);after=snapshot(obj)
assert old_editor.raw==old_bytes
assert after['metrics_scaled']==[25.,25.] and after['attrs_family']=='FamilyOne'
assert after['role']==0 and after['pending_actions']==1 and after['stored_arc_matches']
scenarios.append({'operation':'take_editor preserves font policy/action queue and current scaled metrics','state':after})
report={'binary_sha256':sha,'original_instruction_execution':True,'game_launched':False,'gpu_executed':False,'original_hashmap_algorithm_reimplemented':False,'bridges':['explicit preallocated HashMap buckets populated by original hash function','CRT allocation/free/memcpy/memset/memcmp boundary jumps; scope uses <=8 alignment','dummy Arc header counts1 weak1; no drop-last'],'role_cases':role_cases,'entries':entries,'scenarios':scenarios,'passed':len(role_cases)+len(scenarios)+5,'scope':'role visitor, FontTable::get, TextFragment::new<f32>/set_family/set_metrics<f32> successful paths; no panic/unwind/std allocator equivalence'}
(OUT/'native-font.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps({'ok':True,'passed':report['passed'],'set_family_role_unchanged':True}))
k.VirtualFree(base,0,0x8000)
