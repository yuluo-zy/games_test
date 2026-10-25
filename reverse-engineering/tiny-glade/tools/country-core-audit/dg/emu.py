import re,struct,sys
sys.path.insert(0,'/home/ubuntu/test/games_test/reverse-engineering/tiny-glade/tools/country-core-audit')
from rdva import rd
M64=(1<<64)-1
INS={}
for l in open('/tmp/bdl.txt'):
    m=re.match(r'^([0-9a-f]{9}): (\S+)\t?(.*)$',l.rstrip('\n'))
    if not m: continue
    a=int(m.group(1),16); mn=m.group(2); rest=m.group(3)
    ann=None
    mm=re.search(r'#\s*(0x[0-9a-f]+)',rest)
    if mm: ann=int(mm.group(1),16)
    ops=rest.split('#')[0]
    ops=re.sub(r'\s{3,}.*$','',ops).strip()
    # strip trailing symbol after jump target e.g. "0x1409ba630 <.text+0x..>"
    ops=re.sub(r'\s*<[^>]*>.*$','',ops)
    oplist=[o.strip() for o in re.split(r',\s*(?![^\[]*\])',ops)] if ops else []
    INS[a]=(mn,oplist,ann)
addrs=sorted(INS)
NEXT={a:b for a,b in zip(addrs,addrs[1:])}
R64=['rax','rcx','rdx','rbx','rsp','rbp','rsi','rdi']+[f'r{i}' for i in range(8,16)]
def regmap():
    d={}
    names32=['eax','ecx','edx','ebx','esp','ebp','esi','edi']
    names16=['ax','cx','dx','bx','sp','bp','si','di']
    names8=['al','cl','dl','bl','spl','bpl','sil','dil']
    for i,r in enumerate(R64):
        d[r]=(r,64)
        if i<8:
            d[names32[i]]=(r,32); d[names16[i]]=(r,16); d[names8[i]]=(r,8)
        else:
            d[r+'d']=(r,32); d[r+'w']=(r,16); d[r+'b']=(r,8)
    return d
