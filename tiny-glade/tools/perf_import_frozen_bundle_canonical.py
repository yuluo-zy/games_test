"""Reviewed relocation adapter; never builds, launches a target, or changes tools.

Native bundles remain immutable. The derived build differs only in executable.
Only fresh, clean release common-harness cpu-probe bundles are accepted.
"""
import argparse,datetime,hashlib,json,os,pathlib,re,shutil,stat,subprocess,uuid
Path=pathlib.Path
COMMON = {'crates/tiny-glade/tests/cpu_measurement/mod.rs': 'd2d1e71eeb425cbf5119c17bad6de718c7cd0fa699540bee1aa82e278ce4813b', 'crates/tiny-glade/tests/cpu_measurement/workload_probe.rs': 'bb357cf46e3a51e5e921f454ed4b9d41c935524a8409af76eef4ff5c970d9ee8', 'crates/tiny-glade/tests/cpu_measurement/gpu_diagnostic.rs': 'f30f54453e05ffe9b98bc147a13ae6da5b863f36aed4980b1f0a0ecc74431fa7', 'crates/tiny-glade/tests/support/performance_sparse_layout.rs': '5ccf99448777fdee307a417c1dcf43d12b0f5551938d3f783eace9a21ff14ff9'}
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
ADDED={'canonical-source-archive.json','original-build.json','original-inventory.json','bundle-import-plan.json','bundle-importer.py','frozen-bundle-import.json'}
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
    canonical_archive=archive(root,source)
    canonical_contract=validate_archive(canonical_archive,source)
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
    return {'build':build,'source':source,'inventory':inv,'current_caps':current_caps,
        'canonical_source_archive':canonical_archive,'canonical_source_contract':canonical_contract}
def prepare_plan(bundle,source_root,host_root,expected_commit,expected_content,expected_build_sha):
    bundle,source_root,host_root=map(absolute,[bundle,source_root,host_root]);no_reparse(host_root)
    proof=validate_origin(bundle,source_root,expected_commit,expected_content,expected_build_sha);tools(host_root)
    destination=host_root/'.perf/headless-builds'/proof['build']['label'];chain(destination.parent,host_root)
    require(not destination.exists() and not destination.is_symlink() and not destination.is_junction(),'host label exists')
    return {'schema':'headless-canonical-frozen-bundle-import-plan-v2','import_nonce':str(uuid.uuid4()),
        'created_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'source_bundle':str(bundle),'source_root':str(source_root),
        'host_root':str(host_root),'destination':str(destination),'label':proof['build']['label'],
        'source_commit':expected_commit,'source_content_sha256':expected_content,'original_build_sha256':expected_build_sha,
        'original_inventory':proof['inventory'],'current_caps':proof['current_caps'],'qualified_tools_sha256':TOOLS,
        'canonical_source_archive':proof['canonical_source_archive'],
        'canonical_source_contract':proof['canonical_source_contract'],
        'importer_sha256':digest(__file__),'original_build_nonce':proof['build'].get('nonce'),
        'only_derived_build_changes':['executable']}
def verify_import(bundle,check_origin=True,*,logical_bundle=None):
    bundle=absolute(bundle);manifest=read(bundle/'frozen-bundle-import.json');require(manifest['schema']=='headless-canonical-frozen-bundle-import-v2','unknown import schema')
    logical_bundle=bundle if logical_bundle is None else absolute(logical_bundle)
    plan=read(bundle/'bundle-import-plan.json');require(signature(plan)==manifest['plan_content_sha256'],'import plan differs')
    require(plan.get('schema')=='headless-canonical-frozen-bundle-import-plan-v2','unknown canonical import plan schema')
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
    source=read(bundle/'source.json')
    contract=validate_archive(read(bundle/'canonical-source-archive.json'),source)
    require(contract==manifest['canonical_source_contract']==plan['canonical_source_contract']
            and read(bundle/'canonical-source-archive.json')==plan['canonical_source_archive'],
            'canonical imported source contract/archive changed')
    if check_origin:
        proof=validate_origin(plan['source_bundle'],plan['source_root'],plan['source_commit'],plan['source_content_sha256'],plan['original_build_sha256'])
        require(proof['inventory']==plan['original_inventory'] and proof['current_caps']==plan['current_caps']
                and proof['canonical_source_archive']==plan['canonical_source_archive'],
                'original source/caps/payload/canonical archive changed after import')
    tools(plan['host_root']);return manifest
