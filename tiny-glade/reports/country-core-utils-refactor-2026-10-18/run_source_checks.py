"""在完整 core 被其他模块阻断时，直接编译当前工具与真实网格/资产库源码。"""
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

def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

arguments, artifacts = [], {}
for name in ["glam", "serde", "serde_json", "anyhow", "bevy_ecs", "parking_lot", "tempfile", "log", "utils", "tabloid"]:
    matches = list(DEPS.glob(f"lib{name}-*.rlib"))
    if name == "glam":
        matches = [path for path in matches if "/glam-0.29.3/" in (DEPS / (path.name[3:-5] + ".d")).read_text()]
    if not matches:
        raise SystemExit(f"请先构建依赖：{name}")
    path = max(matches, key=lambda path: path.stat().st_mtime_ns)
    arguments.extend(["--extern", f"{name}={path}"])
    artifacts[str(path)] = digest(path)

names = ["conservative_line_raster", "dirty_tiles", "float_grid", "index_condenser", "child_wall", "geo", "load_json", "audio_utils", "project_cursor_onto_ray"]
source_paths = [CORE / f"utils/{name}.rs" for name in names]
source_paths += [CORE / path for path in ["render/mesh.rs", "asset_libraries/mod.rs", "asset_libraries/mesh_library.rs", "utils_contract.rs", "frozen_map.rs", "utils/components/transform.rs"]]
before = {str(path.relative_to(ROOT)): digest(path) for path in source_paths}
with tempfile.TemporaryDirectory(prefix="country-utils-refactor-check-") as directory:
    directory = Path(directory)
    assets = directory / "assets.rs"
    prelude = (CORE / "asset_libraries/mod.rs").read_text().split("pub mod ", 1)[0]
    assets.write_text(prelude + module("mesh_library", CORE / "asset_libraries/mesh_library.rs"))
    source = "extern crate self as country_core;\n"
    source += module("asset_libraries", assets)
    source += module("frozen_map", CORE / "frozen_map.rs")
    source += module("utils_contract", CORE / "utils_contract.rs")
    source += "pub mod render {\n" + module("mesh", CORE / "render/mesh.rs") + "}\n"
    source += "pub mod utils {\n"
    source += "pub mod components {\n" + module("transform", CORE / "utils/components/transform.rs") + "}\n"
    source += "".join(module(name, CORE / f"utils/{name}.rs") for name in names) + "}\n"
    entry, binary = directory / "main.rs", directory / "tests"
    entry.write_text(source)
    environment = dict(os.environ, CARGO_MANIFEST_DIR=str(CORE.parent), CARGO_PKG_NAME="country-core")
    command = ["rustc", "--edition=2024", "--test", str(entry), "-L", f"dependency={DEPS}", *arguments, "-o", str(binary)]
    with (HERE / "source-checks.log").open("w") as output:
        output.write("$ " + " ".join(command) + "\n")
        output.flush()
        compiled = subprocess.run(command, env=environment, stdout=output, stderr=subprocess.STDOUT)
        tested = None if compiled.returncode else subprocess.run([str(binary), "--test-threads=1"], stdout=output, stderr=subprocess.STDOUT)
after = {str(path.relative_to(ROOT)): digest(path) for path in source_paths}
metadata = {"sources_sha256": before, "sources_unchanged": before == after, "dependencies_sha256": artifacts, "compile_exit": compiled.returncode, "test_exit": None if tested is None else tested.returncode, "excluded": ["utils/mod.rs helpers", "debug_viz and full ECS rendering", "save/load integration", "other core modules"]}
(HERE / "source-checks.json").write_text(json.dumps(metadata, indent=2) + "\n")
if before != after: raise SystemExit("检查期间源码发生变化")
raise SystemExit(compiled.returncode if tested is None else tested.returncode)
