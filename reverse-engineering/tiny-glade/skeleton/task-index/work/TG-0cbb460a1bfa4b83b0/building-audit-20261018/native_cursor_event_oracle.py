"""Execute original B update_cursor_position inline: two distinct vectors and actual event gate/payload."""
import json,struct,hashlib
from pathlib import Path
OUT=Path(__file__).resolve().parent
recipe=Path('/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/shared-foundation-2026-10-13/utils-base/native_spline_rdp.py')
s=recipe.read_text().split('def spline(')[0].replace("Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')", "Path('/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')")
exec(compile(s,str(recipe),'exec'),globals());OUT=Path(__file__).resolve().parent
STATES=INPUT+0x1000;CAPTURE=INPUT+0x2000;CURSOR_ENTRY=INPUT+0x3000;POSITION=INPUT+0x4000;EVENT_ENTRY=INPUT+0x5000;EVENTS=INPUT+0x6000;BUFFER=INPUT+0x7000

def case(name,input_bits,hardware_bits,captured=False,start=0x14173db1f,end=0x14173dba5):
 reset()
 for p in[STATES,CAPTURE,CURSOR_ENTRY,POSITION,EVENT_ENTRY,EVENTS,BUFFER]:u.mem_write(p,bytes(512))
 u.mem_write(STATES+0x140,struct.pack('<4I',*input_bits,*hardware_bits));u.mem_write(CAPTURE,bytes([captured]));u.mem_write(POSITION,pack([50,20]));u.mem_write(EVENTS+0x20,struct.pack('<3Q',16,BUFFER,0));u.mem_write(EVENTS+0x40,struct.pack('<Q',7))
 for r,v in[(UC_X86_REG_R9,STATES),(UC_X86_REG_R11,CAPTURE),(UC_X86_REG_R8,CURSOR_ENTRY),(UC_X86_REG_R10,POSITION),(UC_X86_REG_RAX,EVENT_ENTRY),(UC_X86_REG_RDI,EVENTS),(UC_X86_REG_ESI,77)]:u.reg_write(r,v)
 u.emu_start(start,end,count=10000)
 count=struct.unpack('<Q',u.mem_read(EVENTS+0x30,8))[0]
 return {'name':name,'input_bits':input_bits,'hardware_bits':hardware_bits,'captured':captured,'initial_position_bits':[bits(50),bits(20)],'output_position_bits':list(struct.unpack('<2I',u.mem_read(POSITION,8))),'event_payload_bits':[list(struct.unpack('<2I',u.mem_read(BUFFER+i*16+8,8)))for i in range(count)],'event_count_after':struct.unpack('<Q',u.mem_read(EVENTS+0x40,8))[0],'cursor_changed_tick':struct.unpack('<I',u.mem_read(CURSOR_ENTRY+0x5c,4))[0],'events_changed_tick':struct.unpack('<I',u.mem_read(EVENT_ENTRY+0x5c,4))[0]}
zero=[0,0]
cases=[case('idle',zero,zero),case('raw_differs_from_hardware',[bits(3),bits(20)],[bits(1),bits(-2)]),case('captured_still_emits_raw',[bits(3),bits(20)],[bits(99),bits(99)],True),case('hardware_without_raw',zero,[bits(1),bits(-2)]),case('negative_zero_is_zero',[0x80000000,0x80000000],zero),case('nan_x_is_nonzero',[0x7fc01234,bits(1)],zero),case('infinities_are_nonzero',[0x7f800000,0xff800000],zero),case('subnormals_are_nonzero',[1,0x80000001],zero),case('nan_y_is_nonzero',[0,0x7fc00000],zero),case('negative_raw',[bits(-3),bits(-20)],[bits(-1),bits(2)])]
other_cases=[case(c['name'],c['input_bits'],c['hardware_bits'],c['captured'],0x1414fbcc2,0x1414fbd48)for c in cases]
assert other_cases==cases
(OUT/'native-cursor-event-fixtures.json').write_text(json.dumps({'target':'B','binary_sha256':hashlib.sha256(raw).hexdigest(),'ranges':['0x14173db1f..0x14173dba5','0x1414fbcc2..0x1414fbd48'],'second_inline_results_equal':other_cases==cases,'method':'Original inline instructions execute position update, raw-vector nonzero gate and actual native EventWriter push against resolved resource pointers and preallocated capacity; no arithmetic/event bridge or allocator.','cases':cases},indent=2)+'\n')
print({'cases':len(cases),'original_inline_executions':len(cases)+len(other_cases),'emitted_payloads':[c['event_payload_bits']for c in cases]})
