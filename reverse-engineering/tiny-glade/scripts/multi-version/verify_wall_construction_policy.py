"""Actual B Roof/style/maxcache top owner and cloned-arch caller repair."""
import argparse,hashlib,json,random,struct,subprocess
from pathlib import Path
from unicorn import UC_HOOK_CODE
from unicorn.x86_const import *
from verify_parameters import cpu,DATA,STACK,ROOT,baseline,ret
from verify_wall_construction_sources import reset,run,original_seed,original_perturb,bits
active=False

def bridge(c,pc,size,user):
 if active and pc==0x142923c00:
  dst,src,n=[c.reg_read(r)for r in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8]]
  if n:c.mem_write(dst,bytes(c.mem_read(src,n)))
  c.reg_write(UC_X86_REG_RAX,dst);ret(c)
cpu.hook_add(UC_HOOK_CODE,bridge)

def original(case):
 global active
 wall,count,length,clutter,style,special,base,extra,maximum,shape,roof,heights=case
 seed,brick=original_seed(wall,count).split();reset();frame=STACK+0x11000
 cpu.mem_write(DATA+0x258,shape);cpu.mem_write(DATA+0x295,bytes([clutter,special]));cpu.mem_write(DATA+0x299,bytes([style]));cpu.mem_write(DATA+0x108,struct.pack('<II',1,maximum));cpu.mem_write(DATA+0x274,struct.pack('<I',base));cpu.mem_write(DATA+0x284,struct.pack('<I',extra));cpu.mem_write(frame+0x420,struct.pack('<Q',DATA));cpu.mem_write(frame+0x418,struct.pack('<Q',DATA+0x2a0));cpu.reg_write(UC_X86_REG_RSI,DATA+0x2a0)
 run(0x1407581b1,0x14075936e if special else 0x1407581bb)
 if special:return 'error SeparateSpecialConstruction'
 pointer=DATA+0x1000 if roof is not None else 0
 if roof is None:run(0x1407581e1,0x140758210)
 else:
  cpu.mem_write(pointer,roof);cpu.reg_write(UC_X86_REG_RDI,pointer);run(0x140758300,0x140758348)
 kind=struct.unpack('<Q',cpu.mem_read(frame+0x3f0,8))[0]&255
 if roof is None:gable=0
 else:
  cpu.reg_write(UC_X86_REG_RCX,pointer);cpu.mem_write(STACK+0x10000,struct.pack('<Q',STACK));run(0x1408e33b0,STACK);gable=cpu.reg_read(UC_X86_REG_RAX)&255
  cpu.reg_write(UC_X86_REG_RSP,STACK+0x10000);cpu.reg_write(UC_X86_REG_RCX,frame+0x2f0);cpu.reg_write(UC_X86_REG_RDX,pointer);cpu.mem_write(STACK+0x10000,struct.pack('<Q',STACK));run(0x1408e33e0,STACK)
  option,first,second=struct.unpack('<QQQ',cpu.mem_read(frame+0x2f0,24));cpu.mem_write(frame+0x3c8,struct.pack('<Q',option));cpu.mem_write(frame+0x3d0,struct.pack('<Q',second));cpu.reg_write(UC_X86_REG_R14,first)
 cpu.reg_write(UC_X86_REG_RSP,STACK+0x10000);cpu.reg_write(UC_X86_REG_R13,gable);cpu.reg_write(UC_X86_REG_RSI,DATA+0x2a0);run(0x14075873c,0x1407587dd)
 tag=cpu.reg_read(UC_X86_REG_R12)&255
 if tag==2:top='2 - - - - -'
 else:
  values=[struct.unpack('<I',cpu.mem_read(frame+0x398,4))[0],cpu.reg_read(UC_X86_REG_XMM11)&0xffffffff,cpu.reg_read(UC_X86_REG_XMM12)&0xffffffff,cpu.reg_read(UC_X86_REG_XMM13)&0xffffffff,struct.unpack('<I',cpu.mem_read(frame+0x39c,4))[0]];top=str(tag)+' '+ ' '.join(f'{v:08x}'for v in values)
 # Original clone memcpy plus actualflat/no-rim guards and whole scalar/SIMD loop.
 cpu.mem_write(DATA+0x2000,struct.pack('<'+'I'*len(heights),*heights));cpu.mem_write(frame+0x3a0,struct.pack('<Q',len(heights)));cpu.reg_write(UC_X86_REG_R12,len(heights)*4);cpu.reg_write(UC_X86_REG_RAX,DATA+0x3000);cpu.reg_write(UC_X86_REG_RBX,DATA+0x2000);cpu.reg_write(UC_X86_REG_RDI,pointer);cpu.reg_write(UC_X86_REG_XMM15,0xbf23d70a);cpu.reg_write(UC_X86_REG_XMM8,0x3f000000);cpu.reg_write(UC_X86_REG_XMM14,int.from_bytes(struct.pack('<I',0xbf23d70a)*4,'little'));cpu.reg_write(UC_X86_REG_XMM10,int.from_bytes(struct.pack('<I',0x3f000000)*4,'little'))
 active=True
 try:run(0x1407585c6,0x1407586b0)
 finally:active=False
 result=struct.unpack('<'+'I'*len(heights),cpu.mem_read(DATA+0x3000,len(heights)*4))if heights else []
 cpu.reg_write(UC_X86_REG_RAX,DATA+0x2a0);run(0x140759056,0x14075905b);base_read=cpu.reg_read(UC_X86_REG_XMM7)&0xffffffff
 run(0x14075919b,0x1407591a7);extra_read=cpu.reg_read(UC_X86_REG_XMM1)&0xffffffff
 perturb=original_perturb(tag,clutter,length)
 return f'{kind} {seed} {brick} {base_read:08x} {perturb} {extra_read:08x} {top} | '+' '.join(f'{v:08x}'for v in result)

