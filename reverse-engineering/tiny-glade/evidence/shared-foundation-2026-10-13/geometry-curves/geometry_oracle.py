"""Private geometry fixtures from unmodified B instructions."""
import native_oracle as n
from native_oracle import *
import random,math
output=[]
def rust(v):return json.dumps(v,separators=(',',':'))
source=['use super::*;use glam::{Vec2,Vec3};','fn f(b:u32)->f32 {f32::from_bits(b)}','fn v2(b:[u32;2])->Vec2 {Vec2::new(f(b[0]),f(b[1]))}','fn v3(b:[u32;3])->Vec3 {Vec3::new(f(b[0]),f(b[1]),f(b[2]))}','fn close(actual:f32,expected:u32){if f(expected).is_nan(){assert!(actual.is_nan())}else{assert_eq!(actual.to_bits(),expected)}}']
def blob(a,values):cpu.mem_write(a,struct.pack('<'+'f'*len(values),*values))
def encoded(values):return [bits(v)for v in values]
def tri(ray,points):
    setup();blob(DATA+0x100,ray);blob(DATA+0x200,[v for p in points for v in p]);arguments(DATA,DATA+0x100,DATA+0x200);call(0x14123afa0)
    hit=u64(DATA)&0xffffffff;return floats(DATA+4,3) if hit else None
def sphere(ray,center,radius):
    setup();blob(DATA+0x100,ray);blob(DATA+0x200,center);arguments(DATA,DATA+0x100,DATA+0x200);cpu.reg_write(UC_X86_REG_XMM3,bits(radius));call(0x14123b230)
    return floats(DATA+4,2) if u64(DATA)&0xffffffff else None
def uv(positions,uvs,p):
    setup();blob(DATA+0x100,[v for t in positions for a in t for v in a]);blob(DATA+0x1000,[v for t in uvs for a in t for v in a]);arguments(DATA,DATA+0x100,len(positions));cpu.reg_write(UC_X86_REG_R9,DATA+0x1000)
    sp=STACK+0x100008;cpu.mem_write(sp+0x28,struct.pack('<Q',len(uvs)));cpu.mem_write(sp+0x30,struct.pack('<f',p[0]));cpu.mem_write(sp+0x38,struct.pack('<f',p[1]));call(0x1416f34a0)
    return floats(DATA+4,3) if u64(DATA)&0xffffffff else None
def rect3(rect,point,delta):
    setup();blob(DATA,rect);blob(DATA+0x100,point);arguments(DATA,DATA+0x100);call(0x140c985a0);inside=bool(cpu.reg_read(UC_X86_REG_RAX)&0xff)
    arguments(DATA+0x200,DATA);call(0x140c98660);extent=floats(DATA+0x200,3)
    arguments(DATA+0x300,DATA);call(0x140c98690);points=floats(DATA+0x300,24)
    blob(DATA+0x100,delta);arguments(DATA,DATA+0x100);call(0x140c98620);expanded=floats(DATA,8)
    return {'inside':inside,'extent':extent,'points':points,'expanded':expanded}
def slice2(points,start,end,circular=False):
    curve(points);arguments(DATA+0x300,DATA);cpu.reg_write(UC_X86_REG_XMM2,bits(start));cpu.reg_write(UC_X86_REG_XMM3,bits(end));call(0x1411f8750 if circular else 0x140154b60)
    out=DATA+0x300
    if u64(out)==1<<63:return {'error':cpu.mem_read(out+8,1)[0]}
    length=u64(out+16);raw=floats(u64(out+8),length*2)
    return {'points':[raw[i:i+2]for i in range(0,len(raw),2)],'length':floats(out+48,1)[0],'points_u':floats(u64(out+32),length)}
