"""Scan original critter compilands. No imports, database writes, or analysis."""
from pathlib import Path
import subprocess,re,json,sys
ROOT=Path(__file__).resolve().parents[2];GAME=ROOT.parents[2];OUT=ROOT/'evidence/parallel-critters'
LLVM='/opt/homebrew/Cellar/llvm/22.1.2/bin/llvm-pdbutil'
for v in sys.argv[1:] or ['B']:
 inv=json.loads((OUT/f'{v}-inventory.json').read_text())
 actual={'A':GAME/'Tiny Glade/tiny_glade.pdb','B':GAME/'ljxsj_92385/Tiny Glade/tiny_glade.pdb','C':GAME/'tiny.glade/XJ07036/Tiny Glade v1.16.0/tiny_glade.pdb'}[v]
 mp=OUT/f'{v}-modules.txt'
 modules=mp.read_text() if mp.exists() else subprocess.run([LLVM,'dump','--modules',str(actual)],text=True,capture_output=True,check=True).stdout
 if not mp.exists():mp.write_text(modules)
 mods=[int(m[1]) for m in re.finditer(r'Mod (\d+) \| `system_critters-',modules)]
 report={'baseline':v,'identities':inv['identities'],'definition_owned_procedures':[],'compilands':mods,'limitations':['Only system_critters compilands scanned; external inlined game functions need caller evidence','PDB type void() is not original Rust signature','Dependency generic wrappers and derive registration excluded from business recovery']}
 for mod in mods:
  cmd=[LLVM,'dump',f'--modi={mod}','--symbols','--files','--il',str(actual)]
  old=OUT/f'{v}-module-{mod}.txt'
  text=old.read_text() if old.exists() else subprocess.run(cmd,capture_output=True,text=True,check=True).stdout
  lines=text.splitlines();kept=[]
  for i,l in enumerate(lines):
   if ('S_GPROC32' in l or 'S_LPROC32' in l) and '`system_critters::' in l:
    m=re.search(r'`([^`]+)`',l);addr=re.search(r'addr = (\d+):(\d+), code size = (\d+)',lines[i+1]);sn,off,size=map(int,addr.groups())
    row={'name':m[1],'module':mod,'section':sn,'section_offset':off,'code_size':size,'va':hex(0x140001000+off) if sn==1 else None,'pdb_type_record':lines[i+2].strip()};report['definition_owned_procedures'].append(row)
    end=re.search(r'end = (\d+)',lines[i+1]);boundary=int(end[1]) if end else 0;j=i
    while j<len(lines):
     kept.append(lines[j]);j+=1
     m2=re.match(r'\s*(\d+) \|',lines[j] if j<len(lines) else '')
     if m2 and int(m2[1])>boundary:break
  op=OUT/f'{v}-owned-module-{mod}.txt'
  if kept and not op.exists():op.write_text('\n'.join(kept)+'\n')
  print(json.dumps({'baseline':v,'mod':mod,'procedures_so_far':len(report['definition_owned_procedures'])}),flush=True)
 op=OUT/f'{v}-procedures.json'
 if op.exists():raise RuntimeError('Refuse overwrite existing inventory')
 op.write_text(json.dumps(report,indent=2)+'\n')
