from pathlib import Path
import json,random,math,struct,collections
import oracle as o
from oracle import *
rows=[];groups={};src=['use super::*;use glam::{Vec2,Vec3,Mat2,Mat4,Quat};','fn f(b:u32)->f32{f32::from_bits(b)}','fn v2(b:[u32;2])->Vec2{Vec2::from_array(b.map(f))}','fn v3(b:[u32;3])->Vec3{Vec3::from_array(b.map(f))}',
 'fn close(a:f32,b:u32){let e=f(b);if e.is_nan(){assert!(a.is_nan(),"expected NaN, got {a}")}else if e.is_infinite(){assert_eq!(a,e)}else{assert!((a-e).abs()<=2e-5*e.abs().max(1.0),"got {a:?} {:x}, expected {e:?} {b:x}",a.to_bits());}}',
 'fn vec_close(a:impl IntoIterator<Item=f32>,b:impl IntoIterator<Item=u32>){let a:Vec<_>=a.into_iter().collect();let b:Vec<_>=b.into_iter().collect();assert_eq!(a.len(),b.len());for(a,b)in a.into_iter().zip(b){close(a,b)}}',
 'fn rect(b:[u32;6])->Rectangle2d{let b=b.map(f);Rectangle2d{axis:Vec2::new(b[0],b[1]),center:Vec2::new(b[2],b[3]),size:Vec2::new(b[4],b[5])}}']
rng=random.Random(2026101307)
def begin(name):groups[name]=0;src.append(f'#[test]fn {name}(){{')
def row(name,data,code):groups[name]+=1;rows.append({'group':name,**data});src.append(code)
def end():src.append('}')
def rs_curve(expr,expected,dim=2):
 return '{let got='+expr+';assert_eq!(got.points.len(),'+str(len(expected['points']))+');for(p,e)in got.points.iter().zip('+rust(expected['points'])+'){vec_close(p.to_array(),e)}vec_close(got.points_u.iter().copied(),'+rust(expected['u'])+');close(got.length,'+str(expected['length'])+');}'
name='original_completion_transform';begin(name)
for _ in range(60):
 q=[rng.uniform(-1,1)for _ in range(4)];length=math.sqrt(sum(x*x for x in q));q=[x/length for x in q];t=[rng.uniform(-20,20)for _ in range(3)];s=[rng.uniform(-3,3)for _ in range(3)];value=q+t+s;expected=transform(value)
 code='{let q='+rust(enc(q))+'.map(f);let t=crate::components::transform::Transform{r:Quat::from_array(q),t:v3('+rust(enc(t))+'),s:v3('+rust(enc(s))+')};vec_close(t.forward().to_array(),'+rust(expected['forward'])+');let a=t.compute_affine();'
 for i,e in enumerate(expected['affine']):code+='vec_close('+(['a.matrix3.x_axis','a.matrix3.y_axis','a.matrix3.z_axis','a.translation'][i])+'.to_array(),'+rust(e)+');'
 code+='let matrix=t.compute_matrix();vec_close(matrix.x_axis.truncate().to_array(),'+rust(expected['affine'][0])+');vec_close(matrix.y_axis.truncate().to_array(),'+rust(expected['affine'][1])+');vec_close(matrix.z_axis.truncate().to_array(),'+rust(expected['affine'][2])+');vec_close(matrix.w_axis.truncate().to_array(),'+rust(expected['affine'][3])+');}'
 row(name,{'input':enc(value),'expected':expected},code)
end();name='original_completion_circle_curves';begin(name)
for i in range(40):
 value=[rng.uniform(-10,10),rng.uniform(-10,10),rng.uniform(.2,4),rng.uniform(-15,15)];step=rng.uniform(.3,5);h=rng.uniform(-5,5);start,end_u=sorted([rng.random(),rng.random()]);expected=circle_curve(value,step);expected3=circle_curve(value,step,height=h);expected_range=circle_curve(value,step,start=start,end=end_u)
 c='Circle2d::new_rotated(v2('+rust(enc(value[:2]))+'),f('+str(bits(value[2]))+'),f('+str(bits(value[3]))+'))'
 code=rs_curve(c+'.as_curve2(f('+str(bits(step))+'))',expected)+rs_curve(c+'.as_curve3(f('+str(bits(step))+'),f('+str(bits(h))+'))',expected3,3)+rs_curve(c+'.as_curve2_range(f('+str(bits(step))+'),f('+str(bits(start))+'),f('+str(bits(end_u))+'))',expected_range)
 code+='{let c='+c+';let points:Vec<_>=c.as_points2(f('+str(bits(step))+')).collect();assert_eq!(points.len(),'+str(len(expected['points']))+');for(p,e)in points.iter().zip('+rust(expected['points'])+'){vec_close(p.to_array(),e)}}'
 row(name,{'circle':enc(value),'step':bits(step),'height':bits(h),'start':bits(start),'end':bits(end_u),'expected':expected,'expected3':expected3,'expected_range':expected_range},code)
