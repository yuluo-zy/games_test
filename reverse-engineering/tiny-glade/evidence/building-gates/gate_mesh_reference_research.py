"""Numerical Gate mesh hypothesis, not accepted Rust/full Gate owner.
Raw inputs only; every output is compared with full original CPU constructor.
"""
import struct,math,bisect
F=lambda x:struct.unpack('<f',struct.pack('<f',x))[0]
A=lambda a,b:F(a+b)
S=lambda a,b:F(a-b)
M=lambda a,b:F(a*b)
D=lambda a,b:F(a/b)
Q=lambda a:F(math.sqrt(a))
pack=lambda xs:b''.join(struct.pack('<f',v)for row in xs for v in row).hex()

def distance(a,b):
 dx,dy,dz=[S(x,y)for x,y in zip(a,b)]
 return Q(A(A(M(dy,dy),M(dx,dx)),M(dz,dz)))
def curve(pts):
 length=-0.;dist=[0.]
 for a,b in zip(pts,pts[1:]):length=A(length,distance(a,b));dist.append(length)
 return pts,[D(v,length)for v in dist],length

def coord(us,u):
 i=bisect.bisect_right(us,u)-1;i=max(0,min(i,len(us)-2));return i,D(S(u,us[i]),S(us[i+1],us[i]))
def sample(pts,us,h,u):
 i,t=coord(us,u);p=[A(M(S(b,a),t),a)for a,b in zip(pts[i],pts[i+1])];hh=A(M(S(h[i+1],h[i]),t),h[i]);return p,hh

def normal(pts):
 dx=S(pts[-1][0],pts[0][0]);dz=S(pts[-1][2],pts[0][2]);inv=D(1.,Q(A(M(dz,dz),M(dx,dx))))
 return [M(-inv,dz),0.,M(inv,dx)]

def mesh_reference(segment,scalar):
 old=[[F(x)for x in p]for p in segment['positions']];h=list(map(F,segment['heights']));n=len(old);_,old_us,total=curve(old)
 minimum=min(p[1]for p in old);highest=max(0.,max(A(p[1],y)for p,y in zip(old,h)));floor=max(minimum,S(highest,A(F(scalar),.5)))
 bottom=[]
 for t in old_us:
  omt=S(1.,t);bottom.append([A(M(t,old[-1][0]),M(omt,old[0][0])),A(M(t,floor),M(omt,floor)),A(M(t,old[-1][2]),M(omt,old[0][2]))])
 pts,us,length=curve(bottom);hh=[S(y,S(p[1],o[1]))for y,p,o in zip(h,pts,old)];norm=normal(pts);offset=F(.1680000126361847)
 pts=[[A(v,M(k,offset))for v,k in zip(p,norm)]for p in pts]
 wide=length>F(2.7);gap=min(D(F(.05),length),F(.1));count=max(n//2,2)if wide else n
 intervals=[(0.,S(.5,gap),count,True),(A(.5,gap),1.,count,True)]if wide else[(0.,1.,count,False)]
 color=[F(.1704),F(.114),F(.0348)];dark=[F(.0994),F(.0665),F(.0203)];norm=normal(pts);meshes=[]
 def create_quad(out,a,b,ua,ub,ca):
  pa,ha=a;pb,hb=b;up=lambda p,y:[A(x,M(k,y))for x,k in zip(p,[0.,1.,0.])];positions=[pa,pb,up(pb,hb),up(pa,ha)];uv=[[ua,0.],[ub,0.],[ub,hb],[ua,ha]]
  for i in[0,1,2,0,2,3]:
   out['positions'].append(positions[i]);out['normals'].append(norm);out['colors'].append(ca);out['uvs'].append(uv[i])
 for ordinal,(start,end,cnt,flip)in enumerate(intervals):
  out={k:[]for k in['positions','normals','colors','uvs']};samples=[]
  for j in range(cnt):
   t=D(F(j),S(F(cnt),1.));u=A(M(t,end),M(S(1.,t),start));samples.append((u,sample(pts,us,hh,u)))
  for (ua,a),(ub,b)in zip(samples,samples[1:]):
   ta=S(end,ua)if flip and start==0. else S(ua,start)if flip else ua;tb=S(end,ub)if flip and start==0. else S(ub,start)if flip else ub
   create_quad(out,a,b,M(ta,length),M(tb,length),color)
  if wide and ordinal==1:
   stops=[S(.5,gap),.5,A(.5,gap)];ss=[]
   for j,u in enumerate(stops):
    p,y=sample(pts,us,hh,u)
    if j==1:p=[A(v,M(k,F(-.1)))for v,k in zip(p,norm)]
    ss.append((u,(p,y)))
   for(ua,a),(ub,b)in zip(ss,ss[1:]):create_quad(out,a,b,M(ua,length),M(ub,length),dark)
  attrs={name:pack(out[key])for name,key in [('Vertex_Position','positions'),('Vertex_Normal','normals'),('Vertex_Color','colors'),('Vertex_UV','uvs')]};meshes.append(dict(indices=struct.pack('<'+'I'*len(out['positions']),*range(len(out['positions']))).hex(),attributes=attrs))
 top=[[A(x,M(k,y))for x,k in zip(p,[0.,1.,0.])]for p,y in zip(pts,hh)]
 return dict(meshes=meshes,top_points=pack(top),bottom=bottom,heights=hh,length=length)
