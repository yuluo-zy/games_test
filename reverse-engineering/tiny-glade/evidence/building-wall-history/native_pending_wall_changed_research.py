"""Native-only pending free-wall growth producer and actual deferred recording."""
import sys,struct,json,hashlib
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parent))
import native_wall_history as n
from unicorn.x86_const import *
cpu=n.cpu;nf=n.nf;data=n.data;author=n.author
def curve_source(p,points):
 # Curve arrays and scalars are explicit resolved typed caller data; this is
 # not an expected PendingWallChanged or a private output callback.
 lengths=[((b[0]-a[0])**2+(b[1]-a[1])**2)**.5 for a,b in zip(points,points[1:])];total=sum(lengths);us=[0.];run=0.
 for length in lengths:run+=length;us.append(run/total)
 point=nf.alloc(len(points)*8);ups=nf.alloc(len(points)*4);cpu.mem_write(point,b''.join(struct.pack('<2f',*v)for v in points));cpu.mem_write(ups,b''.join(struct.pack('<f',u)for u in us));cpu.mem_write(p,struct.pack('<6Qf',len(points),point,len(points),len(points),ups,len(points),total)+bytes(4)+struct.pack('<I',2)+bytes(24)+struct.pack('<3fI',3.2,.7,2.8,0x12345678)+bytes([197,1,2,31,1,5])+bytes(6))
def run(old_points,next_points,start):
 author.reset();old=data+0x1000;next=data+0x1100;pending=data+0x2000;curve_source(old,old_points);curve_source(next,next_points)
 sp=n.stack+0x100008;cpu.mem_write(sp,struct.pack('<Q',n.stop));cpu.mem_write(sp+0x28,struct.pack('<Q',int(start)));cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_RCX,pending);cpu.reg_write(UC_X86_REG_RDX,9);cpu.reg_write(UC_X86_REG_R8,old);cpu.reg_write(UC_X86_REG_R9,next);author.active=True
 try:cpu.emu_start(0x1408dc9f0,n.stop,count=1000000);assert cpu.reg_read(UC_X86_REG_RIP)==n.stop
 finally:author.active=False
 present=nf.u64(pending)!=1<<63;result={'actual_pending_return':True,'present':present}
 history=data+0x3000;hres=data+0x3100;walls=data+0x4000;wres=data+0x4300;cpu.mem_write(history+0x30,struct.pack('<3Q',128,data+0x5000,0));cpu.mem_write(history+0x58,struct.pack('<QQii',1,2,-1,-1));cpu.mem_write(walls+0x120,bytes(cpu.mem_read(pending,244)))
 if present:result.update(previous=n.snapshot(pending),next=n.snapshot(pending+112),wall=nf.u64(pending+224),operation=bytes(cpu.mem_read(pending+232,12)).hex())
 for r,p in[(hres,history),(wres,walls)]:cpu.mem_write(r,struct.pack('<QQQI',p,0,r+0x60,0)+struct.pack('<I',0x12345678))
 cpu.reg_write(UC_X86_REG_RCX,hres);cpu.reg_write(UC_X86_REG_RDX,wres);author.active=True
 try:author.call(0x1408dccf0)
 finally:author.active=False
 result['actual_deferred_return']=True;result['pending_cleared']=nf.u64(walls+0x120)==1<<63;count=nf.u64(history+0x40);result['history_count']=count;result['groups']=n.compound(data+0x5000)if count else[];return result
if __name__=='__main__':
 cases=[]
 for start in[False,True]:
  for extend in[0.,.2,1.,5.]:
   for old in[[[0.,0.],[2.,0.]],[[0.,0.],[1.,1.],[2.,0.]],[[0.,0.],[0.,0.],[2.,0.]]]:
    new=[[-extend,0.]]+old if start and extend else old+[[2.+extend,0.]]if not start and extend else old
    r=run(old,new,start);assert r['actual_pending_return']and r['actual_deferred_return']and r['pending_cleared'];assert r['history_count']==int(r['present']);cases.append({'start':start,'extension':extend,'old_points':old,'next_points':new,'native':r})
 root=Path(__file__).resolve().parents[2];paths=[Path(__file__),root/'evidence/building-wall-history/native_wall_history.py',root/'evidence/building-authoring/native_authoring.py',root/'evidence/building-constructor/native_fixture.py'];report={'ok':True,'cases':cases,'pending_returns':len(cases),'deferred_returns':len(cases),'original_binary_sha256':nf.b['binary']['sha256'],'oracle_sources':{str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest()for p in paths},'scope':'Original fullPendingWallChanged.try_new andhistory_record_wall_changed actualRET. Explicit finite Curve DTO inputarrays include duplicatepoints; no reconstructedPending result injection. Not a Rust proof ororiginalUI selection/schedule recovery.'};Path(__file__).with_name('B-pending-wall-changed-native-research.json').write_text(json.dumps(report,indent=2)+'\n');print({'ok':True,'pending_returns':len(cases),'deferred_returns':len(cases)})