end();name='original_completion_circle_lerp';begin(name)
for _ in range(60):
 a=[rng.uniform(-10,10),rng.uniform(-10,10),rng.uniform(.1,5),rng.uniform(-20,20)];other=[rng.uniform(-10,10),rng.uniform(-10,10),rng.uniform(.1,5),rng.uniform(-20,20)];t=rng.uniform(-1,2);expected=circle_lerp(a,other,t)
 def c(v):return 'Circle2d::new_rotated(v2('+rust(enc(v[:2]))+'),f('+str(bits(v[2]))+'),f('+str(bits(v[3]))+'))'
 row(name,{'a':enc(a),'b':enc(other),'t':bits(t),'expected':expected},'{let c='+c(a)+'.lerp('+c(other)+',f('+str(bits(t))+'));vec_close([c.center.x,c.center.y,c.radius,c.rotation],'+rust(expected)+');}')
end();name='original_completion_aabb_folds';begin(name)
for dim in [2,3]:
 points_sets=[[],[[0]*dim],[[float('inf')]*dim],[[float('nan')]*dim],[[float('nan')]*dim,[2]*dim],[[float('-inf')]*dim,[float('inf')]*dim]]+[[[rng.uniform(-10,10)for _ in range(dim)]for _ in range(rng.randint(1,10))]for _ in range(30)]
 for points in points_sets:
  expected=aabb_inline(points,dim);vals=rust([enc(p)for p in points]);ty='Aabb'+str(dim);v='v'+str(dim)
  row(name,{'dimension':dim,'points':[enc(p)for p in points],'expected':expected},'{let points:Vec<_>='+vals+'.into_iter().map('+v+').collect();let a='+ty+'::from_points(points);vec_close(a.min.to_array().into_iter().chain(a.max.to_array()),'+rust(expected)+');}')
end();name='original_completion_aabb_segment2';begin(name)
for i in range(90):
 box=[-1,-2,3,4];a,bp=[([0,0],[.1,.1]),([-2,0],[5,0]),([-1,0],[-2,0]),([-1,0],[-.5,0]),([0,4],[1,4]),([0,0],[0,0])][i]if i<6 else ([rng.uniform(-5,5),rng.uniform(-5,5)],[rng.uniform(-5,5),rng.uniform(-5,5)]);expected=aabb_segment(box,a,bp)
 code='{let a=Aabb2{min:v2('+rust(enc(box[:2]))+'),max:v2('+rust(enc(box[2:]))+')};let hit=a.intersect_segment(v2('+rust(enc(a))+'),v2('+rust(enc(bp))+'));'
 code+= 'assert!(hit.is_none());}'if expected is None else 'vec_close(hit.unwrap().to_array(),'+rust(expected)+');}'
 row(name,{'aabb':enc(box),'start':enc(a),'end':enc(bp),'expected':expected},code)
