"""Original init_this_update/count/actual stairs ADD then regular SUB policy."""
import sys,random,json,struct,hashlib,subprocess
from verification_provenance import ROOT
sys.path.insert(0,str(ROOT/'evidence/building-constructor'))
from native_fixture import *
def invoke(entry,args):
 sp=stack+0x100008;cpu.mem_write(sp,struct.pack('<Q',stop));cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
 for reg,val in args:cpu.reg_write(reg,val)
 cpu.emu_start(entry,stop,count=100000);assert cpu.reg_read(UC_X86_REG_RIP)==stop
def main():
 rng=random.Random(20261103);lines=[];expected=[];operations=0
 for i in range(512):
  word=rng.getrandbits(32)
  if word&0x7f800000==0x7f800000:word=bits(65504.)
  special=[0,1,2,255][i%4];count=rng.getrandbits(48);delta=[bits(rng.uniform(0.,20.))for _ in range(i%5)];operations+=len(delta)
  cpu.mem_write(data,rng.randbytes(0x200));cpu.mem_write(data+0x70,bytes(8))
  invoke(0x140a88280,[(UC_X86_REG_RCX,data),(UC_X86_REG_R8,special),(UC_X86_REG_XMM1,word)]);assert bytes(cpu.mem_read(data+0xa4,4))==bytes(4)
  invoke(0x140a88440,[(UC_X86_REG_RCX,data),(UC_X86_REG_RDX,count&0xffffffff)])
  for d in delta:invoke(0x140a88590,[(UC_X86_REG_RCX,data),(UC_X86_REG_XMM1,d)])
  fields=[struct.unpack('<I',cpu.mem_read(data+at,4))[0]for at in [0x90,0x9c,0xac,0xa4]];expected.append(''.join(f'{v:08x}'for v in fields));lines.append(f'{word:08x} {special} {count} '+' '.join(f'{d:08x}'for d in delta))
 probe=ROOT/'reconstruction/target/release/wall-platform-stair-stats-probe';actual=subprocess.run([str(probe)],input='\n'.join(lines)+'\n',capture_output=True,text=True,check=True).stdout.splitlines();assert len(actual)==len(expected);failures=[{'case':i,'input':lines[i],'original':want,'rust':got}for i,(want,got)in enumerate(zip(expected,actual))if want!=got]
 files=['reconstruction/building/wall_segmentation.rs','reconstruction/src/bin/wall-platform-stair-stats-probe.rs','reconstruction/Cargo.lock','reconstruction/Cargo.toml'];oracles=['scripts/multi-version/verify_wall_platform_stair_stats.py','evidence/building-constructor/native_fixture.py'];digest=lambda xs:{p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest()for p in xs}
 report={'ok':not failures,'cases':len(lines),'add_stairs_operations':operations,'failure_count':len(failures),'failures':failures[:10],'scope':'Original MaterialStats init_this_update140a88280 on arbitrary observed priorbytes with resolved empty std Vec, set_path_segmentation_count140a88440 then actual add_stairs_length140a88590 repeated0..4 times. Compare identified +90 regular/+9c special/+ac count/+a4 stairs; prove +a4 initial positivezero. Original lockcmpxchg single-thread instructions execute; candidate exclusive mutableWorld projection uses f32 +=stairs and -=regular, no atomic storage/retry internals reimplemented. Finite raw initiallength, realpositive length contributions including overflow; no meaning inferred for remaining opaque resettail.','binary_sha256':b['binary']['sha256'],'candidate_binary_sha256':hashlib.sha256(probe.read_bytes()).hexdigest(),'candidate_sources':digest(files),'oracle_sources':digest(oracles)}
 name='B-platform-stair-stats-verification.json'if not failures else'B-platform-stair-stats-first-failure.json';(ROOT/'evidence/building-segmentation'/name).write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'ok':not failures,'cases':len(lines),'operations':operations,'failures':len(failures),'first':failures[:1]}));assert not failures
if __name__=='__main__':main()
