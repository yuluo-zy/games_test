"""Original Gate CPU research. Game geometry and Mesh methods never bridged.
Bevy entity/queue identities are opaque dependency transport. Stop edges and
resource schemas are explicit, and no partial run is accepted as whole owner.
"""
import sys,struct
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
sys.path.insert(0,str(ROOT/'evidence/building-arches'))
import native_arches as arch_native
from native_arches import *
state={}
# These exact functions are confirmed Bevy Commands spawn wrappers in
# B-helpers/next-owner-metadata.json, not game numerical code.
def dependency_transport(uc,pc,size,user):
 if not state.get('active'):return
 if pc==0x140769760:
  state['constructor_calls'].append(observe_wall_constructor_call())
 elif pc==0x141214ed0:
  sp=uc.reg_read(UC_X86_REG_RSP);p=uc.reg_read(UC_X86_REG_RDX);normal=uc.reg_read(UC_X86_REG_R8);uv=uc.reg_read(UC_X86_REG_R9);color=struct.unpack('<Q',uc.mem_read(sp+0x28,8))[0];state['quads'].append(dict(caller=hex(struct.unpack('<Q',uc.mem_read(sp,8))[0]),positions=bytes(uc.mem_read(p,48)).hex(),normals=bytes(uc.mem_read(normal,48)).hex(),uvs=bytes(uc.mem_read(uv,32)).hex(),color=bytes(uc.mem_read(color,12)).hex()))
 elif pc==0x14092cc60:
  mesh=uc.reg_read(UC_X86_REG_RDX);state['mesh_inputs'].append(decode_mesh(mesh))
 elif pc in [0x140d146a0,0x14120f7c0]:
  target=uc.reg_read(UC_X86_REG_RCX);queue=state['queue'];entity=state['next_entity'];state['next_entity']+=1
  if pc==0x14120f7c0:
   payload=uc.reg_read(UC_X86_REG_R8);state['spawn_bundles'].append(bytes(uc.mem_read(payload,136)).hex())
  uc.mem_write(target,struct.pack('<5Q',0,queue,queue,0,entity));ret(target)
 # Confirmed std RandomState KEY TLS; game fastrand/seed code is untouched.
 elif pc==0x1400d9b20:ret(state['keys'])
 elif pc==0x140d4d740:ret(state['keys_option'])
 elif pc==0x140d64e00:uc.reg_write(UC_X86_REG_RDX,0x5678);ret(0x1234)
cpu.hook_add(UC_HOOK_CODE,dependency_transport)
def observe_wall_constructor_call():
 sp=cpu.reg_read(UC_X86_REG_RSP)
 word=lambda off:struct.unpack('<Q',cpu.mem_read(sp+off,8))[0]
 f=lambda off:struct.unpack('<f',cpu.mem_read(sp+off,4))[0]
 curve=cpu.reg_read(UC_X86_REG_RDX);cp,cn=struct.unpack('<QQ',cpu.mem_read(curve+8,16));up,un=struct.unpack('<QQ',cpu.mem_read(curve+32,16))
 fvec=lambda ptr,count: list(struct.unpack('<'+'f'*count,cpu.mem_read(ptr,count*4)))
 arch=cpu.reg_read(UC_X86_REG_R9);hn=word(0x28);second=word(0x30);sn=word(0x38);third=word(0x40);tn=word(0x48)
 hp=word(0x60);hc,hptr,hcount,hscale=struct.unpack('<QQQf',cpu.mem_read(hp,28));top=word(0x80)
 return dict(caller=hex(word(0)),points=[list(struct.unpack('<2f',cpu.mem_read(cp+i*8,8)))for i in range(cn)],us=fvec(up,un),length=struct.unpack('<f',cpu.mem_read(curve+48,4))[0],arch=fvec(arch,hn),second=fvec(second,sn),third=fvec(third,tn),kind=word(0x50)&255,seed=word(0x58),source=cpu.reg_read(UC_X86_REG_R8),holes=bytes(cpu.mem_read(hptr,hcount*20)).hex(),holes_length=hscale,closed=cpu.mem_read(hp+28,1)[0],perturb=f(0x68),base=f(0x78),top=bytes(cpu.mem_read(top,21)).hex(),width_flag=word(0x88)&255,width=f(0x90),thickness_flag=word(0x98)&255,thickness=f(0xa0),wall_float=f(0xa8),fence_fit=word(0xb0)&255)

