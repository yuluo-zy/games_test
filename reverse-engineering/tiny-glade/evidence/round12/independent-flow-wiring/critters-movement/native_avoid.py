from pathlib import Path
exec(Path(__file__).with_name('native_move.py').read_text().split('cases=[]')[0])
def execute_avoid(case):
 u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(0x60000000,0x40000);u.mem_map(0x70000000,0x100000)
 rbp=0x70080000;transform=0x60001000;goal=0x60002000;loc=0x60004000;tick=0x60005000;wallptr=0x60010000
 result={'despawn':0};calls=[];water_index=0;path_index=[0]*4
 def sf(reg):return struct.unpack('<f',struct.pack('<I',u.reg_read(reg)&0xffffffff))[0]
 def floats(addr,n):return list(struct.unpack('<'+'f'*n,u.mem_read(addr,n*4)))
 def ret():
  sp=u.reg_read(UC_X86_REG_RSP);pc=struct.unpack('<Q',u.mem_read(sp,8))[0];u.reg_write(UC_X86_REG_RSP,sp+8);u.reg_write(UC_X86_REG_RIP,pc)
 def memory(uc,access,addr,size,value,user):
  page=addr&~4095
  if 0x140000000<=page<0x144000000:
   try:b=read(page,4096)
   except ValueError:return False
   uc.mem_map(page,4096);uc.mem_write(page,b);return True
  return False
 def code(uc,addr,size,user):
  nonlocal water_index
  if addr in [0x142318809,0x142317e4f]:result['status']='ok';uc.emu_stop();return
  if addr in [0x142318945,0x1428d9310]:result['status']='panic';uc.emu_stop();return
  if addr==0x140a14df0:
   view=uc.reg_read(UC_X86_REG_RCX)
   if view==rbp+0x50:kind='water';value=case['water'][min(water_index,len(case['water'])-1)];water_index+=1
   else:
    i=(view-rbp-0x10)//16;assert 0<=i<4;kind='path'+str(i);j=path_index[i];path_index[i]+=1;value=case['paths'][min(j,len(case['paths'])-1)][i]
   calls.append({'kind':kind,'position':[bits(sf(UC_X86_REG_XMM1)),bits(sf(UC_X86_REG_XMM2))]});uc.reg_write(UC_X86_REG_XMM0,bits(value));ret()
  elif addr==0x1409ea800:
   calls.append({'kind':'rock','position':[bits(sf(UC_X86_REG_XMM2)),bits(sf(UC_X86_REG_XMM3))]});uc.reg_write(UC_X86_REG_RAX,case['rock']);ret()
  elif addr==0x140aa5730:
   calls.append({'kind':'walls','position':[bits(sf(UC_X86_REG_XMM1)),bits(sf(UC_X86_REG_XMM2))]});uc.reg_write(UC_X86_REG_RAX,wallptr);uc.reg_write(UC_X86_REG_RDX,len(case['walls']));ret()
  elif addr==0x140a91ef0:
   i=int(uc.reg_read(UC_X86_REG_RDX));wall=case['walls'][i-1];calls.append({'kind':'public','wall':i});uc.reg_write(UC_X86_REG_RAX,0 if wall.get('missing',False) else 0x60030000+i*4);ret()
  elif addr==0x140b2a680:
   i=(uc.reg_read(UC_X86_REG_RCX)-0x60030000)//4-1;uc.reg_write(UC_X86_REG_XMM0,bits(case['walls'][i]['height']));ret()
  elif addr==0x1408ccc90:
   ray=uc.reg_read(UC_X86_REG_R8);origin=floats(ray,3);direction=floats(ray+12,3);radius=sf(UC_X86_REG_XMM3);calls.append({'kind':'sphere','origin':[bits(x) for x in origin],'direction':[bits(x) for x in direction],'radius':bits(radius)})
   dst=uc.reg_read(UC_X86_REG_RCX);uc.mem_write(dst,bytes(64));hit=case['sphere'];uc.mem_write(dst+0x30,struct.pack('<I',2 if hit is None else 0))
   if hit is not None:uc.mem_write(dst+0x18,struct.pack('<3f',*hit))
   ret()
  elif addr==0x14261f3e0:result['despawn']+=1;result['despawn_entity']=int(uc.reg_read(UC_X86_REG_RDX));ret()
 u.hook_add(UC_HOOK_MEM_UNMAPPED,memory);u.hook_add(UC_HOOK_CODE,code)
 u.mem_write(transform,struct.pack('<10f',*case['rotation'],*case['position'],*case['scale'])+bytes(8));u.mem_write(goal,struct.pack('<I3f',case['goal_tag'],*case['goal']));u.mem_write(loc,struct.pack('<II',case['loc'],0))
 for i,wall in enumerate(case['walls']):u.mem_write(wallptr+i*56,struct.pack('<4f',*wall['a'],*wall['b'])+bytes(24)+struct.pack('<Q',i+1)+bytes(8))
 for offset,value in [(0x120,tick),(0x150,loc),(0x98,0x9911),(0x80,0x60030000),(0xa0,0x60030000),(0xa8,0x60030000),(0x298,0x60030000)]:u.mem_write(rbp+offset,struct.pack('<Q',value))
 for offset,value in [(0x158,17),(0x15c,case['duck'])]:u.mem_write(rbp+offset,struct.pack('<I',value))
 for reg,value in [(UC_X86_REG_RBP,rbp),(UC_X86_REG_RSP,rbp-0x80),(UC_X86_REG_RBX,transform),(UC_X86_REG_RDI,goal),(UC_X86_REG_RSI,0x60006000 if case['sheep'] else 0),(UC_X86_REG_R9,tick),(UC_X86_REG_RCX,0x9911)]:u.reg_write(reg,value)
 for reg,values in [(UC_X86_REG_XMM6,[130,0,0,0]),(UC_X86_REG_XMM12,[1,0,0,0]),(UC_X86_REG_XMM13,[.1,0,0,0]),(UC_X86_REG_XMM14,[0,0,0,0]),(UC_X86_REG_XMM15,[.3,0,0,0])]:u.reg_write(reg,int.from_bytes(struct.pack('<4f',*values),'little'))
 try:u.emu_start(0x1423182cd,0,count=100000)
 except Exception as e:raise RuntimeError((case['name'],hex(u.reg_read(UC_X86_REG_RIP)),str(e)))
 assert 'status' in result,(case['name'],hex(u.reg_read(UC_X86_REG_RIP)))
 result['goal_tag']=struct.unpack('<I',u.mem_read(goal,4))[0];result['calls']=calls;return result
