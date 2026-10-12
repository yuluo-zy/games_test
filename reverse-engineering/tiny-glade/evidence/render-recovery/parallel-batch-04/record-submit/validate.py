from pathlib import Path
import hashlib
import json
import subprocess
import sys

OUT = Path(__file__).resolve().parent
ROOT = OUT.parents[3]
SOURCE = ROOT / "reconstruction/render/work-packages/record-submit-r6"
commands = [
    [sys.executable,str(OUT / "verify.py")],
    ["cargo","test","--offline","--locked","--manifest-path",str(SOURCE / "Cargo.toml")],
    ["cargo","clippy","--offline","--locked","--all-targets","--manifest-path",str(SOURCE / "Cargo.toml"),"--","-D","warnings"],
    ["rustfmt","--check","--edition","2021","--config","skip_children=true",str(SOURCE / "src/lib.rs"),str(SOURCE / "src/main.rs")],
]
results = []
for command in commands:
    result = subprocess.run(command,text=True,capture_output=True)
    results.append({"command":command,"exit_code":result.returncode,"stdout":result.stdout,"stderr":result.stderr})
    if result.returncode != 0:
        print(result.stdout + result.stderr)
        raise SystemExit(result.returncode)
native = json.loads((OUT / "native-validation.json").read_text(encoding="utf8"))
report = {"passed":native["passed"] and all(result["exit_code"] == 0 for result in results),"status":"scoped-native-event-differential-pass","original_sha256":native["sha256"],"cases":native["cases"],"by_mode":native["by_mode"],"failure_count":native["failure_count"],"rust_tests":3,"candidate_sha256":hashlib.sha256((SOURCE / "src/lib.rs").read_bytes()).hexdigest(),"commands":results,"original_entries":["0x1402c1d40","0x1402c3240","0x140bebdf0"],"injected_tail":["0x140c06151","0x140c0634f"],"public_boundaries":["std allocator and memcpy","command create/begin/end","timing/profiling","user callbacks","structured queue payload capture","device-error conversion fixture","post-submit/reclamation boundaries"],"not_claimed":["nonempty resource record integration","parallel recording","full queue entry","GPU/Vulkan","ABI/padding/unwinding"],"write_scope":["parallel-batch-04/record-submit","work-packages/record-submit-r6"]}
(OUT / "validation.json").write_text(json.dumps(report,indent=2),encoding="utf8")
print(json.dumps({key:report[key] for key in ["passed","cases","by_mode","failure_count"]}))
