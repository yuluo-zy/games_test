import sys,re,json
sys.path.insert(0,'/tmp/c3');sys.path.insert(0,'/home/ubuntu/test/games_test/reverse-engineering/tiny-glade/tools/country-core-audit')
from rdva import rd
M=(1<<64)-1
ids=open('/tmp/c3/ids.txt').read().split()
def fnv1a(b):
    h=0xcbf29ce484222325
    for c in b: h=((h^c)*0x100000001b3)&M
    return h
F={fnv1a(s.encode()):s for s in ids}
X={int(k):v for k,v in json.load(open('/tmp/c3/hash2name.json')).items()}
src=open(sys.argv[1],errors='ignore').read().replace('\r','')
def sub(m):
    v=int(m.group(0),16)
    if v>=1<<63 and len(m.group(0))<=18: pass
    if v in F: return f'{m.group(0)}/*F:{F[v]}*/'
    if v in X: return f'{m.group(0)}/*X:{X[v]}*/'
    w=(-v)&M if False else None
    return m.group(0)
src=re.sub(r'0x[0-9a-fA-F]{12,16}\b',sub,src)
def sub2(m):
    a=int(m.group(1),16)
    try: s=rd(a,60).split(b'\0')[0]
    except Exception: return m.group(0)
    return f'{m.group(0)}/*"{s[:48].decode("latin1")}"*/'
src=re.sub(r'&UNK_([0-9a-f]{9})',sub2,src)
from cca import load_procs
import os
_P=None
def pname(a):
    global _P
    if _P is None: _P=load_procs()
    if a in _P:
        n=_P[a][1]
        n=n.replace('country_core::render::pass_builder::','').replace('rhapsody::','')
        return n[:130]
    return None
def fsub(m):
    a=int(m.group(2),16)
    n=pname(a)
    return f'{m.group(1)}{m.group(2)}' + (f'/*{n}*/' if n else '')
src=re.sub(r'(FUN_|func_0x0)([0-9a-f]{9})',fsub,src)
def usub(m):
    try: v=int.from_bytes(rd(int(m.group(1),16),4),'little')
    except Exception: return m.group(0)
    return f'_UNK_{m.group(1)}/*=0x{v:x}*/'
src=re.sub(r'_UNK_([0-9a-f]{9})',usub,src)
src=re.sub(r'\n\s*undefined[0-9]* [A-Za-z_0-9\[\] ]+;','',src)
sys.stdout.write(src)
