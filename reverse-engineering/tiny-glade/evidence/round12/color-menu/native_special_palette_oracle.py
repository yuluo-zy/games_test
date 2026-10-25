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
    def xmm_stack(op):
        result=b''
        for reg in range(6,16):
            result+=(b'\x44' if reg>=8 else b'')+bytes([0x0f,op,0x84|((reg&7)<<3),0x24])+struct.pack('<I',0x400+(reg-6)*16)
        return result
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
    prefix=bytes.fromhex('55 41 57 41 56 41 55 41 54 56 57 53 48 81 ec 08 05 00 00 48 8d ac 24 80 00 00 00')
    prefix+=xmm_stack(0x11)
    prefix+=bytes.fromhex('48 89 8d d0 03 00 00 49 89 d7 48 89 95 f0 01 00 00 4c 89 85 00 02 00 00 4c 89 8d 58 03 00 00')
    capture=bytes.fromhex('48 8b 85 d0 03 00 00 0f 10 85 10 01 00 00 0f 11 00 48 8b 95 20 01 00 00 48 89 50 10')
    capture+=xmm_stack(0x10)
    capture+=bytes.fromhex('48 81 c4 08 05 00 00 5b 5f 5e 41 5c 41 5d 41 5e 41 5f 5d c3')
    wrapper=k.VirtualAlloc(None,4096,0x3000,0x40)
    assert wrapper
    overwrite(0x14229e9e0,mov_abs_jump(wrapper+len(prefix)+12))
    fn=C.CFUNCTYPE(None,C.c_void_p,C.c_void_p,C.c_void_p,C.c_void_p)(wrapper)
    rows=[]
    try:
        for kind,start,variant,derived,class_tag,enabled,present in [
            *[(3,0x14229b5fd,v,d,0,False,True)for v,d in [(0,0),(1,0),(1,9),(1,10)]],
            *[(6,0x14229b5b9,0,0,c,e,p)for e in [False,True]for c,p in [(0,False),(0,True),(1,True),(2,True),(3,True),(4,True),(5,True)]],
            *[(10,0x14229ae6e,0,d,0,e,p)for d in [0,20]for e in [False,True]for p in [False,True]],
        ]:
            code=prefix+mov_abs_jump(start)+capture;C.memmove(wrapper,code,len(code))
            for demo in [False,True]:
                overwrite(mode_addr,bytes([int(demo)]))
                out=C.create_string_buffer(24);target=C.create_string_buffer(24);target[0]=bytes([kind]);target[1]=bytes([variant])
                decorator=C.create_string_buffer(0x80);extra=C.create_string_buffer(32)
                decorator[0x22]=bytes([class_tag]);extra[0]=bytes([int(enabled)]);extra[4]=bytes([int(enabled)])
                fixture.update(decorator=C.addressof(decorator)if present else None,derived_kind=derived,extra=C.addressof(extra))
                print(f"fixture {kind=} {variant=} {derived=} {class_tag=} {enabled=} {present=} {demo=}",flush=True)
                fn(out,target,None,None)
                cap,ptr,count=struct.unpack('<QQQ',out.raw)
                assert 0<count<=50 and count<=cap
                values=[]
                for i in range(count):
                    record=read(ptr+i*40,40)
                    name_cap,name_ptr,name_len=struct.unpack_from('<QQQ',record)
                    assert name_cap==name_len and name_len<256
                    values.append({'name':read(name_ptr,name_len).decode('utf8'),'kind':record[24],
                      'b19':record[25],'u1a':struct.unpack_from('<H',record,26)[0],
                      'v1c':struct.unpack_from('<I',record,28)[0],'v20':struct.unpack_from('<I',record,32)[0],
                      'v24':struct.unpack_from('<I',record,36)[0],'raw_payload':record[24:].hex()})
                rows.append({'target_kind':kind,'original_start':hex(start),'demo':demo,'variant':variant,'derived_kind':derived,'class_tag':class_tag,'enabled':enabled,'present':present,'count':count,'icons':values})
                (ROOT/'show-special-fixture-progress.json').write_text(json.dumps(rows,indent=2),encoding='utf8')
    finally:k.VirtualFree(wrapper,0,0x8000)
    result={'original_sha256':hashlib.sha256(raw).hexdigest(),'address':'0x14229a7f0',
       'wooden_helper_sha256':hashlib.sha256(original).hexdigest(),'original_function_sha256':hashlib.sha256(read(0x14229a7f0,74991)).hexdigest(),'method':'original conditional show builder instructions; explicit fixture hooks only for decorator lookup, derived kind lookup and chimney extra-info access; generic alloc/realloc/free/memmove host shims; capture before common layout. This validates option construction, not original world resource lookup.',
       'allocation_count':len(allocations),'rows':rows}
    (ROOT/'show-special-original-execution.json').write_text(json.dumps(result,indent=2),encoding='utf8')
    print(json.dumps({'output':'show-special-original-execution.json','rows':[(r['target_kind'],r['demo'],r['count'])for r in rows]}))
finally:k.VirtualFree(base,0,0x8000)
