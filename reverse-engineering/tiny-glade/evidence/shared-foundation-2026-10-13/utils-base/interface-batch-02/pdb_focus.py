"""Only selected B modules; original inline range/type evidence, not global analysis."""
from pathlib import Path
import subprocess,json,hashlib,re
OUT=Path(__file__).parent
PDB=Path('D:/game/ljxsj_92385/Tiny Glade/tiny_glade.pdb')
results=[]
for module in [540,523,554,757]:
    command=['C:/Program Files/LLVM/bin/llvm-pdbutil.exe','dump','--symbols',f'--modi={module}',str(PDB)]
    p=subprocess.run(command,capture_output=True,text=True,encoding='utf8',errors='replace')
    path=OUT/f'B-module-{module}.txt';path.write_text(p.stdout,encoding='utf8')
    results.append({'module':module,'command':command,'exit_code':p.returncode,'stdout':path.name,'bytes':path.stat().st_size,'sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'stderr':p.stderr})
(OUT/'pdb-focus.json').write_text(json.dumps(results,indent=2)+'\n',encoding='utf8')
print([(r['module'],r['exit_code'],r['bytes']) for r in results])
