from pathlib import Path
import json,struct,hashlib
ROOT=Path('/Users/yuluo/Desktop/game')
OUT=Path(__file__).resolve().parent
recipe=ROOT/'games_test/reverse-engineering/tiny-glade/evidence/shared-foundation-2026-10-13/utils-base/native_spline_rdp.py'
source=recipe.read_text().split('def spline(')[0].replace("Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')", "Path('/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')")
exec(compile(source,str(recipe),'exec'),globals())
OUT=Path(__file__).resolve().parent
MODE=INPUT+0x1000;PROFILE=INPUT+0x2000;STATE=INPUT+0x3000;READER=INPUT+0x4000;CURSOR=INPUT+0x5000;EVENTS=INPUT+0x6000;BUFFER=INPUT+0x7000;COMPLETE=INPUT+0x8000;COMPLETEBUF=INPUT+0x9000;REQUEST=INPUT+0xa000;HOVER=INPUT+0xb000;TICK=INPUT+0xc000;PARAM=INPUT+0xd000
ENTRIES={'layering':0x142283cc0,'focus':0x142283980,'steepness':0x142283640}
def param(offset,data):
    p=PARAM+offset
    u.mem_write(p,struct.pack('<Q',data))
    u.mem_write(p+0x10,struct.pack('<Q',TICK))
    u.mem_write(p+0x1c,struct.pack('<I',7))
    return p
def case(kind,tags,done=False):
    sp=reset()
    u.mem_write(INPUT,bytes(0x10000))
    # Mode0 excludes the display-hint backend while preserving real event loops.
    u.mem_write(STATE,struct.pack('<QBB',0,int(done),int(done)) if kind=='layering' else bytes([int(done),int(done)]))
    u.mem_write(READER,struct.pack('<2Q',CURSOR,EVENTS))
    u.mem_write(EVENTS,struct.pack('<9Q',32,BUFFER,len(tags),0,0,8,0,len(tags),len(tags)))
    for i,tag in enumerate(tags):u.mem_write(BUFFER+16*i,struct.pack('<QB',i,tag)+bytes(7))
    u.mem_write(COMPLETE,struct.pack('<9Q',0,8,0,0,32,COMPLETEBUF,0,0,0))
    u.reg_write(UC_X86_REG_RCX,param(0x00,MODE))
    u.reg_write(UC_X86_REG_RDX,param(0x40,PROFILE))
    u.reg_write(UC_X86_REG_R8,param(0x80,STATE))
    u.reg_write(UC_X86_REG_R9,READER)
    u.mem_write(sp+0x28,struct.pack('<Q',param(0xc0,COMPLETE)))
    u.mem_write(sp+0x30,struct.pack('<Q',param(0x100,REQUEST)))
    u.mem_write(sp+0x38,struct.pack('<Q',param(0x140,HOVER)))
    run(ENTRIES[kind])
    count=struct.unpack('<Q',u.mem_read(COMPLETE+0x30,8))[0]
    state=list(u.mem_read(STATE,10 if kind=='layering' else 2))
    return {'kind':kind,'tags':tags,'initial_done':done,'counter':struct.unpack('<Q',bytes(state[:8]))[0] if kind=='layering' else None,'state_bits':state,'completion_tags':[u.mem_read(COMPLETEBUF+16*i+8,1)[0] for i in range(count)],'reader_consumed':struct.unpack('<Q',u.mem_read(CURSOR,8))[0]}
cases=[case(kind,tags,done)for kind in ENTRIES for tags in [[],[0],[1],[2],[0,0,1],[1,1],[2,2]]for done in [False,True]]
(OUT/'native-hint-terrain-fixtures.json').write_text(json.dumps({'target':'B','binary_sha256':hashlib.sha256(raw).hexdigest(),'entries':{k:hex(v)for k,v in ENTRIES.items()},'cases':cases,'boundary':'Actual original event-reader loops, state writes and completion EventInstance writes execute. Mode0 suppresses display backend; queues preallocated; only allocator/CRT boundaries bridged. No Rust run or display builder acceptance.'},indent=2)+'\n')
print({'original_cases':len(cases)})
