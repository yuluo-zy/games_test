"""Pure synthetic proof checks; no Cargo, Job, executable, Vulkan or live source.

Local cold-checker tests begin after the outer file-hash boundary: synthetic DLL
bytes are not the shipped FMOD DLLs. Existing outer-validator tests cover that
boundary; the local policy's pinned FMOD identities are checked independently.
"""
import copy
import json
import os
from pathlib import Path
import shutil
import unittest
from types import SimpleNamespace
from unittest.mock import patch
import freeze_surface_local_target as producer
import surface_frozen_v2 as dispatch
import surface_frozen_local_v2 as local
import test_surface_frozen_v2 as old
import run_surface_pixels as runner
import run_surface_local_pair as local_runner
from compare_surface_pixels import sha256

HERE=Path(__file__).parent

class LocalProofTests(unittest.TestCase):
    def setUp(self):
        self.f=old.SurfaceV2Tests();self.f.setUp();f=self.f
        self.policy=json.loads((HERE/'fixtures/surface-local-adapter-policy.json').read_text())
        p=self.policy
        shutil.copyfile(HERE/'fixtures/surface-local-build-adapter.py',f.folder/'surface-build-adapter.py')
        shutil.copyfile(HERE/'fixtures/surface-local-adapter-policy.json',f.folder/'local-adapter-policy.json')
        shutil.copyfile(f.folder/'source-files.json',f.folder/'source-after.json')
        f.row.update(label='synthetic-local',adapter_sha256=local.ADAPTER_SHA,
            local_adapter_policy_sha256=local.POLICY_SHA,features=p['metadata_features'],
            cargo_artifact_features=p['artifact_features'],command=local.COMMAND,
            fixture_entry=p['fixture_entry'],fixture_mode=p['fixture_mode'],build_timeout_seconds=600,
            controlled_environment=p['fixed_environment'],original_scene={
                'path':str(Path(p['tg_install_root'])/p['asset_scene_relative']),'sha256':p['asset_scene_sha256']})
        f.row['files'].update(p['fmod_sha256'])
        f.life.update(timeout_seconds=600,timed_out=False,deadline_assigned_before_go=True,deadline_errors=[])
        guard=json.loads((f.folder/'source-guard.json').read_text())
        guard.update(schema='surface-local-exact-source-guard-v1',label=f.row['label'],
            fixture_entry=p['fixture_entry'],expected_cargo_command=local.COMMAND,
            expected_artifact_features=p['artifact_features'],local_adapter_policy_sha256=local.POLICY_SHA,
            fixed_environment={**p['fixed_environment'],'CARGO_TARGET_DIR':str(Path(guard['root'])/'target')})
        f.write('source-guard.json',guard);f.row['source_guard_sha256']=sha256(f.folder/'source-guard.json')
        f.artifact['features']=p['artifact_features'];f.write('requested-test-artifact.json',f.artifact);f.log()
        spec=json.loads((f.folder/'build-job-spec.json').read_text());spec['command']=local.COMMAND;f.write('build-job-spec.json',spec)
        (f.folder/'test-list.txt').write_text(p['fixture_entry']+': test\n')
        self.sync()
    def tearDown(self):self.f.tearDown()
    def sync(self):
        self.f.write('cargo-lifecycle.json',self.f.life);self.f.write('build.json',self.f.row)
    def identity(self):return dispatch.validate(self.f.meta,self.f.row,self.f.exe)
    def reject(self):
        self.sync()
        with self.assertRaises((ValueError,FileNotFoundError)):self.identity()
    def test_local_proof_accepts_only_requested_target_and_seals14(self):
        with patch('subprocess.run',side_effect=AssertionError('no live process')):result=self.identity()
        self.assertEqual(len(result['files']),14)
        self.assertFalse(result['entire_cargo_build_passed'])
        self.assertEqual(result['fixture_entry'],producer.ENTRY)
    def test_explicit_local_policy_required(self):
        self.f.row.pop('local_adapter_policy_sha256');self.reject()
    def test_mixed_legacy_adapter_local_policy_rejected(self):
        self.f.row['adapter_sha256']=dispatch.ADAPTER_SHA;self.reject()
    def test_policy_bytes_change_rejected(self):
        (self.f.folder/'local-adapter-policy.json').write_text('{}');self.reject()
    def test_source_after_change_rejected(self):
        (self.f.folder/'source-after.json').write_text('{}');self.reject()
    def test_guard_entry_change_even_with_rehashed_guard_rejected(self):
        g=json.loads((self.f.folder/'source-guard.json').read_text());g['fixture_entry']=runner.TEST
        self.f.write('source-guard.json',g);self.f.row['source_guard_sha256']=sha256(self.f.folder/'source-guard.json');self.reject()
    def test_fixture_entry_change_rejected(self):
        self.f.row['fixture_entry']=runner.TEST;self.reject()
    def test_cpu_probe_missing_rejected(self):
        self.f.row['features']=['default','surface-compare-test'];self.reject()
    def test_late_job_assignment_rejected(self):
        self.f.life['job_assigned_before_cargo_start']=False;self.reject()
    def test_deadline_not_before_go_rejected(self):
        self.f.life['deadline_assigned_before_go']=False;self.reject()
    def test_expired_build_rejected(self):
        self.f.life['timed_out']=True;self.reject()
    def test_whole_cargo_false_cannot_be_upgraded(self):
        self.f.life['entire_cargo_build_passed']=True;self.f.row['entire_cargo_build_passed']=True;self.reject()
    def test_entry_substring_is_not_exact_test_listing(self):
        (self.f.folder/'test-list.txt').write_text('prefix_'+producer.ENTRY+': test\n');self.reject()
    def test_local_schema_cannot_downgrade_to_v1(self):
        for schema in ('surface-frozen-build-v1','headless-frozen-build-v1'):
            self.f.row['schema']=schema;self.sync()
            with self.assertRaisesRegex(ValueError,'local'):runner.frozen_identity(self.f.meta)
    def test_each_local_marker_alone_forbids_legacy_schema(self):
        f=self.f
        markers={'adapter_sha256':local.ADAPTER_SHA,'fixture_entry':producer.ENTRY,
                 'fixture_mode':'same-prefix-local-correctness','local_adapter_policy_sha256':local.POLICY_SHA,
                 'features':['default','surface-compare-test'],'cargo_artifact_features':['cpu-probe','surface-compare-test']}
        for schema in ('surface-frozen-build-v1','headless-frozen-build-v1'):
            for key,value in markers.items():
                row=copy.deepcopy(f.row)
                for name in markers:row.pop(name,None)
                row.update(schema=schema,features=['default','cpu-probe'],cargo_artifact_features=['cpu-probe','default','mimalloc'])
                row[key]=value;f.write('build.json',row)
                with self.subTest(schema=schema,marker=key),self.assertRaisesRegex(ValueError,'local'):runner.frozen_identity(f.meta)
    def test_wrong_known_fmod_identity_rejected(self):
        self.f.row['files']['fmod.dll']='0'*64;self.reject()
    def test_legacy_pair_controller_cannot_launch_local_bundle(self):
        with patch.object(runner,'frozen_identity',return_value={'build_evidence':{'fixture_mode':'same-prefix-local-correctness'}}),patch.object(runner.subprocess,'run',side_effect=AssertionError('no launch')):
            with self.assertRaisesRegex(ValueError,'run_surface_local_pair'):runner.run_pair(self.f.meta,self.f.folder/'new-output')
    def test_local_runner_requires_validated_new_entry_and_policy(self):
        proof=self.identity();local_runner.require_local_build({'build_evidence':proof})
        for field,value in (('fixture_entry',runner.TEST),('local_adapter_policy_sha256','0'*64),('fixture_mode','legacy')):
            wrong=copy.deepcopy(proof);wrong[field]=value
            with self.assertRaises(ValueError):local_runner.require_local_build({'build_evidence':wrong})
        with self.assertRaises(ValueError):local_runner.require_local_build({})

