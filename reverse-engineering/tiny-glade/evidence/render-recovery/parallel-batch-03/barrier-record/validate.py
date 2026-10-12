from pathlib import Path
import hashlib
import json
import subprocess
import sys

OUT = Path(__file__).resolve().parent
ROOT = OUT.parents[3]
SOURCE = ROOT / "reconstruction/render/work-packages/barrier-record-r5"
commands = [[sys.executable, str(OUT / script)] for script in ["verify.py", "verify-population.py", "audit.py", "check-snapshots.py"]]
commands += [
    ["cargo", "test", "--offline", "--locked", "--manifest-path", str(SOURCE / "Cargo.toml")],
    ["cargo", "clippy", "--offline", "--locked", "--all-targets", "--manifest-path", str(SOURCE / "Cargo.toml"), "--", "-D", "warnings"],
    ["rustfmt", "--check", "--edition", "2021", "--config", "skip_children=true", str(SOURCE / "src/lib.rs"), str(SOURCE / "src/main.rs")],
]
results = []
for command in commands:
    result = subprocess.run(command, text=True, capture_output=True)
    results.append({"command":command,"exit_code":result.returncode,"stdout":result.stdout,"stderr":result.stderr})
    if result.returncode != 0:
        print(result.stdout + result.stderr)
        raise SystemExit(result.returncode)
barrier = json.loads((OUT / "validation.json").read_text(encoding="utf8"))
population = json.loads((OUT / "population-validation.json").read_text(encoding="utf8"))
audit = json.loads((OUT / "negative-search-audit.json").read_text(encoding="utf8"))
snapshots = json.loads((OUT / "snapshot-contract-checks.json").read_text(encoding="utf8"))
report = {"passed": all(result["exit_code"] == 0 for result in results), "original_sha256":barrier["sha256"], "native_rust_barrier_cases":barrier["cases"], "native_population_cases":population["cases"], "snapshot_checks":len(snapshots["checks"]), "bounded_callers":audit["caller_count"], "candidate_sha256":hashlib.sha256((SOURCE / "src/lib.rs").read_bytes()).hexdigest(), "commands":results,"status":"scoped-native-differential-pass", "original_entry_scopes":["pre barriers","record initial prefix","populate resources"], "injected_scope":"post-record final/export gate only", "not_claimed":["full graph compile or record differential","Vulkan runtime validation","natural local-map reuse producer","whole-program negative search","original ABI/padding"], "write_scope":["parallel-batch-03/barrier-record","work-packages/barrier-record-r5"]}
(OUT / "validation-summary.json").write_text(json.dumps(report, indent=2), encoding="utf8")
print(json.dumps({key:report[key] for key in ["passed","native_rust_barrier_cases","native_population_cases","snapshot_checks","bounded_callers"]}))
