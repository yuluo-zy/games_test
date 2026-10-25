#!/usr/bin/env python3
"""Run the existing desktop fixture; keep raw logs and source/binary-bound measurements."""
import argparse, contextlib, datetime, fcntl, hashlib, json, math, os, pathlib, re, subprocess, time

ROOT=pathlib.Path(__file__).resolve().parents[4]

def percentile(values, q):
    if not values: return None
    ordered=sorted(values)
    return ordered[max(0, math.ceil(len(ordered)*q)-1)]

def summarize(log, warmup, frames):
    text=log.read_text(errors='replace'); observations=[]; stages={}; extras={}; frame_counters={}; receipt_frames={}
    for line in text.splitlines():
        fields=dict(re.findall(r'([a-z_]+)=([^\s]+)',line))
        if line.startswith('PERF_FRAME '):
            observations.append({k:int(v) for k,v in fields.items()})
        elif ' render_stages ' in line:
            receipt=re.search(r'receipt=Some\((\d+)\)',line)
            if receipt:
                stages[int(receipt[1])]={k:float(v) for k,v in fields.items() if k.endswith('_ms')}
                receipt_frames[int(receipt[1])]=int(fields['gameplay_n'])
        elif ' draw_collect ' in line:
            index=int(fields['gameplay_n']);extras[index]={k:float(v) for k,v in fields.items() if k.endswith('_ms') or k in ['passes','tasks','cross_pass_tasks']}
        elif ' frame_counters ' in line:
            frame_counters[int(fields['gameplay_n'])]=int(fields['upload_pending_yields'])
    selected=[r for r in observations if warmup < r['frame'] <= warmup+frames]
    if len(selected)!=frames: raise RuntimeError(f'Expected {frames} measured frames, got {len(selected)}')
    previous={r['frame']:r for r in observations}
    intervals=[(r['elapsed_ns']-previous[r['frame']-1]['elapsed_ns'])/1e6 for r in selected if r['frame']-1 in previous]
    distribution=lambda values:dict(n=len(values),p50=percentile(values,.5),p95=percentile(values,.95),p99=percentile(values,.99))
    metrics={'observed_native_frame_interval_ms':distribution(intervals)}
    for key in sorted({key for r in selected for key in stages.get(r['receipt'],{})}):
        metrics[key]=distribution([stages[r['receipt']][key] for r in selected if key in stages.get(r['receipt'],{})])
    gpu_windows=re.findall(r'gpu_spans window_frames=\d+ command_spans_per_frame=([0-9.]+)',text)
    measured_counters=[frame_counters[n] for r in selected
                       if (n:=receipt_frames.get(r['receipt'])) in frame_counters]
    measured_collect=[extras[n] for r in selected if (n:=receipt_frames.get(r['receipt'])) in extras]
    allocator=re.search(r'^WSI_ALLOCATOR (\S+)$',text,re.MULTILINE)
    return {'frames':len(selected),'warmup':warmup,'observed_fps':1000*len(intervals)/sum(intervals),
            'allocator':allocator[1] if allocator else None,
            'measured_upload_pending_yields':{'rows':len(measured_counters),'total':sum(measured_counters),'maximum':max(measured_counters,default=0)} if measured_counters else None,
            'metrics':metrics,'render_stage_rows':sum(r['receipt'] in stages for r in selected),
            'gpu_window_count':len(gpu_windows),'gpu_windows_with_samples':sum(float(count)>0 for count in gpu_windows),
            'collect_measured':{'rows':len(measured_collect), **{key:sum(row.get(key,0) for row in measured_collect) for key in ['tasks','cross_pass_tasks']}},
            'collect_totals_all_gameplay':{key:sum(row.get(key,0) for row in extras.values()) for key in ['tasks','cross_pass_tasks']} if extras else None,
            'startup_line':next((line for line in text.splitlines() if line.startswith('PERF_WORLD_FIRST ')),None),
            'scope':'Real desktop loop and native WSI; render-feedback intervals measure throughput, not GPU completion or photon latency. Stage wall times overlap CPU/GPU work and must not be summed. Missing timing rows/counters mean instrumentation was disabled.'}