def decode_mesh(mesh):
 raw=bytes(cpu.mem_read(mesh,144));words=struct.unpack('<18Q',raw)
 control,mask,growth,length=words[7:11];entries=[]
 assert mask<1000000 and length<=mask+1
 for bucket in range(mask+1):
  if cpu.mem_read(control+bucket,1)[0]>=128:continue
  entry=control-56*(bucket+1);record=bytes(cpu.mem_read(entry,56));cap,key,n=struct.unpack('<3Q',record[:24]);name=bytes(cpu.mem_read(key,n)).decode()
  tag,vcap,vp,vn=struct.unpack('<4Q',record[24:]);assert tag in[2,3]and vn<=vcap;entries.append(dict(name=name,tag=tag,element_width=tag*4,count=vn,data=bytes(cpu.mem_read(vp,vn*tag*4)).hex()))
 return dict(raw=raw.hex(),indices=bytes(cpu.mem_read(words[1],words[2]*4)).hex(),attributes=sorted(entries,key=lambda e:e['name']))

def invoke(address,registers):
 sp=stack+0x1f0008;cpu.mem_write(sp,struct.pack('<Q',stop));cpu.reg_write(UC_X86_REG_RSP,sp)
 for reg,value in registers:cpu.reg_write(reg,value)
 cpu.emu_start(address,stop,count=10000000);assert cpu.reg_read(UC_X86_REG_RIP)==stop,'source initialization did not return'

def initialize_libraries(meshlib,shaders,shader_handle):
 invoke(0x14092c920,[(UC_X86_REG_RCX,meshlib)])
 invoke(0x140936c60,[(UC_X86_REG_RCX,shaders)])
 # Standard name->opaque library handle transport is oracle input authority.
 # No shader code/driver initialization is claimed by this CPU environment.
 name=alloc(30);cpu.mem_write(name,b'solid_vertex_color_wooden_gate')
 invoke(0x14152b830,[(UC_X86_REG_RCX,shaders+0x88),(UC_X86_REG_RDX,name),(UC_X86_REG_R8,30),(UC_X86_REG_R9,shader_handle)])

def segment_view(segment):
 v=alloc(120);n=len(segment['positions'])
 for offset,rows,width in[(0,segment['heights'],4),(24,segment['positions'],12),(48,segment['us'],4)]:
  payload=struct.pack('<'+'f'*len(rows),*rows)if width==4 else b''.join(struct.pack('<3f',*p)for p in rows);p=alloc(len(payload));cpu.mem_write(p,payload);cpu.mem_write(v+offset,struct.pack('<3Q',len(rows),p,len(rows)))
 cpu.mem_write(v+72,struct.pack('<QQiii',0,n,0,n,n)+bytes(4));return v

def setup(segment,global_points,global_us,wall_float,wall_id,ordinal,pub29a,terrain_min,terrain_max):
 allocation[0]=heap;cpu.mem_write(data,bytes(0x100000));cpu.mem_write(stack,bytes(0x200000));state.clear()
 queue=data+0x80000;qp=alloc(0x100000);cpu.mem_write(queue,struct.pack('<3Q',0x100000,qp,0));keys=data+0x81000;cpu.mem_write(keys,struct.pack('<2Q',0x1234,0x5678));option=data+0x82000;cpu.mem_write(option,struct.pack('<3Q',1,0x1234,0x5678));state.update(active=True,queue=queue,keys=keys,keys_option=option,next_entity=0x100000000+1000,spawn_bundles=[],mesh_inputs=[],constructor_calls=[],quads=[])
 seg=segment_view(segment);cpu.mem_write(seg+104,struct.pack('<QQ',wall_id,ordinal));pub=data+0x1000;pp=alloc(len(global_points)*12);up=alloc(len(global_us)*4);cpu.mem_write(pp,b''.join(struct.pack('<3f',*p)for p in global_points));cpu.mem_write(up,struct.pack('<'+'f'*len(global_us),*global_us));cpu.mem_write(pub+0x88,struct.pack('<Q',len(global_points)));cpu.mem_write(pub+0xa0,struct.pack('<Q',len(global_us)));cpu.mem_write(pub+0x90,struct.pack('<QQ',pp,len(global_points)));cpu.mem_write(pub+0xa8,struct.pack('<QQ',up,len(global_us)));cpu.mem_write(pub+0x288,struct.pack('<f',wall_float));cpu.mem_write(pub+0x29a,bytes([pub29a]))
 terrain=data+0x3000;tp=alloc(512*512*2);cpu.mem_write(tp,bytes(512*512*2));cpu.mem_write(terrain,struct.pack('<3Q',512*512,tp,512*512));cpu.mem_write(terrain+0x50,struct.pack('<2f',terrain_min,terrain_max));cpu.mem_write(0x14362c308,struct.pack('<Q',data+0x3100));cpu.mem_write(data+0x3100,b'\1')
 meshlib=data+0x4000;shaders=data+0x5000;initialize_libraries(meshlib,shaders,42);commands=data+0x6000;writer=create_brick_writer(128);growth=data+0x6500;signifiers=data+0x7000;sp=stack+0x100008;cpu.mem_write(sp,struct.pack('<Q',stop))
 for offset,value in {0x28:shaders,0x30:terrain,0x38:commands,0x40:writer['writer'],0x48:growth,0x50:wall_id,0x58:bits(wall_float),0x60:ordinal,0x68:signifiers}.items():cpu.mem_write(sp+offset,struct.pack('<Q',value))
 for register,value in[(UC_X86_REG_RSP,sp),(UC_X86_REG_RCX,data+0x200),(UC_X86_REG_RDX,pub),(UC_X86_REG_R8,seg),(UC_X86_REG_R9,meshlib),(UC_X86_REG_MXCSR,0x1f80)]:cpu.reg_write(register,value)
 return dict(seg=seg,pub=pub,meshlib=meshlib,shaders=shaders,writer=writer,out=data+0x200)

