"""Build a deterministic work graph for the observed rhapsody native instances."""

from __future__ import annotations

import argparse
import bisect
import hashlib
import json
import re
import shutil
import struct
import subprocess
from collections import Counter, defaultdict
from pathlib import Path


PACKAGE_ORDER = [
    "A-descriptors-sync",
    "B-device-resource-creation",
    "C-gpu-lifecycle",
    "D-graph-compiler",
    "E-graph-executor",
    "F-pipeline-binding",
    "G-swapchain-diagnostics",
]

PACKAGE_TITLES = {
    "A-descriptors-sync": "描述与同步模型",
    "B-device-resource-creation": "设备与资源创建",
    "C-gpu-lifecycle": "GPU 生命周期",
    "D-graph-compiler": "图编译器",
    "E-graph-executor": "图执行器",
    "F-pipeline-binding": "Pipeline 与绑定",
    "G-swapchain-diagnostics": "交换链与诊断",
}

INSTRUCTION = re.compile(r"^\s*([0-9a-fA-F]+):\s+(?:[0-9a-fA-F]{2}\s+)+\s*([a-zA-Z][\w.]*)\s*(.*)$")
DIRECT_TARGET = re.compile(r"^(0x[0-9a-fA-F]+)")
HASH_SUFFIX = re.compile(r"::h[0-9a-f]{16}(?=$|\s|\.)")
LLVM_SUFFIX = re.compile(r"\s*\(\.llvm\.\d+\)$")


