from pathlib import Path
import json,re,subprocess,struct
ROOT=Path('D:/game/ljxsj_92385/Tiny Glade'); OUT=Path('D:/game/reverse-engineering/tiny-glade/evidence/pdb-structure');OUT.mkdir(parents=True,exist_ok=True)
LLVM=Path('C:/Program Files/LLVM/bin');PDB=ROOT/'tiny_glade.pdb'
def stream(args):
    p=subprocess.Popen([str(LLVM/'llvm-pdbutil.exe'),'dump',*args,str(PDB)],stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True,encoding='utf-8',errors='replace')
    yield from p.stdout
    p.wait()
    if p.returncode:raise RuntimeError(p.stderr.read())
targets={'roof_create':35298976,'wall_brick_columns':33432128,'history_finish':11032960,'decorator_stairs':33601520,'window_aspect_ratio':9735200}
targets.update({'roof_generate_floor_bottom':0x1421a6d40-0x140001000,'roof_update_spatial':0x1421a7710-0x140001000})
matches=[]
for line in stream(['--section-contribs']):
    m=re.search(r'mod = (\d+), (\d+):(\d+), size = (\d+)',line)
    if not m:continue
    mod,sec,offset,size=map(int,m.groups())
    for label,addr in targets.items():
        if sec==1 and offset<=addr<offset+size:matches.append({'target':label,'module':mod,'section':sec,'offset':offset,'size':size,'raw':line.strip()})
(OUT/'contributions.json').write_text(json.dumps(matches,indent=2),encoding='utf-8')
print(json.dumps(matches,indent=2))
