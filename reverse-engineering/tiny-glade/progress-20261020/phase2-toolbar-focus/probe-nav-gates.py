from native_focus_world import Native,EV
from unicorn.x86_const import *
import struct,json
out=[]
for current in range(4):
 for staged in range(4):
  for action in ['idle','cancel_released','confirm','left','left_and_right']:
   n=Native([]);q=n.query('all',[0,1,2]);q2=n.query('all',[0,1]);actions=0x1d0000;actionarg=0x1e0000;nav=0x1f0000;navarg=0x200000;layoutarg=0x210000;n.wq(actionarg,actions)
   for i in range(40):n.w(actions+8*i,struct.pack('<If',4,0))
   if action=='cancel_released':n.w(actions+8*18,struct.pack('<I',3))
   if action=='confirm':n.w(actions+8*19,struct.pack('<I',0))
   if action in ['left','left_and_right']:n.w(actions+8*20,struct.pack('<I',0))
   if action=='left_and_right':n.w(actions+8*21,struct.pack('<I',0))
   n.w(nav,struct.pack('<IIQIIQ',staged,0,0x100000009,current,0,0x100000007));n.wq(navarg,nav);n.wq(navarg+16,0x220000);n.w(navarg+28,struct.pack('<I',42));n.wq(layoutarg,0x230000)
   entrysp=0x2ffff8;n.wq(entrysp,0x100000);n.wq(entrysp+0x28,navarg);n.wq(entrysp+0x30,n.eventarg)
   for reg,val in [(UC_X86_REG_RSP,entrysp),(UC_X86_REG_RCX,q),(UC_X86_REG_RDX,q2),(UC_X86_REG_R8,actionarg),(UC_X86_REG_R9,layoutarg)]:n.u.reg_write(reg,val)
   n.u.emu_start(0x14233eed0,0x100000,count=500000)
   if n.u.reg_read(UC_X86_REG_RIP)!=0x100000:raise RuntimeError('bound')
   b=bytes(n.u.mem_read(nav,32));out.append({'current':current,'staged':staged,'action':action,'after_staged':struct.unpack_from('<I',b,0)[0],'after_current':struct.unpack_from('<I',b,16)[0],'data':b.hex(),'events':n.q(n.eventdata+48)})
(EV/'native-nav-empty-query-gates.json').write_text(json.dumps(out,indent=2));print('executed complete original nav entry with empty genuine query protocol',len(out),'cases')
for x in out:
 if x['current']==2 or x['staged']==1 and x['current']==0:print({k:v for k,v in x.items() if k!='data'})