def execute_plan(plan):
    require(plan['schema']=='headless-canonical-frozen-bundle-import-plan-v2' and plan['importer_sha256']==digest(__file__),'plan/importer identity changed')
    require(str(uuid.UUID(plan['import_nonce']))==plan['import_nonce'] and uuid.UUID(plan['import_nonce']).version==4,'invalid nonce')
    source,root,host,dest=map(absolute,[plan['source_bundle'],plan['source_root'],plan['host_root'],plan['destination']])
    proof=validate_origin(source,root,plan['source_commit'],plan['source_content_sha256'],plan['original_build_sha256'])
    require(proof['inventory']==plan['original_inventory'] and proof['current_caps']==plan['current_caps']
            and proof['canonical_source_archive']==plan['canonical_source_archive']
            and proof['canonical_source_contract']==plan['canonical_source_contract'],'origin changed since planning')
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
    write_new(temporary/'canonical-source-archive.json',plan['canonical_source_archive'])
    shutil.copy2(__file__,temporary/'bundle-importer.py')
    derived=dict(proof['build']);derived['executable']=str(dest/Path(derived['executable']).name)
    (temporary/'build.json').write_text(json.dumps(derived,indent=2,ensure_ascii=False)+'\n',encoding='utf-8',newline='\n')
    after=validate_origin(source,root,plan['source_commit'],plan['source_content_sha256'],plan['original_build_sha256'])
    require(after==proof,'origin changed during copy')
    manifest={'schema':'headless-canonical-frozen-bundle-import-v2','import_nonce':plan['import_nonce'],
        'original_build_sha256':plan['original_build_sha256'],'derived_build_sha256':digest(temporary/'build.json'),
        'original_bundle':str(source),'derived_bundle':str(dest),'importer_sha256':digest(__file__),
        'plan_content_sha256':signature(plan),'only_derived_build_changes':['executable'],
        'canonical_source_contract':plan['canonical_source_contract'],
        'current_source_caps_verified_before_after':True,'original_build_nonce':plan['original_build_nonce'],
        'entire_cargo_build_passed_unchanged':derived['entire_cargo_build_passed'],
        'source_commit_unchanged':derived['source_commit'],'source_content_sha256_unchanged':derived['source_content_sha256']}
    write_new(temporary/'frozen-bundle-import.json',manifest)
    verify_import(temporary,logical_bundle=dest)
    chain(dest.parent,host);require(not dest.exists() and not dest.is_symlink() and not dest.is_junction(),'destination appeared during copy')
    temporary.rename(dest)
    verify_import(dest);return manifest
import base64
import hashlib
import json
import re
from pathlib import Path

