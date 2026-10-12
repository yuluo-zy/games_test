from native_production import *
import math,subprocess,json

def make_line(c):
 n=len(c['points']);cfg=struct.pack('<5fBB',c.get('base_height',0),c.get('perturb',0),c.get('width_padding')or 0,c.get('thickness_scale')or 0,c.get('wall_float',1),int(c.get('width_padding')is not None),int(c.get('thickness_scale')is not None)).hex()
 return f"{c['seed']} {c.get('material',23)} {c['length']} {c.get('holes_length',c['length'])} {int(c.get('closed',True))} {n} {cfg} "+' '.join(struct.pack('<6f',*p,u,a,b,t).hex()for p,u,a,b,t in zip(c['points'],c['us'],c['arch'],c['second'],c['third']))+' '+' '.join(struct.pack('<4fB',*h,o).hex()for h,k,o in c['holes'])
def run(cases):
 lines=[make_line(c)for c in cases];expected=[produce(**c)for c in cases];got=subprocess.run([str(ROOT/'reconstruction/target/release/wall-production-probe')],input='\n'.join(lines)+'\n',text=True,capture_output=True).stdout.splitlines();fail=[]
 for i,(want,row)in enumerate(zip(expected,got)):
  if row.startswith('error'):fail.append({'case':i,'input':lines[i],'actual':row});continue
  count,rest=row.split(' ',1);actual=[bytes.fromhex(x)for x in rest.split(',')if x];diff=[{'record':j,'different_offsets':[k for k,(x,y)in enumerate(zip(a,b))if x!=y],'expected':a.hex(),'actual':b.hex()}for j,(a,b)in enumerate(zip(want,actual))if a!=b]
  if len(want)!=len(actual)or diff:fail.append({'case':i,'input':lines[i],'expected_count':len(want),'actual_count':len(actual),'diff':diff[:2]})
 return fail,expected
if __name__=='__main__':
 ps=[[-2.,-1.],[-2.,1.],[2.,1.],[2.,-1.],[-2.,-1.]];us=[0.,1/6,.5,2/3,1.]
 basic=dict(points=ps,us=us,length=12.,arch=[2.]*5,second=[0.]*5,third=[0.]*5,holes=[],seed=27)
 cases=[basic,{**basic,'arch':[.8,2.,1.1,2.,.8]}, {**basic,'second':[.2,.3,.5,.1,.2]}, {**basic,'third':[.2,.1,.3,.1,.2]}, {**basic,'base_height':.2}, {**basic,'perturb':.7},{**basic,'width_padding':.1},{**basic,'thickness_scale':1.4},{**basic,'wall_float':.2},{**basic,'holes':[([2,.2,5,1],0,0),([3,.4,4,1.2],0,0),([0,.1,1,.4],0,0)]}]
 circle=[[math.cos(i*math.pi/8)*2,math.sin(i*math.pi/8)*2]for i in range(17)];lens=[math.hypot(circle[i+1][0]-circle[i][0],circle[i+1][1]-circle[i][1])for i in range(16)];ll=sum(lens);cs={**basic,'points':circle,'us':[sum(lens[:i])/ll for i in range(17)],'length':ll,'arch':[2.7]*17,'second':[0.]*17,'third':[0.]*17};cases+=[cs,{**cs,'arch':[.7+math.sin(i*math.pi/8)**2*2 for i in range(17)]}];f,e=run(cases);print(json.dumps({'cases':len(cases),'records':sum(map(len,e)),'failed':len(f),'first':f[:4]}));(ROOT/'evidence/building-production/initial-general-failures.json').write_text(json.dumps({'failed':len(f),'failures':f},indent=2)+'\n')
