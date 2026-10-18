from pathlib import Path
import subprocess, hashlib, json, datetime, re
workspace = Path(__file__).resolve().parents[5]
report = Path(__file__).resolve().parent
report.mkdir(parents=True, exist_ok=True)
checks = [
 ('all-tests', ['cargo','test','-p','rhapsody','--offline','--all-features','--lib','--tests','--no-fail-fast','-j','2','--','--nocapture']),
 ('default-tests', ['cargo','test','-p','rhapsody','--offline','--lib','--tests','--no-fail-fast','-j','2','--','--nocapture']),
 ('shader-tests', ['cargo','test','-p','shader-compiler','-p','shader-pipeline','--offline','--all-features','--lib','--tests','--no-fail-fast','-j','2','--','--nocapture']),
 ('default-check', ['cargo','check','-p','rhapsody','--offline','--all-targets','-j','2']),
 ('no-default-check', ['cargo','check','-p','rhapsody','--offline','--no-default-features','--all-targets','-j','2']),
 ('gpu-text-targets', ['cargo','check','-p','rhapsody','-p','tg-text','--offline','--all-features','--all-targets','-j','2']),
 ('consumer-targets', ['cargo','check','-p','rhapsody','-p','tg-text','-p','tiny-glade','--offline','--all-features','--all-targets','-j','2']),
 ('country-core-lib', ['cargo','check','-p','rhapsody','-p','tg-text','-p','tiny-glade','-p','country-core','--offline','--all-features','--lib','-j','2']),
 ('window-source-check', ['cargo','check','-p','rhapsody','--offline','--all-features','--example','__window_lab_sourcecheck','-j','2']),
 ('strict-clippy', ['cargo','clippy','-p','rhapsody','--offline','--all-features','--lib','--tests','--no-deps','-j','2','--','-D','warnings']),
]
def fingerprint():
    digest = hashlib.sha256()
    paths = sorted((workspace/'crates/rhapsody/src').rglob('*.rs')) + sorted((workspace/'crates/rhapsody/tests').rglob('*.rs')) + sorted((workspace/'crates/rhapsody/examples').rglob('*.rs'))
    paths += sorted((workspace/'crates/rhapsody/crates').rglob('*.rs'))
    paths += [workspace/'crates/country-core/src/render/execution_host.rs',workspace/'crates/country-core/src/rhapsody_contract.rs',workspace/'crates/country-core/src/rhapsody_types.rs',workspace/'crates/country-core/src/render/ssbo.rs',workspace/'crates/tiny-glade/src/render_host.rs',workspace/'crates/tiny-glade/src/raster_host.rs',workspace/'crates/tg-text/src/gpu.rs',workspace/'crates/tg-text/examples/text_gpu.rs',workspace/'crates/tiny-glade/tests/core_raster_draw.rs',workspace/'crates/tiny-glade/tests/render_host.rs',workspace/'Cargo.toml',workspace/'Cargo.lock',workspace/'crates/rhapsody/Cargo.toml']
    for path in sorted(paths):
        if path.name == '__window_lab_sourcecheck.rs': continue
        digest.update(str(path.relative_to(workspace)).encode()+b'\0'+path.read_bytes()+b'\0')
    return digest.hexdigest()
results=[]
start=fingerprint()
manifest={'recorded_at':datetime.datetime.now(datetime.timezone.utc).isoformat(), 'source_sha256_start':start,'git_head':subprocess.run(['git','rev-parse','HEAD'],cwd=workspace,capture_output=True,text=True).stdout.strip(), 'git_dirty':bool(subprocess.run(['git','status','--porcelain'],cwd=workspace,capture_output=True,text=True).stdout.strip()),'fingerprint_scope':'rhapsody Rust src/tests/examples and nested shader crates, plus manifest; actual consumer files; workspace manifest and lockfile','checks':results,'hardware':'BLOCKED/deferred by user; no Windows or Vulkan runtime validation; test early returns are not GPU PASS'}
for name, command in checks:
    print('RUN '+name,flush=True)
    before=fingerprint()
    wrapper=workspace/'crates/rhapsody/examples/__window_lab_sourcecheck.rs'
    if name=='window-source-check':
        if wrapper.exists(): raise RuntimeError('refusing to replace existing source-check wrapper')
        wrapper.write_text('#[path = \"window_lab/app.rs\"] mod app;\n#[path = \"window_lab/fixture.rs\"] mod fixture;\n#[path = \"window_lab/win32.rs\"] mod win32;\nfn main() { let _ = app::run(); }\n')
    try:
        with (report/(name+'.log')).open('w') as stream:
            result=subprocess.run(command,cwd=workspace,stdout=stream,stderr=subprocess.STDOUT)
    finally:
        if name=='window-source-check': wrapper.unlink()
    entry={'name':name,'command':command,'exit_code':result.returncode,'log':name+'.log','source_sha256_before':before,'source_sha256_after':fingerprint()}
    if name=='window-source-check': entry['limitation']='temporary wrapper type-checks Windows host sources on macOS; does not validate Windows target ABI, linking, or runtime'
    if name in ('all-tests', 'default-tests', 'shader-tests'):
        output=(report/(name+'.log')).read_text()
        suites=re.findall(r'test result: (\w+)\. (\d+) passed; (\d+) failed; (\d+) ignored',output)
        entry['process_results']={'suites':len(suites),'passed':sum(int(row[1]) for row in suites),'failed':sum(int(row[2]) for row in suites),'ignored':sum(int(row[3]) for row in suites),'includes_gpu_skips':name!='shader-tests','includes_environment_skips':True}
    results.append(entry)
    (report/'checks.json').write_text(json.dumps(manifest,ensure_ascii=False,indent=2)+'\n')
    print('DONE '+name+' exit='+str(result.returncode),flush=True)
manifest['source_sha256_end']=fingerprint()
manifest['source_unchanged_during_matrix']=start==manifest['source_sha256_end'] and all(item['source_sha256_before']==start and item['source_sha256_after']==start for item in results)
manifest['toolchain']=subprocess.run(['rustc','-vV'],cwd=workspace,capture_output=True,text=True).stdout.strip()
(report/'checks.json').write_text(json.dumps(manifest,ensure_ascii=False,indent=2)+'\n')
print(json.dumps({'failed':[item['name'] for item in results if item['exit_code']], 'stable':manifest['source_unchanged_during_matrix']},ensure_ascii=False),flush=True)

raise SystemExit(1 if any(item['exit_code'] for item in results) else 0)
