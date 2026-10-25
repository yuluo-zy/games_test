"""Execute untouched B private leaves in Unicorn; never launch the game."""
from pathlib import Path
import json,struct,hashlib,random
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
OUT=Path(__file__).resolve().parent
ROOT=OUT.parents[2]
raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
SHA='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
assert hashlib.sha256(raw).hexdigest()==SHA
pe=pefile.PE(data=raw);base=pe.OPTIONAL_HEADER.ImageBase
cpu=Uc(UC_ARCH_X86,UC_MODE_64);cpu.mem_map(base,(pe.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);cpu.mem_write(base,pe.get_memory_mapped_image())
DATA,STACK,HEAP,STOP=0x20000000,0x30000000,0x40000000,0x50000000
cpu.mem_map(DATA,0x100000);cpu.mem_map(STACK,0x200000);cpu.mem_map(HEAP,0x1000000);cpu.mem_map(STOP,0x1000)
heap=HEAP
def u64(a):return struct.unpack('<Q',cpu.mem_read(a,8))[0]
def bits(x):return struct.unpack('<I',struct.pack('<f',x))[0]
def floats(a,n):return list(struct.unpack('<'+'I'*n,cpu.mem_read(a,n*4)))
def ret(v=None):
    if v is not None:cpu.reg_write(UC_X86_REG_RAX,v)
    sp=cpu.reg_read(UC_X86_REG_RSP);pc=u64(sp);cpu.reg_write(UC_X86_REG_RSP,sp+8);cpu.reg_write(UC_X86_REG_RIP,pc)
def hook(c,pc,size,user):
    global heap
    if pc in (0x140613c10,0x140613c40):
        n=c.reg_read(UC_X86_REG_RCX);p=heap;heap+=max((n+15)&~15,0x1000);assert heap<HEAP+0x1000000
        if pc==0x140613c40 and n:c.mem_write(p,bytes(n))
        ret(p)
    elif pc in (0x140613c20,0x14289aa40):ret()
    elif pc==0x140613c30:
        old,n,new=c.reg_read(UC_X86_REG_RCX),c.reg_read(UC_X86_REG_RDX),c.reg_read(UC_X86_REG_R9);p=heap;heap+=max((new+15)&~15,0x1000);c.mem_write(p,bytes(c.mem_read(old,min(n,new))));ret(p)
    elif pc in (0x142923c00,0x142923c10):
        dst,src,n=[c.reg_read(r) for r in (UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8)]
        if n:c.mem_write(dst,bytes(c.mem_read(src,n)))
        ret(dst)
    elif pc==0x142923c20:
        dst,v,n=[c.reg_read(r) for r in (UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8)]
        if n:c.mem_write(dst,bytes([v&255])*n)
        ret(dst)
    elif 0x1428d8f00<=pc<=0x1428f1800:raise RuntimeError('original panic '+hex(pc))
cpu.hook_add(UC_HOOK_CODE,hook)
def setup():
    global heap
    heap=HEAP;cpu.mem_write(DATA,bytes(0x10000));cpu.mem_write(STACK,bytes(0x200000))
    for r in (UC_X86_REG_RAX,UC_X86_REG_RBX,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_RSI,UC_X86_REG_RDI,UC_X86_REG_RBP,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,UC_X86_REG_R13,UC_X86_REG_R14,UC_X86_REG_R15):cpu.reg_write(r,0)
    for r in range(UC_X86_REG_XMM0,UC_X86_REG_XMM15+1):cpu.reg_write(r,0)
    sp=STACK+0x100008;cpu.mem_write(sp,struct.pack('<Q',STOP));cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_MXCSR,0x1f80);cpu.reg_write(UC_X86_REG_RFLAGS,0x202)
def call(a):
    sp=STACK+0x100008;cpu.mem_write(sp,struct.pack('<Q',STOP));cpu.reg_write(UC_X86_REG_RSP,sp)
    cpu.emu_start(a,STOP,count=1000000)
    assert cpu.reg_read(UC_X86_REG_RIP)==STOP,hex(cpu.reg_read(UC_X86_REG_RIP))
def arguments(a,b=0,c=0):
    for r,v in zip((UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8),(a,b,c)):cpu.reg_write(r,v)