class LocalArtifactTests(unittest.TestCase):
    def setUp(self):
        self.f=old.SurfaceV2Tests();self.f.setUp();f=self.f
        self.root=f.folder/'source';exe=self.root/'target/debug/deps/surface.exe';exe.parent.mkdir(parents=True)
        exe.write_bytes(b'EXE');exe.with_suffix('.pdb').write_bytes(b'PDB')
        self.event=copy.deepcopy(f.artifact);self.event.update(executable=str(exe),
            manifest_path=str(self.root/'crates/tiny-glade/Cargo.toml'),features=['cpu-probe','default','mimalloc','surface-compare-test'])
        self.event['target']['src_path']=str(self.root/'crates/tiny-glade/tests/gpu_surface_collect.rs')
    def tearDown(self):self.f.tearDown()
    def test_exact_local_artifact_accepted(self):self.assertTrue(producer.validate_artifact(self.event,self.root))
    def test_mixed_features_rejected(self):
        for features in (['cpu-probe','default','mimalloc'],['default','surface-compare-test'],
                         ['cpu-probe','default','mimalloc','pprof','surface-compare-test']):
            self.event['features']=features
            with self.assertRaises(ValueError):producer.validate_artifact(self.event,self.root)
    def test_missing_pdb_rejected(self):
        Path(self.event['executable']).with_suffix('.pdb').unlink()
        with self.assertRaises(ValueError):producer.validate_artifact(self.event,self.root)
    def test_wrong_target_is_never_completion(self):
        self.event['target']['name']='tiny-glade-host';self.assertFalse(producer.validate_artifact(self.event,self.root))
    def test_wrong_profile_rejected(self):
        self.event['profile']['opt_level']='3'
        with self.assertRaises(ValueError):producer.validate_artifact(self.event,self.root)
    def test_fixed_policy_self_and_helper_identity(self):
        api=producer.guard_api();p=HERE/'fixtures/surface-local-adapter-policy.json'
        self.assertEqual(producer.read_policy(p,sha256(p),api)['fixture_entry'],producer.ENTRY)
        with self.assertRaises(ValueError):producer.read_policy(p,'0'*64,api)
    def test_codegen_override_rejected(self):
        with patch.dict(os.environ,{'RUSTFLAGS':'-C opt-level=3'},clear=True):
            with self.assertRaisesRegex(ValueError,'codegen'):producer.build_environment(None,{}, {})
    def test_validation_override_rejected(self):
        with patch.dict(os.environ,{'VK_LOADER_LAYERS_DISABLE':'*'},clear=True):
            with self.assertRaisesRegex(ValueError,'Vulkan'):producer.build_environment(None,{}, {})
    def mock_build(self,expire=False):
        dest=self.f.folder/'build';dest.mkdir();steps=[]
        class Job:
            def assign(inner,process):
                self.assertFalse((dest/'build-job-assigned.go').exists());steps.append('assign')
            def terminate(inner):steps.append('terminate')
            def wait_empty(inner):steps.append('empty')
            def close(inner):steps.append('close')
        class Timer:
            def __init__(inner,timeout,callback):inner.callback=callback
            def start(inner):
                self.assertEqual(steps,['assign']);self.assertFalse((dest/'build-job-assigned.go').exists());steps.append('timer')
                if expire:inner.callback()
            def cancel(inner):steps.append('cancel')
            def join(inner):steps.append('join')
        process=SimpleNamespace(pid=111,stdout=[json.dumps({'reason':'surface-owned-cargo-start','pid':222}),json.dumps(self.event)],
            wait=lambda timeout:local.TERMINATED,poll=lambda:local.TERMINATED)
        module=SimpleNamespace(Job=Job,TERMINATED_AFTER_TARGET=local.TERMINATED)
        with patch.object(producer.subprocess,'Popen',return_value=process),patch.object(producer.threading,'Timer',Timer):
            if expire:
                with self.assertRaises(TimeoutError):producer.build_target(self.root,{},dest,module,600)
            else:
                _,life=producer.build_target(self.root,{},dest,module,600)
                self.assertTrue(life['requested_target_completed']);self.assertFalse(life['entire_cargo_build_passed'])
        life=json.loads((dest/'cargo-lifecycle.json').read_text())
        self.assertEqual(life['timed_out'],expire);self.assertLess(steps.index('assign'),steps.index('timer'))
        self.assertLess(steps.index('join'),steps.index('close'))
        return life
    def test_private_job_assigned_then_deadline_then_go_and_target_stop(self):self.mock_build()
    def test_deadline_cannot_be_accepted_even_if_target_arrives(self):self.mock_build(expire=True)

if __name__=='__main__':unittest.main()
