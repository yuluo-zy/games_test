"""Stateful CurveSlice B differential: genuine entries and inline projections."""
from pathlib import Path
import sys,json,random,math
OUT=Path(__file__).resolve().parent
sys.path.insert(0,str(OUT.parent))
import native_oracle as n
from native_oracle import *
OUT=Path(__file__).resolve().parent
exit_pc=None;exits=set()
def run_inline(begin,end):
    global exits,exit_pc
    exits={end};exit_pc=None
    try:cpu.emu_start(begin,STOP,count=1000000)
    except RuntimeError as e:
        if 'original panic'in str(e):return True
        raise
    assert exit_pc==end,hex(cpu.reg_read(UC_X86_REG_RIP));return False
def inline_hook(c,pc,size,user):
    global exit_pc
    if pc in exits:exit_pc=pc;c.emu_stop()
    elif pc in [0x140750ebd,0x140751902,0x14121f6ca,0x14121fb22]:raise RuntimeError('original panic '+hex(pc))
cpu.hook_add(UC_HOOK_CODE,inline_hook)
def put(r,x):cpu.reg_write(r,bits(x))
def rust(v):return json.dumps(v,separators=(',',':'))
def coord(s,t):return {'segment':s,'t':bits(t)}
def extension(points,start,end,distance,direction,loop=None):
    setup();cpu.mem_write(DATA+0x1000,struct.pack('<'+'f'*len(points)*3,*[v for p in points for v in p]));s,t=start;e,u=end
    if loop is None:
        cpu.mem_write(DATA,struct.pack('<QQQIfIf',DATA+0x1000,len(points),0x141206c40,s,t,e,u));address=0x141206ed0 if direction=='left' else 0x141207c10;offset=24
    else:
        cpu.mem_write(DATA+0x100,struct.pack('<QQiiii',DATA+0x1000,len(points),*loop,0));cpu.mem_write(DATA,struct.pack('<QQIfIf',DATA+0x100,0x141206c40,s,t,e,u));address=0x141206c50 if direction=='left' else 0x141207a10;offset=16
    arguments(DATA);put(UC_X86_REG_XMM1,distance)
    try:call(address)
    except RuntimeError as error:
        if 'original panic'in str(error):return {'panic':True}
        raise
    state=list(struct.unpack('<IIII',cpu.mem_read(DATA+offset,16)))
    return {'panic':False,'expected_first':[state[0],state[1]],'expected_last':[state[2],state[3]],'address':hex(address)}
def coords(first,last):
    setup();s,t=first;e,u=last;cpu.reg_write(UC_X86_REG_RAX,e);cpu.reg_write(UC_X86_REG_RCX,s);cpu.reg_write(UC_X86_REG_RBP,DATA+0x8000);put(UC_X86_REG_XMM0,t);put(UC_X86_REG_XMM1,u)
    panic=run_inline(0x140750bb0,0x140750c15)
    if panic:return {'panic':True}
    put(UC_X86_REG_XMM3,1);put(UC_X86_REG_XMM2,0)
    run_inline(0x140750c1d,0x140750c4d)
    return {'panic':False,'inner_begin':cpu.reg_read(UC_X86_REG_RDX),'inner_end':cpu.reg_read(UC_X86_REG_R9),'first':[s,bits(t)],'last':[e,bits(u)]}
def center_range(points,index):
    setup();cpu.mem_write(DATA+0x100,struct.pack('<QQiiii',DATA+0x1000,len(points),0,len(points),len(points),0));cpu.reg_write(UC_X86_REG_R15,DATA+0x110);cpu.reg_write(UC_X86_REG_RDI,index);cpu.reg_write(UC_X86_REG_R12,index+1);cpu.reg_write(UC_X86_REG_RSI,DATA+0x100)
    panic=run_inline(0x14075168e,0x1407516f9)
    if panic:return {'panic':True}
    state=list(struct.unpack('<IIII',cpu.mem_read(STACK+0x100008+0x40,16)))
    return {'panic':False,'first':state[:2],'last':state[2:]}
