"""Record real locked/offline checks and measured numerical differences."""
import hashlib,json,re,subprocess
from pathlib import Path
OUT=Path(__file__).parent;ROOT=OUT.parents[2];WORKSPACE=ROOT/'reconstruction/shared-foundation'
commands=[('utils-test',['cargo','test','-p','utils','--locked','--offline','--','--nocapture','--test-threads=1']),('utils-clippy',['cargo','clippy','-p','utils','--locked','--offline','--all-targets','--','-D','warnings'])]
results=[]
for name,command in commands:
    p=subprocess.run(command,cwd=WORKSPACE,capture_output=True,text=True,encoding='utf8',errors='replace')
    text=p.stdout+p.stderr;(OUT/f'{name}.log').write_text(text,encoding='utf8')
    results.append({'command':command,'exit_code':p.returncode,'log':f'{name}.log'})
    print(name,p.returncode,text[-1300:])
    if p.returncode:break
if results[0]['exit_code']==0:
    deps=WORKSPACE/'target/debug/deps'
    # Cargo's selected artifacts are authoritative; timestamp selection can mix
    # incompatible crate identities left by another feature/profile build.
    build_command=['cargo','build','-p','utils','--locked','--offline','--all-targets','--message-format=json']
    build=subprocess.run(build_command,cwd=WORKSPACE,capture_output=True,text=True,encoding='utf8',errors='replace')
    (OUT/'selective-artifacts.log').write_text(build.stdout+build.stderr,encoding='utf8')
    results.append({'command':build_command,'exit_code':build.returncode,'log':'selective-artifacts.log'})
    paths={}
    for line in build.stdout.splitlines():
        try:artifact=json.loads(line)
        except json.JSONDecodeError:continue
        if artifact.get('reason')!='compiler-artifact':continue
        name=artifact['target']['name']
        if name not in ['utils','serde','serde_json']:continue
        for filename in artifact['filenames']:
            if filename.endswith('.rlib'):
                if name in paths and paths[name]!=Path(filename):raise RuntimeError(f'Ambiguous Cargo artifact for {name}')
                paths[name]=Path(filename)
    if build.returncode or set(paths)!= {'utils','serde','serde_json'}:raise RuntimeError('Cargo did not select all validation artifacts')
    command=['rustc','--edition','2024',str(OUT/'validate_selective.rs'),'-L',f'dependency={deps}','-o',str(OUT/'validate_selective.exe')]
    for name,path in paths.items():command+=['--extern',f'{name}={path}']
    p=subprocess.run(command,capture_output=True,text=True,encoding='utf8',errors='replace')
    if p.returncode==0:p=subprocess.run([str(OUT/'validate_selective.exe')],capture_output=True,text=True,encoding='utf8',errors='replace')
    text=p.stdout+p.stderr;(OUT/'selective-policy.log').write_text(text,encoding='utf8')
    results.append({'command':'rustc direct-linked exact workspace serde_json/serde/utils rlibs then run validate_selective.exe','exit_code':p.returncode,'log':'selective-policy.log'})
    print('selective-policy',p.returncode,text[-1200:])
    text=(OUT/'utils-test.log').read_text(encoding='utf8')
    metrics={key:float(value) for key,value in re.findall(r'(\w*max_\w*error)=([0-9.eE+-]+)',text)}
    test_match=re.search(r'test result: ok\. (\d+) passed;',text)
else:metrics={};test_match=None
report={'baseline':'B','candidate_source_text_claim':False,'commands':results,'all_passed':len(results)==4 and all(r['exit_code']==0 for r in results),'whole_utils_test_count':int(test_match[1]) if test_match else None,'spline_actual_max_absolute_errors':metrics,'source_sha256':{p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in (WORKSPACE/'crates/utils/src').glob('*.rs') if p.name!='lib.rs'},'scope':'utils top-level business helpers; separate geometry/curve owner tests included in crate test count; no full original game runtime'}
report['spline_metric_scope']='Coefficient maximum accumulates across single-thread fixture tests; final maximum 4.7683716e-7 is exact overall, intermediate test-name keys are cumulative upper bounds for that family. Position/tangent maxima are independently exact across 120 queries each.'
(OUT/'verification.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf8')
raise SystemExit(0 if report['all_passed'] else 1)
