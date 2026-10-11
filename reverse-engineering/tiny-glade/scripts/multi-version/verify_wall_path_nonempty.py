"""B full prepared owner, explicit four encoded nonempty path resources."""
import argparse,json,hashlib,random,struct,subprocess,sys
from pathlib import Path
from verification_provenance import ROOT
sys.path.insert(0,str(ROOT/'evidence/building-production-authority'))
from native_path_empty import prepare,b
from verify_wall_path_preparation import hx,words,f32

def raster(pattern):
 buffers=[bytearray(512*512)for _ in range(4)]
 for channel,x0,x1,z0,z1,code in pattern:
  for z in range(z0,z1):buffers[channel][z*512+x0:z*512+x1]=bytes([code])*(x1-x0)
 return buffers

def main():
 parser=argparse.ArgumentParser();parser.add_argument('--cases',type=int,default=64);parser.add_argument('--no-report',action='store_true');a=parser.parse_args();rng=random.Random(20261021);cases=[]
 for i in range(a.cases):
  n=[3,5,8,17][i%4]
  points=[[f32(-4+j*8/(n-1)),0. if i%3 else f32((j%3)*.2),f32((j%3)*.1)]for j in range(n)]
  if i%7==0:points.append(points[0])
  if i%11==0:points.insert(1,points[0])
  relative=[.8 if i%4==0 and j%3==1 else 0. for j in range(len(points))]
  scalar=[0.,-0.,.6,-.6,5.][i%5];pattern=[(i%4,252+(i%3),260-(i%2),250,260,[1,5,10,16,24,32,128,255][i%8])]
  if i>=64:pattern.extend([((i+1)%4,250,258,253,257,(i*13)%256),((i+2)%4,255,263,249,261,(i*17)%256),((i+3)%4,251,265,254,258,(i*23)%256)])
  cases.append((points,relative,scalar,bool(i%2),pattern))
 lines=[hx(s)+f' {int(flag)} '+','.join(':'.join(str(x)for x in patch)for patch in pattern)+' '+' '.join(''.join(hx(v)for v in[*p,r])for p,r in zip(pts,rel))for pts,rel,s,flag,pattern in cases]
 probe=ROOT/'reconstruction/target/release/wall-path-preparation-probe';actual=subprocess.run([str(probe)],input='\n'.join(lines)+'\n',capture_output=True,text=True,check=True).stdout.splitlines();assert len(actual)==len(cases);fail=[];records=0;coverage={}
 for i,((pts,rel,s,flag,pattern),got)in enumerate(zip(cases,actual)):
  out=prepare(pts,rel,s,flag,raster(pattern));want=f'{out["count"]} {out["segment_count"]} {words(out["length"])} {words(out["points"])+words(out["points_u"])} {words(out["segments"])}';records+=out['count']
  for name,count in out['observed_counts'].items():coverage[name]=coverage.get(name,0)+count
  if got!=want:fail.append({'case':i,'input':lines[i],'original':want,'rust':got})
 report={'ok':not fail,'baseline':'B','cases':len(cases),'prepared_point_count':records,'observed_native_branch_counts':coverage,'failure_count':len(fail),'failures':fail[:20],'input_lines':lines,'scope':'Original resolved fullB path producer actual4encoded nonempty grids; compare allXYZ/U/length/segment values; genericFloatGrid executes original, candidate reuses existing recovered coordinate policy via mathematically exactu8*257→u16 adaptation','native_range':['0x14074cbe0','0x14074e3dc'],'dependency_bridges':['Only IAT CRT expf, allocate/reallocate/free and memory routines; original4grid FloatGrid, threshold logic, Curve3 rebuilding execute'],'excluded':['Original outerPublicWalls/EventReader/profiling','Original4buffer-current selection caller supplies selected bytes','A/C execution','Invalid/nonfinite source domain','Runtime scheduling and completeWall/Roof rendering'],'binary_sha256':b['binary']['sha256'],'candidate_binary_sha256':hashlib.sha256(probe.read_bytes()).hexdigest(),'candidate_sources':{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest()for p in [ROOT/'reconstruction/building/wall_path_preparation.rs',ROOT/'reconstruction/building/wall_collection.rs',ROOT/'reconstruction/src/bin/wall-path-preparation-probe.rs',ROOT/'reconstruction/Cargo.lock',ROOT/'reconstruction/Cargo.toml']},'oracle_sources':{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest()for p in[Path(__file__),ROOT/'evidence/building-production-authority/native_path_empty.py',ROOT/'evidence/building-constructor/native_fixture.py']}}
 if not a.no_report:
  name='B-nonempty-path-verification.json'if not fail else 'B-nonempty-path-failure.json';(ROOT/'evidence/building-production-authority'/name).write_text(json.dumps(report,indent=2)+'\n')
 print(json.dumps({'ok':not fail,'cases':len(cases),'failures':len(fail),'first':fail[:1]}));assert not fail,fail[:1]
if __name__=='__main__':main()