def curve(points,dim=2,projected=False,reject=False):
    flat=[v for p in points for v in p];setup();cpu.mem_write(DATA+0x1000,struct.pack('<'+'f'*len(flat),*flat));cpu.mem_write(DATA+0x100,struct.pack('<QQQ',len(points),DATA+0x1000,len(points)));arguments(DATA,DATA+0x100,int(reject));call(0x141469430 if dim==2 else 0x1412699d0 if projected else 0x141269ce0)
    out=bytes(cpu.mem_read(DATA,56));tag=struct.unpack_from('<Q',out)[0]
    if tag==1<<63:return {'error':out[8], 'detail':out[9] if out[8]==0 else struct.unpack_from('<Q',out,16)[0] if out[8]==2 else None}
    return {'length':floats(DATA+48,1)[0],'points_u':floats(u64(DATA+32),len(points))}
def sample(u,dim=2):
    def run(a):arguments(DATA);cpu.reg_write(UC_X86_REG_XMM1,bits(u));call(a)
    run(0x141467ce0);i=cpu.reg_read(UC_X86_REG_RAX)&0xffffffff;t=cpu.reg_read(UC_X86_REG_XMM0)&0xffffffff
    run(0x141467270);pos=[cpu.reg_read(r)&0xffffffff for r in (UC_X86_REG_XMM0,UC_X86_REG_XMM1)]
    run(0x141468f90);tan=[cpu.reg_read(r)&0xffffffff for r in (UC_X86_REG_XMM0,UC_X86_REG_XMM1)]
    return {'u':bits(u),'index':i,'t':t,'pos':pos,'tangent':tan}
def nearest(point):
    arguments(DATA);cpu.reg_write(UC_X86_REG_XMM1,bits(point[0]));cpu.reg_write(UC_X86_REG_XMM2,bits(point[1]));call(0x14146a060);i=cpu.reg_read(UC_X86_REG_RAX)&0xffffffff;t=cpu.reg_read(UC_X86_REG_XMM0)&0xffffffff
    arguments(DATA);cpu.reg_write(UC_X86_REG_XMM1,bits(point[0]));cpu.reg_write(UC_X86_REG_XMM2,bits(point[1]));call(0x141469a20)
    return {'point':[bits(v) for v in point],'index':i,'t':t,'u':cpu.reg_read(UC_X86_REG_XMM0)&0xffffffff}
def look_at(translation,target,up):
    setup();cpu.mem_write(DATA,struct.pack('<10f',0,0,0,1,*translation,2,3,4));cpu.mem_write(DATA+0x100,struct.pack('<3f',*target));cpu.mem_write(DATA+0x200,struct.pack('<3f',*up));arguments(DATA,DATA+0x100,DATA+0x200);call(0x140c9ce00)
    return {'translation':[bits(v) for v in translation],'target':[bits(v) for v in target],'up':[bits(v) for v in up],'rotation':floats(DATA,4),'preserved':floats(DATA+16,6)}
