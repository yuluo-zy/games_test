"""Executable, hash-bound cross-package gate. Never opens a Ghidra database.

Replays recorded cases against original CPU instructions and freshly built Rust,
checks shipped SPIR-V spans and current reflection, and executes a Rust cross-
crate harness. Candidate tests, native cases and interface checks stay separate.
"""
from __future__ import annotations
import argparse
from collections import Counter
import gzip
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import struct
import subprocess
import sys
import tomllib

BINARY_SHA = "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03"
LOCK_SHA = "9e0cb4a0fa173122fa7cd552aa639f5ee977abc22c74aec92398808c707184bd"

def read(path):
    return json.loads(Path(path).read_text(encoding="utf-8"))

def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def digest(value, **kwargs):
    return hashlib.sha256(json.dumps(value, ensure_ascii=False, **kwargs).encode()).hexdigest()

def build(root: Path, original: Path) -> dict:
    batch = root / "evidence/render-recovery/parallel-batch-04"
    tooling = root / "evidence/render-recovery/tooling"
    tooling.mkdir(parents=True, exist_ok=True)
    checks, commands, inputs = [], [], {}

    def track(path):
        path = Path(path)
        try: name = str(path.relative_to(root)).replace("\\", "/")
        except ValueError: name = str(path)
        inputs[name] = {"path": str(path), "sha256": sha(path)}
        return path

    def load(path):
        return read(track(path))

    def check(name, ok, detail):
        checks.append({"name": name, "ok": bool(ok), "detail": detail})

    def command(label, argv, stdin=None):
        result = subprocess.run([str(x) for x in argv], cwd=root, input=stdin, text=True, capture_output=True)
        log = {"command": [str(x) for x in argv], "exit_code": result.returncode, "stdout": result.stdout, "stderr": result.stderr}
        path = tooling / ("batch04-" + label + ".json")
        path.write_text(json.dumps(log, indent=2) + "\n", encoding="utf-8")
        commands.append({"label": label, "path": str(path.relative_to(root)), "sha256": sha(path), "exit_code": result.returncode})
        if result.returncode:
            check(label, False, result.stderr[-3000:])
            raise RuntimeError(f"required command failed: {label}; see {path}")
        return result.stdout

    contract = load(batch / "contract.json")
    check("target_original", sha(track(original / "tiny-glade.rne")) == BINARY_SHA, "Original PE digest bound to this build")
    check("target_analysis_copy", sha(track(root / "workspace/input/tiny-glade.exe")) == BINARY_SHA, "MCP analysis copy equals shipped PE")
    check("dependency_baseline", sha(track(root / "tracking/original-Cargo.lock")) == LOCK_SHA and sha(track(original / "build-info/Cargo.lock")) == LOCK_SHA, "Original lock and tracked baseline match")
    check("contract_binary_identity", contract["identity"]["binary_sha256"] == BINARY_SHA, BINARY_SHA)
    for key, path in [("functions", root / "evidence/render-recovery/workgraph/functions.json"), ("calls", root / "evidence/render-recovery/workgraph/calls.json"), ("batch_03_integration", root / "evidence/render-recovery/parallel-batch-03/integration.json")]:
        check("contract_" + key + "_identity", contract["identity"][key + "_sha256"] == sha(track(path)), str(path))
    r5integration = read(root / "evidence/render-recovery/parallel-batch-03/integration.json")
    check("prior_gate_passed", r5integration["status"] == "consistent_partial" and not r5integration["failed_checks"], "Batch03 prior gate is still hash-bound; scoped candidate limits remain")

    device = batch / "device-counters"
    record = batch / "record-submit"
    pipeline = batch / "pipeline-shader-interfaces"
    dv, dn, dc = load(device / "validation.json"), load(device / "native-validation.json"), load(device / "checksums.json")
    rv, rn, rc, ru = [load(record / file) for file in ["validation.json", "native-validation.json", "implementation-contract.json", "unknowns.json"]]
    pv, summary, catalog, inventory, families, iv, dependencies = [load(pipeline / file) for file in ["validation.json", "summary.json", "catalog.json", "inventory.json", "interface-families.json", "interface-validation.json", "dependency-verification.json"]]
    candidates = {"device": root / "reconstruction/render/work-packages/device-counters-r6", "record": root / "reconstruction/render/work-packages/record-submit-r6", "pipeline": root / "reconstruction/render/work-packages/pipeline-interfaces-r6", "r5": root / "reconstruction/render/work-packages/sequence-domain-r5"}
    for group, folder in [("evidence", device), ("candidate", candidates["device"])]:
        mismatches = [name for name, expected in dc[group].items() if sha(track(folder / name)) != expected]
        check("device_" + group + "_hashes", not mismatches, {"verified": len(dc[group]), "mismatches": mismatches})
    for folder in [device, record, pipeline]:
        for file in sorted(folder.iterdir()):
            if file.is_file(): track(file)
    for folder in candidates.values():
        for file in [folder / "Cargo.toml", folder / "Cargo.lock", *sorted((folder / "src").rglob("*.rs"))]:
            track(file)
    check("native_report_identities", dv["target_sha256"] == dn["binary_sha256"] == rv["original_sha256"] == rn["sha256"] == BINARY_SHA, "Both native packages refer to the verified target")
    check("device_native_source_binding", dn["candidate_sha256"] == sha(candidates["device"] / "src/lib.rs"), "Device oracle candidate source unchanged")
    check("record_native_source_binding", rv["candidate_sha256"] == sha(candidates["record"] / "src/lib.rs"), "Record/submit oracle candidate source unchanged")
    check("native_retained_success_manifests", dn["cases"] == dv["native"]["cases"] == 2840 and dn["mismatches"] == dv["native"]["mismatches"] == 0 and not dn["failures"] and rv["passed"] is True and rn["passed"] is True and rv["cases"] == rn["cases"] == 108 and rv["failure_count"] == rn["failure_count"] == 0 and not rn["failures"], "Retained case counts/failure declarations agree with fresh replay inputs")
    for item in pv["determinism"]:
        check("shader_retained_" + item["file"] + "_hash", item["before"] == item["after"] == sha(pipeline / item["file"]) and item["identical"], "Retained deterministic artifact matches its manifest")
    check("shader_inventory_report_hash", pv["identity"]["inventory_sha256"] == sha(pipeline / "inventory.json"), "Validation inventory points to current file")
    mcp = load(device / "mcp-report.json")
    closes = [x for x in mcp["results"] if x["name"] == "close_database"]
    check("retained_mcp_cleanup", len(closes) == 1 and closes[0]["args"]["save"] is False and not closes[0]["result"].get("isError", False) and dv["mcp"]["close_save"] is False, "Historical device evidence closed without save; this gate opens no database")

    baseline = tomllib.loads((root / "tracking/original-Cargo.lock").read_text())["package"]
    baseline_keys = {(p["name"], p["version"], p.get("source"), p.get("checksum")) for p in baseline}
    external = []
    for name, folder in candidates.items():
        packages = tomllib.loads((folder / "Cargo.lock").read_text())["package"]
        foreign = [p for p in packages if p.get("source")]
        mismatch = [p for p in foreign if (p["name"], p["version"], p.get("source"), p.get("checksum")) not in baseline_keys]
        check(name + "_dependencies", not mismatch, {"external_packages": len(foreign), "mismatches": mismatch})
        if name == "pipeline": external = foreign
    check("shader_dependency_manifest", dependencies["packages"] == [{k: p[k] for k in ["name", "version", "source", "checksum"] if k in p} for p in external] and not dependencies["mismatches"] and len(external) == 17, "17 original source/version/checksum matches, including spirq fork")

    tests = {}
    for name, folder in candidates.items():
        output = command(name + "-tests", ["cargo", "test", "--offline", "--locked", "--manifest-path", folder / "Cargo.toml"])
        tests[name] = sum(int(x) for x in re.findall(r"test result: ok\. (\d+) passed", output))
        check(name + "_tests_passed", tests[name] == {"device": 6, "record": 3, "pipeline": 4, "r5": 2}[name], f"actual cargo test: {tests[name]}")
    for name in ["device", "record"]:
        command(name + "-build", ["cargo", "build", "--release", "--offline", "--locked", "--manifest-path", candidates[name] / "Cargo.toml"])
    command("pipeline-build", ["cargo", "build", "--offline", "--locked", "--bins", "--manifest-path", candidates["pipeline"] / "Cargo.toml"])

    # Fresh CPU execution of the exact stored cases. No report in a package is mutated.
    sys.path.insert(0, str(root / "evidence/render-recovery/parallel-batch-03/barrier-record"))
    spec = importlib.util.spec_from_file_location("batch04_original_record", record / "verify.py")
    native_record = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(native_record)
    check("native_harness_target", sha(track(native_record.base.TARGET)) == BINARY_SHA, "Imported original-PE harness is bound to the same executable")
    observations = rn["observations"]
    fresh_record = [(native_record.chain(o["case"]) if o["case"][0] == 3 else native_record.execute(o["case"]))[0] for o in observations]
    record_actual = command("record-oracle-replay", [candidates["record"] / "target/release/tiny-glade-record-submit-r6.exe"], "".join(" ".join(map(str, o["case"])) + "\n" for o in observations)).splitlines()
    check("record_fresh_native_rust_replay", len(record_actual) == len(fresh_record) == 108 and record_actual == fresh_record == [o["native"] for o in observations], "108 freshly emulated original entry/tail cases equal fresh Rust and retained oracle")
    check("record_mode_coverage", dict(sorted(Counter(str(o["case"][0]) for o in observations).items())) == rv["by_mode"] == rn["by_mode"] == {"0": 28, "1": 48, "2": 20, "3": 12}, "Empty-resource sequential record, submit, injected tail and chain counted separately")
    from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
    from unicorn.x86_const import UC_X86_REG_RSI, UC_X86_REG_RBX, UC_X86_REG_RBP, UC_X86_REG_RDI
    raw = (original / "tiny-glade.rne").read_bytes()
    header = struct.unpack_from("<I", raw, 60)[0]; optional = header + 24
    image_base = struct.unpack_from("<Q", raw, optional + 24)[0]
    cpu = Uc(UC_ARCH_X86, UC_MODE_64)
    image_size = struct.unpack_from("<I", raw, optional + 56)[0]
    cpu.mem_map(image_base, (image_size + 4095) & ~4095)
    for i in range(struct.unpack_from("<H", raw, header + 6)[0]):
        section = optional + struct.unpack_from("<H", raw, header + 20)[0] + 40 * i
        _, address, size, offset = struct.unpack_from("<IIII", raw, section + 8)
        if size: cpu.mem_write(image_base + address, raw[offset:offset + size])
    check("counter_initializer_original_bytes", list(struct.unpack("<QQ", cpu.mem_read(0x142ae0d00, 16))) == dn["initializer_simd_constant"]["u64_values"] == [1, 1], "Original SIMD initializer stores ones")
    data = 0x20000000; cpu.mem_map(data, 0x10000)
    cpu.hook_add(UC_HOOK_CODE, lambda machine, address, size, user: machine.emu_stop() if address in [0x140bed31d, 0x140bed3a8] else None)
    with gzip.open(device / "native-projection-oracle.jsonl.gz", "rt", encoding="utf-8") as stream:
        cases = [json.loads(line) for line in stream]
    fresh_device = []
    for item in cases:
        tag, index, *values = [int(v, 16) for v in item["input"].split()]
        cpu.mem_write(data + 0x668, struct.pack("<QQ", data + 0x3000, len(values)))
        cpu.mem_write(data + 0x3000, struct.pack("<" + "Q" * len(values), *values))
        cpu.mem_write(data + 0x2000, bytes([0xa5]) * 88)
        for reg, value in [(UC_X86_REG_RSI, data), (UC_X86_REG_RBX, data + 0x2000), (UC_X86_REG_RBP, tag), (UC_X86_REG_RDI, index)]: cpu.reg_write(reg, value)
        cpu.emu_start(0x140bed2d8, 0x140bed3b4, count=50)
        slot = 0 if tag & 1 == 0 else (index + 1) & 0xffffffff
        after = struct.unpack("<" + "Q" * len(values), cpu.mem_read(data + 0x3000, 8 * len(values)))
        if slot < len(values):
            cb_tag, sequence, stored_tag, stored_index = struct.unpack("<QQII", cpu.mem_read(data + 0x2000, 24))
            if cb_tag != 1: raise RuntimeError("unexpected original CB state")
            if stored_tag != tag or stored_index != index or cpu.mem_read(data + 0x2000 + 0x56, 1) != b'\x00': raise RuntimeError("unexpected original CB queue fields")
            prefix = f"ok {sequence:x}"
        else: prefix = "bounds"
        fresh_device.append(prefix + "".join(f" {v:x}" for v in after))
    device_actual = command("device-oracle-replay", [candidates["device"] / "target/release/device-counter-oracle.exe"], "\n".join(x["input"] for x in cases) + "\n").splitlines()
    check("device_fresh_native_rust_replay", len(device_actual) == len(cases) == 2840 and device_actual == fresh_device == [x["native"] for x in cases], "2840 freshly emulated counter-selection cases equal Rust and retained oracle")

    machine = load(device / "mapping-state-machine.json")
    check("queue_counter_contract", machine["constructor"]["counter_initial_value"] == 1 and machine["mapping"]["globally_unique_cb_sequence"] is False and machine["mapping"]["cb_sequence_domain"] == "per queue counter slot" and machine["constructor"]["global_submission_timeline_initial_value"] == 1, "Per-queue CB counters, separate global numeric submission timeline")
    check("record_limits_preserved", rc["production_integration"] is False and ru["nonempty_resource_record_integration"] is False and ru["parallel_recording"] is False and ru["full_queue_original_entry_executed"] is False and ru["gpu_or_vulkan_validation"] is False, "R6B remains empty-resource sequential plus injected queue-tail scope")

    harness = tooling / "batch04-cross-crate.rs"
    # Include the exact candidates read-only. This is a test adapter, not new game source.
    harness.write_text('''#![allow(dead_code)]
#[path = "%s"] mod device;
#[path = "%s"] mod sequence;
#[path = "%s"] mod record;
#[test] fn queue_domain_and_fixture_seed_boundary() {
 let counters=device::DeviceCounters::new(1);
 assert_eq!(counters.begin_sequence(device::QueueIdentity::Primary),Some(1));
 assert_eq!(counters.begin_sequence(device::QueueIdentity::Auxiliary(0)),Some(1));
 let mut d=sequence::SequenceDomain::default();
 let p=sequence::QueueIdentity{tag:0,aux_index:0};let a=sequence::QueueIdentity{tag:1,aux_index:0};
 assert_eq!(d.begin_cb(p),Ok(0)); // Explicit R5B fixture seed, not constructor behavior.
 assert_eq!(d.begin_cb(a),Err(sequence::Error::SequenceCollision(0)));
}
#[test] fn record_submit_timeline_domain() {
 let c=device::DeviceCounters::new(0);let mut d=sequence::SequenceDomain::default();
 let p=sequence::QueueIdentity{tag:0,aux_index:0};let a=sequence::QueueIdentity{tag:1,aux_index:0};
 let original_sequence=c.begin_sequence(device::QueueIdentity::Primary).unwrap();let fixture_sequence=d.begin_cb(p).unwrap();
 assert_eq!(original_sequence,fixture_sequence+1); // Bounded fixture adapter, not production mapping.
 let r=record::record(4,None,true);assert!(!r.panic);assert_eq!(r.command_states,[2,1]);
 let s=record::submit(r.command_states,false,false);assert_eq!(s.command_states,[2,2]);
 let (queue,timeline)=s.receipt.unwrap();assert_eq!(queue,0);assert_eq!(timeline,41);assert_ne!(timeline,original_sequence);
 assert_eq!(d.submit(a,fixture_sequence,timeline),Err(sequence::Error::QueueMismatch));
 d.submit(p,fixture_sequence,timeline).unwrap();assert!(d.complete(p,timeline-1).unwrap().is_empty());
 assert_eq!(d.complete(p,timeline).unwrap(),[fixture_sequence]);d.dispose(fixture_sequence).unwrap();
}
#[test] fn callback_and_end_failure_block_submit() {
 let r=record::record(3,Some(1),true);assert!(r.panic);assert_eq!(r.command_states,[1]);assert_eq!(r.consumed_passes,2);
 let s=record::submit(vec![2,1],true,false);assert!(s.panic);assert!(s.receipt.is_none());assert!(!s.events.iter().any(|e|matches!(e,record::Event::Queue(_))));
}
''' % tuple(str(candidates[name] / "src/lib.rs").replace("\\", "/") for name in ["device", "r5", "record"]), encoding="utf-8")
    executable = tooling / "batch04-cross-crate.exe"
    command("cross-crate-build", ["rustc", "--edition", "2021", "--test", harness, "-o", executable])
    output = command("cross-crate-tests", [executable, "--skip", "device::", "--skip", "sequence::", "--skip", "record::"])
    check("executable_cross_crate_contract", "test result: ok. 3 passed" in output, "Actual candidate functions validate queue collision/fixture seed boundary, timeline threshold and record failure effects")

    # Hash every original container and all 638 exact SPIR-V spans.
    shader_root = original / "compiled-shaders"
    shipped = sorted(shader_root.glob("*.bin"), key=lambda p:p.name.lower())
    actual_inventory = [{"name": p.name, "size": p.stat().st_size, "sha256": sha(track(p))} for p in shipped]
    check("shader_contract_inventory", digest(actual_inventory, sort_keys=True) == contract["packages"]["R6C-pipeline-shader-interfaces"]["inventory_sha256"] == pv["identity"]["batch04_contract_inventory_sha256"], "404 source size/name/digest records match batch contract")
    check("shader_catalog_inventory", sorted((f["file"], f["bytes"], f["sha256"]) for f in catalog["files"]) == sorted((f["name"], f["size"], f["sha256"]) for f in actual_inventory) and inventory["files"] == [{"file": f["file"], "bytes": f["bytes"], "sha256": f["sha256"]} for f in catalog["files"]], "Catalog, inventory and original files agree")
    check("shader_inventory_digest", digest([{ "file": f["file"], "sha256": f["sha256"]} for f in catalog["files"]], separators=(",", ":")) == inventory["digest"], "Recomputed compact inventory digest")
    modules = [m for f in catalog["files"] for m in f["modules"]]
    span_errors = []
    for file in catalog["files"]:
        data = (shader_root / file["file"]).read_bytes()
        for module in file["modules"]:
            start, end, words = module["offset"], module["container_end"], module["word_count"]
            if start < 4 or end > len(data) or end != start + 4 * words or struct.unpack_from("<I", data, start-4)[0] != words or struct.unpack_from("<I", data, start)[0] != 0x07230203 or hashlib.sha256(data[start:end]).hexdigest() != module["sha256"]: span_errors.append(module["id"])
    check("shader_all_payload_hashes", not span_errors and len(modules) == 638, {"module_spans": len(modules), "errors": span_errors})
    reflected = json.loads(command("fresh-reflection", [candidates["pipeline"] / "target/debug/reflect-shaders.exe", shader_root]))
    mismatches = []
    by_file = {f["file"]: f for f in catalog["files"]}
    for file in reflected:
        saved = by_file.get(file["file"])
        if not saved or file["bytes"] != saved["bytes"] or len(file["modules"]) != len(saved["modules"]): mismatches.append(file["file"]); continue
        for fresh, prior in zip(file["modules"], saved["modules"]):
            # Family IDs/raw declarations are catalog annotations, absent from spirq output.
            old = {k: prior[k] for k in fresh}
            if "entries" in old: old["entries"] = [{k:v for k,v in e.items() if k not in ["layout_family", "interface_family"]} for e in old["entries"]]
            if fresh != old: mismatches.append(prior["id"])
    check("shader_fresh_reflection", len(reflected) == 404 and not mismatches and all(not m.get("error") for f in reflected for m in f["modules"]), {"files": len(reflected), "mismatches": mismatches})
    fresh_iv = json.loads(command("fresh-interface-validation", [candidates["pipeline"] / "target/debug/validate-catalog.exe", pipeline / "catalog.json"]))
    check("shader_executable_interface_validation", fresh_iv == iv and iv["files"] == 404 and iv["raster_pairs"] == 234 and all(iv[x] == 0 for x in ["descriptor_failures", "push_range_failures", "stage_location_failures"]), "Fresh Rust descriptor merge/push range/stage-location checks match retained per-file results")
    entries = [e for m in modules for e in m.get("entries", [])]
    for family_key, field, prefix, expected in [("layout_families", "layout_family", "layout", 240), ("interface_families", "interface_family", "interface", 402)]:
        groups = families[family_key]
        errors = [f["id"] for f in groups if f["id"] != prefix + "-" + digest(f["signature"], separators=(",", ":"))]
        entry_members = sorted((m["id"], e["stage"], e["entry_point"], e[field]) for m in modules for e in m.get("entries", []))
        family_members = sorted((member["module_id"], member["stage"], member["entry_point"], f["id"]) for f in groups for member in f["members"])
        check("shader_" + family_key + "_coverage", len(groups) == expected and not errors and entry_members == family_members, {"families": len(groups), "entry_members": len(entry_members), "hash_errors": errors})
    def strip_names(value):
        if isinstance(value, list): return [strip_names(v) for v in value]
        if isinstance(value, dict): return {k:strip_names(v) for k,v in value.items() if k != "name"}
        return value
    family_signature_errors = []
    for module in modules:
        for entry in module.get("entries", []):
            layout = {"descriptors": [{"set": d["set"], "binding": d["binding"], "descriptor_type": d["descriptor_type"].split("(")[0], "count": d["count"]} for d in entry["descriptors"]], "push_constants": [strip_names(p["type"]) for p in entry["push_constants"]]}
            interface = {"stage": entry["stage"], "descriptors": [strip_names(d) for d in entry["descriptors"]], "inputs": [strip_names(v) for v in entry["inputs"]], "outputs": [strip_names(v) for v in entry["outputs"]], "push_constants": [strip_names(v) for v in entry["push_constants"]]}
            if entry["layout_family"] != "layout-" + digest(layout, separators=(",", ":")) or entry["interface_family"] != "interface-" + digest(interface, separators=(",", ":")): family_signature_errors.append(module["id"])
    check("shader_family_signatures_recomputed_from_entries", not family_signature_errors, {"entries": len(entries), "mismatches": family_signature_errors})
    check("shader_counts_recomputed", len(entries) == 638 and Counter(e["stage"] for e in entries) == summary["stages"] and len(modules) == summary["reflected_modules"] and summary["reflection_failed_modules"] == 0, "638 entries across 170 compute/234 vertex/234 fragment, no reflection failure")
    shader_contract = load(pipeline / "implementation-contract.json")
    check("shader_library_reused_boundary", shader_contract["library"]["reimplemented"] is False and shader_contract["library"]["commit"] == "4f22e6f149750401705d47ae15a60e77cd5a6905" and pv["target_execution"] is False and pv["database_opened"] is False, "Original spirq fork reused; static interfaces do not validate algorithms/runtime/Vulkan")
    track(harness);track(root / "scripts/render-recovery/validate_fourth_batch_contract.py")
    failures = [c["name"] for c in checks if not c["ok"]]
    return {"schema_version": 2, "status": "consistent_partial" if not failures else "inconsistent", "scope": "R6A counter producers -> bounded R5B fixture adapter; R6B scoped record/submit -> timeline; R6C shipped interfaces", "identity": {"binary_sha256": BINARY_SHA, "original_lock_sha256": LOCK_SHA}, "inputs": inputs, "checks": checks, "failed_checks": failures, "commands": commands, "counts": {"candidate_tests": tests, "cross_crate_tests": 3, "fresh_native_device_cases": len(cases), "fresh_native_record_cases": len(observations), "shader_files": len(shipped), "shader_modules": len(modules)}, "integration_boundaries": [{"interface": "R6A -> R5B", "production_connected": False, "fixture_seed": 0, "original_counter_seed": 1, "reason": "R5B has no external-seed/sequence injection API. Bounded test adapter demonstrates semantics; production integration needs an explicit recovered sequence consumer."}, {"interface": "R6B -> GPU queue", "production_connected": False, "scope": "empty-resource sequential record plus injected prebuilt queue tail"}, {"interface": "R6C -> country-core", "production_connected": False, "scope": "shipped descriptor/push/stage interfaces; name associations are candidates"}], "remaining": ["private multi-queue caller reachability and resource exclusivity", "R5B production external-sequence API/seed adaptation", "nonempty resource and parallel full pass recording", "complete queue-array assembly, Vulkan/GPU runtime", "pipeline/descriptor creation and concrete country-core pass selection", "temporal reset/drop and full image equivalence"], "negative_search_limits": "Xref completeness unknown when auto-analysis is disabled. Empty/DATA-only database records cannot prove whole-program absence; workgraph direct-call evidence must be considered.", "target_execution": "CPU instruction emulation only; no game process or GPU", "database_opened": False}

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--project-root", type=Path, default=Path(__file__).resolve().parents[2])
    parser.add_argument("--original-root", type=Path, default=Path("D:/game/ljxsj_92385/Tiny Glade"))
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    root = args.project_root.resolve()
    output = args.output or root / "evidence/render-recovery/parallel-batch-04/integration.json"
    result = build(root, args.original_root.resolve())
    output.write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({"status": result["status"], "checks": len(result["checks"]), "failed": result["failed_checks"], "counts": result["counts"]}))
    if result["failed_checks"]: raise SystemExit(1)

if __name__ == "__main__": main()
