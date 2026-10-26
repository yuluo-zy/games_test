"""Reviewed relocation adapter; never builds, launches a target, or changes tools.

Native bundles remain immutable. The derived build differs only in executable.
Only fresh, clean release common-harness cpu-probe bundles are accepted.
"""
import argparse,datetime,hashlib,json,os,pathlib,re,shutil,stat,subprocess,uuid
Path=pathlib.Path
COMMON={
 'mod.rs':'f8c81e31f52a9116b713b98e087a759d2feaef9b1d6c62dacbe3ed25ca42fe3f',
 'workload_probe.rs':'bb357cf46e3a51e5e921f454ed4b9d41c935524a8409af76eef4ff5c970d9ee8',
 'gpu_diagnostic.rs':'f30f54453e05ffe9b98bc147a13ae6da5b863f36aed4980b1f0a0ecc74431fa7'}
TOOLS={
 'perf_build_target.py':'7fdd373b73872490d52a4e1ce8a41832dbfc2d997c45688b6c1c7dd8dfb02b88',
 'perf_clock_policy.py':'76a75670bfde9b8326f210d11884d5a1fbd7554c3a67e5991cadcae93a2485dc',
 'perf_headless.py':'770a08d7c31a442ba087a78525dc6951ff42a09d1ad67e63c0adf329db100ed8',
 'perf_headless_account.py':'f20af03985b526cc6fcd2306157ee8591cc7b066bee0c4b1ef05816fa90a0395',
 'perf_etw.py':'abdfd9165cd49d3814b48cca3ca882315ef1467dad683b1b72f20e3c79ccb97d',
 'perf_trace_archive.py':'90062b4a1982e41344adfc8cdf3a7b008bfd36410c3d51eea0de30bbe8c27702',
 'perf_windows.py':'19ca12a556214b8211312157603585dff1f6f5e758830be23f1dedaba4d6e3db'}
META={'build-job-assigned.go','build-job-spec.json','build.json','build.jsonl','build.stderr.log',
      'cargo-lifecycle.json','hardware.json','requested-target-artifact.json','source.json','test-list.txt','vulkan-summary.txt'}
ADDED={'original-build.json','original-inventory.json','bundle-import-plan.json','bundle-importer.py','frozen-bundle-import.json'}
FLAGS={'reflection_scope':'TG_REFLECTION_SCOPE','vao_typed_lookup':'TG_VAO_TYPED_LOOKUP',
       'vao_wrapper_reuse':'TG_VAO_WRAPPER_REUSE','constant_batch':'TG_CONSTANT_BATCH'}
TARGET='original_gameplay_chain'
COMMAND=['cargo','build','-p','tiny-glade','--test',TARGET,'--locked','--offline','-j1',
         '--message-format=json-render-diagnostics','--features','cpu-probe','--release']
def require(ok,message):
    if not ok:raise ValueError(message)
def digest(path):
    with Path(path).open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
def signature(value):return hashlib.sha256(json.dumps(value,sort_keys=True,separators=(',',':'),ensure_ascii=False).encode()).hexdigest()
def read(path):return json.loads(Path(path).read_text(encoding='utf-8'))
def absolute(path):
    path=Path(path)
    require(path.is_absolute() and '..' not in path.parts,'absolute path without traversal required')
    return path.absolute()
def write_new(path,value):
    with Path(path).open('x',encoding='utf-8',newline='\n') as f:json.dump(value,f,indent=2,ensure_ascii=False);f.write('\n')
def no_reparse(path):
    path=Path(path)
    require(not path.is_symlink() and not path.is_junction(),'symlink/junction rejected: '+str(path))
    info=path.lstat()
    require(not (getattr(info,'st_file_attributes',0)&getattr(stat,'FILE_ATTRIBUTE_REPARSE_POINT',0x400)),'reparse point rejected')
def regular(path):
    no_reparse(path);require(stat.S_ISREG(Path(path).lstat().st_mode),'non-regular bundle file')
def chain(path,anchor):
    path=absolute(path);anchor=absolute(anchor)
    require(path==anchor or anchor in path.parents,'path outside declared root')
    for part in [path,*list(path.parents)[:len(path.parents)-len(anchor.parents)]]:
        if part.exists() or part.is_symlink() or part.is_junction():no_reparse(part)
def safe_name(name):
    require(type(name) is str and Path(name).name==name and not any(c in name for c in '/\\:'),'unsafe filename')
