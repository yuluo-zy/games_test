from pathlib import Path
import json,re,subprocess,hashlib
R=Path('D:/game');O=Path(__file__).resolve().parent
P=R/'reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv'
procs={}
for line in P.read_text(encoding='utf-8').splitlines():
 f=line.split('\t');procs.setdefault(f[0],[]).append(f)
f=procs['0x140817a40'][0];size=int(f[1]);va=int(f[0],16)
exe=R/'ljxsj_92385/Tiny Glade/tiny-glade.rne'
assert hashlib.sha256(exe.read_bytes()).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
asm=subprocess.run(['C:/Program Files/LLVM/bin/llvm-objdump.exe','-d',f'--start-address={va:#x}',f'--stop-address={va+size:#x}',str(exe)],check=True,stdout=subprocess.PIPE).stdout
(O/'restore-original.asm').write_bytes(asm)
calls=[]
for a in dict.fromkeys(re.findall(r'callq\s+(0x[0-9a-f]+)',asm.decode())):
 calls.append({'va':a,'names':[x[-1] for x in procs.get(a,[])]})
(O/'restore-callees.json').write_text(json.dumps({'size':size,'pdb_rows':procs[f[0]],'calls':calls},indent=2))
for c in calls: print(c['va'], str(c['names'])[:650])
p=O/'restore-readable.c';p.write_text(p.read_text().replace('\n\n','\n'))