def main():
    rng=random.Random(5392)
    source.append('#[test]fn original_b_ray_triangle(){')
    rays=[[0.2,0.2,1,0,0,-1],[0.2,0.2,-1,0,0,1],[1,0,1,0,0,-1],[0,0,1,0,0,-1],[0.2,0.2,1e-5,0,0,-1],[0.2,0.2,1,0,0,0],[0.2,0.2,1,0,0,-0.00001]]
    points=[[0,0,0],[1,0,0],[0,1,0]]
    for _ in range(75):rays.append([rng.uniform(-2,2)for _ in range(6)])
    for ray in rays:
        expected=tri(ray,points);output.append({'kind':'ray_triangle','ray':encoded(ray),'triangle':[encoded(p)for p in points],'expected':expected})
        source.append('{let r=Ray{origin:v3('+rust(encoded(ray[:3]))+'),direction:v3('+rust(encoded(ray[3:]))+')};let t=intersect::Triangle::new(Vec3::ZERO,Vec3::X,Vec3::Y);let got=intersect::ray_triangle_intersection(r,t);'+('assert!(got.is_none());' if expected is None else 'let got=got.unwrap();close(got.distance,'+str(expected[0])+');close(got.u,'+str(expected[1])+');close(got.v,'+str(expected[2])+');')+'}')
    source.append('}\n#[test]fn original_b_ray_sphere(){')
    for ray in [[0,0,2,0,0,1],[0,1,-2,0,0,1],[0,0,0,0,0,1],[0,0,0,0,0,0]]+[[rng.uniform(-3,3) for _ in range(6)] for _ in range(90)]:
        radius=rng.choice([0.5,1,2,-1]);expected=sphere(ray,[0,0,0],radius);output.append({'kind':'ray_sphere','ray':encoded(ray),'radius':bits(radius),'expected':expected})
        source.append('{let r=Ray{origin:v3('+rust(encoded(ray[:3]))+'),direction:v3('+rust(encoded(ray[3:]))+')};let got=intersect::ray_sphere_intersection(r,intersect::Sphere{center:Vec3::ZERO,radius:f('+str(bits(radius))+')});'+('assert!(got.is_none());' if expected is None else 'let got=got.unwrap();close(got.near,'+str(expected[0])+');close(got.far,'+str(expected[1])+');')+'}')
    source.append('}\n#[test]fn original_b_mesh_uv(){')
    positions=[[[1,2,3],[4,5,6],[7,8,9]],[[10,11,12],[13,14,15],[16,17,18]]];uvs=[[[0,0],[1,0],[0,1]],[[0,0],[1,0],[0,1]]]
    for p in [[0,0],[1,0],[0,1],[0.25,0.5],[-0.1,0.5],[0.5,0.500001]]+[[rng.uniform(-0.2,1.2),rng.uniform(-0.2,1.2)]for _ in range(60)]:
        expected=uv(positions,uvs,p);output.append({'kind':'uv','p':encoded(p),'expected':expected})
        source.append('{let positions=[[Vec3::new(1.0,2.0,3.0),Vec3::new(4.0,5.0,6.0),Vec3::new(7.0,8.0,9.0)],[Vec3::new(10.0,11.0,12.0),Vec3::new(13.0,14.0,15.0),Vec3::new(16.0,17.0,18.0)]];let uvs=[[Vec2::ZERO,Vec2::X,Vec2::Y];2];let got=position_from_mesh_uvs::get_position_from_mesh_uv(&positions,&uvs,v2('+rust(encoded(p))+'));'+('assert!(got.is_none());' if expected is None else 'let got=got.unwrap();for (a,e)in got.to_array().into_iter().zip('+rust(expected)+'){close(a,e);}')+'}')
    source.append('}\n#[test]fn original_b_rectangle3d(){')
    for i in range(50):
        axis=[1,0] if i%2==0 else [0.6,0.8];rect=axis+[rng.uniform(-10,10),rng.uniform(-10,10)]+[rng.uniform(0.1,4),rng.uniform(0.1,4)]+[-2,3];point=[rng.uniform(-10,10),rng.choice([-2,3,0]),rng.uniform(-10,10)];delta=[rng.uniform(-1,1)for _ in range(3)];expected=rect3(rect,point,delta);output.append({'kind':'rect3','rect':encoded(rect),'point':encoded(point),'delta':encoded(delta),'expected':expected})
        source.append('{let mut r=Rectangle3d{footprint:Rectangle2d{axis:v2('+rust(encoded(rect[:2]))+'),center:v2('+rust(encoded(rect[2:4]))+'),size:v2('+rust(encoded(rect[4:6]))+')},bottom:-2.0,top:3.0};assert_eq!(r.contains_point(v3('+rust(encoded(point))+')),'+str(expected['inside']).lower()+');assert_eq!(r.extent().to_array().map(f32::to_bits),'+rust(expected['extent'])+');let got:Vec<_>=r.points().iter().flat_map(|p|p.to_array().map(f32::to_bits)).collect();assert_eq!(got,'+rust(expected['points'])+');r.expand(v3('+rust(encoded(delta))+'));let got=[r.footprint.axis.x,r.footprint.axis.y,r.footprint.center.x,r.footprint.center.y,r.footprint.size.x,r.footprint.size.y,r.bottom,r.top].map(f32::to_bits);assert_eq!(got,'+rust(expected['expanded'])+');}')
    source.append('}\n#[test]fn original_b_curve_slice(){')
    for i in range(40):
        points=[[0,0],[1,0],[1,3],[3,3]] if i<4 else [[rng.uniform(-10,10),rng.uniform(-10,10)]for _ in range(rng.randint(2,8))]
        start,end=[(0,1),(0.2,0.8),(0.5,0.7),(-0.5,2)][i] if i<4 else sorted([rng.random(),rng.random()]);expected=slice2(points,start,end);output.append({'kind':'slice2','points':[encoded(p)for p in points],'start':bits(start),'end':bits(end),'expected':expected})
        source.append('{let points='+rust([encoded(p)for p in points])+';let c=crate::curve::Curve2::try_new(points.into_iter().map(v2).collect(),false).unwrap();let got=c.try_slice(f('+str(bits(start))+'),f('+str(bits(end))+')).unwrap();assert_eq!(got.points.iter().map(|p|p.to_array().map(f32::to_bits)).collect::<Vec<_>>(),'+rust(expected['points'])+');assert_eq!(got.length.to_bits(),'+str(expected['length'])+');assert_eq!(got.points_u.iter().map(|v|v.to_bits()).collect::<Vec<_>>(),'+rust(expected['points_u'])+');}')
    source.append('}')
    source.append('#[test]fn original_b_curve_circular_slice(){')
    for i in range(20):
        points=[[0,0],[1,0],[1,3],[0,3],[0,0]]
        start,end=[(0.8,0.2),(1,0.2),(0.8,0),(0.5,0.5),(0,0.5),(1,1),(0,1),(0.2,0.8)][i] if i<8 else (rng.uniform(0.5,1),rng.uniform(0,0.5))
        expected=slice2(points,start,end,True);output.append({'kind':'circular_slice2','points':[encoded(p)for p in points],'start':bits(start),'end':bits(end),'expected':expected})
        source.append('{let points='+rust([encoded(p)for p in points])+';let c=crate::curve::Curve2::try_new(points.into_iter().map(v2).collect(),false).unwrap();let got=c.circular_slice(f('+str(bits(start))+'),f('+str(bits(end))+'));')
        if 'error' in expected:source.append('assert!(got.is_err());}')
        else:source.append('let got=got.unwrap();assert_eq!(got.points.iter().map(|p|p.to_array().map(f32::to_bits)).collect::<Vec<_>>(),'+rust(expected['points'])+');assert_eq!(got.length.to_bits(),'+str(expected['length'])+');assert_eq!(got.points_u.iter().map(|v|v.to_bits()).collect::<Vec<_>>(),'+rust(expected['points_u'])+');}')
    source.append('}')
    (OUT/'geometry-fixtures.json').write_text(json.dumps({'binary_sha256':SHA,'target_execution':False,'cases':output},indent=2)+'\n',encoding='utf-8');(OUT/'geometry_tests.rs').write_text('\n'.join(source)+'\n',encoding='utf-8');print(json.dumps({'cases':len(output),'functions':5}))
if __name__=='__main__':main()