def require_parallel_evidence(metadata, binary_sha256):
    if metadata.get('exit_code') != 0 or metadata.get('analysis_error'):
        raise RuntimeError('Parallel evidence must be a successful diagnostic')
    if metadata.get('binary_sha256') != binary_sha256:
        raise RuntimeError('Parallel evidence belongs to a different binary')
    collect=metadata.get('measurement',{}).get('collect_measured',{})
    if (not metadata.get('stage_timing') or collect.get('rows') != metadata.get('frames')
            or collect.get('tasks',0) <= 0 or collect.get('cross_pass_tasks',0) <= 0):
        raise RuntimeError('Parallel diagnostic must observe actual tasks and cross-pass tasks throughout the measured frame range')

def cpu_summary(snapshots,warmup,frames):
    # macOS ps can report a just-exited child as 0:00.00 / RSS 0 before
    # waitpid reaps it. That is not a cumulative CPU counter sample.
    measured=[s for s in snapshots if warmup < s['last_observed_world_frame'] <= warmup+frames
              and len(s['cpu_time_and_rss_kib'].split()) == 2
              and int(s['cpu_time_and_rss_kib'].split()[1]) > 0]
    if len(measured)<2:return None
    def cpu_seconds(sample):
        value=sample['cpu_time_and_rss_kib'].split()[0];result=0.
        for part in value.split(':'):result=result*60+float(part)
        return result
    first,last=measured[0],measured[-1]
    cpu_delta=cpu_seconds(last)-cpu_seconds(first)
    if cpu_delta<0:return {'unavailable':'process cumulative CPU counter regressed'}
    frame_delta=last['last_observed_world_frame']-first['last_observed_world_frame']
    return {'average_process_cpu_percent':100*cpu_delta/(last['elapsed_s']-first['elapsed_s']),
        'sampled_cpu_ms_per_observed_frame':1000*cpu_delta/frame_delta if frame_delta else None,
        'peak_rss_kib':max(int(s['cpu_time_and_rss_kib'].split()[1]) for s in measured),
        'observed_frame_range':[first['last_observed_world_frame'],last['last_observed_world_frame']],
        'scope':'1-second process snapshots; 100% is one CPU core; sampled subrange excludes startup/warmup, not exact full-frame CPU attribution.'}

@contextlib.contextmanager
def build_exclusion():
    # Cargo uses these same advisory locks. Keep compiler CPU/memory pressure
    # out of each hardware measurement; release them between measurements.
    with contextlib.ExitStack() as stack:
        for profile in ('debug', 'release'):
            path=ROOT/'target'/profile/'.cargo-lock'
            path.parent.mkdir(parents=True,exist_ok=True)
            lock=stack.enter_context(path.open('a'))
            fcntl.flock(lock,fcntl.LOCK_EX)
        yield

def frozen_build(binary_sha256):
    matches=[]
    for path in pathlib.Path(__file__).parent.glob('production-v*-build.json'):
        metadata=json.loads(path.read_text())
        if metadata.get('binary_sha256')==binary_sha256 and metadata.get('exit_code')==0:
            matches.append((path,metadata))
    if len(matches)!=1:raise RuntimeError('Expected one frozen build manifest for this binary')
    path,metadata=matches[0]
    if metadata.get('changed_during_build'):raise RuntimeError('Sources changed during the frozen build')
    archive=pathlib.Path(metadata['audited_source_archive'])
    if hashlib.sha256(archive.read_bytes()).hexdigest()!=metadata['audited_source_archive_sha256']:
        raise RuntimeError('Frozen source archive identity mismatch')
    return path,metadata

def run(args,index):
    with build_exclusion():
        return run_exclusive(args,index)

