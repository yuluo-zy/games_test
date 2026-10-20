"""Original1585B Sphere factory; record existing builder boundary and queue."""
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
    queue=C.create_string_buffer(24);data=C.create_string_buffer(1024);builders=[]
    @C.CFUNCTYPE(C.c_void_p,*([C.c_void_p]*5))
    def build(out,builder,meshes,shaders,commands):
        b=read(builder,144);builders.append(b.hex())
        put(out,bytes(40));put(out+8,struct.pack('<Q',C.addressof(queue)));put(out+32,struct.pack('<Q',0x100000007));return out
    put(0x14089dd50,b'\x48\xb8'+struct.pack('<Q',C.cast(build,C.c_void_p).value)+b'\xff\xe0')
    name=C.create_string_buffer(b'debug_sphere_red');env=C.create_string_buffer(40)
    result=C.create_string_buffer(24);fn=C.CFUNCTYPE(C.c_void_p,*([C.c_void_p]*5))(0x140b027d0)
    fixtures=[]
    for solid in [False,True]:
        put(C.addressof(queue),struct.pack('<3Q',1024,C.addressof(data),0));C.memset(data,0xcc,1024)
        put(C.addressof(env),struct.pack('<QQ3fffB',C.addressof(name),len(name.value),1.,2.,3.,.15,2.,solid))
        fn(result,env,1,2,3)
        n=struct.unpack_from('<Q',queue.raw,16)[0];retcap,retptr,retn=struct.unpack('<3Q',result.raw)
        assert retn==1 and struct.unpack('<Q',read(retptr,8))[0]==0x100000007
        fixtures.append({'filled_in':solid,'builder144_hex':builders[-1],'queue_bytes':n,'queued_raw_hex':read(C.addressof(data),n).hex(),'returned_entity':hex(0x100000007)})
    (OUT/'native-sphere-closure-fixtures.json').write_text(json.dumps({'original_pe_sha256':hashlib.sha256(raw).hexdigest(),'cases':fixtures,'scope':'Full original1585B closure; profiler disabled, actual generic allocator/memcpy, only SignifierEntityBuilder::build destination recorded'},indent=2),encoding='utf8')
    print(json.dumps(fixtures))
finally:k.VirtualFree(base,0,0x8000)