def inventory(bundle):
    no_reparse(bundle);items={};folded=set()
    for path in Path(bundle).iterdir():
        safe_name(path.name);require(path.name.casefold() not in folded,'case-folded filename collision');folded.add(path.name.casefold())
        regular(path);items[path.name]={'sha256':digest(path),'size':path.stat().st_size}
    return items
def isolated_git_env():
    env={key:value for key,value in os.environ.items() if not key.upper().startswith('GIT_')}
    env.update(GIT_CONFIG_NOSYSTEM='1',GIT_CONFIG_GLOBAL=os.devnull,GIT_CONFIG_SYSTEM=os.devnull,
        GIT_NO_REPLACE_OBJECTS='1',GIT_NO_LAZY_FETCH='1',GIT_TERMINAL_PROMPT='0',
        GIT_ALLOW_PROTOCOL='',GIT_OPTIONAL_LOCKS='0',GIT_ATTR_NOSYSTEM='1')
    return env
def git(root,*args):return subprocess.check_output(['git','-c','safe.directory='+Path(root).as_posix(),
    '-c','core.fsmonitor=false','-c','protocol.allow=never','-c','core.excludesFile='+os.devnull,
    '-c','core.attributesFile='+os.devnull,'-C',str(root),*args],env=isolated_git_env())
def source_snapshot(root):
    names=git(root,'ls-files','-z','--cached','--others','--exclude-standard')
    files={name:digest(Path(root)/name) if (Path(root)/name).is_file() else None
           for name in sorted(set(os.fsdecode(s) for s in names.split(b'\0') if s))}
    return {'commit':git(root,'rev-parse','HEAD').decode().strip(),'status':git(root,'status','--porcelain').decode().strip(),
            'files':files,'content_sha256':signature(files)}
def caps(root,source):
    # Byte-bound to the exact qualified implementation above, with the same
    # source filter/string test; no runtime globals or source tree are changed.
    parts=[]
    for name,expected in sorted(source['files'].items()):
        if (name.startswith('crates/tiny-glade/src/') and name.endswith('.rs')) or name=='crates/tiny-glade/tests/cpu_measurement/mod.rs':
            contents=(Path(root)/name).read_bytes();require(hashlib.sha256(contents).hexdigest()==expected,'capability source changed');parts.append(contents.decode())
    joined='\n'.join(parts);known='"management_modes"' in joined and all(v in joined for v in FLAGS.values())
    return {'probe_runtime_controls':sorted(FLAGS) if known else [],'default_modes':{k:True for k in FLAGS} if known else None,
            'evidence':'frozen source declares modes report and all four host-construction controls' if known else 'legacy or unsupported',
            'source_content_sha256':source['content_sha256']}
def tools(root):
    actual={name:digest(Path(root)/'tools'/name) for name in TOOLS}
    require(actual==TOOLS,'qualified builder/measurement tool identity differs');return actual