RM=regmap()
class Emu:
    def __init__(s,name):
        s.r={r:0 for r in R64}; s.x={f'xmm{i}':0 for i in range(16)}
        s.mem={}
        s.zf=s.cf=s.sf=s.of=0
        s.NAME=0x70000000; s.r['r13']=s.NAME; s.r['r14']=len(name); s.r['rdx']=len(name)
        s.r['rbp']=0x10000000; s.r['rsp']=0x20000000
        s.name=name; s.visited=[]
        for i,c in enumerate(name+b'\0'*32): s.mem[s.NAME+i]=c
    def rdmem(s,a,n):
        if 0x140000000<=a<0x150000000: return rd(a,n)
        return bytes(s.mem.get(a+i,0) for i in range(n))
    def wrmem(s,a,b):
        for i,c in enumerate(b): s.mem[a+i]=c
    def addr(s,op):
        m=re.search(r'\[(.*)\]',op).group(1)
        if m.startswith('rip'):
            return s.cur_ann
        t=m.replace(' ','')
        tot=0
        for sign,term in re.findall(r'([+-]?)([^+-]+)',t):
            sg=-1 if sign=='-' else 1
            if '*' in term:
                rr,sc=term.split('*'); 
                if rr[0].isdigit(): rr,sc=sc,rr
                tot+=sg*s.getreg(rr)*int(sc)
            elif term.startswith('0x'): tot+=sg*int(term,16)
            elif term in RM: tot+=sg*s.getreg(term)
            else: tot+=sg*int(term)
        return tot&M64
    def getreg(s,n):
        r,w=RM[n]; v=s.r[r]
        return v&((1<<w)-1)
    def setreg(s,n,v):
        r,w=RM[n]
        if w==64: s.r[r]=v&M64
        elif w==32: s.r[r]=v&0xffffffff
        else:
            mk=(1<<w)-1; s.r[r]=(s.r[r]&~mk)|(v&mk)
    def width(s,op):
        if op in RM: return RM[op][1]
        for k,w in (('xmmword',128),('qword',64),('dword',32),('word',16),('byte',8)):
            if op.startswith(k+' ptr'): return w
        return None
    def val(s,op,w=None):
        if op in RM: return s.getreg(op)
        if op in s.x: return s.x[op]
        if re.match(r'^-?0x[0-9a-f]+$|^-?\d+$',op):
            v=int(op,0); return v&((1<<(w or 64))-1)
        ww=s.width(op)
        b=s.rdmem(s.addr(op),ww//8)
        return int.from_bytes(b,'little')
    def setval(s,op,v,w=None):
        if op in RM: s.setreg(op,v); return
        if op in s.x: s.x[op]=v&((1<<128)-1); return
        ww=s.width(op) 
        s.wrmem(s.addr(op),(v&((1<<ww)-1)).to_bytes(ww//8,'little'))
    def flags(s,res,w,cf=0,of=0):
        mk=(1<<w)-1; res&=mk
        s.zf=int(res==0); s.sf=(res>>(w-1))&1; s.cf=cf; s.of=of
    def cond(s,mn):
        c=mn[1:]
        return {'e':s.zf,'z':s.zf,'ne':not s.zf,'nz':not s.zf,'b':s.cf,'ae':not s.cf,'a':(not s.cf and not s.zf),'be':(s.cf or s.zf),
         'l':s.sf!=s.of,'ge':s.sf==s.of,'g':(not s.zf and s.sf==s.of),'le':(s.zf or s.sf!=s.of),'s':s.sf,'ns':not s.sf}[c]
    def run(s,start,stops,maxn=3000):
        pc=start; n=0; trace=[]
        while n<maxn:
            n+=1
            if pc in stops: return pc,trace
            s.visited.append(pc)
            mn,ops,ann=INS[pc]; s.cur_ann=ann; nxt=NEXT[pc]
            if mn=='nop' or mn=='nopw' or mn=='nopl' or mn.startswith('nop'): pc=nxt; continue
            if mn=='mov' or mn=='movabs' or mn=='movzx' or mn=='movsxd' or mn=='movdqu' or mn=='movdqa' or mn=='movaps' or mn=='movd' or mn=='movq':
                d,sr=ops
                if mn=='movsxd':
                    v=s.val(sr); v=v-(1<<32) if v>>31 else v; s.setval(d,v&M64)
                elif mn=='movzx': s.setval(d,s.val(sr))
                elif mn in('movd','movq'):
                    s.setval(d,s.val(sr))
                else:
                    w=s.width(d) or s.width(sr)
                    s.setval(d,s.val(sr,w))
                pc=nxt; continue
            if mn=='lea':
                d,sr=ops; s.setval(d,s.addr(sr)); pc=nxt; continue
            if mn in('xor','or','and','add','sub'):
                d,sr=ops; w=s.width(d) or 64
                a=s.val(d); b=s.val(sr,w)
                if sr not in RM and re.match(r'^-',sr): b&=(1<<w)-1
                if mn=='xor': r=a^b
                elif mn=='or': r=a|b
                elif mn=='and': r=a&b
                elif mn=='add': r=a+b
                else: r=a-b
                s.flags(r,w,cf=int(mn=='sub' and a<b)); s.setval(d,r); pc=nxt; continue
            if mn in('cmp','test'):
                a,b=ops; w=s.width(a) or 64
                va=s.val(a); vb=s.val(b,w)
                if mn=='cmp':
                    r=va-vb; mk=(1<<w)-1
                    sa=va>>(w-1)&1; sb=vb>>(w-1)&1; sr_=(r&mk)>>(w-1)&1
                    s.flags(r,w,cf=int(va<vb),of=int(sa!=sb and sr_!=sa))
                else: s.flags(va&vb,w)
                pc=nxt; continue
            if mn=='inc' or mn=='dec':
                d=ops[0]; w=s.width(d); v=s.val(d)+(1 if mn=='inc' else -1); s.flags(v,w,cf=s.cf); s.setval(d,v); pc=nxt; continue
            if mn=='shl':
                d,c=ops; w=s.width(d); s.setval(d,s.val(d)<<int(c,0)); pc=nxt; continue
            if mn.startswith('set'):
                s.setval(ops[0],int(bool(s.cond('j'+mn[3:])))); pc=nxt; continue
            if mn=='pcmpeqb':
                a=s.val(ops[0]).to_bytes(16,'little'); b=(s.val(ops[1]) if ops[1] in s.x else s.val(ops[1])).to_bytes(16,'little')
                s.setval(ops[0],int.from_bytes(bytes(255 if x==y else 0 for x,y in zip(a,b)),'little')); pc=nxt; continue
            if mn=='pand': s.setval(ops[0],s.val(ops[0])&s.val(ops[1])); pc=nxt; continue
            if mn=='pmovmskb':
                b=s.val(ops[1]).to_bytes(16,'little'); s.setval(ops[0],sum((1<<i) for i,x in enumerate(b) if x&0x80)); pc=nxt; continue
            if mn=='jmp':
                if ops[0].startswith('0x'): pc=int(ops[0],16); continue
                raise Exception('indirect jmp %x'%pc)
            if mn.startswith('j'):
                pc=int(ops[0],16) if s.cond(mn) else nxt; continue
            if mn=='call':
                # bcmp
                tgt=int(ops[0],16)
                if tgt==BCMP:
                    a=s.rdmem(s.r['rcx'],s.r['r8']); b=s.rdmem(s.r['rdx'],s.r['r8']); s.r['rax']=0 if a==b else 1
                    pc=nxt; continue
                raise Exception('call %x at %x'%(tgt,pc))
            raise Exception('unhandled %s %s at %x'%(mn,ops,pc))
        raise Exception('maxn')
BCMP=None