def read_json(path: Path):
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path: Path, value) -> None:
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as source:
        for block in iter(lambda: source.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def pe_runtime_function_sizes(binary: Path) -> dict[str, int]:
    raw = binary.read_bytes()
    pe = struct.unpack_from("<I", raw, 0x3C)[0]
    if raw[pe : pe + 4] != b"PE\0\0":
        raise ValueError(f"not a PE image: {binary}")
    section_count = struct.unpack_from("<H", raw, pe + 6)[0]
    optional_size = struct.unpack_from("<H", raw, pe + 20)[0]
    optional = pe + 24
    if struct.unpack_from("<H", raw, optional)[0] != 0x20B:
        raise ValueError("expected PE32+ image")
    image_base = struct.unpack_from("<Q", raw, optional + 24)[0]
    exception_rva, exception_size = struct.unpack_from("<II", raw, optional + 112 + 3 * 8)
    sections = []
    section_table = optional + optional_size
    for index in range(section_count):
        at = section_table + index * 40
        virtual_size, virtual_address, raw_size, raw_offset = struct.unpack_from("<IIII", raw, at + 8)
        sections.append((virtual_address, max(virtual_size, raw_size), raw_offset, raw_size))

    def rva_to_offset(rva: int) -> int:
        for virtual_address, span, raw_offset, raw_size in sections:
            if virtual_address <= rva < virtual_address + span:
                delta = rva - virtual_address
                if delta >= raw_size:
                    raise ValueError(f"RVA is not file-backed: {hex(rva)}")
                return raw_offset + delta
        raise ValueError(f"RVA is outside PE sections: {hex(rva)}")

    exception_offset = rva_to_offset(exception_rva)
    sizes = {}
    for at in range(exception_offset, exception_offset + exception_size, 12):
        begin, end, _unwind = struct.unpack_from("<III", raw, at)
        if begin and end > begin:
            sizes[hex(image_base + begin)] = end - begin
    return sizes


def strip_generic_arguments(name: str) -> str:
    result = []
    depth = 0
    for char in name:
        if char == "<":
            depth += 1
            if depth == 1:
                continue
        elif char == ">" and depth:
            depth -= 1
            if depth == 0:
                continue
        if depth == 0:
            result.append(char)
    return "".join(result)


def semantic_family(name: str) -> str:
    name = LLVM_SUFFIX.sub("", HASH_SUFFIX.sub("", name))
    return strip_generic_arguments(name)


def operation_name(name: str) -> str:
    normalized = semantic_family(name)
    return normalized.rsplit("::", 1)[-1]


def module_name(name: str) -> str:
    parts = name.split("::")
    if len(parts) < 3:
        return parts[1] if len(parts) > 1 else "root"
    if parts[1] == "raw" and len(parts) > 3 and parts[2] == "managed":
        return "raw::managed::" + parts[3]
    return parts[1] + "::" + parts[2]


def classify(name: str) -> str:
    lowered = name.lower()
    if "::fmt" in name or "print_log" in lowered or "dump_log" in lowered:
        return "diagnostic_or_formatting"
    if any(token in name for token in ("::drop", "::clone", "::deref", "::as_ref", "::borrow")):
        return "trait_or_lifecycle_glue"
    if "closure" in lowered:
        return "inlined_or_closure_instance"
    if "<" in name:
        return "generic_instance"
    return "owned_semantic_candidate"


def assign_package(name: str) -> str:
    lowered = name.lower()
    if name.startswith(("rhapsody::leek", "rhapsody::init_logging", "rhapsody::dump_log", "rhapsody::print_log_dump")):
        return "G-swapchain-diagnostics"
    if "rhapsody::raw::managed::" in name or any(
        token in lowered
        for token in (
            "rhapsody::raw::command_buffer",
            "rhapsody::raw::semaphore",
            "rhapsody::raw::submit",
            "rhapsody::raw::disposal",
            "rhapsody::raw::dynamic_constants",
            "rhapsody::raw::temp_list",
        )
    ):
        return "C-gpu-lifecycle"
    if any(
        token in lowered
        for token in (
            "rhapsody::raw::swapchain",
            "rhapsody::raw::profiler",
            "rhapsody::raw::stats",
            "rhapsody::raw::debug",
            "rhapsody::raw::crash",
            "rhapsody::raw::ray_tracing",
        )
    ):
        return "G-swapchain-diagnostics"
    if any(
        token in lowered
        for token in (
            "rhapsody::raw::shader",
            "rhapsody::raw::descriptor_set",
            "rhapsody::raw::render_pass",
            "rhapsody::raw::framebuffer",
            "rhapsody::rg::binding",
        )
    ):
        return "F-pipeline-binding"
    if any(
        token in lowered
        for token in (
            "rhapsody::rg::recorder",
            "rhapsody::rg::executor",
            "rhapsody::rg::import_export",
            "rhapsody::rg::temporal",
            "rhapsody::rg::util",
        )
    ):
        return "E-graph-executor"
    if "rhapsody::rg::" in name:
        return "D-graph-compiler"
    if any(
        token in lowered
        for token in (
            "rhapsody::raw::instance",
            "rhapsody::raw::init",
            "rhapsody::raw::physical_device",
            "rhapsody::raw::device",
            "rhapsody::raw::allocator",
        )
    ):
        return "B-device-resource-creation"
    if any(
        token in lowered
        for token in (
            "rhapsody::raw::format",
            "rhapsody::raw::buffer",
            "rhapsody::raw::texture",
            "rhapsody::raw::binding",
            "rhapsody::raw::barrier",
            "rhapsody::raw::bytes",
            "rhapsody::raw::sampler",
        )
    ):
        if any(token in lowered for token in ("::create", "::new_buffer", "::new_texture", "::allocate")):
            return "B-device-resource-creation"
        return "A-descriptors-sync"
    return "G-swapchain-diagnostics"


def locate_objdump(explicit: str | None) -> Path:
    candidates = [
        Path(explicit) if explicit else None,
        Path(shutil.which("llvm-objdump") or ""),
        Path(r"C:\Program Files\LLVM\bin\llvm-objdump.exe"),
    ]
    for candidate in candidates:
        if candidate and candidate.is_file():
            return candidate
    raise FileNotFoundError("llvm-objdump was not found")


def merge_ranges(functions: list[dict], merge_gap: int, max_span: int) -> list[tuple[int, int]]:
    ranges = []
    for function in functions:
        start = int(function["va"], 16)
        end = start + function["code_size"]
        if not ranges or start - ranges[-1][1] > merge_gap or end - ranges[-1][0] > max_span:
            ranges.append([start, end])
        else:
            ranges[-1][1] = max(ranges[-1][1], end)
    return [(start, end) for start, end in ranges]


def scan_calls(binary: Path, objdump: Path, functions: list[dict], merge_gap: int, max_span: int):
    starts = [int(function["va"], 16) for function in functions]
    by_start = {int(function["va"], 16): function for function in functions}
    calls = {function["va"]: {"direct": [], "external_jumps": [], "indirect_calls": [], "instruction_count": 0} for function in functions}
    batch_reports = []
    for batch_start, batch_end in merge_ranges(functions, merge_gap, max_span):
        command = [
            str(objdump),
            "-d",
            f"--start-address={hex(batch_start)}",
            f"--stop-address={hex(batch_end)}",
            str(binary),
        ]
        completed = subprocess.run(command, capture_output=True, text=True, encoding="utf-8", errors="replace")
        if completed.returncode:
            raise RuntimeError(completed.stderr.strip() or f"llvm-objdump failed for {hex(batch_start)}")
        decoded = 0
        for line in completed.stdout.splitlines():
            match = INSTRUCTION.match(line)
            if not match:
                continue
            address = int(match.group(1), 16)
            index = bisect.bisect_right(starts, address) - 1
            if index < 0:
                continue
            function = by_start[starts[index]]
            function_end = starts[index] + function["code_size"]
            if not starts[index] <= address < function_end:
                continue
            decoded += 1
            entry = calls[function["va"]]
            entry["instruction_count"] += 1
            mnemonic = match.group(2).lower()
            operands = match.group(3).strip()
            direct = DIRECT_TARGET.match(operands)
            if mnemonic.startswith("call"):
                if direct:
                    entry["direct"].append({"site": hex(address), "target": hex(int(direct.group(1), 16))})
                else:
                    entry["indirect_calls"].append({"site": hex(address), "operand": operands})
            elif mnemonic.startswith("jmp") and direct:
                target = int(direct.group(1), 16)
                if not starts[index] <= target < function_end:
                    entry["external_jumps"].append({"site": hex(address), "target": hex(target)})
        batch_reports.append({"start": hex(batch_start), "end_exclusive": hex(batch_end), "instructions_in_function_ranges": decoded})
    return calls, batch_reports


def build(args: argparse.Namespace) -> dict:
    root = args.project_root.resolve()
    procedures_path = args.procedures or root / "evidence/render-skeleton/native-procedures.json"
    symbols_path = args.symbols or root / "evidence/render-architecture/public-symbols.json"
    binary = args.binary.resolve()
    output = args.output or root / "evidence/render-recovery/workgraph"
    output.mkdir(parents=True, exist_ok=True)

    records = [row for row in read_json(procedures_path) if row["name"].startswith("rhapsody::")]
    public_records = [
        row
        for row in read_json(symbols_path)
        if row.get("va") and (row.get("demangled") or row.get("name", "")).startswith("rhapsody::")
    ]
    grouped = defaultdict(list)
    public_grouped = defaultdict(list)
    for row in records:
        grouped[row["va"].lower()].append(row)
    for row in public_records:
        public_grouped[row["va"].lower()].append(row)

    runtime_sizes = pe_runtime_function_sizes(binary)
    unbounded_public_symbols = []

    functions = []
    for va in sorted(set(grouped) | set(public_grouped), key=lambda value: int(value, 16)):
        aliases = grouped.get(va, [])
        public_aliases = public_grouped.get(va, [])
        sizes = sorted({row["code_size"] for row in aliases})
        if len(sizes) > 1:
            raise ValueError(f"conflicting code sizes at {va}: {sizes}")
        if sizes:
            code_size = sizes[0]
            size_source = "PDB procedure"
        elif va in runtime_sizes:
            code_size = runtime_sizes[va]
            size_source = "PE exception directory RUNTIME_FUNCTION"
        else:
            unbounded_public_symbols.extend(
                {"va": va, "name": row.get("demangled") or row.get("name")} for row in public_aliases
            )
            continue
        names = sorted(
            {row["name"] for row in aliases}
            | {row.get("demangled") or row.get("name") for row in public_aliases}
        )
        packages = Counter(assign_package(name) for name in names)
        selected_package = min(packages, key=lambda package: (-packages[package], PACKAGE_ORDER.index(package)))
        families = sorted({semantic_family(name) for name in names})
        operations = sorted({operation_name(name) for name in names})
        modules = sorted({module_name(name) for name in names})
        functions.append(
            {
                "va": va,
                "code_size": code_size,
                "end_exclusive": hex(int(va, 16) + code_size),
                "size_source": size_source,
                "names": names,
                "semantic_families": families,
                "operation_names": operations,
                "modules": modules,
                "classifications": dict(sorted(Counter(classify(name) for name in names).items())),
                "work_package": selected_package,
                "work_package_votes": dict(sorted(packages.items())),
                "shared_address_semantic_collision": len(packages) > 1 or len(operations) > 1,
                "pdb_records": len(aliases),
                "public_symbol_records": len(public_aliases),
                "observed_sources": sorted(
                    ({"PDB procedure"} if aliases else set())
                    | ({"PDB public symbol"} if public_aliases else set())
                ),
                "module_indices": sorted({row["module_index"] for row in aliases}),
                "raw_evidence": sorted({row["raw_evidence"] for row in aliases}),
            }
        )
    functions.sort(key=lambda function: int(function["va"], 16))

    symbol_map = defaultdict(set)
    for row in read_json(symbols_path):
        va = row.get("va")
        if va:
            symbol_map[va.lower()].add(row.get("demangled") or row.get("name"))
    for function in functions:
        symbol_map[function["va"]].update(function["names"])

    calls, batches = scan_calls(
        binary,
        locate_objdump(args.llvm_objdump),
        functions,
        args.merge_gap,
        args.max_batch_span,
    )

    function_by_va = {function["va"]: function for function in functions}
    edges = defaultdict(lambda: {"sites": [], "kinds": set()})
    unresolved_direct = 0
    indirect_calls = 0
    for source, call_info in calls.items():
        indirect_calls += len(call_info["indirect_calls"])
        for kind, rows in (("call", call_info["direct"]), ("tail_jump", call_info["external_jumps"])):
            for row in rows:
                target = row["target"].lower()
                edge = edges[(source, target)]
                edge["sites"].append(row["site"])
                edge["kinds"].add(kind)
                if target not in symbol_map:
                    unresolved_direct += 1

    serialized_edges = []
    package_edges = Counter()
    for (source, target), edge in sorted(edges.items(), key=lambda item: (int(item[0][0], 16), int(item[0][1], 16))):
        source_function = function_by_va[source]
        target_function = function_by_va.get(target)
        source_package = source_function["work_package"]
        target_package = target_function["work_package"] if target_function else None
        if target_package:
            package_edges[(source_package, target_package)] += len(edge["sites"])
        serialized_edges.append(
            {
                "source": source,
                "source_names": source_function["names"],
                "target": target,
                "target_names": sorted(symbol_map.get(target, [])),
                "target_is_rhapsody": target in function_by_va,
                "sites": edge["sites"],
                "kinds": sorted(edge["kinds"]),
            }
        )

    packages = []
    for package_id in PACKAGE_ORDER:
        members = [function for function in functions if function["work_package"] == package_id]
        families = sorted({family for function in members for family in function["semantic_families"]})
        packages.append(
            {
                "id": package_id,
                "title": PACKAGE_TITLES[package_id],
                "status": "inventory_ready",
                "unique_addresses": len(members),
                "pdb_records": sum(function["pdb_records"] for function in members),
                "machine_code_bytes": sum(function["code_size"] for function in members),
                "semantic_families": len(families),
                "shared_address_semantic_collisions": sum(function["shared_address_semantic_collision"] for function in members),
                "modules": dict(sorted(Counter(module for function in members for module in function["modules"]).items())),
                "classification_counts": dict(sorted(Counter(key for function in members for key in function["classifications"]).items())),
                "function_vas": [function["va"] for function in members],
                "next_gate": "recover shared types and state invariants before implementation",
            }
        )

    summary = {
        "ok": sum(function["pdb_records"] for function in functions) == len(records),
        "scope": "observed rhapsody PDB native records in the selected Tiny Glade build",
        "binary": str(binary),
        "binary_sha256": sha256(binary),
        "procedures_sha256": sha256(procedures_path),
        "symbols_sha256": sha256(symbols_path),
        "pdb_records": len(records),
        "public_symbol_records": len(public_records),
        "unique_public_symbol_addresses": len(public_grouped),
        "bounded_public_only_addresses": sum(not function["pdb_records"] for function in functions),
        "unbounded_public_symbols": unbounded_public_symbols,
        "unique_addresses": len(functions),
        "semantic_families": len({family for function in functions for family in function["semantic_families"]}),
        "shared_address_semantic_collisions": sum(function["shared_address_semantic_collision"] for function in functions),
        "direct_edge_pairs": len(serialized_edges),
        "direct_call_sites": sum(len(edge["sites"]) for edge in serialized_edges),
        "unresolved_direct_call_sites": unresolved_direct,
        "indirect_call_sites": indirect_calls,
        "functions_without_decoded_instructions": [function["va"] for function in functions if calls[function["va"]]["instruction_count"] == 0],
        "objdump_batches": len(batches),
        "work_packages": {package["id"]: package["unique_addresses"] for package in packages},
        "limitations": [
            "PDB void signatures do not recover Rust ABI",
            "Static direct calls do not establish runtime execution",
            "Indirect calls and vtable targets remain unresolved",
            "ICF/shared-address names are retained as collisions instead of being merged semantically",
            "Initial work-package assignment is deterministic routing, not a proof of original source ownership",
        ],
    }
    if summary["functions_without_decoded_instructions"]:
        summary["ok"] = False
    if unbounded_public_symbols:
        summary["ok"] = False

    write_json(output / "functions.json", functions)
    write_json(output / "calls.json", {"functions": calls, "edges": serialized_edges})
    write_json(
        output / "work-packages.json",
        {
            "packages": packages,
            "package_edges": [
                {"source": source, "target": target, "call_sites": count}
                for (source, target), count in sorted(package_edges.items())
            ],
        },
    )
    write_json(output / "disassembly-batches.json", batches)
    write_json(output / "summary.json", summary)
    readme = [
        "# rhapsody 工作图",
        "",
        "由 `scripts/render-recovery/build_rhapsody_workgraph.py` 从 PDB 范围和原二进制确定性生成。",
        "它是工作分包与缺口导航，不是源码恢复完成证明。",
        "",
        f"- PDB 记录：{summary['pdb_records']}",
        f"- Public symbol 记录：{summary['public_symbol_records']}",
        f"- 仅 public symbol 的已定界入口：{summary['bounded_public_only_addresses']}",
        f"- 唯一地址：{summary['unique_addresses']}",
        f"- 语义族：{summary['semantic_families']}",
        f"- 直接调用位置：{summary['direct_call_sites']}",
        f"- 间接调用位置：{summary['indirect_call_sites']}",
        f"- 共享地址语义冲突：{summary['shared_address_semantic_collisions']}",
        "",
        "下一步只对 D 包的 `ResourceInfo::calculate → registry/pool → compiled/state` 缺口做定向取证。",
    ]
    (output / "README.md").write_text("\n".join(readme) + "\n", encoding="utf-8")
    return summary


def parse_args() -> argparse.Namespace:
    script = Path(__file__).resolve()
    root = script.parents[2]
    parser = argparse.ArgumentParser()
    parser.add_argument("--project-root", type=Path, default=root)
    parser.add_argument(
        "--binary",
        type=Path,
        default=Path(r"D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne"),
    )
    parser.add_argument("--procedures", type=Path)
    parser.add_argument("--symbols", type=Path)
    parser.add_argument("--output", type=Path)
    parser.add_argument("--llvm-objdump")
    parser.add_argument("--merge-gap", type=lambda value: int(value, 0), default=0x4000)
    parser.add_argument("--max-batch-span", type=lambda value: int(value, 0), default=0x40000)
    return parser.parse_args()


if __name__ == "__main__":
    result = build(parse_args())
    print(json.dumps(result, ensure_ascii=False, indent=2))
    raise SystemExit(0 if result["ok"] else 1)
