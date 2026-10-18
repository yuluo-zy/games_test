"""Original RMB generated-ID read, std HashSet insert boundary, real History start/add.
The HashSet primitive is observed/bridged as a dependency; deletion membership
implementation is not claimed. No game History leaf is bridged. Skips rotation,
mesh conversion and creation writer, already audited separately.
"""
from pathlib import Path
HERE=Path(__file__).resolve().parent
source=(HERE/'history-harness-prefix.py').read_text()
exec(compile(source.split('# Obtain structurally valid empty auxiliary owners')[0],'existing-native-history-initializer','exec'),globals());OUT=HERE
DELETED=INPUT+0xe000;AUTO=INPUT+0xe100
inserts=[]
def primitive(uc,a,size,data):
 if a==0x1402ca2f0:
  inserts.append({'set':u.reg_read(UC_X86_REG_RCX),'id':u.reg_read(UC_X86_REG_RDX)});ret(1)
u.hook_add(UC_HOOK_CODE,primitive)
cases=[]
for generated in[0,73,0x1234567887654321,0xffffffffffffffff]:
 sp=seed(False,0)-8;u.reg_write(UC_X86_REG_RSP,sp);inserts=[];u.mem_write(AUTO,struct.pack('<Q',generated));u.reg_write(UC_X86_REG_R15,AUTO);u.reg_write(UC_X86_REG_R12,DELETED);u.reg_write(UC_X86_REG_XMM0,0)
 u.emu_start(0x140977060,0x140977071,count=1000);assert inserts==[{'set':DELETED,'id':generated}]
 u.mem_write(MODE,struct.pack('<I',0));u.reg_write(UC_X86_REG_RBX,HISTORY);u.reg_write(UC_X86_REG_R14,param(13,MODE))
 u.emu_start(0x140977120,0x14097717b,count=100000)
 cap,ptr,n=struct.unpack('<3Q',u.mem_read(HISTORY+0x30,24));tag,arc=struct.unpack('<2Q',u.mem_read(ptr,16));refs1,refs2,actual=struct.unpack('<3Q',u.mem_read(arc,24))
 assert n==1 and tag==13 and actual==generated and refs1==1 and refs2==1,(cap,ptr,n,tag,arc,actual)
 cases.append({'generated_id':generated,'inserted_id':inserts[0]['id'],'native_calls':observed['calls'],'pending_count':n,'edit_tag':tag,'arc_strong':refs1,'arc_weak':refs2,'history_id':actual})
(HERE/'native-clutter-id-fixtures.json').write_text(json.dumps({'binary_sha256':hashlib.sha256(raw).hexdigest(),'method':__doc__,'ranges':[['0x140977060','0x140977071'],['0x140977120','0x14097717b']],'cases':cases},indent=2)+'\n');print('cases',len(cases))
