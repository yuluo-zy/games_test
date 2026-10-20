"""Execute original Sphere::show, including original std Debug formatting.
Only allocator/CRT and canonical append-stream destination are bridged.
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
    @C.CFUNCTYPE(C.c_void_p,C.c_void_p,C.c_uint64,C.c_uint64,C.c_uint64)
    def reallocate(pointer,old,alignment,new):
        result=allocate(new,alignment);C.memmove(result,pointer,min(old,new));return result
    crt=C.CDLL('ucrtbase')
    for va,fn in [(0x140613c10,allocate),(0x140613c20,deallocate),(0x140613c30,reallocate),(0x140613c40,allocate),(0x142923c00,memory_move),(0x142923c10,memory_move),(0x142923bf0,crt.memcmp)]:
        put(va,b'\x48\xb8'+struct.pack('<Q',C.cast(fn,C.c_void_p).value)+b'\xff\xe0')
    records=[]
    @C.CFUNCTYPE(None,C.c_void_p,C.c_void_p)
    def append_stream(stream,record):
        cap,ptr,n,arc,vtable=struct.unpack('<5Q',read(record,40))
        env=read(arc+16,40)
        meshptr,meshlen=struct.unpack_from('<QQ',env)
        records.append({'name':read(ptr,n).decode(),'mesh_name':read(meshptr,meshlen).decode(),'env_center_bits':list(struct.unpack_from('<3I',env,16)),'env_radius_bits':struct.unpack_from('<I',env,28)[0],'env_width_bits':struct.unpack_from('<I',env,32)[0],'env_solid':env[36],'vtable':hex(vtable)})
    put(0x14146c880,b'\x48\xb8'+struct.pack('<Q',C.cast(append_stream,C.c_void_p).value)+b'\xff\xe0')
    fn=C.CFUNCTYPE(None,C.c_void_p,C.c_void_p)(0x140b02520)
    cases=[]
    for color in range(6):
        sphere=C.create_string_buffer(40)
        put(C.addressof(sphere),struct.pack('<I3ff3ffB',color,.2,.4,.6,.1,1.,2.,3.,1.,False))
        fn(sphere,1)
        cases.append({'color':color,**records[-1]})
    (OUT/'native-sphere-show-fixtures.json').write_text(json.dumps({'original_pe_sha256':hashlib.sha256(raw).hexdigest(),'cases':cases,'scope':'Full original676B show, original Debug/format machinery; only generic allocator/CRT and final append destination bridged'},indent=2),encoding='utf8')
    print(json.dumps(cases))
finally:k.VirtualFree(base,0,0x8000)
