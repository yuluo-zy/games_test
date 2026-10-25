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

PENDING=INPUT+0xc000;PENDING_PARAM=INPUT+0xd000;WRITER2=INPUT+0xe000;EVENTS2=INPUT+0xf000;BUFFER2=INPUT+0x10000

def release(points,height):
    global missing
    reset();missing=False
    for p in [STROKE,TERRAIN,PARAM,WRITER,EVENTS,BUFFER,TICK,POINTS,PENDING,PENDING_PARAM,WRITER2,EVENTS2,BUFFER2]:u.mem_write(p,bytes(512))
    u.mem_write(POINTS,pack([v for p in points for v in p]));u.mem_write(STROKE,struct.pack('<3Q',32,POINTS,len(points)));u.mem_write(STROKE+0x20,pack([height]))
    u.mem_write(TERRAIN,struct.pack('<I',1));u.mem_write(TERRAIN+0x38,struct.pack('<Q',10))
    for p,data in [(PARAM,TERRAIN),(WRITER,EVENTS),(WRITER2,EVENTS2),(PENDING_PARAM,PENDING)]:
        u.mem_write(p,struct.pack('<Q',data));u.mem_write(p+0x10,struct.pack('<Q',TICK));u.mem_write(p+0x1c,struct.pack('<I',77))
    for queue,buffer in [(EVENTS,BUFFER),(EVENTS2,BUFFER2)]:
        u.mem_write(queue+0x20,struct.pack('<3Q',32,buffer,0));u.mem_write(queue+0x40,struct.pack('<Q',42))
    u.mem_write(PENDING,struct.pack('<Q',0x8000000000000002))
    frame=STACK+0x8000;u.mem_write(frame+0x110,struct.pack('<Q',PENDING_PARAM));u.mem_write(frame+0x138,struct.pack('<Q',WRITER));u.mem_write(frame+0x140,struct.pack('<Q',WRITER2))
    u.reg_write(UC_X86_REG_RBP,frame);u.reg_write(UC_X86_REG_RCX,PARAM)
    u.emu_start(0x140aa02c7,0x140aa076d,count=100000)
    state,cap,ptr,n,index,h=struct.unpack('<4Q2I',u.mem_read(PENDING,40))
    return {'points':[[bits(v)for v in p]for p in points],'height':bits(height),'pending_state':state,'pending_points':list(struct.unpack('<'+'I'*(n*3),u.mem_read(ptr,n*12))),'pending_index':index,'pending_height':h,'editing_active':struct.unpack('<I',u.mem_read(TERRAIN,4))[0],'placement_count':struct.unpack('<Q',u.mem_read(EVENTS+0x30,8))[0],'recalculate_count':struct.unpack('<Q',u.mem_read(EVENTS2+0x30,8))[0]}
cases=[release([[0,0,0]],2.5),release([[1,2,3],[4,5,6]],7.0),release([],3.0)]
(OUT/'native-release-fixtures.json').write_text(json.dumps({'binary_sha256':hashlib.sha256(raw).hexdigest(),'range':'0x140aa02c7..0x140aa076d','method':'Original state3 instructions with explicitly supplied last stroke, preallocated queues, allocator/CRT bridges; native point cloning, pending payload/id/height, event writes and editing flag clear execute.','cases':cases},indent=2)+'\n')
print({'release_cases':len(cases),'indices':[c['pending_index']for c in cases]})
