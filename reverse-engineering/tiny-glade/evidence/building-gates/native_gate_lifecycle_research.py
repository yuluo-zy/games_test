"""Whole original update_gates_hashmap. Only actual Bevy despawn transport
is observed/bridged; event iteration, map entry/drain/remove run original code.
The initial empty std map is copied from an actual initialized std container.
"""
import native_gates_research as n
from native_gates_research import *

def dependency_despawn(c,pc,size,user):
 if state.get('lifecycle_active')and pc==0x140d14770:
  param=c.reg_read(UC_X86_REG_RCX);entity=struct.unpack('<Q',c.mem_read(param+32,8))[0];state['despawned'].append(entity);ret(0)
cpu.hook_add(UC_HOOK_CODE,dependency_despawn)

def event_reader(payloads,width):
 resource=alloc(72);reader=alloc(8);param=alloc(16);count=len(payloads);p=alloc(max(8,count*width))
 cpu.mem_write(reader,bytes(8));cpu.mem_write(resource,struct.pack('<4Q',0,8,0,0)+struct.pack('<4Q',count,p,count,0)+struct.pack('<Q',count));cpu.mem_write(param,struct.pack('<QQ',reader,resource))
 if count:cpu.mem_write(p,b''.join(payloads))
 return param

def map_view(gates):
 control,mask,growth,length=struct.unpack('<4Q',cpu.mem_read(gates,32));rows=[]
 for i in range(mask+1):
  if cpu.mem_read(control+i,1)[0]>=128:continue
  entry=control-32*(i+1);wall,cap,p,n=struct.unpack('<4Q',cpu.mem_read(entry,32));rows.append(dict(wall_id=wall,cap=cap,entities=list(struct.unpack('<'+'Q'*n,cpu.mem_read(p,n*8))),entry=entry))
 assert len(rows)==length
 return rows

def update(gates,walls_changed=(),walls_deleted=(),create_roofs=(),switch_types=(),delete_roofs=()):
 readers=[event_reader([struct.pack('<QQ',i,w)for i,w in enumerate(walls_changed)],16),event_reader([struct.pack('<QQ',0,w)+bytes(48)for w in create_roofs],64),event_reader([struct.pack('<QQ',0,w)+bytes(16)for w in switch_types],32),event_reader([struct.pack('<QQ',i,w)for i,w in enumerate(walls_deleted)],16),event_reader([struct.pack('<QQ',0,w)+bytes(8)for w in delete_roofs],24)]
 res=alloc(32);ticks=alloc(8);cpu.mem_write(res,struct.pack('<3Q2I',gates,ticks,ticks+4,0,17));cpu.mem_write(ticks,bytes(8));commands=alloc(32);cpu.mem_write(commands,bytes(32));sp=stack+0x1f0008;cpu.mem_write(sp,struct.pack('<Q',stop))
 for off,v in[(0x28,readers[4]),(0x30,res),(0x38,commands)]:cpu.mem_write(sp+off,struct.pack('<Q',v))
 for r,v in[(UC_X86_REG_RSP,sp),(UC_X86_REG_RCX,readers[0]),(UC_X86_REG_RDX,readers[1]),(UC_X86_REG_R8,readers[2]),(UC_X86_REG_R9,readers[3])]:cpu.reg_write(r,v)
 state['lifecycle_active']=True;state['active']=True;state['despawned']=[]
 try:cpu.emu_start(0x140755690,stop,count=30000000)
 finally:state['lifecycle_active']=False;state['active']=False
 assert cpu.reg_read(UC_X86_REG_RIP)==stop,'whole gate lifecycle owner did not return'
 return dict(rows=map_view(gates),despawned=state['despawned'],ticks=bytes(cpu.mem_read(ticks,8)).hex())

def lifecycle(kind='changed',remove_kind='wall'):
 s=dict(positions=[[0.,0.,0.],[2.,0.,0.],[4.,0.,0.]],heights=[2.,2.,2.],us=[0.,.5,1.]);out=n.run_complete(s,s['positions'],s['us']);view=out['view'];gates=alloc(48)
 cpu.mem_write(gates,bytes(cpu.mem_read(view['meshlib']+0x68,48)))
 first=update(gates,[12345]);entry=first['rows'][0]['entry'];cpu.mem_write(entry+8,bytes(cpu.mem_read(view['out'],24)))
 before=map_view(gates)
 parameters={'walls_changed':[12345]}if kind=='changed'else{'create_roofs':[12345]}if kind=='create'else{'switch_types':[12345]}
 rebuild=update(gates,**parameters);missing=update(gates,[12346])
 # Refill from the actual constructor's still-owned source allocation so remove
 # is tested with live entity refs as well as clearing an already empty entry.
 cpu.mem_write(entry+8,bytes(cpu.mem_read(view['out'],24)))
 remove=update(gates,**({'walls_deleted':[12345]}if remove_kind=='wall'else{'delete_roofs':[12345]}))
 return dict(first=first,before=before,rebuild=rebuild,missing=missing,remove=remove)
if __name__=='__main__':
 import json,hashlib
 cases=[]
 for kind in['changed','create','switch']:
  for remove_kind in['wall','roof']:
   result=lifecycle(kind,remove_kind);ids=result['before'][0]['entities'];assert result['rebuild']['despawned']==ids and result['remove']['despawned']==ids
   assert result['rebuild']['rows'][0]['wall_id']==12345 and result['rebuild']['rows'][0]['entities']==[]
   assert [r['wall_id']for r in result['remove']['rows']]==[12346]
   cases.append(dict(rebuild=kind,remove=remove_kind,actual_return=True,result=result));print(kind,remove_kind,'match',len(ids),flush=True)
   files=['native_gate_lifecycle_research.py','native_gates_research.py']
   (n.ROOT/'evidence/building-gates/B-gate-lifecycle-original-verification.json').write_text(json.dumps(dict(status='whole original lifecycle contract; Rust/World integration pending',cases=cases,bindings={f:hashlib.sha256((n.ROOT/'evidence/building-gates'/f).read_bytes()).hexdigest()for f in files}),indent=2))
