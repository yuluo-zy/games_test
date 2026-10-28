import copy
import os
import json
from pathlib import Path
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch
from test_perf_headless import report_fixture as cpu_fixture
import perf_binding_scope_diagnostic as tool


def fixture(enabled):
    report=cpu_fixture(bill=True)
    report.update(scenario='static',workload={'scene':'static'},cpu_probe_feature=True,
        measurement_class='management_diagnostic',clean_cpu_acceptance_eligible=False,gpu_diagnostic=None,
        binding_scope_reuse_enabled=enabled,binding_scope_bill_valid=True,
        management_modes={name:True for name in tool.runner.CONTROL_ENV})
    rows=[]
    upload={'manager_id':4,'submitted_jobs':100,'submitted_payload_bytes':1000}
    device={'generation':5,'valid':True,**{key:0 for key in tool.DEVICE}}
    before_upload=copy.deepcopy(upload);before_device=copy.deepcopy(device)
    for index,cpu in enumerate(report['frames']):
        bill={'valid':True,'algorithm_enabled':enabled,'callbacks':100+index*5,
            'vec_capacity_growths':(10+index*2) if enabled else (1000+index*100),
            'vec_capacity_bytes_after_growth_sum':(1024+index*128) if enabled else (100000+index*4096),
            'snapshot_builds':(20+index*10) if enabled else (2000+index*100),
            'snapshot_hits':1980+index*90 if enabled else 0,
            'shape_fallback_callbacks':0,'failed_callbacks':0}
        cpu['binding_scope_bill']=bill
        upload['submitted_jobs']+=1;upload['submitted_payload_bytes']+=8
        device[tool.DEVICE[0]]+=8;device[tool.DEVICE[3]]+=4
        native={'valid':True,**{key:0 for key in tool.NATIVE}}
        native.update(direct_draw_api_calls=9,direct_dispatch_api_calls=3,staging_h2d_copy_recorded_bytes=8)
        rows.append({**{key:cpu[key] for key in ('world_tick','receipt','attempt')},'valid':True,
            'native_commands':native,**{key:0 for key in tool.GRAPH},'graph_commands_recorded':8,
            'graph_recording_chunks':8,'graph_retained_passes':12,'draw_list_lengths':[1]*21,
            'collect':{'valid':True,'overflow':False,'failed_waves':0,'tasks':4,'waves':1,
                       'cross_pass_tasks':2,'cross_pass_waves':1},
            'queued_upload_snapshot':copy.deepcopy(upload),'queued_buffer_h2d_submitted_jobs':1,
            'queued_buffer_h2d_submitted_payload_bytes':8,'device_upload_snapshot':copy.deepcopy(device),
            'device_upload_submitted_payload':{'generation':5,'valid':True,tool.DEVICE[0]:8,
                tool.DEVICE[1]:0,tool.DEVICE[2]:0,tool.DEVICE[3]:4}})
    report['actual_workload']={'schema':'headless-actual-workload-v1','valid':True,'errors':[],
        'truncated':False,'capacity':3,'frame_count':3,'coverage':{'included':['real']},'frames':rows,
        'queued_upload_before_measurement':before_upload,'device_upload_before_measurement':before_device}
    return report


