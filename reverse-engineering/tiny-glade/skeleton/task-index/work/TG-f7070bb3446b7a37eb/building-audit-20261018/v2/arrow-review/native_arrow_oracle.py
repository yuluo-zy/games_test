"""Execute full original B arrow query/event-consumption/numeric loop."""
import json,struct,hashlib
from pathlib import Path
OUT=Path(__file__).resolve().parent
recipe=Path('/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/shared-foundation-2026-10-13/utils-base/native_spline_rdp.py')
s=recipe.read_text().split('def spline(')[0].replace("Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')", "Path('/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')")
exec(compile(s,str(recipe),'exec'),globals());OUT=Path(__file__).resolve().parent
QUERY=INPUT+0x1000;QSTATE=INPUT+0x2000;WORLD=INPUT+0x3000;TABLE_IDS=INPUT+0x4000;TABLES=INPUT+0x5000;ARCHETYPES=INPUT+0x6000;COLUMNS=INPUT+0x7000;COMPMAP=INPUT+0x8000;ENTITY_ROWS=INPUT+0x9000;AXES=INPUT+0xa000;TICKS=INPUT+0xb000;ENABLE_READER=INPUT+0xc000;ENABLE_LAST=INPUT+0xd000;ENABLE_EVENTS=INPUT+0xe000;ENABLE_BUFFER=INPUT+0xf000;CURSOR_READER=INPUT+0x10000;CURSOR_LAST=INPUT+0x11000;CURSOR_EVENTS=INPUT+0x12000;CURSOR_BUFFER=INPUT+0x13000

def reader(wrapper,last,events,buffer,items,stride):
 u.mem_write(wrapper,struct.pack('<2Q',last,events));u.mem_write(events+0x20,struct.pack('<3Q',len(items),buffer,len(items)));u.mem_write(events+0x40,struct.pack('<Q',len(items)))
 for i,data in enumerate(items):u.mem_write(buffer+i*stride,struct.pack('<Q',i)+data)
def case(axes,enable_count,deltas):
 reset();frame=STACK+0x8000
 for p in[QUERY,QSTATE,WORLD,TABLE_IDS,TABLES,ARCHETYPES,COLUMNS,COMPMAP,ENTITY_ROWS,AXES,TICKS,ENABLE_READER,ENABLE_LAST,ENABLE_EVENTS,ENABLE_BUFFER,CURSOR_READER,CURSOR_LAST,CURSOR_EVENTS,CURSOR_BUFFER]:u.mem_write(p,bytes(512))
 u.mem_write(frame-0x80,bytes(512));u.mem_write(QUERY,struct.pack('<2Q',QSTATE,WORLD));u.mem_write(QUERY+0x14,struct.pack('<I',77));u.mem_write(QSTATE+0xf8,struct.pack('<2Q',TABLE_IDS,1));u.mem_write(WORLD+0x100,struct.pack('<Q',TABLES));u.mem_write(WORLD+0x1a8,struct.pack('<Q',ARCHETYPES));u.mem_write(TABLES+0x58,struct.pack('<Q',len(axes)));u.mem_write(TABLES+0x50,struct.pack('<Q',ENTITY_ROWS));u.mem_write(ARCHETYPES+0x18,struct.pack('<Q',COLUMNS));u.mem_write(ARCHETYPES+0x38,struct.pack('<Q',COMPMAP));u.mem_write(COMPMAP,struct.pack('<Q',0xffffffffffffffff));u.mem_write(COLUMNS+0x10,struct.pack('<Q',AXES));u.mem_write(COLUMNS+0x28,struct.pack('<Q',TICKS))
 for i,a in enumerate(axes):u.mem_write(AXES+i*8,pack(a));u.mem_write(ENTITY_ROWS+i*16+8,struct.pack('<I',i))
 reader(ENABLE_READER,ENABLE_LAST,ENABLE_EVENTS,ENABLE_BUFFER,[b'' for _ in range(enable_count)],8);reader(CURSOR_READER,CURSOR_LAST,CURSOR_EVENTS,CURSOR_BUFFER,[pack(v)for v in deltas],16)
 u.reg_write(UC_X86_REG_RBP,frame);u.reg_write(UC_X86_REG_R15,QUERY);u.reg_write(UC_X86_REG_R14,ENABLE_READER);u.reg_write(UC_X86_REG_RDI,CURSOR_READER);u.emu_start(0x140a931df,0x140a93642,count=100000)
 return {'axes_bits':[list(map(bits,a))for a in axes],'enable_count':enable_count,'delta_bits':[list(map(bits,v))for v in deltas],'output_axes_bits':[list(struct.unpack('<2I',u.mem_read(AXES+i*8,8)))for i in range(len(axes))],'changed_ticks':list(struct.unpack('<'+'I'*len(axes),u.mem_read(TICKS,len(axes)*4)))if axes else[],'enable_last':struct.unpack('<Q',u.mem_read(ENABLE_LAST,8))[0],'cursor_last':struct.unpack('<Q',u.mem_read(CURSOR_LAST,8))[0]}
axes_sets=[[],[[.5,.6]],[[.5,.6],[.7,.8]],[[.5,.6],[.7,.8],[-.5,.05]]]
deltas_sets=[[],[[0,0]],[[10,1]],[[1,-10]],[[2,2]],[[float('nan'),1]],[[1,1],[3,20]]]
cases=[case(axes,enable,deltas)for axes in axes_sets for enable in[0,2]for deltas in deltas_sets]
def rerun_one():
 frame=STACK+0x8000
 u.reg_write(UC_X86_REG_RBP,frame);u.reg_write(UC_X86_REG_R15,QUERY);u.reg_write(UC_X86_REG_R14,ENABLE_READER);u.reg_write(UC_X86_REG_RDI,CURSOR_READER);u.emu_start(0x140a931df,0x140a93642,count=100000)
 return {'output_axes_bits':list(struct.unpack('<2I',u.mem_read(AXES,8))),'enable_last':struct.unpack('<Q',u.mem_read(ENABLE_LAST,8))[0],'cursor_last':struct.unpack('<Q',u.mem_read(CURSOR_LAST,8))[0]}
empty=case([],2,[[3,20]])
u.mem_write(TABLES+0x58,struct.pack('<Q',1));u.mem_write(AXES,pack([.5,.6]));u.mem_write(ENTITY_ROWS+8,struct.pack('<I',0));after_spawn=rerun_one()
unread=case([[.5,.6]],0,[[3,20]])
u.mem_write(ENABLE_EVENTS+0x20,struct.pack('<3Q',1,ENABLE_BUFFER,1));u.mem_write(ENABLE_EVENTS+0x40,struct.pack('<Q',1));after_enable=rerun_one()
sequence_cases={'empty_then_spawn':{'first':empty,'after_spawn':after_spawn},'unread_cursor_then_enable':{'first':unread,'after_enable':after_enable}}
(OUT/'native-arrow-fixtures.json').write_text(json.dumps({'binary_sha256':hashlib.sha256(raw).hexdigest(),'range':'0x140a931df..0x140a93642','method':'Original dense query loop, row lookup, both real native EventReader cursors, axis guards/numerics and change-tick writes execute. Matching one-table/one-archetype component column and preallocated event buffers supplied. No business formula bridge, allocator, GPU or target process.','cases':cases,'sequence_cases':sequence_cases},indent=2)+'\n');print({'full_loop_cases':len(cases)})
