"""Original nonempty archivist restore: std table operations bridged, game branches run."""
import native_wall_history as n
import struct,json,hashlib
from pathlib import Path
from unicorn import UC_HOOK_CODE
from unicorn.x86_const import *
cpu=n.cpu;nf=n.nf;data=n.data;author=n.author
active=False;index={};stdlib=[];removed_by_address={};restore_entries=0;restore_returns=0;index_owner=0;removed_owner=0
def table_entries(owner,stride):
 ctrl,mask,growth,length=struct.unpack('<4Q',cpu.mem_read(owner,32));result=[]
 for i in range(mask+1):
  if cpu.mem_read(ctrl+i,1)[0]<128:result.append(bytes(cpu.mem_read(ctrl-(i+1)*stride,stride)))
 assert len(result)==length,(hex(owner),stride,len(result),length)
 return result
def table(owner,entries,stride):
 assert len(entries)<=14
 allocation=nf.alloc(16*stride+32);ctrl=allocation+16*stride;cpu.mem_write(ctrl,bytes([0]*len(entries)+[255]*(16-len(entries)))*2)
 for i,row in enumerate(entries):assert len(row)==stride;cpu.mem_write(ctrl-(i+1)*stride,row)
 cpu.mem_write(owner,struct.pack('<4Q',ctrl,15,14-len(entries),len(entries)))
def sync_index():
 table(index_owner,[struct.pack("<I4xQQ",k,*v)for k,v in index.items()],24)
def set_insert(owner,key):
 rows=table_entries(owner,4)
 if struct.pack('<I',key)not in rows:rows.append(struct.pack('<I',key))
 table(owner,rows,4)
def hook(c,pc,size,user):
 global restore_entries,restore_returns
 if not active:return
 if pc==0x140b2c4b0:restore_entries+=1
 if pc==0x140b2ca25:restore_returns+=1
 if pc==0x1405b0d70 and author.active:return # registered author std bridge already returned
 if pc in[0x1405b0d70,0x1405b0ed0,0x1412e77b0]:stdlib.append(hex(pc));nf.ret(0) # PDB std hash_one only
 elif pc==0x1400d9b20:nf.ret(data+0xe0000)
 elif pc in[0x140b716a0,0x140b72180]:
  target,source=[c.reg_read(r)for r in[UC_X86_REG_RCX,UC_X86_REG_RDX]];stride=4 if pc==0x140b716a0 else 80;table(target,table_entries(source,stride),stride);stdlib.append(hex(pc));nf.ret()
 elif pc==0x1402c9200:
  owner,key=[c.reg_read(r)for r in[UC_X86_REG_RCX,UC_X86_REG_RDX]];set_insert(owner,key&0xffffffff);stdlib.append(hex(pc));nf.ret(1)
 elif pc==0x140b776d0:
  owner,keyptr=[c.reg_read(r)for r in[UC_X86_REG_RCX,UC_X86_REG_R8]];key=bytes(c.mem_read(keyptr,4));rows=table_entries(owner,4);present=key in rows;table(owner,[r for r in rows if r!=key],4);stdlib.append(hex(pc));nf.ret(int(present))
 elif pc==0x1411fbfa0:
  output,owner,key,info=[c.reg_read(r)for r in[UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_R9]];key&=0xffffffff;rows=table_entries(owner,80);old=next((r[8:]for r in rows if struct.unpack('<I',r[:4])[0]==key),None);row=struct.pack('<I4x',key)+bytes(c.mem_read(info,72));rows=[r for r in rows if struct.unpack('<I',r[:4])[0]!=key]+[row];table(owner,rows,80);c.mem_write(output,old if old is not None else bytes(71)+bytes([2]));stdlib.append(hex(pc));nf.ret(output)
 elif pc==0x140b766e0:
  output,owner,keyptr=[c.reg_read(r)for r in[UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R9]];assert owner==removed_owner;tag,wall=struct.unpack('<2Q',c.mem_read(keyptr,16));key=(tag,wall if tag==0 else 0);removed=removed_by_address.pop(key,None)
  rows=table_entries(owner,64);old=next((row for row in rows if struct.unpack("<2Q",row[:16])==key),None);assert (old is None)==(removed is None)
  if removed is None:c.mem_write(output,struct.pack('<I',3)+bytes(60))
  else:
   c.mem_write(output,old)
   table(owner,[row for row in rows if row!=old],64)
  stdlib.append(hex(pc));nf.ret(output) # actual std map remove_entry
 elif pc==0x140b78140:
  output,owner,keyptr=[c.reg_read(r)for r in[UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R9]];assert owner==index_owner;key=struct.unpack('<I',c.mem_read(keyptr,4))[0];old=index.pop(key,None);sync_index();c.mem_write(output,struct.pack('<I4xQQ',key,3 if old is None else old[0],0 if old is None else old[1]));stdlib.append(hex(pc));nf.ret(output)
 elif pc==0x14152bc60:
  assert c.reg_read(UC_X86_REG_RCX)==index_owner
  key,tag,wall=[c.reg_read(r)for r in[UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_R9]];key&=0xffffffff;old=index.get(key);index[key]=(tag,wall);sync_index();stdlib.append(hex(pc));nf.ret(3 if old is None else old[0]);c.reg_write(UC_X86_REG_RDX,0 if old is None else old[1])