class BindingComparisonTests(unittest.TestCase):
    def test_equal_real_workload_counts_compare_without_cpu_gain_claim(self):
        result=tool.compare_reports(fixture(False),fixture(True))
        self.assertTrue(result['valid']);self.assertTrue(result['reuse_hit_observed'])
        self.assertEqual(result['off']['callbacks'],10)
        self.assertEqual(result['off']['vec_capacity_growths'],200)
        self.assertEqual(result['on']['vec_capacity_growths'],4)
        self.assertFalse(result['cpu_performance_evidence'])
        self.assertFalse(result['clean_cpu_acceptance'])
        self.assertFalse(result['formal_source_A_B'])

    def test_top_and_each_row_mode_must_match_request(self):
        for mutation in ('top','row','missing'):
            report=fixture(True)
            if mutation=='top':report['binding_scope_reuse_enabled']=False
            elif mutation=='row':report['frames'][1]['binding_scope_bill']['algorithm_enabled']=False
            else:report.pop('binding_scope_reuse_enabled')
            with self.subTest(mutation=mutation),self.assertRaises(ValueError):tool.validate_report(report,True)

    def test_missing_bill_actual_workload_or_classification_rejected(self):
        for mutation in ('bill','workload','native'):
            report=fixture(True)
            if mutation=='bill':report['frames'][1].pop('binding_scope_bill')
            elif mutation=='workload':report.pop('actual_workload')
            else:report['actual_workload']['frames'][0]['native_commands'].pop(tool.NATIVE[0])
            with self.subTest(mutation=mutation),self.assertRaises(ValueError):tool.validate_report(report,True)

    def test_overflow_failure_and_regression_rejected(self):
        for mutation in ('invalid','failed','regression'):
            report=fixture(True);bill=report['frames'][1]['binding_scope_bill']
            if mutation=='invalid':bill['valid']=False
            elif mutation=='failed':bill['failed_callbacks']=1
            else:bill['snapshot_builds']=0
            with self.subTest(mutation=mutation),self.assertRaises(ValueError):tool.validate_report(report,True)

    def test_raw_input_or_native_commands_cannot_change(self):
        for mutation in ('input','native','graph'):
            off,on=fixture(False),fixture(True)
            if mutation=='input':on['input_before']['draws'][0]=999;on['input_after']['draws'][0]=999
            elif mutation=='native':on['actual_workload']['frames'][1]['native_commands']['direct_draw_api_calls']+=1
            else:on['actual_workload']['frames'][1]['graph_retained_passes']+=1
            with self.subTest(mutation=mutation),self.assertRaises(ValueError):tool.compare_reports(off,on)

    def test_queued_or_device_upload_difference_rejected(self):
        off,on=fixture(False),fixture(True)
        for row in on['actual_workload']['frames']:
            row['queued_upload_snapshot']['submitted_payload_bytes']+=8
        on['actual_workload']['frames'][0]['queued_buffer_h2d_submitted_payload_bytes']+=8
        with self.assertRaises(ValueError):tool.compare_reports(off,on)
        off,on=fixture(False),fixture(True)
        for row in on['actual_workload']['frames']:
            row['device_upload_snapshot'][tool.DEVICE[0]]+=8
        on['actual_workload']['frames'][0]['device_upload_submitted_payload'][tool.DEVICE[0]]+=8
        with self.assertRaises(ValueError):tool.compare_reports(off,on)

    def test_manager_reset_device_generation_or_receipt_mismatch_rejected(self):
        for mutation in ('manager','generation','receipt'):
            report=fixture(True);row=report['actual_workload']['frames'][1]
            if mutation=='manager':row['queued_upload_snapshot']['manager_id']=9
            elif mutation=='generation':row['device_upload_snapshot']['generation']=9
            else:row['receipt']+=1
            with self.subTest(mutation=mutation),self.assertRaises(ValueError):tool.validate_report(report,True)

    def test_old_four_modes_and_cpu_diagnostic_scope_not_changed(self):
        for mutation in ('modes','cpu_accept','gpu'):
            report=fixture(True)
            if mutation=='modes':report['management_modes']['reflection_scope']=False
            elif mutation=='cpu_accept':report['clean_cpu_acceptance_eligible']=True
            else:report['gpu_diagnostic']={'valid':True}
            with self.subTest(mutation=mutation),self.assertRaises(ValueError):tool.validate_report(report,True)

    def test_warmup_cumulative_cost_is_not_reported_as_measured_interval(self):
        result=tool.compare_reports(fixture(False),fixture(True))
        self.assertEqual(result['off']['snapshot_builds'],200)
        self.assertEqual(result['on']['snapshot_builds'],20)

    def test_env_wrapper_sets_request_after_cleanup_and_restores_runner(self):
        original=tool.runner.environment
        with patch.dict(os.environ,{'TG_BINDING_SCOPE_REUSE':'wrong','TG_FRAME_TIMING':'1',
                                     'TG_REFLECTION_SCOPE':'0','TG_GPU_DIAGNOSTIC':'1'},clear=False):
            with tool.requested_environment(False):
                env=tool.runner.environment(True)
                self.assertEqual(env['TG_BINDING_SCOPE_REUSE'],'0')
                self.assertEqual(env['TG_CPU_BILL'],'1')
                self.assertNotIn('TG_FRAME_TIMING',env)
                self.assertNotIn('TG_REFLECTION_SCOPE',env)
                self.assertNotIn('TG_GPU_DIAGNOSTIC',env)
        self.assertIs(tool.runner.environment,original)


