"""Execute the original1129B entire threshold visualization producer.
Hash result is a generic dependency bridge; real map traversal, validated wall
lookup and the four-sphere business policy execute unchanged.
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
    # Hashbrown's game-independent hash result selects a valid fixture bucket.
    put(0x1405b0d70,b'\x31\xc0\xc3')
    records=[]
    @C.CFUNCTYPE(None,C.c_void_p,C.c_void_p)
    def sphere_show(sphere,stream):
        b=read(sphere,40)
        records.append({'color':struct.unpack_from('<I',b)[0],'radius_bits':struct.unpack_from('<I',b,16)[0],'position_bits':list(struct.unpack_from('<3I',b,20)),'line_width_bits':struct.unpack_from('<I',b,32)[0],'filled_in':b[36]})
    put(0x140b02520,b'\x48\xb8'+struct.pack('<Q',C.cast(sphere_show,C.c_void_p).value)+b'\xff\xe0')
    def owned(data):
        p=allocate(max(1,len(data)),16);put(p,data);return p
    def reference(p):return owned(struct.pack('<Q',p))
    fn=C.CFUNCTYPE(None,C.c_void_p,C.c_void_p,C.c_void_p)(0x140a8b0c0)
    cases=[]
    finite=[([1.,2.,3.],.8,6.,1.25),([-0.,-0.,-0.],-0.,-0.,-0.),([4.,-2.,7.],-3.,10.,-9.)]
    exceptional=[([1.,2.,3.],float('inf'),6.,-float('inf')),([0.,0.,0.],float('nan'),float('nan'),float('nan')),([-0.,-0.,-0.],-float('inf'),-float('inf'),0.)]
    for enabled in [False,True]:
      for lengths in [(0,0,0,0),(1,1,1,1),(3,3,3,3),(2,1,3,4),(4,3,1,2),(2,3,4,0)]:
       for data in finite+exceptional:
        position,threshold,arch,third=data
        points=[position for _ in range(lengths[0])]
        arrays=[[threshold]*lengths[1],[arch]*lengths[2],[third]*lengths[3]]
        ctrl_alloc=owned(bytes(16*0x2a8+64));ctrl=ctrl_alloc+16*0x2a8;put(ctrl,bytes([0]+[255]*31));row=ctrl-0x2a8
        put(row,struct.pack('<Q',100));put(row+0x18,struct.pack('<Q',2));put(row+0x38,struct.pack('<f',1.))
        ptr=owned(b''.join(struct.pack('<3f',*p) for p in points));put(row+0x98,struct.pack('<QQ',ptr,len(points)))
        for offset,values in zip([0xd0,0xe8,0x100],arrays):
            ptr=owned(b''.join(struct.pack('<f',v) for v in values));put(row+offset,struct.pack('<QQ',ptr,len(values)))
        wallmap=owned(struct.pack('<4Q',ctrl,15,0,1)+bytes(32));debug=owned(bytes(256));put(debug+0xf3,bytes([enabled]));before=len(records)
        fn(reference(debug),reference(wallmap),reference(1))
        emitted=records[before:];expected=min(lengths)*4 if enabled else 0
        assert len(emitted)==expected,(lengths,len(emitted))
        cases.append({'enabled':enabled,'lengths':lengths,'position_bits':list(struct.unpack('<3I',struct.pack('<3f',*position))),'threshold_bits':struct.unpack('<I',struct.pack('<f',threshold))[0],'arch_bits':struct.unpack('<I',struct.pack('<f',arch))[0],'third_bits':struct.unpack('<I',struct.pack('<f',third))[0],'native':emitted})
    (OUT/'native-threshold-fixtures.json').write_text(json.dumps({'original_pe_sha256':hashlib.sha256(raw).hexdigest(),'cases':cases,'scope':'Original1129B entire producer; only generic hash result and final Sphere::show destination recorded, all map iteration/lookup/validity/zip/float arithmetic original'},indent=2),encoding='utf8')
    print('original1129B threshold producer:',len(cases),'cases')
finally:k.VirtualFree(base,0,0x8000)
