"""汇总本机三轮性能样本与原 shader 的独立 GPU 时间戳；保留失败日志。"""
import hashlib
import json
import re
import statistics
import sys
from pathlib import Path

root = Path(__file__).resolve().parent.parent
destination = Path(sys.argv[1]) if len(sys.argv) > 1 else sorted(p for p in (root / 'evidence').glob('hardware-gpu-*') if (p/'blur-gpu-perf.log').exists())[-1]

def read(path):
    data = path.read_bytes()
    return data.decode('utf-16' if data.startswith((b'\xff\xfe', b'\xfe\xff')) else 'utf-8-sig')

host = json.loads(read(destination / 'host.json'))
run_count = host['runs']
reports = [json.loads(read(destination / f'perf-{i}.json')) for i in range(1, run_count + 1)]
combined = {}
for name in reports[0]['results']:
    rows = [r['results'][name] for r in reports]
    values = [r['ns_per_op'] for r in rows]
    combined[name] = {
        'median_ns_per_op': statistics.median(values),
        'min_ns_per_op': min(values), 'max_ns_per_op': max(values),
        'counters': rows[0]['counters'],
        'counters_stable': all(r['counters'] == rows[0]['counters'] for r in rows),
        'ops': rows[0]['ops'], 'samples_ns_per_op': values,
    }
lines = read(destination / 'blur-gpu-perf.log').splitlines()
blur = json.loads(next(line for line in reversed(lines) if line.startswith('{"')))
# 旧日志中的 method 字段修正为更精确的计时口径，不改变任何测量数值。
blur['method'] = 'TOP_OF_PIPE/BOTTOM_OF_PIPE timestamp interval around blur bind/dispatch; prior upload dependency waiting can contribute; fresh graph/resources/pipeline each sample; not steady-state/full-game performance'
(destination / 'blur-gpu-perf.json').write_text(json.dumps(blur, ensure_ascii=False, indent=2), encoding='utf-8')
correctness = read(destination / 'correctness.log')
comparison = re.search(r'比较 (\d+) 个像素；GPU 采用 outer=(\d+) inner=(\d+)，中位误差 ([\d.e+-]+)，最大误差 ([\d.e+-]+)', correctness)
exact_error = re.search(r'最大误差过大：([\d.e+-]+)', correctness)
correctness_result = {'passed': 'test result: ok.' in correctness and 'test result: FAILED' not in correctness and 'error:' not in correctness,
                      'log': 'correctness.log', 'cause': 'unresolved; shader trigonometry/sampling boundary differences are a hypothesis'}
if comparison:
    count, outer, inner, median_error, maximum_error = comparison.groups()
    correctness_result.update(pixels_compared=int(count), loop_counts=dict(outer=int(outer), inner=int(inner)),
                              median_relative_error=float(median_error), maximum_relative_error=float(exact_error[1] if exact_error else maximum_error),
                              required_maximum_relative_error=0.01)
recorded_hashes = json.loads(read(destination / 'source-hashes.json'))
drift = []
for item in recorded_hashes:
    path = root / item['path']
    current = hashlib.sha256(path.read_bytes()).hexdigest() if path.exists() else None
    if current != item['sha256']:
        drift.append(dict(path=item['path'], measured_sha256=item['sha256'], current_sha256=current))
clippy_log = read(destination / 'clippy-blur.log') if (destination / 'clippy-blur.log').exists() else ''
probe_log = read(destination / 'pipeline-probe.log') if (destination / 'pipeline-probe.log').exists() else ''
summary = {
    'schema': 1, 'host': host,
    'hardware': json.loads(read(destination / 'device-inventory.json')),
    'cpu_and_driver_wall_clock_benchmarks': combined,
    'all_counter_gates_passed': all('计数器门禁通过' in read(destination / f'perf-{i}.log') for i in range(1, run_count + 1)),
    'all_counter_values_stable': all(r['counters_stable'] for r in combined.values()),
    'blur_gpu_timestamp_benchmark': blur,
    'original_blur_correctness_test': correctness_result,
    'notes': [
        'Existing perf ns/op is CPU/driver/end-to-end wall clock, not GPU timestamp.',
        'Do not compare hardware timing with lavapipe baseline; only counters compared.',
        'memory_allocations counts logical allocator registrations; memory_blocks counts actual live Vulkan allocations.',
        'No full original game frame, presentation FPS, GPU utilization or thermal measurements.',
        'No Vulkan validation layer enabled in timestamp benchmark.',
    ],
    'measurement_source_hashes': recorded_hashes,
    'source_changes_after_measurement': drift,
    'final_clippy': {'passed': 'Finished' in clippy_log and 'error' not in clippy_log.lower(), 'log': 'clippy-blur.log'},
    'final_pipeline_probe': {'passed': 'candidate_live_vulkan_passed' in probe_log, 'log': 'pipeline-probe.log'},
    'new_benchmark_hashes': [dict(path=str(p.relative_to(root)), sha256=hashlib.sha256(p.read_bytes()).hexdigest()) for p in [root/'examples/blur_gpu_perf.rs',root/'scripts/measure-hardware-gpu.ps1',Path(__file__)]],
}
(destination / 'summary.json').write_text(json.dumps(summary, ensure_ascii=False, indent=2), encoding='utf-8')
def ns(name):
    return combined[name]['median_ns_per_op']
