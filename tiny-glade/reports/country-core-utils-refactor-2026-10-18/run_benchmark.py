"""重放矩形微基准，校验所用共享依赖产物没有变化。"""
from pathlib import Path
import hashlib
import json
import subprocess
import tempfile

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
metadata = json.loads((HERE / "source-checks.json").read_text())
arguments = []
for name in ["glam", "tabloid", "utils"]:
    paths = [Path(path) for path in metadata["dependencies_sha256"] if Path(path).name.startswith("lib" + name + "-")]
    if len(paths) != 1:
        raise SystemExit(f"无法唯一选择依赖：{name}")
    path = paths[0]
    if not path.exists() or hashlib.sha256(path.read_bytes()).hexdigest() != metadata["dependencies_sha256"][str(path)]:
        raise SystemExit("依赖产物已改变，请先运行 run_source_checks.py 更新限定源码检查")
    arguments.extend(["--extern", name + "=" + str(path)])
with tempfile.TemporaryDirectory(prefix="country-utils-benchmark-") as directory:
    binary = Path(directory) / "benchmark"
    command = ["rustc", "--edition=2024", "-O", str(HERE / "benchmark.rs"), "-L", "dependency=" + str(paths[0].parent), *arguments, "-o", str(binary)]
    with (HERE / "benchmark-replay.log").open("w") as output:
        output.write("$ " + " ".join(command) + "\n")
        output.flush()
        result = subprocess.run(command, cwd=ROOT, stdout=output, stderr=subprocess.STDOUT)
        if result.returncode == 0:
            result = subprocess.run([str(binary)], stdout=output, stderr=subprocess.STDOUT)
raise SystemExit(result.returncode)
