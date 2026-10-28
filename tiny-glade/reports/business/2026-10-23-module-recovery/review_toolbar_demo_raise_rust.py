"""直接抽取生产纯数值函数与原指令夹具比对，不冒称完整Bevy系统验收。"""
from pathlib import Path
import re,subprocess,tempfile,hashlib,json
root=Path(__file__).resolve().parents[4]
source=root/'tiny-glade/crates/country-core/src/systems/ui_raise_shape.rs'
s=source.read_text()
functions=[]
for begin,end in [('fn target_elevation(', '\nfn ground_floor'),('fn raise_arrow_normal(', '\nfn feedback_kind'),('fn feedback_kind(', '\n/// B 140955650')]:
 functions.append(s[s.index(begin):s.index(end,s.index(begin))])
rs='const MAX_ELEVATION:f32=60.; struct Vec3{x:f32,y:f32,z:f32} struct Vec2{x:f32,y:f32} impl Vec2{fn new(x:f32,y:f32)->Self{Self{x,y}}} struct RaiseShape{origin:Vec3,initial_elevation:f32,terrain_floor:f32}\n'+'\n'.join(functions)+'\nfn main(){\n'
fixtures=[]
for name,n,m in [('target',4,1),('normal',3,2),('feedback',2,1)]:
 fixture=root/f'tiny-glade/tests/fixtures/native/toolbar-raise-{name}.ron';fixtures.append(fixture)
 rows=[tuple(map(int,re.findall(r'\d+',line)))for line in fixture.read_text().splitlines()[1:-1]]
 if name=='target':
  rs+='let rows:&[([u32;4],u32)]=&['+','.join(f'([{a},{b},{c},{d}],{e})'for a,b,c,d,e in rows)+'];\n'
  rs+='for(input,expected)in rows{let[o,p,i,f]=input.map(f32::from_bits);let actual=target_elevation(&RaiseShape{origin:Vec3{x:0.,y:o,z:0.},initial_elevation:i,terrain_floor:f},Vec3{x:0.,y:p,z:0.});if f32::from_bits(*expected).is_nan(){assert!(actual.is_nan(),"{input:?}")}else{assert_eq!(actual.to_bits(),*expected,"{input:?}");}}\n'
 elif name=='normal':
  rs+='let rows:&[([u32;3],[u32;2])]=&['+','.join(f'([{a},{b},{c}],[{d},{e}])'for a,b,c,d,e in rows)+'];\n'
  rs+='for(input,expected)in rows{let[x,y,z]=input.map(f32::from_bits);let actual=raise_arrow_normal(Vec3{x,y,z});for(actual,expected)in [actual.x,actual.y].into_iter().zip(*expected){if f32::from_bits(expected).is_nan(){assert!(actual.is_nan(),"{input:?}")}else{assert_eq!(actual.to_bits(),expected,"{input:?}");}}}\n'
 else:
  rs+='let rows:&[([u32;2],u8)]=&['+','.join(f'([{a},{b}],{c})'for a,b,c in rows)+'];\n'
  rs+='for(input,expected)in rows{let[p,c]=input.map(f32::from_bits);assert_eq!(feedback_kind(p,c).unwrap_or(255),*expected,"{input:?}");}\n'
 rs+=f'println!("{name}: {len(rows)} original instruction cases passed");\n'
rs+='}\n'
with tempfile.TemporaryDirectory(prefix='toolbar-demo-raise-')as directory:
 path=Path(directory)/'oracle.rs';path.write_text(rs);binary=Path(directory)/'oracle'
 compile=subprocess.run(['rustc','--edition','2024',str(path),'-o',str(binary)],capture_output=True,text=True)
 assert compile.returncode==0,compile.stderr
 run=subprocess.run([str(binary)],capture_output=True,text=True)
 assert run.returncode==0,run.stderr
 print(run.stdout,end='')
 Path(__file__).with_name('toolbar-raise-arithmetic-rust.json').write_text(json.dumps({'production_source':str(source.relative_to(root)),'production_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),'fixtures':{p.name:hashlib.sha256(p.read_bytes()).hexdigest()for p in fixtures},'method':'直接抽取当前生产三个纯数值函数，rustc单独编译；未替换函数体，无额外业务逻辑','stdout':run.stdout,'exit_code':run.returncode,'limitations':'仅纯数值片段；Bevy事件、交互、资源、历史与实际绘制另外验收'},ensure_ascii=False,indent=2)+'\n')
