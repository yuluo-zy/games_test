from pathlib import Path
import json,subprocess,time,hashlib
OUT=Path(__file__).parent;ROOT=OUT.parents[3]/'tiny-glade'
commands=[
 ('utils-debug',['cargo','test','--locked','--offline','-p','utils','--all-targets']),
 ('utils-release',['cargo','test','--locked','--offline','-p','utils','--all-targets','--release']),
 ('utils-clippy',['cargo','clippy','--locked','--offline','-p','utils','--all-targets','--','-D','warnings']),
 ('country-consumer',['cargo','test','--locked','--offline','-p','country-core','--lib']),
]
results=[]
def inputs():
    paths=[ROOT/'Cargo.toml',ROOT/'Cargo.lock',ROOT/'crates/utils/Cargo.toml']
    paths+=list((ROOT/'crates/utils/src').rglob('*.rs'))
    paths+=list((ROOT/'crates/utils/tests').rglob('*.rs'))
    paths+=list((ROOT/'crates/utils/tests/fixtures').rglob('*.json'))
    return {str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}
for name,cmd in commands:
    start=time.time()
    before=inputs()
    with (OUT/(name+'.log')).open('w',encoding='utf8') as f:
        r=subprocess.run(cmd,cwd=ROOT,stdout=f,stderr=subprocess.STDOUT)
    after=inputs()
    results.append({'name':name,'command':cmd,'exit_code':r.returncode,'seconds':round(time.time()-start,2),'log':name+'.log','inputs':before,'changes_during_stage':{k:v for k,v in after.items() if v!=before.get(k)}})
    (OUT/'gate-results.json').write_text(json.dumps(results,indent=2),encoding='utf8')
    print(name,r.returncode,flush=True)
