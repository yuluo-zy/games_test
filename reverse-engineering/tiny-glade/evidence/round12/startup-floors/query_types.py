from pathlib import Path
import subprocess,json,re
base=Path(__file__).resolve().parent
args=['C:/Program Files/LLVM/bin/llvm-pdbutil.exe','dump','--types','D:/game/ljxsj_92385/Tiny Glade/tiny_glade.pdb']
p=subprocess.Popen(args,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True,encoding='utf8',errors='replace')
rows=[]; record=[]
def save(record):
 s=''.join(record)
 if any(x in s for x in ['country_core::resources::walls::Floor','system_wall_constructor::construct_floors::FloorStorage']): rows.append(s)
for line in p.stdout:
 if re.match(r'^  0x[0-9A-F]+ \|',line):
  save(record);record=[]
 record.append(line)
save(record)
p.wait()
(base/'pdb-floor-types.json').write_text(json.dumps({'exit':p.returncode,'matches':rows},indent=2),encoding='utf8')
print(json.dumps({'exit':p.returncode,'matches':rows},indent=2))
