"""Validate the seven-package source and a reproducible seven-member Git snapshot."""
from pathlib import Path
import hashlib,json,subprocess,re,shutil,tomllib
from datetime import datetime,timezone
ROOT=Path(__file__).resolve().parents[2];REPO=ROOT.parents[1]
OUT=ROOT/'evidence/foundation-completion-2026-10-13';WS=ROOT/'reconstruction/shared-foundation'
SNAP=REPO/'tools/reverse-runtime/commit-seven-snapshot';SW=SNAP/'reconstruction/shared-foundation'
LIBS=('asset','asset-pipe','paths','lang-lib','theme-defs','tabloid','utils')
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def source_hashes():
    return {str(p.relative_to(ROOT)).replace('\\','/'):sha(p) for lib in LIBS
            for p in sorted((WS/'crates'/lib).rglob('*')) if p.is_file() and p.suffix in {'.rs','.toml'} and 'target' not in p.parts}
SW.mkdir(parents=True,exist_ok=True)
manifest=(WS/'Cargo.toml').read_text(encoding='utf-8')
manifest=re.sub(r'members\s*=\s*\[.*?\]','members = [\n'+''.join('    "crates/'+lib+'",\n'for lib in LIBS)+']',manifest,count=1,flags=re.S)
manifest=manifest.split('[patch.crates-io]')[0].rstrip()+'\n'
(SW/'Cargo.toml').write_text(manifest,encoding='utf-8')
shutil.copy2(WS/'Cargo.lock',SW/'Cargo.lock')
before=source_hashes()
for lib in LIBS:shutil.copytree(WS/'crates'/lib,SW/'crates'/lib,dirs_exist_ok=True,ignore=shutil.ignore_patterns('target','__pycache__','*.exe','*.pdb'))
fixture_files=[]
for folder in ('shared-foundation-2026-10-13','shared-foundation-review-2026-10-13','foundation-completion-2026-10-13'):
    for p in (ROOT/'evidence'/folder).rglob('*'):
        if not p.is_file() or p.suffix not in {'.rs','.json'}:continue
        relative=p.relative_to(ROOT);target=SNAP/relative;target.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(p,target)
        fixture_files.append({'path':str(relative).replace('\\','/'),'sha256':sha(p)})
metadata=subprocess.run(['cargo','metadata','--offline','--format-version','1'],cwd=SW,capture_output=True,text=True,encoding='utf-8')
assert metadata.returncode==0,metadata.stderr
(SNAP/'metadata.json').write_text(metadata.stdout,encoding='utf-8')
original=tomllib.loads((ROOT/'tracking/original-Cargo.lock').read_text(encoding='utf-8'))['package']
locked=tomllib.loads((SW/'Cargo.lock').read_text(encoding='utf-8'))['package']
def identity(p):return (p['name'],p['version'],p.get('source'),p.get('checksum'))
original_identities={identity(p)for p in original}
external=[p for p in locked if p.get('source')]
assert all(identity(p)in original_identities for p in external),'Dependency identity mismatch'
checks=[]
for name,args in [('test',['test','--locked','--offline']),('clippy',['clippy','--locked','--offline','--all-targets','--','-D','warnings'])]:
    command=['cargo']+args[:3]+['--target-dir',str(REPO/'tools/reverse-runtime/completion-check-target')]+args[3:]
    result=subprocess.run(command,cwd=SW,capture_output=True,text=True,encoding='utf-8',errors='replace')
    log=OUT/('integration-'+name+'.log');log.write_text(result.stdout+result.stderr,encoding='utf-8')
    checks.append({'name':name,'command':command,'cwd':str(SW),'exit_code':result.returncode,'log':str(log.relative_to(ROOT)).replace('\\','/'),'sha256':sha(log),
                   'passed_tests':sum(int(n)for n in re.findall(r'test result: ok\. (\d+) passed',result.stdout))})
    print(json.dumps(checks[-1]),flush=True)
    if result.returncode:break
after=source_hashes()
doc={'utc':datetime.now(timezone.utc).isoformat(),'scope':list(LIBS),'checks':checks,'source_hashes':after,
     'source_unchanged_during_checks':before==after,'fixtures':fixture_files,
     'snapshot_manifest_sha256':sha(SW/'Cargo.toml'),'snapshot_lock_sha256':sha(SW/'Cargo.lock'),
     'live_workspace_manifest_sha256':sha(WS/'Cargo.toml'),'live_workspace_lock_sha256':sha(WS/'Cargo.lock'),
     'dependency_identity':{'original_lock_sha256':sha(ROOT/'tracking/original-Cargo.lock'),'external_packages':len(external),'all_match_original':True},
     'passed':len(checks)==2 and all(c['exit_code']==0 for c in checks) and before==after,
     'full_original_feature_accepted':False,'publication_policy':'Seven-member snapshot only; live shared workspace and other chat code untouched.'}
(OUT/'integration-validation.json').write_text(json.dumps(doc,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
assert doc['passed'],'Integration failed or source changed during checks; inspect report'
print(json.dumps({'passed':True,'tests':checks[0]['passed_tests'],'external_dependencies':len(external)}),flush=True)