HARNESS = 'common-cpu-workload-canonical-v2'
CONTRACT = 'canonical-common-input-closure-v2'
PREPARATION = 'canonical-brick-layout-v1'
LEGACY_HARNESS = 'common-cpu-workload-v1'
COMMON4 = {'crates/tiny-glade/tests/cpu_measurement/mod.rs': 'd2d1e71eeb425cbf5119c17bad6de718c7cd0fa699540bee1aa82e278ce4813b', 'crates/tiny-glade/tests/cpu_measurement/workload_probe.rs': 'bb357cf46e3a51e5e921f454ed4b9d41c935524a8409af76eef4ff5c970d9ee8', 'crates/tiny-glade/tests/cpu_measurement/gpu_diagnostic.rs': 'f30f54453e05ffe9b98bc147a13ae6da5b863f36aed4980b1f0a0ecc74431fa7', 'crates/tiny-glade/tests/support/performance_sparse_layout.rs': '5ccf99448777fdee307a417c1dcf43d12b0f5551938d3f783eace9a21ff14ff9'}
SHARED = {'crates/tiny-glade/tests/cpu_measurement/mod.rs': 'd2d1e71eeb425cbf5119c17bad6de718c7cd0fa699540bee1aa82e278ce4813b', 'crates/tiny-glade/tests/cpu_measurement/workload_probe.rs': 'bb357cf46e3a51e5e921f454ed4b9d41c935524a8409af76eef4ff5c970d9ee8', 'crates/tiny-glade/tests/cpu_measurement/gpu_diagnostic.rs': 'f30f54453e05ffe9b98bc147a13ae6da5b863f36aed4980b1f0a0ecc74431fa7', 'crates/tiny-glade/tests/support/performance_sparse_layout.rs': '5ccf99448777fdee307a417c1dcf43d12b0f5551938d3f783eace9a21ff14ff9', 'crates/tiny-glade/tests/support/performance_input.rs': 'ba6c70d1ce83bd1ff664f8295db05822d796e7cc55f0ecea69826376670faa4d', 'crates/tiny-glade/tests/support/performance_workload.rs': '3890c544d3f79c2d8a542acf064d70ef66cdae351a5b33d297d6bd2b6359eae4', 'crates/tiny-glade/tests/support/mod.rs': 'd4d2b5130c6375d839fce04421e252dda357fcada838dfcde21b0aba51c3c554', 'crates/tiny-glade/tests/support/upload_wait.rs': 'd5263fb610c19d996e8038bc3b41b5bdbe3ac9dc7b0ae29fd3c87c6a43ed825e'}
GETTERS = {'crates/country-core/src/render/sparse_instance_buffer.rs': ['pub struct SparseFixtureReferences {\n    pub cached_draw_snapshot: bool,\n    pub draw_snapshot_strong: usize,\n    pub draw_snapshot_weak: usize,\n    pub owned_buffer_present: bool,\n    pub owned_buffer_strong: usize,\n    pub owned_buffer_weak: usize,\n}', '    pub fn fixture_reference_state(&self) -> SparseFixtureReferences {\n        let cache = self.draw_snapshot.lock();\n        SparseFixtureReferences {\n            cached_draw_snapshot: cache.is_some(),\n            draw_snapshot_strong: cache.as_ref().map_or(0, Arc::strong_count),\n            draw_snapshot_weak: cache.as_ref().map_or(0, Arc::weak_count),\n            owned_buffer_present: self._buffer_owner.is_some(),\n            owned_buffer_strong: self._buffer_owner.as_ref().map_or(0, Arc::strong_count),\n            owned_buffer_weak: self._buffer_owner.as_ref().map_or(0, Arc::weak_count),\n        }\n    }'], 'crates/rhapsody/src/transfer/queue.rs': ['    pub(super) fn fixture_weak_history_count(&self, target: &Arc<T>) -> usize {\n        self.epochs.iter().filter(|entry| entry.target.as_ptr() == Arc::as_ptr(target)).count()\n    }'], 'crates/rhapsody/src/transfer.rs': ['    pub fn fixture_buffer_weak_history_count(&self, target: &Arc<Buffer>) -> usize {\n        self.queued.fixture_weak_history_count(target)\n    }'], 'crates/tiny-glade/src/render_upload_boundary.rs': ['    pub(crate) fn fixture_has_active_reader(&self) -> bool { self.owner.lock().is_some() }'], 'crates/tiny-glade/src/render_host.rs': ['pub struct BufferFixtureReferences {\n    pub frame_bound: bool,\n    pub active_read_lease: bool,\n    pub version_strong: usize,\n    pub version_weak: usize,\n    pub snapshot_pins: usize,\n    pub physical_strong: usize,\n    pub physical_weak: usize,\n    pub known_queue_history_weak: usize,\n    pub known_cow_retired_weak: usize,\n}', '    pub fn fixture_reference_state(&self, target: &Buffer) -> Result<BufferFixtureReferences, BackendError> {\n        let _serial = self.publication.lock();\n        let version = self.current_version(target).ok_or_else(|| BackendError::invalid_usage("fixture target is stale/foreign"))?;\n        let raw = version.buffer.shared();\n        let known_queue_history_weak = self.uploads.lock().as_ref().map_or(0,\n            |service| service.manager.fixture_buffer_weak_history_count(&raw));\n        let known_cow_retired_weak = self.cow_budget.lock().retired.iter()\n            .filter(|entry| entry.owner.as_ptr() == Arc::as_ptr(&raw)).count();\n        Ok(BufferFixtureReferences {\n            frame_bound: self.frame.is_some(),\n            active_read_lease: self.boundary().fixture_has_active_reader(),\n            version_strong: Arc::strong_count(&version) - 1,\n            version_weak: Arc::weak_count(&version),\n            snapshot_pins: version.pins.load(Ordering::Acquire),\n            physical_strong: Arc::strong_count(&raw) - 1,\n            physical_weak: Arc::weak_count(&raw),\n            known_queue_history_weak,\n            known_cow_retired_weak,\n        })\n    }']}
RICH_CALLER_SHA256 = '2d7dd64b1a36d07139d7ebe098b214e8cf1b3cc6ea9f673becf40fd5d79e23f8'

