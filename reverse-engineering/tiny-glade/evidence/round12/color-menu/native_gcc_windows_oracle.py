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
    def mov_abs_jump(address):return b'\x48\xb8'+struct.pack('<Q',address)+b'\xff\xe0'
    fixture={'decorator':None,'derived_kind':0,'extra':None}
    @C.CFUNCTYPE(C.c_void_p,C.c_void_p,C.c_uint64)
    def find_decorator(storage,ident):return fixture['decorator']
    @C.CFUNCTYPE(C.c_uint32,C.c_void_p,C.c_uint64)
    def derived_kind(resource,ident):return fixture['derived_kind']
    @C.CFUNCTYPE(C.c_void_p,C.c_void_p,C.c_void_p)
    def extra_info(data,location):return fixture['extra']
    @C.CFUNCTYPE(None,C.c_void_p,C.c_uint64,C.c_uint64)
    def deallocate(ptr,size,alignment):pass # host allocation owners stay live until subprocess exit
    @C.CFUNCTYPE(C.c_void_p,C.c_void_p,C.c_uint64,C.c_uint64,C.c_uint64)
    def reallocate(ptr,size,alignment,new_size):
        new_ptr=allocate(new_size,alignment);C.memmove(new_ptr,ptr,min(size,new_size));return new_ptr
    @C.CFUNCTYPE(C.c_void_p,C.c_void_p,C.c_void_p,C.c_uint64)
    def memory_move(dst,src,size):C.memmove(dst,src,size);return dst
    for address,fn_hook in [(0x140a1c930,find_decorator),(0x140b26970,derived_kind),(0x1408392d0,extra_info),(0x140839340,extra_info),
          (0x140613c20,deallocate),(0x140613c30,reallocate),(0x142923c00,memory_move),(0x142923c10,memory_move)]:
        overwrite(address,mov_abs_jump(C.cast(fn_hook,C.c_void_p).value))
    overwrite(0x140b26a70,bytes.fromhex('31 c0 31 d2 c3')) # returned derived resource view is unused by fixture lookup
    fn=C.CFUNCTYPE(None,C.c_void_p,C.c_void_p)(0x14085ee90)
    rows=[]
    for value in [None,0,8,4294967295]:
        out=C.create_string_buffer(24);gcc=C.create_string_buffer(0x48)
        C.memmove(gcc,struct.pack('<II',int(value is not None),value or 0),8)
        fn(out,gcc)
        cap,ptr,count=struct.unpack('<QQQ',out.raw)
        assert count<4096 and cap>=count
        text=read(ptr,count).decode('utf8');parsed=json.loads(text)
        assert parsed['windows']==value
        rows.append({'native_windows_tag':int(value is not None),'native_windows_payload':value,'json':text,'windows_wire':parsed['windows']})
    @C.CFUNCTYPE(C.c_int,C.c_void_p,C.c_void_p,C.c_uint64)
    def memory_compare(left,right,size):
        a=read(left,size);b=read(right,size);return (a>b)-(a<b)
    overwrite(0x142923bf0,mov_abs_jump(C.cast(memory_compare,C.c_void_p).value))
    visit=C.CFUNCTYPE(None,C.c_void_p,C.c_void_p,C.c_uint64)(0x14085f550)
    visitor_rows=[]
    for field in [b'windows',b'window_light',b'gates',b'stairs',b'unknown']:
        name=C.create_string_buffer(field);out=C.create_string_buffer(16)
        visit(out,name,len(field))
        visitor_rows.append({'field':field.decode(),'native_result_tag':out.raw[0],'native_field_index':out.raw[1]})
    assert visitor_rows[0]['native_result_tag']==0 and visitor_rows[0]['native_field_index']==10
    result={'original_sha256':hashlib.sha256(raw).hexdigest(),'address':'0x14085ee90',
       'wooden_helper_sha256':hashlib.sha256(original).hexdigest(),'original_function_sha256':hashlib.sha256(read(0x14085ee90,880)).hexdigest(),'method':'execute original complete GCC to_json_string at14085ee90 with native 0x48B GCC memory; only host alloc/realloc/free/memmove shims; no serializer or game-logic stubs',
       'allocation_count':len(allocations),'visitor_address':'0x14085f550','visitor_rows':visitor_rows,'rows':rows}
    (ROOT/'gcc-windows-original-execution.json').write_text(json.dumps(result,indent=2),encoding='utf8')
    print(json.dumps({'output':'gcc-windows-original-execution.json','rows':[(r['native_windows_tag'],r['native_windows_payload'],r['windows_wire'])for r in rows]}))
finally:k.VirtualFree(base,0,0x8000)
