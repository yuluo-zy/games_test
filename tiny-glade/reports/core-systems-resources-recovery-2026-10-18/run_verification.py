#!/usr/bin/env python3
"""Run a bounded check and record source drift; never infer semantic acceptance."""
from pathlib import Path
import datetime, hashlib, json, subprocess, sys, time
ROOT = Path(__file__).resolve().parents[2]
REPORT = Path(__file__).resolve().parent

def fingerprint():
    paths = []
    # Cover all actual workspace crate sources, including public API consumers.
    # Each child is used as the traversal root so directory symlinks keep their
    # current workspace dependency source in scope as well.
    for base in sorted(p for p in (ROOT / "crates").iterdir() if p.is_dir()):
        paths.extend(p for p in base.rglob("*") if p.suffix in {".rs", ".json", ".bin", ".hlsl", ".wgsl"} and p.is_file())
        paths.extend(base.rglob("Cargo.toml"))
    paths.extend([ROOT / "Cargo.toml", ROOT / "Cargo.lock"])
    return {str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest()
            for p in sorted(set(paths)) if p.is_file()}

if __name__ == "__main__":
    label, *command = sys.argv[1:]
    if command and command[0] == "--": command.pop(0)
    assert label.replace("-", "").replace("_", "").isalnum() and command
    before = fingerprint()
    (REPORT / (label + "-sources-before.json")).write_text(json.dumps(before, indent=2) + "\n")
    started = datetime.datetime.now(datetime.timezone.utc).isoformat()
    monotonic = time.monotonic()
    with (REPORT / (label + ".log")).open("w") as log:
        process = subprocess.run(command, cwd=ROOT, stdout=log, stderr=subprocess.STDOUT)
    after = fingerprint()
    drift = [{"path": p, "before": before.get(p), "after": after.get(p)}
             for p in sorted(before.keys() | after.keys()) if before.get(p) != after.get(p)]
    result = {"command": command, "cwd": str(ROOT), "started_utc": started,
              "elapsed_seconds": round(time.monotonic() - monotonic, 3),
              "exit_code": process.returncode, "tracked_files": len(before),
              "source_drift": drift,
              "boundary": "Compilation/tests only. Drift must be assessed before associating this run with final sources."}
    (REPORT / (label + "-sources-after.json")).write_text(json.dumps(after, indent=2) + "\n")
    (REPORT / (label + "-result.json")).write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps({"label": label, "exit_code": process.returncode,
                      "source_drift_count": len(drift), "elapsed_seconds": result["elapsed_seconds"]}))
    sys.exit(process.returncode)
