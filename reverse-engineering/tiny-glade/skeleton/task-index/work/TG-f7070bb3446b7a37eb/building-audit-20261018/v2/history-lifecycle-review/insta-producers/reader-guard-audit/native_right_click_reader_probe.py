"""Original ui_convert EvRightClicked last-reader and entity guards only.
Stops at invalid exit or after all native validity guards, before component
address resolution/handle_rmb_autoclutter. No business leaf is bridged.
"""
from pathlib import Path
import json,struct,hashlib
ROOT=Path('/Users/yuluo/Desktop/game');HERE=Path(__file__).resolve().parent
s=(ROOT/'games_test/reverse-engineering/tiny-glade/evidence/shared-foundation-2026-10-13/utils-base/native_spline_rdp.py').read_text().split('def spline(')[0]
s=s.replace('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne',str(ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne'))
exec(compile(s,'shared-native-memory-harness','exec'),globals());OUT=HERE
WORLD,ROWS,QUERY,BITSET,EVENTS,FIRST,SECOND,CURSOR=[INPUT+i*0x1000 for i in range(8)]
obs={}
def spy(uc,a,size,d):
 if a==0x14099d75b:obs['selected_entity']=u.reg_read(UC_X86_REG_RAX)
 if a in[0x14099d7d0,0x14099d8ca]:obs['passed_entity_guards']=a==0x14099d7d0;uc.emu_stop()
u.hook_add(UC_HOOK_CODE,spy)
def run_case(first,second,cursor=0,flag='valid'):
 global obs
 reset();obs={'selected_entity':None};frame=STACK+0x10000;u.reg_write(UC_X86_REG_RBP,frame);u.reg_write(UC_X86_REG_RSI,WORLD);u.reg_write(UC_X86_REG_RDI,CURSOR)
 u.mem_write(INPUT,bytes(0x10000));u.mem_write(WORLD+8,struct.pack('<QQ',ROWS,4));u.mem_write(QUERY+0x28,struct.pack('<Q',BITSET));u.mem_write(QUERY+0x38,struct.pack('<Q',4));u.mem_write(BITSET,struct.pack('<Q',15))
 for i in range(4):u.mem_write(ROWS+i*20,struct.pack('<5I',1,i,0,0,0))
 if flag=='generation':u.mem_write(ROWS+20,struct.pack('<I',2))
 if flag=='d2s':u.mem_write(ROWS+20+4,struct.pack('<I',0xffffffff))
 if flag=='bit':u.mem_write(BITSET,struct.pack('<Q',13))
 if flag=='bounds':u.mem_write(QUERY+0x38,struct.pack('<Q',1))
 for ptr,items in[(FIRST,first),(SECOND,second)]:
  for i,v in enumerate(items):u.mem_write(ptr+i*16,struct.pack('<QQ',100+i,v))
 u.mem_write(EVENTS,struct.pack('<9Q',16,FIRST,len(first),0,16,SECOND,len(second),len(first),len(first)+len(second)));u.mem_write(CURSOR,struct.pack('<Q',cursor));u.mem_write(frame+0x188,struct.pack('<Q',EVENTS));u.mem_write(frame+0x2a8,struct.pack('<Q',QUERY))
 u.emu_start(0x14099d6a1,0x14099d8ca,count=1000)
 if 'passed_entity_guards'not in obs:obs['passed_entity_guards']=False
 obs['cursor_after']=struct.unpack('<Q',u.mem_read(CURSOR,8))[0]
 unread=(first+second)[cursor:];selected=unread[-1]if unread else None
 assert obs['selected_entity']==selected,(obs,selected)
 expected=selected is not None and selected>>32==1 and(selected&0xffffffff)<4 and not(flag!='valid'and(selected&0xffffffff)==1)
 assert obs['passed_entity_guards']==expected,(obs,expected)
 assert obs['cursor_after']==len(first)+len(second)
 return{'first':first,'second':second,'cursor_before':cursor,'guard':flag,**obs}
e0=(1<<32);e1=e0+1;e2=e0+2;invalid=e0+9
cases=[run_case(*v)for v in[([],[]),([e0],[]),([e0,e1],[]),([e0],[e2]),([e0,e1],[e2,e0]),([e0,invalid],[]),([invalid,e1],[]),([e0,e1],[],2),([e0,e1],[e2],2),([e0,e1],[e2],3)]]
cases +=[run_case([e0,e1],[],0,flag)for flag in['generation','d2s','bit','bounds']]
(HERE/'native-right-click-reader-fixtures.json').write_text(json.dumps({'binary_sha256':hashlib.sha256(raw).hexdigest(),'method':__doc__,'entry':'0x14099d6a1','cases':cases},indent=2)+'\n');print('cases',len(cases))
