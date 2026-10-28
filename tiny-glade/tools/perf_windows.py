"""Reproducible Windows build / freeze / run / compare entry point (stdlib only)."""
import argparse
import csv
import ctypes
import hashlib
import json
import math
import os
from pathlib import Path
import platform
import re
import shutil
import subprocess
import sys
import time
from perf_etw import account

ROOT = Path(__file__).resolve().parents[1]
PERF = ROOT / '.perf'
SCENES = ('static', 'many_draws', 'local_edit', 'continuous_edit')
INSTALL = Path(os.environ.get('TG_INSTALL_ROOT', r'D:\game\ljxsj_92385\Tiny Glade'))
XPERF = Path(r'C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit\xperf.exe')


def dump(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, ensure_ascii=False)+'\n', encoding='utf-8')


def digest(path):
    with path.open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def git(*args):
    return subprocess.check_output(['git', '-c', 'safe.directory='+ROOT.as_posix(), '-C', str(ROOT), *args], text=True).strip()


def environment():
    result = os.environ.copy()
    result['TG_INSTALL_ROOT'] = str(INSTALL)
    result['RUST_LOG'] = 'warn'
    for name in ('TG_FRAME_TIMING', 'TG_FRAME_TIMING_EVERY', 'TG_GPU_TIMING', 'TG_PROFILE_OUT',
                 'TG_STARTUP_VISUAL_TRACE', 'TG_PERF_DIAGNOSTICS', 'TG_DRAW_COLLECT_CHUNK'):
        result.pop(name, None)
    return result


def label(value):
    if not re.fullmatch(r'[A-Za-z0-9_-]+', value):
        raise ValueError('label must contain only letters, numbers, underscore or hyphen')
    return value


def build(args):
    destination = PERF/'builds'/label(args.label)
    if destination.exists():
        raise ValueError('frozen build already exists: '+str(destination))
    status = git('status', '--porcelain')
    source_commit = git('rev-parse','HEAD')
    if status and not args.allow_dirty:
        raise ValueError('commit source before freezing a build; --allow-dirty is diagnostic only')
    destination.mkdir(parents=True)
    command = ['cargo', 'build', '-p', 'tiny-glade', '--release', '--test', 'original_loading_present',
               '--features', 'cpu-probe', '--locked', '--offline', '-j'+str(args.jobs), '--message-format=json-render-diagnostics']
    with (destination/'build.jsonl').open('w', encoding='utf-8') as output, (destination/'build.stderr.log').open('w', encoding='utf-8') as errors:
        code = subprocess.run(command, cwd=ROOT, env=environment(), stdout=output, stderr=errors).returncode
    if code:
        raise RuntimeError('release build failed; see '+str(destination/'build.stderr.log'))
    if git('rev-parse','HEAD') != source_commit or git('status','--porcelain') != status:
        raise RuntimeError('source changed during build; do not freeze this executable')
    executable = None
    for line in (destination/'build.jsonl').read_text(encoding='utf-8').splitlines():
        try:
            event = json.loads(line)
        except ValueError:
            continue
        if event.get('reason') == 'compiler-artifact' and event.get('executable'):
            executable = Path(event['executable'])
    if executable is None:
        raise RuntimeError('Cargo did not report executable')
    frozen = destination/executable.name
    shutil.copy2(executable, frozen)
    pdb = executable.with_suffix('.pdb')
    if pdb.is_file():
        shutil.copy2(pdb, destination/pdb.name)
    for dll in ('fmod.dll', 'fmodstudio.dll'):
        shutil.copy2(INSTALL/dll, destination/dll)
    record = {'schema':'windows-perf-build-v1', 'label':args.label, 'source_commit':source_commit,
              'dirty':bool(status), 'executable':str(frozen), 'executable_sha256':digest(frozen),
              'pdb_sha256':digest(destination/pdb.name) if pdb.is_file() else None,
              'cargo_lock_sha256':digest(ROOT/'Cargo.lock'), 'command':command,
              'features':['default(mimalloc)','cpu-probe'], 'os':platform.platform(),
              'cpu':platform.processor(), 'logical_cpus':os.cpu_count(), 'install_root':str(INSTALL),
              'rustc':subprocess.check_output(['rustc','-Vv'], text=True)}
    vulkaninfo = Path(shutil.which('vulkaninfo') or str(Path(os.environ.get('VULKAN_SDK',''))/'Bin/vulkaninfo.exe'))
    if vulkaninfo.is_file():
        result = subprocess.run([str(vulkaninfo),'--summary'],capture_output=True,text=True)
        (destination/'vulkan-summary.txt').write_text(result.stdout+result.stderr,encoding='utf-8')
        record['vulkan_info_exit_code'] = result.returncode
        record['vulkan_summary_sha256'] = digest(destination/'vulkan-summary.txt')
    subprocess.run(['git','-c','safe.directory='+ROOT.as_posix(),'-C',str(ROOT),
                    'archive','--format=zip','--output='+str(destination/'source.zip'),'HEAD'],check=True)
    dump(destination/'build.json',record)
    print(json.dumps(record,ensure_ascii=False),flush=True)


