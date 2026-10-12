"""Original dirty primary branch and complete terrain-republication predicate."""
import hashlib,json,random,struct,subprocess
from pathlib import Path
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
from verification_provenance import ROOT,candidate_provenance
def bits(f):return struct.unpack('<I',struct.pack('<f',f))[0]
def main():
 b=json.loads((ROOT/'tracking/baselines.json').read_text())['baselines']['B'];raw=Path(b['binary']['path']).read_bytes();assert hashlib.sha256(raw).hexdigest()==b['binary']['sha256'];pe=b['pe'];base=pe['image_base'];cpu=Uc(UC_ARCH_X86,UC_MODE_64);end=max(s['rva']+max(s['raw_size'],s['virtual_size']) for s in pe['sections']);cpu.mem_map(base,(end+4095)&~4095)
 for s in pe['sections']:
  if s['raw_size']:cpu.mem_write(base+s['rva'],raw[s['raw_pointer']:s['raw_pointer']+s['raw_size']])
 data,stack=0x20000000,0x30000000;cpu.mem_map(data,0x100000);cpu.mem_map(stack,0x10000);cpu.mem_write(data+0x10000,b''.join(struct.pack('<H',(i*73+(i//512)*101)&65535) for i in range(512*512)));cpu.mem_write(data+0x1008,struct.pack('<Q',data+0x10000));cpu.mem_write(data+0x1050,struct.pack('<ff',-3.,18.));cpu.mem_write(0x14362c308,struct.pack('<Q',data+0x100));marked=[False]
 def hook(uc,address,size,user):
  if address==0x1402ca2f0:marked[0]=True;uc.emu_stop()
 cpu.hook_add(UC_HOOK_CODE,hook);rng=random.Random(20261012);lines=[];want=[];edges=[0,0x80000000,0x7f800000,0xff800000,0x7fc12345,bits(.5),bits(.50001),bits(-2.7)]
 for i in range(256):
  relative=edges[i%len(edges)] if i<64 else bits(rng.uniform(-1,2));cpu.mem_write(data+0x2a0-0x24,struct.pack('<I',relative));cpu.reg_write(UC_X86_REG_RAX,data+0x2a0);cpu.reg_write(UC_X86_REG_XMM11,bits(.5));cpu.reg_write(UC_X86_REG_RSP,stack+0x8000);marked[0]=False;cpu.emu_start(0x140adf3e6,0x140adf400,count=100);lines.append(f'dirty primary {relative:08x} 0');want.append(str(int(marked[0])))
 for i in range(1024):
  fix=i%2;n=[0,1,2,3,8][i%5];floor=edges[i%len(edges)] if i<100 else bits(rng.uniform(-3,14));positions=[(bits(rng.uniform(-70,70)),edges[(i+j)%len(edges)] if i<100 else bits(rng.uniform(-4,18)),bits(rng.uniform(-70,70))) for j in range(n)]
  cpu.mem_write(data+0x100,bytes([fix]));cpu.mem_write(data+0x4000,b''.join(struct.pack('<III',*p) for p in positions));cpu.reg_write(UC_X86_REG_R12,data+0x4000);cpu.reg_write(UC_X86_REG_RSI,data+0x4000+n*12);cpu.reg_write(UC_X86_REG_R13,1);cpu.reg_write(UC_X86_REG_RBX,0);cpu.reg_write(UC_X86_REG_XMM8,floor);cpu.reg_write(UC_X86_REG_RBP,stack+0x8000);cpu.reg_write(UC_X86_REG_RSP,stack+0x6000);cpu.mem_write(stack+0x8000+0xce0,struct.pack('<Q',data+0x1000));cpu.reg_write(UC_X86_REG_MXCSR,0x1f80);marked[0]=False;cpu.emu_start(0x140a8c0e5,0x140a8c150,count=100000);lines.append(f'republication {fix} {bits(-3.):08x} {bits(18.):08x} {floor:08x} '+' '.join(f'{v:08x}' for p in positions for v in p));want.append(str(int(marked[0])))
 got=subprocess.run([str(ROOT/'reconstruction/target/release/wall-collection-probe')],input='\n'.join(lines)+'\n',capture_output=True,text=True,check=True).stdout.splitlines();assert len(got)==len(want);fail=[{'case':i,'input':lines[i],'expected':e,'actual':a} for i,(a,e) in enumerate(zip(got,want)) if a!=e]
 report={'ok':not fail,'case_count':1280,'primary_dirty_cases':256,'terrain_predicate_cases':1024,'failure_count':len(fail),'failures':fail[:20],'baseline':'B','binary_sha256':b['binary']['sha256'],'scope':'Original primary tile candidate threshold and complete per-outline terrain invalidation loop, actual game terrain sampler, ordered/NaN/Inf comparisons. Secondary membership selection is instruction-reviewed separately, not whole native HashSet integration.','dependency_boundary':'HashSet insert captures business intention; no library container implementation reconstructed.',**candidate_provenance()};(ROOT/'evidence/building-collection/B-dirty-verification.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'ok':not fail,'cases':1280,'failures':len(fail)}));assert not fail
if __name__=='__main__':main()
