"""Compile actual mesh/loading/library sources without the unrelated core input modules.

Only asset_libraries/mod.rs's declarations before its first `pub mod` are selected.
No algorithm, Handle, Mesh, or asset-library implementation is replaced by a stub.
Public and shared dependencies use the artifacts built by the preceding Cargo check.
"""
from pathlib import Path
import hashlib
import json
import os
import subprocess
import tempfile

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
CORE = ROOT / "crates/country-core/src"
DEPS = Path("/tmp/tg-country-core-utils-review-20261018-target/debug/deps")

def module(name, path):
    return f"#[path = {json.dumps(str(path))}] pub mod {name};\n"

names = ["glam", "serde", "serde_json", "anyhow", "bevy_ecs", "parking_lot",
         "tempfile", "log", "utils"]
arguments = []
artifacts = {}
for name in names:
    matches = list(DEPS.glob(f"lib{name}-*.rlib"))
    if name == "glam":
        matches = [p for p in matches if "/glam-0.29.3/" in
                   (DEPS / (p.name.removeprefix("lib").removesuffix(".rlib") + ".d")).read_text()]
    # Host/target feature variants may both exist. parking_lot types do not cross
    # this harness's external boundary; retain the most recently built variant.
    if name == "parking_lot" and len(matches) > 1:
        matches = [max(matches, key=lambda p: p.stat().st_mtime_ns)]
    if len(matches) != 1:
        raise SystemExit(f"expected exactly one built dependency for {name}: {matches}")
    path = matches[0]
    arguments.extend(["--extern", f"{name}={path}"])
    artifacts[str(path)] = hashlib.sha256(path.read_bytes()).hexdigest()

source_paths = [CORE / p for p in ["render/mesh.rs", "utils/load_json.rs",
    "asset_libraries/mod.rs", "asset_libraries/mesh_library.rs", "utils_contract.rs", "frozen_map.rs"]]
before = {str(p): hashlib.sha256(p.read_bytes()).hexdigest() for p in source_paths}
with tempfile.TemporaryDirectory(prefix="country-utils-source-review-") as directory:
    directory = Path(directory)
    # Keep the original asset identities and methods byte-for-byte; omit unrelated modules.
    prelude = (CORE / "asset_libraries/mod.rs").read_text().split("pub mod ", 1)[0]
    assets = directory / "assets.rs"
    assets.write_text(prelude + module("mesh_library", CORE / "asset_libraries/mesh_library.rs"))
    source = "extern crate self as country_core;\n"
    source += module("asset_libraries", assets)
    source += module("frozen_map", CORE / "frozen_map.rs")
    source += module("utils_contract", CORE / "utils_contract.rs")
    source += "pub mod render {\n" + module("mesh", CORE / "render/mesh.rs") + "}\n"
    source += "pub mod utils {\n" + module("load_json", CORE / "utils/load_json.rs") + "}\n"
    source += (HERE / "probes.rs").read_text()
    entry = directory / "main.rs"
    entry.write_text(source)
    binary = directory / "probes"
    command = ["rustc", "--edition=2024", "--test", str(entry), "-L", f"dependency={DEPS}",
               *arguments, "-o", str(binary)]
    with (HERE / "source-probes.log").open("w") as output:
        output.write("$ " + " ".join(command) + "\n")
        output.flush()
        environment = dict(os.environ, CARGO_MANIFEST_DIR=str(CORE.parent), CARGO_PKG_NAME="country-core")
        compile_result = subprocess.run(command, cwd=ROOT, env=environment,
                                        stdout=output, stderr=subprocess.STDOUT)
        run_result = None
        if compile_result.returncode == 0:
            run_result = subprocess.run([str(binary), "--test-threads=1"], cwd=ROOT,
                                        stdout=output, stderr=subprocess.STDOUT)
after = {str(p): hashlib.sha256(p.read_bytes()).hexdigest() for p in source_paths}
metadata = {"sources_sha256": before, "sources_unchanged_during_run": before == after,
            "dependency_artifacts_sha256": artifacts, "compile_exit_code": compile_result.returncode,
            "run_exit_code": None if run_result is None else run_result.returncode,
            "boundary": "current mesh/load_json/library/frozen_map and exact Handle declaration prefix; no full core scheduling"}
(HERE / "source-probes.json").write_text(json.dumps(metadata, indent=2) + "\n")
if before != after:
    raise SystemExit("reviewed sources changed during the source probe")
raise SystemExit(compile_result.returncode if run_result is None else run_result.returncode)
