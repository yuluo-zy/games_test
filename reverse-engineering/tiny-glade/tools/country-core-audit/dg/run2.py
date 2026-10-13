import emu,struct,sys,re,collections
emu.BCMP=0x142923bf0
from emu import *
import run1
leafs=sorted(a for a,(mn,ops,ann) in INS.items() if mn=='mov' and ops and ops[0]=='dword ptr [rbp + 0x390]' and 0x1409bb2f5<=a<0x1409bd530)
print(len(leafs),'leaf stores')
hits=collections.defaultdict(list)
def go(name):
    e=Emu(name.encode())
    for i,a in run1.XM.items(): e.x[f'xmm{i}']=int.from_bytes(rd(a,16),'little')
    e.x['xmm7']=int.from_bytes(rd(0x142abe2f0,4),'little')
    stops={0x1409bc717}|{a for a in INS if a>=0x1409bd530}
    try: pc,_=e.run(0x1409bb30c,stops)
    except Exception as ex: return None
    if pc!=0x1409bc717: return None
    lf=[a for a in e.visited if a in set(leafs)]
    return lf[-1]
if __name__=='__main__':
    for c in run1.cands:
        l=go(c)
        if l: hits[l].append(c)
    for l in leafs:
        print(hex(l),hits.get(l))
