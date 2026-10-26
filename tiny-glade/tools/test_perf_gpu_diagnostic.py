import copy
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch
import perf_gpu_diagnostic as tool


def report_fixture():
    native={name:0 for name in ('direct_draw_api_calls','indexed_draw_api_calls','indirect_draw_api_calls',
        'indirect_count_draw_api_calls','indexed_indirect_draw_api_calls','indexed_indirect_count_draw_api_calls',
        'direct_dispatch_api_calls','indirect_dispatch_api_calls','staging_h2d_copy_recorded_bytes',
        'immutable_producer_h2d_copy_recorded_bytes','cached_d2d_copy_recorded_bytes')}
    native.update(valid=True,direct_draw_api_calls=9)
    rows=[{'world_tick':n+1,'receipt':n+11,'attempt':n+21,'valid':True,'native_commands':native,
           'collect':{'valid':True,'overflow':False,'failed_waves':0},'draw_list_lengths':[1]*21,
           'graph_commands_recorded':2} for n in range(2)]
    frames=[{'world_tick':n,'receipt':n+10,'attempt':n+20,'measured':n>0,'expected_commands':2} for n in range(3)]
    commands=[{'queue':0,'completion_value':n+100,'recording_epoch':n+1,'duration_ns':duration}
              for n in range(3) for duration in (20,30)]
    return {'schema':'headless-cpu-closure-v1','valid':True,'process_id':7,'scenario':'static',
        'warmup':1,'samples':1,'measurement_class':'gpu_diagnostic','clean_cpu_acceptance_eligible':False,
        'diagnostic_bill':False,'mode':'on','cpu_probe_feature':True,'allocator':'mimalloc',
        'input_before':{'original':'scene'},'input_after':{'original':'scene'},'frames':rows,
        'actual_workload':{'schema':'headless-actual-workload-v1','valid':True,'errors':[],'truncated':False,
                           'frame_count':2,'frames':copy.deepcopy(rows)},
        'gpu_diagnostic':{'schema':'headless-gpu-command-diagnostic-v1','valid':True,'pass_spans':False,
            'command_span_sum_and_max_are_not_frame_latency':True,'timestamp_period_ns':1.0,
            'queue_timestamp_valid_bits':[64],'ring_stats':{'pending':0,'capacity_skips':0,'retained_samples':0,'completed':6},
            'drained_samples':6,'sample_count':6,'frames':frames,
            'receipt_points':[{'queue':0,'value':n+100,'frame':n} for n in range(3)],
            'samples':commands,'sample_frame_indices':[0,0,1,1,2,2]}}


class DiagnosticValidationTests(unittest.TestCase):
    def validate(self,report):
        tool.validate_session(report,7,'static',1,1)

    def test_same_point_two_real_commands_are_preserved(self):
        self.validate(report_fixture())

    def test_missing_or_partial_workload_not_treated_as_zero(self):
        for field in ('actual_workload','gpu_diagnostic','input_before'):
            report=report_fixture();del report[field]
            with self.subTest(field=field),self.assertRaises(ValueError):self.validate(report)
        report=report_fixture();report['actual_workload']['frames'][0]['native_commands']['valid']=False
        with self.assertRaises(ValueError):self.validate(report)

    def test_missing_native_classification_rejected(self):
        report=report_fixture();del report['actual_workload']['frames'][0]['native_commands']['direct_draw_api_calls']
        with self.assertRaises(ValueError):self.validate(report)

    def test_skipped_pending_and_evicted_timestamps_rejected(self):
        for key in ('pending','capacity_skips','retained_samples','completed'):
            report=report_fixture();report['gpu_diagnostic']['ring_stats'][key]+=1
            with self.subTest(field=key),self.assertRaises(ValueError):self.validate(report)

    def test_missing_or_duplicated_command_sample_rejected(self):
        for operation in ('remove','duplicate'):
            report=report_fixture();gpu=report['gpu_diagnostic']
            if operation=='remove':gpu['samples'].pop()
            else:gpu['samples'].append(gpu['samples'][-1].copy())
            with self.subTest(operation=operation),self.assertRaises(ValueError):self.validate(report)

    def test_unmatched_receipt_duplicate_point_and_wrong_epoch_rejected(self):
        for mutation in ('point','duplicate','epoch','row'):
            report=report_fixture();gpu=report['gpu_diagnostic']
            if mutation=='point':gpu['samples'][0]['completion_value']=999
            elif mutation=='duplicate':gpu['receipt_points'].append(gpu['receipt_points'][0].copy())
            elif mutation=='epoch':gpu['samples'][1]['recording_epoch']=9
            else:report['actual_workload']['frames'][0]['receipt']=999
            with self.subTest(mutation=mutation),self.assertRaises(ValueError):self.validate(report)

    def test_diagnostic_cannot_claim_clean_cpu_eligibility(self):
        report=report_fixture();report['clean_cpu_acceptance_eligible']=True
        with self.assertRaises(ValueError):self.validate(report)

    def test_explicit_environment_added_after_parent_probe_cleanup(self):
        with patch.dict(tool.os.environ,{'TG_FRAME_TIMING':'1','TG_CPU_BILL':'1','TG_GPU_TIMING':'1',
                                         'TG_PERF_SCENARIO':'wrong','RHAPSODY_UNKNOWN':'old'},clear=True):
            env=tool.environment('asset','report','static',600,3000,500000)
        self.assertEqual(env['TG_GPU_DIAGNOSTIC'],'1')
        self.assertEqual(env['TG_PERF_SCENARIO'],'static')
        self.assertEqual(env['RHAPSODY_REQUIRE_VULKAN'],'1')
        for field in ('TG_FRAME_TIMING','TG_CPU_BILL','TG_GPU_TIMING','RHAPSODY_UNKNOWN'):
            self.assertNotIn(field,env)

    def test_log_accepts_nocapture_boot_but_rejects_vuid_zero_ignored(self):
        good='running 1 test\ntest '+tool.TEST+' ... boot diagnostics\nok\n\ntest result: ok. 1 passed; 0 failed; 0 ignored;\n'
        tool.strict_log(good)
        for text in (good+'VUID-image-123',good.replace('1 passed','0 passed'),
                     good.replace('0 ignored','1 ignored'),good.replace(tool.TEST,'other_test')):
            with self.assertRaises(ValueError):tool.strict_log(text)


