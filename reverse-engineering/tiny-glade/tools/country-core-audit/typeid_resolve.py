import sys,struct,re,bisect
import os; sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from cca import EXE,BASE,PROCS_TSV
e=open(EXE,'rb').read()
# procs
P=[]
for l in open(PROCS_TSV, encoding='utf8'):
    p=l.rstrip('\n').split('\t')
    if len(p)>=5: P.append((int(p[0],16),int(p[1]),p[4]))
P.sort(); starts=[x[0] for x in P]
pe=struct.unpack_from('<I',e,0x3c)[0]; n=struct.unpack_from('<H',e,pe+6)[0]; o=struct.unpack_from('<H',e,pe+20)[0]
secs=[]
for i in range(n):
    vs,v,rs,rp=struct.unpack_from('<IIII',e,pe+24+o+40*i+8); secs.append((vs,v,rs,rp))
def off2va(off):
    for vs,v,rs,rp in secs:
        if rp<=off<rp+rs: return BASE+v+off-rp
def lookup(va):
    i=bisect.bisect_right(starts,va)-1
    if i>=0 and va<P[i][0]+P[i][1]: return P[i]
def find(u):
    b=struct.pack('<Q',u&0xffffffffffffffff)
    res=[];s=0
    while True:
        k=e.find(b,s)
        if k<0:break
        s=k+1
        va=off2va(k)
        if va: res.append(va)
    return res
if __name__=='__main__':
    for h in sys.argv[1:]:
        u=int(h,16)
        for va in find(u):
            f=lookup(va)
            print(h,hex(va),(hex(f[0]),f[2][:200]) if f else None)

def resolve(hi,lo):
    fs=lambda u:{lookup(v)[0]:lookup(v)[2] for v in find(u) if lookup(v)}
    a=fs(hi);b=fs(lo)
    out=[]
    for k in a:
        if k in b and 'build_draw_lists' not in a[k] and 'draw_shaders' not in a[k]:
            out.append(a[k])
    # extract candidate type: generic arg of init_state/register_info/etc
    cands={}
    for s in out:
        m=re.search(r'(init_state|register_info|component_ids|init_resource|get_resource|resource_id|register_component|register_resource|init_component)<([^,>]+(?:<[^>]*>)?)',s)
        if m: cands[m.group(2)]=cands.get(m.group(2),0)+1
        m=re.search(r'impl\$\d+::register_required_components',s)
    return cands,out[:3]

def resolve2(hi,lo):
    fs=lambda u:{lookup(v)[0]:lookup(v)[2] for v in find(u) if lookup(v)}
    a=fs(hi);b=fs(lo)
    cnt={}
    for k in a:
        if k in b and 'build_draw_lists' not in a[k] and 'draw_shaders' not in a[k]:
            for t in re.findall(r'(?:enum2\$<)?(?:country_core|utils|bevy_[a-z_]+|glam)::[\w:]+',a[k]):
                if t.endswith('::closure_env') or '::impl$' in t or 'closure' in t: continue
                cnt[t]=cnt.get(t,0)+1
    return sorted(cnt.items(),key=lambda x:-x[1])[:4]