cpu.hook_add(UC_HOOK_CODE,hook)
def decorators(p,removed,items,added):
 table(p,[struct.pack('<I',i)for i in removed],4);table(p+0x30,[struct.pack('<I4x',i)+bytes.fromhex(info)for i,info in items],80);table(p+0x60,[struct.pack('<I',i)for i in added],4)
 for o in[0x20,0x50,0x80]:cpu.mem_write(p+o,bytes(16))
def decode(p):
 return {'removed':sorted(struct.unpack('<I',r)[0]for r in table_entries(p,4)),'items':sorted([struct.unpack('<I',r[:4])[0],r[8:].hex()]for r in table_entries(p+0x30,80)),'added':sorted(struct.unpack('<I',r)[0]for r in table_entries(p+0x60,4))}
def prepare_resources(archive,storage,archived,current,owner=0,missing_target=False,saved_removed=()):
 global restore_entries,restore_returns,index_owner,removed_owner
 index_owner=storage+0x60;removed_owner=storage+0x30
 restore_entries=0;restore_returns=0
 index.clear();stdlib.clear();removed_by_address.clear();removed_by_address[(owner,9 if owner==0 else 0)]=list(saved_removed)
 # Every queried storage owner is a real initialized Decorators object.
 wall=data+0xe000;decorators(wall,**current);value=bytes(cpu.mem_read(wall,144));table(storage,[]if missing_target and owner==0 else[struct.pack('<Q',9)+value],152);wallptr=nf.u64(storage)-144
 for tag in[1,2]:
  if missing_target and tag==owner:cpu.mem_write(storage+0x90+(tag-1)*0x90,bytes(144))
  else:decorators(storage+0x90+(tag-1)*0x90,**(current if tag==owner else {'removed':[],'items':[],'added':[]}))
 saved_owner=nf.alloc(48);table(saved_owner,[struct.pack("<I",i)for i in saved_removed],4);cpu.mem_write(saved_owner+32,bytes(16));table(removed_owner,[struct.pack("<2Q",owner,9 if owner==0 else 0)+bytes(cpu.mem_read(saved_owner,48))],64)
 target=wallptr if owner==0 else storage+0x90+(owner-1)*0x90
 if owner==0:
  p=data+0xe100;decorators(p,**archived);table(archive,[struct.pack('<Q',9)+bytes(cpu.mem_read(p,144))],152)
 else:
  table(archive,[],152);decorators(archive+0xc0+(owner-1)*0x90,**archived)
 # Collision archived maps empty valid tables, original clear calls execute.
 table(archive+0x60,[],128);table(archive+0x90,[],128)
 for key,info in ([]if missing_target else current['items']):index[key]=(owner,9 if owner==0 else 0x50)
 index[999]=(2,0x50) # unrelated live index must survive
 sync_index()
 return target
def run(archived,current,owner=0):
 global active
 author.reset();archive=data+0xa000;storage=data+0xb000;target=prepare_resources(archive,storage,archived,current,owner)
 cpu.reg_write(UC_X86_REG_RCX,archive);cpu.reg_write(UC_X86_REG_RDX,storage);active=True
 try:author.call(0x140b2c4b0)
 finally:active=False
 return collect(archive,target)
def collect(archive,target):
 physical={struct.unpack('<I',row[:4])[0]:struct.unpack('<2Q',row[8:])for row in table_entries(index_owner,24)};assert physical==index
 return {'actual_return':True,'target':decode(target),'index':sorted([k,*v]for k,v in physical.items()),'archive_wall_count':nf.u64(archive+24),'archive_special_present':[nf.u64(archive+0xc0)!=0,nf.u64(archive+0x150)!=0],'stdlib_bridges':sorted(set(stdlib)),'saved_removed_count':sum(len(v)for v in removed_by_address.values()),'restore_entries':restore_entries,'restore_returns':restore_returns}
if __name__=='__main__':
 info=bytes(72).hex();print(json.dumps(run({'removed':[8],'items':[[3,info]],'added':[3]},{'removed':[7],'items':[[2,info]],'added':[2]},0),indent=2))
