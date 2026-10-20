from pathlib import Path
import json,re
R=Path('D:/game');O=R/'reverse-engineering/tiny-glade/evidence/round12/decorator-addr-schema';O.mkdir(exist_ok=True)
rows=[]
for line in (R/'reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv').read_text(encoding='utf8').splitlines():
 f=line.split('\t')
 if len(f)<5 or not ('DecoratorStorageAddrHistoryState' in f[-1] or 'DecoratorStorageHistoryDiff' in f[-1]):continue
 name=f[-1]
 rows.append({'va':f[0],'size':int(f[1]),'module':int(f[2]),'symbol':name,
  'serde':any(s in name for s in ['Serialize','Deserialize','serialize','deserialize'])})
(O/'pdb-procs.json').write_text(json.dumps(rows,indent=2))
print('rows',len(rows),'serde',sum(x['serde'] for x in rows))
for x in rows:
 if x['serde']:print(x['va'],x['size'],x['symbol'][:500])
# All source occurrences, including direct removed-field accesses and initializers.
uses=[]
for p in (R/'tiny-glade/crates').rglob('*.rs'):
 s=p.read_text(encoding='utf8');lines=s.splitlines()
 if 'DecoratorStorageAddrHistoryState' not in s:continue
 uses.append({'path':str(p.relative_to(R)),'type_lines':[i+1 for i,l in enumerate(lines) if 'DecoratorStorageAddrHistoryState' in l],
              'removed_lines':[{'line':i+1,'text':l.strip()} for i,l in enumerate(lines) if re.search(r'\.removed\b|\bremoved:',l)]})
(O/'source-consumers.json').write_text(json.dumps(uses,indent=2))
print('source files',len(uses))
