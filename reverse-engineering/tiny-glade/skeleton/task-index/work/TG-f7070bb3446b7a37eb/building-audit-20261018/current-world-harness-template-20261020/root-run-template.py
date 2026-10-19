#!/usr/bin/env python3
"""Root-only run template. Default prints plan; --execute is required for Cargo/tests."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys

here = Path(__file__).resolve().parent
parser = argparse.ArgumentParser()
parser.add_argument("--execute", action="store_true")
parser.add_argument("--output-dir", type=Path)
args = parser.parse_args()
manifest = json.loads((here / "inputs.json").read_text())
workspace = Path(manifest["workspace"])
build = ["cargo", "test", "-p", "country-core", "--lib", "--features", "native-physics",
         "--locked", "--offline", "--no-run", "--message-format=json-render-diagnostics"]
plan = {"build": build, "cwd": str(workspace),
        "module_filters": manifest["expected_test_modules"],
        "scope": "Current complete country-core, world.rs and actual canonical types; no replacement functions/old core rlib"}
if not args.execute:
    print(json.dumps(plan, indent=2))
    sys.exit(0)
if args.output_dir is None:
    parser.error("--execute requires a root-owned --output-dir")
output = args.output_dir.resolve()
output.mkdir(parents=True, exist_ok=False)

def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def verify_inputs():
    changed = [entry["path"] for entry in manifest["inputs"]
               if sha(entry["path"]) != entry["sha256"]]
    if changed:
        raise RuntimeError("Frozen input mismatch; require a separately reviewed update: " + repr(changed))

report = {"plan": plan, "status": "not_started", "runs": [], "source_gate": False}
try:
    verify_inputs()
    report["source_gate"] = True
    (output / "plan.json").write_text(json.dumps(plan, indent=2) + "\n")
    built = subprocess.run(build, cwd=workspace, capture_output=True, text=True)
    (output / "build.jsonl").write_text(built.stdout)
    (output / "build.stderr").write_text(built.stderr)
    report["build_returncode"] = built.returncode
    if built.returncode:
        raise RuntimeError("Actual complete country-core did not compile; retain diagnostics, no mock/rlib fallback")
    records = []
    for line in built.stdout.splitlines():
        try:
            record = json.loads(line)
        except json.JSONDecodeError:
            continue
        if record.get("reason") == "compiler-artifact":
            records.append(record)
    artifacts = [r for r in records if r.get("executable")
                 and r.get("target", {}).get("name") == "country_core"
                 and r.get("profile", {}).get("test")
                 and Path(r["target"]["src_path"]).resolve()
                     == workspace / "crates/country-core/src/lib.rs"]
    if len(artifacts) != 1:
        raise RuntimeError("Require unique actual current country-core unit-test artifact")
    executable = Path(artifacts[0]["executable"])
    report["artifact"] = {"path": str(executable), "sha256": sha(executable),
                          "cargo_record": artifacts[0]}
    verify_inputs()
    listed = subprocess.run([str(executable), "--list", "--format=terse"], capture_output=True, text=True)
    (output / "test-list.txt").write_text(listed.stdout)
    (output / "test-list.stderr").write_text(listed.stderr)
    if listed.returncode:
        raise RuntimeError("Actual test binary list failed")
    names = [line[:-6] for line in listed.stdout.splitlines() if line.endswith(": test")]
    for module, count in manifest["expected_test_modules"].items():
        selected = [name for name in names if name.startswith(module)]
        if len(selected) != count:
            raise RuntimeError("Missing/mismatched current focused test registrations: " + module)
        command = [str(executable), module, "--nocapture", "--test-threads=1"]
        ran = subprocess.run(command, capture_output=True, text=True)
        label = module.rstrip(":").split("::")[-1]
        (output / (label + ".stdout")).write_text(ran.stdout)
        (output / (label + ".stderr")).write_text(ran.stderr)
        counts = re.search(r"(\d+) passed; (\d+) failed; (\d+) ignored", ran.stdout)
        run = {"command": command, "returncode": ran.returncode, "tests": selected,
               "expected_count": count,
               "counts": list(map(int, counts.groups())) if counts else None}
        report["runs"].append(run)
        if ran.returncode or run["counts"] != [count, 0, 0]:
            raise RuntimeError("Focused test group failed or did not execute every expected test: " + module)
    verify_inputs()
    report["status"] = "passed_current_source_focused_scope"
except Exception as error:
    report["status"] = "failed"
    report["error"] = str(error)
finally:
    (output / "result.json").write_text(json.dumps(report, indent=2) + "\n")
if report["status"] != "passed_current_source_focused_scope":
    sys.exit(1)