class FrozenIdentityTests(unittest.TestCase):
    def setUp(self):
        self.temporary=tempfile.TemporaryDirectory();self.addCleanup(self.temporary.cleanup)
        self.root=Path(self.temporary.name)
        self.exe=self.root/'fixture.exe'
        for name in ('fixture.exe','fixture.pdb','fmod.dll','fmodstudio.dll'):
            (self.root/name).write_bytes(name.encode())
        source={'commit':'a'*40,'files':{name:'b'*64 for name in tool.FIXTURE_FILES}}
        source['content_sha256']=tool.signature(source['files'])
        tool.dump(self.root/'source.json',source)
        self.build={'schema':'headless-frozen-build-v1','features':['cpu-probe'],'allocator':'mimalloc',
                    'incremental':False,'entire_cargo_build_passed':True,'executable':str(self.exe),
                    'executable_sha256':tool.digest(self.exe),
                    'files':{path.name:tool.digest(path) for path in self.root.iterdir() if path.suffix!='.json'},
                    'source_commit':source['commit'],'source_content_sha256':source['content_sha256']}
        self.path=self.root/'build.json';tool.dump(self.path,self.build)

    def test_frozen_exe_pdb_source_identity_and_tamper(self):
        tool.frozen_identity(self.path)
        (self.root/'fixture.pdb').write_bytes(b'changed')
        with self.assertRaises(ValueError):tool.frozen_identity(self.path)

    def test_frozen_source_or_incomplete_build_not_accepted(self):
        original=copy.deepcopy(self.build)
        self.build['entire_cargo_build_passed']=False;tool.dump(self.path,self.build)
        with self.assertRaises(ValueError):tool.frozen_identity(self.path)
        tool.dump(self.path,original)
        source=json.loads((self.root/'source.json').read_text())
        source['files'][tool.FIXTURE_FILES[0]]='c'*64;tool.dump(self.root/'source.json',source)
        with self.assertRaises(ValueError):tool.frozen_identity(self.path)

    def target_only(self):
        workspace=self.root/'already-archived-workspace'
        self.command=['cargo','build','-p','tiny-glade','--test','original_gameplay_chain',
                      '--locked','--offline','-j1','--message-format=json-render-diagnostics',
                      '--features','cpu-probe']
        original=workspace/'target/debug/deps'/self.exe.name
        self.artifact={'reason':'compiler-artifact','target':{'kind':['test'],'name':'original_gameplay_chain'},
                       'profile':{'test':True,'opt_level':'0'},'features':['cpu-probe','default','mimalloc'],
                       'executable':str(original),'filenames':[str(original),str(original.with_suffix('.pdb'))],
                       'manifest_path':str(workspace/'crates/tiny-glade/Cargo.toml')}
        self.lifecycle={'strategy':'requested test target only','requested_target_completed':True,
                        'job_assigned_before_cargo_start':True,'termination_was_intentional_after_requested_artifact':True,
                        'active_build_processes_remaining':False,'remaining_targets_are_not_accepted':True,
                        'entire_cargo_build_passed':False,'job_termination_exit_code':tool.TERMINATED_AFTER_TARGET,
                        'wrapper_exit_code':tool.TERMINATED_AFTER_TARGET,'cargo_exit_code':None,
                        'wrapper_pid':81,'cargo_pid':82}
        self.events=[{'reason':'perf-owned-cargo-start','pid':82,'wrapper_pid':81},copy.deepcopy(self.artifact)]
        tool.dump(self.root/'build-job-spec.json',{'cwd':str(workspace),'command':self.command,
                                                  'gate':str(self.root/'build-job-assigned.go')})
        (self.root/'build-job-assigned.go').write_text('assigned',encoding='utf-8')
        (self.root/'build.stderr.log').write_text('warning: test fixture\n',encoding='utf-8')
        source=json.loads((self.root/'source.json').read_text())
        source['status']='';source['files'].update(tool.REVIEWED_TARGET_FREEZER)
        source['content_sha256']=tool.signature(source['files'])
        tool.dump(self.root/'source.json',source)
        self.build.update(entire_cargo_build_passed=False,requested_test_target_completed=True,
                          profile='debug',command=self.command,dirty=False,
                          features=['default(mimalloc)','cpu-probe'],cpu_probe_feature=True,
                          build_helper_sha256=tool.REVIEWED_TARGET_FREEZER['tools/perf_build_target.py'],
                          source_content_sha256=source['content_sha256'])
        self.save_target_evidence()

    def save_target_evidence(self):
        tool.dump(self.root/'requested-target-artifact.json',self.artifact)
        tool.dump(self.root/'cargo-lifecycle.json',self.lifecycle)
        (self.root/'build.jsonl').write_text(''.join(json.dumps(event)+'\n' for event in self.events),encoding='utf-8')
        self.build.update(requested_target_evidence_sha256=tool.digest(self.root/'requested-target-artifact.json'),
                          build_jsonl_sha256=tool.digest(self.root/'build.jsonl'),cargo_lifecycle=copy.deepcopy(self.lifecycle))
        tool.dump(self.path,self.build)

    def test_completed_target_only_job_accepted_without_live_source_or_other_targets(self):
        self.target_only()
        _,identity=tool.frozen_identity(self.path)
        proof=identity['build_acceptance_scope']
        self.assertFalse(proof['entire_cargo_build_passed'])
        self.assertTrue(proof['remaining_targets_are_not_accepted'])
        self.assertEqual(proof['policy'],'requested-target-private-job-reviewed-freezer-v1')
        self.assertFalse((self.root/'already-archived-workspace').exists())

    def test_target_only_missing_evidence_or_artifact_sha_tamper_rejected(self):
        self.target_only()
        self.build.pop('requested_target_evidence_sha256');tool.dump(self.path,self.build)
        with self.assertRaises(ValueError):tool.frozen_identity(self.path)
        self.save_target_evidence()
        (self.root/'requested-target-artifact.json').write_text('{}',encoding='utf-8')
        with self.assertRaises(ValueError):tool.frozen_identity(self.path)

    def test_target_only_residual_process_or_unassigned_job_rejected(self):
        for field,value in [('active_build_processes_remaining',True),('job_assigned_before_cargo_start',False),
                            ('remaining_targets_are_not_accepted',False),('wrapper_exit_code',1)]:
            self.target_only();self.lifecycle[field]=value;self.save_target_evidence()
            with self.subTest(field=field),self.assertRaises(ValueError):tool.frozen_identity(self.path)

    def test_target_only_unfinished_or_wrong_compiler_artifact_rejected(self):
        self.target_only();self.build['requested_test_target_completed']=False;tool.dump(self.path,self.build)
        with self.assertRaises(ValueError):tool.frozen_identity(self.path)
        self.target_only();self.artifact['profile']['test']=False
        self.events[-1]=copy.deepcopy(self.artifact);self.save_target_evidence()
        with self.assertRaises(ValueError):tool.frozen_identity(self.path)

    def test_target_only_compiler_error_or_missing_jsonl_artifact_rejected(self):
        self.target_only();self.events.insert(1,{'reason':'compiler-message','message':{'level':'error'}})
        self.save_target_evidence()
        with self.assertRaises(ValueError):tool.frozen_identity(self.path)
        self.target_only();self.events.pop();self.save_target_evidence()
        with self.assertRaises(ValueError):tool.frozen_identity(self.path)

    def test_target_only_dirty_or_unreviewed_freezer_cannot_assert_source_guard(self):
        self.target_only();self.build['dirty']=True;tool.dump(self.path,self.build)
        with self.assertRaises(ValueError):tool.frozen_identity(self.path)
        self.target_only()
        source=json.loads((self.root/'source.json').read_text())
        source['files']['tools/perf_headless.py']='e'*64
        source['content_sha256']=tool.signature(source['files'])
        self.build['source_content_sha256']=source['content_sha256']
        tool.dump(self.root/'source.json',source);tool.dump(self.path,self.build)
        with self.assertRaises(ValueError):tool.frozen_identity(self.path)

    def test_target_only_cannot_be_relabelled_as_whole_cargo_success(self):
        self.target_only();self.build['entire_cargo_build_passed']=True
        tool.dump(self.path,self.build)
        with self.assertRaises(ValueError):tool.frozen_identity(self.path)

    def test_target_exit_race_still_accepts_only_requested_target(self):
        self.target_only();self.lifecycle['wrapper_exit_code']=0;self.save_target_evidence()
        _,identity=tool.frozen_identity(self.path)
        self.assertFalse(identity['build_acceptance_scope']['entire_cargo_build_passed'])
        self.assertTrue(identity['build_acceptance_scope']['remaining_targets_are_not_accepted'])

    def test_target_declared_features_must_match_real_requested_artifact(self):
        self.target_only();self.build['features'].append('pprof');tool.dump(self.path,self.build)
        with self.assertRaises(ValueError):tool.frozen_identity(self.path)

    def test_relabelled_target_only_cannot_hide_standalone_lifecycle_by_removing_inline(self):
        self.target_only();self.build['entire_cargo_build_passed']=True
        self.build.pop('cargo_lifecycle');tool.dump(self.path,self.build)
        with self.assertRaises(ValueError):tool.frozen_identity(self.path)

    def test_target_specific_proof_still_needs_lifecycle_after_standalone_removal(self):
        self.target_only();self.build['entire_cargo_build_passed']=True
        self.build.pop('cargo_lifecycle');(self.root/'cargo-lifecycle.json').unlink()
        tool.dump(self.path,self.build)
        with self.assertRaises(ValueError):tool.frozen_identity(self.path)

    def test_whole_cargo_known_producer_with_consistent_lifecycle_remains_valid(self):
        self.target_only()
        self.lifecycle.update(entire_cargo_build_passed=True,cargo_exit_code=0,wrapper_exit_code=0,
                              termination_was_intentional_after_requested_artifact=False)
        self.build['entire_cargo_build_passed']=True
        self.events.extend([{'reason':'build-finished','success':True},
                            {'reason':'perf-owned-cargo-finish','pid':82,'exit_code':0}])
        self.save_target_evidence()
        _,identity=tool.frozen_identity(self.path)
        self.assertEqual(identity['build_acceptance_scope']['policy'],'whole-cargo-build-passed')

    def test_whole_flags_cannot_promote_known_killed_cargo_without_zero_exit(self):
        self.target_only();self.build['entire_cargo_build_passed']=True
        self.lifecycle['entire_cargo_build_passed']=True;self.save_target_evidence()
        with self.assertRaises(ValueError):tool.frozen_identity(self.path)

    def whole_success(self):
        self.target_only()
        self.build['entire_cargo_build_passed']=True
        self.lifecycle.update(entire_cargo_build_passed=True,cargo_exit_code=0,wrapper_exit_code=0,
                              termination_was_intentional_after_requested_artifact=False)
        self.events.extend([{'reason':'build-finished','success':True},
                            {'reason':'perf-owned-cargo-finish','pid':82,'exit_code':0}])
        self.save_target_evidence()

    def test_known_whole_zero_flags_need_real_success_events(self):
        for missing in ('build-finished','perf-owned-cargo-finish'):
            self.whole_success();self.events=[e for e in self.events if e.get('reason')!=missing]
            self.save_target_evidence()
            with self.subTest(missing=missing),self.assertRaises(ValueError):tool.frozen_identity(self.path)

    def test_known_whole_finish_error_duplicate_or_wrong_pid_rejected(self):
        for mutation in ('build_error','duplicate','wrong_pid','cargo_error'):
            self.whole_success()
            if mutation=='build_error':self.events[-2]['success']=False
            elif mutation=='duplicate':self.events.insert(-1,copy.deepcopy(self.events[-2]))
            elif mutation=='wrong_pid':self.events[-1]['pid']=999
            else:self.events[-1]['exit_code']=1
            self.save_target_evidence()
            with self.subTest(mutation=mutation),self.assertRaises(ValueError):tool.frozen_identity(self.path)

    def test_non_windows_whole_success_needs_build_finished_without_private_wrapper(self):
        self.whole_success()
        for key in ('wrapper_pid','cargo_pid','job_assigned_before_cargo_start'):
            self.lifecycle.pop(key)
        self.lifecycle['wrapper_exit_code']=None
        self.events=[e for e in self.events if not e.get('reason','').startswith('perf-owned-cargo-')]
        self.save_target_evidence()
        _,identity=tool.frozen_identity(self.path)
        self.assertTrue(identity['build_acceptance_scope']['entire_cargo_build_passed'])


if __name__=='__main__':unittest.main()