def validate_origin(bundle,root,expected_commit,expected_content,expected_build_sha):
    bundle,root=absolute(bundle),absolute(root);chain(bundle,root)
    inv=inventory(bundle);require(digest(bundle/'build.json')==expected_build_sha,'original build SHA changed')
    build,source=read(bundle/'build.json'),read(bundle/'source.json')
    label=build.get('label');require(type(label) is str and re.fullmatch(r'[A-Za-z0-9_-]+',label),'unsafe label')
    require(bundle==root/'.perf/headless-builds'/label,'origin outside own labeled freeze directory')
    require(build.get('schema')=='headless-frozen-build-v1' and build['source_commit']==expected_commit and build['source_content_sha256']==expected_content,'source binding differs')
    require(re.fullmatch(r'[0-9a-f]{40}',expected_commit) is not None and re.fullmatch(r'[0-9a-f]{64}',expected_content) is not None,'missing full source identity')
    require(build['profile']=='release' and build['dirty'] is False and build['features']==['default(mimalloc)','cpu-probe'] and build['cpu_probe_feature'] is True,'clean common release probe required')
    require(build['allocator']=='mimalloc' and build['jobs']==1 and build['incremental'] is False and build['command']==COMMAND,'build configuration differs')
    require(signature(build['build_environment'])==build['build_environment_sha256'],'build environment hash differs')
    require(source['commit']==expected_commit and not source['status'] and source['content_sha256']==expected_content and signature(source['files'])==expected_content,'source archive identity differs')
    require(source_snapshot(root)==source,'current original source/clean status changed')
    tools(root)
    for name,expected in COMMON.items():require(source['files']['crates/tiny-glade/tests/cpu_measurement/'+name]==expected,'common source payload differs')
    current_caps=caps(root,source);require(build['management_controls']==current_caps,'current capability differs')
    require(current_caps['default_modes'] is None and current_caps['probe_runtime_controls']==[],'runtime ablation capability not common contract')
    exe=Path(build['executable']);require(exe.parent==bundle,'original executable outside bundle')
    require(re.fullmatch(r'original_gameplay_chain-[0-9a-f]{16}\.exe',exe.name) is not None,'unexpected target executable')
    names={exe.name,exe.with_suffix('.pdb').name,'fmod.dll','fmodstudio.dll'}
    require(set(build['files'])==names and set(inv)==names|META,'unknown/missing frozen files or DLLs')
    for name,expected in build['files'].items():safe_name(name);require(inv[name]['sha256']==expected,'frozen payload SHA changed')
    require(build['executable_sha256']==build['files'][exe.name],'executable identity differs')
    require(Path(build['executable']).resolve(strict=True)==(bundle/exe.name).resolve(strict=True),'executable relocation/reparse differs')
    lifecycle=read(bundle/'cargo-lifecycle.json');require(lifecycle==build['cargo_lifecycle'],'Job lifecycle differs')
    require(build['requested_test_target_completed'] is True and lifecycle['requested_target_completed'] is True and lifecycle['active_build_processes_remaining'] is False,'requested target incomplete or Job active')
    require(lifecycle['job_assigned_before_cargo_start'] is True and type(lifecycle['wrapper_pid']) is int and lifecycle['wrapper_pid']>0
            and type(lifecycle['cargo_pid']) is int and lifecycle['cargo_pid']>0,'owned Job assignment/process proof differs')
    require(build['entire_cargo_build_passed']==lifecycle['entire_cargo_build_passed']==(lifecycle['cargo_exit_code']==0),'whole Cargo status differs')
    if lifecycle['termination_was_intentional_after_requested_artifact']:
        require(lifecycle['job_termination_exit_code']==0xE000C0DE,'owned Job termination identity differs')
    for field,name in [('requested_target_evidence_sha256','requested-target-artifact.json'),('build_jsonl_sha256','build.jsonl')]:require(build[field]==inv[name]['sha256'],'target/log evidence SHA differs')
    require(build['build_helper_sha256']==TOOLS['perf_build_target.py'],'builder helper identity differs')
    spec=read(bundle/'build-job-spec.json')
    require(Path(spec['cwd'])==root and spec['command']==COMMAND and Path(spec['gate'])==bundle/'build-job-assigned.go','owned Job cwd/command/gate differs')
    require((bundle/'build-job-assigned.go').read_text()=='assigned','Job assignment proof differs')
    artifact=read(bundle/'requested-target-artifact.json')
    require(artifact.get('reason')=='compiler-artifact' and artifact['target']['name']==TARGET and artifact['target']['kind']==['test'],'requested artifact kind differs')
    require(artifact['features']==['cpu-probe','default','mimalloc'] and artifact['profile']['test'] is True and artifact['profile']['opt_level']=='3','target profile/features differ')
    require(Path(artifact['manifest_path']).resolve()==(root/'crates/tiny-glade/Cargo.toml').resolve(),'target manifest differs')
    require(Path(artifact['executable']).name==exe.name and Path(artifact['executable']).parent.resolve()==(root/'target/release/deps').resolve(),'target cache path differs')
    events=[];starts=[]
    for line in (bundle/'build.jsonl').read_text().splitlines():
        try:event=json.loads(line)
        except ValueError:continue
        require(not (event.get('reason')=='compiler-message' and event.get('message',{}).get('level')=='error'),'compiler error in frozen log')
        if event.get('reason')=='compiler-artifact' and event.get('target',{}).get('name')==TARGET:events.append(event)
        if event.get('reason')=='perf-owned-cargo-start':starts.append(event)
    require(events==[artifact],'target compiler JSON missing, duplicated, or differs')
    require(starts==[{'reason':'perf-owned-cargo-start','pid':lifecycle['cargo_pid'],'wrapper_pid':lifecycle['wrapper_pid']}],'owned Cargo start evidence differs')
    require('cpu_measurement::profile_shipped_session_cpu: test' in (bundle/'test-list.txt').read_text(),'requested ignored fixture missing')
    require(read(bundle/'hardware.json')==build['hardware'],'hardware snapshot differs')
    require(inv['vulkan-summary.txt']['sha256']==build['hardware']['vulkan_summary_sha256'],'Vulkan identity differs')
    return {'build':build,'source':source,'inventory':inv,'current_caps':current_caps}
