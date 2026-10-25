"""Execute original grading's Halloween and saved-image lifecycle in isolation.

No entry point, disk-save/game/audio/GPU execution. The complete original
load_glade_grading and ImageFormationConfigs::default instructions run unchanged.
Only allocator/deallocator and CRT memory boundaries are hosted. No game hook.
Non-Halloween file I/O and JSON parsing are explicitly outside this oracle.
"""
import ctypes as C
import hashlib, json, struct
from pathlib import Path
OUT=Path(__file__).resolve().parent
ROOT=OUT.parents[4]
raw=(ROOT/'reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe').read_bytes()
assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=struct.unpack_from('<I',raw,0x3c)[0];opt=pe+24
base=struct.unpack_from('<Q',raw,opt+24)[0]
image_size=struct.unpack_from('<I',raw,opt+56)[0]
k=C.WinDLL('kernel32',use_last_error=True)
k.VirtualAlloc.argtypes=[C.c_void_p,C.c_size_t,C.c_uint32,C.c_uint32];k.VirtualAlloc.restype=C.c_void_p
k.VirtualFree.argtypes=[C.c_void_p,C.c_size_t,C.c_uint32];k.VirtualFree.restype=C.c_int
actual=k.VirtualAlloc(base,image_size,0x3000,0x40)
if actual!=base:raise OSError('Original image base unavailable',actual,base)
keep=[]
@C.CFUNCTYPE(C.c_void_p,C.c_uint64,C.c_uint64)
def allocate(size,alignment):
    assert size<0x1000000 and alignment<0x10000
    owner=C.create_string_buffer(max(1,size)+max(1,alignment));keep.append(owner)
    return (C.addressof(owner)+alignment-1)&-alignment
@C.CFUNCTYPE(None,C.c_void_p,C.c_uint64,C.c_uint64)
def deallocate(pointer,size,alignment):pass
@C.CFUNCTYPE(C.c_void_p,C.c_void_p,C.c_void_p,C.c_uint64)
def memory_move(dst,src,size):C.memmove(dst,src,size);return dst
def put(addr,data):C.memmove(addr,data,len(data))
def read(addr,n):return C.string_at(addr,n)
def string(text):
    data=text.encode();ptr=allocate(len(data),1);put(ptr,data)
    return struct.pack('<QQQ',len(data),ptr,len(data))
def decode(config):
    b=read(config,0x40);cap,ptr,n=struct.unpack_from('<QQQ',b)
    return {'name':read(ptr,n).decode(),'grading':list(struct.unpack_from('<4f',b,0x18)),
      'bypass':b[0x28],'lens_bits':list(struct.unpack_from('<3I',b,0x30)),
      'lighting_only':b[0x3c],'tree_see_thru':b[0x3d]}
try:
    put(base,raw[:struct.unpack_from('<I',raw,opt+60)[0]])
    sec=opt+struct.unpack_from('<H',raw,pe+20)[0]
    for i in range(struct.unpack_from('<H',raw,pe+6)[0]):
        size,rva,n,ptr=struct.unpack_from('<IIII',raw,sec+i*40+8)
        if n:put(base+rva,raw[ptr:ptr+n])
    expected={0x1408bf9c0:(2346,'2e66d6843d0b2e8baa1512df898e0359fb0ba619e8211fd3de24ad898331b7d3'),
      0x1408ae7f0:(607,'d6e1cf2fede1fc093e970ce4ade1fa90e6bc4d25db21d69e61c991ba5237b5a8')}
    for va,(size,sha) in expected.items():assert hashlib.sha256(read(va,size)).hexdigest()==sha
    for va,fn in [(0x140613c10,allocate),(0x140613c20,deallocate),(0x142923c00,memory_move),(0x142923c10,memory_move)]:
        put(va,b'\x48\xb8'+struct.pack('<Q',C.cast(fn,C.c_void_p).value)+b'\xff\xe0')
    default=C.CFUNCTYPE(None,C.c_void_p)(0x1408ae7f0)
    grading=C.CFUNCTYPE(None,C.c_void_p,C.c_void_p)(0x1408bf9c0)
    configs=C.create_string_buffer(0xc8);default(configs)
    defaults={'kind':configs.raw[0xc0],'slots':[decode(C.addressof(configs)+i*64) for i in range(3)]}
    tick=C.c_uint32(0);mut=C.create_string_buffer(32);meta=C.create_string_buffer(0x80);res=C.create_string_buffer(32)
    put(C.addressof(mut),struct.pack('<Q',C.addressof(configs)))
    put(C.addressof(mut)+0x10,struct.pack('<Q',C.addressof(tick)))
    put(C.addressof(mut)+0x1c,struct.pack('<I',73))
    put(C.addressof(res),struct.pack('<Q',C.addressof(meta)))
    tags=C.create_string_buffer(bytes([1,0,2]))
    put(C.addressof(meta),struct.pack('<QQQ',3,C.addressof(tags),3))
    saved=string('saved-full-config')+struct.pack('<4f',.25,-1.5,1.75,.875)+bytes([1])+bytes(7)+struct.pack('<3f',.125,.0625,-.25)+bytes([1,1])+bytes(2)
    assert len(saved)==64
    rows=[]
    for kind in range(3):
        for has_saved in [False,True,False]:
            put(C.addressof(configs)+0xc0,bytes([kind]))
            put(C.addressof(meta)+0x18,saved if has_saved else struct.pack('<Q',0x8000000000000000)+bytes(56))
            grading(mut,res)
            decoded=[decode(C.addressof(configs)+i*64) for i in range(3)]
            assert configs.raw[0xc0]==kind and tick.value==73
            for i,row in enumerate(decoded):
                if has_saved and i>0:
                    temp=C.create_string_buffer(saved)
                    assert row==decode(C.addressof(temp))
                else:
                    assert row['name']=='RoastMeat_Cinema_709_1'
                    assert row['grading']==list(struct.unpack('<4f',struct.pack('<4f',.75,0,1,1.06)))
                    assert row['bypass']==0 and row['lens_bits']==[0,0,0]
                    assert row['lighting_only']==0 and row['tree_see_thru']==0
            rows.append({'kind':kind,'saved_override':has_saved,'slots':decoded,'changed_tick':tick.value})
    (OUT/'grading-original-lifecycle-fixtures.json').write_text(json.dumps({
      'original_pe_sha256':hashlib.sha256(raw).hexdigest(),
      'scope':'Complete original Halloween mutator path and saved-image policy; file I/O/JSON parser branches are not exercised.',
      'hooks':['allocator140613c10','deallocator140613c20','memcpy142923c00','memmove142923c10'],
      'default':defaults,'fixtures':rows},indent=2),encoding='utf8')
    print('9 original grading lifecycle fixtures matched: 3 selected kinds x None -> Some -> None; saved full config and clearing verified')
finally:k.VirtualFree(base,0,0x8000)
