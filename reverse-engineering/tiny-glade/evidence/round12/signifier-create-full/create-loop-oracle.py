from pathlib import Path
import pefile,struct,json
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
ROOT=Path(__file__).resolve().parent;raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes();pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase
c=Uc(UC_ARCH_X86,UC_MODE_64);c.mem_map(base,(pe.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);c.mem_write(base,pe.get_memory_mapped_image());DATA,STACK,STOP,FN=0x20000000,0x30000000,0x50000000,0x50000100
for p,n in [(DATA,0x100000),(STACK,0x100000),(STOP,0x1000)]:c.mem_map(p,n)
def q(a):return struct.unpack('<Q',c.mem_read(a,8))[0]
def put(a,fmt,*v):c.mem_write(a,struct.pack(fmt,*v))
def ret(v=None):
 if v is not None:c.reg_write(UC_X86_REG_RAX,v)
 s=c.reg_read(UC_X86_REG_RSP);p=q(s);c.reg_write(UC_X86_REG_RSP,s+8);c.reg_write(UC_X86_REG_RIP,p)
wakes=[];entities=[]
def hook(cpu,pc,size,user):
 if pc==0x140b2d856:c.emu_stop();return
 if pc==FN:
  out=cpu.reg_read(UC_X86_REG_RCX);record=cpu.reg_read(UC_X86_REG_RDI);nameptr=q(record+8);n=q(record+16);wakes.append(bytes(cpu.mem_read(nameptr,n)).decode())
  p=DATA+0x8000+len(wakes)*0x100
  for i,e in enumerate(entities):put(p+i*8,'<Q',e)
  put(out,'<3Q',len(entities),p,len(entities));ret(out)
 elif pc==0x1405b0d70:ret(0) # std HashSet probe hasher, fixed bucket0 fixture
 elif pc in (0x140613c20,):ret()
 elif pc in (0x142923c00,0x142923c10):
  dst,src,n=[cpu.reg_read(r)for r in(UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8)];cpu.mem_write(dst,bytes(cpu.mem_read(src,n)));ret(dst)
c.hook_add(UC_HOOK_CODE,hook)
def run(names,widget=11,returned=(0x100000007,0x100000008),existing_hash=None):
 wakes.clear();entities[:]=returned;c.mem_write(DATA,bytes(0x10000));c.mem_write(STACK,bytes(0x100000));rbp=STACK+0x80000;sp=rbp-0x100;state=DATA+0x1000;records=DATA+0x2000;vt=DATA+0x3000;commands=DATA+0x4000;queue=DATA+0x5000;buf=DATA+0x6000
 put(rbp+0x250,'<Q',state);put(state+0x38,'<Q',widget);put(rbp+0x158,'<Q',records+40*len(names));put(rbp+0x160,'<Q',DATA+0xa100);put(rbp+0x168,'<Q',DATA+0xa200);put(rbp+0x248,'<Q',commands);put(commands+8,'<Q',queue);put(queue,'<3Q',0x2000,buf,0)
 # Existing-query hash set is empty. Native branch executes its direct path.
 put(rbp+0x188,'<Q',0)
 if existing_hash is not None:
  ctrl=DATA+0xd000;put(rbp+0x170,'<4Q',ctrl,15,0,1);c.mem_write(ctrl,bytes([0]+[255]*31));put(ctrl-8,'<Q',existing_hash)
 put(vt+0x10,'<Q',8);put(vt+0x28,'<Q',FN)
 for i,n in enumerate(names):
  b=n.encode();p=DATA+0xb000+i*0x100;c.mem_write(p,b or b'\0');put(records+40*i,'<5Q',len(b),p,len(b),DATA+0xc000+i*0x100,vt)
 c.reg_write(UC_X86_REG_RBP,rbp);c.reg_write(UC_X86_REG_RSP,sp);c.reg_write(UC_X86_REG_RDI,records);c.reg_write(UC_X86_REG_RBX,rbp-0x40);c.reg_write(UC_X86_REG_XMM6,(1<<128)-1)
 c.emu_start(0x140b2d670,0x140b2d856,count=200000)
 assert c.reg_read(UC_X86_REG_RIP)==0x140b2d856
 n=q(queue+16);data=bytes(c.mem_read(buf,n));cmds=[]
 for i in range(0,n,48):cmds.append(dict(entity=int.from_bytes(data[i+16:i+24],'little'),hash=int.from_bytes(data[i+24:i+32],'little'),widget=int.from_bytes(data[i+32:i+40],'little'),raw=data[i:i+48].hex()))
 return dict(names=names,widget=widget,returned=list(returned),existing_hash=existing_hash,wakes=list(wakes),commands=cmds)
if __name__=='__main__':
 rows=[run(n,w,e)for n in [[],[''],['a'],['a','a'],['a','b','a']]for w in [0,11,91]for e in [(),(0x100000007,),(0x100000007,0x100000008)]]
 hashes=json.loads((ROOT/'native-hash-fixtures.json').read_text())
 for w in [0,11,91]:
  h=run(['water'],w)['commands'][0]['hash']
  for names in [['water','water'],['wall','water'],['water','wall','water']]:rows.append(run(names,w,existing_hash=h))
 for r in rows:
  want=r['names'] if r['existing_hash'] is None else [n for n in r['names'] if n!='water']
  assert r['wakes']==want;assert len(r['commands'])==len(want)*len(r['returned']);assert all(x['widget']==r['widget']for x in r['commands'])
 (ROOT/'native-create-loop-fixtures.json').write_text(json.dumps(rows,indent=2));print(len(rows),'original create-loop fixtures; duplicate wake and both SignifierId fields passed')
