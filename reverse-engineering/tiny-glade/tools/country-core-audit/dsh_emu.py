import sys, re, subprocess
DIS='/home/ubuntu/test/games_test/reverse-engineering/tiny-glade/tools/country-core-audit/dis.py'
addr=sys.argv[1]
out=subprocess.run(['python3','-I',DIS,addr],capture_output=True,text=True).stdout.splitlines()
canon={}
for n in ['ax','cx','dx','bx','si','di','bp','sp']:
    for p in ['r','e','']:
        canon[p+n]='r'+n
    pass
for n in ['al','cl','dl','bl']: canon[n]='r'+n[0]+'x'
for n in ['sil','dil']: canon[n]='r'+n[:2]
for i in range(8,16):
    for s in ['','d','w','b']: canon[f'r{i}{s}']=f'r{i}'
def cr(r): return canon.get(r,r)
def imm(s):
    s=s.strip()
    try: return int(s,0)
    except: return None
reg={}; mem={}
def u64(v): return v & 0xFFFFFFFFFFFFFFFF
def val(op):
    op=op.strip()
    m=re.match(r'^(-?0x[0-9a-f]+|-?\d+)$',op)
    if m: return ('imm',imm(op))
    if op in canon: return reg.get(cr(op))
    m=re.match(r'^(?:\w+ ptr )?\[(r\w+)(?: ([+-]) (0x[0-9a-f]+))?\]$',op)
    if m:
        b=cr(m.group(1)); o=0 if not m.group(2) else int(m.group(3),16)*(1 if m.group(2)=='+' else -1)
        return mem.get((b,o))
    return None
def addr_of(op):
    m=re.match(r'^\[(r\w+)(?: ([+-]) (0x[0-9a-f]+))?\]$',op.strip())
    if not m: return None
    b=cr(m.group(1)); o=0 if not m.group(2) else int(m.group(3),16)*(1 if m.group(2)=='+' else -1)
    return (b,o)
def show(v):
    if v is None: return '?'
    if v[0]=='imm': return hex(u64(v[1])) if v[1]<0 or v[1]>9 else str(v[1])
    if v[0]=='lea': 
        m=mem.get(v[1])
        return f'&{v[1][0]}{v[1][1]:+#x}' + (f'[={show(m)}]' if m else '')
    if v[0]=='str': return 'str'+repr(v[1])
    return str(v)
for line in out:
    if line.startswith('#'): print(line); continue
    m=re.match(r'^([0-9a-f]+):\s+(\S+)\t?(.*)$',line.rstrip())
    if not m: continue
    a,mn,rest=m.groups()
    # split annotation
    ann=''
    if '  ' in rest.strip():
        pass
    parts=rest.split('#',1)
    opstr=parts[0]
    ann=parts[1].strip() if len(parts)>1 else ''
    ops=re.split(r',\s*(?![^\[]*\])',opstr.strip())
    # annotation after many spaces
    mm=re.match(r'^(.*?)(\s{3,}.*)$',opstr)
    if mm:
        opstr=mm.group(1); ann=(mm.group(2).strip()+' '+ann).strip()
        ops=re.split(r',\s*(?![^\[]*\])',opstr.strip())
    if mn in('mov','movabs','movzx') and len(ops)==2:
        d,s=ops[0].strip(),ops[1].strip()
        v=val(s)
        if d in canon:
            reg[cr(d)]=v
        else:
            ad=addr_of(re.sub(r'^\w+ ptr ','',d))
            if ad: mem[ad]=v
    elif mn=='lea' and len(ops)==2:
        d=ops[0].strip(); ad=addr_of(ops[1])
        if ad:
            reg[cr(d)]=('lea',ad)
            if ann.startswith('0x') or 'str=' in ann:
                sm=re.search(r"str=(b'.*')",ann)
                if sm and ad[0]=='rip': reg[cr(d)]=('str',sm.group(1)[:60])
        if ad and ad[0]=='rip':
            sm=re.search(r"str=(b'.*')",line)
            if sm: reg[cr(d)]=('str',sm.group(1)[:70])
    elif mn in('xor',) and len(ops)==2 and ops[0].strip()==ops[1].strip() and ops[0].strip() in canon:
        reg[cr(ops[0].strip())]=('imm',0)
    elif mn in ('or','and') and len(ops)==2: 
        d=ops[0].strip()
        if d in canon: reg[cr(d)]=None
    if mn=='call' and ('142923c00' in rest or 'drop_in_place' in rest):
        for r in ['rax','rcx','rdx','r8','r9','r10','r11']: reg[r]=None
        continue
    if mn=='call':
        tgt=re.sub(r'\s+',' ',(ann+' '+rest))
        nm=re.search(r'(\S*::\S+|\S+::\S+)',rest)
        name=re.sub(r'<\.text\+0x[0-9a-f]+>','',rest).strip()
        name=re.sub(r'^(0x[0-9a-f]+)\s+',r'\1 ',name)
        args=[show(reg.get(r)) for r in ['rcx','rdx','r8','r9']]
        st=[show(mem.get(('rsp',o))) for o in (0x20,0x28,0x30)]
        print(f'{a} CALL {name[:200]}\n      rcx={args[0]} rdx={args[1]} r8={args[2]} r9={args[3]} stk20={st[0]} stk28={st[1]} stk30={st[2]}')
        for r in ['rax','rcx','rdx','r8','r9','r10','r11']: reg[r]=None
    elif mn=='ret':
        print(a,'ret'); break
    elif mn.startswith('j') or mn=='ret' or mn.startswith('cmp') or mn=='test':
        if mn=='ret' or mn.startswith('cmp') or mn=='test' or mn.startswith('j'):
            print(f'{a} {mn} {rest.strip()[:120]}')
