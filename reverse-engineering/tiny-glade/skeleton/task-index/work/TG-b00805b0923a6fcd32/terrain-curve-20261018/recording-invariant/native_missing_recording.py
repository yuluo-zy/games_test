"""Original History::add missing-pending invariant path, no source formula bridge."""
from pathlib import Path
import json,struct,hashlib
ROOT=Path('/Users/yuluo/Desktop/game');OUT=Path(__file__).resolve().parent
recipe=ROOT/'games_test/reverse-engineering/tiny-glade/evidence/shared-foundation-2026-10-13/utils-base/native_spline_rdp.py'
s=recipe.read_text().split('def spline(')[0].replace("Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')", "Path('/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')")
exec(compile(s,str(recipe),'exec'),globals());OUT=Path(__file__).resolve().parent
HISTORY=INPUT+0x1000;EDIT=INPUT+0x2000;LOCATION=INPUT+0x3000
seen=[]
def panic(uc,addr,size,data):
 if addr==0x1428d9390:
  seen.append({'target':hex(addr),'return_address':hex(struct.unpack('<Q',u.mem_read(u.reg_read(UC_X86_REG_RSP),8))[0])});u.emu_stop()
u.hook_add(UC_HOOK_CODE,panic)
reset();u.mem_write(INPUT,bytes(0x5000));u.mem_write(HISTORY+0x30,struct.pack('<Q',0x8000000000000000))
for r,v in[(UC_X86_REG_RCX,HISTORY),(UC_X86_REG_RDX,EDIT),(UC_X86_REG_R8,LOCATION)]:u.reg_write(r,v)
u.emu_start(0x140a84ec0,0x140a852d5,count=10000)
assert seen==[{'target':'0x1428d9390','return_address':'0x140a852a5'}]
report={'target':'B','binary_sha256':hashlib.sha256(raw).hexdigest(),'entry':'0x140a84ec0','pending_niche':'0x8000000000000000','observed_calls':seen,'method':__doc__,'panic_boundary':'Stopped at actual core::panicking::panic_fmt entry; no panic formatting/runtime executed.'}
(OUT/'native-missing-recording.json').write_text(json.dumps(report,indent=2)+'\n');print(report)
