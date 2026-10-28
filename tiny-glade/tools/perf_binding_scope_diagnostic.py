"""Same frozen EXE binding-scope ON/OFF counts; no ETW or CPU gain claim."""
import argparse
from contextlib import contextmanager
import hashlib
import json
from pathlib import Path
from types import SimpleNamespace

import perf_headless as runner
import perf_headless_account as accounting
import perf_gpu_diagnostic as frozen_tools

COUNTERS=('callbacks','vec_capacity_growths','vec_capacity_bytes_after_growth_sum',
          'snapshot_builds','snapshot_hits','shape_fallback_callbacks','failed_callbacks')
NATIVE=('direct_draw_api_calls','indexed_draw_api_calls','indirect_draw_api_calls',
        'indirect_count_draw_api_calls','indexed_indirect_draw_api_calls','indexed_indirect_count_draw_api_calls',
        'direct_dispatch_api_calls','indirect_dispatch_api_calls','staging_h2d_copy_recorded_bytes',
        'immutable_producer_h2d_copy_recorded_bytes','cached_d2d_copy_recorded_bytes')
DEVICE=('setup_submitted_payload_bytes','direct_region_submitted_payload_bytes',
        'transfer_submitted_payload_bytes','buffer_setup_submitted_payload_bytes')
GRAPH=('graph_commands_recorded','graph_recording_chunks','graph_parallel_batches','graph_batches',
       'graph_retained_passes','graph_culled_passes')


def require(condition,message):
    if not condition:raise ValueError(message)


def integer(value,name):
    require(type(value) is int and 0<=value<2**64,'invalid/missing counter: '+name)
    return value


def tool_identities():
    paths={Path(__file__).name:Path(__file__).resolve(),
           Path(frozen_tools.__file__).name:Path(frozen_tools.__file__).resolve(),
           'perf_build_target.py':Path(runner.__file__).with_name('perf_build_target.py')}
    paths.update({name:Path(runner.__file__).with_name(name) for name in runner.clocks.TOOLS})
    return {name:frozen_tools.digest(path) for name,path in sorted(paths.items())}


@contextmanager
def requested_environment(enabled):
    """Add one new request after the frozen runner clears inherited TG flags."""
    original=runner.environment
    def environment(diagnostic=False):
        require(diagnostic is True,'binding count experiment requires the rich bill')
        env=original(diagnostic)
        # Count-only bill: existing phase log probes are explicitly absent.
        env.pop('TG_FRAME_TIMING',None)
        env.pop('TG_FRAME_TIMING_EVERY',None)
        env.pop('TG_GPU_TIMING',None)
        env['TG_BINDING_SCOPE_REUSE']='1' if enabled else '0'
        env['VK_INSTANCE_LAYERS']='VK_LAYER_KHRONOS_validation'
        return env
    runner.environment=environment
    try:yield
    finally:runner.environment=original


def workload_identity(report):
    actual=report.get('actual_workload') or {}
    rows=actual.get('frames')
    expected=report['samples']+1
    require(actual.get('schema')=='headless-actual-workload-v1' and actual.get('valid') is True
            and actual.get('errors')==[] and actual.get('truncated') is False
            and actual.get('capacity')==actual.get('frame_count')==expected
            and isinstance(rows,list) and len(rows)==expected,'actual workload proof absent/incomplete/invalid')
    require(isinstance(actual.get('coverage'),dict) and actual['coverage'],'actual workload coverage missing')
    previous_upload=actual.get('queued_upload_before_measurement')
    previous_device=actual.get('device_upload_before_measurement')
    require(isinstance(previous_upload,dict) and isinstance(previous_device,dict),'upload baselines missing')
    normalized=[]
    for index,(row,cpu) in enumerate(zip(rows,report['frames'])):
        require(row.get('valid') is True and all(row.get(k)==cpu.get(k) for k in ('world_tick','receipt','attempt')),
                'actual workload/CPU receipt association differs')
        native=row.get('native_commands') or {}
        require(set(native)=={'valid',*NATIVE} and native['valid'] is True,'native API/copy classification incomplete')
        for key in NATIVE:integer(native[key],key)
        graph={key:integer(row.get(key),key) for key in GRAPH}
        require(graph['graph_commands_recorded']>0,'no real recorded command')
        draws=row.get('draw_list_lengths')
        require(isinstance(draws,list) and len(draws)==21,'draw-list workload missing')
        for value in draws:integer(value,'draw-list length')
        collect=row.get('collect') or {}
        require(collect.get('valid') is True and collect.get('overflow') is False
                and collect.get('failed_waves')==0,'Collect failed/overflowed')
        for key in ('tasks','waves','cross_pass_tasks','cross_pass_waves'):integer(collect.get(key),key)
        require(collect['cross_pass_tasks']<=collect['tasks'] and collect['cross_pass_waves']<=collect['waves'],
                'cross-pass work is not a subset')
        upload=row.get('queued_upload_snapshot') or {}
        for snapshot in (previous_upload,upload):
            for key in ('manager_id','submitted_jobs','submitted_payload_bytes'):integer(snapshot.get(key),key)
            require(snapshot['manager_id']!=0 or snapshot['submitted_jobs']==snapshot['submitted_payload_bytes']==0,
                    'uninitialized upload manager has nonzero counters')
        require(previous_upload['manager_id']==0 or previous_upload['manager_id']==upload['manager_id'],
                'queued upload manager changed within run')
        queued={}
        for raw,key in (('submitted_jobs','queued_buffer_h2d_submitted_jobs'),
                        ('submitted_payload_bytes','queued_buffer_h2d_submitted_payload_bytes')):
            delta=integer(row.get(key),key)
            require(upload[raw]>=previous_upload[raw] and upload[raw]-previous_upload[raw]==delta,
                    'queued upload delta does not match raw monotonic counters')
            queued[key]=delta
        device=row.get('device_upload_snapshot') or {}
        device_delta=row.get('device_upload_submitted_payload') or {}
        require(previous_device.get('valid') is True and device.get('valid') is True
                and device_delta.get('valid') is True,'invalid device upload snapshot')
        generation=integer(previous_device.get('generation'),'device generation')
        require(generation>0 and device.get('generation')==device_delta.get('generation')==generation,
                'device generation changed within run')
        device_cost={}
        for key in DEVICE:
            before=integer(previous_device.get(key),key);after=integer(device.get(key),key)
            delta=integer(device_delta.get(key),key)
            require(after>=before and after-before==delta,'device upload delta does not match raw counters')
            device_cost[key]=delta
        normalized.append({'identity':{k:row[k] for k in ('world_tick','receipt','attempt')},
            'native_commands':native,'graph':graph,'draw_list_lengths':draws,'collect':collect,
            'queued_buffer_upload':queued,'device_upload':device_cost})
        previous_upload,previous_device=upload,device
    return {'coverage':actual['coverage'],'frames':normalized}