BOOT_PATH = "crates/tiny-glade/tests/original_gameplay_chain.rs"
BOOT = 'fn boot_shipped_session() -> Result<BootedSession> {\n    struct Diagnostics;\n    impl log::Log for Diagnostics {\n        fn enabled(&self, metadata: &log::Metadata<\'_>) -> bool { metadata.level() <= log::Level::Error }\n        fn log(&self, record: &log::Record<\'_>) {\n            if self.enabled(record.metadata()) { eprintln!("{} {}", record.target(), record.args()); }\n        }\n        fn flush(&self) {}\n    }\n    static DIAGNOSTICS: Diagnostics = Diagnostics;\n    if log::set_logger(&DIAGNOSTICS).is_ok() { log::set_max_level(log::LevelFilter::Error); }\n\n    let installed = std::env::var_os("TG_INSTALL_ROOT")\n        .map(PathBuf::from)\n        .unwrap_or_else(|| {\n            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../ljxsj_92385/Tiny Glade")\n        })\n        .canonicalize()?;\n    // Both shipped-session tests boot a whole App in this process: the account paths are process\n    // global, so they are initialised once and the boots are serialised.\n    static BOOT: std::sync::Mutex<()> = std::sync::Mutex::new(());\n    static ACCOUNT: std::sync::OnceLock<tempfile::TempDir> = std::sync::OnceLock::new();\n    let boot_guard = BOOT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);\n    if ACCOUNT.get().is_none() {\n        let account = tempfile::tempdir()?;\n        paths::initialize_paths_with_installation(0, Some(account.path()), &installed)?;\n        let _ = ACCOUNT.set(account);\n    }\n    std::env::set_current_dir(&installed)?;\n    utils::local_time::init_local_time();\n    tiny_glade::load_language_library::load_language_library(None, None);\n    let cli = CliArgs::parse_from(["original-gameplay-chain"]);\n    let config = get_app_config(&cli)?;\n    assert!(\n        !config.dev.demo,\n        "this acceptance requires the full original build"\n    );\n    let mut app = run_game_thread(config, &cli, {\n        // 只用于出高分辨率验证图：TG_TEST_SIZE=宽x高，默认 480x270。\n        let (w, h) = std::env::var("TG_TEST_SIZE")\n            .ok()\n            .and_then(|v| v.split_once(\'x\').and_then(|(w, h)| Some((w.parse().ok()?, h.parse().ok()?))))\n            .unwrap_or((480, 270));\n        WindowSize::new(w, h)\n    })?;\n    app.insert_resource(\n        country_core::resources::platform::RuntimePlatform::from_detection(false, false),\n    );\n    tiny_glade::game_loop::register_platform_events(&mut app);\n    tiny_glade::game_loop::register_settings_application_system(&mut app);\n    let device =\n        rhapsody::device::Device::new_headless(&rhapsody::device::DeviceConfig {\n            // Same budget as the production window path (the original allocator has no cap).\n            memory_budget_bytes: tiny_glade::production_renderer::UNBOUNDED_DEVICE_BUDGET_BYTES,\n            ..rhapsody::device::DeviceConfig::headless()\n        })\n        .context("strict gameplay physical device")?;\n    let shaders = app.world().resource::<ApplicationShaderLibrary>().0.clone();\n    let bridge = CoreRenderBridge::new(device.clone(), installed.clone(), shaders.clone())?;\n    let mut final_post = None;\n    country_core::startup::load_shaders::load_shaders(\n        &shaders,\n        bridge.shader_backend(),\n        &mut final_post,\n    );\n    // Same shader set as ProductionRenderer::new: the world tail (ui_2d, text, vermeer,\n    // painterly, blit) is part of the compiled production graph.\n    country_core::startup::load_shaders::load_ui_shaders(&shaders, bridge.shader_backend());\n    country_core::startup::startup_blit::startup_blit(&shaders, bridge.shader_backend());\n    struct TailShaders<\'a> {\n        library: &\'a country_core::asset_libraries::shader_library::AssetShaderLibrary,\n        backend: &\'a dyn country_core::render::shader::boundary::ShaderBackend,\n    }\n    impl country_core::systems::render_loop::painterly::ShaderLoader for TailShaders<\'_> {\n        fn load_shader_into_library(&mut self, path: &\'static str, name: &\'static str) {\n            country_core::startup::load_shaders::load_shader_into_library(\n                self.library,\n                self.backend,\n                path,\n                name,\n            );\n        }\n    }\n    country_core::systems::render_loop::painterly::startup_painterly(&mut TailShaders {\n        library: &shaders,\n        backend: bridge.shader_backend(),\n    });\n    country_core::systems::render_loop::vermeer::startup_vermeer(&mut TailShaders {\n        library: &shaders,\n        backend: bridge.shader_backend(),\n    });\n    bridge.install_asset_libraries(app.world_mut())?;\n    tiny_glade::gameplay_startup::install_gameplay_device_runtime(\n        &mut app,\n        device.clone(),\n        bridge.hosts().clone(),\n        installed,\n    )?;\n    // The window path uses this driver to install the same asset maintenance\n    // and canonical egui Context before its first App::update.\n    let context = egui::Context::default();\n    let mut driver = tiny_glade::game_loop::GameFrameDriver::new(app, context.clone());\n    driver\n        .app_mut()\n        .insert_resource(bevy_time::TimeUpdateStrategy::ManualDuration(\n            Duration::from_secs_f64(1.0 / 60.0),\n        ));\n    let mut loaded = false;\n    for frame in 0..600 {\n        context.begin_frame(egui::RawInput::default());\n        driver.app_mut().update();\n        let _ = context.end_frame();\n        let world = driver.app().world();\n        if frame % 60 == 0 {\n            eprintln!(\n                "gameplay App frame {frame}: session={}, walls={}",\n                world.contains_resource::<ActiveSession>(),\n                world.resource::<InnerWalls>().wall_ids().count()\n            );\n        }\n        if world.contains_resource::<ActiveSession>()\n            && world.resource::<LoadGameManager>().options.is_none()\n            && world.resource::<InnerWalls>().wall_ids().count() > 0\n        {\n            loaded = true;\n            break;\n        }\n    }\n    anyhow::ensure!(\n        loaded,\n        "shipped intro did not finish through the real gameplay schedule"\n    );\n    // Let actual asynchronous assets, constructor uploads and intro migration\n    // settle through their normal schedules before asserting visible owners.\n    for _ in 0..90 {\n        context.begin_frame(egui::RawInput::default());\n        driver.app_mut().update();\n        let _ = context.end_frame();\n    }\n    Ok(BootedSession { driver, context, bridge, shaders, device, _boot: boot_guard })\n}'

