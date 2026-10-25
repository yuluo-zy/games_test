from pathlib import Path
import subprocess,hashlib,json,tomllib

ROOT=Path('D:/game/reverse-engineering/tiny-glade')
OUT=Path(__file__).parent
WORKSPACE=ROOT/'reconstruction/shared-foundation'
manifest=str(WORKSPACE/'Cargo.toml')
commands=[
 ['cargo','check','--manifest-path',manifest,'-p','paths','-p','theme-defs','--locked','--offline'],
 ['cargo','test','--manifest-path',manifest,'-p','paths','-p','theme-defs','--locked','--offline'],
 ['cargo','clippy','--manifest-path',manifest,'-p','paths','-p','theme-defs','--all-targets','--locked','--offline','--','-D','warnings'],
 ['cargo','fmt','--manifest-path',manifest,'-p','paths','-p','theme-defs','--','--check'],
]
records=[]
for i,cmd in enumerate(commands):
 p=subprocess.run(cmd,cwd=ROOT,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,encoding='utf-8',errors='replace')
 log=OUT/f'validation-{i}.log';log.write_text(p.stdout,encoding='utf-8')
 records.append({'command':cmd,'exit_code':p.returncode,'log':str(log),'stdout_sha256':hashlib.sha256(p.stdout.encode()).hexdigest()})
 if p.returncode:break
files=[f for package in ['paths','theme-defs'] for f in (WORKSPACE/'crates'/package).rglob('*') if f.is_file()]
inputs={str(f.relative_to(ROOT)):hashlib.sha256(f.read_bytes()).hexdigest() for f in files}
original=tomllib.loads((ROOT/'tracking/original-Cargo.lock').read_text())['package']
current=tomllib.loads((WORKSPACE/'Cargo.lock').read_text())['package']
package_match=[]
for name in ['anyhow','known-folders','serde','serde_derive','thiserror','thiserror-impl']:
 p=next(p for p in current if p['name']==name)
 same=[q for q in original if q['name']==p['name'] and q['version']==p['version']]
 package_match.append({'name':name,'version':p['version'],'source_checksum_match':any(all(q.get(k)==p.get(k) for k in ['source','checksum']) for q in same)})
drift=[{'name':p['name'],'resolved_version':p['version'],'original_versions':[q['version'] for q in original if q['name']==p['name']]} for p in current if p.get('source') and not any(all(p.get(k)==q.get(k) for k in ['name','version','source','checksum']) for q in original)]
report={'scope':['paths','theme-defs'],'passed':len(records)==len(commands) and all(x['exit_code']==0 for x in records),'kind':'candidate_compile_tests_and_public_dependency_identity; NOT original-machine differential','commands':records,'source_inputs':inputs,'direct_dependency_identity':package_match,'workspace_transitive_lock_drift':drift,'original_executed':False,'original_full_module_equivalence_verified':False}
(OUT/'validation.json').write_text(json.dumps(report,indent=2),encoding='utf-8')
print(json.dumps({'passed':report['passed'],'commands':len(records),'direct_dependency_matches':sum(x['source_checksum_match'] for x in package_match),'transitive_drift':len(drift)}))
