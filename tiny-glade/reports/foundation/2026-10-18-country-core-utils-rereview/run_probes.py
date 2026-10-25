"""用当前 core 运行审查反例，并移除临时集成测试文件。"""
from pathlib import Path
import hashlib
import json
import subprocess

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
CORE = ROOT / "crates/country-core"
target = CORE / "tests/utils_rereview_20261018.rs"
source = (HERE / "probes.rs").read_bytes()
inputs = [*sorted((CORE / "src/utils").rglob("*.rs")),
          *[CORE / "src" / path for path in [
              "systems/ui/resources/slider3d.rs", "systems/ui/resources/ui_state.rs",
              "systems/ui/signifiers/signifier_create.rs", "systems/ui/signifiers/signifier_mark_for_delete.rs",
              "systems/render_loop/draw_passes/draw_callbacks_a.rs",
              "systems/render_loop/draw_passes/draw_callbacks_a_views.rs", "components/rendering.rs"]]]
def snapshot():
    return {str(path.relative_to(ROOT)): hashlib.sha256(path.read_bytes()).hexdigest() for path in inputs}
before = snapshot()
target.parent.mkdir(exist_ok=True)
with target.open("xb") as output:
    output.write(source)
command = ["cargo", "test", "--offline", "--locked", "--target-dir", "/tmp/tg-country-core-utils-review-20261018-target",
           "-p", "country-core", "--test", target.stem, "--", "--test-threads=1", "--nocapture"]
try:
    with (HERE / "probes.log").open("w") as output:
        output.write("$ " + " ".join(command) + "\n")
        output.flush()
        result = subprocess.run(command, cwd=ROOT, stdout=output, stderr=subprocess.STDOUT)
finally:
    if target.read_bytes() == source:
        target.unlink()
    else:
        raise RuntimeError(f"临时文件被外部修改，保留：{target}")
after = snapshot()
(HERE / "verification.json").write_text(json.dumps({
    "command": command, "exit_code": result.returncode, "inputs_sha256": before,
    "inputs_unchanged": before == after, "scope": "审查反例；未修改业务代码，未验证真实 GPU 设备",
}, ensure_ascii=False, indent=2) + "\n")
if before != after:
    raise SystemExit("审查期间输入源码发生变化")
raise SystemExit(result.returncode)