REQUIRED = tuple(dict.fromkeys((*SHARED, *GETTERS, BOOT_PATH)))
OBLIGATIONS = (
    'fresh_boot_before_first_gameplay_warmup_checked',
    'known_owners_complete_unknown_owners_refused_checked',
    'chunk_segments_full_tail_bytes_visibility_preserved_checked',
    'escaped_offsets_source_audit_and_retained_arc_weak_checked',
    'once_attempt_preflight_before_mutation_failure_not_retried_checked',
    'readonly_getters_no_default_algorithm_or_hot_path_change_checked',
    'transitive_fixture_dependencies_reviewed_checked',
)


def require(ok, message):
    if not ok:
        raise ValueError(message)


def signature(value):
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(',', ':'),
        ensure_ascii=False, allow_nan=False).encode()).hexdigest()


def bytes_sha(raw):
    return hashlib.sha256(raw).hexdigest()


def _source_contract(source_files, raw_files, shared, harness, schema):
    """Check known input closure and own getter files, never infer a missing version.

    All full bytes are bound to source.json. The original test file may contain
    different unrelated tests; its exact audited boot body must still agree.
    These literal checks are narrow reviewed capability checks, not a Rust parser.
    """
    require(set(raw_files) == set(REQUIRED), 'canonical source archive closure incomplete/extra')
    require(all(source_files.get(name) == expected for name, expected in shared.items()),
            'canonical shared input source bytes differ; explicit new contract required')
    texts = {}
    for name in REQUIRED:
        raw = raw_files[name]
        require(type(raw) is bytes and bytes_sha(raw) == source_files.get(name),
                'canonical archived source differs from frozen manifest: '+name)
        texts[name] = raw.decode('utf-8').replace('\r\n', '\n')
    for name, blocks in GETTERS.items():
        require(all(texts[name].count(part) == 1 for part in blocks),
                'unreviewed/missing canonical readonly getter body: '+name)
    require(texts[BOOT_PATH].count(BOOT) == 1, 'canonical fresh-boot body differs from source review')
    common = {name: shared[name] for name in COMMON4}
    return {'schema': schema, 'harness': harness, 'preparation_schema': PREPARATION,
        'common_source_sha256': common, 'shared_input_closure_sha256': dict(shared),
        'own_getter_source_sha256': {name: source_files[name] for name in GETTERS},
        'readonly_getter_body_sha256': signature(GETTERS),
        'own_boot_source_sha256': source_files[BOOT_PATH], 'boot_body_sha256': bytes_sha(BOOT.encode()),
        'reviewed_source_sha256': {name: source_files[name] for name in REQUIRED}}


