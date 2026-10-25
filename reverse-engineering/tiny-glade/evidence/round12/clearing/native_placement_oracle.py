"""Original forest initializer closure and particle-transform execution.

Execute original game policy on synthetic owned input records. No PE entry,
disk-save/game/GPU execution. Generic allocator/CRT boundaries only; no game hook.
"""
from pathlib import Path
_template=Path(__file__).resolve().parents[1]/'color-startup/native_grading_oracle.py'
# Reuse the already verified PE mapping/host-boundary setup, not game logic.
exec(_template.read_text(encoding='utf8').split('try:\n',1)[0])
import math
try:
    put(base,raw[:struct.unpack_from('<I',raw,opt+60)[0]])
    sec=opt+struct.unpack_from('<H',raw,pe+20)[0]
    for i in range(struct.unpack_from('<H',raw,pe+6)[0]):
        size,rva,n,ptr=struct.unpack_from('<IIII',raw,sec+i*40+8)
        if n:put(base+rva,raw[ptr:ptr+n])
    crt=C.CDLL('ucrtbase')
    for va,fn in [(0x140613c10,allocate),(0x140613c20,deallocate),(0x142923c00,memory_move),(0x142923c10,memory_move),
      (0x142923fd0,crt.sinf),(0x142923e80,crt.cosf)]:
        put(va,b'\x48\xb8'+struct.pack('<Q',C.cast(fn,C.c_void_p).value)+b'\xff\xe0')
    sha={hex(va):hashlib.sha256(read(va,n)).hexdigest() for va,n in [(0x1412fa010,500),(0x140a123a0,521)]}
    def f32(x):return struct.unpack('<f',struct.pack('<f',x))[0]
    constants={hex(va):struct.unpack('<f',read(va,4))[0] for va in [0x142934620,0x1429cd030]}
    descriptors={}
    for va in [0x142ad6ca0,0x142ad6cc8]:
        pairs=[]
        for i in range(2):
            ptr,n=struct.unpack('<QQ',read(va+i*16,16));pairs.append(read(ptr,n).decode())
        descriptors[hex(va)]=pairs
    extend=C.CFUNCTYPE(None,C.c_void_p,C.c_void_p,C.c_void_p)(0x1412fa010)
    particle=C.CFUNCTYPE(None,C.c_void_p,C.c_void_p,C.c_void_p)(0x140a123a0)
    points=[(x,2,z) for x in [-math.inf,-66,-65,-64.999,0,64.999,65,66,math.inf,math.nan]
            for z in [-66,-65,0,64.999,65,math.nan]]
    placements=[]
    for i,pos in enumerate(points):
        placements.append(struct.pack('<5fi6I',*pos,1.25,3.5,-i-1,0x12345678,0xabcdef00+i,0x3c003c00,0x80000000,0x3f000000,0xff000000+i))
    assert all(len(p)==48 for p in placements)
    data=C.create_string_buffer(b''.join(placements));records=C.create_string_buffer(len(points)*80)
    vec=C.create_string_buffer(struct.pack('<QQQ',len(points),C.addressof(records),0))
    arc=C.create_string_buffer(struct.pack('<QQ',1,1)+b'clearing/tree/native-fixture')
    name=C.create_string_buffer(struct.pack('<QQ',C.addressof(arc),len(b'clearing/tree/native-fixture')))
    expansion=C.c_float(2.25)
    iterator=C.create_string_buffer(struct.pack('<QQQQ',C.addressof(data),C.addressof(data)+len(placements)*48,C.addressof(name),C.addressof(expansion)))
    extend(vec,iterator,None)
    cap,ptr,count=struct.unpack_from('<QQQ',vec.raw)
    selected=[(i,p) for i,p in enumerate(placements) if not(abs(struct.unpack_from('<f',p,0)[0])<65 and abs(struct.unpack_from('<f',p,8)[0])<65)]
    assert count==len(selected)
    rows=[]
    for j,(i,placement) in enumerate(selected):
        record=read(ptr+j*80,80)
        assert record[:16]==name.raw[:16] and record[16:64]==placement
        assert record[64:72]==bytes(8) and record[72:76]==struct.pack('<f',2.25)
        rows.append({'input_index':i,'placement_hex':placement.hex(),'entity_bits':0,'billboard_expansion_scale':2.25})
    assert struct.unpack_from('<Q',arc.raw)[0]==1+count
    particle_rows=[]
    max_radius=constants['0x142934620'];min_scale=constants['0x1429cd030']
    tests=[((0.,0.,0.),1.,0.,(1.,2.,3.)),((3.,4.,5.),2.,math.pi*.5,(-2.,.5,4.)),
      ((max_radius,0.,0.),1.,.25,(1.,2.,3.)),((0.,0.,0.),min_scale,.25,(1.,2.,3.)),
      ((0.,0.,0.),f32(min_scale+0.001),-.75,(1.,2.,3.)),((math.nan,0.,0.),1.,0.,(1.,2.,3.)),
      ((0.,0.,0.),math.nan,0.,(1.,2.,3.)),((0.,0.,0.),1.,math.nan,(1.,2.,3.))]
    for position,scale,angle,vertex in tests:
        placement=C.create_string_buffer(struct.pack('<5fi6I',*position,scale,angle,1,0,0,0,0,0,0))
        placement_ref=C.c_void_p(C.addressof(placement));env=C.c_void_p(C.addressof(placement_ref))
        local=C.create_string_buffer(struct.pack('<3f',*vertex));result=C.create_string_buffer(20)
        particle(result,C.byref(env),local)
        tag=struct.unpack_from('<I',result.raw)[0]
        particle_rows.append({'position_bits':list(struct.unpack_from('<3I',placement.raw)),
          'scale_bits':struct.unpack_from('<I',placement.raw,12)[0],
          'rotation_bits':struct.unpack_from('<I',placement.raw,16)[0],
          'vertex_bits':list(struct.unpack('<3I',local.raw[:12])),'result_tag':tag,
          'result_payload_hex':result.raw[4:20].hex() if tag else None})
    (OUT/'original-placement-fixtures.json').write_text(json.dumps({'original_pe_sha256':hashlib.sha256(raw).hexdigest(),
      'native_range_sha256':sha,'constants':constants,'clearing_shader_format_fragments':descriptors,
      'forest_input_count':len(points),'forest_output_count':count,'forest_rows':rows,'particle_rows':particle_rows,
      'scope':'Original policy leaves only, CRT sinf/cosf from ucrtbase; unused native padding excluded.'},indent=2),encoding='utf8')
    print(json.dumps({'forest_inputs':len(points),'forest_outputs':count,'particle_cases':len(particle_rows),'constants':constants,'shader_fragments':descriptors}))
finally:k.VirtualFree(base,0,0x8000)
