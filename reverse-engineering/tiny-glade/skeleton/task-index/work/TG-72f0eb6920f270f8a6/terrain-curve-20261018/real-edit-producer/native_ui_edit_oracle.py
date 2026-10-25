"""Original B UI-edit dispatch and case2 parameter-command instructions."""
import json,struct,hashlib
from pathlib import Path
OUT=Path(__file__).resolve().parent
recipe=Path('/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/shared-foundation-2026-10-13/utils-base/native_spline_rdp.py')
s=recipe.read_text().split('def spline(')[0].replace("Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')", "Path('/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')")
exec(compile(s,str(recipe),'exec'),globals());OUT=Path(__file__).resolve().parent
mode='';jump=None
TARGETS={0x140aef71a:0,0x140aef9c5:1,0x140aefa80:2,0x140aefbc0:3}
def bridge(uc,address,size,data):
 global jump
 if mode=='dispatch' and address in TARGETS:jump=TARGETS[address];uc.emu_stop()
 elif mode=='case2' and address==0x140aef71a:uc.emu_stop()
 elif address==0x1405b0ed0:ret(0)
u.hook_add(UC_HOOK_CODE,bridge)
def dispatch(subtype):
 global mode,jump
 reset();mode='dispatch';jump=None;u.reg_write(UC_X86_REG_RAX,subtype);u.emu_start(0x140aef9a3,STOP,count=100);assert jump is not None
 return jump
TERRAIN=INPUT+0x4000;CONTROL=INPUT+0x5000;BUCKET=CONTROL-48;POS=INPUT+0x6000;ACCUM=INPUT+0x7000;TICK=INPUT+0x8000;UNIT=INPUT+0x9000;EDIT=INPUT+0xa000;FOCUS=INPUT+0xb000;UNITBUF=INPUT+0xc000;EDITBUF=INPUT+0xd000;FOCUSBUF=INPUT+0xe000
REGS=[UC_X86_REG_XMM12,UC_X86_REG_XMM13,UC_X86_REG_XMM14]
def case(dx,dy,height):
 global mode
 reset();mode='case2';frame=STACK+0x10008
 for p in [TERRAIN,CONTROL,POS,ACCUM,TICK,UNIT,EDIT,FOCUS,UNITBUF,EDITBUF,FOCUSBUF]:u.mem_write(p,bytes(512))
 u.mem_write(BUCKET,bytes(48));u.mem_write(BUCKET,struct.pack('<I',17));u.mem_write(BUCKET+40,pack([height]));u.mem_write(CONTROL,bytes([0])+bytes([255])*31);u.mem_write(TERRAIN+8,struct.pack('<Q',CONTROL));u.mem_write(TERRAIN+0x20,struct.pack('<Q',1));u.mem_write(POS,pack([1,2,3]))
 for q,b in [(UNIT,UNITBUF),(EDIT,EDITBUF),(FOCUS,FOCUSBUF)]:u.mem_write(q+0x20,struct.pack('<3Q',32,b,0));u.mem_write(q+0x40,struct.pack('<Q',77))
 for offset,value in [(0x1e0,TICK),(0xd0,UNIT),(0x48,EDIT),(0x98,TICK),(0x68,POS),(0xc8,ACCUM),(0x30,FOCUS),(0xb8,TICK),(0x60,TERRAIN),(0x1a0,TERRAIN+0x28)]:u.mem_write(frame+offset,struct.pack('<Q',value))
 u.reg_write(UC_X86_REG_XMM6,int.from_bytes(pack([dx,dy])+bytes(8),'little'));u.emu_start(0x140aef4d4,0x140aef51f,count=100)
 for r,f in zip(REGS,[0,.3,20]):u.reg_write(r,bits(f))
 u.reg_write(UC_X86_REG_RDI,1);u.reg_write(UC_X86_REG_RBP,BUCKET);u.emu_start(0x140aefa80,STOP,count=10000)
 count=struct.unpack('<Q',u.mem_read(EDIT+0x30,8))[0];data=bytes(u.mem_read(EDITBUF,40));out={'dx_bits':bits(dx),'dy_bits':bits(dy),'height_bits':bits(height),'command_count':count,'unit_count':struct.unpack('<Q',u.mem_read(UNIT+0x30,8))[0]}
 if count:
  ft=struct.unpack_from('<I',data,0)[0];ht=struct.unpack_from('<I',data,8)[0];out.update({'falloff_tag':ft,'falloff_bits':struct.unpack_from('<I',data,4)[0] if ft else None,'height_tag':ht,'height_delta_bits':struct.unpack_from('<I',data,12)[0] if ht else None,'position_bits':list(struct.unpack_from('<3I',data,16)),'id':struct.unpack_from('<I',data,28)[0]})
 return out
jumps=[dispatch(n)for n in range(256)]
cases=[case(dx,dy,h)for dx,dy in [(0,0),(10,1),(-10,1),(1,10),(1,-10),(2,2)]for h in [0,2.5,20]]
(OUT/'native-ui-edit-fixtures.json').write_text(json.dumps({'binary_sha256':hashlib.sha256(raw).hexdigest(),'dispatch_range':'0x140aef9a3..jump targets','case2_ranges':['0x140aef4d4..0x140aef51f','0x140aefa80..0x140aef71a'],'method':'Execute original dispatch, delta preprocessing and case2 instructions; resolved valid single stroke map, standard hash bridged and native event queues preallocated. Every command payload write and height arithmetic is original; no window/GPU/allocator-failure execution.','dispatch_all_256':jumps,'cases':cases},indent=2)+'\n')
print({'dispatch_cases':len(jumps),'parameter_cases':len(cases)})
