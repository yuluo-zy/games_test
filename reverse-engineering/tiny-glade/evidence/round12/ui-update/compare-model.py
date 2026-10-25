import importlib.util,pathlib,json,collections
p=pathlib.Path('reverse-engineering/tiny-glade/evidence/round12/ui-update/native-state-fixtures.json');cs=json.loads(p.read_text())
def model(x):
 tag=x['tag'];phase=x['phase'];hit=x['hit'];old=11;widget=11;arc=0x4000;ss=[];look=[]
 def hover():
  nonlocal tag,phase,arc,widget
  if hit is None:tag=0;phase=0;widget=0;arc=0;return
  if tag==0 or widget!=hit:ss.append((1 if hit==11 else 101))
  if not(tag==2 and hit==widget):arc=0x4000 if hit==11 else 0x4200;look.append(hit)
  tag=1;phase=1;widget=hit
 if tag!=2 and not(tag==1 and phase==0):hover()
 forced=x.get('escape',0) or x.get('shutdown',0) or (x.get('focus') and x['focus'][-1]==0) or x['action']==3
 if tag==2:
  if phase in (3,4):phase=int(hit==widget) if forced else 4
  elif phase!=2:hover()
 elif tag==1 and not forced and x['action'] in (0,2):tag=2;phase=3
 if tag==2 and phase in (0,1,3):ss.append({0:3,1:4,3:2}[phase]+(100 if arc==0x4200 else 0))
 return tag,phase,widget,arc,look,ss
fail=[]
for c in cs:
 m=model(c['inputs']);r=c['result'];n=(r['tag'],r['phase'],r['widget'],r['arc'],r['lookups'],[int.from_bytes(bytes.fromhex(s)[:4],'little')for s in r['sounds']])
 if r["tag"]==0:n=(0,0,0,0,n[4],n[5]);m=(m[0],0,0,0,m[4],m[5])
 if n!=m:fail.append((c['inputs'],m,n))
print(len(fail),'mismatch');print(fail[:12])
