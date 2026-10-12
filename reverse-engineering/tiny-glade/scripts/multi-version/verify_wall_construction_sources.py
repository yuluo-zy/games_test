"""Bounded original construct_walls source policy; no injected game outputs."""
import argparse,hashlib,json,random,struct,subprocess
from unicorn.x86_const import *
from verify_parameters import cpu,DATA,STACK,ROOT,baseline

def reset():
 cpu.mem_write(DATA,bytes(0x3000));cpu.mem_write(STACK,bytes(0x16000))
 for r in [UC_X86_REG_RAX,UC_X86_REG_RBX,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_RDI,UC_X86_REG_RSI,UC_X86_REG_RBP,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,UC_X86_REG_R13,UC_X86_REG_R14,UC_X86_REG_R15]:cpu.reg_write(r,0)
 cpu.reg_write(UC_X86_REG_RSP,STACK+0x10000);cpu.reg_write(UC_X86_REG_RBP,STACK+0x11000);cpu.reg_write(UC_X86_REG_MXCSR,0x1f80);cpu.reg_write(UC_X86_REG_RFLAGS,0x202)
def run(start,end):
 cpu.emu_start(start,end,count=1000);assert cpu.reg_read(UC_X86_REG_RIP)==end
def original_seed(wall,count):
 reset();frame=STACK+0x11000;cpu.mem_write(frame+0x428,struct.pack('<Q',wall));cpu.mem_write(DATA+0x48,struct.pack('<Q',count));cpu.reg_write(UC_X86_REG_RSI,DATA+0x2a0)
 run(0x140758381,0x14075839b);seed=cpu.reg_read(UC_X86_REG_RAX)
 cpu.reg_write(UC_X86_REG_RCX,wall);cpu.mem_write(STACK+0x10000,struct.pack('<Q',STACK));run(0x140858790,STACK)
 return f'{seed} {cpu.reg_read(UC_X86_REG_RAX)&0xffffffff}'
def original_perturb(tag,version,length):
 reset();frame=STACK+0x11000;cpu.mem_write(frame+0x418,struct.pack('<Q',DATA+0x2a0));cpu.mem_write(DATA+0x295,bytes([version]));cpu.mem_write(DATA+0x28c,struct.pack('<I',length));cpu.reg_write(UC_X86_REG_RAX,DATA+0x2a0);cpu.reg_write(UC_X86_REG_R12,tag)
 run(0x140758fe5,0x140759056);return f'{cpu.reg_read(UC_X86_REG_XMM6)&0xffffffff:08x}'
def original_options(tag,radius):
 reset();frame=STACK+0x11000;cpu.mem_write(frame+0x420,struct.pack('<Q',DATA));cpu.mem_write(frame+0x418,struct.pack('<Q',DATA+0x2a0));cpu.mem_write(DATA+0x258,struct.pack('<I',tag));cpu.mem_write(DATA+0x264,struct.pack('<I',radius))
 run(0x1407590ba,0x1407591a7)
 width=f'{cpu.reg_read(UC_X86_REG_XMM15)&0xffffffff:08x}' if cpu.reg_read(UC_X86_REG_RSI)&0xffffffff else '-'
 thickness=f'{cpu.reg_read(UC_X86_REG_XMM0)&0xffffffff:08x}' if cpu.reg_read(UC_X86_REG_RCX)&0xffffffff else '-'
 return f'{width} {thickness}'
def bits(f):return struct.unpack('<I',struct.pack('<f',f))[0]
def original_kind(version,style,roof):
 reset();cpu.mem_write(DATA+0x295,bytes([version]));cpu.mem_write(DATA+0x299,bytes([style]));cpu.reg_write(UC_X86_REG_RSI,DATA+0x2a0)
 if roof==0:run(0x1407581e1,0x140758210)
 else:
  cpu.mem_write(DATA+0x1030,struct.pack('<f',1.0 if roof==1 else 0.0));cpu.mem_write(DATA+0x103d,bytes([roof==3]));cpu.reg_write(UC_X86_REG_RDI,DATA+0x1000)
  run(0x140758300,0x140758348)
 return str(struct.unpack('<Q',cpu.mem_read(STACK+0x11000+0x3f0,8))[0]&255)
def main():
 p=argparse.ArgumentParser();p.add_argument('--no-report',action='store_true');args=p.parse_args();rng=random.Random(20261018);cases=[]
 for i in range(1024):
  wall=(rng.getrandbits(32)<<32)|(100000+i%20) if i<256 else rng.getrandbits(64)
  count=rng.getrandbits(64) if i%4==0 else rng.randrange(10000);cases.append(('seed',(wall,count)))
 edges=[0,0x80000000,0x7f800000,0xff800000,0x7fc00001,0xffc00001,*[bits(v)for v in [.69,1.38,.9,3,3.6]]]
 for i in range(2048):
  value=edges[i%len(edges)]if i<256 else bits(rng.uniform(-10,30));cases.append(('perturb',(i%4,i%3,value)))
 for i in range(2048):
  value=edges[i%len(edges)]if i<256 else bits(rng.uniform(-10,30));cases.append(('options',(i%4,value)))
 for i in range(1024):cases.append(('kind',(i%256,(i//4)%256,i%4)))
 lines=[kind+' '+(' '.join(str(x)for x in xs)if kind in ('seed','kind')else ' '.join(str(x)for x in xs[:-1])+f' {xs[-1]:08x}')for kind,xs in cases]
 probe=ROOT/'reconstruction/target/release/wall-construction-sources-probe';actual=subprocess.run([str(probe)],input='\n'.join(lines)+'\n',capture_output=True,text=True,check=True).stdout.splitlines();assert len(actual)==len(cases)
 functions={'seed':original_seed,'perturb':original_perturb,'options':original_options,'kind':original_kind};fail=[]
 for i,((kind,xs),got)in enumerate(zip(cases,actual)):
  want=functions[kind](*xs)
  if want!=got:fail.append({'case':i,'input':lines[i],'original':want,'rust':got})
 report={'ok':not fail,'cases':len(cases),'failure_count':len(fail),'failures':fail[:16],
  'scope':'B original WallId/BrickSourceId source conversion and construct_walls caller seed/row perturb/circle option policy, including reserved IDs, wraparound and SSE NaN/inf boundaries',
  'executed_business':{'seed':['0x140758381','0x14075839b'],'brick_source_id':'0x140858790','row_end':['0x140758fe5','0x140759056'],'options':['0x1407590ba','0x1407591a7']},
  'dependency_bridges':[],
  'excluded':['Prepared segmentation Curve3 owner','Arch/gable/third-array subsequent writers','Complete constructor producer separately verified','A/C native execution','World integration'],
  'binary_sha256':baseline['binary']['sha256'],'candidate_binary_sha256':hashlib.sha256(probe.read_bytes()).hexdigest(),
  'candidate_sources':{str(path.relative_to(ROOT)):hashlib.sha256(path.read_bytes()).hexdigest()for path in [ROOT/'reconstruction/building/wall_construction_sources.rs',ROOT/'reconstruction/src/bin/wall-construction-sources-probe.rs']}}
 if not args.no_report:(ROOT/'evidence/building-production-authority/B-source-policy-verification.json').write_text(json.dumps(report,indent=2)+'\n')
 print(json.dumps({'ok':not fail,'cases':len(cases),'failures':len(fail),'report_written':not args.no_report}));assert not fail,fail[:2]
if __name__=='__main__':main()
