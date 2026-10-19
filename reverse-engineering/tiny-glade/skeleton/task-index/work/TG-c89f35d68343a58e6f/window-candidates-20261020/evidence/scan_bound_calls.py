from pathlib import Path
import struct,csv,bisect,json,hashlib
pe=Path('/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
root=Path('/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade')
layout=json.loads((root/'skeleton/_pdb/B/input.json').read_text())
text=next(s for s in layout['sections'] if s['name']=='.text');start=0x140000000+text['rva'];data=pe[text['raw_pointer']:text['raw_pointer']+text['virtual_size']]
rows=[]
for r in csv.reader((root/'evidence/country-core-audit/data/procs.tsv').open(),delimiter='\t'):
 try:rows.append((int(r[0],16),int(r[1]),r[4]))
 except ValueError:pass
rows.sort();starts=[r[0] for r in rows]
targets={0x140ee6ee0:'candidate-resource-insert',0x14078ca00:'shared-grow80',0x1412baca0:'candidate-equality',0x140ed3da0:'candidate-drain-drop'}
results=[];idx=0
while True:
 idx=data.find(b'\xe8',idx)
 if idx<0 or idx+5>len(data):break
 va=start+idx;dest=va+5+struct.unpack_from('<i',data,idx+1)[0]
 if dest in targets:
  i=bisect.bisect_right(starts,va)-1;r=rows[i]
  results.append({'callsite':hex(va),'target':hex(dest),'target_label':targets[dest],'pdb_physical':{'va':hex(r[0]),'length':r[1],'name':r[2]} if r[0]<=va<r[0]+r[1] else None,'note':'E8 byte candidate; confirm instruction boundary against complete function before treating as call'})
 idx+=1
out=Path(__file__).parent/'bounded-call-target-candidates.json';out.write_text(json.dumps({'binary_sha256':hashlib.sha256(pe).hexdigest(),'targets':{hex(k):v for k,v in targets.items()},'results':results},indent=2)+'\n')
for x in results:
 p=x['pdb_physical'];print(x['callsite'],x['target_label'],None if not p else (p['va'],p['length'],p['name'][:180]))