def main():
    rng=random.Random(20261013);rows=[]
    paths=[[[0,0],[3,4],[6,4]],[[0,0],[0,0],[1,0]],[[0,0],[1e-6,0],[2e-6,0]],[[0,0],[1,0],[0,0]],[[1,1],[2,2],[3,0],[8,-2]]]
    for _ in range(36):paths.append([[rng.uniform(-20,20),rng.uniform(-20,20)] for _ in range(rng.randint(2,10))])
    for points in paths:
        built=curve(points);row={'points':[[bits(v) for v in p] for p in points],**built,'samples':[],'nearest':[]}
        for u in [-2,0,0.01,0.25,0.5,0.99,1,2]:row['samples'].append(sample(u))
        for _ in range(8):row['nearest'].append(nearest([rng.uniform(-25,25),rng.uniform(-25,25)]))
        rows.append(row)
    errors=[]
    for points,reject in [([],False),([[0,0]],False),([[0,0],[0,0]],False),([[0,0],[0,0],[1,0]],True),([[0,0],[1,0],[1,0]],True),([[0,0],[float('nan'),0]],False),([[0,0],[float('inf'),0]],False)]:errors.append({'points':[[bits(v) for v in p]for p in points],'reject':reject,**curve(points,reject=reject)})
    curves3=[]
    for projected in (False,True):
        for _ in range(40):
            points=[[rng.uniform(-20,20) for _ in range(3)] for _ in range(rng.randint(2,8))]
            curves3.append({'points':[[bits(v)for v in p]for p in points],'projected':projected,**curve(points,3,projected)})
    rotations=[look_at([rng.uniform(-20,20) for _ in range(3)],[rng.uniform(-20,20)for _ in range(3)],[rng.uniform(-1,1)for _ in range(3)])for _ in range(100)]
    payload={'identity':SHA,'target_execution':False,'bridges':['allocator/free/realloc','memcpy/memmove/memset'],'curves2':rows,'curve_errors':errors,'curves3':curves3,'look_at':rotations}
    (OUT/'native-fixtures.json').write_text(json.dumps(payload,indent=2)+'\n',encoding='utf-8')
    source=['use super::*;','fn f(bits:u32)->f32 {f32::from_bits(bits)}','fn v2(b:[u32;2])->Vec2 {Vec2::new(f(b[0]),f(b[1]))}','fn v3(b:[u32;3])->Vec3 {Vec3::new(f(b[0]),f(b[1]),f(b[2]))}', '#[test]fn original_b_curve2_rules(){']
    def rust(value):return json.dumps(value,separators=(',',':')).replace('true','true').replace('false','false')
    for row in rows:
        source.append('{let points='+rust(row['points'])+';let c=Curve2::try_new(points.into_iter().map(v2).collect(),false).unwrap();')
        source.append('assert_eq!(c.length.to_bits(),'+str(row['length'])+');assert_eq!(c.points_u.iter().map(|v|v.to_bits()).collect::<Vec<_>>(),'+rust(row['points_u'])+');')
        for sample_row in row['samples']:
            source.append('{let u=f('+str(sample_row['u'])+');let coord=c.get_coord_at_u(u);assert_eq!((coord.segment,coord.t.to_bits()),('+str(sample_row['index'])+','+str(sample_row['t'])+'));let p=c.get_pos_at_u(u);assert_eq!(p.to_array().map(f32::to_bits),'+rust(sample_row['pos'])+');let t=c.get_tangent_at_u(u);assert_eq!(t.to_array().map(f32::to_bits),'+rust(sample_row['tangent'])+');}')
        for near_row in row['nearest']:
            source.append('{let p=v2('+rust(near_row['point'])+');let coord=c.get_approx_coord_from_pos(p);assert_eq!((coord.segment,coord.t.to_bits()),('+str(near_row['index'])+','+str(near_row['t'])+'));assert_eq!(c.get_approx_u_from_pos(p).to_bits(),'+str(near_row['u'])+');}')
        source.append('}')
    source.append('}\n#[test]fn original_b_curve3_arc_length(){')
    for row in curves3:
        source.append('{let points='+rust(row['points'])+';let c='+('Curve3XZ' if row['projected'] else 'Curve3')+'::try_new(points.into_iter().map(v3).collect(),false).unwrap();assert_eq!(c.length.to_bits(),'+str(row['length'])+');assert_eq!(c.points_u.iter().map(|v|v.to_bits()).collect::<Vec<_>>(),'+rust(row['points_u'])+');}')
    source.append('}\n#[test]fn original_b_curve_errors(){')
    for row in errors:
        points='vec!['+','.join('v2('+rust(p)+')' for p in row['points'])+']'
        error='CurveError::'+('TooFewPoints('+str(row['detail'])+')' if row['error']==0 else 'NonFiniteLength' if row['error']==1 else 'DuplicateSegment('+str(row['detail'])+')')
        source.append('assert_eq!(Curve2::try_new('+points+','+str(row['reject']).lower()+').unwrap_err(),'+error+');')
    source.append('}\n#[test]fn original_b_transform_look_at(){')
    for row in rotations:
        source.append('{let mut t=crate::components::transform::Transform{t:v3('+rust(row['translation'])+'),r:glam::Quat::IDENTITY,s:Vec3::new(2.0,3.0,4.0)};t.look_at(v3('+rust(row['target'])+'),v3('+rust(row['up'])+'));assert_eq!(t.r.to_array().map(f32::to_bits),'+rust(row['rotation'])+');assert_eq!(t.t.to_array().map(f32::to_bits),'+rust(row['translation'])+');assert_eq!(t.s,Vec3::new(2.0,3.0,4.0));}')
    source.append('}')
    (OUT/'native_tests.rs').write_text('\n'.join(source)+'\n',encoding='utf-8')
    print(json.dumps({'curves2':len(rows),'sample_entries':sum(len(r['samples']) for r in rows),'nearest_entries':sum(len(r['nearest']) for r in rows),'curve_errors':len(errors),'curves3':len(curves3),'look_at':len(rotations)}))
if __name__=='__main__':main()
