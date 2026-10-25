import sys,re,subprocess,struct
sys.path.insert(0,'/home/ubuntu/test/games_test/reverse-engineering/tiny-glade/tools/country-core-audit')
from rdva import rd
DIS='/home/ubuntu/test/games_test/reverse-engineering/tiny-glade/tools/country-core-audit/dis.py'
def dis(a): return subprocess.run(['python3','-I',DIS,a],capture_output=True,text=True).stdout.splitlines()
def info(a):
    L=dis(a); name=L[0]
    res={'slot':None,'rarbuf':None,'id':None,'name':None,'full':None}
    # slot: lea r8,[rip..] # 0xADDR ; mov r9d,len before rustc_entry call
    last_lea={}
    for i,l in enumerate(L):
        m=re.search(r'lea\t(r\w+), \[rip \+ [^\]]*\]\s+# (0x[0-9a-f]+)',l)
        if m: last_lea[m.group(1)]=int(m.group(2),16)
        m2=re.search(r'mov\t(r9d|r9), (0x[0-9a-f]+)',l)
        if m2: last_len=int(m2.group(2),16)
        if 'rustc_entry' in l and res['slot'] is None:
            res['slot']=rd(last_lea['r8'],last_len).decode('latin1')
        m3=re.search(r'call\t(0x[0-9a-f]+)\s.*read_access_buffer<',l)
        if m3: res['rarbuf']=m3.group(1)
    return res,L
def rgbuf(a):
    L=dis(a)
    t=None
    for l in L:
        m=re.search(r'call\t(0x[0-9a-f]+)\s.*get_rg_buffer<',l)
        if m: t=m.group(1)
    return t
def getrg(a):
    L=dis(a); ids=[]; lea=[]
    for l in L:
        m=re.search(r'movabs\t\w+, (-?0x[0-9a-f]+)',l)
        if m: ids.append(int(m.group(1),16)&0xffffffffffffffff)
    return ids,L
if __name__=='__main__':
    for a in sys.argv[1:]:
        r,L=info(a)
        ra=r['rarbuf']; g=rgbuf(ra) if ra else None
        ids=[]
        if g:
            ids,GL=getrg(g)
        short=re.search(r'<(.*)>',L[0]).group(1).split('::')[-1]
        print(short,a,'slot=',repr(r['slot']),'rg=',g,[hex(i) for i in ids][:3])
