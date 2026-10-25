import native_oracle as n
from native_oracle import *
import random
def rust(v):return json.dumps(v,separators=(',',':'))
def enc(v):return [bits(x)for x in v]
def blob(a,v):cpu.mem_write(a,struct.pack('<'+'f'*len(v),*v))
source=['use super::*;use glam::{Vec2,Vec3};','fn f(b:u32)->f32 {f32::from_bits(b)}','fn v2(b:[u32;2])->Vec2 {Vec2::new(f(b[0]),f(b[1]))}','fn v3(b:[u32;3])->Vec3 {Vec3::new(f(b[0]),f(b[1]),f(b[2]))}','fn eq(a:f32,b:u32){if f(b).is_nan(){assert!(a.is_nan())}else{assert_eq!(a.to_bits(),b)}}'];cases=[]
def box3(ray,end):
    setup();blob(DATA+0x100,[-1,-2,-3,1,2,3]);blob(DATA+0x200,ray);blob(DATA+0x300,end);arguments(DATA,DATA+0x100,DATA+0x200);call(0x141240860);classification=floats(DATA,3)
    arguments(DATA,DATA+0x100,DATA+0x200);cpu.reg_write(UC_X86_REG_R9,DATA+0x300);call(0x141240570);segment=floats(DATA+4,3)if u64(DATA)&0xffffffff else None
    return {'classification':classification,'segment':segment}
def plane(point,normal,ray,min_t):
    setup();blob(DATA+0x100,point+normal);blob(DATA+0x200,ray);arguments(DATA,DATA+0x100,DATA+0x200);cpu.reg_write(UC_X86_REG_XMM3,bits(min_t));call(0x1412803f0);return floats(DATA+4,4)if u64(DATA)&0xffffffff else None
def line(a,b):
    setup();blob(DATA+0x100,a);blob(DATA+0x200,b);arguments(DATA,DATA+0x100,DATA+0x200);call(0x14261f470);tag=u64(DATA)&0xffffffff;return {'tag':tag,'point':floats(DATA+4,2)if tag==0 else None}
def barycentric(point,a,b,c):
    setup();blob(DATA+0x100,point);blob(DATA+0x200,a);blob(DATA+0x300,b);blob(DATA+0x400,c);arguments(DATA,DATA+0x100,DATA+0x200);cpu.reg_write(UC_X86_REG_R9,DATA+0x300);cpu.mem_write(STACK+0x100008+0x28,struct.pack('<Q',DATA+0x400));call(0x1416c2f20);return floats(DATA,3)
def main():
    rng=random.Random(65465);source.append('#[test]fn original_b_aabb3_ray_segment(){')
    rays=[[0,0,0,1,0,0],[-1,0,0,1,0,0],[1,0,0,0,1,0],[3,0,0,1,0,0],[0,0,0,0,0,0],[2,3,4,-1,-1,-1]]+[[rng.uniform(-5,5)for _ in range(6)]for _ in range(64)]
    for ray in rays:
        end=[ray[i]+ray[i+3]for i in range(3)];expected=box3(ray,end);cases.append({'kind':'aabb3','ray':enc(ray),'end':enc(end),'expected':expected});tag=expected['classification'][0]
        source.append('{let b=Aabb3{min:Vec3::new(-1.0,-2.0,-3.0),max:Vec3::new(1.0,2.0,3.0)};let ray=Ray{origin:v3('+rust(enc(ray[:3]))+'),direction:v3('+rust(enc(ray[3:]))+')};let got=b.classify_ray_intersection(ray);')
        if tag==0:source.append('assert!(matches!(got,RayBoxIntersection::NoHit));')
        else:source.append('match got{RayBoxIntersection::'+('Inside'if tag==1 else 'Hit')+'{enter,exit}=>{eq(enter,'+str(expected['classification'][1])+');eq(exit,'+str(expected['classification'][2])+');},_=>panic!("classification mismatch")}')
        source.append('let p=b.intersect_segment(ray.origin,v3('+rust(enc(end))+'));'+('assert!(p.is_none());'if expected['segment']is None else 'for (a,e)in p.unwrap().to_array().into_iter().zip('+rust(expected['segment'])+'){eq(a,e)}')+'}')
    source.append('}\n#[test]fn original_b_plane_ray(){')
    for i in range(70):
        point=[rng.uniform(-5,5)for _ in range(3)];normal=[rng.uniform(-1,1)for _ in range(3)];ray=[rng.uniform(-5,5)for _ in range(6)];min_t=rng.choice([-1,0,1]);expected=plane(point,normal,ray,min_t);cases.append({'kind':'plane','point':enc(point),'normal':enc(normal),'ray':enc(ray),'min_t':bits(min_t),'expected':expected});source.append('{let p=Plane{point:v3('+rust(enc(point))+'),normal:v3('+rust(enc(normal))+')};let r=Ray{origin:v3('+rust(enc(ray[:3]))+'),direction:v3('+rust(enc(ray[3:]))+')};let got=p.intersect_ray(r,f('+str(bits(min_t))+'));'+('assert!(got.is_none());'if expected is None else 'let(t,p)=got.unwrap();eq(t,'+str(expected[0])+');for (a,e)in p.to_array().into_iter().zip('+rust(expected[1:])+'){eq(a,e)}')+'}')
    source.append('}\n#[test]fn original_b_line_relation(){')
    for i in range(70):
        a=[0,0,1,1,0,1]if i<3 else [rng.uniform(-5,5)for _ in range(4)]+[0,1];b=[[1,1,2,2,0,1],[0,1,1,2,0,1],[0,1,1,0,0,1]][i]if i<3 else [rng.uniform(-5,5)for _ in range(4)]+[0,1];expected=line(a,b);cases.append({'kind':'line','a':enc(a),'b':enc(b),'expected':expected});source.append('{let a=line_relation::LineInterval::line_segment(v2('+rust(enc(a[:2]))+'),v2('+rust(enc(a[2:4]))+'));let b=line_relation::LineInterval::line_segment(v2('+rust(enc(b[:2]))+'),v2('+rust(enc(b[2:4]))+'));let got=a.relate(b);')
        if expected['tag']==0:source.append('match got{line_relation::LineRelation::Intersection(p)=>{for(a,e)in p.to_array().into_iter().zip('+rust(expected['point'])+'){eq(a,e)}},_=>panic!("intersection mismatch")}')
        else:source.append('assert_eq!(got,line_relation::LineRelation::'+{1:'Disjoint',2:'Collinear',3:'Parallel'}[expected['tag']]+');')
        source.append('}')
    source.append('}\n#[test]fn original_b_barycentric(){')
    for i in range(70):
        point,a,b,c=[[rng.uniform(-5,5)for _ in range(3)]for _ in range(4)];expected=barycentric(point,a,b,c);cases.append({'kind':'barycentric','point':enc(point),'a':enc(a),'b':enc(b),'c':enc(c),'expected':expected});source.append('{let p=barycentric::barycentric(v3('+rust(enc(point))+'),v3('+rust(enc(a))+'),v3('+rust(enc(b))+'),v3('+rust(enc(c))+'));for(a,e)in p.to_array().into_iter().zip('+rust(expected)+'){eq(a,e)}}')
    source.append('}')
    (OUT/'linear-fixtures.json').write_text(json.dumps({'binary_sha256':SHA,'target_execution':False,'cases':cases},indent=2)+'\n',encoding='utf-8');(OUT/'linear_tests.rs').write_text('\n'.join(source)+'\n',encoding='utf-8');print(json.dumps({'cases':len(cases)}))
if __name__=='__main__':main()
