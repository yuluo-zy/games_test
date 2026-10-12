"""Condense the rhapsody work graph into the R1 graph-compiler contract."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
from collections import Counter, defaultdict, deque
from pathlib import Path


CORE_PATTERNS = [
    re.compile(r"ResourceInfo::"),
    re.compile(r"ResourcePool::"),
    re.compile(r"ResourceRegistry::"),
    re.compile(r"CompiledRenderGraph.*::(?:new|add_pre_pass_barriers|record_batch)"),
    re.compile(r"BatchCompiler::"),
    re.compile(r"RenderGraphState::compile"),
]

CRITICAL_OPERATIONS = {
    "ResourceInfo::calculate": 100,
    "RenderGraphState::compile": 95,
    "CompiledRenderGraph": 90,
    "ResourceRegistry::populate_resources": 85,
    "ResourceRegistry::into_exports": 80,
    "ResourcePool::take_": 75,
    "ResourcePool::insert_": 75,
    "BatchCompiler": 70,
}

VERIFIED = {
    "ResourceInfo::link_up_next_access_chains": {
        "source": "reconstruction/render/resource_info.rs",
        "report": "evidence/render-recovery/verification.json",
    },
    "ResourceInfo::calculate_lifetime_events": {
        "source": "reconstruction/render/resource_info.rs",
        "report": "evidence/render-recovery/verification.json",
    },
    "buffer_access_mask_to_usage_flags": {
        "source": "reconstruction/render/resource_usage.rs",
        "report": "evidence/render-recovery/verification.json",
    },
    "image_access_mask_to_usage_flags": {
        "source": "reconstruction/render/resource_usage.rs",
        "report": "evidence/render-recovery/verification.json",
    },
}


def read_json(path: Path):
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path: Path, value) -> None:
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def name_blob(function: dict) -> str:
    return " | ".join(function["names"])


def matches_core(function: dict) -> bool:
    names = name_blob(function)
    return any(pattern.search(names) for pattern in CORE_PATTERNS)


def verification_for(function: dict) -> list[dict]:
    names = name_blob(function)
    return [dict(name=needle, **evidence) for needle, evidence in VERIFIED.items() if needle in names]


def priority(function: dict) -> int:
    names = name_blob(function)
    score = max((value for needle, value in CRITICAL_OPERATIONS.items() if needle in names), default=20)
    score += min(function["code_size"] // 256, 20)
    if function["pdb_records"] == 0:
        score += 10
    if verification_for(function):
        score -= 100
    return score


def build(args: argparse.Namespace) -> dict:
    root = args.project_root.resolve()
    workgraph = args.workgraph or root / "evidence/render-recovery/workgraph"
    output = args.output or root / "evidence/render-recovery/graph-compiler"
    output.mkdir(parents=True, exist_ok=True)

    functions_path = workgraph / "functions.json"
    calls_path = workgraph / "calls.json"
    summary_path = workgraph / "summary.json"
    functions = read_json(functions_path)
    calls = read_json(calls_path)
    summary = read_json(summary_path)
    by_va = {function["va"]: function for function in functions}

    outgoing = defaultdict(list)
    incoming = defaultdict(list)
    for edge in calls["edges"]:
        outgoing[edge["source"]].append(edge)
        incoming[edge["target"]].append(edge)

    core = {function["va"] for function in functions if matches_core(function)}
    selected = set(core)
    frontier = deque((va, 0) for va in core)
    while frontier:
        va, depth = frontier.popleft()
        if depth >= args.internal_depth:
            continue
        for edge in outgoing[va]:
            target = edge["target"]
            function = by_va.get(target)
            if function and function["work_package"] in {
                "A-descriptors-sync",
                "D-graph-compiler",
                "E-graph-executor",
            } and target not in selected:
                selected.add(target)
                frontier.append((target, depth + 1))

    selected_functions = []
    for va in sorted(selected, key=lambda value: int(value, 16)):
        function = by_va[va]
        decompile = root / "evidence/render-architecture" / f"{va}.c"
        internal_edges = [edge for edge in outgoing[va] if edge["target"] in selected]
        rhapsody_edges = [
            edge for edge in outgoing[va] if edge["target_is_rhapsody"] and edge["target"] not in selected
        ]
        external_edges = [edge for edge in outgoing[va] if not edge["target_is_rhapsody"]]
        selected_functions.append(
            {
                "va": va,
                "code_size": function["code_size"],
                "names": function["names"],
                "work_package": function["work_package"],
                "is_core": va in core,
                "priority": priority(function),
                "public_only_entry": function["pdb_records"] == 0,
                "source_bound_by_pdb": function["pdb_records"] > 0,
                "existing_decompilation": str(decompile.relative_to(root)).replace("\\", "/") if decompile.is_file() else None,
                "verification": verification_for(function),
                "internal_call_targets": sorted({edge["target"] for edge in internal_edges}),
                "other_rhapsody_call_targets": sorted({edge["target"] for edge in rhapsody_edges}),
                "external_call_sites": sum(len(edge["sites"]) for edge in external_edges),
                "incoming_selected_callers": sorted(
                    {edge["source"] for edge in incoming[va] if edge["source"] in selected}
                ),
                "indirect_call_sites": len(calls["functions"][va]["indirect_calls"]),
            }
        )

    query_candidates = [
        function
        for function in selected_functions
        if function["is_core"] and not function["existing_decompilation"] and not function["verification"]
    ]
    query_candidates.sort(key=lambda function: (-function["priority"], -function["code_size"], int(function["va"], 16)))
    query_queue = [
        {
            "address": function["va"],
            "names": function["names"],
            "requested_evidence": [
                "decompilation",
                "direct callers and callees",
                "field reads and writes",
                "panic/error branches",
            ],
            "question": "Which shared type fields and state invariant does this function establish or consume?",
        }
        for function in query_candidates[: args.query_limit]
    ]

    core_edges = [
        edge
        for edge in calls["edges"]
        if edge["source"] in selected and edge["target"] in selected
    ]
    type_ledger = [
        {
            "type": type_name,
            "status": status,
            "required_facts": facts,
        }
        for type_name, status, facts in (
            ("ResourceInfo", "partial", ["vector fields and offsets", "access record layout", "lifetime event encoding"]),
            ("PassResourceAccess", "partial", ["access kind", "synchronization group", "previous/next indices"]),
            ("ResourcePool", "unknown", ["texture/buffer key", "compatibility predicate", "reuse ownership"]),
            ("ResourceRegistry", "unknown", ["registry resource variants", "physical handle mapping", "export ownership"]),
            ("CompiledRenderGraph", "unknown", ["batch layout", "barrier placement", "pass/resource ranges"]),
            ("BatchCompiler", "unknown", ["batch flush condition", "queue/pass range fields"]),
            ("RenderGraphState", "partial", ["compile inputs", "persistent pool/state", "monomorphized game closure boundary"]),
        )
    ]

    contract = {
        "status": "evidence_contract_started",
        "scope": "R1 graph compiler: ResourceInfo through registry/pool and compiled graph",
        "identity": {
            "binary_sha256": summary["binary_sha256"],
            "workgraph_summary_sha256": sha256(summary_path),
            "functions_sha256": sha256(functions_path),
            "calls_sha256": sha256(calls_path),
        },
        "observations": [
            "ResourceInfo::calculate and RenderGraphState::compile are public-only entries in the retained PDB procedure inventory and are bounded by PE RUNTIME_FUNCTION records.",
            "Three RenderGraphState::compile monomorphs exist; their machine bodies can include game closure work and cannot be attributed wholesale to generic rhapsody source.",
            "Four ResourceInfo leaf rules already have scoped native differential verification.",
            "ResourceRegistry::populate_resources is a 5274-byte emitted private function and crosses into resource creation and pool reuse.",
        ],
        "inferences": [
            "The fastest recovery unit is the compiled-graph snapshot boundary, not each helper function.",
            "ResourceInfo::calculate should be recovered before interpreting registry allocation and batch compilation fields.",
        ],
        "unknowns": [
            "Complete ResourceInfo and access-record layouts",
            "ResourcePool compatibility keys and ownership transfer",
            "Registry resource variants and import/export representation",
            "CompiledRenderGraph batch and barrier field layout",
            "Exact generic rhapsody portion of each monomorphized RenderGraphState::compile body",
            "Indirect call targets in the selected closure",
        ],
        "type_ledger": type_ledger,
        "functions": selected_functions,
        "internal_edges": core_edges,
        "query_queue": query_queue,
        "acceptance": {
            "implementation_unit": "pass/resource declaration to compiled graph snapshot",
            "oracle_fields": [
                "pass order",
                "resource lifetime endpoints",
                "previous/next access chain",
                "physical allocation identity",
                "pre/post barriers",
                "import/export state",
                "temporal parity and history mapping",
            ],
            "not_accepted_by": [
                "number of translated functions",
                "successful compilation alone",
                "a single decompiler output",
                "PDB void signatures",
            ],
        },
        "limitations": [
            "This contract is generated from static evidence and does not observe runtime execution.",
            "Query queue items are evidence requests, not permission to mutate an analysis database.",
            "Existing decompilation is pseudocode and does not establish original Rust source or ABI.",
        ],
    }
    write_json(output / "contract.json", contract)
    write_json(output / "query-queue.json", query_queue)

    verified_count = sum(bool(function["verification"]) for function in selected_functions)
    decompiled_count = sum(bool(function["existing_decompilation"]) for function in selected_functions)
    markdown = [
        "# R1 图编译器合同",
        "",
        "状态：`evidence_contract_started`。这是工作包合同，不是实现或验证完成报告。",
        "",
        f"- 核心函数地址：{len(core)}",
        f"- 含一层内部依赖的地址：{len(selected_functions)}",
        f"- 已有反编译：{decompiled_count}",
        f"- 已有局部原码验证：{verified_count}",
        f"- 下一批定向查询：{len(query_queue)}",
        "",
        "## 已确认入口",
        "",
    ]
    for function in sorted(selected_functions, key=lambda item: (-item["priority"], int(item["va"], 16)))[:16]:
        markers = []
        if function["public_only_entry"]:
            markers.append("public-only")
        if function["existing_decompilation"]:
            markers.append("已有伪代码")
        if function["verification"]:
            markers.append("局部已验证")
        suffix = f"（{'；'.join(markers)}）" if markers else ""
        markdown.append(f"- `{function['va']}` / {function['code_size']} bytes：`{function['names'][0]}`{suffix}")
    markdown.extend(
        [
            "",
            "## 下一关口",
            "",
            "先关闭 `ResourceInfo`、访问记录和 compiled graph 的字段约束，再写整条 CPU 编译路径。",
            "查询只使用 `query-queue.json` 中的有限地址；不得重新扫描全 PDB 或逐函数翻译。",
        ]
    )
    (output / "README.md").write_text("\n".join(markdown) + "\n", encoding="utf-8")

    result = {
        "ok": True,
        "core_functions": len(core),
        "selected_functions": len(selected_functions),
        "existing_decompilations": decompiled_count,
        "verified_functions": verified_count,
        "query_queue": len(query_queue),
        "output": str(output),
    }
    write_json(output / "summary.json", result)
    return result


def parse_args() -> argparse.Namespace:
    root = Path(__file__).resolve().parents[2]
    parser = argparse.ArgumentParser()
    parser.add_argument("--project-root", type=Path, default=root)
    parser.add_argument("--workgraph", type=Path)
    parser.add_argument("--output", type=Path)
    parser.add_argument("--internal-depth", type=int, default=1)
    parser.add_argument("--query-limit", type=int, default=20)
    return parser.parse_args()


if __name__ == "__main__":
    result = build(parse_args())
    print(json.dumps(result, ensure_ascii=False, indent=2))
