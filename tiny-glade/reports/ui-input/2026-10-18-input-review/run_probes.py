"""Run review probes against the current crate without retaining a test target."""
from pathlib import Path
import subprocess

report = Path(__file__).resolve().parent
workspace = report.parents[1]
target = workspace / "crates/country-core/tests/input_review_probe_20261018.rs"
source = (report / "probes.rs").read_bytes()
# Exclusive creation prevents overwriting an existing task's file.
with target.open("xb") as output:
    output.write(source)
try:
    with (report / "probes.log").open("w") as log:
        result = subprocess.run(
            ["cargo", "test", "-p", "country-core", "--test",
             "input_review_probe_20261018", "--offline", "--locked", "--",
             "--nocapture", "--test-threads=1"],
            cwd=workspace, stdout=log, stderr=subprocess.STDOUT,
        )
finally:
    if target.read_bytes() == source:
        target.unlink()
print(f"Probe log: {report / 'probes.log'}; exit={result.returncode}")
raise SystemExit(result.returncode)
