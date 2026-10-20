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
@C.WINFUNCTYPE(C.c_long,C.c_void_p)
def exception_probe(info):
    exc,context=struct.unpack('<QQ',C.string_at(info,16))
    regs={name:hex(struct.unpack('<Q',C.string_at(context+off,8))[0])for name,off in [('rax',0x78),('rcx',0x80),('rdx',0x88),('rbx',0x90),('rsp',0x98),('rbp',0xa0),('rsi',0xa8),('rdi',0xb0),('r8',0xb8),('r9',0xc0),('r15',0xf0),('rip',0xf8)]}
    regs['stack']=[hex(x)for x in struct.unpack('<'+('Q'*32),C.string_at(int(regs['rsp'],16),256))]
    (ROOT/'wall-flat-native-fault.json').write_text(json.dumps(regs,indent=2),encoding='utf8')
    print('NATIVE EXCEPTION '+json.dumps(regs),flush=True)
    return 0
k.AddVectoredExceptionHandler.argtypes=[C.c_uint32,C.c_void_p];k.AddVectoredExceptionHandler.restype=C.c_void_p
handler=k.AddVectoredExceptionHandler(1,C.cast(exception_probe,C.c_void_p))
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
    @C.CFUNCTYPE(C.c_void_p,C.c_void_p,C.c_uint64,C.c_void_p)
    def find_wall(storage,ident,location):return fixture['wall']
    @C.CFUNCTYPE(C.c_uint64,C.c_void_p,C.c_uint64)
    def find_roof(storage,ident):return fixture['roof_entity']
    @C.CFUNCTYPE(C.c_uint64,C.c_void_p)
    def wall_index(ident):return 0
    @C.CFUNCTYPE(C.c_uint8,C.c_void_p)
    def roof_flat(roof):return fixture['flat']
    for address,hook in [(0x140a91ef0,find_wall),(0x140a922b0,find_roof),(0x1400885d0,wall_index),(0x1408e3380,roof_flat)]:
        overwrite(address,mov_abs_jump(C.cast(hook,C.c_void_p).value))
    @C.CFUNCTYPE(C.c_void_p,C.c_void_p)
    def stairs_extra(data):return fixture['extra'] if fixture['has_extra'] else None
    @C.CFUNCTYPE(None,C.c_void_p,C.c_void_p)
    def camera_eye(out,camera):C.memmove(out,struct.pack('<fff',10.0,20.0,30.0),12)
    for address,hook in [(0x140839490,stairs_extra),(0x1408394b0,extra_info),(0x140839420,extra_info),(0x140911dd0,camera_eye)]:
        overwrite(address,mov_abs_jump(C.cast(hook,C.c_void_p).value))
    prefix=bytes.fromhex('55 41 57 41 56 41 55 41 54 56 57 53 48 81 ec 08 05 00 00 48 8d ac 24 80 00 00 00')
    prefix+=xmm_stack(0x11)
    prefix+=bytes.fromhex('48 89 8d d0 03 00 00 49 89 d7 48 89 95 f0 01 00 00 4c 89 85 00 02 00 00 4c 89 8d 58 03 00 00')
    # Exact pre-extracted system parameter views used by the original option blocks.
    prefix+=bytes.fromhex('49 8b 30 49 8b 40 08 48 89 85 90 00 00 00 49 8b 40 10 48 89 85 08 01 00 00 49 8b 40 18 48 89 85 28 01 00 00 49 8b 00 48 89 85 78 01 00 00 49 8b 40 18 48 89 85 a8 00 00 00 31 ff 41 80 3f 00 40 0f 94 c7 c1 e7 02')
    prefix+=bytes.fromhex('49 8b 40 20 48 89 45 28 49 8b 40 10 48 89 85 88 00 00 00 49 8b 40 18 48 89 85 b0 01 00 00')
    prefix+=bytes.fromhex('48 c7 85 10 01 00 00 00 00 00 00 48 c7 85 18 01 00 00 08 00 00 00 48 c7 85 20 01 00 00 00 00 00 00 b8 00 00 80 3f 66 44 0f 6e f0')
    capture=bytes.fromhex('48 8b 85 d0 03 00 00 0f 10 85 80 01 00 00 0f 11 00 0f 10 85 90 01 00 00 0f 11 40 10 8b 95 a0 01 00 00 89 50 20')
    capture+=xmm_stack(0x10)
    capture+=bytes.fromhex('48 81 c4 08 05 00 00 5b 5f 5e 41 5c 41 5d 41 5e 41 5f 5d c3')
    wrapper=k.VirtualAlloc(None,4096,0x3000,0x40)
    assert wrapper
    overwrite(0x14229f2c9,mov_abs_jump(wrapper+len(prefix)+12))
    fn=C.CFUNCTYPE(None,C.c_void_p,C.c_void_p,C.c_void_p,C.c_void_p)(wrapper)
    rows=[]
    try:
        for kind,roof_present,has_extra,enabled in [
            (kind,present,extra,enabled) for kind in [0,1,2,3,5,6,8,9,10,11,12]
            for present in ([False,True]if kind==0 else[True])
            for extra in ([False,True]if kind==11 else[True])
            for enabled in ([False,True]if kind in[6,10]else[False])
        ]:
            code=prefix+mov_abs_jump(0x14229ea64)+capture;C.memmove(wrapper,code,len(code))
            demo=False
            out=C.create_string_buffer(36);target=C.create_string_buffer(24);target[0]=bytes([kind])
            wall=C.create_string_buffer(0x300);struct.pack_into('<I',wall,0x258,0);wall[0x295]=b'\1';wall[0x299]=b'\2';struct.pack_into('<f',wall,0x290,0.625)
            colors=C.create_string_buffer(struct.pack('<I',21));colors_vec=C.create_string_buffer(struct.pack('<QQQ',1,C.addressof(colors),1))
            half=C.create_string_buffer(struct.pack('<I',41));half_vec=C.create_string_buffer(struct.pack('<QQQ',1,C.addressof(half),1))
            gate=C.create_string_buffer(struct.pack('<I',31));gate_vec=C.create_string_buffer(struct.pack('<QQQ',1,C.addressof(gate),1))
            roof=C.create_string_buffer(88);roof[0x3d]=b'\1';roof[0x40]=b'\2';roof[0x41]=b'\1';struct.pack_into('<III',roof,0x44,99,88,0);struct.pack_into('<I',roof,0x50,55)
            bitwords=C.create_string_buffer(struct.pack('<Q',1));component_indices=C.create_string_buffer(struct.pack('<Q',0xffffffffffffffff))
            table=C.create_string_buffer(48);struct.pack_into('<Q',table,0x10,C.addressof(roof))
            chunk=C.create_string_buffer(72);struct.pack_into('<Q',chunk,0x18,C.addressof(table));struct.pack_into('<Q',chunk,0x38,C.addressof(component_indices));struct.pack_into('<Q',chunk,0x40,1)
            record=C.create_string_buffer(struct.pack('<IIIII',1,0,0,0,0));entities=C.create_string_buffer(0x1b0);struct.pack_into('<QQ',entities,8,C.addressof(record),1);struct.pack_into('<Q',entities,0x1a8,C.addressof(chunk))
            query=C.create_string_buffer(0x120);struct.pack_into('<Q',query,0x28,C.addressof(bitwords));struct.pack_into('<Q',query,0x38,64)
            ctx=C.create_string_buffer(struct.pack('<QQQQQ',C.addressof(colors_vec),C.addressof(half_vec),C.addressof(query),C.addressof(entities),C.addressof(gate_vec)))
            decorator=C.create_string_buffer(0x80);struct.pack_into('<I',decorator,0x28,22)
            extra=C.create_string_buffer(32);struct.pack_into('<f',extra,0,0.375);struct.pack_into('<II',extra,8,51,4);extra[0x10]=b'\3'
            if kind==6:extra[0]=bytes([int(enabled)])
            extra[4]=bytes([int(enabled)])
            fixture.update(wall=C.addressof(wall),roof_entity=0x100000000 if roof_present else 0,flat=True,
               decorator=C.addressof(decorator),extra=C.addressof(extra),has_extra=has_extra)
            print(f"snapshot {kind=} {roof_present=} {has_extra=} {enabled=}",flush=True)
            fn(out,target,ctx,None)
            payload=out.raw
            rows.append({'target_kind':kind,'roof_present':roof_present,'has_extra':has_extra,'enabled':enabled,'native_snapshot36':payload.hex()})
    finally:k.VirtualFree(wrapper,0,0x8000)
    result={'original_sha256':hashlib.sha256(raw).hexdigest(),'address':'0x14229a7f0',
       'wooden_helper_sha256':hashlib.sha256(original).hexdigest(),'original_function_sha256':hashlib.sha256(read(0x14229a7f0,74991)).hexdigest(),'method':'original full13-table initial-selection instructions until first create; explicit native query tables plus controlled lookup/extra-info/camera data dependency hooks; captures original36B state. Generic engine lookup equivalence is not claimed.',
       'allocation_count':len(allocations),'rows':rows}
    (ROOT/'show-selection-original-execution.json').write_text(json.dumps(result,indent=2),encoding='utf8')
    print(json.dumps({'output':'show-selection-original-execution.json','rows':len(rows)}))
finally:k.VirtualFree(base,0,0x8000)
