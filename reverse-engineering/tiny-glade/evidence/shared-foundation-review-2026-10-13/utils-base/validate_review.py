"""Record current Cargo artifacts and source hashes; do not rewrite old proofs."""
import hashlib,json,re,subprocess
from pathlib import Path
OUT=Path(__file__).parent;ROOT=OUT.parents[2];WS=ROOT/'reconstruction/shared-foundation'
def digest(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def run(name,command,cwd=WS):
    p=subprocess.run(command,cwd=cwd,capture_output=True,text=True,encoding='utf-8',errors='replace')
    (OUT/(name+'.log')).write_text(p.stdout+p.stderr,encoding='utf-8')
    print(name,p.returncode,(p.stdout+p.stderr)[-800:])
    return {'command':command,'exit_code':p.returncode,'log':name+'.log'},p.stdout
commands=[]
for name,args in [('utils-test',['cargo','test','-p','utils','--locked','--offline']),
                  ('utils-clippy',['cargo','clippy','-p','utils','--locked','--offline','--all-targets','--','-D','warnings'])]:
    result,_=run(name,args);commands.append(result)
result,stdout=run('selected-artifacts',['cargo','build','-p','utils','--locked','--offline','--all-targets','--message-format=json']);commands.append(result)
artifacts={}
for line in stdout.splitlines():
    try:j=json.loads(line)
    except json.JSONDecodeError:continue
    if j.get('reason')!='compiler-artifact':continue
    name=j['target']['name']
    if name not in ['utils','serde','serde_json']:continue
    for file in j['filenames']:
        if file.endswith('.rlib'):
            assert name not in artifacts or artifacts[name]==file
            artifacts[name]=file
if result['exit_code']==0 and set(artifacts)=={'utils','serde','serde_json'}:
    deps=WS/'target/debug/deps'
    for name,driver in [('selector-allocation','selector-allocation-probe.rs'),('selector-policy','../../shared-foundation-2026-10-13/utils-base/validate_selective.rs')]:
        executable=OUT/(name+'.exe')
        command=['rustc','--edition','2021',str(OUT/driver),'-L',f'dependency={deps}','-o',str(executable)]
        for crate,file in artifacts.items():command+=['--extern',f'{crate}={file}']
        compiled,_=run(name+'-compile',command);commands.append(compiled)
        if compiled['exit_code']==0:
            executed,_=run(name,[str(executable)]);commands.append(executed)
test_log=(OUT/'utils-test.log').read_text(encoding='utf-8')
report={'baseline':'B','scope':'22 utils top-level source files excluding lib.rs; full crate also includes peer geometry/curve work',
        'commands':commands,'all_commands_passed':all(c['exit_code']==0 for c in commands),
        'source_sha256':{p.name:digest(p) for p in sorted((WS/'crates/utils/src').glob('*.rs')) if p.name!='lib.rs'},
        'workspace_manifest_sha256':digest(WS/'Cargo.toml'),'workspace_lock_sha256':digest(WS/'Cargo.lock'),
        'artifacts':{k:{'path':v,'sha256':digest(Path(v))} for k,v in artifacts.items()},
        'full_crate_failures':re.findall(r'^test (.*) \.\.\. FAILED$',test_log,re.M),
        'whole_original_game_verified':False,'new_original_differential_proofs':'Only bounded RDP boundary probe. Retained original fixture outputs rerun against current source; no original consumers/entire runtime validation.'}
(OUT/'validation.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8')
