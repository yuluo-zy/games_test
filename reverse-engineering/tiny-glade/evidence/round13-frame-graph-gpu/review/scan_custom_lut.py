exec(open('reverse-engineering/tiny-glade/evidence/round13-frame-graph-gpu/review/verify_original.py').read().split('report=')[0])
procs=json.loads((root/'reverse-engineering/tiny-glade/evidence/render-skeleton/native-procedures.json').read_text())
syms=json.loads((root/'reverse-engineering/tiny-glade/evidence/render-architecture/public-symbols.json').read_text())
results=[]
for val in [0x5f06b298510fb768,0x054e14bacecb858d]:
 needle=struct.pack('<Q',val);pos=0
 while True:
  pos=b.find(needle,pos)
  if pos<0:break
  va=None
  for name,vr,vs,raw,rs in sections:
   if raw<=pos<raw+rs:va=0x140000000+vr+pos-raw;break
  if va:
   near=[q for q in procs if (int(q['va'],16) if isinstance(q['va'],str) else (q['va'] or 0))<=va<(int(q['va'],16) if isinstance(q['va'],str) else (q['va'] or 0))+q['code_size']]
   public=sorted([q for q in syms if (int(q['va'],16) if isinstance(q['va'],str) else (q['va'] or 0))<=va],key=lambda q:(int(q['va'],16) if isinstance(q['va'],str) else (q['va'] or 0)),reverse=True)[:1]
   results.append({'typeid':hex(val),'va':hex(va),'procedure':[{'va':q['va'],'size':q['code_size'],'name':q['name']} for q in near],'nearest_public':[{'va':q['va'],'name':q['demangled']} for q in public]})
  pos+=1
(out/'custom-lut-typeid-scan.json').write_text(json.dumps(results,indent=2));print(json.dumps(results,indent=2))


