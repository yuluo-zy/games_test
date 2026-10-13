import concurrent.futures,hashlib,json,re,subprocess
from pathlib import Path
OUT=Path(__file__).parent
PDB=Path('D:/game/ljxsj_92385/Tiny Glade/tiny_glade.pdb')
def dump(module):
    result=subprocess.run(['C:/Program Files/LLVM/bin/llvm-pdbutil.exe','dump','--symbols',f'--modi={module}',str(PDB)],capture_output=True,text=True,encoding='utf-8',errors='replace')
    assert result.returncode==0,result.stderr
    path=OUT/f'B-module-{module}.txt';path.write_text(result.stdout,encoding='utf-8')
    return {'module':module,'exit_code':result.returncode,'bytes':path.stat().st_size,'sha256':hashlib.sha256(path.read_bytes()).hexdigest()}
with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:
    rows=list(pool.map(dump,[3167,4091,4583,4590,4589,4605]))
(OUT/'asset-pdb-focus.json').write_text(json.dumps(rows,indent=2)+'\n',encoding='utf-8')
for row in rows:
    text=(OUT/f'B-module-{row["module"]}.txt').read_text(encoding='utf-8')
    print(row['module'],row['bytes'],[(int(m[1]),m[2]) for m in re.finditer(r'^\s*(\d+) \| (S_GPROC32[^\n]*|S_LPROC32[^\n]*)',text,re.M) if 'asset::' in m[2]])
