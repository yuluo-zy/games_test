"""B actual wall consumers: row production, authority trim, normalization,
proposal hole consumer and real 96-byte CPU instance emission. Stage acceptance;
the unported row->proposal middle is explicitly not full from_curve acceptance.
"""
import hashlib,json,math,random,struct,subprocess,sys
from verification_provenance import ROOT,candidate_provenance
sys.path.insert(0,str(ROOT/'evidence/building-constructor'))
from inspect_overlap import resolve,rows,trim,normalize,row_holes
from native_fixture import b,call
from capture_emission import contexts
from capture_proposals import groups
from capture_partitions import partitions

def brick(row):return struct.pack('<8fIBB',*row)
def f32(v):return struct.unpack('<f',struct.pack('<f',v))[0]
def main():
 rng=random.Random(20261012);lines=[];wants=[];metadata=[];counts={}
 def add(stage,line,expected):lines.append(line);wants.append(expected);metadata.append(stage);counts[stage]=counts.get(stage,0)+1
 fixed=[[-.2,-.3,.4,.5],[.6,-.3,1.2,.5],[-.2,.5,.4,1.3],[.6,.5,1.2,1.3],[-1,-1,2,.5],[-1,.5,2,2],[-1,-1,.5,2],[.5,-1,2,2],[.25,.25,.75,.75],[-1,-1,2,2],[2,2,3,3]]
 for i in range(320):
  bs=[]
  for j in range([1,2,3,7][i%4]):
   x,y,w,h=rng.uniform(-2,2),rng.uniform(-1,1),rng.uniform(.1,3),rng.uniform(.1,3)
   if i<44:x,y,w,h=0.,0.,1.,1.
   values=[x,y,x+w,y+h]+[rng.uniform(-.3,.3)for _ in range(4)]+[rng.randrange(0x1000000),int(i%5==0),int(i%7==0)];values=list(struct.unpack('<8fIBB',brick(values)));bs.append((values[:4],values[4:8],values[8],values[9],values[10]))
  hole=fixed[i%11]if i<44 else [rng.uniform(-2,1),rng.uniform(-1,1),rng.uniform(1,3),rng.uniform(1,3)];holes=[list(struct.unpack('<4f',struct.pack('<4f',*hole)))]
  if i>=220:holes.append([.1,.1,.4,.5])
  actual=bs
  for h in holes:
   result=resolve(h,actual);actual=[(x[:4],x[4:8],*x[8:])for x in result]
  inputs=[brick([*x[0],*x[1],*x[2:]]).hex()for x in bs];add('hole_consumer','cut '+str(len(holes))+' '+' '.join(struct.pack('<4f',*h).hex()for h in holes)+' '+' '.join(inputs),f'{len(actual)} '+','.join(brick([*x[0],*x[1],*x[2:]]).hex()for x in actual))
 for i in range(160):
  heights=[0.]if i%4 else []
  for j in range(1+i%5):heights.append(f32((heights[-1]if heights else 0)+rng.uniform(.03,2)))
  scale=heights[-1];seed=rng.randrange(1<<32);out=rows(heights,scale,seed);add('random_rows',f'rows {scale} {seed} '+' '.join(struct.pack('<f',x).hex()for x in heights),f'{len(out)//4} '+','.join(out[j:j+4].hex()for j in range(0,len(out),4)))
 for i in range(200):
  scale=f32(rng.uniform(.05,5));length=f32(rng.uniform(.1,10));us=[f32(j/(4+i%7))for j in range(5+i%7)];heights=[f32(rng.uniform(.05,scale))for _ in us];raw=rows([0.,scale],scale,i);bs=list(struct.unpack('<'+'f'*(len(raw)//4),raw));out=trim(bs,us,heights,scale,length,i%2);add('authority_trim',f'trim {scale} {length} {i%2} {len(bs)} '+' '.join(struct.pack('<f',x).hex()for x in bs)+' '+' '.join(struct.pack('<2f',u,h).hex()for u,h in zip(us,heights)),f'{len(out)//32} '+','.join(out[j:j+32].hex()for j in range(0,len(out),32)))
 for i in range(120):
  hs=[];length=f32(rng.uniform(1,20));height=f32(rng.uniform(.3,4))
  for j in range(i%6):
   x,y=rng.uniform(-.8,length+.8),rng.uniform(-.8,height);a=list(struct.unpack('<4f',struct.pack('<4f',x,y,x+rng.uniform(.02,1),y+rng.uniform(.03,2))));hs.append((a,0,int(j%4==3)))
  out=normalize(hs,length,height,i%2);add('normalization',f'normalize {length} {height} {i%2} '+' '.join(struct.pack('<4fB',*a,o).hex()for a,k,o in hs),f'{len(out)//16} '+','.join(out[j:j+16].hex()for j in range(0,len(out),16)))
 def test_row(row,holes,length,height):
  indices,mins=row_holes(row,holes,length,height);add('row_hole_collector','row-holes '+struct.pack('<4f',*row).hex()+f' {length} {height} '+' '.join(struct.pack('<4f',*h).hex()for h in holes),','.join(map(str,indices))+' '+','.join(mins[j:j+4].hex()for j in range(0,len(mins),4)))
 for i in range(320):
  length,height=f32(rng.uniform(.1,20)),f32(rng.uniform(.1,6));x,y=rng.uniform(-1,1),rng.uniform(-1,1);row=list(struct.unpack('<4f',struct.pack('<4f',x,y,x+rng.uniform(.1,2),y+rng.uniform(.1,2))));holes=[]
  for j in range(i%19):
   x,y=rng.uniform(-1,1),rng.uniform(-1,1);holes.append(list(struct.unpack('<4f',struct.pack('<4f',x,y,x+rng.uniform(.05,2),y+rng.uniform(.05,2)))))
  holes.sort(key=lambda h:h[2]);test_row(row,holes,length,height)
 def neighbor(v,direction):
  bits=struct.unpack('<I',struct.pack('<f',v))[0];bits+=direction if v>=0 else -direction;return struct.unpack('<f',struct.pack('<I',bits))[0]
 for length,height in [(1.,1.),(12.,2.),(.23,.46),(7.3,3.1)]:
  length,height=f32(length),f32(height);half=[f32(f32(.46)/length)*.5,f32(f32(.46)/height)*.5]
  for axis in [0,1]:
   for side in [-1,1]:
    for shift in [-1,0,1]:
     edge=f32((0.25-half[axis])if side==-1 else(0.75+half[axis]));edge=neighbor(edge,shift)if shift else edge;h=[.3,.3,.7,.7];h[axis if side==1 else axis+2]=edge
     if side==-1:h[axis]=f32(edge-.3)
     else:h[axis+2]=f32(edge+.3)
     test_row([.25,.25,.75,.75],[h,h],length,height)
 for holes in [[[0.,0.,0.,0.],[-0.,0.,0.,0.]],[[.8,-.2,1.,.1],[-.2,-.2,.1,.1]],[[0.,0.,1.,float('nan')]],[[float('nan'),0.,1.,1.]]]:test_row([0.,0.,1.,1.],holes,12.,2.)
 contexts.clear();groups.clear()
 for seed in [27,13,1]:call(seed=seed)
 for seed in [17,41]:call(seed=seed,holes=[([2.,.4,3.,1.4],0,0)])
 points=[[math.cos(i*math.pi/8)*2,math.sin(i*math.pi/8)*2]for i in range(17)];lens=[math.hypot(points[i+1][0]-points[i][0],points[i+1][1]-points[i][1])for i in range(16)];length=sum(lens);us=[sum(lens[:i])/length for i in range(17)];call(seed=23,points=points,us=us,length=length,height=2.7,holes=[([2.,.4,3.,1.4],0,0)])
 call(holes=[([2,.2,5,1.],0,0),([3,.4,4,1.2],0,0),([0,.1,1,.4],0,0)])
 for c in contexts:
  vals=c['ends']+c['u']+c['scale']+[c['width_padding']]+c['depth_axis']+c['surface'];add('resolved_instance','emit '+c['brick']+' '+struct.pack('<15f2I',*vals,c['material'],c['flags']).hex(),c['output'])
 for g in groups:
  expected=f"{len(g['expected'])} "+','.join(g['expected']);add('proposal_consumer','proposal '+g['brick']+' '+struct.pack('<4fB',*g['values'],g['split_enabled']).hex()+' '+' '.join(g['holes']),expected)
  if 'context'not in g:continue
  c=g['context'];cb=struct.pack('<15f2I',*(c['ends']+c['u']+c['scale']+[c['width_padding']]+c['depth_axis']+c['surface']),c['material'],0);add('resolved_plain_chain','chain '+struct.pack('<2f',g['values'][2],g['values'][3]).hex()+' '+g['brick']+' '+cb.hex()+' '+' '.join(g['holes']),f"{len(g['records'])} "+','.join(g['records']))
 partitions.clear()
 for seed in [1,27,128]:
  for h in [.4,1.,2.,2.7]:call(seed=seed,height=h)
 call(seed=39,authority_heights=[.8,2.,1.1,2.,.8])
 for p in partitions:
  add('plain_row_partition','partition '+struct.pack('<2f',*p['row_u']).hex()+f" {p['length']} {p['source_seed']} {p['boundary_count']} {p['row_index']}",f"{len(p['expected'])//8} "+','.join(p['expected'][i:i+8]for i in range(0,len(p['expected']),8)))
 got=subprocess.run([str(ROOT/'reconstruction/target/release/wall-constructor-probe')],input='\n'.join(lines)+'\n',capture_output=True,text=True,check=True).stdout.splitlines();assert len(got)==len(wants)
 failures=[{'case':i,'stage':metadata[i],'input':lines[i],'expected':e,'actual':a}for i,(e,a)in enumerate(zip(wants,got))if e.rstrip()!=a.rstrip()]
 out=ROOT/'evidence/building-constructor/B-wall-constructor-verification.json';report={'ok':not failures,'baseline':'B','binary_sha256':b['binary']['sha256'],'oracle_sources':{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest()for p in [ROOT/'scripts/multi-version/verify_wall_constructor.py',*(ROOT/'evidence/building-constructor').glob('*.py')]},'cases':len(lines),'stages':counts,'failure_count':len(failures),'failures':failures[:20],'candidate_binary_sha256':hashlib.sha256((ROOT/'reconstruction/target/release/wall-constructor-probe').read_bytes()).hexdigest(),'scope':'Full original row splitter and authority trim; periodic hole normalization and actual row candidate collector with reverse-minimum thresholds and raw row horizontal partition owner including original seed mixing; ordered hole consumer; actual proposal owner including horizontal split; actual joined resolved-plain proposal -> cuts -> fresh piece metadata -> 96-byte CPU instances. Raw proposal upstream is not accepted.','limitations':['Rust row->proposal middle, noise/seed owners, curve endpoint ownership, surface selection and depth choice are not recovered. Resolved emission API requires their explicit inputs; this is not full from_curve or World chain acceptance.','MyBrick 2-byte padding excluded; all 96-byte instance fields compared.','Native full from_curve scenarios include rectangle/circle empty/single/multi-hole calls. Ordinary kind2/tag2/fence-fit-false resolved proposal consumer is joined through fresh per-piece flags and complete96byte output. Raw upstream ownership is not accepted.'],'bridges':['Standard allocator/free/reallocator and CRT memcpy/memmove/memset, floorf/ceilf/sinf/truncf/roundf; puffin profiler off.','Preallocated original standard Vec/VecDeque storage for proposal/normalization tests. No game geometry/seed/branch bridge.','Dependency fastrand 1.8 used in Rust, original dependency instructions executed in native.'],**candidate_provenance()};out.write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'ok':not failures,'cases':len(lines),'stages':counts,'failures':len(failures),'first':failures[:1]}));assert not failures
if __name__=='__main__':main()
