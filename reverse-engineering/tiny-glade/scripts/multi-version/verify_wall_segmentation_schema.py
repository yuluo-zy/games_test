"""Native LoopRange/thresholdcoord and complete empty calc owner. Separate full-mask report covers nonempty."""
import sys,random,json,struct,hashlib,subprocess
from verification_provenance import ROOT
from verify_parameters import cpu,DATA,STACK,baseline
from unicorn.x86_const import *
sys.path.insert(0,str(ROOT/'evidence/building-segmentation'))
from native_segmentation import produce

def call(entry,args,xmm=None):
 sp=STACK+0x10008;cpu.mem_write(sp,struct.pack('<Q',STACK));cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
 for reg,value in args:cpu.reg_write(reg,value)
 if xmm is not None:cpu.reg_write(UC_X86_REG_XMM1,xmm)
 cpu.emu_start(entry,STACK,count=100);assert cpu.reg_read(UC_X86_REG_RIP)==STACK

def main():
 rng=random.Random(20261023);lines=[];expected=[]
 for i in range(1024):
  count=rng.randrange(2,128);closed=bool(i%2);modulus=count-1 if closed else count
  start=rng.randrange(-modulus+1,modulus if not closed else modulus+1)
  end=rng.randrange(max(0,start),min(modulus+1,start+(modulus+1 if closed else modulus))+1)
  call(0x141200390 if closed else 0x141200450,[(UC_X86_REG_RCX,DATA),(UC_X86_REG_RDX,start&0xffffffff),(UC_X86_REG_R8,end),(UC_X86_REG_R9,modulus)])
  fields=struct.unpack('<3i',cpu.mem_read(DATA,12));assert fields==(start,end,modulus)
  call(0x141200350,[(UC_X86_REG_RCX,DATA)]);n=cpu.reg_read(UC_X86_REG_RAX);assert n==end-start
  # Signed idiv and negative remainder correction are original index policy.
  values=[(start+j)%modulus for j in range(n)]
  lines.append(f'range {int(closed)} {start} {end} {count}');expected.append(str(n)+' '+','.join(map(str,values)))
 for i in range(1024):
  n=rng.randrange(2,128);index=rng.randrange(n);fraction=0 if index==n-1 else rng.choice([0,0x80000000,0x3f000000,0x3f800000,rng.randrange(0x3e000000,0x3f800000)])
  call(0x1411fb2b0,[(UC_X86_REG_RCX,index),(UC_X86_REG_R8,n)],fraction)
  expected.append(f'{cpu.reg_read(UC_X86_REG_RAX)&0xffffffff} {cpu.reg_read(UC_X86_REG_XMM0)&0xffffffff:08x}');lines.append(f'coord {index} {fraction:08x} {n}')
 for i in range(128):
  n=[0,1,2,3,8,9,17,33][i%8];vals=[struct.unpack('<f',struct.pack('<I',rng.choice([0,0x80000000,0xbe99999a,0xbf800000])))[0]for _ in range(n)];points=[[float(j),0.,float(j%3)]for j in range(n)];shape=i%3;special=i%2
  out=produce(points,vals,shape,special);assert not out
  lines.append(f'empty {shape} {special} {n} '+' '.join(f'{struct.unpack("<I",struct.pack("<f",v))[0]:08x}'for v in vals));expected.append('0')
 probe=ROOT/'reconstruction/target/release/wall-segmentation-probe';actual=subprocess.run([str(probe)],input='\n'.join(lines)+'\n',capture_output=True,text=True,check=True).stdout.splitlines();assert len(actual)==len(expected)
 fail=[{'case':i,'input':lines[i],'original':want,'rust':got}for i,(want,got)in enumerate(zip(expected,actual))if want.rstrip()!=got.rstrip()]
 report={'ok':not fail,'baseline':'B','range_constructor_and_len_cases':1024,'threshold_coord_cases':1024,'empty_whole_calc_cases':128,'failure_count':len(fail),'failures':fail[:10],'scope':'Actual native LoopRange closed/open constructors+len and thresholded CurveInternalCoord; complete original calc→emptyVec when allpathvalues<=0. Index_at modulo schema uses explicitoriginal idiv/correction evidence; standalone nativeindex computation not separately executed here. Nonempty maskproducer not accepted.','native_entries':['0x141200390','0x141200450','0x141200350','0x1411fb2b0','0x14074f7a0'],'binary_sha256':baseline['binary']['sha256'],'oracle_sources':{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest()for p in [ROOT/'evidence/building-segmentation/native_segmentation.py',ROOT/'evidence/building-constructor/native_fixture.py',ROOT/'scripts/multi-version/verify_wall_segmentation_schema.py']},'candidate_binary_sha256':hashlib.sha256(probe.read_bytes()).hexdigest(),'candidate_sources':{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest()for p in [ROOT/'reconstruction/building/wall_segmentation.rs',ROOT/'reconstruction/src/bin/wall-segmentation-probe.rs']}}
 (ROOT/'evidence/building-segmentation/B-schema-empty-verification.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'ok':not fail,'cases':len(lines),'failures':len(fail)}));assert not fail,fail[:1]
if __name__=='__main__':main()
