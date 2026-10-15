"""Bind existing B chapters/FIX-LIST to compiler-reachable formal code.

Rust modules and associated APIs come from rustdoc JSON, not text matches.
Indexed caller relationships are navigation evidence; presence is not semantic
acceptance. Never turn an address comment or a name match into 'implemented'.
"""
import argparse
import hashlib
import json
import sqlite3
import subprocess
import tomllib
from collections import Counter, defaultdict
from pathlib import Path


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--workspace", type=Path, required=True)
    ap.add_argument("--project", type=Path, required=True)
    ap.add_argument("--output", type=Path, required=True)
    ap.add_argument("--markdown", type=Path)
    args = ap.parse_args()
    workspace, project = args.workspace.resolve(), args.project.resolve()
    root = workspace / "crates/country-core/src"
    docs_path = workspace / "target/doc/country_core.json"
    doc = json.loads(docs_path.read_text())
    index = doc["index"]
    reachable = set()
    module_paths = {}
    derived_traits = defaultdict(list)

    def walk(item_id, parents):
        item = index[str(item_id)]
        if item["crate_id"] != 0:
            return
        reachable.add(str(item_id))
        kind, value = next(iter(item["inner"].items()))
        path = parents + ([item["name"]] if item.get("name") else [])
        module_paths[str(item_id)] = "::".join(path)
        if kind == "module":
            for child in value["items"]:
                walk(child, path)
        elif kind in ("struct", "enum", "union"):
            for impl_id in value["impls"]:
                impl = index[str(impl_id)]
                reachable.add(str(impl_id))
                definition = impl["inner"]["impl"]
                if "automatically_derived" in impl["attrs"]:
                    if definition.get("trait"):
                        derived_traits[str(item_id)].append(definition["trait"]["path"])
                    continue
                for child in definition["items"]:
                    walk(child, path)
        elif kind == "trait":
            for child in value["items"]:
                walk(child, path)

    walk(doc["root"], [])
    formal = defaultdict(list)
    for item_id in sorted(reachable, key=int):
        item = index[item_id]
        span = item.get("span")
        if not span:
            continue
        path = Path(span["filename"])
        if not path.is_absolute():
            path = workspace / path
        if not path.is_relative_to(root):
            continue
        kind = next(iter(item["inner"]))
        if kind not in ("module", "struct", "enum", "trait", "function", "type_alias", "use"):
            continue
        relative = str(path.relative_to(root))
        formal[relative].append({"name": item.get("name"), "kind": kind,
            "path": module_paths.get(item_id), "line": span["begin"][0],
            "visibility": item["visibility"], "attrs": item["attrs"],
            "derived_traits": sorted(derived_traits[item_id])})

    corpus = json.loads((project / "evidence/country-core-deep/SOURCE_MAP.json").read_text())
    old_coverage = json.loads((workspace / "crates/country-core/country-source-coverage.json").read_text())
    imports = json.loads((workspace / "crates/country-core/render-source-map.json").read_text())
    mappings = defaultdict(set)
    for entry in imports["entries"]:
        source = entry["source"].split("country-core/src/", 1)[-1]
        destination = entry["destination"].split("country-core/src/", 1)[-1]
        if destination in formal:
            mappings[source].add(destination)
    overrides = {
        "components/mod.rs": ["components.rs"],
        "systems/camera_rig.rs": ["systems/camera_rig/mod.rs"],
        "systems/main_camera.rs": ["systems/camera_rig/main_camera.rs"],
        "render/nani.rs": ["render/nani/mod.rs"],
        "resources/walls.rs": ["resources/walls/mod.rs"],
        "resources/roofs.rs": ["resources/roofs/mod.rs"],
        "resources/walls/inner_wall_state.rs": ["resources/walls/inner_wall_state.rs"],
        "resources/walls/inner_walls.rs": ["resources/walls/inner_walls.rs"],
        "systems/wall/wall_state/publish_public_wall_state.rs": ["systems/wall/wall_state/publish_public_wall_state.rs"],
    }
    for source, targets in overrides.items():
        mappings[source].update(p for p in targets if p in formal)
    for source in corpus:
        if source in formal:
            mappings[source].add(source)

    contracts_path = root.parent / "recovery/contracts.json"
    contracts = json.loads(contracts_path.read_text()) if contracts_path.exists() else {}
    for source, record in contracts.items():
        mappings[source].update(p for p in record.get("formal_paths", []) if p in formal)

    db = sqlite3.connect(f"file:{project}/skeleton/task-index/index.sqlite3?mode=ro", uri=True)
    functions = {}
    by_source = defaultdict(list)
    for item_id, path, payload in db.execute("SELECT id,path,payload FROM functions WHERE version='B'"):
        record = json.loads(payload)
        functions[item_id] = record
        if path.startswith("crates/country-core/src/"):
            by_source[path.split("crates/country-core/src/", 1)[1]].append(record)

    retained_files = set(corpus)
    for row in old_coverage["all_source_file_inventory"]:
        retained_files.add(row["source"].split("country-core/src/", 1)[-1])
    original_files = {path.split("country-core/src/", 1)[1]: json.loads(payload)
        for path, payload in db.execute(
            "SELECT path,payload FROM files WHERE version='B' AND path LIKE 'crates/country-core/src/%'")}
    source_files = set(original_files)
    entries = []
    for source in sorted(source_files):
        chapter = project / "evidence/country-core-deep/chapters" / (source + ".md")
        targets = sorted(mappings[source] | ({source} if source in formal else set()))
        records = by_source[source]
        consumers = {}
        for record in records:
            for caller_id in record.get("distinct_callers", []):
                caller = functions.get(caller_id)
                if caller:
                    consumers[caller_id] = {"id": caller_id, "path": caller["path"],
                        "name": caller["name"], "kind": "indexed_direct_or_inline_caller"}
        chapter_functions = corpus.get(source, {}).get("functions", [])
        counts = Counter(item["kind"] for item in chapter_functions)
        entry = {
            "original_path": "crates/country-core/src/" + source,
            "original_identity": {k: original_files[source].get(k) for k in
                ["source", "metadata_path", "source_sha256", "source_fingerprint", "original_paths"]},
            "chapter": str(chapter.relative_to(project)) if chapter.is_file() else None,
            "chapter_sha256": digest(chapter) if chapter.is_file() else None,
            "expected_formal_path": "crates/country-core/src/" + source,
            "formal_candidates": [{"path": "crates/country-core/src/" + target,
                "sha256": digest(root / target),
                "line": min((i["line"] for i in formal[target]), default=1),
                "compiler_items_ref": "compiled_files[path=" + "crates/country-core/src/" + target + "]"}
                for target in targets],
            "status": "present_requires_semantic_review" if targets else "no_formal_mapping",
            "contract": contracts.get(source),
            "chapter_kind_counts": dict(counts),
            "original_functions": chapter_functions,
            "original_consumers": sorted(consumers.values(), key=lambda c: (c["path"], c["name"])),
            "consumer_boundary": "indexed navigation; not a dynamic or complete call graph",
        }
        if not consumers:
            entry["consumer_gap"] = "consumer_not_located_in_retained_index"
        entries.append(entry)

    # rustdoc exposes type/signature dependencies, not calls inside function bodies.
    signature_edges = set()
    item_files = {item_id: item.get("span", {}).get("filename")
                  for item_id, item in index.items() if item.get("span")}

    def referenced_ids(value):
        if isinstance(value, dict):
            resolved = value.get("resolved_path")
            if isinstance(resolved, dict) and resolved.get("id") is not None:
                yield str(resolved["id"])
            for child in value.values():
                yield from referenced_ids(child)
        elif isinstance(value, list):
            for child in value:
                yield from referenced_ids(child)

    for item_id in sorted(reachable, key=int):
        source = item_files.get(item_id)
        if source is None:
            continue
        for dependency in referenced_ids(index[item_id]["inner"]):
            target = item_files.get(dependency)
            if target and target != source:
                signature_edges.add((source, target))

    metadata = json.loads(subprocess.run(
        ["cargo", "metadata", "--no-deps", "--format-version", "1", "--locked", "--offline"],
        cwd=workspace, check=True, capture_output=True, text=True).stdout)
    package = next(p for p in metadata["packages"] if p["name"] == "country-core")
    original_lock = tomllib.loads((workspace / "provenance/original-Cargo.lock").read_text())
    current_lock = tomllib.loads((workspace / "Cargo.lock").read_text())
    dependencies = []
    for dependency in package["dependencies"]:
        candidates = [p for p in current_lock["package"] if p["name"] == dependency["name"]
            and (not dependency["req"].startswith("=") or
                 p["version"].split("+")[0] == dependency["req"][1:].split("+")[0])]
        identities = []
        for candidate in candidates:
            identity = {k: candidate.get(k) for k in ["name", "version", "source", "checksum"]}
            identity["matches_original_lock"] = any(
                all(p.get(k) == candidate.get(k) for k in ["name", "version", "source", "checksum"])
                for p in original_lock["package"])
            identities.append(identity)
        dependencies.append({"name": dependency["name"], "requirement": dependency["req"],
            "optional": dependency["optional"], "features": dependency["features"],
            "source": dependency["source"], "path": dependency.get("path"),
            "lock_candidates": identities,
            "classification": "workspace_recovered_dependency" if dependency.get("path") else "dependency_reuse"})

    fixes = json.loads((project / "evidence/country-core-deep/round2/FIX-LIST.json").read_text())
    assessments = []
    for number, fix in enumerate(fixes):
        key = fix["file"].replace("\\", "/").strip("（）() ")
        original_candidates = [e for e in entries if e["original_path"].endswith("/" + key)]
        assessments.append({"id": f"FIX-{number + 1:03d}", "original_review": fix,
            "original_candidates": [e["original_path"] for e in original_candidates],
            "formal_candidates": sorted({x["path"] for e in original_candidates for x in e["formal_candidates"]}),
            "assessment": "requires_current_semantic_review",
            "unresolved_path": not bool(original_candidates)})
        resolutions = [c["fix_records"][f"FIX-{number + 1:03d}"] for c in contracts.values()
                       if f"FIX-{number + 1:03d}" in c.get("fix_records", {})]
        if resolutions:
            assessments[-1]["assessment"] = resolutions
    report = {
        "schema_version": 1, "baseline": "B", "semantic_acceptance": False,
        "scope": "B PDB country-core file identities, retained chapters/FIX-LIST, and compiler-reachable formal code (kept separate)",
        "binary_sha256": "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03",
        "pdb_sha256": "2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911",
        "rustdoc_sha256": digest(docs_path), "rustdoc_format": doc["format_version"],
        "original_source_map_sha256": digest(project / "evidence/country-core-deep/SOURCE_MAP.json"),
        "cargo_lock_sha256": digest(workspace / "Cargo.lock"),
        "contracts_sha256": digest(contracts_path) if contracts_path.exists() else None,
        "manifest_sha256": digest(root.parent / "Cargo.toml"),
        "verification_input_hashes": {str(path.relative_to(workspace)): digest(path)
            for path in sorted((root.parent / "tests").rglob("*")) if path.is_file()},
        "historical_only_paths": sorted(retained_files - source_files),
        "historical_only_boundary": "Retained reconstruction/adapter paths, not B PDB source identities; do not include in original completion denominator.",
        "direct_dependencies": dependencies,
        "signature_dependency_edges": [list(e) for e in sorted(signature_edges)],
        "compiled_files": [{"path": "crates/country-core/src/" + path,
            "sha256": digest(root / path), "items": formal[path],
            "mapped_original_paths": [e["original_path"] for e in entries
                if any(t["path"] == "crates/country-core/src/" + path for t in e["formal_candidates"])]}
            for path in sorted(formal)],
        "summary": {"source_files": len(entries), "source_map_entries": len(corpus),
                    "chapter_files_matching_original_files": sum(bool(e["chapter"]) for e in entries),
                    "historical_only_paths": len(retained_files - source_files),
                    "formal_source_files": len(formal), "fix_records": len(assessments),
                    "file_status_counts": dict(Counter(e["status"] for e in entries))},
        "entries": entries, "fix_records": assessments,
        "limitations": ["SOURCE_MAP names/addresses include historical misattributions; retain rather than silently trust them.",
            "Derived serializers/defaults need type-contract review; kind labels do not establish bespoke business logic.",
            "Compiler reachability and retained caller edges are evidence inventory, not semantic completion.",
            "Known moved/projection business modules require explicit consumer mapping before marking missing functions."],
    }
    args.output.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n")
    if args.markdown:
        groups = defaultdict(list)
        for entry in entries:
            source = entry["original_path"].split("country-core/src/", 1)[1]
            groups[str(Path(source).parent)].append(entry)
        lines = ["# country-core 逐目录源码核对", "",
            "由 tools/audit_core_foundation.py 生成；B 为唯一目标。原目录以 B PDB 文件索引为准，编译导出取自 rustdoc。",
            "有正式映射不等于完整实现；未映射不等于算法不存在（可能仍在 recovered_building 等投影中）。",
            "完成验收必须另外核对状态所有者、消费者、失败分支、编辑/删除/历史传播和原版算法。", "",
            f"B 索引文件 {len(entries)}；其中有章节 {sum(bool(e['chapter']) for e in entries)}；编译可达文件 {len(formal)}；旧纠错条目 {len(assessments)}。不据此计算业务完成率。",
            f"旧库存另有 {len(retained_files - source_files)} 个路径未出现在 B PDB 文件表，单列 historical_only_paths，不混入原目录完成分母。", "",
            "| 原目录 | 原文件数 | 有正式源码映射 | 本轮逐文件合同 | 状态 |",
            "|---|---:|---:|---:|---|"]
        for directory, rows in sorted(groups.items()):
            mapped = sum(bool(row["formal_candidates"]) for row in rows)
            reviewed = sum(bool(row["contract"]) for row in rows)
            lines.append(f"| `{directory}/` | {len(rows)} | {mapped} | {reviewed} | "
                         + ("恢复/核验中" if reviewed else "待逐文件复核") + " |")
        lines += ["", "## 文件与证据", "",
                  "每条列出正式源码、章节及原调用者数量；具体调用者、符号和 FIX 行见同目录 inventory.json。", ""]
        for directory, rows in sorted(groups.items()):
            lines += [f"### {directory}/", "",
                      "| 原文件 | 当前源码 | 当前判定 | 原调用者 |", "|---|---|---|---:|"]
            for row in rows:
                source = row["original_path"].split("country-core/src/", 1)[1]
                name = Path(source).name
                evidence = project / row["chapter"] if row["chapter"] else None
                label = f"[{name}]({evidence})" if evidence else f"`{name}`（无章节）"
                locations = []
                for target in row["formal_candidates"]:
                    location = workspace / target["path"]
                    line = target["line"]
                    locations.append(f"[{Path(target['path']).name}]({location}:{line})")
                contract = row["contract"]
                state = contract["status_label"] if contract else (
                    "已有代码，语义待复核" if locations else "缺正式映射，待查现有投影")
                lines.append(f"| {label} | {'; '.join(locations) or '-'} | {state} | {len(row['original_consumers'])} |")
            lines.append("")
        lines += ["## 实际依赖", "", "| 包 | 锁定要求 | 处理 |", "|---|---|---|"]
        for dependency in dependencies:
            lines.append(f"| `{dependency['name']}` | `{dependency['requirement']}` | "
                + ("已有 workspace 实现" if dependency["path"] else "复用原依赖")
                + ("；可选" if dependency["optional"] else "") + " |")
        args.markdown.write_text("\n".join(lines) + "\n")
    print(json.dumps(report["summary"], ensure_ascii=False))


if __name__ == "__main__":
    main()
