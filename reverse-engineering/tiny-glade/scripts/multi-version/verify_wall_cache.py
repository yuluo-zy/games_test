"""Run each original regular-wall cache producer across A/B/C, preallocated Vec
storage supplied at dependency boundary. No game calculation is stubbed."""
import json,hashlib,random,struct,subprocess
from pathlib import Path
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import *
from verification_provenance import ROOT,candidate_provenance

def bits(f):return struct.unpack('<I',struct.pack('<f',f))[0]
def main():
 baselines=json.loads((ROOT/'tracking/baselines.json').read_text())['baselines'];rows=[];fail=[];rng=random.Random(20261011)
 specs={'A':(0x1404fb3f4,0x1404fb6c9,0x288,0x270,UC_X86_REG_RDI,[(UC_X86_REG_XMM6,0xc0200000),(UC_X86_REG_XMM8,0xbeeb851f),(UC_X86_REG_XMM9,0x3f75c290)]),
 'B':(0x14078b865,0x14078bc96,0x2a0,0x284,UC_X86_REG_RBX,[(UC_X86_REG_XMM8,0xc0200000),(UC_X86_REG_XMM9,0xbeeb851f),(UC_X86_REG_XMM10,0x3f75c290)]),
 'C':(0x1408401e7,0x1408405b0,0x2a8,0x288,UC_X86_REG_RDI,[(UC_X86_REG_XMM9,0xc0200000),(UC_X86_REG_XMM10,0xbeeb851f),(UC_X86_REG_XMM11,0x3f75c290)])}
 for version,b in baselines.items():
  raw=Path(b['binary']['path']).read_bytes();assert hashlib.sha256(raw).hexdigest()==b['binary']['sha256'];pe=b['pe'];base=pe['image_base']
  cpu=Uc(UC_ARCH_X86,UC_MODE_64);end=max(s['rva']+max(s['raw_size'],s['virtual_size']) for s in pe['sections']);cpu.mem_map(base,(end+4095)&~4095)
  for s in pe['sections']:
   if s['raw_size']:cpu.mem_write(base+s['rva'],raw[s['raw_pointer']:s['raw_pointer']+s['raw_size']])
  data,stack=0x20000000,0x30000000;cpu.mem_map(data,0x10000);cpu.mem_map(stack,0x10000)
  start,stop,size,extra_offset,record_reg,xmms=specs[version];lines=[];expected=[]
  edges=[0,0x80000000,0x7f800000,0xff800000,0x7fc01234,0xffc01234,0x7f800001,0xc0200000,0x3f75c28f,0x3f75c290,0x3f75c291]
  for index in range(1024):
   n=[0,1,2,3,7,8,9,15,16,17,31][index%11]
   values=[rng.choice(edges) if index<200 else bits(rng.uniform(-4,20)) for _ in range(n)]
   extra=rng.choice(edges) if index<200 else bits(rng.uniform(.3,14))
   positions=[(bits(float(i)),v,bits(-float(i))) for i,v in enumerate(values)]
   record=bytearray([0xa5]*size);struct.pack_into('<QQ',record,0x90,data+0x1000,n);struct.pack_into('<QQQ',record,0xd8,64,data+0x2000,19);struct.pack_into('<I',record,extra_offset,extra)
   cpu.mem_write(data,bytes(record));cpu.mem_write(data+0x1000,b''.join(struct.pack('<III',*p) for p in positions));cpu.mem_write(data+0x2000,bytes(256));cpu.mem_write(stack,bytes(0x10000))
   for r in [UC_X86_REG_RAX,UC_X86_REG_RBX,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_RDI,UC_X86_REG_RSI,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,UC_X86_REG_R13,UC_X86_REG_R14,UC_X86_REG_R15]:cpu.reg_write(r,0)
   cpu.reg_write(record_reg,data+size);cpu.reg_write(UC_X86_REG_RBP,stack+0x8000);cpu.reg_write(UC_X86_REG_RSP,stack+0x7000);cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
   for r,v in xmms:cpu.reg_write(r,v)
   cpu.emu_start(start,stop,count=10000);assert cpu.reg_read(UC_X86_REG_RIP)==stop
   updated=bytes(cpu.mem_read(data,size));assert struct.unpack_from('<I',updated,0x108)[0]==1 and struct.unpack_from('<I',updated,0x110)[0]==1
   max_bits=struct.unpack_from('<I',updated,0x10c)[0];assert struct.unpack_from('<I',updated,0x114)[0]==max_bits
   assert struct.unpack_from('<Q',updated,0xe8)[0]==n
   arch=struct.unpack('<'+'I'*n,cpu.mem_read(data+0x2000,n*4)) if n else []
   shape=struct.pack('<I',index%2)+bytes(24)
   lines.append('cache '+shape.hex()+f' {extra:08x} '+ ' '.join(f'{v:08x}' for p in positions for v in p))
   expected.append(f'{max_bits:08x} '+','.join(f'{v:08x}' for v in arch))
  result=subprocess.run([str(ROOT/'reconstruction/target/release/lifecycle-probe')],input='\n'.join(lines)+'\n',capture_output=True,text=True,check=True).stdout.splitlines();assert len(result)==len(expected)
  for i,(got,want) in enumerate(zip(result,expected)):
   if got.rstrip()!=want.rstrip():fail.append({'version':version,'case':i,'original':want,'rust':got})
  rows.append({'baseline':version,'binary_sha256':b['binary']['sha256'],'entry':hex(start),'stop':hex(stop),'cases':len(expected),'public_wall_size':size,'extra_offset':hex(extra_offset),'flags':[hex(0x108),hex(0x110)]})
 report={'ok':not fail,'versions':rows,'case_count':3072,'failure_count':len(fail),'failures':fail[:20],
 'scope':'Circle/rectangle base maximum, both cache flags/values, arch height array/length from original instruction ranges versus Rust. Not gable arch profile, publication gathering or entire scheduler.',
 'dependency_boundary':'Existing standard Vec storage is preallocated; no allocator/Vec implementation or game core stub.',**candidate_provenance()}
 (ROOT/'evidence/building-runtime/three-version-cache-verification.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'ok':not fail,'cases':3072,'failures':len(fail)}));assert not fail
if __name__=='__main__':main()
