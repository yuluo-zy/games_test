"""Original full B AdjustWallHeightCmd reader, clamp, anchors, writes and audio."""
import argparse,hashlib,json,random,struct,subprocess
from unicorn import UC_HOOK_CODE
from unicorn.x86_const import *
from verify_parameters import cpu,DATA,STACK,ROOT,baseline,ret
from verification_provenance import candidate_provenance

GRID=0x25000000;cpu.mem_map(GRID,0x100000);cpu.mem_write(GRID,bytes(512*512*2))
dirty=[];failure=None;roof_pointer=0
def hook(c,pc,size,user):
 global failure
 if pc==0x1405b0d70:c.reg_write(UC_X86_REG_RAX,0);ret(c)
 elif pc==0x1402ca2f0:dirty.append(c.reg_read(UC_X86_REG_RDX));ret(c)
 elif pc==0x140a923a0:c.reg_write(UC_X86_REG_RAX,roof_pointer);ret(c)
 elif pc==0x1428d9310:failure='MissingAuthority';c.emu_stop()
 elif pc==0x1428d9430:failure='UnsupportedScalarMode';c.emu_stop()
cpu.hook_add(UC_HOOK_CODE,hook)
def put(p,fmt,*values):cpu.mem_write(p,struct.pack(fmt,*values))
def get(p,fmt):return struct.unpack(fmt,cpu.mem_read(p,struct.calcsize(fmt)))
def bits(f):return struct.unpack('<I',struct.pack('<f',f))[0]

def execute(payload,current,kind,public_present,roof,special,dt,terrain,points,present=True):
 global failure,roof_pointer
 dirty.clear();failure=None;cpu.mem_write(DATA,bytes(0x20000));cpu.mem_write(STACK,bytes(0x12000))
 wall=struct.unpack_from('<Q',payload,24)[0]
 owner=DATA+0x1000;control=DATA+0x1800;source=control-0x70
 put(owner,'<QQQQ',control,15,14,int(present));cpu.mem_write(control,bytes([0])+bytes([255])*15)
 put(control-0x78,'<Q',wall);put(source+0x54,'<I',current);put(source+0x38,'<I',kind)
 ptr=DATA+0x2000;uptr=DATA+0x2100
 put(ptr,'<IIII',*points);put(uptr,'<ff',0.,1.)
 put(source,'<QQQQQQf',2,ptr,2,2,uptr,2,1.)
 public=DATA+0x3000;pcontrol=DATA+0x3800;pstate=pcontrol-0x2a0
 put(public,'<QQQQ',pcontrol,15,14,int(public_present));cpu.mem_write(pcontrol,bytes([0])+bytes([255])*15)
 put(pcontrol-0x2a8,'<Q',wall);put(pstate+0x10,'<Q',2);put(pstate+0x30,'<f',1.);put(pstate+0x296,'<B',special)
 roof_pointer=0 if roof is None else DATA+0x4000
 if roof is not None:put(roof_pointer+0x30,'<I',roof)
 terrain_state=DATA+0x5000;put(terrain_state+8,'<Q',GRID);put(terrain_state+0x50,'<II',terrain,terrain)
 put(0x14362c308,'<Q',DATA+0x5800)
 time=DATA+0x5900;put(time+0x58,'<I',dt)
 events=DATA+0x6000;records=DATA+0x6200;reader_count=DATA+0x6100;reader=DATA+0x6120
 put(events,'<QQQQQQQQQ',4,records,1,0,0,8,0,1,1);cpu.mem_write(records,payload+bytes(8));put(reader,'<QQ',reader_count,events)
 inner_res=DATA+0x7000;public_res=DATA+0x7100;terrain_res=DATA+0x7200;time_res=DATA+0x7300
 for resource,value in [(inner_res,owner),(public_res,public),(terrain_res,terrain_state),(time_res,time)]:put(resource,'<QQQII',value,0,DATA+0x7900,0,0x1234)
 material=DATA+0x8000;audio=DATA+0x9000;mw=DATA+0xa000;aw=DATA+0xa040
 for queue,writer in [(material,mw),(audio,aw)]:
  put(queue+0x20,'<QQQQQ',8,queue+0x100,0,0,0);put(writer,'<QQQII',queue,0,DATA+0x7900,0,0x1234)
 sp=STACK+0x8008;put(sp,'<Q',STACK)
 for offset,value in [(0x28,terrain_res),(0x30,public_res),(0x38,DATA+0x7400),(0x40,time_res)]:put(sp+offset,'<Q',value)
 for reg in [UC_X86_REG_RAX,UC_X86_REG_RBX,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_RSI,UC_X86_REG_RDI,UC_X86_REG_RBP,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,UC_X86_REG_R13,UC_X86_REG_R14,UC_X86_REG_R15]:cpu.reg_write(reg,0)
 for reg in range(UC_X86_REG_XMM0,UC_X86_REG_XMM15+1):cpu.reg_write(reg,0)
 cpu.reg_write(UC_X86_REG_RFLAGS,0x202);cpu.reg_write(UC_X86_REG_MXCSR,0x1f80);cpu.reg_write(UC_X86_REG_RSP,sp)
 cpu.reg_write(UC_X86_REG_RCX,reader);cpu.reg_write(UC_X86_REG_RDX,inner_res);cpu.reg_write(UC_X86_REG_R8,mw);cpu.reg_write(UC_X86_REG_R9,aw)
 cpu.emu_start(0x140ac1f20,STACK,count=30000)
 if failure:return 'error '+(f'UnsupportedScalarMode({struct.unpack_from("<I",payload,16)[0]})'if failure=='UnsupportedScalarMode'else failure)
 assert cpu.reg_read(UC_X86_REG_RIP)==STACK,hex(cpu.reg_read(UC_X86_REG_RIP))
 assert get(reader_count,'<Q')[0]==1
 new=get(source+0x54,'<I')[0];mc=get(material+0x30,'<Q')[0];ac=get(audio+0x30,'<Q')[0]
 if not dirty:
  assert mc==ac==0;return f'none {new:08x}'
 assert dirty==[wall]*len(dirty) and mc+ac==1
 queue=material if mc else audio;anchor=bytes(cpu.mem_read(queue+0x110,12)).hex()
 output=f'M{get(queue+0x11c,"<B")[0]}:{get(queue+0x11d,"<B")[0]}'if mc else f'B{get(queue+0x120,"<B")[0]}:{get(queue+0x11c,"<I")[0]:08x}'
 return f'some {new:08x} {len(dirty)} {anchor} {output}'

