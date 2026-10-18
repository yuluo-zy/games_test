from pathlib import Path
import hashlib, json, struct
ROOT = Path('/Users/yuluo/Desktop/game')
OUT = Path(__file__).resolve().parent
recipe = ROOT/'games_test/reverse-engineering/tiny-glade/evidence/shared-foundation-2026-10-13/utils-base/native_spline_rdp.py'
source = recipe.read_text().split('def spline(')[0].replace("Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')", "Path('/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')")
exec(compile(source,str(recipe),'exec'),globals())
OUT = Path(__file__).resolve().parent
ARC=INPUT+0x1000
TERRAIN=ARC+16
POINTS=INPUT+0x2000
SERIALIZER=INPUT+0x3000
BYTES=HEAP+0xd00000
def serialize(kind):
    reset()
    u.mem_write(INPUT,bytes(0x4000))
    u.mem_write(ARC,struct.pack('<2Q',1,1)+bytes(40))
    u.mem_write(INPUT,struct.pack('<2Q',0,ARC))
    if kind=='new':
        expected={'Terrain':{'NewTerrainStroke':{'index':9,'stroke_pts':[[1.0,2.0,3.0],[4.0,5.0,6.0]],'height_ws':2.5}}}
        u.mem_write(TERRAIN,struct.pack('<4QIf',0x8000000000000000,2,POINTS,2,9,2.5))
        u.mem_write(POINTS,pack([1,2,3,4,5,6]))
    elif kind=='edit':
        expected={'Terrain':{'EditTerrainStroke':{'index':7,'prev_height_ws':1.0,'new_height_ws':2.0,'prev_falloff':0.25,'new_falloff':0.5}}}
        u.mem_write(TERRAIN,struct.pack('<QI4f',0x8000000000000001,7,1.0,2.0,0.25,0.5))
    elif kind=='move':
        expected={'Terrain':{'MoveTerrainStroke':{'id':3,'original_stroke_pts':[],'move_vec':[1.25,-2.0]}}}
        u.mem_write(TERRAIN,struct.pack('<3Q2fI',0,POINTS,0,1.25,-2.0,3))
    u.mem_write(OUTPUT,struct.pack('<3Q',0x10000,BYTES,0))
    u.mem_write(SERIALIZER,struct.pack('<Q',OUTPUT)+bytes(24))
    u.reg_write(UC_X86_REG_RCX,INPUT)
    u.reg_write(UC_X86_REG_RDX,SERIALIZER)
    run(0x140316660)
    assert u.reg_read(UC_X86_REG_RAX)==0
    cap,ptr,n=struct.unpack('<3Q',u.mem_read(OUTPUT,24))
    actual=bytes(u.mem_read(ptr,n)).decode()
    assert json.loads(actual)==expected, (actual,expected)
    return {'kind':kind,'json':actual,'value':expected}
cases=[serialize(kind) for kind in ['new','edit','move']]
reset()
u.mem_write(INPUT,b'Terrain')
u.reg_write(UC_X86_REG_RCX,OUTPUT)
u.reg_write(UC_X86_REG_RDX,INPUT)
u.reg_write(UC_X86_REG_R8,7)
run(0x140b3b630)
result=bytes(u.mem_read(OUTPUT,2))
assert result==b'\x00\x00',result
report={'target':'B','binary_sha256':hashlib.sha256(raw).hexdigest(),'outer_serializer':'0x140316660','terrain_serializer':'0x1404f41b0','variant_visitor':'0x140b3b630','visitor_Terrain':{'success_tag':result[0],'variant_index':result[1]},'cases':cases,'boundary':'Original outer Arc<TerrainEdit> and all three terrain JSON serializers execute, with original Serde and Vec code. Only allocator/CRT are bridged. Native JSON parser and whole History save-file/reload are outside this leaf probe.'}
(OUT/'terrain-history-native-fixtures.json').write_text(json.dumps(report,indent=2)+'\n')
print({'original_terrain_serialize_cases':len(cases),'variant_visitor_pass':True})
