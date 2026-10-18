"""B ui_draw_wall full-entry state/port oracle. UI registry, border containment,
archival and the empty-ID-set fold are resolved dependency boundaries. The
original mode gates, drag dispatch, state writes and every event store execute.
Intersection tag2 deliberately excludes the water-hint renderer branch.
"""
from pathlib import Path
import json,struct,hashlib
ROOT=Path('/Users/yuluo/Desktop/game');OUT=Path(__file__).resolve().parent
recipe=ROOT/'games_test/reverse-engineering/tiny-glade/evidence/shared-foundation-2026-10-13/utils-base/native_spline_rdp.py'
s=recipe.read_text().split('def spline(')[0].replace("Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')", "Path('/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')")
exec(compile(s,str(recipe),'exec'),globals());OUT=Path(__file__).resolve().parent
at=INPUT+0x100;inside=True;ui=0;calls=[];archival=[]
def q(addr):return struct.unpack('<Q',u.mem_read(addr,8))[0]
def put(addr,*v):u.mem_write(addr,struct.pack('<'+'Q'*len(v),*v))
def new(n):
 global at
 p=at;at+=(n+15)&~15;return p
def res(p):
 r=new(32);tick=new(8);put(r,p,0,tick);u.mem_write(r+28,struct.pack('<I',99));return r
def event(stride):
 p=new(72);buf=new(stride*16);put(p+32,16,buf,0,0,0);return res(p),p,buf,stride
def deps(uc,addr,size,data):
 if addr==0x14092df50:
  sp=u.reg_read(UC_X86_REG_RSP);n=u.reg_read(UC_X86_REG_R8)
  calls.append({'name':bytes(u.mem_read(u.reg_read(UC_X86_REG_RDX),n)).decode(),'argument':u.reg_read(UC_X86_REG_R9)&255,'layer':u.mem_read(sp+40,1)[0]});ret(ui);u.reg_write(UC_X86_REG_RDX,0x1234)
 elif addr in [0x140c980f0,0x140c9a630]:ret(int(inside))
 elif addr==0x1413e7230:
  archival.append({'selector':u.reg_read(UC_X86_REG_RDX),'label_pointer':q(u.reg_read(UC_X86_REG_RSP)+48)});ret()
 elif addr==0x141293280:ret()
u.hook_add(UC_HOOK_CODE,deps)
def case(mode,subtype,state,phase,in_border,active):
 global at,inside,ui,calls,archival
 sp=reset();u.mem_write(INPUT,bytes(0x1f000));at=INPUT+0x100;inside=in_border;calls=[];archival=[]
 m=new(32);u.mem_write(m,struct.pack('<IB',mode,subtype));m=res(m)
 st=new(16);u.mem_write(st,bytes([state]));st_r=res(st)
 nursery=new(32);put(nursery,0,4,2);u.mem_write(nursery+24,b'\x01');nursery_r=res(nursery)
 record=event(8);finish=event(16);debug=event(8);bend=event(8);outside=event(8);feedback=event(8);hide=event(24)
 cursor=new(32);u.mem_write(cursor,struct.pack('<I3f',1,1,2,3));cursor=res(cursor)
 ui=new(192);u.mem_write(ui+24,bytes([2,phase]));u.mem_write(ui+40,struct.pack('<I',2));put(ui+56,0x1234);ui_r=res(ui)
 suppress=new(24);sup_r=res(suppress)
 border=new(40);border_r=res(border)
 border_tuple=new(96);u.mem_write(border_tuple,bytes(u.mem_read(border_r,32)));u.mem_write(border_tuple+32,bytes(u.mem_read(outside[0],32)));u.mem_write(border_tuple+64,bytes(u.mem_read(feedback[0],32)))
 terrain=res(new(64));stream=res(new(80))
 progress=new(176);put(progress,0 if active else 0x8000000000000000);put(progress+128,7);progress=res(progress)
 archivist=new(512);storage=new(512);archive_tuple=new(128);u.mem_write(archive_tuple,bytes(u.mem_read(res(archivist),32)));u.mem_write(archive_tuple+32,bytes(u.mem_read(res(storage),32)))
 args=[m,st_r,nursery_r,record[0],finish[0],cursor,debug[0],ui_r,sup_r,bend[0],border_tuple,terrain,stream,hide[0],progress,archive_tuple]
 for r,v in zip([UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_R9],args[:4]):u.reg_write(r,v)
 for i,v in enumerate(args[4:]):put(sp+40+8*i,v)
 run(0x140aafe40)
 outputs={}
 for name,e in [('record',record),('finish',finish),('debug',debug),('bend',bend),('outside',outside),('feedback',feedback),('hide',hide)]:
  n=q(e[1]+48);outputs[name]=[bytes(u.mem_read(e[2]+i*e[3],e[3])).hex()for i in range(n)]
 return {'mode':mode,'subtype':subtype,'initial_state':state,'phase':phase,'inside':in_border,'active':active,'state':u.mem_read(st,1)[0],'nursery_len':q(nursery+16),'nursery_kind':u.mem_read(nursery+24,1)[0],'suppressed':u.mem_read(suppress+8,1)[0],'canvases':calls.copy(),'archival':archival.copy(),'events':outputs}
if __name__=='__main__':
 cases=[case(m,sub,s,p,b,a)for m,sub in [(6,0),(18,0),(6,1),(0,0)]for s in range(4)for p in range(5)for b in [False,True]for a in [False,True]]
 report={'target':'B','binary_sha256':hashlib.sha256(raw).hexdigest(),'entry':'0x140aafe40','method':__doc__,'cases':cases}
 (OUT/'native-ui-wall.json').write_text(json.dumps(report,indent=2)+'\n');print({'cases':len(cases),'all_original_returned':True})