def main():
 p=argparse.ArgumentParser();p.add_argument('--no-report',action='store_true');a=p.parse_args();rng=random.Random(20261022);cases=[]
 edges=[0,0x80000000,0x3f000000,0x3f91eb85,0x7f800000,0xff800000,0x7fc00001,0xffc00001];sizes=[0,1,2,3,7,8,9,15,16,17,31]
 for i in range(512):
  variant=i%6;shape=struct.pack('<I6f',0 if i%2 and variant<4 else 1,1.,0.,.2,.3,rng.uniform(.5,12),rng.uniform(.5,12));roof=None
  if variant:
   roof=bytearray(88);roof[8:36]=shape;struct.pack_into('<ff',roof,0x30,0. if variant<3 else .8,.7 if variant<4 else 1.);roof[0x3c]=int(variant==5);roof[0x3d]=1 if variant==2 else 0;roof=bytes(roof)
  n=sizes[i%len(sizes)];heights=[edges[(i+j)%len(edges)]if i<128 else bits(rng.uniform(-2,10))for j in range(n)]
  cases.append((rng.getrandbits(64),rng.randrange(10000),bits(rng.uniform(.1,30)),i%3,i%4,1 if i%16==15 else 0,bits(rng.uniform(-2,2)),bits(rng.uniform(.5,14)),bits(rng.uniform(.6,14)),shape,roof,heights))
 lines=[]
 for wall,count,length,clutter,style,special,base,extra,maximum,shape,roof,heights in cases:
  lines.append(f'policy {wall} {count} {length:08x} {clutter} {style} {special} {base:08x} {extra:08x} {maximum:08x} {shape.hex()} '+('none'if roof is None else roof.hex())+' '+' '.join(f'{v:08x}'for v in heights))
 probe=ROOT/'reconstruction/target/release/wall-construction-sources-probe';actual=subprocess.run([str(probe)],input='\n'.join(lines)+'\n',capture_output=True,text=True,check=True).stdout.splitlines();assert len(actual)==len(cases);fail=[]
 for i,(case,got)in enumerate(zip(cases,actual)):
  want=original(case)
  if want.rstrip()!=got.rstrip():fail.append({'case':i,'input':lines[i],'original':want,'rust':got})
 sources=[ROOT/'reconstruction/building/wall_construction_sources.rs',ROOT/'reconstruction/building/wall_gable.rs',ROOT/'reconstruction/roof/roof_state.rs',ROOT/'reconstruction/src/bin/wall-construction-sources-probe.rs',ROOT/'reconstruction/Cargo.lock']
 report={'ok':not fail,'baseline':'B','cases':len(cases),'special_guard_cases':32,'failure_count':len(fail),'failures':fail[:16],'scope':'Original regular construct_walls gate, actualRoof predicates/gable sides/top cached height andinterval caller; fullarch memcpy/flat-no-rim repair including original SIMD+scalar tails; seed/row/base/scalar linked to original policy ranges','executed_business':{'special_gate':['0x1407581b1','0x14075936e'],'roof_kind':['0x140758300','0x140758348'],'top':['0x14075873c','0x1407587dd'],'arch_clone_repair':['0x1407585c6','0x1407586b0']},'dependency_bridges':['CRT memcpy only; original allRoof/privateShape/Rectangle/cachehelpers execute'],'excluded':['Special construction consumer after0x14075936e','Missingmaxcache/invalidshape panic behavior','Fullfrom_curve instance production separately verified','A/C execution','RuntimeWorld integration'],'binary_sha256':baseline['binary']['sha256'],'candidate_binary_sha256':hashlib.sha256(probe.read_bytes()).hexdigest(),'candidate_sources':{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest()for p in sources}}
 if not a.no_report:(ROOT/'evidence/building-production-authority'/('B-construction-policy-verification.json'if not fail else 'B-construction-policy-failure.json')).write_text(json.dumps(report,indent=2)+'\n')
 print(json.dumps({'ok':not fail,'cases':len(cases),'failures':len(fail),'first':fail[:1]}));assert not fail,fail[:1]
if __name__=='__main__':main()