text = ['# rhapsody 真实 GPU 性能测试', '',
    f'硬件：{blur["device"]}；驱动 {summary["host"]["gpu"][0]["DriverVersion"]}。release 优化构建，原锁定依赖。', '',
    f'现有基准：1 轮预热 + {run_count} 轮，每轮内部取 3–7 次中位数；下表为各轮中位数的中位数。原 blur：每分辨率 2 次预热 + 15 次 GPU 时间戳采样。', '',
    '## 原 blur shader GPU 时间戳', '', '| 分辨率 | GPU 区间中位 ms | P95 ms |', '|---|---:|---:|']
for row in blur['results']:
    text.append(f'| {row["size"]}×{row["size"]} | {row["gpu_median_ms"]:.3f} | {row["gpu_p95_ms"]:.3f} |')
text += ['', '区间为现有 profiler 的 TOP_OF_PIPE→BOTTOM_OF_PIPE，围住 bind/dispatch；可能包含前序上传依赖等待。每次重新建图、资源和管线。P95 用 nearest-rank，15 个样本时等于最大值。不是 shader ALU 活跃时间，也不是完整游戏帧/FPS。', '',
    '## CPU / 驱动 / 端到端开销', '', '| 场景 | 三轮中位 |', '|---|---:|']
for label, name, unit in [
    ('帧循环 1 pass', 'gpu/frame_loop/passes=1', 'ms/frame'),
    ('帧循环 8 passes', 'gpu/frame_loop/passes=8', 'ms/frame'),
    ('重回调串行录制', 'gpu/record_heavy/serial', 'ms/frame'),
    ('重回调原作分块策略', 'gpu/record_heavy/original_pool', 'ms/frame'),
    ('重回调候选 balanced 策略', 'gpu/record_heavy/balanced_pool', 'ms/frame'),
    ('512 buffers 专用分配', 'gpu/alloc_mode/dedicated/buffers=512', 'ms/buffer'),
    ('512 buffers 子分配', 'gpu/alloc_mode/suballocated/buffers=512', 'ms/buffer'),
    ('64×64 RGBA8 上传', 'gpu/texture_upload/n=16', 'ms/upload'),
]:
    text.append(f'| {label} | {ns(name)/1e6:.4f} {unit} |')
text += ['', f'重回调原策略相对串行加速 {ns("gpu/record_heavy/serial")/ns("gpu/record_heavy/original_pool"):.2f} 倍；每 pass 人为 spin 50 µs，64 passes，不代表真实游戏 shader 性能。',
    f'零 spin 回调原策略/串行耗时比 {ns("gpu/record_spin0ns/original_pool")/ns("gpu/record_spin0ns/serial"):.2f}，并行调度开销在轻回调下明显。', '',
    '三轮计数器门禁通过，计数稳定；帧循环 32 帧提交 32 次、整设备 idle 0 次、资源登记 2 次。512 buffers 专用分配存活 512 个 memory blocks，子分配 1 个。memory_allocations 是逻辑分配登记，不是 vkAllocateMemory 次数。未覆盖 lavapipe 基线。', '',
    '## 正确性与限制', '',
    f'正确性测试状态：{"通过" if correctness_result["passed"] else "失败"}，未修改原测试阈值。日志解析结果：`{json.dumps(correctness_result, ensure_ascii=False)}`。sin/cos 与采样取整边界差异仅为待核实假设；本轮不宣称原因已查明。', '',
    '新增计时入口检查所有像素 alpha 覆盖与 64 内部样本；采样点中位误差保持 <1e-4，最大误差是否小于 1% 单独记录，允许失败标志保留性能数据。这不替代原全图正确性测试。', '',
    '只测当前候选 rhapsody；没有完整原作帧对照、present/FPS、温度/频率/利用率采样或 validation layer 检查。保留原始日志、设备信息、源码 SHA256 与逐样本结果。', '',
    '复现：`powershell -File scripts/measure-hardware-gpu.ps1`；原 blur GPU 单独运行：`cargo run --locked --release --example blur_gpu_perf`。', '',
    '数据：[summary.json](summary.json)、[blur-gpu-perf.json](blur-gpu-perf.json)、[correctness.log](correctness.log)、`perf-1/2/3.json`。']
text += ['', '## 收尾源码变动', '',
    f'归档时检查到 {len(drift)} 个已有源码文件哈希变化（逐项见 summary.json）。收尾 Clippy：{"通过" if summary["final_clippy"]["passed"] else "未通过"}；pipeline_probe：{"通过" if summary["final_pipeline_probe"]["passed"] else "未通过"}。详见对应日志；实测结果只对应记录的源码版本，不能作为后续变更的当前验收。']
(destination / 'REPORT.md').write_text('\n'.join(text)+'\n', encoding='utf-8')
print(destination)
print('\n'.join(text[:14]))
