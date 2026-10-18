"""Reproducible checks; successful Cargo exit alone is not a GPU PASS."""
import argparse
import datetime
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess

def workspace_root():
    # Saved harness copies live under reports/, at a different directory depth.
    # Resolve their real workspace rather than silently running elsewhere.
    for candidate in (*Path(__file__).resolve().parents, Path.cwd()):
        if (candidate / "crates/rhapsody/Cargo.toml").is_file() and (candidate / "Cargo.toml").is_file():
            return candidate
    raise RuntimeError("run this harness inside the Tiny Glade workspace")


WORKSPACE = workspace_root()


def fingerprint(scope="all"):
    paths = set()
    directories = ["crates/rhapsody/src", "crates/rhapsody/examples", "crates/rhapsody/crates"]
    if scope == "all":
        directories += ["crates/rhapsody/tests", "crates/tiny-glade/src", "crates/tiny-glade/tests",
                        "crates/tg-text/src", "crates/tg-text/tests", "crates/country-core/src"]
    for directory in directories:
        paths.update((WORKSPACE / directory).rglob("*.rs"))
    # These are the production example's default inputs, outside examples/.
    paths.update(path for path in (WORKSPACE / "crates/rhapsody/assets/window_lab").rglob("*")
                 if path.is_file())
    paths.update(path for path in (WORKSPACE / "crates/rhapsody/examples").rglob("*")
                 if path.suffix in (".ppm", ".json") and "target" not in path.parts)
    paths.update((WORKSPACE / "crates/rhapsody/crates").rglob("Cargo.toml"))
    manifests = ["Cargo.toml", "Cargo.lock", "crates/rhapsody/Cargo.toml",
                 "tools/window-lab/Cargo.toml", "tools/window-lab/Cargo.lock"]
    if scope == "all":
        manifests += ["crates/tiny-glade/Cargo.toml", "crates/tg-text/Cargo.toml", "crates/country-core/Cargo.toml",
                      "crates/country-core/src/render/execution_host.rs", "crates/country-core/src/rhapsody_contract.rs",
                      "crates/country-core/src/rhapsody_types.rs"]
    paths.update(WORKSPACE / name for name in manifests)
    digest = hashlib.sha256()
    for path in sorted(paths):
        if "target" in path.parts:
            continue
        digest.update(str(path.relative_to(WORKSPACE)).encode() + b"\0" + path.read_bytes() + b"\0")
    return digest.hexdigest()


