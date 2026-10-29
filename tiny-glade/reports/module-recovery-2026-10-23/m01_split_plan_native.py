"""执行完整原移出墙体函数，捕获原拆分计划/ID/操作顺序；几何closure另验。"""
from pathlib import Path
import struct,json,itertools
source=Path(__file__).with_name('m01_dispatch_start_place_native.py').read_text()
exec(compile(source[:source.index('\nrows=[]')],'identity-bound-pe-setup','exec'))
alloc_cursor=D+0x40000
def allocate(n):
 global alloc_cursor
 out=alloc_cursor;alloc_cursor=(alloc_cursor+max(n,16)+15)&~15;assert alloc_cursor<D+0xf0000;return out
def split_hook(uc,a,size,_):
 rcx,rdx,r8,r9=[r(x)for x in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_R9]]
 if a==0x140a1c800:ret(info)
 elif a==0x140b26970:ret(7)
 elif a==0x14125cf90:
  key=bytes(u.mem_read(rdx,2));rk=key[1];assert key[0]==7 and 1<=rk<=5,key.hex()
  lookups.append(rk);ret(D+0x20000+rk*128)
 elif a==0x140b5f870:
  oldcap,oldptr,oldlen=[uq(rcx+i*8)for i in range(3)];p=allocate(64)
  if oldlen:u.mem_write(p,bytes(u.mem_read(oldptr,oldlen*4)))
  q(rcx,16);q(rcx+8,p);ret()
 elif a==0x14127a480:
  desc=bytes(u.mem_read(r9,12));descriptors.append({'id':r8&0xffffffff,'tag':struct.unpack_from('<I',desc)[0],'offset_bits':list(struct.unpack_from('<II',desc,4))})
  # 仅隔离outer计划，不宣称closure坐标恢复；用原NoMove哨兵让outer沿源dst构造Place。
  q(rcx,3);ret(rcx)
 elif a==0x141276e30:
  raw=bytes(u.mem_read(rcx,112));ops.append({'kind':uq(rcx),'bytes':raw.hex(),'state':rdx&255,'third_arg16':bytes(u.mem_read(r8,16)).hex()});ret()
 elif a in [0x140613c10,0x1428d9000]:ret(allocate(rcx))
 elif a==0x140613c20:ret()
 elif a in [0x1428d9310,0x1428d9390,0x1428d9430]:raise RuntimeError('original panic '+hex(a))
u.hook_add(UC_HOOK_CODE,split_hook)
rows=[]
for ty,rank,part,attached,compound in itertools.product([56,57,58],range(2,6),range(5),[0,1,3],[False,True]):
 if part>=rank:continue
 alloc_cursor=D+0x40000;u.mem_write(D,bytes(0x100000));u.mem_write(S,bytes([0xa5])*0x100000)
 info=D+0x1000;rawinfo=bytearray(72)
 struct.pack_into('<2f',rawinfo,0,10.,5.);struct.pack_into('<Q',rawinfo,8,9)
 rawinfo[0x20]=attached;rawinfo[0x21]=1;rawinfo[0x22]=int(compound);rawinfo[0x44]=rank;rawinfo[0x46]=ty
 u.mem_write(info,bytes(rawinfo))
 for rk in range(1,6):
  record=D+0x20000+rk*128;u.mem_write(record+0x30,struct.pack('<4f',rk*1.3+rk*rk*.07,1.,-.2,rk*1.7+rk*rk*.11))
 out,addr,view,gen,baseid=D+0x2000,D+0x3000,D+0x4000,D+0x5000,D+0x5100
 q(addr,0);q(addr+8,88);q(view,D+0x8000);q(view+8,D+0x9000);q(gen,baseid);q(gen+8,0);u.mem_write(gen+16,b'\0');w(baseid,1000)
 sp=S+0xf0008;q(sp,STOP)
 for i,val in enumerate([42,D+0x6000,D+0x7000,view,gen,D+0xa000,D+0xb000,D+0xc000,D+0xd000,0]):q(sp+0x28+i*8,val)
 for reg,value in [(UC_X86_REG_RCX,out),(UC_X86_REG_RDX,part),(UC_X86_REG_R8,D+0xe000),(UC_X86_REG_R9,addr),(UC_X86_REG_RSP,sp)]:u.reg_write(reg,value)
 descriptors=[];ops=[];lookups=[];u.emu_start(0x141279410,STOP,count=100000);assert r(UC_X86_REG_RIP)==STOP
 length=uq(out+24);remaining=list(struct.unpack('<'+'I'*length,u.mem_read(uq(out+16),length*4)))if length else[]
 rows.append({'type':ty,'rank':rank,'part':part,'attached':attached,'compound':compound,'descriptors':descriptors,'lookups':lookups,'ops':ops,'moving_id':uw(out),'remaining':remaining,'generated':uq(gen+8)})
Path(__file__).with_name('m01-split-plan-native.json').write_text(json.dumps({'binary_sha256':sha,'entry':'0x141279410','bridges':['std HashMap Info/subtype/collision lookups with non-linear rank metrics','std Vec allocation/deallocation','14127a480 closure captured descriptor -> NoMove sentinel; geometry verified separately','141276e30 captured op112/third_arg16; consumer itself independently restored'], 'rows':rows},indent=2)+'\n')
lines=['# type rank part attached compound descriptors_id_tag_offsetxbits_offsetybits_rank_semicolon moving remaining generated']
for row in rows:
 desc=[]
 for d,op in zip(row['descriptors'],row['ops']):
  rawop=bytes.fromhex(op['bytes']);part_rank=rawop[97]if op['kind']==0 else rawop[94]
  desc.append(','.join(map(str,[d['id'],d['tag'],*d['offset_bits'],part_rank])))
 lines.append('\t'.join(map(str,[row['type'],row['rank'],row['part'],row['attached'],int(row['compound']),';'.join(desc),row['moving_id'],','.join(map(str,row['remaining'])),row['generated']])))
Path(__file__).with_name('m01-split-plan-native.tsv').write_text('\n'.join(lines)+'\n')
print(json.dumps({'cases':len(rows),'ops':sum(len(r['ops'])for r in rows),'descriptors':sum(len(r['descriptors'])for r in rows),'first':rows[0]}))