def source_contract(source_files, raw_files):
    return _source_contract(source_files, raw_files, SHARED, HARNESS, CONTRACT)


def rich_source_contract(source_files, raw_files):
    """Separate optional-rich diagnosis; never a formal common-v2 capability.

    The frozen rich caller explicitly opts in at runtime. Its raw report still
    must prove that opt-in; source capability by itself does not establish it.
    """
    shared = dict(SHARED)
    shared['crates/tiny-glade/tests/cpu_measurement/mod.rs'] = RICH_CALLER_SHA256
    return _source_contract(source_files, raw_files, shared, None,
                            'rich-optional-canonical-source-diagnostic-v1')


def archive(root, source):
    raw = {name: (Path(root)/name).read_bytes() for name in REQUIRED}
    contract = source_contract(source['files'], raw)
    return {'schema': 'canonical-source-archive-v2', 'source_commit': source['commit'],
        'source_content_sha256': source['content_sha256'], 'contract': contract,
        'files': {name: {'sha256': bytes_sha(value), 'raw_base64': base64.b64encode(value).decode('ascii')}
                  for name, value in raw.items()}}


def validate_archive(value, source):
    require(value.get('schema') == 'canonical-source-archive-v2'
            and value['source_commit'] == source['commit']
            and value['source_content_sha256'] == source['content_sha256'],
            'canonical archive source identity differs')
    require(set(value['files']) == set(REQUIRED), 'canonical source archive closure incomplete/extra')
    raw = {}
    for name, ref in value['files'].items():
        try:
            raw[name] = base64.b64decode(ref['raw_base64'], validate=True)
        except (ValueError, TypeError) as error:
            raise ValueError('invalid canonical source archive encoding') from error
        require(bytes_sha(raw[name]) == ref['sha256'], 'canonical archive bytes changed: '+name)
    contract = source_contract(source['files'], raw)
    require(value['contract'] == contract, 'canonical archive contract differs')
    return contract


