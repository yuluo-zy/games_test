"""Synthetic import/series binding tests. No Cargo, GPU, EXE or real ETW replay."""
import copy
import importlib.util
import json
from pathlib import Path
import subprocess
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch
import perf_headless_series as series
import perf_headless_workload as work
import test_perf_headless_series_controller as controller
save = controller.save
from test_perf_headless_workload import actual_fixture

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('_import_fixture', HERE/'fixtures/frozen_bundle_importer.py')
imp = importlib.util.module_from_spec(spec); spec.loader.exec_module(imp)

class ImportBindingTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(prefix='series-import-synthetic-')
        self.addCleanup(self.tmp.cleanup)
        self.area = Path(self.tmp.name); self.host = self.area/'host'; self.host.mkdir()
        self.origins = {}
        for name in imp.TOOLS:
            dst = self.host/'tools'/name; dst.parent.mkdir(parents=True, exist_ok=True)
            dst.write_bytes((HERE/name).read_bytes())
        self.a = self.make_import('baseline')
        self.b = self.make_import('candidate')
        self.cal = self.area/'calibration.json'
        save(self.cal, {'schema':'windows-clock-policy-evidence-v1', 'policy_id':series.POLICY,
            'definition_sha256':'f'*64, 'tools':series.module_identity(self.host/'tools'),
            'hardware':{'os':'synthetic', 'physical_cpu':'synthetic', 'logical_cpus':1}})

    def make_import(self, label):
        root = self.area/('origin-'+label); root.mkdir(); self.origins[label] = root
        def put(name, data):
            dst = root/name; dst.parent.mkdir(parents=True, exist_ok=True); dst.write_bytes(data)
        put('.gitignore', b'/.perf/\n/target/\n')
        put('Cargo.lock', b'unchanged lock')
        put('crates/tiny-glade/Cargo.toml', b'[package]\nname="tiny-glade"\nversion="0.1.0"\n')
        put('crates/tiny-glade/src/synthetic.rs', ('fn recorded() {} // '+label).encode())
        for name in imp.TOOLS: put('tools/'+name, (HERE/name).read_bytes())
        for name in imp.COMMON:
            put('crates/tiny-glade/tests/cpu_measurement/'+name,
                (HERE/'fixtures/common_cpu_harness'/name).read_bytes())
        for args in [('init','-q'),('config','core.autocrlf','false'),('config','user.name','Synthetic'),
                     ('config','user.email','synthetic@example.invalid'),('add','.'),('commit','-qm',label)]:
            subprocess.run(['git',*args],cwd=root,env=imp.isolated_git_env(),check=True,capture_output=True)
        source = imp.source_snapshot(root)
        folder = root/'.perf/headless-builds'/label; folder.mkdir(parents=True)
        exe = 'original_gameplay_chain-0123456789abcdef.exe'; pdb = exe[:-4]+'.pdb'
        for name in (exe,pdb,'fmod.dll','fmodstudio.dll'):
            (folder/name).write_bytes((label if name.endswith(('.exe','.pdb')) else 'shared').encode())
        (folder/'vulkan-summary.txt').write_bytes(b'synthetic Vulkan, not hardware evidence')
        hardware = {'vulkan_summary_sha256':imp.digest(folder/'vulkan-summary.txt')}
        life = {'requested_target_completed':True,'active_build_processes_remaining':False,
            'cargo_exit_code':None,'entire_cargo_build_passed':False,
            'termination_was_intentional_after_requested_artifact':True,
            'job_termination_exit_code':0xE000C0DE,'job_assigned_before_cargo_start':True,
            'wrapper_pid':101,'cargo_pid':102}
        artifact = {'reason':'compiler-artifact','target':{'name':imp.TARGET,'kind':['test']},
            'features':['cpu-probe','default','mimalloc'],'profile':{'test':True,'opt_level':'3'},
            'manifest_path':str(root/'crates/tiny-glade/Cargo.toml'),
            'executable':str(root/'target/release/deps'/exe)}
        for name,value in [('source.json',source),('hardware.json',hardware),('cargo-lifecycle.json',life),
                           ('requested-target-artifact.json',artifact),('build-job-spec.json',
                            {'cwd':str(root),'command':imp.COMMAND,'gate':str(folder/'build-job-assigned.go')})]:
            save(folder/name,value)
        start = {'reason':'perf-owned-cargo-start','pid':102,'wrapper_pid':101}
        (folder/'build.jsonl').write_text(json.dumps(start)+'\n'+json.dumps(artifact)+'\n')
        (folder/'build.stderr.log').write_text('synthetic log')
        (folder/'build-job-assigned.go').write_text('assigned')
        (folder/'test-list.txt').write_text('cpu_measurement::profile_shipped_session_cpu: test\n')
        build = {'schema':'headless-frozen-build-v1','label':label,'source_commit':source['commit'],
            'source_content_sha256':source['content_sha256'],'profile':'release','dirty':False,
            'features':['default(mimalloc)','cpu-probe'],'cpu_probe_feature':True,'allocator':'mimalloc',
            'jobs':1,'incremental':False,'command':imp.COMMAND,'build_environment':{},
            'build_environment_sha256':imp.signature({}),'management_controls':imp.caps(root,source),
            'executable':str(folder/exe),'files':{name:imp.digest(folder/name) for name in (exe,pdb,'fmod.dll','fmodstudio.dll')},
            'executable_sha256':imp.digest(folder/exe),'requested_test_target_completed':True,
            'cargo_lifecycle':life,'entire_cargo_build_passed':False,
            'requested_target_evidence_sha256':imp.digest(folder/'requested-target-artifact.json'),
            'build_jsonl_sha256':imp.digest(folder/'build.jsonl'),'build_helper_sha256':imp.TOOLS['perf_build_target.py'],
            'hardware':hardware,'cargo_lock_sha256':imp.digest(root/'Cargo.lock'),'rustc':'synthetic-rustc'}
        save(folder/'build.json',build)
        plan = imp.prepare_plan(folder,root,self.host,source['commit'],source['content_sha256'],imp.digest(folder/'build.json'))
        imp.execute_plan(plan)
        dest = Path(plan['destination'])
        archive = self.host/'.perf/producer-reviews'/label; archive.mkdir(parents=True)
        rels = (*work.COMMON_HARNESS_FILES,'crates/tiny-glade/src/synthetic.rs')
        refs = {}
        for i,rel in enumerate(rels):
            path = archive/(str(i)+'.rs'); path.write_bytes((root/rel).read_bytes()); refs[rel]=series.reference(path)
        _,_,model = actual_fixture()
        fields = {key:{'meaning':meaning,'symbols':['recorded'], 'source_paths':[rels[-1]]}
                  for key,meaning in work.FIELD_GROUPS.items()}
        review = {'schema':'headless-producer-source-review-v1','source_commit':source['commit'],
            'source_content_sha256':source['content_sha256'],'source_sha256':{name:ref['sha256'] for name,ref in refs.items()},
            'fields':fields,'coverage':model['coverage'],'findings':[],'algorithm_selection':work.ALGORITHM_SELECTION,
            'runtime_ablation_supported':False,'controller_disable_controls_absent_checked':True,
            'receipt_attribution_checked':True,'successful_prefix_failure_invalid_checked':True,'counter_overflow_checked':True}
        save(archive/'review.json',review)
        cap = {'schema':'headless-producer-capability-v1','raw_schema':work.SCHEMA,'source_commit':source['commit'],
            'source_content_sha256':source['content_sha256'],'source_files':refs,'algorithm_selection':work.ALGORITHM_SELECTION,
            'review_report':series.reference(archive/'review.json'),'fields':fields,'coverage':model['coverage']}
        save(archive/'capability.json',cap)
        return dest/'build.json'

    def plan(self):
        path = self.area/'series.json'
        value = series.make_plan('synthetic-import',self.a,self.b,self.cal,self.host/'tools',path,
            baseline_producer_review=self.host/'.perf/producer-reviews/baseline/capability.json',
            candidate_producer_review=self.host/'.perf/producer-reviews/candidate/capability.json',
            baseline_imported=True, candidate_imported=True)
        return value,path

    def test_plan_seals_every_original_and_derived_file(self):
        plan,_ = self.plan(); series.validate_plan(plan)
        binding = plan['import_bindings']['A']
        self.assertEqual(set(binding['artifacts']),set(imp.inventory(self.a.parent)))
        self.assertTrue(set(series.IMPORT_FILES)|{'build.json','source.json'} <= set(binding['artifacts']))
        self.assertEqual(binding['derived_build_sha256'],series.digest(self.a))
        self.assertFalse(plan['build_records']['A']['entire_cargo_build_passed'])
        self.assertIsNone(plan['build_records']['A']['management_controls']['default_modes'])

    def test_later_check_is_cold_and_does_not_require_live_source(self):
        plan,_ = self.plan()
        for root in self.origins.values(): root.rename(root.with_name(root.name+'-removed'))
        with patch('subprocess.Popen',side_effect=AssertionError('sealed validation must not run Git')):
            series.validate_plan(plan)

    def test_dirty_origin_before_declaration_rejected(self):
        (self.origins['baseline']/'Cargo.lock').write_text('changed')
        with self.assertRaisesRegex(ValueError,'current original source'): self.plan()

    def test_missing_one_import_artifact_rejected(self):
        (self.a.parent/'original-inventory.json').unlink()
        with self.assertRaisesRegex(ValueError,'incomplete imported'): self.plan()

    def test_deleting_all_proofs_cannot_masquerade_as_native(self):
        for name in series.IMPORT_FILES: (self.a.parent/name).unlink()
        with self.assertRaisesRegex(ValueError,'missing imported proof'): self.plan()

    def test_explicit_import_requirement_survives_wholesale_origin_proof_removal(self):
        for name in (*series.IMPORT_FILES, 'requested-target-artifact.json'):
            (self.a.parent/name).unlink()
        with self.assertRaisesRegex(ValueError,'explicit imported build'): self.plan()

    def test_required_import_cannot_be_dropped_from_declared_side(self):
        plan,_ = self.plan(); plan['import_bindings']['A'] = None
        with self.assertRaisesRegex(ValueError,'required imported proof'): series.validate_plan(plan)

    def test_post_plan_any_original_log_tamper_rejected(self):
        plan,_ = self.plan(); (self.a.parent/'build.stderr.log').write_text('changed')
        with self.assertRaisesRegex(ValueError,'original bytes differ'): series.validate_plan(plan)

    def test_post_plan_derived_nonpath_edit_rejected(self):
        plan,_ = self.plan(); derived=series.read(self.a); derived['entire_cargo_build_passed']=True; save(self.a,derived)
        with self.assertRaises(ValueError): series.validate_plan(plan)

    def test_post_plan_import_manifest_tamper_rejected(self):
        plan,_ = self.plan(); p=self.a.parent/'frozen-bundle-import.json'; value=series.read(p)
        value['source_commit_unchanged']='0'*40; save(p,value)
        with self.assertRaisesRegex(ValueError,'manifest source identity'): series.validate_plan(plan)

    def test_unqualified_archived_importer_not_executed(self):
        (self.a.parent/'bundle-importer.py').write_text('raise AssertionError("untrusted code executed")')
        with self.assertRaisesRegex(ValueError,'unqualified frozen'): self.plan()

    def test_declared_original_hash_cannot_be_changed(self):
        plan,_ = self.plan(); plan['import_bindings']['A']['original_build_sha256']='0'*64
        with self.assertRaisesRegex(ValueError,'identity changed'): series.validate_plan(plan)

    def test_commit_bytes_must_match_git_oid(self):
        plan,_ = self.plan(); plan['import_bindings']['A']['frozen_source_commit']['raw_base64']='ZmFrZQ=='
        with self.assertRaisesRegex(ValueError,'Git commit object'): series.validate_plan(plan)

    def test_missing_import_contract_is_not_upgraded(self):
        plan,_ = self.plan(); del plan['import_proof_contract']
        with self.assertRaisesRegex(ValueError,'missing versioned'): series.validate_plan(plan)

    def test_audit_seal_contains_import_refs_and_rejects_tamper(self):
        plan,path = self.plan(); entry=plan['entries'][0]
        capture=controller.ControllerTests(); _,_,cpu=capture.capture(plan,entry)
        for root in self.origins.values(): root.rename(root.with_name(root.name+'-removed'))
        with patch('subprocess.Popen',side_effect=AssertionError('cold audit must not run Git')), \
             patch.object(series,'account_module',return_value=SimpleNamespace(account=lambda *a,**k:cpu)):
            seal=series.audit_run(path,1)
        self.assertIn('frozen_bundle_import',seal['artifacts'])
        series.verify_seal(plan,entry)
        (self.a.parent/'original-build.json').write_text('changed')
        with self.assertRaises(ValueError): series.verify_seal(plan,entry)

    def test_evaluate_rejects_import_tamper_before_any_trace_replay(self):
        plan,path = self.plan(); (self.a.parent/'build.stderr.log').write_text('changed')
        with patch.object(series,'account_module',side_effect=AssertionError('must reject before replay')):
            with self.assertRaisesRegex(ValueError,'original bytes differ'): series.evaluate(path,replay=True)

    def test_native_contract_still_has_none_bindings(self):
        native=controller.ControllerTests(); native.setUp()
        try:
            plan,_=native.plan(); self.assertEqual(plan['import_bindings'],{'A':None,'B':None})
        finally: native.doCleanups()

if __name__=='__main__': unittest.main()
