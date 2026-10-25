from pathlib import Path
import re,json,subprocess
R=Path('D:/game/reverse-engineering/tiny-glade');O=R/'evidence/roof-tiles';targets=set()
for f in O.glob('*disassembly.txt'):
 for l in f.read_text().splitlines():
  m=re.search(r'CALL (0x)?([0-9a-f]+)$',l)
  if m:targets.add(int(m[2],16))
p=subprocess.Popen(['C:/Program Files/LLVM/bin/llvm-pdbutil.exe','dump','--publics','D:/game/ljxsj_92385/Tiny Glade/tiny_glade.pdb'],stdout=subprocess.PIPE,text=True,encoding='utf-8');active=None;records=[]
for line in p.stdout:
 if 'S_PUB32' in line:active=line.strip()
 elif active and 'addr =' in line:
  m=re.search(r'addr = (\d+):(\d+)',line)
  if m and int(m[1])==1:
   va=0x140001000+int(m[2]);
   if va in targets:records.append(dict(va=hex(va),symbol=active))
  active=None
p.wait();(O/'call-symbols.json').write_text(json.dumps(records,indent=2),encoding='utf-8');print(json.dumps(records,indent=2))