def capability_contract(cap, review, source_files):
    files = cap['source_files']
    require(all(name in files for name in REQUIRED), 'canonical reviewed source closure incomplete')
    raw = {name: Path(files[name]['path']).read_bytes() for name in REQUIRED}
    contract = source_contract(source_files, raw)
    require(cap.get('canonical_source_contract') == review.get('canonical_source_contract') == contract,
            'canonical capability/source review contract differs')
    require(all(review.get(name) is True for name in OBLIGATIONS),
            'canonical fresh-boot/owner/reference/transitive source review obligations incomplete')
    return contract


def session_preparation(session):
    require(session.get('harness') == HARNESS and
            session.get('canonical_layout_input_policy') == PREPARATION,
            'missing canonical runtime source policy; historical runs cannot be upgraded')
    return preparation_stamp(session.get('canonical_brick_layout'),
                             session.get('input_before'), session.get('input_after'))


def preparation_stamp(prep, input_before, input_after):
    """Shared raw-stamp check only; caller chooses formal/diagnostic policy."""
    require(isinstance(prep, dict) and prep.get('schema') == PREPARATION
            and prep.get('phase') == 'fresh_boot_before_first_gameplay_warmup',
            'missing canonical preparation stamp')
    for name in ('owner_count', 'chunk_count', 'used_records'):
        require(type(prep.get(name)) is int and 0 < prep[name] <= (1 << 64)-1,
                'invalid canonical preparation count: '+name)
    # Empty live owner allocations are legitimate; owner_count need not be
    # bounded by occupied chunks. The report does not count nonempty owners.
    require(prep['chunk_count'] <= 0x10000 and prep['used_records'] <= 0x200000
            and prep['used_records'] <= prep['chunk_count']*32,
            'invalid canonical typed chunk/record capacities')
    for name in ('owner_record_bytes_blake3', 'owner_full_chunk_bytes_blake3',
                 'physical_layout_blake3', 'free_stack_blake3'):
        require(type(prep.get(name)) is str and re.fullmatch('[0-9a-f]{64}', prep[name]),
                'invalid canonical declared BLAKE3 encoding: '+name)
    require(type(prep.get('raw_index_escape_scope')) is str and prep['raw_index_escape_scope'],
            'canonical escaped-reference scope absent')
    ranges = prep.get('normal_ranges')
    require(type(ranges) is list and ranges, 'canonical prepared draw ranges missing')
    end = 0
    for pair in ranges:
        require(type(pair) is list and len(pair) == 2 and
                all(type(x) is int for x in pair) and pair[0] >= end and pair[1] > 0
                and pair[0]+pair[1] <= 0x200000, 'canonical prepared draw ranges invalid')
        end = pair[0]+pair[1]
    require(sum(pair[1] for pair in ranges) <= prep['used_records'],
            'visible prepared ranges exceed owned used records')
    require(all(isinstance(value, dict) and value.get('canonical_brick_layout') == prep
                for value in (input_before, input_after)),
            'canonical preparation/input endpoint association differs')
    # Only encoding and association are checked here. Python has no BLAKE3;
    # exact raw values are paired by the existing input/workload comparison.
    return prep


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