def prepare_plan(bundle,source_root,host_root,expected_commit,expected_content,expected_build_sha):
    bundle,source_root,host_root=map(absolute,[bundle,source_root,host_root]);no_reparse(host_root)
    proof=validate_origin(bundle,source_root,expected_commit,expected_content,expected_build_sha);tools(host_root)
    destination=host_root/'.perf/headless-builds'/proof['build']['label'];chain(destination.parent,host_root)
    require(not destination.exists() and not destination.is_symlink() and not destination.is_junction(),'host label exists')
    return {'schema':'headless-frozen-bundle-import-plan-v1','import_nonce':str(uuid.uuid4()),
        'created_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'source_bundle':str(bundle),'source_root':str(source_root),
        'host_root':str(host_root),'destination':str(destination),'label':proof['build']['label'],
        'source_commit':expected_commit,'source_content_sha256':expected_content,'original_build_sha256':expected_build_sha,
        'original_inventory':proof['inventory'],'current_caps':proof['current_caps'],'qualified_tools_sha256':TOOLS,
        'importer_sha256':digest(__file__),'original_build_nonce':proof['build'].get('nonce'),
        'only_derived_build_changes':['executable']}
def verify_import(bundle,check_origin=True,*,logical_bundle=None):
    bundle=absolute(bundle);manifest=read(bundle/'frozen-bundle-import.json');require(manifest['schema']=='headless-frozen-bundle-import-v1','unknown import schema')
    logical_bundle=bundle if logical_bundle is None else absolute(logical_bundle)
    plan=read(bundle/'bundle-import-plan.json');require(signature(plan)==manifest['plan_content_sha256'],'import plan differs')
    require(str(uuid.UUID(plan['import_nonce']))==manifest['import_nonce']==plan['import_nonce'] and uuid.UUID(plan['import_nonce']).version==4,'import nonce differs')
    require(Path(plan['destination'])==logical_bundle and logical_bundle.name==plan['label'],'derived bundle path/label differs')
    require(digest(bundle/'bundle-importer.py')==manifest['importer_sha256']==plan['importer_sha256'],'importer identity differs')
    require(digest(bundle/'original-build.json')==manifest['original_build_sha256']==plan['original_build_sha256'],'original build archive differs')
    require(read(bundle/'original-inventory.json')==plan['original_inventory'],'original inventory copy differs')
    original=read(bundle/'original-build.json');derived=read(bundle/'build.json')
    expected=dict(original);expected['executable']=str(logical_bundle/Path(original['executable']).name)
    require(derived==expected and manifest['only_derived_build_changes']==['executable'],'derived metadata changed more than executable')
    require(manifest['original_bundle']==plan['source_bundle'] and manifest['derived_bundle']==plan['destination'],'manifest source/destination differs')
    require(manifest['source_commit_unchanged']==derived['source_commit']==plan['source_commit']
            and manifest['source_content_sha256_unchanged']==derived['source_content_sha256']==plan['source_content_sha256'],'manifest source identity differs')
    require(manifest['entire_cargo_build_passed_unchanged']==original['entire_cargo_build_passed']
            and manifest['current_source_caps_verified_before_after'] is True,'manifest proof/status differs')
    require(manifest['original_build_nonce']==plan['original_build_nonce']==original.get('nonce'),'original nonce differs')
    require(digest(bundle/'build.json')==manifest['derived_build_sha256'],'derived build SHA differs')
    inv=inventory(bundle);require(set(inv)==set(plan['original_inventory'])|ADDED,'missing/unknown imported inventory')
    for name,expected in plan['original_inventory'].items():
        if name!='build.json':require(inv[name]==expected,'imported original bytes differ: '+name)
    if check_origin:
        proof=validate_origin(plan['source_bundle'],plan['source_root'],plan['source_commit'],plan['source_content_sha256'],plan['original_build_sha256'])
        require(proof['inventory']==plan['original_inventory'] and proof['current_caps']==plan['current_caps'],'original source/caps/payload changed after import')
    tools(plan['host_root']);return manifest
