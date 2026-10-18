"""B resolved terrain continuation branch; lookup is explicitly supplied."""
import json,struct,hashlib
from pathlib import Path
OUT=Path(__file__).resolve().parent
recipe=Path('/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/shared-foundation-2026-10-13/utils-base/native_spline_rdp.py')
source=recipe.read_text().split('def spline(')[0]
source=source.replace("Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')", "Path('/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')")
exec(compile(source,str(recipe),'exec'),globals())
OUT=Path(__file__).resolve().parent
STROKE=INPUT+0x4000;TERRAIN=INPUT+0x5000;PARAM=INPUT+0x6000;WRITER=INPUT+0x7000;EVENTS=INPUT+0x8000;BUFFER=INPUT+0x9000;TICK=INPUT+0xa000;POINTS=INPUT+0xb000
missing=False
def resolved_lookup(uc,address,size,data):
    if address==0x140924900:ret(0 if missing else STROKE)
u.hook_add(UC_HOOK_CODE,resolved_lookup)
def case(points,point,inside,has_stroke=True):
    global missing
    reset();missing=not has_stroke
    for p in [STROKE,TERRAIN,PARAM,WRITER,EVENTS,BUFFER,TICK,POINTS]:u.mem_write(p,bytes(512))
    u.mem_write(POINTS,pack([v for p in points for v in p]));u.mem_write(STROKE,struct.pack('<3Q',32,POINTS,len(points)))
    u.mem_write(TERRAIN+0x38,struct.pack('<Q',10));u.mem_write(PARAM,struct.pack('<Q',TERRAIN));u.mem_write(PARAM+0x10,struct.pack('<Q',TICK));u.mem_write(PARAM+0x1c,struct.pack('<I',77))
    u.mem_write(WRITER,struct.pack('<Q',EVENTS));u.mem_write(WRITER+0x10,struct.pack('<Q',TICK));u.mem_write(WRITER+0x1c,struct.pack('<I',77))
    u.mem_write(EVENTS+0x20,struct.pack('<3Q',32,BUFFER,0));u.mem_write(EVENTS+0x40,struct.pack('<Q',42))
    frame=STACK+0x8000;u.mem_write(frame+0x48,pack(point));u.reg_write(UC_X86_REG_RBP,frame);u.reg_write(UC_X86_REG_RCX,PARAM);u.reg_write(UC_X86_REG_RDI,WRITER);u.reg_write(UC_X86_REG_RBX,int(inside))
    u.emu_start(0x140aa0235,0x140aa076d,count=100000)
    assert u.reg_read(UC_X86_REG_RIP)==0x140aa076d
    n=struct.unpack('<Q',u.mem_read(STROKE+0x10,8))[0];event_n=struct.unpack('<Q',u.mem_read(EVENTS+0x30,8))[0]
    return {'input':[[bits(v)for v in p]for p in points],'point':[bits(v)for v in point],'inside':inside,'has_stroke':has_stroke,'points':list(struct.unpack('<'+'I'*(n*3),u.mem_read(POINTS,n*12))),'event_count':event_n,'event':list(struct.unpack('<QI',u.mem_read(BUFFER,12))) if event_n else None}
inputs=[[],[[0,0,0]],[[0,0,0],[.2,0,0]],[[0,0,0],[.4,0,0]],[[0,0,0],[.3,0,0]]]
cases=[case(p,[.8,.6,.1],inside) for p in inputs for inside in [False,True]]
cases.append(case([[0,0,0]],[0,0,0],True));cases.append(case([[0,0,0]],[.8,.6,.1],True,False))
report={'binary_sha256':hashlib.sha256(raw).hexdigest(),'range':'0x140aa0235..0x140aa076d','method':'Execute original state2 and original insert_or_morph_last_point; return an explicitly resolved stroke for get_last_mut_stroke. Hash lookup, ECS resolution and grow branches excluded; event and point allocations preallocated. Original 0.3 load, inside gate, XYZ insertion, event id=next_index-1, counter updates execute.','cases':cases}
(OUT/'native-extend-fixtures.json').write_text(json.dumps(report,indent=2)+'\n');print({'native_cases':len(cases),'events':[c['event_count']for c in cases]})