def shape_before_mesh_registration(segment,points,us,scalar=0.,wall_id=12345,ordinal=1,pub29a=1,minimum=0.,maximum=0.):
 view=setup(segment,points,us,scalar,wall_id,ordinal,pub29a,minimum,maximum)
 try:run_to_edge(0x141210110,0x141212940,30000000)
 finally:state['active']=False
 bp=cpu.reg_read(UC_X86_REG_RBP);mesh=cpu.reg_read(UC_X86_REG_RDX);cap,p,n=struct.unpack('<3Q',cpu.mem_read(state['queue'],24));assert n<=cap
 return dict(mesh_raw=bytes(cpu.mem_read(mesh,128)).hex(),command_queue=bytes(cpu.mem_read(p,n)).hex(),spawn_bundles=state['spawn_bundles'],bricks=[r.hex()for r in brick_records(view['writer'])],bp=bp,mesh=mesh,view=view)
def run_complete(segment,points,us,scalar=0.,wall_id=12345,ordinal=1,pub29a=1,minimum=0.,maximum=0.):
 view=setup(segment,points,us,scalar,wall_id,ordinal,pub29a,minimum,maximum)
 try:cpu.emu_start(0x141210110,stop,count=30000000)
 finally:state['active']=False
 assert cpu.reg_read(UC_X86_REG_RIP)==stop,'whole gate constructor did not return'
 cap,p,n=struct.unpack('<3Q',cpu.mem_read(state['queue'],24));assert n<=cap
 gate_views=[]
 for raw in state['spawn_bundles']:
  b=bytes.fromhex(raw);cap,ptr,gate_n=struct.unpack('<3Q',b[80:104]);assert gate_n<=cap;gate_views.append(dict(transform=b[16:56].hex(),mesh_handle=struct.unpack('<Q',b[64:72])[0],shader_handle=struct.unpack('<Q',b[72:80])[0],top_points=bytes(cpu.mem_read(ptr,gate_n*12)).hex(),wall_id=struct.unpack('<Q',b[104:112])[0],endpoint_directions=b[112:128].hex(),seed=struct.unpack('<I',b[128:132])[0],public_29a=b[132]))
 return dict(constructor_calls=state['constructor_calls'],quads=state['quads'],gate_views=gate_views,mesh_inputs=state['mesh_inputs'],out_raw=bytes(cpu.mem_read(view['out'],24)).hex(),command_queue=bytes(cpu.mem_read(p,n)).hex(),spawn_bundles=state['spawn_bundles'],bricks=[r.hex()for r in brick_records(view['writer'])],view=view)

if __name__=='__main__':
 segment=dict(heights=[2.,2.,2.],positions=[[0.,0.,0.],[2.,0.,0.],[4.,0.,0.]],us=[0.,.5,1.])
 try:
  result=run_complete(segment,segment['positions'],segment['us']);print({k:len(v)if isinstance(v,(list,str))else v for k,v in result.items()if k not in['view']});import json;(ROOT/'evidence/building-gates/B-full-gate-mesh-research.json').write_text(json.dumps({k:v for k,v in result.items()if k!='view'},indent=2))
 except Exception as e:print('FAIL',repr(e),'RIP',hex(cpu.reg_read(UC_X86_REG_RIP)),last[-8:]);raise