def run_exclusive(args,index):
    directory=args.out/f'{args.scenario}-{args.mode}-round{index}';directory.mkdir(parents=True,exist_ok=False)
    env={k:v for k,v in os.environ.items() if not k.startswith(('TG_VALIDATE_', 'TG_MULTIFRAME_', 'TG_PROFILE_'))};env.update(VK_ICD_FILENAMES='/opt/homebrew/etc/vulkan/icd.d/MoltenVK_icd.json',
        VK_LAYER_PATH='/opt/homebrew/share/vulkan/explicit_layer.d',DYLD_FALLBACK_LIBRARY_PATH='/opt/homebrew/lib',
        RHAPSODY_REQUIRE_VULKAN='1',MVK_CONFIG_USE_METAL_ARGUMENT_BUFFERS='1',RUST_BACKTRACE='1',TG_INSTALL_ROOT=str(args.install),TG_VALIDATE_GAMEPLAY_WSI='1',
        TG_VALIDATE_WORLD_FRAMES=str(args.warmup+args.frames),TG_VALIDATE_TIMEOUT_SECONDS=str(args.timeout),
        TG_PERF_SCENARIO=args.scenario,TG_FRAME_TIMING='1' if args.timing else '0',TG_FRAME_TIMING_EVERY='1',
        TG_DRAW_COLLECT_CHUNK='64',TG_REQUIRE_CROSS_PASS='1')
    env.pop('TG_DRAW_COLLECT',None)
    if args.gpu:env['TG_GPU_TIMING']='1'
    else:env.pop('TG_GPU_TIMING',None)
    cpu_profile=getattr(args,'cpu_profile',False)
    if cpu_profile:env.update(TG_PROFILE_OUT=str(directory/'cpu-stacks.folded'),TG_PROFILE_DELAY='20',TG_PROFILE_SECS='10')
    binary_sha256=hashlib.sha256(args.binary.read_bytes()).hexdigest()
    build_path,build_metadata=frozen_build(binary_sha256)
    evidence=getattr(args,'parallel_evidence',None)
    if not args.timing:
        if evidence is None:raise RuntimeError('Run --timing first, then provide its run.json with --parallel-evidence for uninstrumented measurements')
        require_parallel_evidence(json.loads(evidence.read_text()),binary_sha256)
    log=directory/'runtime.log';start=time.monotonic();cpu=[];system=[];sample=None
    with log.open('w') as output:
        process=subprocess.Popen([str(args.binary)],cwd=ROOT,env=env,stdout=output,stderr=subprocess.STDOUT)
        metadata={'pid':process.pid,'binary':str(args.binary),'binary_sha256':binary_sha256,
            'parallel_evidence':str(evidence) if evidence else None,
            'frozen_build_manifest':str(build_path),
            'audited_source_archive_sha256':build_metadata['audited_source_archive_sha256'],
            'scenario':args.scenario,'mode':args.mode,'warmup':args.warmup,'frames':args.frames,'stack_sampling':args.sample,
            'gpu_timestamps':args.gpu,'stage_timing':args.timing,'cpu_sampling':cpu_profile,'started_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),
            'workspace_sha256_at_start':{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in
                [ROOT/'Cargo.lock', ROOT/'crates/tiny-glade/tests/original_loading_present.rs', ROOT/'crates/tiny-glade/src/production_renderer.rs']},
            'environment':{k:v for k,v in env.items() if k.startswith(('TG_','RHAPSODY_','VK_','DYLD_','MVK_'))}}
        (directory/'run.json').write_text(json.dumps(metadata,indent=2))
        sampled=False
        system_at=0.
        while process.poll() is None:
            elapsed=time.monotonic()-start
            if elapsed>args.timeout+30:
                process.terminate()
                try:process.wait(timeout=15)
                except subprocess.TimeoutExpired:process.kill();process.wait()
                break
            result=subprocess.run(['/bin/ps','-p',str(process.pid),'-o','time=,rss='],capture_output=True,text=True)
            if result.returncode==0:
                observed_frames=re.findall(r'PERF_FRAME frame=(\d+)',log.read_text(errors='replace'))
                cpu.append({'elapsed_s':elapsed,'last_observed_world_frame':int(observed_frames[-1]) if observed_frames else 0,'cpu_time_and_rss_kib':result.stdout.strip()})
            if elapsed>=system_at:
                snapshot=subprocess.run(['/bin/ps','-axo','pid=,pcpu=,comm='],capture_output=True,text=True)
                system.append({'elapsed_s':elapsed,'process_cpu':snapshot.stdout,'exit_code':snapshot.returncode})
                system_at=elapsed+10
            if args.sample and not sampled and 'PERF_WORKLOAD frame=400 ' in log.read_text(errors='replace'):
                sampled=True
                with (directory/'sample-command.log').open('w') as sample_log:
                    sample=subprocess.Popen(['/usr/bin/sample',str(process.pid),'5','1','-file',str(directory/'stacks.txt')],stdout=sample_log,stderr=subprocess.STDOUT)
            time.sleep(1)
    if sample is not None:
        metadata['sample_exit_code']=sample.wait(timeout=15)
        if (directory/'stacks.txt').exists():metadata['stack_sha256']=hashlib.sha256((directory/'stacks.txt').read_bytes()).hexdigest()
    metadata.update(exit_code=process.returncode,elapsed_s=time.monotonic()-start,log_sha256=hashlib.sha256(log.read_bytes()).hexdigest(),cpu=cpu_summary(cpu,args.warmup,args.frames))
    (directory/'cpu-snapshots.json').write_text(json.dumps(cpu,indent=2))
    (directory/'system-snapshots.json').write_text(json.dumps(system,indent=2))
    if process.returncode==0:
        try:
            metadata['measurement']=summarize(log,args.warmup,args.frames)
            if args.gpu and not metadata['measurement']['gpu_windows_with_samples']:raise RuntimeError('No completed GPU timestamp samples; empty capture is not zero GPU time')
            if args.timing:
                if metadata['measurement']['render_stage_rows']!=args.frames:raise RuntimeError('Stage timings did not cover every measured render receipt')
                require_parallel_evidence(metadata,binary_sha256)
                metadata['parallel_acceptance']=True
        except Exception as error:metadata['analysis_error']=str(error)
    if args.sample and (sample is None or sample.returncode!=0 or not (directory/'stacks.txt').exists()):metadata['analysis_error']='Requested process stack sampling did not complete successfully'
    if cpu_profile:
        folded=directory/'cpu-stacks.folded'
        if not folded.exists() or not folded.stat().st_size:metadata['analysis_error']='Requested CPU profile did not produce samples'
        else:metadata['cpu_profile_sha256']=hashlib.sha256(folded.read_bytes()).hexdigest()
    (directory/'run.json').write_text(json.dumps(metadata,indent=2))
    print(json.dumps({'directory':str(directory),'exit_code':process.returncode,'measurement':metadata.get('measurement'),'analysis_error':metadata.get('analysis_error')}),flush=True)
    if process.returncode or metadata.get('analysis_error'):raise SystemExit(1)

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--binary',type=pathlib.Path,required=True)
    parser.add_argument('--out',type=pathlib.Path,required=True);parser.add_argument('--install',type=pathlib.Path,default=ROOT.parent/'B版本内容')
    parser.add_argument('--scenario',choices=['static','many_draws','local_edit','continuous_edit','loading_switch'],default='static')
    parser.add_argument('--mode',choices=['parallel'],default='parallel');parser.add_argument('--rounds',type=int,default=3)
    parser.add_argument('--warmup',type=int,default=300);parser.add_argument('--frames',type=int,default=3000)
    parser.add_argument('--timeout',type=int,default=900);parser.add_argument('--sample',action='store_true');parser.add_argument('--gpu',action='store_true');parser.add_argument('--timing',action='store_true')
    parser.add_argument('--cpu-profile',action='store_true')
    parser.add_argument('--parallel-evidence',type=pathlib.Path,help='Successful --timing run.json for the same frozen binary; required when timing is disabled')
    arguments=parser.parse_args();arguments.binary=arguments.binary.resolve();arguments.out=arguments.out.resolve();arguments.out.mkdir(parents=True,exist_ok=True)
    for iteration in range(1,arguments.rounds+1):
        with pathlib.Path(__file__).with_name('.measurement.lock').open('a') as guard:
            fcntl.flock(guard,fcntl.LOCK_EX)
            run(arguments,iteration)
