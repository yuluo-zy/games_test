"""执行真实 Vulkan 管线冒烟并记录证据；它不是原游戏行为差分。"""
import datetime
import hashlib
import json
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
files = [ROOT / "Cargo.lock", ROOT / "Cargo.toml", ROOT / "examples/pipeline_probe.rs", Path(__file__)]
files += sorted((ROOT / "src").rglob("*.rs"))


def hashes():
    return {str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest() for p in files}


before = hashes()
result = subprocess.run(
    ["cargo", "run", "--offline", "--locked", "--manifest-path", str(ROOT / "Cargo.toml"), "--example", "pipeline_probe"],
    text=True, capture_output=True, check=True,
)
report = json.loads(result.stdout.strip().splitlines()[-1])
report["timestamp_utc"] = datetime.datetime.now(datetime.timezone.utc).isoformat()
report["limitations"] = [
    "最小空计算着色器的真实 GPU 接线验证；没有原游戏 shader 或画面对照",
    "descriptor 写入/绑定已调用实际 Vulkan；此着色器不读取该数据",
    "动态常量设备地址尚未启用",
    "原 shader 反射/管线 key/缓存淘汰策略尚未对照",
]
report["input_hashes"] = before
report["inputs_changed_during_run"] = before != hashes()
if report["inputs_changed_during_run"]:
    report["status"] = "stale_inputs"
(ROOT / "evidence/pipeline-vulkan-probe.json").write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
print(json.dumps({k: report[k] for k in ["status", "device_name", "gpu_dispatches", "descriptor_write", "inputs_changed_during_run"]}, ensure_ascii=False))
assert not report["inputs_changed_during_run"], "运行期间源码发生变化，请合并完成后重跑"