cases=[]
for loc in range(9):
 for species in ['other','duck','sheep','both']:
  c=base('avoid-loc%d-%s'%(loc,species));c.update(goal=[2,1,-3],goal_tag=1,paths=[[0,0,0,0]],loc=loc,duck=species in ['duck','both'],sheep=species in ['sheep','both']);cases.append(c)
for tag in [0,2,0xffffffff]:
 c=base('goal-tag-'+str(tag));c.update(goal=[2,1,-3],goal_tag=tag,paths=[[0,0,0,0]]);cases.append(c)
for goalpos in [[0,0,0],[math.nan,0,2],[9,1,2]]:
 c=base('goal-position-'+str(goalpos));c.update(position=[4,0,0] if goalpos[0]==9 else [0,0,0],goal=goalpos,goal_tag=1,paths=[[1,0,0,0],[0,0,0,0]],duck=True);cases.append(c)
for paths in [[[0,0,0,0],[0,0,.1,0]],[[math.nan,0,0,0],[0,0,0,0]],[[2,3,4,5],[0,0,0,0]]]:
 c=base('paths-'+str(paths));c.update(goal=[9,1,2],goal_tag=1,paths=paths,duck=True);cases.append(c)
for species in ['duck','sheep','both']:
 for distance in [None,.39999998,.4,.99999994,1,math.nan]:
  c=base('avoid-sphere-'+species+'-'+str(distance));c.update(position=[0,0,0],goal=[2,1,-3],goal_tag=1,paths=[[0,0,0,0]],duck=species in ['duck','both'],sheep=species in ['sheep','both'],sphere=None if distance is None else [distance,0,0]);cases.append(c)
for kind in ['intersect','parallel','low','missing','flying-missing']:
 c=base('avoid-wall-'+kind);c.update(goal=[2,1,0],goal_tag=1,paths=[[0,0,0,0]],loc=3 if kind=='flying-missing' else 2,walls=[{'a':[1.5,-1],'b':[1.5,1] if kind!='parallel' else [3.5,-1],'height':-1 if kind=='low' else 2,'missing':kind in ['missing','flying-missing']}]);cases.append(c)
rows=[{'input':c,'output':execute_avoid(c)} for c in cases]
def encode(v):
 if isinstance(v,float):return {'f32':bits(v)}
 if isinstance(v,list):return [encode(x) for x in v]
 if isinstance(v,dict):return {k:encode(x) for k,x in v.items()}
 return v
(here/'native-avoid-policy.json').write_text(json.dumps({'original_sha256':sha,'scope':'Original complete per-entity avoid CFG1423182cd onward, original segment geometry executes unchanged; injected water/path/mask/collision/public owner results explicitly recorded. Query/Events storage traversal remains outside harness.','cases':len(rows),'results':encode(rows)},indent=2),encoding='utf-8')
print('native avoid',len(rows),'return',sum(r['output']['status']=='ok' for r in rows),'panic',sum(r['output']['status']=='panic' for r in rows),'despawn',sum(r['output']['despawn'] for r in rows))