def main():
 parser=argparse.ArgumentParser();parser.add_argument('--no-report',action='store_true');args=parser.parse_args();rng=random.Random(20261018);cases=[]
 for i in range(1024):
  payload=bytearray(40);struct.pack_into('<I',payload,0,i%2)
  struct.pack_into('<fff',payload,4,rng.uniform(-10,10),rng.uniform(-5,5),rng.uniform(-10,10))
  mode=0 if i%16==0 else 2 if i%32==1 else 1
  value=bits(rng.uniform(-3,18));old=value if i%7==0 else bits(rng.uniform(-3,18))
  if i%23==0:value=0x7fc00001
  struct.pack_into('<IIQ',payload,16,mode,value,rng.getrandbits(64));payload[32]=[0,1,2,255][i%4]
  kind=i%4;public=i%3!=0;roof=[None,bits(0.),bits(.1),bits(.10001),bits(.7)][i%5];special=i%2
  dt=bits([0.,.016,.1,1.,float('inf')][i%5]);ty=bits(rng.uniform(-2,5));points=[bits(rng.uniform(-10,10))for _ in range(4)]
  cases.append((bytes(payload),old,kind,public,roof,special,dt,ty,points))
 edges=[0,0x80000000,bits(-1.),bits(.5),bits(14.),0x7f800000,0xff800000,0x7fc00001,0xffc00001]
 for i in range(256):
  payload=bytearray(40);struct.pack_into('<I',payload,0,1);struct.pack_into('<fff',payload,4,1.,2.,3.)
  struct.pack_into('<IIQ',payload,16,1,edges[i%len(edges)],rng.getrandbits(64));payload[32]=i%2
  old=edges[(i//len(edges))%len(edges)]
  cases.append((bytes(payload),old,i%4,i%3!=0,None,i%2,bits([0.,.1,float('inf')][i%3]),bits(.75),[bits(0.),bits(0.),bits(1.),bits(1.)]))
 lines=[p.hex()+f' {old:08x} {kind} {int(pub)} '+('none'if roof is None else f'{roof:08x}')+f' {special} {dt:08x} {ty:08x} '+' '.join(f'{x:08x}'for x in pts)for p,old,kind,pub,roof,special,dt,ty,pts in cases]
 probe=ROOT/'reconstruction/target/release/wall-height-command-probe';actual=subprocess.run([str(probe)],input='\n'.join(lines)+'\n',text=True,capture_output=True,check=True).stdout.splitlines();assert len(actual)==len(cases)
 fail=[]
 for i,(case,result)in enumerate(zip(cases,actual)):
  original=execute(*case)
  if original!=result:fail.append({'case':i,'input':lines[i],'original':original,'rust':result})
 report={'ok':not fail,'cases':len(cases),'failure_count':len(fail),'failures':fail[:10],
  'scope':'B complete AdjustWallHeightCmd reader with actual source/public-ID probes, native clamp numeric policy, explicit-or-originalCurveU0.5 anchor, actual Terrain sampler, source height authority and both audio outputs; resolved Roof optional query bridged',
  'executed_business':{'consumer':'0x140ac1f20','clamp':'0x140ac7f30','source_authority':'0x140aaaca0','terrain':'0x1409e9640','source_anchor_U0.5':'0x141467ce0'},
  'dependency_bridges':{'0x1405b0d70':'std RandomState hash output0; game probes original','0x1402ca2f0':'std dirty-set insert storage only; original repeated calls captured','0x140a923a0':'resolved optional Roof query; native numeric clamp executes'},
  'binary_sha256':baseline['binary']['sha256'],'candidate_binary_sha256':hashlib.sha256(probe.read_bytes()).hexdigest(),
  'excluded':['UI Delta toAbsolute producer','Original History recording owner and complete Wall compound transactions','Original Bevy schedule/dirty ownership structures','A/C runtime validation'],**candidate_provenance()}
 if not args.no_report:(ROOT/'evidence/building-history-owners/B-wall-height-command-verification.json').write_text(json.dumps(report,indent=2)+'\n')
 print(json.dumps({'ok':not fail,'cases':len(cases),'failures':len(fail),'report_written':not args.no_report}))
 if fail:print(json.dumps(fail[:1]));raise SystemExit(1)
if __name__=='__main__':main()