def execute_plan(plan):
    require(plan['schema']=='headless-frozen-bundle-import-plan-v1' and plan['importer_sha256']==digest(__file__),'plan/importer identity changed')
    require(str(uuid.UUID(plan['import_nonce']))==plan['import_nonce'] and uuid.UUID(plan['import_nonce']).version==4,'invalid nonce')
    source,root,host,dest=map(absolute,[plan['source_bundle'],plan['source_root'],plan['host_root'],plan['destination']])
    proof=validate_origin(source,root,plan['source_commit'],plan['source_content_sha256'],plan['original_build_sha256'])
    require(proof['inventory']==plan['original_inventory'] and proof['current_caps']==plan['current_caps'],'origin changed since planning')
    require(plan['qualified_tools_sha256']==TOOLS and plan['only_derived_build_changes']==['executable'],'plan policy changed')
    require(plan['label']==proof['build']['label'] and dest==host/'.perf/headless-builds'/plan['label'],'destination label/root changed')
    tools(host);chain(dest.parent,host);require(not dest.exists() and not dest.is_symlink() and not dest.is_junction(),'refusing existing destination')
    dest.parent.mkdir(parents=True,exist_ok=True);chain(dest.parent,host)
    temporary=dest.with_name('.import-'+dest.name+'-'+plan['import_nonce']);temporary.mkdir(exist_ok=False)
    # Failure preserves this exclusive incomplete directory; it is never published
    # as a labeled build and never auto-deletes a computed path.
    for name in proof['inventory']:
        regular(source/name);shutil.copy2(source/name,temporary/name)
        require(digest(temporary/name)==proof['inventory'][name]['sha256'],'copy verification failed')
    shutil.copy2(source/'build.json',temporary/'original-build.json')
    write_new(temporary/'original-inventory.json',plan['original_inventory'])
    write_new(temporary/'bundle-import-plan.json',plan)
    shutil.copy2(__file__,temporary/'bundle-importer.py')
    derived=dict(proof['build']);derived['executable']=str(dest/Path(derived['executable']).name)
    (temporary/'build.json').write_text(json.dumps(derived,indent=2,ensure_ascii=False)+'\n',encoding='utf-8',newline='\n')
    after=validate_origin(source,root,plan['source_commit'],plan['source_content_sha256'],plan['original_build_sha256'])
    require(after==proof,'origin changed during copy')
    manifest={'schema':'headless-frozen-bundle-import-v1','import_nonce':plan['import_nonce'],
        'original_build_sha256':plan['original_build_sha256'],'derived_build_sha256':digest(temporary/'build.json'),
        'original_bundle':str(source),'derived_bundle':str(dest),'importer_sha256':digest(__file__),
        'plan_content_sha256':signature(plan),'only_derived_build_changes':['executable'],
        'current_source_caps_verified_before_after':True,'original_build_nonce':plan['original_build_nonce'],
        'entire_cargo_build_passed_unchanged':derived['entire_cargo_build_passed'],
        'source_commit_unchanged':derived['source_commit'],'source_content_sha256_unchanged':derived['source_content_sha256']}
    write_new(temporary/'frozen-bundle-import.json',manifest)
    verify_import(temporary,logical_bundle=dest)
    chain(dest.parent,host);require(not dest.exists() and not dest.is_symlink() and not dest.is_junction(),'destination appeared during copy')
    temporary.rename(dest)
    verify_import(dest);return manifest
def main():
    p=argparse.ArgumentParser();sub=p.add_subparsers(dest='operation',required=True)
    a=sub.add_parser('plan');a.add_argument('source_bundle',type=Path);a.add_argument('--source-root',type=Path,required=True);a.add_argument('--host-root',type=Path,required=True)
    for flag in ['expected-commit','expected-content','expected-build-sha256']:a.add_argument('--'+flag,required=True)
    a.add_argument('--output',type=Path,required=True)
    a=sub.add_parser('import');a.add_argument('plan',type=Path);a.add_argument('--plan-sha256',required=True)
    a=sub.add_parser('verify');a.add_argument('bundle',type=Path)
    args=p.parse_args()
    if args.operation=='plan':
        result=prepare_plan(args.source_bundle,args.source_root,args.host_root,args.expected_commit,args.expected_content,args.expected_build_sha256);write_new(args.output,result)
    elif args.operation=='import':
        require(digest(args.plan)==args.plan_sha256,'import plan SHA changed');result=execute_plan(read(args.plan))
    else:result=verify_import(args.bundle)
    print(json.dumps(result,indent=2,ensure_ascii=False))
if __name__=='__main__':main()
