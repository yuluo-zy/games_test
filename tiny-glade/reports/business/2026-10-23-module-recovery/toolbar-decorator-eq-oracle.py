from pathlib import Path
import pefile,random,subprocess,json,hashlib,struct
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import UC_X86_REG_RDX,UC_X86_REG_RDI,UC_X86_REG_R11,UC_X86_REG_RBP,UC_X86_REG_RSP
binary=Path('/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne');raw=binary.read_bytes();pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase
uc=Uc(UC_ARCH_X86,UC_MODE_64)
for address,size in [(0x141523000,0x3000),(0x142c6c000,0x1000)]:
 uc.mem_map(address,size);uc.mem_write(address,pe.get_data(address-base,size))
uc.mem_map(0x200000,0x3000);uc.mem_map(0x210000,0x3000);uc.mem_map(0x300000,0x4000)
result=[]
def hook(uc,address,size,data):
 if address in [0x141523bd6,0x141523c0e]:
  result.append(address==0x141523bd6);uc.emu_stop()
uc.hook_add(UC_HOOK_CODE,hook)
rng=random.Random(0xA2C7)
cases=[]
for tag in range(5):
 for run in range(16):
  a=bytearray(rng.randbytes(80));a[0x22]=tag;a[0x4a]=rng.choice([0,0x37,0x38,0x39,0x3a,0x3b,0x3c,0x3d,0x3e,0x3f,0xff])
  # Normal finite scalars first; arbitrary unused union bytes are retained.
  for off in [0,4,8,12,16,20,24,28,48,56,60]:struct.pack_into('<f',a,off,rng.uniform(-10,10))
  cases.append((bytes(a),bytes(a),'identical'))
  for off in range(80):
   b=a.copy();b[off]^=1;cases.append((bytes(a),bytes(b),'flip-'+str(off)))
  for off in [0,4,8,12,16,20,24,28,48,56,60]:
   for bits,other in [(0,0x80000000),(0x7fc12345,0x7fc12345),(0x7f800000,0x7f800000),(0xff800000,0x7f800000)]:
    aa=a.copy();bb=a.copy();struct.pack_into('<I',aa,off,bits);struct.pack_into('<I',bb,off,other);cases.append((bytes(aa),bytes(bb),'float-'+str(off)))
inputs=''.join(a.hex()+' '+b.hex()+'\n' for a,b,_ in cases)
Path('/tmp/newdaily-archive-eq-inputs.txt').write_text(inputs)
candidate=subprocess.run(['/tmp/newdaily-archive-eq-candidate'],input=inputs,text=True,capture_output=True,check=True).stdout.splitlines()
for index,((a,b,label),expected) in enumerate(zip(cases,candidate)):
 uc.mem_write(0x200000,a);uc.mem_write(0x210000,b)
 uc.reg_write(UC_X86_REG_RDX,0x200050);uc.reg_write(UC_X86_REG_RDI,0x210050);uc.reg_write(UC_X86_REG_R11,0)
 uc.reg_write(UC_X86_REG_RBP,0x301000);uc.reg_write(UC_X86_REG_RSP,0x302000)
 result.clear();uc.emu_start(0x141523a60,0x141523f00,count=1000)
 assert len(result)==1,(index,label)
 if result[0] != (expected=='1'):
  mismatch={'index':index,'label':label,'a':a.hex(),'b':b.hex(),'original':result[0],'candidate':expected}
  Path('/tmp/newdaily-archive-eq-first-failure.json').write_text(json.dumps(mismatch,indent=2));raise AssertionError(mismatch)
report={'binary_sha256':hashlib.sha256(raw).hexdigest(),'entry':'0x141523a60','equal_exit':'0x141523bd6','unequal_exit':'0x141523c0e','count':len(cases),'seed':0xA2C7,'bridges':[],'excluded':'HashMap lookup/erase and allocation; resolved valid destination tags 0..4 and matching DecoratorId input','source_sha256':hashlib.sha256(Path('/Users/yuluo/Desktop/game/tiny-glade/crates/country-core/src/resources/walls/decorator_archivist.rs').read_bytes()).hexdigest(),'candidate_sha256':hashlib.sha256(Path('/tmp/newdaily-archive-eq-candidate').read_bytes()).hexdigest(),'inputs_sha256':hashlib.sha256(inputs.encode()).hexdigest()}
Path('/tmp/newdaily-archive-eq-report.json').write_text(json.dumps(report,indent=2));print(json.dumps(report))
