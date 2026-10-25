import re,sys,subprocess
DIS='/home/ubuntu/test/games_test/reverse-engineering/tiny-glade/tools/country-core-audit/dis.py'
def load(addr):
    out=subprocess.run(['python3','-I',DIS,addr],capture_output=True,text=True).stdout.splitlines()
    ins=[]
    for l in out:
        m=re.match(r'^([0-9a-f]+): (\S+)\t?(.*)$',l.rstrip())
        if not m: continue
        a,mn,rest=m.groups()
        sym=''
        ann=re.search(r'\s{3,}(.*)$',rest)
        if ann: sym=ann.group(1); rest=rest[:ann.start()]
        cm=re.search(r'#\s*(0x[0-9a-f]+)',rest)
        rest=rest.split('#')[0].strip()
        rest=re.sub(r'\s*<[^>]*>','',rest)
        ops=[o.strip() for o in re.split(r',\s*(?![^\[]*\])',rest)] if rest else []
        ins.append((int(a,16),mn,ops,sym,(cm.group(1) if cm else None)))
    return ins
CAN={}
for n in ['ax','cx','dx','bx','si','di','bp','sp']:
    for p in ['r','e','']: CAN[p+n]='r'+n
for n in ['al','cl','dl','bl']: CAN[n]='r'+n[0]+'x'
for n in ['sil','dil']: CAN[n]='r'+n[:2]
for i in range(8,16):
    for s in ['','d','w','b']: CAN[f'r{i}{s}']=f'r{i}'
def c(r): return CAN.get(r,r)
def resolve(ins,idx,reg,depth=0,win=60):
    reg=c(reg)
    if depth>4: return '?'
    for j in range(idx-1,max(idx-win,0),-1):
        a,mn,ops,sym,ann=ins[j]
        if mn in('call','ret','jmp') : 
            if mn=='call' and reg in('rax','rcx','rdx','r8','r9','r10','r11'): return f'ret({sym[:30]})'
            continue
        if not ops: continue
        d=c(ops[0]) if ops[0] in CAN else ops[0]
        if d!=reg: continue
        if mn in('mov','movabs','movzx','movsxd','movss','movsd','movaps','movups','movq','movd','movdqu') and len(ops)==2:
            return expr(ins,j,ops[1],depth)
        if mn=='lea': return '&'+expr(ins,j,ops[1],depth)
        if mn in('xor',) and ops[0]==ops[1]: return '0'
        if mn in('add','sub','or','and','shl','shr','neg','not','imul'):
            return f'({mn} {resolve(ins,j,reg,depth+1)} {expr(ins,j,ops[1],depth) if len(ops)>1 else ""})'
        return f'?{mn}'
    return reg+'@in'
def expr(ins,j,op,depth):
    m=re.match(r'^(?:\w+ ptr )?\[(.*)\]$',op)
    if m:
        inner=m.group(1).replace(' ','')
        mm=re.match(r'^(r\w+)([+-]0x[0-9a-f]+)?$',inner)
        if mm:
            base,off=mm.group(1),mm.group(2) or '+0x0'
            if base=='rbp': return f'M[rbp{off}]'
            if base=='rip': return 'M[rip]'
            b=resolve(ins,j,base,depth+1)
            return f'[{b}{off}]'
        return f'[{inner}]'
    if op in CAN: return resolve(ins,j,op,depth+1)
    return op
if __name__=='__main__':
    addr=sys.argv[1]; ins=load(addr)
    for i,(a,mn,ops,sym,ann) in enumerate(ins):
        if mn=='call':
            nm=sym
            if not nm: nm='?'
            short=re.sub(r'country_core::render::pass_builder::','',nm)[:110]
            args=[]
            for r in ('rcx','rdx','r8','r9'):
                args.append(f'{r}={resolve(ins,i,r)}')
            xs=resolve(ins,i,'xmm3')
            print(f'{a:x} {short}\n    '+' '.join(args)+f' xmm3={xs}')
