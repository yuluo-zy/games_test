import emu,struct,sys,re
emu.BCMP=0x142923bf0
from emu import *
sys.path.insert(0,'/home/ubuntu/test/games_test/reverse-engineering/tiny-glade/tools/country-core-audit')
from rdva import rd
# candidate names
cands=open('/tmp/dg/cands.txt').read().split()
XM={6:0x142abe160,8:0x142abe190,9:0x142abe150,11:0x142abe120,12:0x142abe130,13:0x142abe140,14:0x142abe140-0x80+0x0,10:0x142abe0d0}
XM[14]=0x142abe0c0
def run(name):
    e=Emu(name.encode())
    for i,a in XM.items(): e.x[f'xmm{i}']=int.from_bytes(rd(a,16),'little')
    e.x['xmm7']=int.from_bytes(rd(0x142abe2f0,4),'little')
    stops={0x1409bc717}|{a for a in INS if a>=0x1409bd530}
    try:
        pc,_=e.run(0x1409bb30c,stops)
    except Exception as ex:
        return ('ERR',str(ex),None)
    if pc!=0x1409bc717: return ('PANIC',hex(pc),None)
    k=struct.unpack('<I',e.rdmem(e.r['rbp']+0x380,4))[0]; v=struct.unpack('<I',e.rdmem(e.r['rbp']+0x384,4))[0]
    fn=struct.unpack('<Q',e.rdmem(e.r['rbp']+0x388,8))[0]; key=struct.unpack('<I',e.rdmem(e.r['rbp']+0x390,4))[0]
    return ('OK',(k,v,key,fn),None)
if __name__=='__main__':
    for c in cands:
        print(c,run(c))
