from try_cases import *
def variant_line(c):
 return 'variant '+str(c['kind'])+' '+struct.pack('<5fBB',*c.get('top_bounds',(0.,0.,0.,0.,0.,2)),int(c.get('fence_fit',False))).hex()+' '+make_line(c)
def check(cases):
 lines=[variant_line(c)for c in cases];expected=[produce(**c)for c in cases];p=subprocess.run([str(ROOT/'reconstruction/target/release/wall-production-probe')],input='\n'.join(lines)+'\n',capture_output=True,text=True,check=True);a=p.stdout.splitlines();assert len(a)==len(cases);fs=[]
 for i,(w,line)in enumerate(zip(expected,a)):
  if line.startswith('error'):fs.append({'case':i,'input':lines[i],'actual':line,'expected_count':len(w)});continue
  n,packed=line.split(' ',1);g=[bytes.fromhex(s)for s in packed.split(',')if s];dif=[{'record':j,'offsets':[k for k,(x,y)in enumerate(zip(e,t))if x!=y],'expected':e.hex(),'actual':t.hex()}for j,(e,t)in enumerate(zip(w,g))if e!=t]
  if len(w)!=len(g)or dif:fs.append({'case':i,'input':lines[i],'expected_count':len(w),'actual_count':len(g),'first':dif[:2]})
 return fs,sum(map(len,expected))
if __name__=='__main__':
 r=json.loads((ROOT/'evidence/building-production/B-wall-production-verification.json').read_text());cs=[]
 for tag in [0,1,2]:
  for fit in [False,True]:
   for kind in [0,1,2,3]:cs.append({**r['input_cases'][1],'base_height':0.,'kind':kind,'top_bounds':(.7,.15,.4,.65,.9,tag),'fence_fit':fit})
 fs,count=check(cs);print(json.dumps({'cases':len(cs),'records':count,'failed':len(fs),'first':fs[:2]}));(ROOT/'evidence/building-production/B-variant-initial.json').write_text(json.dumps({'ok':not fs,'cases':len(cs),'failures':fs},indent=2)+'\n')