def main():
    parser = argparse.ArgumentParser(__doc__)
    parser.add_argument("--suite", choices=("cpu", "gpu", "consumers", "all"), default="all")
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--target-dir", type=Path)
    parser.add_argument("--shader-assets", type=Path)
    args = parser.parse_args()
    args.output.mkdir(parents=True, exist_ok=True)
    script = Path(__file__).resolve()
    saved_script = args.output / script.name
    if saved_script.resolve() != script:
        shutil.copy2(script, saved_script)
    checks = []
    cargo = ["cargo"]
    common = ["--offline", "--locked", "-j", "2"]
    if args.target_dir:
        common += ["--target-dir", str(args.target_dir.resolve())]
    if args.suite == "cpu":
        checks += [("cpu-unit", cargo + ["test", "-p", "rhapsody", "--all-features", "--lib"] + common + ["--", "--test-threads=1", "--nocapture"], False)]
    if args.suite in ("gpu", "all"):
        for label, features in (("all", ["--all-features"]), ("default", []), ("no-default", ["--no-default-features"])):
            checks.append((label + "-gpu-tests", cargo + ["test", "-p", "rhapsody", "--lib", "--tests", "--no-fail-fast"] + features + common + ["--", "--test-threads=1", "--nocapture"], True))
        checks.append(("gpu-allocation-gate", cargo + ["test", "-p", "rhapsody", "--all-features", "--test", "gpu_frame_allocations"] + common + ["--", "--ignored", "--exact", "real_vulkan_session_reports_framework_execution_allocations", "--test-threads=1", "--nocapture"], True))
    if args.suite in ("consumers", "all"):
        checks += [
            ("cpu-allocation-gate", cargo + ["test", "-p", "rhapsody", "--all-features", "--test", "cpu_frame_allocations"] + common + ["--", "--ignored", "--exact", "fixed_graph_reports_real_build_and_cached_execution_allocations", "--test-threads=1", "--nocapture"], False),
            ("default-all-targets", cargo + ["check", "-p", "rhapsody", "--all-targets"] + common, False),
            ("no-default-all-targets", cargo + ["check", "-p", "rhapsody", "--no-default-features", "--all-targets"] + common, False),
            ("window-cpu", cargo + ["test", "-p", "rhapsody", "--all-features", "--example", "window_lab"] + common + ["--", "--test-threads=1", "--nocapture"], False),
            ("consumers", cargo + ["check", "-p", "rhapsody", "-p", "tiny-glade", "-p", "tg-text", "--all-features", "--all-targets"] + common, False),
            ("country-host", cargo + ["check", "-p", "country-core", "--all-features", "--lib"] + common, False),
            ("country-render-cpu", cargo + ["test", "-p", "country-core", "--lib"] + common + ["--", "render::", "--test-threads=1", "--nocapture"], False),
            ("country-render-loop-cpu", cargo + ["test", "-p", "country-core", "--lib"] + common + ["--", "systems::render_loop::", "--test-threads=1", "--nocapture"], False),
            ("country-pass-kit-cpu", cargo + ["test", "-p", "country-core", "--lib"] + common + ["--", "compute_passes::pass_kit::", "--test-threads=1", "--nocapture"], False),
            ("country-shader-factory-cpu", cargo + ["test", "-p", "country-core", "--lib"] + common + ["--", "compute_passes::shader_refactoring::", "--test-threads=1", "--nocapture"], False),
            ("shader-crates", cargo + ["test", "-p", "shader-pipeline", "-p", "shader-compiler", "--all-features", "--lib", "--tests"] + common + ["--", "--nocapture"], False),
            ("clippy", cargo + ["clippy", "-p", "rhapsody", "--all-features", "--all-targets", "--no-deps"] + common + ["--", "-D", "warnings"], False),
            ("tiny-consumer-gpu", cargo + ["test", "-p", "tiny-glade", "--all-features", "--lib", "--tests", "--no-fail-fast"] + common + ["--", "--test-threads=1", "--nocapture"], True),
            ("text-consumer-gpu", cargo + ["test", "-p", "tg-text", "--all-features", "--lib", "--tests", "--no-fail-fast"] + common + ["--", "--test-threads=1", "--nocapture"], True),
        ]
    manifest = {"started_utc": datetime.datetime.now(datetime.timezone.utc).isoformat(),
        "suite": args.suite, "source_start": fingerprint(), "checks": [],
        "toolchain": subprocess.run(["rustc", "-vV"], cwd=WORKSPACE, capture_output=True, text=True, check=True).stdout,
        "git_head": subprocess.run(["git", "rev-parse", "HEAD"], cwd=WORKSPACE, capture_output=True, text=True, check=True).stdout.strip(),
        "git_dirty": bool(subprocess.run(["git", "status", "--porcelain"], cwd=WORKSPACE, capture_output=True, text=True, check=True).stdout),
        "harness_sha256": hashlib.sha256(script.read_bytes()).hexdigest(),
        "shader_assets": str(args.shader_assets.resolve()) if args.shader_assets else None,
        "limits": "macOS GPU only; Windows, screen capture and interactive keyboard checks are separate evidence. Country CPU filters use default features; all-features lib checking is separate. Native-audio all-features linking requires the external FMOD SDK and is not covered by those default-feature tests."}
    for name, command, gpu in checks:
        print("RUN", name, flush=True)
        before = fingerprint()
        env = os.environ.copy()
        env.update(CARGO_INCREMENTAL="0", CARGO_TERM_COLOR="never", DYLD_FALLBACK_LIBRARY_PATH="/opt/homebrew/lib")
        if gpu:
            env.update(RHAPSODY_REQUIRE_VULKAN="1", VK_INSTANCE_LAYERS="VK_LAYER_KHRONOS_validation", VK_LAYER_VALIDATE_SYNC="1")
        if name == "gpu-allocation-gate":
            env.update(RHAPSODY_REQUIRE_ZERO_FRAME_ALLOCATIONS="1",
                       RHAPSODY_GPU_ALLOCATION_REPORT=str((args.output / "gpu-allocations.json").resolve()))
        if name == "cpu-allocation-gate":
            env.update(RHAPSODY_CPU_ALLOCATION_REPORT=str((args.output / "cpu-allocations.json").resolve()))
        if name == "text-consumer-gpu":
            env.update(TG_TEXT_REQUIRE_GPU="1")
            if args.shader_assets:
                env.update(TG_TEXT_ORIGINAL_ROOT=str(args.shader_assets.resolve()))
        log = args.output / (name + ".log")
        with log.open("w") as stream:
            try:
                code = subprocess.run(command, cwd=WORKSPACE, env=env, stdout=stream, stderr=subprocess.STDOUT, timeout=1800).returncode
            except subprocess.TimeoutExpired:
                code = 124
        output = log.read_text(errors="replace")
        validation_errors = len(re.findall(r"\bValidation Error\b|SYNC-HAZARD|VUID[^\n]*ERROR", output, re.IGNORECASE))
        skips = re.findall(r"^.*(?:\bSKIP\b|\bskip:|跳过|\bBLOCKED\b).*$", output, re.MULTILINE | re.IGNORECASE)
        exercised_skips = [line for line in skips if line.endswith("SKIP Vulkan initialization at injected missing loader: not installed")]
        skips = [line for line in skips if line not in exercised_skips]
        after = fingerprint()
        binaries = []
        for binary_name in dict.fromkeys(re.findall(r"^\s*Running .+? \(([^)]+)\)\s*$", output, re.MULTILINE)):
            source_binary = Path(binary_name)
            if not source_binary.is_absolute():
                source_binary = WORKSPACE / source_binary
            if source_binary.is_file():
                destination = args.output / "binaries" / name / source_binary.name
                destination.parent.mkdir(parents=True, exist_ok=True)
                shutil.copy2(source_binary, destination)
                binaries.append(dict(path=str(destination.relative_to(args.output)),
                    sha256=hashlib.sha256(destination.read_bytes()).hexdigest()))
        status = "FAIL" if code or validation_errors else "BLOCKED" if skips else "PASS"
        if before != after:
            status = "INVALID_SOURCE_CHANGED"
        manifest["checks"].append(dict(name=name, command=command, exit_code=code, status=status,
            source_before=before, source_after=after, validation_errors=validation_errors, skips=skips,
            intentional_cpu_gate_exercises=exercised_skips, binaries=binaries, log=log.name,
            test_totals={label: sum(int(row[index]) for row in re.findall(
                r"test result: \w+\. (\d+) passed; (\d+) failed; (\d+) ignored;", output))
                for index, label in enumerate(("passed", "failed", "ignored"))},
            ignored_tests=re.findall(r"^test .+ \.\.\. ignored[^\n]*$", output, re.MULTILINE)))
        (args.output / "checks.json").write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + "\n")
        print("DONE", name, status, flush=True)
    manifest["source_end"] = fingerprint()
    manifest["source_stable"] = manifest["source_start"] == manifest["source_end"] and all(
        entry["source_before"] == manifest["source_start"] == entry["source_after"] for entry in manifest["checks"])
    (args.output / "checks.json").write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + "\n")
    return 0 if manifest["source_stable"] and all(entry["status"] == "PASS" for entry in manifest["checks"]) else 1


if __name__ == "__main__":
    raise SystemExit(main())
