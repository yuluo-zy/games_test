from pathlib import Path
import re,json
R=Path('D:/game/reverse-engineering/tiny-glade');O=R/'evidence/roof-tiles'
addresses={0x141a36700}|{int(f.stem,16) for f in O.glob('0x*.c')}
found=[]
for f in O.glob('module*.txt'):
 lines=f.read_text(encoding='utf-8').splitlines();mod=int(f.stem[6:])
 for i,l in enumerate(lines):
  if 'S_GPROC32 ' not in l and 'S_LPROC32 ' not in l:continue
  name=re.search(r'`([^`]+)`',l);a=re.search(r'addr = (\d+):(\d+), code size = (\d+)',lines[i+1])
  if name and a:
   sec,off,size=map(int,a.groups());va=0x140001000+off
   if sec==1 and va in addresses:found.append(dict(name=name[1],va=hex(va),code_size=size,end_exclusive=hex(va+size),module=mod,evidence_file=f.name,evidence_line=i+1))
(O/'procedures.json').write_text(json.dumps(sorted(found,key=lambda x:int(x['va'],16)),indent=2),encoding='utf-8');print(len(found))