def validate_report(report,enabled):
    accounting.validate(report)
    require(report.get('mode')=='on' and report.get('diagnostic_bill') is True
            and report.get('cpu_probe_feature') is True and report.get('allocator')=='mimalloc',
            'requires rich cpu-probe/mimalloc bill')
    require(report.get('measurement_class')=='management_diagnostic'
            and report.get('clean_cpu_acceptance_eligible') is False and report.get('gpu_diagnostic') is None,
            'count diagnostic must not claim clean CPU acceptance or enable GPU timing')
    require(report.get('binding_scope_reuse_enabled') is enabled and report.get('binding_scope_bill_valid') is True,
            'requested binding reuse mode was not honored or bill invalid')
    modes=runner.actual_management(report)
    require(modes is not None and all(modes.values()),'four existing management algorithms must retain defaults')
    previous=None
    deltas=[]
    for row in report['frames']:
        bill=row.get('binding_scope_bill') or {}
        require(set(bill)=={'valid','algorithm_enabled',*COUNTERS} and bill['valid'] is True
                and bill['algorithm_enabled'] is enabled,'binding bill absent/invalid/mode mismatch')
        counts={key:integer(bill.get(key),key) for key in COUNTERS}
        require(counts['failed_callbacks']==0 and counts['shape_fallback_callbacks']<=counts['callbacks'],
                'binding callback failed or fallback count invalid')
        if previous is not None:
            require(all(counts[k]>=previous[k] for k in COUNTERS),'binding counter regression')
            deltas.append({k:counts[k]-previous[k] for k in COUNTERS})
        previous=counts
    require(len(deltas)==report['samples'] and sum(d['callbacks'] for d in deltas)>0,
            'binding callbacks did not execute in measured observations')
    if not enabled:require(all(d['snapshot_hits']==0 for d in deltas),'disabled reuse unexpectedly reported snapshot hits')
    return {'input':accounting.input_identity(report),'workload':report['workload'],
            'actual_workload':workload_identity(report),'management_modes':modes,
            'counter_deltas':deltas,'counter_totals':{k:sum(d[k] for d in deltas) for k in COUNTERS}}


def compare_reports(off,on):
    a,b=validate_report(off,False),validate_report(on,True)
    for key in ('input','workload','actual_workload','management_modes'):
        require(a[key]==b[key],'ON/OFF '+key+' differs; diagnostic comparison invalid')
    for left,right in zip(a['counter_deltas'],b['counter_deltas']):
        require(left['callbacks']==right['callbacks']
                and left['snapshot_builds']+left['snapshot_hits']==right['snapshot_builds']+right['snapshot_hits'],
                'binding callbacks/draw snapshot opportunities differ')
    return {'schema':'binding-scope-count-comparison-v1','valid':True,'diagnostic_only':True,
        'clean_cpu_acceptance':False,'cpu_performance_evidence':False,'production_16ms_acceptance':False,
        'formal_source_A_B':False,'same_frozen_binary_algorithm_ablation':True,
        'input_sha256':accounting.signature(a['input']),
        'actual_workload_sha256':accounting.signature(a['actual_workload']),
        'off':a['counter_totals'],'on':b['counter_totals'],
        'on_minus_off':{k:b['counter_totals'][k]-a['counter_totals'][k] for k in COUNTERS},
        'reuse_hit_observed':b['counter_totals']['snapshot_hits']>0,
        'counter_scope':'Cumulative attempted binding callbacks; interval totals are last minus first observation, excluding warmup and first CPU baseline frame',
        'capacity_bytes_scope':'sum of Vec capacity bytes immediately after each growth event, not live bytes/total allocation traffic',
        'unmeasured':['CPU milliseconds/percentiles','ETW scheduling','GPU pixel equivalence','production WSI acceptance']}