def calibrate(args):
    from ctypes import wintypes
    kernel = ctypes.WinDLL('kernel32',use_last_error=True)
    kernel.GetCurrentThread.restype = wintypes.HANDLE
    kernel.GetThreadTimes.argtypes = [wintypes.HANDLE]+[ctypes.POINTER(wintypes.FILETIME)]*4
    handle = kernel.GetCurrentThread()
    def cpu():
        values = [wintypes.FILETIME() for _ in range(4)]
        if not kernel.GetThreadTimes(handle,*[ctypes.byref(value) for value in values]):
            raise ctypes.WinError(ctypes.get_last_error())
        return sum((value.dwHighDateTime<<32)|value.dwLowDateTime for value in values[2:])*100
    before = cpu()
    time.sleep(.2)
    sleep_cpu = cpu()-before
    previous = before = cpu()
    positive, zero, regressions = [],0,0
    wall_start = time.perf_counter()
    while time.perf_counter()-wall_start < .5:
        now = cpu()
        difference = now-previous
        previous = now
        if difference > 0:
            positive.append(difference)
        elif difference == 0:
            zero += 1
        else:
            regressions += 1
    busy_cpu = cpu()-before
    result = {'schema':'windows-cpu-clock-calibration-v1','sleep_cpu_ns':sleep_cpu,
              'busy_cpu_ns':busy_cpu,'min_positive_delta_ns':min(positive) if positive else None,
              'positive_steps':len(positive),'zero_steps':zero,'regressions':regressions,
              'per_frame_clock_usable':bool(positive) and min(positive)<=1_000_000 and not regressions,
              'mean_call_wall_ns':(time.perf_counter()-wall_start)*1e9/(len(positive)+zero+regressions),
              'required_percentile_source':'ETW CSwitch QPC'}
    dump(PERF/'clock-calibration.json',result)
    print(json.dumps(result),flush=True)


def official_totals(session, csv_path, etl_path, destination):
    """Independent Microsoft scheduling integrator; ranges are integer microseconds."""
    base = None
    with csv_path.open(encoding='utf-8-sig',errors='replace') as source:
        for line in source:
            match = re.match(r'^#\s+\d+\s+#\s+(0x[0-9a-fA-F]+)',line)
            if match:
                base = int(match[1],16)
                break
    if base is None:
        raise ValueError('ETW QPC origin absent')
    independent = {}
    for role in ('render','main'):
        owner = session[role]
        begin = math.floor((owner['completion_points'][0]['qpc']-base)*1_000_000/owner['qpc_frequency'])
        end = math.ceil((owner['completion_points'][-1]['qpc']-base)*1_000_000/owner['qpc_frequency'])
        official = destination/('official-'+role+'.csv')
        subprocess.run([str(XPERF),'-i',str(etl_path),'-o',str(official),
            '-a','cswitch','-util','-thread','-exc_dpcisr','-range',str(begin),str(end)],check=True)
        found = []
        for line in official.read_text(encoding='utf-8-sig',errors='replace').splitlines():
            row = [cell.strip() for cell in next(csv.reader([line],skipinitialspace=True))]
            if len(row)==3 and row[0].isdigit() and row[2].isdigit() and int(row[2])==owner['thread_id']:
                found.append(int(row[0])*1000)
        if len(found)!=1:
            raise ValueError('independent xperf thread accounting missing/ambiguous')
        independent[role] = found[0]
    return independent


