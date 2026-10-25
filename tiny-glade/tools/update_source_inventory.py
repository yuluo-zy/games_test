"""Combine reviewed source maps; preserve the first import as historical evidence."""
from pathlib import Path
import hashlib
import json
import subprocess

root = Path(__file__).resolve().parents[1]
repo = root.parent
read_json = lambda p: json.loads(p.read_text(encoding="utf-8"))
digest = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
initial = read_json(root / "provenance/sources-initial-import.json")
selected = {}
audit_paths = [
    "provenance/foundation-audit.json",
    "crates/rhapsody/migration-sources.json",
    "provenance/building-audit.json",
    "crates/country-core/render-source-map.json",
    "provenance/small-system-audit.json",
]


def accept(entry, audit):
    path = entry.get("path", entry.get("destination"))
    source = entry.get("source_path", entry.get("source"))
    if not path or not source or not (root / path).is_file():
        return
    item = dict(entry)
    item["path"] = path.replace("\\", "/")
    item["source_path"] = source.replace("\\", "/")
    item["audit"] = audit
    item["source_commit"] = entry.get("source_commit", entry.get("last_commit", entry.get("commit")))
    selected[item["path"]] = item


for entry in initial["files"]:
    accept(entry, "provenance/sources-initial-import.json")
for name in audit_paths:
    path = root / name
    if not path.is_file():
        continue
    document = read_json(path)
    for entry in document if isinstance(document, list) else document.get("files", document.get("entries", [])):
        accept(entry, name)

# Query tracked historical files once, rather than launching Git per source.
history = subprocess.check_output(
    ["git", "log", "--format=COMMIT:%H", "--name-only", "--", "reverse-engineering/tiny-glade/reconstruction"],
    cwd=repo, text=True, encoding="utf-8",
)
latest = {}
commit = None
for line in history.splitlines():
    if line.startswith("COMMIT:"):
        commit = line[7:]
    elif line.strip():
        latest.setdefault(line.strip(), commit)

for key, item in list(selected.items()):
    if not (root / item["path"]).is_file():
        del selected[key]
        continue
    item["sha256"] = digest(root / item["path"])
    source = repo / item["source_path"]
    if source.is_file():
        item.setdefault("source_sha256", digest(source))
    if not item.get("source_commit"):
        item["source_commit"] = latest.get(item["source_path"])
    item.setdefault("kind", item.get("classification", item.get("status", "recovered_source_or_explicit_input_contract")))

generated = []
for folder in [root / "crates", root / "shaders", root / "tools", root / ".cargo"]:
    for path in folder.rglob("*"):
        if not path.is_file() or "target" in path.parts or "__pycache__" in path.parts:
            continue
        relative = path.relative_to(root).as_posix()
        if relative in selected or path.suffix in {".log", ".pdb", ".exe"}:
            continue
        # Source maps are reviewed separately and would create hash cycles.
        if path.name in {"migration-sources.json", "render-source-map.json", "lint-preservation.json"}:
            continue
        generated.append({"path": relative, "sha256": digest(path), "kind": "engineering_wiring_documentation_or_local_fixture", "note": "No new original algorithm recovery claimed; see package README and audit maps."})
for name in ["Cargo.toml", "Cargo.lock", "README.md", "rustfmt.toml"]:
    path = root / name
    if path.is_file():
        generated.append({"path": name, "sha256": digest(path), "kind": "development_workspace"})

document = {
    "schema_version": 2,
    "baseline": "B",
    "date": "2026-10-14",
    "policy": "Select recovered function bodies individually, preserving static/native/runtime scope and explicit candidate features; do not reject a completed body merely because its enclosing original system is incomplete.",
    "historical_initial_import": "provenance/sources-initial-import.json",
    "audits": audit_paths,
    "files": sorted(selected.values(), key=lambda x: x["path"]),
    "generated": sorted(generated, key=lambda x: x["path"]),
    "exclusions": "provenance/AUDIT.md and provenance/source-coverage.json",
    "dependency_baseline_sha256": digest(root / "provenance/original-Cargo.lock"),
}
(root / "provenance/sources.json").write_text(json.dumps(document, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
print(json.dumps({"source_mappings": len(selected), "engineering_files": len(generated)}, ensure_ascii=False))
