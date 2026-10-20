"""Original Sphere factory plus original3840B SignifierEntityBuilder.
Only named asset lookup results, ECS reservation, generic memory and profiler
boundaries are bridged. No builder/command/shape policy is replaced.
"""
from pathlib import Path
template=Path(__file__).resolve().parent.parent/'color-startup/native_grading_oracle.py'
exec(template.read_text(encoding='utf8').split('try:\n',1)[0])
OUT=Path(__file__).resolve().parent
try:
    put(base,raw[:struct.unpack_from('<I',raw,opt+60)[0]])
    sec=opt+struct.unpack_from('<H',raw,pe+20)[0]
    for i in range(struct.unpack_from('<H',raw,pe+6)[0]):
        size,rva,n,ptr=struct.unpack_from('<IIII',raw,sec+i*40+8)
        if n:put(base+rva,raw[ptr:ptr+n])
    crt=C.CDLL('ucrtbase')
    for va,fn in [(0x140613c10,allocate),(0x140613c20,deallocate),(0x142923c00,memory_move),(0x142923c10,memory_move),(0x142923bf0,crt.memcmp)]:
        put(va,b'\x48\xb8'+struct.pack('<Q',C.cast(fn,C.c_void_p).value)+b'\xff\xe0')
    @C.CFUNCTYPE(C.c_uint8)
    def profiler():return 0
    put(0x140c16a00,b'\x48\xb8'+struct.pack('<Q',C.cast(profiler,C.c_void_p).value)+b'\xff\xe0')
    # Real Option<Handle> return ABI occupies RAX(tag), RDX(handle).
    for va,handle in [(0x14092d570,0xa11),(0x140937aa0,0xb22)]:
        put(va,b'\x48\xb8'+struct.pack('<Q',1)+b'\x48\xba'+struct.pack('<Q',handle)+b'\xc3')
    queue=C.create_string_buffer(24);data=C.create_string_buffer(4096)
    @C.CFUNCTYPE(C.c_void_p,C.c_void_p,C.c_void_p)
    def spawn(out,commands):
        put(out,bytes(40));put(out+8,struct.pack('<Q',C.addressof(queue)));put(out+32,struct.pack('<Q',0x100000007));return out
    put(0x140d146a0,b'\x48\xb8'+struct.pack('<Q',C.cast(spawn,C.c_void_p).value)+b'\xff\xe0')
    name=C.create_string_buffer(b'debug_sphere_red');env=C.create_string_buffer(40)
    result=C.create_string_buffer(24);fn=C.CFUNCTYPE(C.c_void_p,*([C.c_void_p]*5))(0x140b027d0)
    fixtures=[]
    for solid in [False,True]:
        put(C.addressof(queue),struct.pack('<3Q',4096,C.addressof(data),0));C.memset(data,0xcc,4096)
        put(C.addressof(env),struct.pack('<QQ3fffB',C.addressof(name),len(name.value),1.,2.,3.,.15,2.,solid))
        fn(result,env,1,2,3)
        n=struct.unpack_from('<Q',queue.raw,16)[0];retcap,retptr,retn=struct.unpack('<3Q',result.raw)
        assert retn==1 and struct.unpack('<Q',read(retptr,8))[0]==0x100000007
        rawqueue=read(C.addressof(data),n)
        # Bundle152 + UiPass32 + DontCastShadow32, optional topology40.
        assert n==216+(0 if solid else 40),n
        fixtures.append({'filled_in':solid,'queue_bytes':n,'bundle_executor':hex(struct.unpack_from('<Q',rawqueue)[0]),'transform_bits':list(struct.unpack_from('<10I',rawqueue,8)),'mesh':hex(struct.unpack_from('<Q',rawqueue,56)[0]),'shader':hex(struct.unpack_from('<Q',rawqueue,64)[0]),'animation_tag':struct.unpack_from('<I',rawqueue,72)[0],'animation_flags':list(rawqueue[80:82]),'animation_value_bits':struct.unpack_from('<I',rawqueue,84)[0],'animation_duration_bits':struct.unpack_from('<I',rawqueue,88)[0],'animation_t_bits':struct.unpack_from('<I',rawqueue,92)[0],'animation_eased_bits':struct.unpack_from('<I',rawqueue,96)[0],'marker_executors':[hex(struct.unpack_from('<Q',rawqueue,i)[0]) for i in [152,184]],'topology':None if solid else {'executor':hex(struct.unpack_from('<Q',rawqueue,216)[0]),'tag':struct.unpack_from('<I',rawqueue,232)[0],'width_bits':struct.unpack_from('<I',rawqueue,236)[0]},'returned_entity':hex(0x100000007)})
    (OUT/'native-sphere-full-build-fixtures.json').write_text(json.dumps({'original_pe_sha256':hashlib.sha256(raw).hexdigest(),'cases':fixtures,'scope':'Full original Sphere1585B plus builder3840B named-mesh branch; all emitted original queue records, asset-result/ECS reservation boundaries explicit'},indent=2),encoding='utf8')
    print(json.dumps(fixtures))
finally:k.VirtualFree(base,0,0x8000)
