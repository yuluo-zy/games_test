from pathlib import Path
import pefile,struct,json,random
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import *
# Whole artifact identity is independently bound by the adjacent native evidence report.
p=pefile.PE('/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne',fast_load=True)
def b(f):return struct.unpack('<I',struct.pack('<f',f))[0]
rng=random.Random(0x7be730)
quats=[[b(0.),b(0.),b(0.),b(1.)],[b(-0.),b(0.),b(-0.),b(1.)],[b(0.),b(0.),b(0.),b(0.)]]+[[b(rng.uniform(-3,3)) for _ in range(4)] for _ in range(61)]
rows=[]
for q in quats:
 u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(0x1407bf000,0x1000);u.mem_write(0x1407bf000,p.get_data(0x7bf000,0x1000))
 def reg(r,words):u.reg_write(r,int.from_bytes(struct.pack('<4I',*words),'little'))
 reg(UC_X86_REG_XMM15,q);reg(UC_X86_REG_XMM11,[0,0,b(1.),0]);reg(UC_X86_REG_XMM12,[0,0,b(1.),0]);reg(UC_X86_REG_XMM13,[0]*4);reg(UC_X86_REG_XMM14,[b(1.),b(1.),0,0])
 u.emu_start(0x1407bf7a9,0x1407bf830,count=1000)
 out=struct.unpack('<4I',u.reg_read(UC_X86_REG_XMM15).to_bytes(16,'little'));rows.append((q,[out[0],out[2]]))
r=Path('/Users/yuluo/Desktop/game/tiny-glade/reports/module-recovery-2026-10-23')
(r/'m01-trap-forward-native-vectors.json').write_text(json.dumps({'original_B_range':'1407bf7a9..1407bf830','scope':'only native rotate-vector result x/z, no injected business callee','rows':rows},indent=2))
lines=['pub const TRAP_FORWARD_NATIVE: &[([u32;4],[u32;2])] = &[']
for q,out in rows:lines.append('    (['+','.join(hex(v) for v in q)+'],['+','.join(hex(v) for v in out)+']),')
lines.append('];');(r/'m01-trap-forward-native-vectors.rs').write_text('\n'.join(lines)+'\n');print(json.dumps({'vectors':len(rows)}))
