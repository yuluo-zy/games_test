"""B complete original segment cell enumeration, not a resolved AABB oracle."""
import hashlib,json,math,random,struct,subprocess,sys
from pathlib import Path
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
from verification_provenance import ROOT,candidate_provenance
def bits(f):return struct.unpack('<I',struct.pack('<f',f))[0]
def main(version="B"):
 b=json.loads((ROOT/'tracking/baselines.json').read_text())['baselines'][version];raw=Path(b['binary']['path']).read_bytes();assert hashlib.sha256(raw).hexdigest()==b['binary']['sha256'];pe=b['pe'];base=pe['image_base'];cpu=Uc(UC_ARCH_X86,UC_MODE_64);end=max(s['rva']+max(s['raw_size'],s['virtual_size']) for s in pe['sections']);cpu.mem_map(base,(end+4095)&~4095)
 for s in pe['sections']:
  if s['raw_size']:cpu.mem_write(base+s['rva'],raw[s['raw_pointer']:s['raw_pointer']+s['raw_size']])
 entry,coordinate,floor={"A":(0x141045ff0,0x1405d36f0,0x142144d70),"B":(0x1409ef870,0x140aa6520,0x142923ee0),"C":(0x140b3e930,0x140ac2d90,0x142e1be00)}[version]
 data,stack,stop=0x20000000,0x30000000,0x40000000;cpu.mem_map(data,0x20000);cpu.mem_map(stack,0x10000);cpu.mem_map(stop,0x1000);cpu.mem_write(data+0x48,struct.pack('<IIff',64,64,130.,130.));cpu.mem_write(data+0x1000,struct.pack('<QQQ',4096,data+0x2000,0))
 def hook(uc,address,size,user):
  if address==floor:
   u=cpu.reg_read(UC_X86_REG_XMM0)&0xffffffff;f=struct.unpack('<f',struct.pack('<I',u))[0]
   if math.isfinite(f):u=bits(float(math.floor(f)))
   cpu.reg_write(UC_X86_REG_XMM0,u);sp=cpu.reg_read(UC_X86_REG_RSP);ra=struct.unpack('<Q',cpu.mem_read(sp,8))[0];cpu.reg_write(UC_X86_REG_RSP,sp+8);cpu.reg_write(UC_X86_REG_RIP,ra)
 cpu.hook_add(UC_HOOK_CODE,hook);rng=random.Random(20261013);lines=[];want=[];groups=[]
 def reset():
  cpu.mem_write(stack,bytes(0x10000));cpu.reg_write(UC_X86_REG_RSP,stack+0x8008);cpu.mem_write(stack+0x8008,struct.pack('<Q',stop));cpu.reg_write(UC_X86_REG_RCX,data);cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
  for reg in [UC_X86_REG_RAX,UC_X86_REG_RDX,UC_X86_REG_RSI,UC_X86_REG_RDI,UC_X86_REG_RBP,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,UC_X86_REG_R13,UC_X86_REG_R14,UC_X86_REG_R15]:cpu.reg_write(reg,0)
 for i in range(256):
  e=[-math.inf,-100.,-65.,-64.999,0.,-0.,1.,64.999,65.,100.,math.inf,math.nan];x=e[i%len(e)] if i<144 else rng.uniform(-100,100);z=e[(i//len(e))%len(e)] if i<144 else rng.uniform(-100,100);reset();cpu.reg_write(UC_X86_REG_XMM1,bits(x));cpu.reg_write(UC_X86_REG_XMM2,bits(z));cpu.emu_start(coordinate,stop,count=10000);want.append(f'{cpu.reg_read(UC_X86_REG_EAX)-(1<<32) if cpu.reg_read(UC_X86_REG_EAX)&(1<<31) else cpu.reg_read(UC_X86_REG_EAX)} {cpu.reg_read(UC_X86_REG_EDX)-(1<<32) if cpu.reg_read(UC_X86_REG_EDX)&(1<<31) else cpu.reg_read(UC_X86_REG_EDX)}');lines.append(f'coord {bits(x):08x} {bits(z):08x}');groups.append('coordinates')
 fixed=[(0,0,0,0),(-10,0,10,0),(0,-10,0,10),(-10,-10,10,10),(-10,10,10,-10),(-65,0,0,0),(65,0,0,0),(0,0,0,65),(-64.99,-64.99,64.99,64.99),(-2.03125,-2.03125,2.03125,2.03125),(0,0,1e-8,1e-8)]
 for i in range(256):
  p=fixed[i%len(fixed)] if i<44 else tuple(rng.uniform(-64.9,64.9) for _ in range(4));p=tuple(struct.unpack('<f',struct.pack('<f',x))[0] for x in p);radius=bits(.28);reset();cpu.mem_write(data+0x1010,struct.pack('<Q',0));cpu.reg_write(UC_X86_REG_XMM1,bits(p[0]));cpu.reg_write(UC_X86_REG_XMM2,bits(p[1]));cpu.reg_write(UC_X86_REG_XMM3,bits(p[2]));cpu.mem_write(stack+0x8008+0x28,struct.pack('<I',bits(p[3])));cpu.mem_write(stack+0x8008+0x30,struct.pack('<I',radius));cpu.mem_write(stack+0x8008+0x38,struct.pack('<Q',data+0x1000));cpu.emu_start(entry,stop,count=1000000);assert cpu.reg_read(UC_X86_REG_RIP)==stop;n=struct.unpack('<Q',cpu.mem_read(data+0x1010,8))[0];cells=struct.unpack('<'+'Q'*n,cpu.mem_read(data+0x2000,n*8)) if n else [];want.append(','.join(str(c) for c in cells));lines.append('segment '+' '.join(f'{bits(x):08x}' for x in p)+f' {radius:08x}');groups.append('segment')
 got=subprocess.run([str(ROOT/'reconstruction/target/release/wall-spatial-probe')],input='\n'.join(lines)+'\n',capture_output=True,text=True,check=True).stdout.splitlines();assert len(got)==len(want);fail=[{'case':i,'group':groups[i],'input':lines[i],'expected':e,'actual':a} for i,(a,e) in enumerate(zip(got,want)) if a!=e];report={'ok':not fail,'case_count':512,'coordinate_cases':256,'whole_segment_cases':256,'failure_count':len(fail),'failures':fail[:10],'baseline':version,'segment_entry':hex(entry),'coordinate_entry':hex(coordinate),'binary_sha256':b['binary']['sha256'],'scope':'Whole original add_segment-specialized cell enumeration including endpoint guards, candidate bounds, Rectangle2d construction, cell-local transforms/Aabb intersection and ordered output; not original HashMap/Vec storage or metadata height associations.','dependency_boundary':'CRT floorf bridge only; original geometry business helpers all executed. Vec buffer preallocated with capacity4096.',**candidate_provenance()};(ROOT/f'evidence/building-collection/{version}-spatial-index-verification.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'baseline':version,'ok':not fail,'cases':512,'failures':len(fail)}));assert not fail
if __name__=='__main__':
 for version in (['A','B','C'] if len(sys.argv)>1 and sys.argv[1]=='--all' else [sys.argv[1] if len(sys.argv)>1 else 'B']):main(version)
