from pathlib import Path
import json,random,struct
from native_focus_world import Native,EV

eid=lambda i:(1<<32)|i
fixtures={'bfs':[],'visibility':[],'event_frames':[]}
# Execute all 2048 visibility outcomes through the full original function.
for flag in [False,True]:
 rows=[{'id':eid(0),'state':[77,0,88]},{'id':eid(1),'state':[99,0,66],'parent':eid(0),'fpv':flag}];n=Native(rows);args=[n.query('nested',[1,2]),n.query('all',[0]),0x1b0000]
 for mode in range(4):
  n.wq(0x1b0000,0x1c0000);n.w(0x1c0009,bytes([mode]))
  for keep in range(256):
   n.w(n.states[eid(0)]+1,bytes([keep]));n.w(n.states[eid(1)],b'\x63');n.run(0x14233ce00,args);value=n.u.mem_read(n.states[eid(1)],1)[0];fixtures['visibility'].append([flag,mode,keep,value])
# Full native BFS, including cycles, diamonds, missing ancestors and 0xff keep.
prng=random.Random(20261020)
for case in range(32):
 rows=[]
 for i in range(9):
  row={'id':eid(i),'state':[i+20,prng.choice([0,0,1,2,255]),i+80]}
  if i>1:row.update(parent=eid(prng.randrange(11)),fpv=bool(i%2))
  rows.append(row)
 n=Native(rows);n.run(0x14233dcb0,[n.query('nested',[1,0]),n.query('roots',[0])]);result=n.result();fixtures['bfs'].append({'input':rows,'output':[result[str(eid(i))] for i in range(9)]})
# Native Local<HashSet<Entity>> survives between updates; only exact1 on entry emits.
rows=[{'id':eid(i),'state':[i,0,i+40]} for i in range(6)];n=Native(rows);q=n.query('all',[0]);frames=[[0,1,2,255,0,1],[1,1,1,1,0,1],[1,1,1,1,0,1],[0,0,0,0,0,0],[1,1,1,1,1,1],[2,2,2,2,2,2],[1,1,1,1,1,1],[0,1,0,1,0,1],[1,1,1,1,1,1]]
for keeps in frames:
 for i,k in enumerate(keeps):n.w(n.states[eid(i)]+1,bytes([k]))
 n.wq(n.eventdata+0x30,0);n.run(0x14233a170,[q,n.eventarg,n.local]);count=n.q(n.eventdata+0x30);events=[n.q(n.eventbuf+16*i+8)&0xffffffff for i in range(count)]
 n.wq(n.eventdata+0x30,0);n.run(0x14233d680,[q,n.eventarg]);notify=n.q(n.eventdata+0x30);fixtures['event_frames'].append({'keep':keeps,'begin':events,'notify':notify})
(EV/'full-native-focus-fixtures.json').write_text(json.dumps(fixtures,indent=2));print('native full-body fixtures: visibility',len(fixtures['visibility']),'BFS',len(fixtures['bfs']),'persistent event frames',len(frames))
# Rust test fixture output is data from original machine execution, no mirrored algorithm.
r=Path('D:/game/tiny-glade/crates/systems/toolbar/src/ui_interaction/focus_native_fixtures.rs');s='pub const VISIBILITY: &[(bool,u8,u8,u8)] = &[\n'+''.join(f'({str(f).lower()},{m},{k},{v}),\n' for f,m,k,v in fixtures['visibility'])+'];\n';s+='pub const BFS: &[([([u8;3],i32,bool);9],[[u8;3];9])] = &[\n'
for c in fixtures['bfs']:
 rows=[]
 for x in c['input']:
  parent=int(x['parent']&0xffffffff) if 'parent' in x else -1;rows.append('('+str(x['state'])+','+str(parent)+','+str(x.get('fpv',False)).lower()+')')
 expected=[list(bytes.fromhex(b)) for b in c['output']];s+='(['+','.join(rows)+'],'+str(expected)+'),\n'
s+='];\n';s+='pub const EVENTS: &[([u8;6],&[usize],usize)] = &[\n'+''.join('('+str(x['keep'])+',&'+str(x['begin'])+','+str(x['notify'])+'),\n' for x in fixtures['event_frames'])+'];\n';r.write_text(s,encoding='utf-8')
