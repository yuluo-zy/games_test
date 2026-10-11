"""B StoneFloor CPU uploads captured at the original GPU-writer boundary.

No query/Bevy/atlas/renderer implementation is reconstructed. Circle Windows
CRT sin/cos/fmod are explicit dependency bridges to host libm; their outcomes
are separately scoped from the unbridged rectangle game instructions.
"""
import ctypes,json,random,struct,subprocess,hashlib,os
from verify_parameters import cpu,DATA,STACK,ROOT,baseline,bits,ret
from verify_building_state import reset
from verification_provenance import candidate_provenance
from unicorn import UC_HOOK_CODE
from unicorn.x86_const import *
lib=ctypes.CDLL(None)
for n in ['sinf','cosf','fmodf']:
 f=getattr(lib,n);f.restype=ctypes.c_float;f.argtypes=[ctypes.c_float]*(2 if n=='fmodf' else 1)
bridges={0x142923f10:'fmodf',0x142923e80:'cosf',0x142923fd0:'sinf'}
captured=None
def float_xmm(c,n):return struct.unpack('<f',struct.pack('<I',c.reg_read(n)&0xffffffff))[0]
def hook(c,pc,size,data):
 global captured
 if pc==0x141570690:
  ptr=struct.unpack('<Q',c.mem_read(c.reg_read(UC_X86_REG_RSP)+0x28,8))[0]
  record=bytes(c.mem_read(ptr,64))
  center=bytes(c.mem_read(c.reg_read(UC_X86_REG_R8),12))
  radius=(c.reg_read(UC_X86_REG_XMM3)&0xffffffff).to_bytes(4,'little')
  captured=record.hex()+' '+(center+radius).hex();c.emu_stop()
 elif pc in bridges:
  name=bridges[pc];args=[float_xmm(c,UC_X86_REG_XMM0)]
  if name=='fmodf':args.append(float_xmm(c,UC_X86_REG_XMM1))
  c.reg_write(UC_X86_REG_XMM0,bits(getattr(lib,name)(*args)));ret(c)
cpu.hook_add(UC_HOOK_CODE,hook)
def original(roof,alpha):
 global captured
 captured=None;reset();sp=STACK+0x8000;frame=sp+0x80
 cpu.mem_write(sp,bytes(0x400));cpu.mem_write(DATA,roof)
 cpu.mem_write(DATA+0x1000,bytes(48));cpu.mem_write(DATA+0x2000,bytes(16))
 for reg,val in [(UC_X86_REG_RSP,sp),(UC_X86_REG_RBP,frame),
  (UC_X86_REG_R14,DATA),(UC_X86_REG_RBX,DATA+0x1000),(UC_X86_REG_RDX,DATA+0x2000)]:cpu.reg_write(reg,val)
 for reg,val in [(UC_X86_REG_XMM6,bits(1)),(UC_X86_REG_XMM7,bits(1)|(bits(1)<<32)),
  (UC_X86_REG_XMM8,int.from_bytes(bytes.fromhex('ffffff7f'*4),'little')),
  (UC_X86_REG_XMM9,bits(.1)),(UC_X86_REG_XMM11,bits(.5)),(UC_X86_REG_XMM12,bits(.63)),
  (UC_X86_REG_XMM15,alpha)]:cpu.reg_write(reg,val)
 # Enter after resolved optional-animation lookup, preserving its original first channel.
 cpu.emu_start(0x142078dc4,0,count=2000);assert captured is not None
 return captured

def main():
 rng=random.Random(20261011);cases=[]
 for shape in [1,0]:
  for i in range(512):
   roof=bytearray(rng.randbytes(88));struct.pack_into('<I',roof,8,shape)
   for off in range(12,0x40,4):struct.pack_into('<f',roof,off,rng.uniform(-8,8))
   struct.pack_into('<f',roof,0x14,rng.uniform(0.1,15))
   struct.pack_into('<f',roof,0x1c,rng.uniform(0.1,15));struct.pack_into('<f',roof,0x20,rng.uniform(0.1,15))
   struct.pack_into('<f',roof,0x30,[0,.1,.100001,2][i%4]);struct.pack_into('<f',roof,0x34,[0,.5,1][i%3])
   roof[0x3c]=i%2;roof[0x40]=i%3;roof[0x41]=i%2;roof[0x54]=(i//2)%2
   struct.pack_into('<f',roof,0x4c,rng.uniform(.96,20));alpha=bits(rng.uniform(0,1))
   cases.append((bytes(roof),alpha))
 probe=os.environ.get('TG_NANI_PROBE',str(ROOT/'reconstruction/target/release/nani-probe'))
 candidate=subprocess.run([str(probe)],input=''.join(f'{r.hex()} {a:08x}\n' for r,a in cases),text=True,capture_output=True,check=True).stdout.splitlines()
 assert len(candidate)==len(cases)
 failures=[]
 for i,((r,a),actual) in enumerate(zip(cases,candidate)):
  expected=original(r,a)
  if actual!=expected:failures.append({'case':i,'shape':struct.unpack_from('<I',r,8)[0],'original':expected,'rust':actual,'roof':r.hex(),'animation':hex(a)})
 report={'ok':not failures,'cases':len(cases),'rectangle_cases':512,'circle_cases_with_host_libm_bridge':512,
  'source_sha256':baseline['binary']['sha256'],'entry':'0x142078dc4','captured_writer':'0x141570690',
  'scope':'resolved Roof branch CPU upload64 and bounds; original x86 game instructions; no Floor/atlas/ECS/GPU allocation',
  'probe_path':str(probe),'probe_sha256':hashlib.sha256(open(probe,'rb').read()).hexdigest(),
  'candidate_build':'isolated rustc projection source extracted unchanged from nani.rs' if os.environ.get('TG_NANI_PROBE') else 'workspace release nani-probe',
  'module_source_sha256':hashlib.sha256((ROOT/'reconstruction/crates/tg-building-runtime/src/nani.rs').read_bytes()).hexdigest(),
  'bridges':bridges,'failures':failures}
 report.update(candidate_provenance())
 (ROOT/'evidence/building-nani/B-cpu-verification.json').write_text(json.dumps(report,indent=2)+'\n')
 print(json.dumps({'ok':report['ok'],'cases':len(cases),'failures':len(failures)}))
 if failures:print(json.dumps(failures[:2]));raise SystemExit(1)
if __name__=='__main__':main()
