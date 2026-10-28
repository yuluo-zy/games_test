"""Compile live startup sources using fingerprint-matched cached dependencies.

No workspace lock or production sources are changed. The support crate is a
compiled snapshot, so this cannot substitute for a current full Cargo build.
"""
from pathlib import Path
import hashlib
import json
import os
import subprocess

report = Path(__file__).resolve().parent
workspace = report.parents[1]
fingerprints = workspace / "target/debug/.fingerprint"
core_id = "country-core-d46518866f4e2c65"
metadata = json.loads((fingerprints / core_id / "lib-country_core.json").read_text())
artifacts = {"country_core": workspace / "target/debug/deps/libcountry_core-d46518866f4e2c65.rlib"}
for dependency in metadata["deps"]:
    name, expected = dependency[1], dependency[3]
    if name not in {"bevy_ecs", "anyhow", "asset", "glam", "hotwatch", "log", "serde_json"}:
        continue
    matches = [p for p in fingerprints.glob(f"*/lib-{name}")
               if int.from_bytes(bytes.fromhex(p.read_text().strip()), "little") == expected]
    if len(matches) != 1:
        raise SystemExit(f"Expected one fingerprint for {name}, found {len(matches)}")
    suffix = matches[0].parent.name.rsplit("-", 1)[1]
    artifacts[name] = workspace / f"target/debug/deps/lib{name}-{suffix}.rlib"

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

source_files = list((workspace / "crates/country-core/src/startup").glob("*.rs"))
source_hashes = {str(p.relative_to(workspace)): sha(p) for p in source_files}
artifact_hashes = {str(p.relative_to(workspace)): sha(p) for p in artifacts.values()}
binary = report / "source-probes-bin"
command = ["rustc", "--edition=2024", "--test", str(report / "probes.rs"),
           "--crate-name", "startup_source_review", "-L", f"dependency={workspace / 'target/debug/deps'}",
           "-o", str(binary)]
for name, path in artifacts.items():
    command += ["--extern", f"{name}={path}"]
for path in (workspace / "target/debug/build").glob("*/out"):
    command += ["-L", f"native={path}"]
try:
    with (report / "source-probes.log").open("w") as log:
        environment = dict(os.environ, CARGO_MANIFEST_DIR=str(workspace / "crates/country-core"))
        result = subprocess.run(command, cwd=workspace, env=environment, stdout=log, stderr=subprocess.STDOUT)
        build_code = result.returncode
        if build_code == 0:
            result = subprocess.run([str(binary), "--nocapture", "--test-threads=1"],
                                    cwd=workspace, stdout=log, stderr=subprocess.STDOUT)
finally:
    binary.unlink(missing_ok=True)
(report / "source-probes.json").write_text(json.dumps({
    "command": command, "build_exit_code": build_code, "exit_code": result.returncode,
    "current_startup_source_sha256": source_hashes,
    "source_unchanged": all(sha(workspace / p) == h for p, h in source_hashes.items()),
    "cached_support_artifacts_sha256": artifact_hashes,
    "scope": "Current load_prefabs/startup_session/run_game/load_glade_audio/startup_terrain/load_textures sources, cached supporting crate/dependencies; no fresh workspace or original runtime verification",
}, ensure_ascii=False, indent=2) + "\n")
print(f"build={build_code}; exit={result.returncode}; log={report / 'source-probes.log'}")
raise SystemExit(result.returncode)
