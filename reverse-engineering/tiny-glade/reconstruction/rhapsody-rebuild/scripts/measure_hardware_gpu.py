"""Windows 真 GPU 基准入口：保留退出码，验证失败也继续收集性能数据。"""
import argparse
import hashlib
import json
import os
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--runs', type=int, default=3)
    args = parser.parse_args()
    if not 1 <= args.runs <= 100:
        parser.error('--runs must be between 1 and 100')
    destination = ROOT / 'evidence' / datetime.now().strftime('hardware-gpu-%Y-%m-%d-%H%M%S')
    destination.mkdir(parents=True)
    commands = []

    def invoke(argv, log):
        print('Running:', ' '.join(map(str, argv)), flush=True)
        result = subprocess.run(argv, cwd=ROOT, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        stdout = result.stdout.decode('utf-8', errors='replace')
        stderr = result.stderr.decode('utf-8', errors='replace')
        (destination / log).write_text(stderr + '\n' + stdout, encoding='utf-8')
        commands.append(dict(argv=list(map(str, argv)), exit_code=result.returncode, log=log))
        (destination / 'commands.json').write_text(json.dumps(commands, indent=2), encoding='utf-8')
        return result.returncode, stdout

    host_query = """[Console]::OutputEncoding = [System.Text.UTF8Encoding]::new();
@{gpu=@(Get-CimInstance Win32_VideoController | Select-Object Name,DriverVersion,AdapterRAM);
cpu=@(Get-CimInstance Win32_Processor | Select-Object Name,NumberOfCores,NumberOfLogicalProcessors);
os=@(Get-CimInstance Win32_OperatingSystem | Select-Object Caption,Version)} | ConvertTo-Json -Depth 6"""
    code, stdout = invoke(['powershell.exe', '-NoProfile', '-Command', host_query], 'host-query.log')
    if code:
        raise RuntimeError('Host inventory failed')
    host = json.loads(stdout.lstrip('\ufeff'))
    host.update(started_utc=datetime.now(timezone.utc).isoformat(), runs=args.runs, warmup_runs=1,
                environment={k: v for k, v in os.environ.items() if k.startswith(('VK_', 'VULKAN', 'RHAPSODY_', 'CARGO_', 'RUST'))})
    for executable in ['cargo', 'rustc']:
        code, output = invoke([executable, '-V'], executable + '-version.log')
        if code:
            raise RuntimeError(f'{executable} version query failed')
        host[executable] = output.strip()
    (destination / 'host.json').write_text(json.dumps(host, ensure_ascii=False, indent=2), encoding='utf-8')
    paths = sorted(p for folder in ['src', 'benches', 'examples', 'tests'] for p in (ROOT / folder).rglob('*.rs'))
    hashes = [dict(path=str(p.relative_to(ROOT)), sha256=hashlib.sha256(p.read_bytes()).hexdigest()) for p in paths]
    (destination / 'source-hashes.json').write_text(json.dumps(hashes, indent=2), encoding='utf-8')
    paths = [ROOT/'Cargo.toml', ROOT/'Cargo.lock', ROOT/'../../tools/render-passes/self_test/blur_mod0.spv']
    hashes = [dict(path=os.path.relpath(p, ROOT), sha256=hashlib.sha256(p.read_bytes()).hexdigest()) for p in paths]
    (destination / 'input-hashes.json').write_text(json.dumps(hashes, indent=2), encoding='utf-8')
    code, inventory = invoke(['cargo', 'run', '--locked', '--release', '--example', 'device_inventory'], 'inventory-build.log')
    if code:
        raise RuntimeError('Vulkan device inventory failed')
    devices = json.loads(inventory)['devices']
    if len(devices) != 1 or any(word in devices[0]['name'].lower() for word in ['lavapipe', 'llvmpipe', 'software', 'microsoft basic']):
        raise RuntimeError('Requires one unambiguous hardware Vulkan device')
    (destination / 'device-inventory.json').write_text(inventory, encoding='utf-8')
    for index in range(args.runs + 1):
        last = ROOT / 'perf/last.json'
        previous = last.stat().st_mtime_ns if last.exists() else None
        code, _ = invoke(['cargo', 'bench', '--locked', '--bench', 'perf', '--', '--check'], f'perf-{index}.log')
        if not last.exists() or last.stat().st_mtime_ns == previous:
            raise RuntimeError(f'Benchmark {index} did not produce fresh results')
        (destination / f'perf-{index}.json').write_bytes(last.read_bytes())
        if code or 'gpu/frame_loop/passes=8' not in json.loads(last.read_text())['results']:
            raise RuntimeError(f'Benchmark {index} failed or skipped the GPU')
    codes = []
    for argv, log in [
        (['cargo', 'test', '--locked', '--release', '--test', 'real_pass_blur', '--test', 'perf_guards', '--', '--nocapture'], 'correctness.log'),
        (['cargo', 'run', '--locked', '--release', '--example', 'pipeline_probe'], 'pipeline-probe.log'),
        (['cargo', 'run', '--locked', '--release', '--example', 'blur_gpu_perf'], 'blur-gpu-perf.log'),
        (['cargo', 'clippy', '--locked', '--release', '--example', 'blur_gpu_perf', '--', '-D', 'warnings'], 'clippy-blur.log'),
    ]:
        code, _ = invoke(argv, log)
        codes.append(code)
    code, _ = invoke([sys.executable, str(ROOT/'scripts/summarize-hardware-gpu.py'), str(destination)], 'summary.log')
    print(f'RESULT_DIRECTORY={destination}', flush=True)
    return int(bool(code or any(codes)))


if __name__ == '__main__':
    sys.exit(main())
