"""Original identified MaterialStats fields. Opaque reset tail not reinterpreted."""
import sys,random,json,struct,hashlib,subprocess
from verification_provenance import ROOT
sys.path.insert(0,str(ROOT/'evidence/building-constructor'))
from native_fixture import *

def invoke(entry,args):
 sp=stack+0x100008;cpu.mem_write(sp,struct.pack('<Q',stop));cpu.reg_write(UC_X86_REG_RSP,sp)
 for reg,val in args:cpu.reg_write(reg,val)
 cpu.emu_start(entry,stop,count=50000);assert cpu.reg_read(UC_X86_REG_RIP)==stop

def main():
 rng=random.Random(20261025);lines=[];expected=[]
 for i in range(512):
  word=rng.getrandbits(32);special=[0,1,2,255][i%4];count=rng.getrandbits(48);cpu.mem_write(data,bytes(0x200))
  invoke(0x140a88280,[(UC_X86_REG_RCX,data),(UC_X86_REG_R8,special),(UC_X86_REG_XMM1,word)])
  invoke(0x140a88440,[(UC_X86_REG_RCX,data),(UC_X86_REG_RDX,count&0xffffffff)])
  regular,special_length=struct.unpack('<I',cpu.mem_read(data+0x90,4))[0],struct.unpack('<I',cpu.mem_read(data+0x9c,4))[0];segcount=struct.unpack('<I',cpu.mem_read(data+0xac,4))[0]
  lines.append(f'stats {word:08x} {special} {count}');expected.append(f'{regular:08x} {special_length:08x} {segcount}')
 probe=ROOT/'reconstruction/target/release/wall-segmentation-probe';actual=subprocess.run([str(probe)],input='\n'.join(lines)+'\n',capture_output=True,text=True,check=True).stdout.splitlines();assert len(actual)==len(expected)
 failures=[{'case':i,'input':line,'original':want,'rust':got}for i,(line,want,got)in enumerate(zip(lines,expected,actual))if want!=got]
 report={'ok':not failures,'cases':len(lines),'failure_count':len(failures),'failures':failures[:10],'binary_sha256':b['binary']['sha256'],'candidate_binary_sha256':hashlib.sha256(probe.read_bytes()).hexdigest(),'scope':'Actual MaterialStats init_this_update140a88280 followed by set_path_segmentation_count140a88440. Compare only identified+90 regular length/+9c special length/+ac count, arbitraryf32bits. Original opaque tail resets not reinterpreted or represented by typedprojection. Stddealloc excluded.','candidate_sources':{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest()for p in [ROOT/'reconstruction/building/wall_segmentation.rs',ROOT/'reconstruction/src/bin/wall-segmentation-probe.rs']},'oracle_sources':{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest()for p in [ROOT/'scripts/multi-version/verify_wall_segmentation_material_stats.py',ROOT/'evidence/building-constructor/native_fixture.py']}}
 (ROOT/'evidence/building-segmentation/B-material-stats-verification.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'ok':not failures,'cases':len(lines),'failures':len(failures)}));assert not failures
if __name__=='__main__':main()
