from pathlib import Path
import hashlib
import json
import subprocess
import sys

OUT = Path(__file__).resolve().parent
ROOT = OUT.parents[3]
SOURCE = ROOT / "reconstruction/render/work-packages/record-resources-r7"
commands = [
    [sys.executable, str(OUT / "verify.py")],
    ["cargo", "test", "--offline", "--locked", "--manifest-path", str(SOURCE / "Cargo.toml")],
    ["cargo", "clippy", "--offline", "--locked", "--all-targets", "--manifest-path", str(SOURCE / "Cargo.toml"), "--", "-D", "warnings"],
    ["rustfmt", "--check", "--edition", "2021", "--config", "skip_children=true", str(SOURCE / "src/lib.rs"), str(SOURCE / "src/main.rs")],
]
results = []
for command in commands:
    result = subprocess.run(command, text=True, capture_output=True)
    results.append({"command":command,"exit_code":result.returncode,"stdout":result.stdout,"stderr":result.stderr})
    if result.returncode:
        print(result.stdout + result.stderr)
        raise SystemExit(result.returncode)
native = json.loads((OUT / "native-validation.json").read_text(encoding="utf8"))
files = [SOURCE / "Cargo.toml", SOURCE / "Cargo.lock", SOURCE / "src/lib.rs", SOURCE / "src/main.rs", OUT / "verify.py", OUT / "validate.py", OUT / "native-validation.json"]
files += [ROOT / f"reconstruction/render/work-packages/barrier-record-r5/{relative}" for relative in ["Cargo.toml", "Cargo.lock", "src/lib.rs"]]
files += [ROOT / "evidence/render-recovery/parallel-batch-03/barrier-record/verify.py", ROOT / "evidence/render-recovery/parallel-batch-04/record-submit/verify.py"]
files += [ROOT / f"evidence/render-recovery/parallel-batch-04/record-submit/{label}.objdump.txt" for label in ["record", "pass-record", "submit"]]
hashes = {str(path.relative_to(ROOT)).replace("\\", "/"): hashlib.sha256(path.read_bytes()).hexdigest() for path in files}
visited = {address for observation in native["observations"] for address in observation["visited"]}
coverage = {hex(start):sum(start <= int(address,16) < end for address in visited) for start,end in [(0x1402C1D40,0x1402C2F82),(0x1402C3240,0x1402C3A9A),(0x1402C3D00,0x1402C427A),(0x140BD6A70,0x140BD6BC0),(0x140BEBDF0,0x140BEC2C9)]}
report = {
    "passed":native["passed"],"status":"scoped-native-nonempty-record-submit-differential-pass",
    "original_sha256":native["original_sha256"],"cases":native["cases"],"failure_count":native["failure_count"],"rust_tests":5,
    "candidate_sha256":hashlib.sha256((SOURCE / "src/lib.rs").read_bytes()).hexdigest(),"hashes":hashes,
    "nonempty_resource_cases":native["nonempty_resource_cases"],"nonempty_pass_access_cases":native["nonempty_pass_access_cases"],
    "no_access_export_fallback_cases":native["no_access_export_fallback_cases"],"record_to_submit_cases":native["record_to_submit_cases"],
    "begin_tags_observed":native["begin_tags_observed"],"pool_selectors_observed":native["pool_selectors_observed"],
    "sequence_is_original_initialization":False,"fixture_command_sequence":7,"old_108_cases_recounted":False,
    "original_entries":native["original_entries"],"original_unique_instruction_starts_by_region":coverage,"commands":results,
    "original_private_helpers_unmodified":["0x140be78b0","0x140bd6c80","0x140bece70","0x140be9690","0x140bd6a70"],
    "public_boundaries":["std allocation/memcpy/drop fixture", "dependency write classifier", "aspect mapping", "synchronization constructors/append/emit/reset", "device command creation/begin/end", "profiling/timing", "user callback", "queue-submit structured capture"],
    "private_device_boundaries_still_stubbed":["0x140bfc750 queue-family index returns 0 fixture", "0x140becfd0 tracker counted, internals not executed", "0x140c04e20 queue prefix/tail not executed in this oracle"],
    "observed_post_pass_contract":"No separate post-pass barrier scope in sequential helper; next pre-pass transition or final export follows callback success.",
    "not_claimed":["parallel recording", "full queue entry", "natural reachability of reused=true", "real Vulkan/GPU", "actual device command sequence initialization", "ABI/padding/unwinding", "full descriptor or game pass integration"],
    "write_scope":["parallel-batch-05/record-resources","work-packages/record-resources-r7"]
}
(OUT / "validation.json").write_text(json.dumps(report, indent=2), encoding="utf8")
print(json.dumps({key:report[key] for key in ["passed","cases","failure_count","rust_tests","nonempty_pass_access_cases","record_to_submit_cases","candidate_sha256"]}))