class CaptureContractTests(unittest.TestCase):
    def test_two_frozen_mode_captures_really_receive_explicit_environment(self):
        self.exercise(False)

    def test_comparison_failure_keeps_both_raw_capture_paths_and_hashes(self):
        self.exercise(True)

    def exercise(self,change_input):
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory);exe=root/'frozen.exe'
            sha='f'*64;source='a'*64
            frozen={'executable_sha256':sha}
            identity={'files':{exe.name:sha},'source_content_sha256':source}
            args=SimpleNamespace(build_label='frozen',series='binding-test',scene='static',warmup=60,samples=2,timeout=30)
            requested=[]
            def capture(arguments):
                env=tool.runner.environment(arguments.diagnostic)
                enabled=env['TG_BINDING_SCOPE_REUSE']=='1';requested.append(enabled)
                self.assertNotIn('TG_FRAME_TIMING',env)
                self.assertTrue(arguments.without_etw)
                self.assertTrue(arguments.diagnostic)
                path=root/'headless-runs'/arguments.run_label;path.mkdir(parents=True)
                report=fixture(enabled);report['warmup']=60
                if enabled and change_input:
                    report['input_before']['draws'][0]=999;report['input_after']['draws'][0]=999
                record={'etw':False,'diagnostic':True,'valid':False,'process_id':7,'executable_sha256':sha,
                        'build':{'executable':str(exe),'source_content_sha256':source}}
                tool.runner.dump(path/'session.json',report);tool.runner.dump(path/'run.json',record)
                (path/'fixture.log').write_text('running 1 test\ntest '+tool.frozen_tools.TEST+
                    ' ... boot diagnostics\nok\ntest result: ok. 1 passed; 0 failed; 0 ignored;\n',encoding='utf-8')
                return record
            original=tool.runner.environment
            with patch.object(tool.runner,'PERF',root),patch.object(tool.runner,'capture',side_effect=capture),\
                 patch.object(tool.frozen_tools,'frozen_identity',return_value=(frozen,identity)) as proof,\
                 patch.object(tool,'tool_identities',return_value={'new-controller':'b'*64}):
                if change_input:
                    with self.assertRaises(ValueError):tool.run(args)
                else:self.assertTrue(tool.run(args)['valid'])
            self.assertIs(tool.runner.environment,original)
            self.assertEqual(requested,[False,True]);self.assertEqual(proof.call_count,5)
            summary=json.loads((root/'binding-scope-diagnostics/binding-test/summary.json').read_text())
            self.assertIs(summary['valid'],not change_input)
            self.assertFalse(summary['cpu_performance_evidence']);self.assertEqual(len(summary['runs']),2)
            for role in ('off','on'):
                for name in ('run.json','session.json','fixture.log'):
                    self.assertTrue((root/'headless-runs'/('binding-test-'+role)/(name+'.sha256')).is_file())


if __name__=='__main__':unittest.main()
