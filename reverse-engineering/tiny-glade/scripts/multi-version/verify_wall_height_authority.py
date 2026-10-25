"""Original B height authority wrapper, lookup, dirty insertion and source write."""
import argparse,hashlib,json,random,struct,subprocess
from unicorn import UC_HOOK_CODE
from unicorn.x86_const import *
from verify_parameters import cpu,DATA,STACK,ROOT,baseline,ret
from verification_provenance import candidate_provenance

dirty=[]
def bridge(c,pc,size,user):
 if pc==0x1405b0d70:
  # std RandomState hashing only. Actual map probe compares the requested ID.
  c.reg_write(UC_X86_REG_RAX,0);ret(c)
 elif pc==0x1402ca2f0:
  # Actual std dirty-set ownership stays with Rust; capture original call args.
  dirty.append((c.reg_read(UC_X86_REG_RCX),c.reg_read(UC_X86_REG_RDX)));ret(c)
cpu.hook_add(UC_HOOK_CODE,bridge)

def execute(present,wall,height,source):
 dirty.clear();cpu.mem_write(DATA,bytes(0x5000));cpu.mem_write(STACK,bytes(0x12000))
 owner=DATA;control=DATA+0x1000;source_pointer=control-0x70
 cpu.mem_write(control,bytes([0,255])+bytes([255])*14)
 cpu.mem_write(control-0x78,struct.pack('<Q',wall if present else wall^1))
 cpu.mem_write(source_pointer,source)
 # Missing cases cover both empty maps and populated maps with another ID.
 cpu.mem_write(owner,struct.pack('<QQQQ',control,15,14,int(present or bool(wall&1))))
 # Disabled logging; missing-owner branch returns with no panic/write/dirty.
 cpu.mem_write(0x14362c3a8,struct.pack('<Q',DATA+0x4000))
 sp=STACK+0x8008;cpu.mem_write(sp,struct.pack('<Q',STACK));cpu.reg_write(UC_X86_REG_RSP,sp)
 for r in [UC_X86_REG_RAX,UC_X86_REG_RBX,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_RSI,UC_X86_REG_RDI,UC_X86_REG_RBP,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,UC_X86_REG_R13,UC_X86_REG_R14,UC_X86_REG_R15]:cpu.reg_write(r,0)
 cpu.reg_write(UC_X86_REG_MXCSR,0x1f80);cpu.reg_write(UC_X86_REG_RFLAGS,0x202)
 cpu.reg_write(UC_X86_REG_RCX,owner);cpu.reg_write(UC_X86_REG_RDX,wall);cpu.reg_write(UC_X86_REG_XMM2,height)
 cpu.emu_start(0x14085ad60,STACK,count=3000)
 assert cpu.reg_read(UC_X86_REG_RIP)==STACK
 result=bytes(cpu.mem_read(source_pointer,112)).hex()
 # The real publisher subsequently reads the same authority scalar at+54.
 # This executes original addressing/load instructions, not an injected value.
 frame=STACK+0x1000;cpu.mem_write(frame+0xd40,struct.pack('<Q',source_pointer+0x70))
 cpu.reg_write(UC_X86_REG_RBP,frame)
 cpu.emu_start(0x140a8ce58,0x140a8ce65,count=2)
 observed=cpu.reg_read(UC_X86_REG_XMM12)&0xffffffff
 assert observed==struct.unpack_from('<I',bytes.fromhex(result),0x54)[0]
 if present:
  assert dirty==[(owner+0x60,wall)],dirty
  return f'{result} 1 {wall} 1'
 assert not dirty;return f'{result} 0 - 0'

def main():
 parser=argparse.ArgumentParser();parser.add_argument('--no-report',action='store_true');args=parser.parse_args()
 rng=random.Random(20261017);cases=[]
 edges=[0,0x80000000,0x3dcccccd,0x3fcccccd,0x7f800000,0xff800000,0x7fc00001,0x7fa00001,0xffc00001]
 for i in range(1024):
  source=bytearray(rng.randbytes(112));value=edges[i%len(edges)]if i<128 else rng.getrandbits(32)
  if i%4==0:struct.pack_into('<I',source,0x54,value) # Same bit pattern is still dirty.
  cases.append((i%8!=7,rng.getrandbits(64),value,bytes(source)))
 lines=[f'{int(p)} {w} {v:08x} {s.hex()}'for p,w,v,s in cases]
 probe=ROOT/'reconstruction/target/release/wall-height-authority-probe'
 results=subprocess.run([str(probe)],input='\n'.join(lines)+'\n',text=True,capture_output=True,check=True).stdout.splitlines()
 assert len(results)==len(cases);fail=[]
 for i,(case,result)in enumerate(zip(cases,results)):
  original=execute(*case)
  if original!=result:fail.append({'case':i,'input':lines[i],'original':original,'rust':result})
 report={'ok':not fail,'cases':len(cases),'successful_source_updates':896,'missing_owners':128,'failures':fail[:10],'failure_count':len(fail),
  'scope':'B update_wall_state_height wrapper through original map probe and game dirty-write suffix; arbitrary f32 bits/same values; whole112-byte source projection preserved except+54',
  'executed_business':{'entry':'0x14085ad60','try_update_state':'0x140aaaca0','dirty_and_write':['0x140aaae55','0x140aaae6f'],'subsequent_original_publisher_source_read':['0x140a8ce58','0x140a8ce65']},
  'dependency_bridges':{'0x1405b0d70':'RandomState hash output0; original source ID lookup probes execute','0x1402ca2f0':'HashMap<WallId,()> insert captures original dirty-call owner+60 and WallId; dependency storage excluded'},
  'binary_sha256':baseline['binary']['sha256'],'candidate_binary_sha256':hashlib.sha256(probe.read_bytes()).hexdigest(),
  'excluded':['Clamping UI/Replayed height request producer','Bevy ResMut tick wrapper','Original logging output formatting','FullInnerWall startup/Curve2 mutation','A/C execution acceptance','Parent actualWorld publication/Roof/holes chain separately verified'],**candidate_provenance()}
 if not args.no_report:
  destination=ROOT/'evidence/building-history-owners/B-wall-height-authority-verification.json';destination.write_text(json.dumps(report,indent=2)+'\n')
 print(json.dumps({'ok':not fail,'cases':len(cases),'failures':len(fail),'report_written':not args.no_report}))
 assert not fail,fail[:1]

if __name__=='__main__':main()
