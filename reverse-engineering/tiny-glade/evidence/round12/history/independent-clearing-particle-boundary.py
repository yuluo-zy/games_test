"""Independent additional original521B boundary oracle; no Rust/codegen.

Reuse only the Windows image mapping/CRT host setup. Do not execute PE entry.
"""
from pathlib import Path
template=Path(__file__).resolve().parents[1]/'color-startup/native_grading_oracle.py'
exec(template.read_text(encoding='utf8').split('try:\n',1)[0])
import itertools,math
REVIEW=OUT/'clearing-independent-review';REVIEW.mkdir(exist_ok=True)
try:
    put(base,raw[:struct.unpack_from('<I',raw,opt+60)[0]])
    sec=opt+struct.unpack_from('<H',raw,pe+20)[0]
    for i in range(struct.unpack_from('<H',raw,pe+6)[0]):
        size,rva,n,ptr=struct.unpack_from('<IIII',raw,sec+i*40+8)
        if n:put(base+rva,raw[ptr:ptr+n])
    original=read(0x140a123a0,521)
    assert hashlib.sha256(original).hexdigest()=='620b666f1883bd656911c89f06e56b04d289df9900a057ab37e9344c90b920bb'
    crt=C.CDLL('ucrtbase')
    for va,fn in [(0x142923fd0,crt.sinf),(0x142923e80,crt.cosf)]:
        put(va,b'\x48\xb8'+struct.pack('<Q',C.cast(fn,C.c_void_p).value)+b'\xff\xe0')
    particle=C.CFUNCTYPE(None,C.c_void_p,C.c_void_p,C.c_void_p)(0x140a123a0)
    rows=[]
    positions=[(0.,0.,0.),(-0.,-0.,-0.)]
    vertices=list(itertools.product([0.,-0.],repeat=3))+[(1.,2.,3.),(-2.,0.5,4.)]
    for position,angle,scale,vertex in itertools.product(positions,[-.75,.75,-0.,0.,-90.,90.],[1.,2.,math.inf],vertices):
        placement=C.create_string_buffer(struct.pack('<5fi6I',*position,scale,angle,1,0,0,0,0,0,0))
        reference=C.c_void_p(C.addressof(placement));environment=C.c_void_p(C.addressof(reference))
        point=C.create_string_buffer(struct.pack('<3f',*vertex));output=C.create_string_buffer(20)
        particle(output,C.byref(environment),point)
        assert struct.unpack_from('<I',output.raw)[0]==1
        rows.append({'position_bits':list(struct.unpack_from('<3I',placement.raw)),
            'scale_bits':struct.unpack_from('<I',placement.raw,12)[0],
            'angle_bits':struct.unpack_from('<I',placement.raw,16)[0],
            'vertex_bits':list(struct.unpack('<3I',point.raw[:12])),
            'native_bits':list(struct.unpack_from('<4I',output.raw,4))})
    (REVIEW/'particle-extra-original.json').write_text(json.dumps({'original_sha256':hashlib.sha256(raw).hexdigest(),
        'function_sha256':hashlib.sha256(original).hexdigest(),'raw_hex':original.hex(),
        'scope':'360 actual original executions, CRT sinf/cosf only; no Rust comparison yet','cases':rows},indent=2),encoding='utf8')
    print(json.dumps({'case_count':len(rows),'all_negative_zero_case':next(r for r in rows if
        r['position_bits']==[0x80000000]*3 and r['vertex_bits']==[0x80000000]*3
        and r['angle_bits']==struct.unpack('<I',struct.pack('<f',-.75))[0] and r['scale_bits']==0x3f800000)},indent=2))
finally:k.VirtualFree(base,0,0x8000)