def main():
    rng=random.Random(20261015);cases=[];paths=[[[0,0,0],[3,2,4],[6,4,4],[6,6,9]],[[0,0,0],[0,1,0],[1,4,0],[2,7,0]]]
    paths += [[[rng.uniform(-20,20)for _ in range(3)]for _ in range(rng.randint(2,8))]for _ in range(30)]
    for p in paths:
        for direction in ['left','right']:
            for d in [0,.01,3,100,float('inf'),-1,float('nan')]:
                s=rng.randrange(len(p)-1);e=rng.randrange(s,len(p)-1);t=rng.random();u=rng.random()if e>s else max(t,rng.random())
                result=extension(p,(s,t),(e,u),d,direction);cases.append({'points':[[bits(v)for v in q]for q in p],'first':[s,bits(t)],'last':[e,bits(u)],'distance':bits(d),'direction':direction,'loop':None,**result})
    for p in paths[:12]:
        period=len(p);loop=(-1,period,period);logical_count=period+1
        for direction in ['left','right']:
            for d in [0,3,100]:
                s=1;e=min(2,logical_count-2);t=.3;u=.8;cases.append({'points':[[bits(v)for v in q]for q in p],'first':[s,bits(t)],'last':[e,bits(u)],'distance':bits(d),'direction':direction,'loop':list(loop),**extension(p,(s,t),(e,u),d,direction,loop)})
    coordinate_cases=[]
    for s,e in [(0,0),(0,1),(2,5),(3,1)]:
        for t,u in [(0,0),(0,1),(1,0),(1,1),(.2,.8),(.8,.2),(float('nan'),0)]:coordinate_cases.append({'first':[s,bits(t)],'last':[e,bits(u)],**coords((s,t),(e,u))})
    centers=[{'count':len(p),'index':i,**center_range(p,i)}for p in paths[:4]for i in range(len(p)+1)]
    source=['use super::*;use glam::Vec3;','fn f(b:u32)->f32{f32::from_bits(b)}','fn c(v:[u32;2])->CurveInternalCoord{CurveInternalCoord{segment:v[0]as usize,t:f(v[1])}}','fn eq_coord(a:CurveInternalCoord,b:[u32;2]){assert_eq!(a.segment,b[0]as usize);if f(b[1]).is_nan(){assert!(a.t.is_nan())}else{assert_eq!(a.t.to_bits(),b[1])}}','fn points(p:&[[u32;3]])->Vec<Vec3>{p.iter().map(|v|Vec3::new(f(v[0]),f(v[1]),f(v[2]))).collect()}']
    for direction in ['left','right']:
        source.append('#[test]fn original_b_slice_extend_'+direction+'(){')
        for row in [r for r in cases if r['direction']==direction]:
            source.append('{let p=points(&'+rust(row['points'])+');')
            view='p.as_slice()'
            if row['loop']:
                a,b,period=row['loop'];source.append('let view=LoopCurvePoints{points:p.as_slice(),range:crate::loop_range::LoopRange::new_loop('+str(a)+','+str(b)+','+str(period)+')};');view='view'
            source.append('let mut slice=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range('+view+',c('+rust(row['first'])+'),c('+rust(row['last'])+')).remap_2d();')
            expr='slice.extend_'+direction+'(f('+str(row['distance'])+'))'
            if row['panic']:source.append('assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{'+expr+'})).is_err());')
            else:source.append(expr+';eq_coord(slice.start(),'+rust(row['expected_first'])+');eq_coord(slice.end(),'+rust(row['expected_last'])+');')
            source.append('}')
        source.append('}')
    source.append('#[test]fn original_b_slice_coord_ranges(){')
    for r in coordinate_cases:
        expr='CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range([Vec3::ZERO;8].as_slice(),c('+rust(r['first'])+'),c('+rust(r['last'])+'))'
        if r['panic']:source.append('assert!(std::panic::catch_unwind(||{'+expr+'}).is_err());')
        else:
            expected=[r['first']]+[[i,0]for i in range(r['inner_begin'],r['inner_end'])]+[r['last']]
            source.append('{let p=[Vec3::ZERO;8];let s=CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_coord_range(p.as_slice(),c('+rust(r['first'])+'),c('+rust(r['last'])+'));let got=s.iter_coords().map(|p|[p.segment as u32,p.t.to_bits()]).collect::<Vec<_>>();assert_eq!(got,vec!'+rust(expected)+');}')
    source.append('}\n#[test]fn original_b_slice_point_centers(){')
    for r in centers:
        expr='CurveSlice::<Vec3,_,super::super::CurveCoordSemantic2D>::from_point_range(p.as_slice(),'+str(r['index'])+'..='+str(r['index'])+')'
        source.append('{let p=vec![Vec3::ZERO;'+str(r['count'])+'];')
        if r['panic']:source.append('assert!(std::panic::catch_unwind(||{'+expr+'}).is_err());')
        else:source.append('let s='+expr+';eq_coord(s.start(),'+rust(r['first'])+');eq_coord(s.end(),'+rust(r['last'])+');')
        source.append('}')
    source.append('}')
    payload={'original_sha256':SHA,'private_text_unmodified':True,'whole_game_execution':False,'extension_cases':cases,'coordinate_cases':coordinate_cases,'point_center_cases':centers,'scope_limits':['Extensions compare XY metric after original Vec3 XZ remap, ordinary/LoopSlice donors.','Original Thresholded3D sampling strategy is not implied recovered.','Coordinates use explicit inline caller registers; constructor range bound variants not all emitted.']}
    (OUT/'slice-native-fixtures.json').write_text(json.dumps(payload,indent=2)+'\n');(OUT/'slice-native-tests.rs').write_text('\n'.join(source)+'\n');print(json.dumps({'extensions':len(cases),'coordinate_ranges':len(coordinate_cases),'point_centers':len(centers)}))
if __name__=='__main__':main()
