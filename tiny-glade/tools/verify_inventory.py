"""Validate the packaged inventory without RE tools or network."""
from pathlib import Path
import hashlib
import json
import re
import tomllib
import argparse

parser = argparse.ArgumentParser(description="Validate development workspace and original dependency identities")
parser.add_argument("--snapshot", action="store_true", help="Also compare files with the recorded import snapshot")
args = parser.parse_args()

RUST_RAW_STRING = re.compile(r'(?:br|r)(#*)"')
RUST_CHARACTER = re.compile(r"'(?:\\(?:[\\'\"nrt0]|x[0-9a-fA-F]{2}|u\{[0-9a-fA-F_]+\})|[^\\'\r\n])'")

def rust_code(source, mask_literals=False):
    """Mask comments (nested blocks included), preserving strings unless requested.

    Rust comments in recovery notes may mention old todo! macros; only executable
    tokens are inspected. Quotes/raw strings are consumed before looking for //.
    Character literals are distinguished from lifetime tokens such as 'static.
    """
    output = []
    index = 0
    def masked(text):
        return "".join("\n" if c == "\n" else " " for c in text)
    while index < len(source):
        start = index
        if source.startswith("//", index):
            end = source.find("\n", index)
            index = len(source) if end < 0 else end
            output.append(masked(source[start:index]))
        elif source.startswith("/*", index):
            depth = 1
            index += 2
            while index < len(source) and depth:
                if source.startswith("/*", index):
                    depth += 1
                    index += 2
                elif source.startswith("*/", index):
                    depth -= 1
                    index += 2
                else:
                    index += 1
            output.append(masked(source[start:index]))
        else:
            raw = None
            if index == 0 or not (source[index - 1].isalnum() or source[index - 1] == "_"):
                raw = RUST_RAW_STRING.match(source, index)
            character = RUST_CHARACTER.match(source, index) if source[index] == "'" else None
            if raw:
                closing = '"' + raw.group(1)
                end = source.find(closing, raw.end())
                index = len(source) if end < 0 else end + len(closing)
            elif source[index] == '"':
                index += 1
                while index < len(source):
                    if source[index] == "\\":
                        index += 2
                    elif source[index] == '"':
                        index += 1
                        break
                    else:
                        index += 1
            elif character:
                index = character.end()
            else:
                output.append(source[index])
                index += 1
                continue
            literal = source[start:index]
            output.append(masked(literal) if mask_literals else literal)
    return "".join(output)

root = Path(__file__).resolve().parents[1]
manifest = json.loads((root / "provenance/sources.json").read_text(encoding="utf-8"))
errors = []
entries = manifest["files"] + manifest["generated"]
if args.snapshot:
    for entry in entries:
        file = root / entry["path"]
        if not file.is_file():
            errors.append(f"missing: {entry['path']}")
        elif hashlib.sha256(file.read_bytes()).hexdigest() != entry["sha256"]:
            errors.append(f"hash mismatch: {entry['path']}")
for file in root.glob("crates/**/src/**/*.rs"):
    source = file.read_text(encoding="utf-8")
    if re.search(r"\b(?:todo|unimplemented)\s*!\s*\(", rust_code(source, mask_literals=True)):
        errors.append(f"unrecovered stub: {file.relative_to(root)}")
    if re.search(r'"[^"\n]*(?:\.\./)+[^"\n]*evidence/', rust_code(source)):
        errors.append(f"external evidence import: {file.relative_to(root)}")
workspace = tomllib.loads((root / "Cargo.toml").read_text(encoding="utf-8"))
for member in workspace["workspace"]["members"]:
    if not (root / member / "Cargo.toml").is_file():
        errors.append(f"workspace member missing: {member}")
        continue
    package = tomllib.loads((root / member / "Cargo.toml").read_text(encoding="utf-8"))["package"]
    readme = root / member / "README.md"
    if package.get("readme") != "README.md" or not readme.is_file() or readme.stat().st_size < 200:
        errors.append(f"package README missing or not declared: {member}")
