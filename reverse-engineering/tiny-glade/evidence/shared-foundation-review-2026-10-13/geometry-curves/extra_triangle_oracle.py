"""Independent boundary review of original B triangle classification."""
from pathlib import Path
import sys, json, random
OUT = Path(__file__).resolve().parent
BASE = OUT.parents[1] / 'shared-foundation-2026-10-13' / 'geometry-curves'
sys.path.insert(0, str(BASE))
import triangle_oracle as original

def main():
    rng = random.Random(1424181)
    inputs = []
    for scale in [1e-4, 1e-3, .01, 1, 100, 1e4, 1e8]:
        for _ in range(35):
            a = [[rng.uniform(-scale, scale) for _ in range(3)] for _ in range(3)]
            b = [[rng.uniform(-scale, scale) for _ in range(3)] for _ in range(3)]
            inputs.append((a, b))
    for width in [1e-11, 1e-10, 1e-9, 1e-8, 1e-7, 1e-6, 1e-5, .001]:
        for shift in [0, width/2, width, width*2]:
            a = [[0,0,0],[1,0,0],[0,1,0]]
            b = [[shift,0,-1],[shift,0,1],[shift+width,1,0]]
            inputs.append((a,b))
    for special in [float('nan'), float('inf'), -float('inf')]:
        for vertex in range(3):
            for axis in range(3):
                a = [[0,0,0],[1,0,0],[0,1,0]]
                b = [[0,0,-1],[0,0,1],[1,1,0]]
                a[vertex][axis] = special
                inputs.append((a,b))
    rows = []
    for a,b in inputs:
        rows.append({'a':[original.enc(p) for p in a], 'b':[original.enc(p) for p in b], 'expected':original.result(a,b)})
    payload={'original_sha256':original.SHA, 'whole_game_execution':False, 'case_count':len(rows),'cases':rows}
    (OUT/'extra-triangle-fixtures.json').write_text(json.dumps(payload,indent=2)+'\n',encoding='utf-8')
    source=['use super::*; use glam::Vec3;', 'fn v(b:[u32;3])->Vec3 {Vec3::from_array(b.map(f32::from_bits))}', 'fn near(a:Vec3,b:Vec3,scale:f32)->bool {(a-b).length() <= scale*0.00003+1e-10}', '#[test] fn review_original_triangle_boundaries(){', 'let mut failures=Vec::new();']
    for index, row in enumerate(rows):
        a,b = json.dumps(row['a']),json.dumps(row['b'])
        tag=row['expected']['tag']
        source.append(f'{{let a={a}.map(v);let b={b}.map(v);let got=triangles_intersect(a,b); let tag=match got{{TriangleTriangleIntersection::Point(_)=>0,TriangleTriangleIntersection::Segment(_,_)=>1,TriangleTriangleIntersection::Coplanar=>2,TriangleTriangleIntersection::Disjoint=>3}}; if tag!={tag}{{failures.push(({index},tag,{tag}));}}}}')
        if tag in (0,1):
            expected=json.dumps(row['expected']['points'])
            check='near(x,e[0],scale)' if tag==0 else 'near(x,e[0],scale)&&near(y,e[1],scale)'
            match='TriangleTriangleIntersection::Point(x)' if tag==0 else 'TriangleTriangleIntersection::Segment(x,y)'
            source.append(f'{{let a={a}.map(v);let b={b}.map(v);let e={expected}.map(v);let scale=a.iter().chain(b.iter()).chain(e.iter()).map(|p|p.abs().max_element()).fold(0.0f32,f32::max);if let {match}=triangles_intersect(a,b){{assert!({check},"ordered endpoint mismatch case {index}");}}}}')
    source += ['assert!(failures.is_empty(),"classification mismatches {failures:?}");','}']
    (OUT/'extra_triangle_tests.rs').write_text('\n'.join(source)+'\n',encoding='utf-8')
    print(json.dumps({'cases':len(rows),'output':str(OUT)}))
if __name__=='__main__': main()