def run(args):
    frozen_path = PERF/'builds'/label(args.label)/'build.json'
    frozen = json.loads(frozen_path.read_text(encoding='utf-8'))
    executable = Path(frozen['executable'])
    if digest(executable) != frozen['executable_sha256']:
        raise ValueError('frozen executable checksum changed')
    destination = PERF/'runs'/label(args.run_label)
    destination.mkdir(parents=True,exist_ok=False)
    env = environment()
    env.update(TG_VALIDATE_GAMEPLAY_WSI='1', TG_PERF_SCENARIO=args.scene,
        TG_CPU_PROBE_OUT=str(destination/'session.json'), TG_CPU_PROBE_WARMUP=str(args.warmup),
        TG_CPU_PROBE_SAMPLES=str(args.samples), TG_VALIDATE_WORLD_FRAMES=str(args.warmup+args.samples+12),
        TG_VALIDATE_TIMEOUT_SECONDS=str(args.timeout), TG_VALIDATE_WORLD_SECONDS=str(args.seconds))
    trace_started = False
    record = {'schema':'windows-perf-run-v1', 'scene':args.scene, 'build':frozen,
              'warmup':args.warmup,'samples':args.samples,'etw':not args.without_etw,
              'start_utc':time.strftime('%Y-%m-%dT%H:%M:%SZ',time.gmtime())}
    try:
        if not args.without_etw:
            loggers = subprocess.check_output([str(XPERF),'-loggers'],text=True)
            if re.search(r'^Logger Name\s*:\s*NT Kernel Logger\s*$',loggers,re.M):
                raise RuntimeError('another NT Kernel Logger is active; do not replace it')
            flags = 'PROC_THREAD+CSWITCH+DISPATCHER+DPC+INTERRUPT'
            command = [str(XPERF),'-on',flags,'-ClockType','PerfCounter','-BufferSize','256',
                       '-MinBuffers','64','-MaxBuffers','512','-f',str(destination/'schedule.etl')]
            subprocess.run(command,check=True,stdout=subprocess.DEVNULL)
            trace_started = True
        with (destination/'window.log').open('w',encoding='utf-8') as log:
            record['exit_code'] = subprocess.run([str(executable)],cwd=ROOT,env=env,
                stdout=log,stderr=subprocess.STDOUT,timeout=args.timeout+30).returncode
    except Exception as error:
        record['error'] = repr(error)
        raise
    finally:
        if trace_started:
            record['etw_stop_exit_code'] = subprocess.run([str(XPERF),'-stop','NT Kernel Logger'],
                stdout=subprocess.DEVNULL).returncode
        dump(destination/'run.json',record)
    if record['exit_code']:
        raise RuntimeError('window validation failed: '+str(destination/'window.log'))
    session = json.loads((destination/'session.json').read_text(encoding='utf-8'))
    if not args.without_etw:
        csv_path = destination/'schedule.csv'
        with (destination/'xperf.log').open('w',encoding='utf-8') as log:
            subprocess.run([str(XPERF),'-i',str(destination/'schedule.etl'),'-o',str(csv_path),
                            '-a','dumper','-add_rawdata'],stdout=log,stderr=subprocess.STDOUT,check=True)
        # An independent implementation of OS scheduling accounting checks our
        # integration. GetThreadTimes remains visible but is tick sampled on
        # some drivers/Windows builds, not an exact reference for short bursts.
        independent = official_totals(session,csv_path,destination/'schedule.etl',destination)
        precise = account(session,csv_path,independent)
        dump(destination/'cpu.json',precise)
        record['valid'] = precise['valid'] and not frozen['dirty']
        record['render'] = {key:value for key,value in precise['render'].items() if key!='frames'}
        record['main'] = {key:value for key,value in precise['main'].items() if key!='frames'}
    else:
        record['valid'] = False
        record['diagnostic_only'] = 'No ETW; coarse OS clock cannot establish percentiles'
    inputs = []
    for line in (destination/'window.log').read_text(encoding='utf-8',errors='replace').splitlines():
        if line.startswith('PERF_INPUT_SCENE '):
            inputs.append(json.loads(line.partition(' ')[2]))
    if len(inputs)!=1:
        raise ValueError('missing/duplicate semantic input manifest')
    record['input'] = inputs[0]
    record['input_sha256'] = hashlib.sha256(json.dumps(inputs[0],sort_keys=True).encode()).hexdigest()
    dump(destination/'run.json',record)
    print(json.dumps({key:value for key,value in record.items() if key not in {'input','build'}},ensure_ascii=False),flush=True)
    if not record['valid'] and not args.without_etw:
        raise RuntimeError('CPU/OS crosscheck failed; result cannot be accepted')


