"""Synthetic file/metadata tests only. Never runs an EXE, Cargo or a GPU."""
import copy,json,os,pathlib,subprocess,tempfile,unittest,unittest.mock as mock
import perf_import_frozen_bundle as m
ROOT=pathlib.Path(__file__).resolve().parents[1]
QUALIFIED_TOOLS=ROOT/'tools'
COMMON_FIXTURES=ROOT/'tools/fixtures/common_cpu_harness'
TEMP_ROOT=ROOT/'.perf/importer-unit-tests'
class ImportTests(unittest.TestCase):
    def setUp(self):
        TEMP_ROOT.mkdir(parents=True,exist_ok=True)
        self.temp=tempfile.TemporaryDirectory(prefix='synthetic-import-',dir=TEMP_ROOT)
        self.area=pathlib.Path(self.temp.name);self.root=self.area/'source';self.host=self.area/'host';self.root.mkdir();self.host.mkdir()
        for root in [self.root,self.host]:
            (root/'tools').mkdir()
            for name in m.TOOLS:(root/'tools'/name).write_bytes((QUALIFIED_TOOLS/name).read_bytes())
        (self.root/'.gitignore').write_text('/.perf/\n/target/\n')
        self.write_source('Cargo.lock','unchanged dependency identity')
        self.write_source('crates/tiny-glade/Cargo.toml','[package]\nname="tiny-glade"\nversion="0.1.0"\n')
        self.write_source('crates/tiny-glade/src/sample.rs','//! Synthetic native probe source; never compiled.\n')
        for name in m.COMMON:
            dst=self.root/'crates/tiny-glade/tests/cpu_measurement'/name;dst.parent.mkdir(parents=True,exist_ok=True)
            dst.write_bytes((COMMON_FIXTURES/name).read_bytes())
        for args in [('init','-q'),('config','core.autocrlf','false'),('config','user.name','Synthetic Test'),('config','user.email','synthetic@example.invalid'),('add','.'),('commit','-qm','synthetic source only')]:
            subprocess.run(['git',*args],cwd=self.root,check=True,capture_output=True,env=m.isolated_git_env())
        self.source=m.source_snapshot(self.root);self.label='synthetic-probe';self.bundle=self.root/'.perf/headless-builds'/self.label;self.bundle.mkdir(parents=True)
        self.exe='original_gameplay_chain-0123456789abcdef.exe';self.pdb=self.exe[:-4]+'.pdb'
        for name in [self.exe,self.pdb,'fmod.dll','fmodstudio.dll']:(self.bundle/name).write_bytes(('synthetic, not runnable '+name).encode())
        (self.bundle/'vulkan-summary.txt').write_text('synthetic enumeration; not hardware evidence')
        hardware={'vulkan_summary_sha256':m.digest(self.bundle/'vulkan-summary.txt')};self.dump('hardware.json',hardware)
        self.dump('source.json',self.source)
        lifecycle={'requested_target_completed':True,'active_build_processes_remaining':False,'cargo_exit_code':None,
            'entire_cargo_build_passed':False,'termination_was_intentional_after_requested_artifact':True,
            'job_termination_exit_code':0xE000C0DE,'job_assigned_before_cargo_start':True,'wrapper_pid':101,'cargo_pid':102}
        self.dump('cargo-lifecycle.json',lifecycle)
        artifact={'reason':'compiler-artifact','target':{'name':m.TARGET,'kind':['test']},'features':['cpu-probe','default','mimalloc'],
            'profile':{'test':True,'opt_level':'3'},'manifest_path':str(self.root/'crates/tiny-glade/Cargo.toml'),
            'executable':str(self.root/'target/release/deps'/self.exe)}
        self.dump('requested-target-artifact.json',artifact)
        start={'reason':'perf-owned-cargo-start','pid':102,'wrapper_pid':101}
        (self.bundle/'build.jsonl').write_text(json.dumps(start)+'\n'+json.dumps(artifact)+'\n')
        self.dump('build-job-spec.json',{'cwd':str(self.root),'command':m.COMMAND,'gate':str(self.bundle/'build-job-assigned.go')})
        (self.bundle/'build-job-assigned.go').write_text('assigned');(self.bundle/'build.stderr.log').write_text('synthetic log')
        (self.bundle/'test-list.txt').write_text('cpu_measurement::profile_shipped_session_cpu: test\n')
        self.build={'schema':'headless-frozen-build-v1','label':self.label,'source_commit':self.source['commit'],
            'source_content_sha256':self.source['content_sha256'],'profile':'release','dirty':False,'features':['default(mimalloc)','cpu-probe'],
            'cpu_probe_feature':True,'allocator':'mimalloc','jobs':1,'incremental':False,'command':m.COMMAND,'build_environment':{},
            'build_environment_sha256':m.signature({}),'management_controls':m.caps(self.root,self.source),
            'executable':str(self.bundle/self.exe),'files':{name:m.digest(self.bundle/name) for name in [self.exe,self.pdb,'fmod.dll','fmodstudio.dll']},
            'executable_sha256':m.digest(self.bundle/self.exe),'requested_test_target_completed':True,'cargo_lifecycle':lifecycle,
            'entire_cargo_build_passed':False,'requested_target_evidence_sha256':m.digest(self.bundle/'requested-target-artifact.json'),
            'build_jsonl_sha256':m.digest(self.bundle/'build.jsonl'),'build_helper_sha256':m.TOOLS['perf_build_target.py'],'hardware':hardware}
        self.dump('build.json',self.build)
    def tearDown(self):
        resolved_temp_root=TEMP_ROOT.resolve(strict=True)
        self.assertTrue(self.area.resolve(strict=True).is_relative_to(resolved_temp_root))
        self.assertEqual(self.area.resolve(strict=True).parent,resolved_temp_root)
        self.temp.cleanup()
    def write_source(self,name,text):
        p=self.root/name;p.parent.mkdir(parents=True,exist_ok=True);p.write_text(text)
    def dump(self,name,value):
        (self.bundle/name).write_text(json.dumps(value,indent=2,ensure_ascii=False)+'\n')
    def plan(self):
        return m.prepare_plan(self.bundle,self.root,self.host,self.source['commit'],self.source['content_sha256'],m.digest(self.bundle/'build.json'))
    def reject(self,pattern):
        with self.assertRaisesRegex(ValueError,pattern):self.plan()
    def test_success_preserves_all_origin_bytes_and_false_whole_build(self):
        before=m.inventory(self.bundle);plan=self.plan();result=m.execute_plan(plan);dest=pathlib.Path(plan['destination'])
        self.assertEqual(m.inventory(self.bundle),before);self.assertFalse(result['entire_cargo_build_passed_unchanged'])
        derived=m.read(dest/'build.json');expected=dict(self.build);expected['executable']=str(dest/self.exe)
        self.assertEqual(derived,expected);self.assertEqual((dest/'original-build.json').read_bytes(),(self.bundle/'build.json').read_bytes())
        self.assertEqual(m.verify_import(dest),result)
    def test_plan_is_read_only_and_new_nonce(self):
        first=self.plan();second=self.plan();self.assertNotEqual(first['import_nonce'],second['import_nonce']);self.assertFalse(pathlib.Path(first['destination']).exists())
    def test_git_environment_cannot_redirect_source_or_inject_config(self):
        poison={'GIT_DIR':str(self.host/'.git'),'GIT_WORK_TREE':str(self.host),'GIT_INDEX_FILE':str(self.host/'fake-index'),
            'GIT_CONFIG_COUNT':'1','GIT_CONFIG_KEY_0':'core.worktree','GIT_CONFIG_VALUE_0':str(self.host),
            'GIT_CONFIG_GLOBAL':str(self.host/'foreign-config'),'GIT_ALTERNATE_OBJECT_DIRECTORIES':str(self.host)}
        with mock.patch.dict(os.environ,poison):
            self.assertEqual(m.source_snapshot(self.root),self.source)
            self.assertEqual(pathlib.Path(m.git(self.root,'rev-parse','--absolute-git-dir').decode().strip()),self.root/'.git')
            self.plan()
    def test_existing_destination_rejected(self):
        p=self.plan();pathlib.Path(p['destination']).mkdir(parents=True);self.reject('host label exists')
    def test_dirty_source_rejected(self):
        (self.root/'Cargo.lock').write_text('changed');self.reject('current original source')
    def test_wrong_original_hash_rejected(self):
        with self.assertRaisesRegex(ValueError,'original build SHA'):
            m.prepare_plan(self.bundle,self.root,self.host,self.source['commit'],self.source['content_sha256'],'0'*64)
    def test_unknown_dll_and_case_collision_rejected(self):
        (self.bundle/'foreign.dll').write_bytes(b'foreign');self.reject('unknown/missing')
        with mock.patch.object(m.Path,'iterdir',return_value=iter([self.bundle/'build.json',self.bundle/'BUILD.JSON'])):
            with self.assertRaisesRegex(ValueError,'case-folded'):m.inventory(self.bundle)
    def test_missing_pdb_rejected(self):
        (self.bundle/self.pdb).unlink();self.reject('unknown/missing')
    def test_payload_mutation_rejected(self):
        (self.bundle/self.exe).write_bytes(b'different');self.reject('payload SHA')
    def test_incomplete_target_or_active_job_rejected(self):
        self.build['requested_test_target_completed']=False;self.dump('build.json',self.build);self.reject('target incomplete')
        self.build['requested_test_target_completed']=True;self.build['cargo_lifecycle']['active_build_processes_remaining']=True
        self.dump('cargo-lifecycle.json',self.build['cargo_lifecycle']);self.dump('build.json',self.build);self.reject('target incomplete')
    def test_whole_build_cannot_be_upgraded(self):
        self.build['entire_cargo_build_passed']=True;self.dump('build.json',self.build);self.reject('whole Cargo status')
    def test_command_or_job_cwd_rejected(self):
        spec=m.read(self.bundle/'build-job-spec.json');spec['cwd']=str(self.host);self.dump('build-job-spec.json',spec);self.reject('Job cwd')
    def test_current_caps_cannot_be_invented(self):
        self.build['management_controls']['default_modes']={k:True for k in m.FLAGS};self.dump('build.json',self.build);self.reject('current capability')
    def test_debug_and_dirty_build_rejected(self):
        for key,value in [('profile','debug'),('dirty',True)]:
            bad=dict(self.build);bad[key]=value;self.dump('build.json',bad);self.reject('clean common release')
    def test_artifact_or_log_corruption_rejected(self):
        (self.bundle/'build.jsonl').write_text('changed');self.reject('evidence SHA')
    def test_symlink_or_reparse_rejected_without_creating_links(self):
        with mock.patch.object(m.Path,'is_junction',return_value=True):
            with self.assertRaisesRegex(ValueError,'junction'):m.inventory(self.bundle)
    def test_source_or_plan_changed_after_plan_rejected(self):
        p=self.plan();(self.bundle/self.exe).write_bytes(b'changed')
        with self.assertRaisesRegex(ValueError,'payload SHA'):m.execute_plan(p)
    def test_destination_escape_or_relabel_rejected(self):
        p=self.plan();p['destination']=str(self.host/'elsewhere')
        with self.assertRaisesRegex(ValueError,'destination label'):m.execute_plan(p)
    def test_relative_and_traversal_roots_rejected(self):
        with self.assertRaisesRegex(ValueError,'without traversal'):m.absolute(self.host/'..'/'host')
        with self.assertRaisesRegex(ValueError,'without traversal'):m.absolute(pathlib.Path('relative'))
    def test_non_v4_nonce_rejected(self):
        p=self.plan();p['import_nonce']='00000000-0000-0000-0000-000000000000'
        with self.assertRaisesRegex(ValueError,'invalid nonce'):m.execute_plan(p)
    def test_importer_identity_changed_rejected(self):
        p=self.plan();p['importer_sha256']='0'*64
        with self.assertRaisesRegex(ValueError,'importer identity'):m.execute_plan(p)
    def test_post_import_origin_or_derived_mutation_rejected(self):
        p=self.plan();m.execute_plan(p);dest=pathlib.Path(p['destination']);derived=m.read(dest/'build.json');derived['jobs']=2
        (dest/'build.json').write_text(json.dumps(derived))
        with self.assertRaisesRegex(ValueError,'more than executable'):m.verify_import(dest)
    def test_post_import_missing_original_evidence_rejected(self):
        p=self.plan();m.execute_plan(p);dest=pathlib.Path(p['destination']);(dest/'original-build.json').write_bytes(b'changed')
        with self.assertRaisesRegex(ValueError,'original build archive'):m.verify_import(dest)
    def test_origin_changed_after_import_rejected(self):
        p=self.plan();m.execute_plan(p);(self.bundle/'build.stderr.log').write_text('new log')
        with self.assertRaisesRegex(ValueError,'after import'):m.verify_import(p['destination'])
    def test_manifest_status_path_or_source_cannot_lie(self):
        p=self.plan();m.execute_plan(p);dest=pathlib.Path(p['destination']);path=dest/'frozen-bundle-import.json';original=m.read(path)
        for key,value in [('original_bundle',str(self.host)),('source_commit_unchanged','0'*40),('entire_cargo_build_passed_unchanged',True),('original_build_nonce','invented')]:
            bad=dict(original);bad[key]=value;path.write_text(json.dumps(bad))
            with self.assertRaises(ValueError):m.verify_import(dest)
    def test_failed_copy_never_publishes_label(self):
        p=self.plan()
        with mock.patch.object(m.shutil,'copy2',side_effect=OSError('injected write failure')):
            with self.assertRaisesRegex(OSError,'injected'):m.execute_plan(p)
        self.assertFalse(pathlib.Path(p['destination']).exists())
        self.assertEqual(len(list((self.host/'.perf/headless-builds').glob('.import-*'))),1)
if __name__=='__main__':unittest.main()
