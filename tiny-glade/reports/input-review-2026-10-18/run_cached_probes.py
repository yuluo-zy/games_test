"""Re-run the exact compiled snapshot named in cached-artifacts.json."""
from pathlib import Path
import hashlib
import json
import subprocess

report = Path(__file__).resolve().parent
workspace = report.parents[1]
manifest = json.loads((report / "cached-artifacts.json").read_text())
binary = report / "cached-probe-rerun-bin"
command = [
    "rustc", "--edition=2024", "--test", str(report / "probes.rs"),
    "--crate-name", "input_review_cached_probe",
    "-L", f"dependency={workspace / 'target/debug/deps'}",
    "-o", str(binary),
]
for relative, expected in manifest["artifacts"].items():
    artifact = workspace / relative
    actual = hashlib.sha256(artifact.read_bytes()).hexdigest()
    if actual != expected:
        raise SystemExit(f"Cached artifact changed: {artifact}")
    name = artifact.name.removeprefix("lib").split("-", 1)[0]
    command += ["--extern", f"{name}={artifact}"]
for directory in (workspace / "target/debug/build").glob("*/out"):
    command += ["-L", f"native={directory}"]
try:
    with (report / "cached-probes-rerun.log").open("w") as log:
        result = subprocess.run(command, cwd=workspace, stdout=log, stderr=subprocess.STDOUT)
        if result.returncode == 0:
            result = subprocess.run(
                [str(binary), "--nocapture", "--test-threads=1"],
                cwd=workspace, stdout=log, stderr=subprocess.STDOUT,
            )
finally:
    binary.unlink(missing_ok=True)
raise SystemExit(result.returncode)