end();name='original_completion_rectangle_distance_normals';begin(name)
for i in range(80):
 angle=rng.uniform(-math.pi,math.pi);value=[math.cos(angle),math.sin(angle),rng.uniform(-5,5),rng.uniform(-5,5),rng.uniform(.1,8),rng.uniform(.1,8)];point=value[2:4]if i<5 else [rng.uniform(-10,10),rng.uniform(-10,10)];expected=rect_distance(value,point);u=rng.choice([0,.5,1])if i<20 else rng.random();normal=rect_normal(value,u);unnormal=rect_normal(value,u,True)
 row(name,{'rectangle':enc(value),'point':enc(point),'u':bits(u),'expected':expected,'normal':normal,'unnormal':unnormal},'{let r=rect('+rust(enc(value))+');let p=v2('+rust(enc(point))+');close(r.distance_to_closest_edge(p),'+str(expected['distance'])+');close(r.signed_distance_to_closest_edge(p),'+str(expected['signed'])+');vec_close(r.closest_edge_normal(p).to_array(),'+rust(expected['closest_normal'])+');assert_eq!(r.get_closest_edge_id(p),'+str(expected['edge_id'])+');vec_close(r.get_normal_at_u(f('+str(bits(u))+')).to_array(),'+rust(normal)+');vec_close(r.get_normal_at_u_non_normalized_corners(f('+str(bits(u))+')).to_array(),'+rust(unnormal)+');}')
end();name='original_completion_rectangle_slices';begin(name)
value=[.6,.8,2,3,2,4]
for start,end_u in [(0,1),(.8,.2),(1,0),(.5,.5),(0,1/3),(1/3,.5),(.5,5/6),(5/6,1)]+[(rng.random(),rng.random())for _ in range(35)]:
 expected=rect_slice(value,start,end_u)
 row(name,{'rectangle':enc(value),'start':bits(start),'end':bits(end_u),'expected':expected},'{let r=rect('+rust(enc(value))+');vec_close(r.circular_slice(f('+str(bits(start))+'),f('+str(bits(end_u))+')),'+rust(expected)+');}')
end();name='original_completion_rectangle_segment_receiver';begin(name)
for i,(start,end_u)in enumerate([(0,.7),(.8,.2),(0,1),(1,0),(.5,.5),(0,1/3),(1/3,.5),(.5,5/6),(5/6,1)]+[(rng.random(),rng.random())for _ in range(20)]):
 for pending,join in [(None,False),([.25,-.75],True),([.25,-.75],False)]:
  expected=rect_segments(value,start,end_u,pending,join)
  code='{let r=rect('+rust(enc(value))+');let mut state=rectangle::RectangleSliceState{pending_normal:'+('None'if pending is None else 'Some(v2('+rust(enc(pending))+'))')+',join_pending:'+str(join).lower()+'};let mut points=Vec::new();r.circular_slice_segment_points(f('+str(bits(start))+'),f('+str(bits(end_u))+'),&mut state,|p|points.push(p));assert_eq!(points.len(),'+str(len(expected['points']))+');'
  for j,point in enumerate(expected['points']):code+='vec_close(points['+str(j)+'].position.to_array(),'+rust(point['position'])+');vec_close(points['+str(j)+'].normal.to_array(),'+rust(point['normal'])+');close(points['+str(j)+'].relative_u,'+str(point['relative_u'])+');assert_eq!(points['+str(j)+'].tag,'+str(point['tag'])+');'
  code+='assert_eq!(state.join_pending,'+str(expected['join']).lower()+');'
  code+='assert!(state.pending_normal.is_none());}'if expected['pending'] is None else 'vec_close(state.pending_normal.unwrap().to_array(),'+rust(expected['pending'])+');}'
  row(name,{'rectangle':enc(value),'start':bits(start),'end':bits(end_u),'pending':None if pending is None else enc(pending),'join':join,'expected':expected},code)
end();name='original_completion_rectangle_deformation';begin(name)
for i in range(80):
 value=[.6,.8,rng.uniform(-5,5),rng.uniform(-5,5),rng.uniform(.1,8),rng.uniform(.1,8)];edge=i%4;amount=rng.uniform(-10,10);limits=[.2,7];expected=rect_expand(value,edge,amount,limits);matrix=[.4,.7,-.7,.4];extended=value+[-2,3];rotated=rect_rotate(extended,matrix)
 row(name,{'rectangle':enc(value),'edge':edge,'delta':bits(amount),'limits':enc(limits),'expanded':expected,'matrix':enc(matrix),'rotated':rotated},'{let mut r=rect('+rust(enc(value))+');r.expand_along_edge('+str(edge)+',f('+str(bits(amount))+'),f('+str(bits(limits[0]))+'),f('+str(bits(limits[1]))+'));vec_close([r.axis.x,r.axis.y,r.center.x,r.center.y,r.size.x,r.size.y],'+rust(expected)+');let mut r=Rectangle3d{footprint:rect('+rust(enc(value))+'),bottom:-2.0,top:3.0};r.rotate_projective_xz(Mat2::from_cols_array(&'+rust(enc(matrix))+'.map(f)));vec_close([r.footprint.axis.x,r.footprint.axis.y,r.footprint.center.x,r.footprint.center.y,r.footprint.size.x,r.footprint.size.y,r.bottom,r.top],'+rust(rotated)+');}')