shaders = json.loads((root / "shaders/manifest.json").read_text(encoding="utf-8"))
shader_exceptions = json.loads((root / "provenance/shader-exceptions.json").read_text(encoding="utf-8"))["exceptions"]
allowed_shader_limits = {
    (e["file"], e["bin"], e["entry"], e["stage"], e["gate"], e["execution"]): e
    for e in shader_exceptions
}
used_shader_limits = set()
for file in shaders["files"]:
    if not file["modules"]:
        if file["file"] not in shaders["include_files"]:
            errors.append(f"unclassified shader support: {file['file']}")
    for module in file["modules"]:
        if module["gate"] and module["exec"] in {"SAME", "空壳"}:
            continue
        key = (file["file"], module["bin"], module["entry"], module["stage"], module["gate"], module["exec"])
        if key not in allowed_shader_limits:
            errors.append(f"shader recovery gate without scoped exception: {file['file']}:{module['entry']}")
        else:
            used_shader_limits.add(key)
    if not (root / "shaders" / file["file"]).is_file():
        errors.append(f"shader missing: {file['file']}")
for key, exception in allowed_shader_limits.items():
    if key not in used_shader_limits:
        errors.append(f"stale shader exception: {exception['file']}:{exception['entry']}")
    if not (root / "provenance" / exception["evidence"]).is_file():
        errors.append(f"shader exception evidence missing: {exception['file']}")
original = tomllib.loads((root / "provenance/original-Cargo.lock").read_text(encoding="utf-8"))
current = tomllib.loads((root / "Cargo.lock").read_text(encoding="utf-8"))
identity = lambda p: (p["name"], p["version"], p.get("source"), p.get("checksum"))
original_packages = {identity(p) for p in original["package"] if p.get("source")}
external = [p for p in current["package"] if p.get("source")]
dependency_exceptions = json.loads((root / "provenance/dependency-exceptions.json").read_text(encoding="utf-8"))["exceptions"]
allowed_adapters = {}
for exception in dependency_exceptions:
    candidate_manifest = tomllib.loads((root / exception["manifest"]).read_text(encoding="utf-8"))
    dependency = candidate_manifest.get("dependencies", {}).get(exception["dependency"], {})
    features = candidate_manifest.get("features", {})
    enabled = set()
    pending = list(features.get("default", []))
    while pending:
        feature = pending.pop()
        if feature in enabled:
            continue
        enabled.add(feature)
        pending.extend(features.get(feature, []))
    dep_name = exception["dependency"]
    expected = exception["package"]
    if (not isinstance(dependency, dict) or dependency.get("optional") is not True
        or dependency.get("version") != f"={expected['version']}"
        or features.get(exception["feature"]) != [f"dep:{dep_name}"]
        or exception["feature"] in enabled or f"dep:{dep_name}" in enabled or dep_name in enabled
        or any(feature.startswith(f"{dep_name}/") for feature in enabled)):
        errors.append(f"candidate dependency must remain explicit/optional/default-disabled: {dep_name}")
    elif not (root / "provenance" / exception["evidence"]).is_file():
        errors.append(f"candidate dependency evidence missing: {dep_name}")
    else:
        allowed_adapters[identity(expected)] = exception
used_adapters = set()
for package in external:
    if identity(package) not in original_packages:
        if identity(package) in allowed_adapters:
            used_adapters.add(identity(package))
        else:
            errors.append(f"dependency identity not in B lock: {package['name']} {package['version']} {package.get('source')}")
for adapter in allowed_adapters:
    if adapter not in used_adapters:
        errors.append(f"stale candidate dependency exception: {adapter[0]} {adapter[1]}")
result = {"passed": not errors, "snapshot_checked": args.snapshot, "files_checked": len(entries) if args.snapshot else 0, "workspace_packages": len(workspace["workspace"]["members"]), "external_dependencies": len(external), "original_identity_dependencies": len(external) - len(used_adapters), "candidate_dependency_exceptions": len(used_adapters), "shader_entry_files": len(shaders["files"]), "shader_modules": sum(len(f["modules"]) for f in shaders["files"]), "scoped_shader_exceptions": len(used_shader_limits), "errors": errors}
print(json.dumps(result, ensure_ascii=False, indent=2))
raise SystemExit(bool(errors))