def seal_capture(path):
    artifacts={name:frozen_tools.digest(path/name) for name in ('run.json','session.json','fixture.log')
               if (path/name).is_file()}
    for name,sha in artifacts.items():(path/(name+'.sha256')).write_text(sha+'  '+name+'\n',encoding='ascii')
    return artifacts


def capture_mode(args,role,frozen_identity,tools):
    name=runner.label(args.series+'-'+role)
    capture_args=SimpleNamespace(label=args.build_label,run_label=name,mode='on',scene=args.scene,
        warmup=args.warmup,samples=args.samples,timeout=args.timeout,diagnostic=True,without_etw=True,
        official=False,archive_trace_csv=False,clock_policy=runner.clocks.LEGACY,clock_calibration=None)
    build_path=runner.PERF/'headless-builds'/runner.label(args.build_label)/'build.json'
    _,before=frozen_tools.frozen_identity(build_path)
    require(before==frozen_identity and tool_identities()==tools,'frozen build/controller changed before capture')
    path=runner.PERF/'headless-runs'/name
    try:
        with requested_environment(role=='on'):
            record=runner.capture(capture_args)
    finally:artifacts=seal_capture(path)
    require(set(artifacts)=={'run.json','session.json','fixture.log'},'capture raw artifacts incomplete')
    report=json.loads((path/'session.json').read_text(encoding='utf-8'))
    frozen_tools.strict_log((path/'fixture.log').read_text(encoding='utf-8'))
    require(record.get('etw') is False and record.get('diagnostic') is True
            and record.get('valid') is False,'counts capture cannot become clean CPU/ETW evidence')
    require(report.get('process_id')==record.get('process_id')
            and report.get('scenario')==args.scene and report.get('warmup')==args.warmup
            and report.get('samples')==args.samples
            and record['build'].get('source_content_sha256')==frozen_identity['source_content_sha256']
            and record.get('executable_sha256')==frozen_identity['files'][Path(record['build']['executable']).name],
            'capture process/config/binary identity mismatch')
    validate_report(report,role=='on')
    _,after=frozen_tools.frozen_identity(build_path)
    require(after==frozen_identity and tool_identities()==tools,'frozen build/controller changed during capture')
    return report,{'role':role,'path':str(path),'artifacts':artifacts,
        'requested_binding_scope_reuse':role=='on','honored_binding_scope_reuse':report['binding_scope_reuse_enabled']}


def run(args):
    require(args.scene in accounting.SCENES and args.warmup>=1 and 1<=args.samples<=100000 and args.timeout>0,
            'invalid count experiment bounds')
    require(args.scene=='static' or args.warmup>=200,'non-static input requires existing workload warmup')
    build_path=runner.PERF/'headless-builds'/runner.label(args.build_label)/'build.json'
    build,identity=frozen_tools.frozen_identity(build_path)
    tools=tool_identities()
    destination=runner.PERF/'binding-scope-diagnostics'/runner.label(args.series)
    destination.mkdir(parents=True,exist_ok=False)
    result={'schema':'binding-scope-diagnostic-series-v1','valid':False,'diagnostic_only':True,
        'clean_cpu_acceptance':False,'cpu_performance_evidence':False,'production_16ms_acceptance':False,
        'formal_source_A_B':False,'frozen_build':identity,'tools':tools,'settings':{
            'order':['off','on'],'scene':args.scene,'warmup':args.warmup,'samples':args.samples,
            'etw':False,'stage_logs':False,'cpu_bill':True,'vk_instance_layers':'VK_LAYER_KHRONOS_validation',
            'same_executable_sha256':build['executable_sha256']},'runs':[]}
    try:
        reports={}
        for role in ('off','on'):
            result['runs'].append({'role':role,'path':str(runner.PERF/'headless-runs'/(args.series+'-'+role)),
                                   'capture_completed':False})
            runner.dump(destination/'progress.json',result)
            report,record=capture_mode(args,role,identity,tools)
            record['capture_completed']=True
            reports[role]=report;result['runs'][-1]=record
            runner.dump(destination/'progress.json',result)
        result['comparison']=compare_reports(reports['off'],reports['on'])
        result['valid']=True
    except BaseException as error:
        result['error']=repr(error)
        raise
    finally:runner.dump(destination/'summary.json',result)
    return result


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('build_label');parser.add_argument('series')
    parser.add_argument('--scene',choices=accounting.SCENES,default='static')
    parser.add_argument('--warmup',type=int,default=60);parser.add_argument('--samples',type=int,default=30)
    parser.add_argument('--timeout',type=int,default=3600)
    print(json.dumps(run(parser.parse_args()),indent=2,ensure_ascii=False))


if __name__=='__main__':main()
