"""Execute the original isolated wooden-color producer leaf, never the PE entry point.
Only the allocator is replaced with a host-owned allocator. All game instructions,
demo branches, strings, IconInfo payload writes and the returned Vec stay original.
Run in its own subprocess so native faults cannot affect the coordinator process.
"""
import ctypes as C
import hashlib,json,struct
from pathlib import Path
ROOT=Path(__file__).resolve().parent
EXE=ROOT.parents[2]/'workspace/input/tiny-glade.exe'
# ROOT=evidence/round12/color-menu; project root is parents[2].
raw=EXE.read_bytes()
assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=struct.unpack_from('<I',raw,0x3c)[0]
assert raw[pe:pe+4]==b'PE\0\0'
opt=pe+24
assert struct.unpack_from('<H',raw,opt)[0]==0x20b
base=struct.unpack_from('<Q',raw,opt+24)[0]
image_size=struct.unpack_from('<I',raw,opt+56)[0]
headers_size=struct.unpack_from('<I',raw,opt+60)[0]
sections_count=struct.unpack_from('<H',raw,pe+6)[0]
sec=opt+struct.unpack_from('<H',raw,pe+20)[0]
k=C.WinDLL('kernel32',use_last_error=True)
k.VirtualAlloc.argtypes=[C.c_void_p,C.c_size_t,C.c_uint32,C.c_uint32];k.VirtualAlloc.restype=C.c_void_p
k.VirtualFree.argtypes=[C.c_void_p,C.c_size_t,C.c_uint32];k.VirtualFree.restype=C.c_int
actual=k.VirtualAlloc(base,image_size,0x3000,0x40)
if actual!=base:raise OSError('Original image base unavailable; refusing relocated execution',actual,base)
allocations=[]
@C.CFUNCTYPE(C.c_void_p,C.c_uint64,C.c_uint64)
def allocate(size,alignment):
    if size>0x1000000 or alignment>0x10000:raise ValueError('Unexpected allocation',size,alignment)
    buf=C.create_string_buffer(max(1,size)+max(1,alignment));allocations.append(buf)
    return (C.addressof(buf)+alignment-1)&-alignment

def read(addr,n):return C.string_at(addr,n)
def u64(addr):return struct.unpack('<Q',read(addr,8))[0]
def overwrite(addr,data):
    if not base<=addr<base+image_size or addr+len(data)>base+image_size:raise ValueError('Write out of mapped original image')
    C.memmove(addr,data,len(data))
try:
    C.memmove(base,raw[:headers_size],headers_size)
    for i in range(sections_count):
        size,va,raw_size,ptr=struct.unpack_from('<IIII',raw,sec+i*40+8)
        if raw_size:C.memmove(base+va,raw[ptr:ptr+raw_size],raw_size)
    original=read(0x1422acce0,4107)
    assert hashlib.sha256(original).hexdigest()=='5b5b89abc9857408c9bd77c750271d90a031b09014255352381a95825e0ea562'
    overwrite(0x140613c10,b'\x48\xb8'+struct.pack('<Q',C.cast(allocate,C.c_void_p).value)+b'\xff\xe0')
    mode_addr=u64(0x14362c098)
    assert base<=mode_addr<base+image_size
    fn=C.CFUNCTYPE(None,C.c_void_p)(0x1422acce0)
    rows=[]
    for demo in [False,True]:
        overwrite(mode_addr,bytes([int(demo)]))
        out=C.create_string_buffer(24)
        fn(out)
        cap,ptr,count=struct.unpack('<QQQ',out.raw)
        assert cap==count==10
        values=[]
        for i in range(count):
            record=read(ptr+i*40,40)
            name_cap,name_ptr,name_len=struct.unpack_from('<QQQ',record)
            assert name_cap==name_len and name_len<256
            values.append({'name':read(name_ptr,name_len).decode('utf8'),'kind':record[24],
                'b19':record[25],'u1a':struct.unpack_from('<H',record,26)[0],
                'v1c':struct.unpack_from('<I',record,28)[0],'v20':struct.unpack_from('<I',record,32)[0],
                'v24':struct.unpack_from('<I',record,36)[0],'raw_payload':record[24:].hex()})
        rows.append({'demo':demo,'count':count,'icons':values})
    result={'original_sha256':hashlib.sha256(raw).hexdigest(),'address':'0x1422acce0',
       'original_function_sha256':hashlib.sha256(original).hexdigest(),'method':'direct original x64 instructions in isolated process; allocator hook only; no PE entry point',
       'allocation_count':len(allocations),'rows':rows}
    (ROOT/'wooden-palette-original-execution.json').write_text(json.dumps(result,indent=2),encoding='utf8')
    print(json.dumps(result,indent=2))
finally:k.VirtualFree(base,0,0x8000)
