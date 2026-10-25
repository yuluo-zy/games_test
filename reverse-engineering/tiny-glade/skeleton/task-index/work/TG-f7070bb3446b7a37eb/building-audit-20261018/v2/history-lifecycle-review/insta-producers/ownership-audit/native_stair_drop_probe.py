"""Original typed Stair Events drop; poison each element to detect any nested ownership read."""
from pathlib import Path
import struct,json,hashlib,gzip
ROOT=Path('/Users/yuluo/Desktop/game');HERE=Path(__file__).resolve().parent
recipe=(ROOT/'games_test/reverse-engineering/tiny-glade/evidence/shared-foundation-2026-10-13/utils-base/native_spline_rdp.py').read_text().split('def spline(')[0]
recipe=recipe.replace('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne',str(ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne'))
recipe=recipe.replace('elif address==0x140613c20:ret()',"elif address==0x140613c20:\n        deallocations.append([u.reg_read(r) for r in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8]]);ret()")
exec(compile(recipe,'shared-native-memory-harness','exec'),globals());OUT=HERE
from capstone import Cs,CS_ARCH_X86,CS_MODE_64
start=0x14014aec0;code=pe.get_data(start-base,75)
(HERE/'original-stair-events-drop.asm').write_text('\n'.join(f'{i.address:x}: {i.bytes.hex():28} {i.mnemonic} {i.op_str}'for i in Cs(CS_ARCH_X86,CS_MODE_64).disasm(code,start))+'\n')
alias_file=ROOT/'games_test/reverse-engineering/tiny-glade/skeleton/_pdb/B/enrichment/address-aliases.json.gz'
alias=json.loads(gzip.decompress(alias_file.read_bytes()))[str(start-base)]
(HERE/'typed-drop-aliases.json').write_text(json.dumps({'source':str(alias_file),'source_sha256':hashlib.sha256(alias_file.read_bytes()).hexdigest(),'aliases':alias},indent=2)+'\n')
cases=[]
for cap_a,cap_b in [(0,0),(1,0),(0,2),(1,2),(3,4)]:
 for poison in [0,255]:
  for fill in [False,True]:
   sp=reset();deallocations=[];a=INPUT+0x4000;b=INPUT+0x8000
   u.mem_write(a,bytes([poison])*0x1000);u.mem_write(b,bytes([poison])*0x1000)
   u.mem_write(INPUT,struct.pack('<9Q',cap_a,a,cap_a if fill else 0,0,cap_b,b,cap_b if fill else 0,0,99));u.reg_write(UC_X86_REG_RCX,INPUT);run(start)
   expected=([[a,cap_a*120,8]]if cap_a else [])+([[b,cap_b*120,8]]if cap_b else [])
   assert deallocations==expected
   cases.append({'capacity':[cap_a,cap_b],'populated':fill,'poison':poison,'deallocations':deallocations})
(HERE/'native-stair-drop-fixtures.json').write_text(json.dumps({'binary_sha256':hashlib.sha256(raw).hexdigest(),'entry':hex(start),'method':__doc__,'cases':cases},indent=2)+'\n')
print('cases',len(cases))
