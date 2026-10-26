"""Synthetic frozen files only; no EXE, Cargo, Vulkan or live checkout calls."""
import copy
import json
from pathlib import Path
import shutil
import tempfile
import unittest
from unittest.mock import patch
import run_surface_pixels as runner
import surface_frozen_v2 as v2
from compare_surface_pixels import sha256

class SurfaceV2Tests(unittest.TestCase):
    def setUp(self):
        self.tmp=tempfile.TemporaryDirectory();self.folder=Path(self.tmp.name)
        self.meta=self.folder/'build.json';self.exe=self.folder/'gpu_surface_collect-fixture.exe'
        root=self.folder/'nonexistent-original-source'
        self.files={}
        for name in (self.exe.name,self.exe.with_suffix('.pdb').name,'fmod.dll','fmodstudio.dll'):
            (self.folder/name).write_bytes(name.encode());self.files[name]=sha256(self.folder/name)
        # Review/test package ships the pinned producer as a fixture, no staging dependency.
        adapter=Path(__file__).with_name('fixtures')/'surface-build-adapter.py'
        shutil.copyfile(adapter,self.folder/'surface-build-adapter.py')
        sourcefiles={'Cargo.lock':'a'*64,'tools/perf_build_target.py':v2.JOB_SHA}
        source={'commit':'c'*40,'status':'','files':sourcefiles,'content_sha256':v2.signature(sourcefiles)}
        guard={'root':str(root),'expected_source_identity':source,'rustc_verbose':'synthetic rustc',
            'qualified_tools_sha256':{'perf_build_target.py':v2.JOB_SHA}}
        self.write('source-files.json',source);self.write('source-guard.json',guard)
        self.life={'strategy':'surface requested test only / qualified private Job','requested_target_completed':True,
            'job_assigned_before_cargo_start':True,'active_build_processes_remaining':False,'remaining_targets_are_not_accepted':True,
            'termination_was_intentional_after_requested_artifact':True,'job_termination_exit_code':v2.TERMINATED,
            'wrapper_exit_code':v2.TERMINATED,'cargo_exit_code':None,'entire_cargo_build_passed':False,
            'wrapper_pid':111,'cargo_pid':222}
        self.write('cargo-lifecycle.json',self.life)
        self.write('build-job-spec.json',{'command':v2.COMMAND,'cwd':str(root),'gate':str(self.folder/'build-job-assigned.go')})
        (self.folder/'build-job-assigned.go').write_text('assigned',encoding='utf-8')
        self.artifact={'reason':'compiler-artifact','target':{'name':'gpu_surface_collect','kind':['test'],
            'src_path':str(root/'crates/tiny-glade/tests/gpu_surface_collect.rs')},
            'manifest_path':str(root/'crates/tiny-glade/Cargo.toml'),
            'profile':{'test':True,'opt_level':'0'},'features':['cpu-probe','default','mimalloc'],
            'executable':str(root/'target/debug/deps'/self.exe.name)}
        self.write('requested-test-artifact.json',self.artifact);self.log()
        (self.folder/'build.stderr.log').write_text('synthetic warnings',encoding='utf-8')
        (self.folder/'test-list.txt').write_text(runner.TEST+': test\n',encoding='utf-8')
        (self.folder/'vulkan-summary.txt').write_text('synthetic Vulkan',encoding='utf-8')
        hardware={'vulkan_summary_sha256':sha256(self.folder/'vulkan-summary.txt')};self.write('hardware.json',hardware)
        self.row={'schema':'surface-frozen-build-v2','executable':str(self.exe),'executable_sha256':self.files[self.exe.name],
            'files':self.files,'source_commit':source['commit'],'source_content_sha256':source['content_sha256'],
            'cargo_lock_sha256':sourcefiles['Cargo.lock'],'adapter_sha256':v2.ADAPTER_SHA,'qualified_job_helper_sha256':v2.JOB_SHA,
            'source_guard_sha256':sha256(self.folder/'source-guard.json'),'rustc':guard['rustc_verbose'],
            'profile':'debug','features':['default','cpu-probe'],'cargo_artifact_features':['cpu-probe','default','mimalloc'],
            'allocator':'System (correctness fixture; not a CPU benchmark)','jobs':1,'incremental':False,
            'command':v2.COMMAND,'requested_test_target_completed':True,'cargo_lifecycle':self.life,
            'entire_cargo_build_passed':False,'hardware':hardware}
        self.write('build.json',self.row)
    def tearDown(self):self.tmp.cleanup()
    def write(self,name,value):(self.folder/name).write_text(json.dumps(value,indent=2)+'\n',encoding='utf-8')
    def log(self):
        start={'reason':'surface-owned-cargo-start','pid':222,'wrapper_pid':111}
        (self.folder/'build.jsonl').write_text(json.dumps(start)+'\n'+json.dumps(self.artifact)+'\n',encoding='utf-8')
    def identity(self):return runner.frozen_identity(self.meta)
    def reject(self):
        with self.assertRaises((ValueError,FileNotFoundError)):self.identity()
    def test_accepts_cold_target_success_with_whole_cargo_false(self):
        with patch('subprocess.run',side_effect=AssertionError('no live process permitted')):
            identity=self.identity()
        self.assertFalse(identity['build_evidence']['entire_cargo_build_passed'])
        self.assertEqual(len(identity['build_evidence']['files']),12)
        self.assertEqual(identity['build_evidence']['validator_sha256'],sha256(Path(v2.__file__)))
    def test_rejects_wrong_target_coherent_artifact_and_log(self):
        self.artifact['target']['name']='original_gameplay_chain';self.write('requested-test-artifact.json',self.artifact);self.log();self.reject()
    def test_rejects_wrong_artifact_source(self):
        self.artifact['target']['src_path']='C:/different.rs';self.write('requested-test-artifact.json',self.artifact);self.log();self.reject()
    def test_rejects_guard_hash_change(self):
        (self.folder/'source-guard.json').write_text('{}');self.reject()
    def test_rejects_source_manifest_change(self):
        source=json.loads((self.folder/'source-files.json').read_text());source['status']=' M actual';self.write('source-files.json',source);self.reject()
    def test_rejects_source_inventory_selfhash_mismatch(self):
        guard=json.loads((self.folder/'source-guard.json').read_text());guard['expected_source_identity']['files']['Cargo.lock']='b'*64
        self.write('source-files.json',guard['expected_source_identity']);self.write('source-guard.json',guard)
        self.row['source_guard_sha256']=sha256(self.folder/'source-guard.json');self.write('build.json',self.row);self.reject()
    def test_rejects_no_private_job_assignment(self):
        self.life['job_assigned_before_cargo_start']=False;self.write('cargo-lifecycle.json',self.life);self.write('build.json',self.row);self.reject()
    def test_rejects_job_or_active_process_mismatch(self):
        self.life['active_build_processes_remaining']=True;self.write('cargo-lifecycle.json',self.life);self.write('build.json',self.row);self.reject()
    def test_rejects_pdb_mutation(self):
        self.exe.with_suffix('.pdb').write_bytes(b'changed');self.reject()
    def test_rejects_pdb_omitted(self):
        self.row['files'].pop(self.exe.with_suffix('.pdb').name);self.write('build.json',self.row);self.reject()
    def test_rejects_false_upgraded_to_whole_cargo_success(self):
        self.life['entire_cargo_build_passed']=True;self.row['entire_cargo_build_passed']=True
        self.write('cargo-lifecycle.json',self.life);self.write('build.json',self.row);self.reject()
    def test_rejects_log_compiler_error(self):
        path=self.folder/'build.jsonl';lines=path.read_text().splitlines();lines.insert(1,json.dumps({'reason':'compiler-message','message':{'level':'error'}}))
        path.write_text('\n'.join(lines)+'\n');self.reject()
    def test_rejects_wrong_start_pid(self):
        path=self.folder/'build.jsonl';path.write_text(path.read_text().replace('222','223'));self.reject()
    def test_rejects_record_after_requested_completion(self):
        path=self.folder/'build.jsonl';path.write_text(path.read_text()+'{}\n');self.reject()
    def test_rejects_adapter_changes(self):
        (self.folder/'surface-build-adapter.py').write_bytes(b'fake');self.reject()
    def test_rejects_wrong_gate_spec(self):
        spec=json.loads((self.folder/'build-job-spec.json').read_text());spec['gate']=str(self.folder/'other.go')
        self.write('build-job-spec.json',spec);self.reject()
    def test_rejects_missing_proof(self):
        (self.folder/'build-job-assigned.go').unlink();self.reject()
    def test_rejects_wrong_allocator(self):
        self.row['allocator']='mimalloc';self.write('build.json',self.row);self.reject()
    def test_evidence_log_mutation_changes_before_after_seal(self):
        before=self.identity();(self.folder/'build.stderr.log').write_text('changed benign warnings')
        self.assertNotEqual(before,self.identity())
    def test_v1_unchanged_without_v2_sidecars(self):
        for schema in ('surface-frozen-build-v1','headless-frozen-build-v1'):
            self.row['schema']=schema;self.write('build.json',self.row)
            identity=self.identity();self.assertNotIn('build_evidence',identity)
        (self.folder/'source-guard.json').unlink();self.assertNotIn('build_evidence',self.identity())

if __name__=='__main__':unittest.main()
