from pathlib import Path
import re,json,hashlib,subprocess,struct
R=Path('D:/game');O=Path(__file__).resolve().parent
ctx={'__file__':str(R/'reverse-engineering/tiny-glade/evidence/round12/archival-owner/extract_diff_original.py')}
src=Path(ctx['__file__']).read_text();exec(src.split('out=[]')[0],ctx);read=ctx['read'];exe=ctx['exe']
procs={}
for line in (R/'reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv').read_text(encoding='utf8').splitlines():
 f=line.split('\t');procs.setdefault(f[0],[]).append(f[-1])
rows=[]
for a,n in [(0x141582200,798),(0x140965dd0,752),(0x1417c1490,248),(0x140899100,137),(0x14155ba30,6407),(0x140a1b5a0,337)]:
 p=O/f'{a:x}.asm'
 if not p.exists():p.write_bytes(subprocess.run(['C:/Program Files/LLVM/bin/llvm-objdump.exe','-d',f'--start-address={a:#x}',f'--stop-address={a+n:#x}',str(exe)],check=True,stdout=subprocess.PIPE).stdout)
 text=p.read_text();calls=[]
 for target in dict.fromkeys(re.findall(r'callq\s+(0x[0-9a-f]+)',text)):
  calls.append({'va':target,'names':procs.get(target,[])})
 rows.append({'va':hex(a),'size':n,'sha256':hashlib.sha256(read(a,n)).hexdigest(),'calls':calls})
(O/'native-schema.json').write_text(json.dumps({'original_sha256':hashlib.sha256(ctx['data']).hexdigest(),'ranges':rows},indent=2))
for row in rows:
 print(row['va'])
 for c in row['calls']:print(c['va'],str(c['names'])[:450])
