"""Original B TerrainRockMask mapped constructor with only graphics API boundaries supplied."""
from pathlib import Path
import json,struct,hashlib
ROOT=Path('/Users/yuluo/Desktop/game');OUT=Path(__file__).resolve().parent
recipe=ROOT/'games_test/reverse-engineering/tiny-glade/evidence/shared-foundation-2026-10-13/utils-base/native_spline_rdp.py'
s=recipe.read_text().split('def spline(')[0].replace("Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')", "Path('/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')")
exec(compile(s,str(recipe),'exec'),globals());OUT=Path(__file__).resolve().parent
RESULT=INPUT+0x1000;DEVICE=INPUT+0x2000;INITIAL=INPUT+0x3000;MAPPING=HEAP+0x800000
calls=[];panic_target=None

def boundaries(uc,address,size,data):
 global panic_target
 if address==0x140bd4de0:
  calls.append({'kind':'descriptor','byte_size':u.reg_read(UC_X86_REG_RDX),'usage':u.reg_read(UC_X86_REG_R8),'memory':u.reg_read(UC_X86_REG_R9)&255})
 elif address==0x140bd4f20:calls.append({'kind':'alignment','value':u.reg_read(UC_X86_REG_R8)})
 elif address==0x1416a2dc0:
  sp=u.reg_read(UC_X86_REG_RSP);target=u.reg_read(UC_X86_REG_RCX);data_ptr,n=struct.unpack('<2Q',u.mem_read(sp+0x30,16));name_n=u.reg_read(UC_X86_REG_R9);payload=bytes(u.mem_read(data_ptr,n))
  calls.append({'kind':'create_buffer','name':bytes(u.mem_read(u.reg_read(UC_X86_REG_R8),name_n)).decode(),'initial_bytes':n,'initial_sha256':hashlib.sha256(payload).hexdigest()})
  u.mem_write(MAPPING,payload);u.mem_write(target,struct.pack('<Q',11)+struct.pack('<5Q',7,1,0,0x8000,0));ret(target)
 elif address==0x140bd4cd0:ret(MAPPING);u.reg_write(UC_X86_REG_RDX,0x8000)
 elif address in[0x1428d9390,0x1428d9430,0x1428d9596]:panic_target=hex(address);u.emu_stop()
u.hook_add(UC_HOOK_CODE,boundaries)
def case(name,initial):
 global calls,panic_target
 reset();u.mem_write(INPUT,bytes(0x10000));u.mem_write(MAPPING,bytes(0x8000));calls=[];panic_target=None
 if initial is not None:u.mem_write(INITIAL,initial)
 for r,v in[(UC_X86_REG_RCX,RESULT),(UC_X86_REG_RDX,DEVICE),(UC_X86_REG_R8,0 if initial is None else INITIAL),(UC_X86_REG_R9,0 if initial is None else len(initial))]:u.reg_write(r,v)
 u.emu_start(0x141620320,STOP,count=1000000)
 return {'name':name,'initial_size':None if initial is None else len(initial),'calls':calls.copy(),'mapped_words':None if panic_target else struct.unpack('<Q',u.mem_read(RESULT+8,8))[0],'buffer_bytes':None if panic_target else struct.unpack('<Q',u.mem_read(RESULT+0x28,8))[0],'panic':panic_target}
cases=[case('none_zero_initial',None),case('one_u32',struct.pack('<I',0x12345678)),case('full_native_capacity',bytes(0x8000)),case('excessive_one_word',bytes(0x8004)),case('misaligned_byte_count',bytes(3))]
for c in cases[:3]:
 assert c['mapped_words']==0x2000 and c['buffer_bytes']==0x8000 and c['panic']is None
 assert c['calls'][0]=={'kind':'descriptor','byte_size':0x8000,'usage':0x22,'memory':4}
 assert c['calls'][1]=={'kind':'alignment','value':4};assert c['calls'][2]['name']=='TerrainRockMask'
assert all(c['panic']and not c['calls']for c in cases[3:])
report={'target':'B','binary_sha256':hashlib.sha256(raw).hexdigest(),'entry':'0x141620320','cases':cases,'method':'Original constructor executes allocation/default fill, initial-size guards, BufferDesc construction, mapped byte/word conversion and return; graphics create/map are supplied boundaries; native allocator/CRT use shared harness.'}
(OUT/'native-rock-mask-constructor.json').write_text(json.dumps(report,indent=2)+'\n');print(report)