end();name='original_completion_rectangle_intersections';begin(name)
for i in range(100):
 value=[1,0,0,0,4,6];other=[.6,.8,rng.uniform(-5,5),rng.uniform(-5,5),rng.uniform(.2,10),rng.uniform(.2,10)];circle=[rng.uniform(-5,5),rng.uniform(-5,5),rng.uniform(.1,10),0]
 if i==0:other=[1,0,0,0,1,1];circle=[0,0,.1,0]
 if i==1:other=[1,0,0,0,20,20];circle=[0,0,20,0]
 if i==2:other=value.copy();circle=[2,0,1,0]
 er=rect_intersects(value,other);ec=rect_intersects(value,circle,True)
 row(name,{'rectangle':enc(value),'other':enc(other),'circle':enc(circle),'rectangle_intersects':er,'circle_intersects':ec},'{let r=rect('+rust(enc(value))+');assert_eq!(r.intersects(rect('+rust(enc(other))+')),'+str(er).lower()+');assert_eq!(r.intersects_circle(Circle2d::new(v2('+rust(enc(circle[:2]))+'),f('+str(bits(circle[2]))+'))),'+str(ec).lower()+');}')
end();name='original_completion_mouse_ray';begin(name)
for i in range(50):
 near=rng.uniform(.01,2);scale=rng.uniform(.5,4);aspect=rng.uniform(.5,2.5);invproj=[aspect/scale,0,0,0,0,1/scale,0,0,0,0,0,1/near,0,0,-1,0];view=[1,0,0,0,0,1,0,0,0,0,1,0,rng.uniform(-20,20),rng.uniform(-20,20),rng.uniform(-20,20),1]
 if i%2:
  c,s=math.cos(.6),math.sin(.6);view[:12]=[c,0,-s,0,0,1,0,0,s,0,c,0]
 mouse=[rng.uniform(0,1000),rng.uniform(0,700)];viewport=[1000,700];expected=ray(mouse,viewport,invproj,view)
 row(name,{'mouse':enc(mouse),'viewport':enc(viewport),'inverse_projection':enc(invproj),'inverse_view':enc(view),'expected':expected},'{let r=Ray::from_mouse_ss(v2('+rust(enc(mouse))+'),v2('+rust(enc(viewport))+'),Mat4::from_cols_array(&'+rust(enc(invproj))+'.map(f)),Mat4::from_cols_array(&'+rust(enc(view))+'.map(f)));vec_close(r.origin.to_array().into_iter().chain(r.direction.to_array()),'+rust(expected)+');}')
end();name='original_completion_world_size_projection';begin(name)
for distance in [0,-0.,-10,.001,.5,1,10,100,float('inf'),float('nan')]:
 for scale in [.5,1,2]:
  height=700.;cs=ws(distance,scale);ss=ws(distance,scale,height)
  row(name,{'distance':bits(distance),'scale':bits(scale),'height':bits(height),'cs':cs,'ss':ss},'{let mut p=Mat4::IDENTITY;p.y_axis.y=f('+str(bits(scale))+');close(ws_to_cs(f('+str(bits(distance))+'),p),'+str(cs)+');close(ws_to_ss(f('+str(bits(distance))+'),p,700.0),'+str(ss)+');}')
end()
(OUT/'native-fixtures.json').write_text(json.dumps({'baseline':'B','original_binary_sha256':SHA,'full_game_executed':False,'comparison_scope':'unmodified private text; original UCRT delegated; controlled allocator/memory bridges; Aabb from_points controlled original inline blocks; callback payloads captured before foreign sink logic','groups':groups,'rows':rows},indent=2)+'\n',encoding='utf-8')
(OUT/'native_tests.rs').write_text('\n'.join(src)+'\n',encoding='utf-8')
print(json.dumps(groups))
