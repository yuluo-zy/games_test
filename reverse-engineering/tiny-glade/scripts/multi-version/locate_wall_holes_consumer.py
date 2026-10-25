"""Bounded complete-PDB reader provenance, not dependency body recovery."""
import json,re,subprocess
from pathlib import Path
from verification_provenance import ROOT
rows=[]
for v,b in json.loads((ROOT/'tracking/baselines.json').read_text())['baselines'].items():
 root=Path(b['binary']['path']).parent;pdb=root/'tiny_glade.pdb'
 p=subprocess.Popen(['/opt/homebrew/Cellar/llvm/22.1.2/bin/llvm-pdbutil','dump','--publics',str(pdb)],stdout=subprocess.PIPE,text=True)
 name=None;selected=[]
 for line in p.stdout:
  if 'S_PUB32' in line:
   m=re.search(r'`([^`]+)`',line);name=m[1] if m else None
  elif name and 'addr =' in line:
   m=re.search(r'addr = (\d+):(\d+)',line)
   if m and 'RecalculateWallHoles' in name:
    sec=b['pe']['sections'][int(m[1])-1];va=b['pe']['image_base']+sec['rva']+int(m[2]);selected.append({'symbol':name,'va':hex(va),'dependency_body':name.startswith('_ZN4core'), 'actual_reader':('EventReader$LT$country_core..systems..wall..wall_state..wall_roof_hashmap..RecalculateWallHoles$GT$' in name or 'MessageReader$LT$country_core..systems..wall..wall_state..wall_roof_hashmap..RecalculateWallHoles$GT$' in name)})
   name=None
 assert p.wait()==0
 rows.append({'baseline':v,'binary_sha256':b['binary']['sha256'],'symbols':selected})
 print(v,len(selected),[(x['va'],x['symbol'][-200:]) for x in selected if x['actual_reader']])
(ROOT/'evidence/building-holes/three-version-reader-provenance.json').write_text(json.dumps(rows,indent=2)+'\n')
