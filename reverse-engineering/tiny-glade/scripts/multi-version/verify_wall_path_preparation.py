"""Original resolved wall path producer through actual4empty encoded grids."""
import argparse,hashlib,json,random,struct,subprocess,sys
from pathlib import Path
from verification_provenance import ROOT
sys.path.insert(0,str(ROOT/'evidence/building-production-authority'))
from native_path_empty import prepare,b

def f32(v):return struct.unpack('<f',struct.pack('<f',v))[0]
def hx(f):return f'{struct.unpack("<I",struct.pack("<f",f))[0]:08x}'
def words(h):return ''.join(f'{x:08x}'for x in struct.unpack('<'+'I'*(len(h)//8),bytes.fromhex(h)))
def main():
 parser=argparse.ArgumentParser();parser.add_argument('--no-report',action='store_true');args=parser.parse_args();rng=random.Random(20261019);cases=[]
 for i in range(128):
  n=[2,3,5,8,9,17,24,33][i%8];points=[[f32(j*.3),f32(rng.uniform(-2,3)),f32((j%3)*.4)]for j in range(n)]
  if i%4==0:points.append(points[0])
  if i%5==0:points.insert(1,points[0])
  if i%11==0:points[0][0]=-0.;points.insert(1,[0.,points[0][1],points[0][2]])
  relative=[f32(rng.uniform(-1,1))for _ in points];scalar=[0.,-0.,.6,-.6,5.][i%5];cases.append((points,relative,scalar,bool(i%2)))
 lines=[hx(s)+f' {int(flag)} 0 '+' '.join(''.join(hx(v)for v in [*p,r])for p,r in zip(pts,rs))for pts,rs,s,flag in cases]
 probe=ROOT/'reconstruction/target/release/wall-path-preparation-probe';actual=subprocess.run([str(probe)],input='\n'.join(lines)+'\n',capture_output=True,text=True,check=True).stdout.splitlines();assert len(actual)==len(cases)
 fail=[]
 for i,(case,got)in enumerate(zip(cases,actual)):
  out=prepare(*case);want=f'{out["count"]} {out["segment_count"]} {words(out["length"])} {words(out["points"])+words(out["points_u"])} {words(out["segments"])}'
  if got!=want:fail.append({'case':i,'input':lines[i],'original':want,'rust':got})
 sourcepaths=[ROOT/'reconstruction/building/wall_path_preparation.rs',ROOT/'reconstruction/building/wall_collection.rs',ROOT/'reconstruction/src/bin/wall-path-preparation-probe.rs',ROOT/'reconstruction/Cargo.lock'];oraclepaths=[Path(__file__),ROOT/'evidence/building-production-authority/native_path_empty.py',ROOT/'evidence/building-constructor/native_fixture.py']
 report={'ok':not fail,'cases':len(cases),'failure_count':len(fail),'failures':fail[:10],
  'scope':'B resolved per-wall original path producer entry14074cbe0 to14074e3dc. Actual4explicit512x512zero-u8buffers, whole28samples/point attenuation, smoothing,relative gating,threshold-loop,dedup,exp transform and Curve3 rebuild execute. Compare all preparedpositions/U/length/segmentvalues including signedzero and repeated adjacent/closing points.',
  'excluded':['Nonempty path cases separately verified','NativeouterEventReader/PublicWalls lookup/UiSignifier profiling','Native PathRastersResources buffer selection (caller supplies currentselected explicitbuffers)','Invalid/nonfinite source domain','A/C execution','Runtime scheduling and regular/gable caches separately verified'],
  'bridges':['IAT CRT expf, memory/alloc/realloc/free from native_fixture only; allgame/privateFloatGrid/Curve3 helpers execute','No game scalar or geometry output bridge; empty resource bytes are actualinput domain, not samplerreturnstubs'],
  'binary_sha256':b['binary']['sha256'],'candidate_binary_sha256':hashlib.sha256(probe.read_bytes()).hexdigest(),
  'candidate_sources':{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest()for p in sourcepaths},'oracle_sources':{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest()for p in oraclepaths}}
 if not args.no_report:(ROOT/'evidence/building-production-authority/B-empty-path-verification.json').write_text(json.dumps(report,indent=2)+'\n')
 print(json.dumps({'ok':not fail,'cases':len(cases),'failures':len(fail),'first':fail[:1],'report_written':not args.no_report}));assert not fail,fail[:1]
if __name__=='__main__':main()
