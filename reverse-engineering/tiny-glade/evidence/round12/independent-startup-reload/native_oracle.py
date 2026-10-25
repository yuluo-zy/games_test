from pathlib import Path
import json,struct,hashlib,subprocess,re
R=Path('D:/game');O=Path(__file__).resolve().parent;X=R/'ljxsj_92385/Tiny Glade/tiny-glade.rne';data=X.read_bytes()
assert hashlib.sha256(data).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=struct.unpack_from('<I',data,60)[0];base=struct.unpack_from('<Q',data,pe+48)[0];opt=struct.unpack_from('<H',data,pe+20)[0]
secs=[struct.unpack_from('<IIII',data,pe+24+opt+i*40+8) for i in range(struct.unpack_from('<H',data,pe+6)[0])]
def read(a,n):
 for _,start,size,raw in secs:
  if start<=a-base and a-base+n<=start+size:return data[raw+a-base-start:raw+a-base-start+n]
 raise ValueError(hex(a))
procs={}
for line in (R/'reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv').read_text(encoding='utf8').splitlines():
 f=line.split('\t');procs.setdefault(f[0],[]).append({'size':int(f[1]),'module':int(f[2]),'symbol':f[-1]})
rows=[]
for a,n in [(0x140941110,134),(0x140b66480,610),(0x140b66830,396),(0x140b3f550,951),(0x140a04980,421),
 (0x140b3c930,494),(0x140a7a5b0,1527),(0x1408b3510,386),(0x1408397a0,323),(0x140b1b1b0,1707),(0x140862270,6013)]:
 path=O/f'{a:x}.asm'
 if not path.exists():path.write_bytes(subprocess.run(['C:/Program Files/LLVM/bin/llvm-objdump.exe','-d',f'--start-address={a:#x}',f'--stop-address={a+n:#x}',str(X)],check=True,stdout=subprocess.PIPE).stdout)
 s=path.read_text();rows.append({'va':hex(a),'size':n,'sha256':hashlib.sha256(read(a,n)).hexdigest(),'pdb':procs.get(hex(a),[]),
  'calls':[{'va':c,'pdb':procs.get(c,[])} for c in dict.fromkeys(re.findall(r'callq\s+(0x[0-9a-f]+)',s))]})
(O/'native-evidence.json').write_text(json.dumps({'original_sha256':hashlib.sha256(data).hexdigest(),'ranges':rows},indent=2))
assets=[]
for theme in ['flowery','summer','winter']:
 root=R/'ljxsj_92385/Tiny Glade/assets/glade'/theme;plants=json.loads((root/'plants.json').read_text());counts=[]
 for plant in plants:
  p=root/plant['placements'];raw=p.read_bytes();counts.append({'path':str(p),'sha256':hashlib.sha256(raw).hexdigest(),'count':len(json.loads(raw))})
 assets.append({'theme':theme,'batches':len(plants),'count':sum(x['count'] for x in counts),'placements':counts})
(O/'nonempty-theme-oracle.json').write_text(json.dumps(assets,indent=2))
print('original bounded functions',len(rows));print(json.dumps(assets,indent=2))
root=R/'ljxsj_92385/Tiny Glade/assets/glade/autumn';counts=[]
for g in json.loads((root/'trees.json').read_bytes()):
 if g['prefab']=='clearing/tree/bush':continue
 p=root/g['placements'];raw=p.read_bytes();placements=json.loads(raw)
 counts.append({'prefab':g['prefab'],'path':str(p),'sha256':hashlib.sha256(raw).hexdigest(),'accepted':sum(abs(v['position'][0])<65 and abs(v['position'][2])<65 for v in placements)})
(O/'intro-legacy-asset-oracle.json').write_text(json.dumps({'intro_sha256':hashlib.sha256((R/'ljxsj_92385/Tiny Glade/assets/starting-builds/full/intro/history.json').read_bytes()).hexdigest(),'version':16,'accepted':sum(x['accepted'] for x in counts),'groups':counts},indent=2))
