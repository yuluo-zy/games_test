"""A/B/C game terrain sampler against the raw-collection grid adapter."""
import hashlib,json,random,struct,subprocess
from pathlib import Path
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import *
from verification_provenance import ROOT,candidate_provenance
def bits(f):return struct.unpack('<I',struct.pack('<f',f))[0]
def main():
 bs=json.loads((ROOT/'tracking/baselines.json').read_text())['baselines'];fail=[];versions=[];rng=random.Random(20261012)
 for version,b in bs.items():
  raw=Path(b['binary']['path']).read_bytes();assert hashlib.sha256(raw).hexdigest()==b['binary']['sha256'];pe=b['pe'];base=pe['image_base'];cpu=Uc(UC_ARCH_X86,UC_MODE_64);end=max(s['rva']+max(s['raw_size'],s['virtual_size']) for s in pe['sections']);cpu.mem_map(base,(end+4095)&~4095)
  for s in pe['sections']:
   if s['raw_size']:cpu.mem_write(base+s['rva'],raw[s['raw_pointer']:s['raw_pointer']+s['raw_size']])
  data,stack,stop=0x20000000,0x30000000,0x40000000;cpu.mem_map(data,0x100000);cpu.mem_map(stack,0x10000);cpu.mem_map(stop,0x1000);cpu.mem_write(data+0x10000,b''.join(struct.pack('<H',(i*73+(i//512)*101)&65535) for i in range(512*512)));cpu.mem_write(data+0x1008,struct.pack('<Q',data+0x10000));cpu.mem_write(data+0x1050,struct.pack('<ff',-2.5,20.))
  va={'A':0x140c8d2e0,'B':0x1409e9640,'C':0x140c71040}[version]
  flag={'B':0x14362c308,'C':0x143c8aad8}.get(version)
  if flag:cpu.mem_write(flag,struct.pack('<Q',data+0x100))
  lines=[];want=[];edges=[0,0x80000000,bits(-65.),bits(65.),bits(-100.),bits(100.),0x7f800000,0xff800000,0x7fc12345]
  for i in range(512):
   fix=0 if version=='A' else i%2;cpu.mem_write(data+0x100,bytes([fix]));x=edges[i%len(edges)] if i<81 else bits(rng.uniform(-95,95));z=edges[(i//len(edges))%len(edges)] if i<81 else bits(rng.uniform(-95,95));cpu.mem_write(stack+0x8008,struct.pack('<Q',stop));cpu.reg_write(UC_X86_REG_RSP,stack+0x8008);cpu.reg_write(UC_X86_REG_RCX,data+0x1000);cpu.reg_write(UC_X86_REG_XMM1,x);cpu.reg_write(UC_X86_REG_XMM2,z);cpu.reg_write(UC_X86_REG_MXCSR,0x1f80);cpu.emu_start(va,stop,count=10000);assert cpu.reg_read(UC_X86_REG_RIP)==stop;want.append(f'{cpu.reg_read(UC_X86_REG_XMM0)&0xffffffff:08x}');lines.append(f'terrain {fix} {bits(-2.5):08x} {bits(20.):08x} {x:08x} {z:08x}')
  got=subprocess.run([str(ROOT/'reconstruction/target/release/wall-collection-probe')],input='\n'.join(lines)+'\n',capture_output=True,text=True,check=True).stdout.splitlines();assert len(got)==len(want)
  for i,(a,e) in enumerate(zip(got,want)):
   if a!=e:fail.append({'baseline':version,'case':i,'input':lines[i],'actual':a,'expected':e})
  versions.append({'baseline':version,'binary_sha256':b['binary']['sha256'],'va':hex(va),'cases':512,'sampling_fix_input':version!='A','bounds':'A has fixed -2.5/20 original constants; B/C bounds read record +50/+54.'})
 report={'ok':not fail,'versions':versions,'case_count':1536,'failure_count':len(fail),'failures':fail[:20],'scope':'Three original game terrain samples: grid512, u16 decode, axis order, both grid sampling policies (B/C), border/NaN/Inf and lerp order. Not A/C whole publisher or resampling consumer equivalence.','dependency_boundary':'Original game numeric instructions executed including custom utils::lerp; standard allocator/Bevy not entered.',**candidate_provenance()};(ROOT/'evidence/building-collection/three-version-terrain-verification.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'ok':not fail,'cases':1536,'failures':len(fail)}));assert not fail
if __name__=='__main__':main()
