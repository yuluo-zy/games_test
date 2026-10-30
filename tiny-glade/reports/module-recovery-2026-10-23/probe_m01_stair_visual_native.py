import pefile,struct,json,pathlib
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
# Whole artifact identity is independently bound by the adjacent native evidence report.
p=pefile.PE('/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne',fast_load=True)
def data(va,n):return p.get_data(va-0x140000000,n)
def f32(i):return struct.unpack('<f',struct.pack('<I',i))[0]
def bits(f):return struct.unpack('<I',struct.pack('<f',f))[0]
# (straight,turn,flags,outgoing), real outgoing graph cases only.
cases=[(0,0,0,0),(1,0,0,1),(1,0,1,1),(0,1,0,1),(0,1,1,1),(0,0,0,1),(0,1,0,2),(1,1,0,2)]
rows=[]
for case,(straight,turn,flags,count) in enumerate(cases):
 for width_bits in [0x3f800000,0x3f99999a,0x402ccccd,0x80000000,0xc0000000,0x42c80000]:
  u=Uc(UC_ARCH_X86,UC_MODE_64)
  u.mem_map(0x140803000,0x4000);u.mem_write(0x140803000,data(0x140803000,0x4000))
  u.mem_map(0x142a8e000,0x1000);u.mem_write(0x142a8e000,data(0x142a8e000,0x1000))
  u.mem_map(0x142925000,0x1000);u.mem_write(0x142925000,data(0x142925000,0x1000))
  u.mem_map(0x142a7d000,0x3000);u.mem_write(0x142a7d000,data(0x142a7d000,0x3000))
  u.mem_map(0x200000,0x10000);u.reg_write(UC_X86_REG_RBP,0x208000);u.reg_write(UC_X86_REG_RSP,0x207000)
  u.mem_write(0x20835c,struct.pack('<I',straight));u.mem_write(0x2083a0,struct.pack('<I',turn))
  u.reg_write(UC_X86_REG_R14,count);u.reg_write(UC_X86_REG_R15,flags)
  u.reg_write(UC_X86_REG_XMM9,width_bits);u.reg_write(UC_X86_REG_XMM12,0x3f000000)
  panic=[False]
  def stop(mu,a,n,unused):
   if a in [0x140804960,0x14080553b]:panic[0]=a==0x14080553b;mu.emu_stop()
  h=u.hook_add(UC_HOOK_CODE,stop);u.emu_start(0x1408048f0,0,count=1000);u.hook_del(h)
  if panic[0]:rows.append({'case':case,'width':width_bits,'panic':True});continue
  kind=u.reg_read(UC_X86_REG_RAX)&255;offset=u.reg_read(UC_X86_REG_XMM15)&0xffffffff;hole=u.reg_read(UC_X86_REG_R15)&255
  # Original width+0.1 is computed before shape switch, rounding once in f32.
  expanded=bits(f32(width_bits)+f32(0x3dcccccd));u.reg_write(UC_X86_REG_XMM11,expanded);u.reg_write(UC_X86_REG_XMM10,0x3dcccccd)
  u.emu_start(0x140804a7b,0x140804b00,count=1000)
  dims=struct.unpack('<III',u.mem_read(0x208180,12))
  rows.append({'case':case,'width':width_bits,'panic':False,'kind':kind,'offset':offset,'hole':hole,'dimensions':dims})
out=pathlib.Path('/Users/yuluo/Desktop/game/tiny-glade/reports/module-recovery-2026-10-23')
(out/'m01-stair-visual-native-vectors.json').write_text(json.dumps({'target':'original B RNE','executed':'bounded native profile8048f0..4960 and shape switch804a7b..4b00 only','rows':rows},indent=2))
lines=['pub const STAIR_VISUAL_NATIVE: &[(u8,u32,u8,u32,bool,[u32;3])] = &[']
for r in rows:
 if r['panic']:continue
 lines.append(f"    ({r['case']},0x{r['width']:08x},{r['kind']},0x{r['offset']:08x},{'true' if r['hole'] else 'false'},[{','.join(hex(v) for v in r['dimensions'])}]),")
lines.append('];');(out/'m01-stair-visual-native-vectors.rs').write_text('\n'.join(lines)+'\n')
print(json.dumps({'rows':len(rows),'panic_rows':sum(r['panic'] for r in rows),'normal_rows':sum(not r['panic'] for r in rows)}))