def compare(args):
    baseline = json.loads((PERF/'runs'/label(args.baseline)/'run.json').read_text(encoding='utf-8'))
    candidate = json.loads((PERF/'runs'/label(args.candidate)/'run.json').read_text(encoding='utf-8'))
    if not baseline.get('valid') or not candidate.get('valid'):
        raise ValueError('both runs must be valid')
    if baseline['scene']!=candidate['scene'] or baseline['input_sha256']!=candidate['input_sha256']:
        raise ValueError('semantic workload mismatch')
    for field in ('os','logical_cpus','features','cargo_lock_sha256','vulkan_summary_sha256'):
        if baseline['build'].get(field) != candidate['build'].get(field):
            raise ValueError('build/hardware identity mismatch: '+field)
    ratios = {name:candidate['render'][name]/baseline['render'][name]-1
              for name in ('mean_ms','p50_ms','p95_ms','p99_ms')}
    result = {'baseline':args.baseline,'candidate':args.candidate,'relative_change':ratios,
              'p95_16ms':candidate['render']['p95_ms']<=16,
              'regression_gate':ratios['p95_ms']<=.05 and ratios['p99_ms']<=.05,
              'scope':'one pair; three alternating pairs required for acceptance'}
    dump(PERF/'comparisons'/(args.baseline+'__'+args.candidate+'.json'),result)
    print(json.dumps(result),flush=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest='command',required=True)
    commands.add_parser('calibrate').set_defaults(function=calibrate)
    p=commands.add_parser('build');p.add_argument('label');p.add_argument('--allow-dirty',action='store_true')
    p.add_argument('--jobs',type=int,choices=range(1,33),default=1);p.set_defaults(function=build)
    p=commands.add_parser('run');p.add_argument('label');p.add_argument('run_label');p.add_argument('--scene',choices=SCENES,required=True)
    p.add_argument('--warmup',type=int,default=600);p.add_argument('--samples',type=int,default=3000)
    p.add_argument('--seconds',type=int,default=0);p.add_argument('--timeout',type=int,default=1800)
    p.add_argument('--without-etw',action='store_true');p.set_defaults(function=run)
    p=commands.add_parser('compare');p.add_argument('baseline');p.add_argument('candidate');p.set_defaults(function=compare)
    args=parser.parse_args()
    args.function(args)


if __name__=='__main__':
    main()
