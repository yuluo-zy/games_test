import native_oracle as n
from native_oracle import *
import random,collections
def enc(v):return [bits(x)for x in v]
def rust(v):return json.dumps(v,separators=(',',':'))
def result(a,b):
    setup();cpu.mem_write(DATA+0x100,struct.pack('<9f',*[v for p in a for v in p]));cpu.mem_write(DATA+0x200,struct.pack('<9f',*[v for p in b for v in p]));arguments(DATA,DATA+0x100,DATA+0x200);call(0x141fdfd80);tag=u64(DATA)&0xffffffff
    return {'tag':tag,'points':[floats(DATA+4,3),floats(DATA+16,3)]if tag==1 else [floats(DATA+4,3)]if tag==0 else []}
def main():
    rng=random.Random(863434);triangle=[[0,0,0],[1,0,0],[0,1,0]]
    inputs=[(triangle,triangle),(triangle,[[2,2,0],[3,2,0],[2,3,0]]),(triangle,[[0,0,0],[0,-1,1],[-1,0,1]]),(triangle,[[0,0,0],[1,0,0],[0,0,1]]),(triangle,[[0,0,1e-11],[1,0,1e-11],[0,1,1e-11]]),(triangle,[[0,0,1e-8],[1,0,1e-8],[0,1,1e-8]]),(triangle,[[0,0,0],[0,0,0],[0,0,0]])]
    inputs.extend(([[rng.uniform(-10,10)for _ in range(3)]for _ in range(3)],[[rng.uniform(-10,10)for _ in range(3)]for _ in range(3)])for _ in range(250))
    rows=[];source=['use super::*;use glam::Vec3;','fn v(b:[u32;3])->Vec3 {Vec3::new(f32::from_bits(b[0]),f32::from_bits(b[1]),f32::from_bits(b[2]))}','fn same(a:Vec3,b:Vec3)->bool {(a-b).length()<=0.00003}','#[test]fn original_b_triangle_triangle_business(){']
    for a,b in inputs:
        expected=result(a,b);rows.append({'a':[enc(p)for p in a],'b':[enc(p)for p in b],'expected':expected});source.append('{let a='+rust([enc(p)for p in a])+'.map(v);let b='+rust([enc(p)for p in b])+'.map(v);let got=triangle_triangle::triangles_intersect(a,b);')
        if expected['tag']in (2,3):source.append('assert_eq!(got,triangle_triangle::TriangleTriangleIntersection::'+('Coplanar'if expected['tag']==2 else 'Disjoint')+');}')
        elif expected['tag']==0:source.append('match got{triangle_triangle::TriangleTriangleIntersection::Point(p)=>assert!(same(p,v('+rust(expected['points'][0])+'))),_=>panic!("point classification mismatch")} }')
        else:source.append('match got{triangle_triangle::TriangleTriangleIntersection::Segment(a,b)=>{let x=v('+rust(expected['points'][0])+');let y=v('+rust(expected['points'][1])+');assert!(same(a,x)&&same(b,y),"ordered endpoint geometry mismatch: {a:?} {b:?} vs {x:?} {y:?}");},_=>panic!("segment classification mismatch")} }')
    source.append('}');(OUT/'triangle-fixtures.json').write_text(json.dumps({'binary_sha256':SHA,'target_execution':False,'comparison':'exact result category; ordered endpoint distances <= 3e-5; numerical residual is explicitly tolerated, not claimed bit-identical','cases':rows},indent=2)+'\n',encoding='utf-8');(OUT/'triangle_tests.rs').write_text('\n'.join(source)+'\n',encoding='utf-8');print(json.dumps({'cases':len(rows),'categories':dict(collections.Counter(r['expected']['tag']for r in rows))}))
if __name__=='__main__':main()
