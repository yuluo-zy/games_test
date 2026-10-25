"""Save reproducible module evidence and the existing candidate build gates."""
from pathlib import Path
import hashlib
import json
import subprocess
import tomllib

OUT = Path(__file__).resolve().parent
ROOT = OUT.parents[3]
SOURCE = ROOT / "reconstruction/render/work-packages/queue-submit-r8"


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    native = json.loads((OUT / "native-validation.json").read_text(encoding="utf8"))
    inputs = "".join(" ".join(map(str, o["case"])) + "\n" for o in native["observations"])
    (OUT / "native-inputs.txt").write_text(inputs, encoding="utf8", newline="\n")
    gates = []
    for name, args in [
        ("build-release", ["build", "--release", "--locked", "--offline"]),
        ("check-release", ["check", "--release", "--locked", "--offline"]),
        ("tests", ["test", "--locked", "--offline"]),
        ("clippy", ["clippy", "--locked", "--offline", "--all-targets"]),
        ("fmt", ["fmt"]),
    ]:
        cmd = ["cargo", *args, "--manifest-path", str(SOURCE / "Cargo.toml")]
        if name == "clippy":
            cmd.extend(["--", "-D", "warnings"])
        if name == "fmt":
            cmd.extend(["--", "--check"])
        result = subprocess.run(cmd, text=True, capture_output=True)
        log = OUT / (name + ".log")
        log.write_text(result.stdout + result.stderr, encoding="utf8")
        gates.append({"gate": name, "passed": result.returncode == 0, "log": str(log.relative_to(ROOT)).replace("\\", "/")})
    replay = subprocess.run([str(SOURCE / "target/release/tiny-glade-queue-submit-r8.exe")], input=inputs, text=True, capture_output=True, check=True).stdout.splitlines()
    replay_passed = len(replay) == len(native["observations"]) and all(o["native"] == r for o, r in zip(native["observations"], replay))
    lock = tomllib.loads((SOURCE / "Cargo.lock").read_text(encoding="utf8"))
    original = tomllib.loads((ROOT / "tracking/original-Cargo.lock").read_text(encoding="utf8"))
    keys = ("name", "version", "source", "checksum")
    baseline = {tuple(p.get(k) for k in keys) for p in original["package"]}
    dependencies = [{**{k: p.get(k) for k in keys}, "matches_original": tuple(p.get(k) for k in keys) in baseline} for p in lock["package"] if p.get("source")]
    paths = [
        SOURCE / "Cargo.toml", SOURCE / "Cargo.lock", SOURCE / "src/lib.rs", SOURCE / "src/main.rs",
        OUT / "verify.py", OUT / "finalize_validation.py", OUT / "native-inputs.txt", OUT / "native-validation.json",
        OUT / "findings.md", OUT / "consumer-interface.json", OUT / "module-evidence-index.json",
        ROOT / "workspace/input/tiny-glade.exe", ROOT / "tracking/original-Cargo.lock",
        ROOT / "evidence/render-recovery/parallel-batch-03/barrier-record/verify.py",
        ROOT / "evidence/render-recovery/parallel-batch-04/record-submit/queue.objdump.txt",
        ROOT / "evidence/render-recovery/parallel-batch-02/gpu-lifecycle/0x140c04e20.c",
        ROOT / "evidence/render-recovery/parallel-batch-02/executor-temporal/0x140bebdf0.c",
        ROOT / "evidence/render-recovery/parallel-batch-02/executor-temporal/0x140bebc90.c",
        ROOT / "evidence/render-recovery/parallel-batch-02/executor-temporal/0x140bf0b90.c",
        ROOT / "evidence/render-recovery/parallel-batch-02/gpu-lifecycle/0x140bed230.c",
        ROOT / "evidence/render-recovery/parallel-batch-02/gpu-lifecycle/0x140bed050.c",
        ROOT / "evidence/render-recovery/parallel-batch-04/device-counters/0x140bf2560.c",
        ROOT / "evidence/render-recovery/parallel-batch-05/queue-callers/callsite-inventory.json",
        ROOT / "evidence/render-recovery/parallel-batch-05/queue-callers/native-oracle.json",
        ROOT / "evidence/render-recovery/parallel-batch-03/sequence-domain/0x1418c3b80.c",
        ROOT / "evidence/render-architecture/public-symbols.json",
        ROOT / "evidence/roof-tiles/call-symbols.json",
    ]
    report = {
        "status": "verified_partial_cpu_module_projection",
        "passed": native["passed"] and replay_passed and all(g["passed"] for g in gates) and all(d["matches_original"] for d in dependencies),
        "candidate_replay_after_dependency_pin": replay_passed,
        "native": {k: native[k] for k in ("passed", "cases", "failure_count", "original_sha256", "original_entry", "entry_from_beginning", "submit_arrays_injected", "normal_return_cases", "backend_capture_cases")},
        "candidate_glue_tests": 2,
        "gates": gates,
        "dependencies": dependencies,
        "inputs": [{"path": str(p.relative_to(ROOT)).replace("\\", "/"), "sha256": digest(p)} for p in paths],
        "shared_consumer": "reconstruction/render/execution.rs::ExecutionChain::submit_recorded",
        "shared_consumer_acceptance_owner": "root; this validation does not validate the shared graph-to-submit chain",
        "private_boundaries": ["GraphicsDevice::handle_error result tag supplied", "diagnostic ringbuffer/backtrace capture omitted", "flushed-command typed API represents native end-success only"],
        "remaining": ["live locks/contention", "native end-error conversion", "panic unwind and opaque managed-field ownership", "real prior batch/extra semaphore producers", "full original graph ABI and game pass", "live Vulkan/GPU validation"],
        "module_complete": False,
        "live_gpu": False,
    }
    (OUT / "validation.json").write_text(json.dumps(report, indent=2), encoding="utf8")
    print(json.dumps({"passed": report["passed"], "native": report["native"], "gates": gates}))
    assert report["passed"]


if __name__ == "__main__":
    main()
