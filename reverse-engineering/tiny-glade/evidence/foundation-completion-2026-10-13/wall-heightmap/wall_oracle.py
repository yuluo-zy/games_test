"""Original WallSpace helpers used by the observed coordinate inline wrappers."""
from pathlib import Path
import sys,json,struct,math
OUT=Path(__file__).resolve().parent;ROOT=OUT.parents[2]
sys.path.insert(0,str(ROOT/'evidence/shared-foundation-2026-10-13/geometry-curves'))
import native_oracle as n
from unicorn import UC_HOOK_CODE
from unicorn.x86_const import *
def f(b):return struct.unpack('<f',struct.pack('<I',b))[0]
def fmod_bridge(cpu,pc,size,user):
    if pc==0x142923f10:
        a,b=(f(cpu.reg_read(r)&0xffffffff)for r in (UC_X86_REG_XMM0,UC_X86_REG_XMM1))
        try:v=math.fmod(a,b)
        except ValueError:v=float('nan')
        cpu.reg_write(UC_X86_REG_XMM0,n.bits(v));n.ret()
n.cpu.hook_add(UC_HOOK_CODE,fmod_bridge)
def query(loop,length,value,addr):
    n.setup();n.cpu.mem_write(n.DATA,struct.pack('<If',int(loop),length));n.arguments(n.DATA)
    n.cpu.reg_write(UC_X86_REG_XMM1,n.bits(value));n.call(addr)
    return n.cpu.reg_read(UC_X86_REG_XMM0)&0xffffffff,n.cpu.reg_read(UC_X86_REG_RAX)&1
src=['use super::*;','fn f(b:u32)->f32{f32::from_bits(b)}',
     'fn equal(a:f32,b:u32){if f(b).is_nan(){assert!(a.is_nan());}else{assert_eq!(a.to_bits(),b);}}',
     '#[test]fn original_wall_projection_wrapper_policies(){']
rows=[]
for loop in (False,True):
 for length in (0.,1.,7.5):
  for u in (-15.,-7.5,-1.,-0.,0.,0.5,1.,7.5,8.,15.,float('nan'),float('inf')):
   added=f(n.bits(0.0+u))
   bounded,present=query(loop,length,added,0x140c91e40)
   clamped,_=query(loop,length,added,0x140c91eb0)
   new_clamped,_=query(loop,length,u,0x140c91eb0)
   unbounded,_=query(loop,length,added,0x140c91f20)
   rows.append({'is_loop':loop,'length':n.bits(length),'u':n.bits(u),'project_present':present,'project':bounded,'clamped':clamped,'unbounded':unbounded})
   src.append('{let space=WallSpace{is_loop:'+str(loop).lower()+',length:f('+str(n.bits(length))+')};let c=WallSpaceCoord::new_unchecked(0.,2.);let d=Vec2::new(f('+str(n.bits(u))+'),3.);')
   src.append('let got=c.with_offset(&space,d);assert_eq!(got.is_some(),'+str(bool(present)).lower()+');')
   if present:src.append('equal(got.unwrap().u,'+str(bounded)+');assert_eq!(got.unwrap().y,5.);')
   src.append('equal(c.with_offset_clamped(&space,d).u,'+str(clamped)+');equal(WallSpaceCoord::new(&space,d.x,2.).u,'+str(new_clamped)+');equal(c.with_offset_unbounded(&space,d).u,'+str(unbounded)+');}')
src.append('}')
(OUT/'wall-fixtures.json').write_text(json.dumps({'original_binary_sha256':n.SHA,'inline_mapping':['new -> project_clamp','with_offset -> project','with_offset_clamped -> project_clamp','with_offset_unbounded -> project_unbounded'],'bridge':'CRT fmodf delegates to host math.fmod, NaN payload excluded','cases':rows},indent=2)+'\n',encoding='utf-8')
(OUT/'wall_tests.rs').write_text('\n'.join(src)+'\n',encoding='utf-8')
print(json.dumps({'cases':len(rows)}))
