#!/usr/bin/env python3
"""Read-only PE x86/x64 source/inline + bounded ASM packs. Python stdlib only."""
import argparse
import hashlib
import json
import math
import os
from pathlib import Path
import re
import shutil
import struct
import subprocess
import sys
import tempfile
import time
import uuid

SCHEMA = 1
MAX_RANGE = 65536
MAX_REQUESTS = 128
MAX_POINTS = 32768

class PackError(Exception):
    pass

def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode()

def digest(path):
    with Path(path).open("rb") as f:
        return hashlib.file_digest(f, "sha256").hexdigest()

def number(value, label):
    if isinstance(value, bool):
        raise PackError(f"{label}: boolean is not an address/count")
    try:
        result = int(value, 0) if isinstance(value, str) else value
    except (ValueError, TypeError):
        raise PackError(f"{label}: expected integer or 0x-prefixed string")
    if not isinstance(result, int) or result < 0:
        raise PackError(f"{label}: expected nonnegative integer")
    return result

def atomic_json(path, value):
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    fd, temp = tempfile.mkstemp(prefix=".pack-", dir=path.parent)
    try:
        with os.fdopen(fd, "wb") as f:
            f.write(json.dumps(value, ensure_ascii=False, indent=2).encode() + b"\n")
            f.flush()
            os.fsync(f.fileno())
        os.replace(temp, path)
    finally:
        if os.path.exists(temp):
            os.unlink(temp)

def bound_artifact(value, label, root):
    if not isinstance(value, dict):
        raise PackError(f"{label}: missing object")
    path = Path(value.get("path", ""))
    if not path.is_absolute():
        path = root / path
    path = path.resolve()
    if not path.is_file():
        raise PackError(f"{label}: file missing: {path}")
    wanted = value.get("sha256", "")
    if not re.fullmatch(r"[a-fA-F0-9]{64}", wanted):
        raise PackError(f"{label}: full SHA-256 required")
    actual = digest(path)
    if actual != wanted.lower():
        raise PackError(f"{label}: SHA-256 mismatch: observed {actual}")
    return dict(path=str(path), sha256=actual, bytes=path.stat().st_size)

class PE:
    def __init__(self, path):
        self.path = Path(path)
        with self.path.open("rb") as f:
            dos = f.read(64)
            if len(dos) < 64 or dos[:2] != b"MZ":
                raise PackError("Unsupported format: this version requires PE x86/x64")
            f.seek(struct.unpack_from("<I", dos, 60)[0])
            header = f.read(24)
            if len(header) != 24 or header[:4] != b"PE\0\0":
                raise PackError("Invalid PE header")
            machine, count = struct.unpack_from("<HH", header, 4)
            if machine not in (0x14c, 0x8664):
                raise PackError("Unsupported PE architecture: x86/x64 only")
            optional = f.read(struct.unpack_from("<H", header, 20)[0])
            if len(optional) < 96:
                raise PackError("Truncated PE optional header")
            magic = struct.unpack_from("<H", optional)[0]
            if magic == 0x20b:
                base, directory = struct.unpack_from("<Q", optional, 24)[0], 112
            elif magic == 0x10b:
                base, directory = struct.unpack_from("<I", optional, 28)[0], 96
            else:
                raise PackError("Unsupported PE optional header")
            self.image_base = base
            self.sections = []
            for _ in range(count):
                section = f.read(40)
                if len(section) != 40:
                    raise PackError("Truncated PE section table")
                virtual_size, rva, raw_size, offset = struct.unpack_from("<IIII", section, 8)
                self.sections.append(dict(name=section[:8].rstrip(b"\0").decode("ascii", "replace"),
                    rva=rva, virtual_size=virtual_size, raw_size=raw_size, offset=offset,
                    executable=bool(struct.unpack_from("<I", section, 36)[0] & 0x20000000)))
            self.rsds = []
            if len(optional) >= directory + 7 * 8:
                debug_rva, size = struct.unpack_from("<II", optional, directory + 6 * 8)
                if size:
                    debug = self.read_rva(debug_rva, size)
                    if len(debug) % 28:
                        raise PackError("Invalid PE debug directory size")
                    for i in range(0, len(debug), 28):
                        _, _, _, _, kind, length, _, pointer = struct.unpack_from("<IIHHIIII", debug, i)
                        if kind != 2:
                            continue
                        f.seek(pointer)
                        data = f.read(length)
                        if len(data) >= 24 and data[:4] == b"RSDS":
                            self.rsds.append(dict(guid=str(uuid.UUID(bytes_le=data[4:20])).upper(),
                                age=struct.unpack_from("<I", data, 20)[0],
                                pdb_name=data[24:].split(b"\0")[0].decode("utf-8", "replace")))

    def section_for(self, rva, length, executable=False):
        for s in self.sections:
            if s["rva"] <= rva and rva + length <= s["rva"] + s["raw_size"]:
                if executable and not s["executable"]:
                    raise PackError("Requested range is not an executable file-backed section")
                if s["offset"] + rva - s["rva"] + length > self.path.stat().st_size:
                    raise PackError("Requested PE range exceeds actual file")
                return s
        raise PackError("Requested range is not within one file-backed PE section")

    def read_rva(self, rva, length):
        s = self.section_for(rva, length)
        with self.path.open("rb") as f:
            f.seek(s["offset"] + rva - s["rva"])
            data = f.read(length)
        if len(data) != length:
            raise PackError("Short PE range read")
        return data

def pdb_identity(path):
    """Read MSF7 Info stream identity only; no CodeView types/locals parser."""
    with Path(path).open("rb") as f:
        header = f.read(56)
        if header[:32] != b"Microsoft C/C++ MSF 7.00\r\n\x1aDS\0\0\0":
            raise PackError("Unsupported PDB container; MSF7 required")
        block, _, blocks, directory_bytes, _, block_map = struct.unpack_from("<6I", header, 32)
        if block not in (512, 1024, 2048, 4096, 8192) or blocks * block > Path(path).stat().st_size:
            raise PackError("Invalid PDB MSF block header")
        count = math.ceil(directory_bytes / block)
        if count * 4 > block:
            raise PackError("Unsupported multi-block PDB directory map")
        f.seek(block_map * block)
        offsets = struct.unpack("<" + "I" * count, f.read(count * 4))
        chunks = []
        for offset in offsets:
            f.seek(offset * block)
            chunks.append(f.read(block))
        directory = b"".join(chunks)[:directory_bytes]
        n = struct.unpack_from("<I", directory)[0]
        if n < 2:
            raise PackError("PDB Info stream absent")
        sizes = struct.unpack_from("<" + "I" * n, directory, 4)
        cursor = 4 + n * 4
        skip = 0 if sizes[0] == 0xffffffff else math.ceil(sizes[0] / block)
        cursor += skip * 4
        if sizes[1] == 0xffffffff or sizes[1] < 28:
            raise PackError("PDB Info stream truncated")
        k = math.ceil(sizes[1] / block)
        stream_blocks = struct.unpack_from("<" + "I" * k, directory, cursor)
        pieces = []
        for b in stream_blocks:
            f.seek(b * block)
            pieces.append(f.read(block))
        info = b"".join(pieces)
        return dict(guid=str(uuid.UUID(bytes_le=info[12:28])).upper(), age=struct.unpack_from("<I", info, 8)[0])

class Runner:
    def __init__(self, timeout):
        self.timeout, self.calls = timeout, []

    def run(self, argv, phase, stdin=None):
        started = time.perf_counter()
        row = dict(phase=phase, argv=list(map(str, argv)))
        try:
            result = subprocess.run(argv, input=stdin, capture_output=True, text=True,
                encoding="utf-8", errors="replace", timeout=self.timeout, shell=False)
            row.update(returncode=result.returncode, stdout_bytes=len(result.stdout.encode()),
                stderr_bytes=len(result.stderr.encode()), elapsed_seconds=time.perf_counter() - started,
                stderr=result.stderr)
            self.calls.append(row)
            if result.returncode or result.stderr.strip():
                raise PackError(f"{phase}: tool failed or emitted diagnostics: {result.stderr.strip()}")
            return result.stdout
        except subprocess.TimeoutExpired:
            row.update(returncode=None, timeout=True, elapsed_seconds=time.perf_counter() - started)
            self.calls.append(row)
            raise PackError(f"{phase}: tool timeout")
        except OSError as error:
            row.update(returncode=None, error=str(error), elapsed_seconds=time.perf_counter() - started)
            self.calls.append(row)
            raise PackError(f"{phase}: tool unavailable: {error}")

def tool_identity(value, name, runner):
    path = shutil.which(value) if value else shutil.which(name)
    if not path:
        raise PackError(f"Tool unavailable: {value or name}")
    path = str(Path(path).resolve())
    version = runner.run([path, "--version"], "tool_version").strip()
    if not version:
        raise PackError("Empty tool version")
    return dict(path=path, version=version, sha256=digest(path))

def parse_assembly(text, pe, rva, length):
    start, stop = pe.image_base + rva, pe.image_base + rva + length
    records = []
    for line in text.splitlines():
        match = re.match(r"\s*([0-9a-fA-F]+):\s*((?:[0-9a-fA-F]{2}\s+)+)(.*)$", line)
        if not match:
            continue
        address = int(match[1], 16)
        raw = bytes.fromhex(match[2])
        asm = match[3].strip()
        if not start <= address < stop:
            raise PackError("objdump emitted an instruction start outside requested bounds")
        if not raw or not asm or "<unknown>" in asm:
            raise PackError("objdump emitted an undecoded instruction")
        if raw != pe.read_rva(address - pe.image_base, len(raw)):
            raise PackError("Disassembly bytes do not match original artifact")
        if address != (start if not records else int(records[-1]["va"], 16) + records[-1]["size"]):
            raise PackError("Non-contiguous disassembly; do not invent missing instructions")
        records.append(dict(va=hex(address), rva=hex(address - pe.image_base),
            bytes=raw.hex(), size=len(raw), assembly=asm, crosses_requested_stop=address + len(raw) > stop))
    if not records or int(records[-1]["va"], 16) + records[-1]["size"] < stop:
        raise PackError("Empty or incomplete disassembly")
    overhang = int(records[-1]["va"], 16) + records[-1]["size"] - stop
    if overhang > 14:
        raise PackError("Unexpected instruction overhang")
    return records, overhang

def parse_symbols(text, expected):
    try:
        parsed = json.loads(text)
        records = parsed if isinstance(parsed, list) else [parsed]
    except json.JSONDecodeError:
        try:
            records = [json.loads(line) for line in text.splitlines() if line.strip()]
        except json.JSONDecodeError:
            raise PackError("Invalid llvm-symbolizer JSON")
    result = {}
    for row in records:
        if not isinstance(row, dict) or "Address" not in row or "Symbol" not in row:
            raise PackError("Unexpected llvm-symbolizer record schema")
        address = number(row["Address"], "symbol address")
        if address not in expected or address in result:
            raise PackError("Symbolizer address mismatch or duplicate")
        frames = row["Symbol"]
        if not isinstance(frames, list) or not all(isinstance(f, dict) for f in frames):
            raise PackError("Invalid inline frame schema")
        result[address] = row
    if set(result) != set(expected):
        raise PackError("Symbolizer did not return every requested address")
    return result

def collect(manifest_path, outdir, cache, timeout=30):
    started = time.perf_counter()
    runner = Runner(timeout)
    report = dict(schema_version=SCHEMA, status="error", target_executed=False,
        manifest_path=str(Path(manifest_path).resolve()), requests=[], stats={}, tool_invocations=runner.calls)
    outdir, cache = Path(outdir), Path(cache)
    try:
        manifest = json.loads(Path(manifest_path).read_text())
        if not isinstance(manifest, dict):
            raise PackError("Manifest must be a JSON object")
        report["stats"]["requests"] = len(manifest.get("requests", [])) if isinstance(manifest.get("requests"), list) else 0
        if manifest.get("schema_version") != SCHEMA:
            raise PackError("Unsupported manifest schema_version")
        root = Path(manifest_path).resolve().parent
        artifact = bound_artifact(manifest.get("artifact"), "artifact", root)
        for name in ("id", "version"):
            if not isinstance(manifest["artifact"].get(name), str) or not manifest["artifact"][name]:
                raise PackError(f"artifact.{name}: nonempty string required")
            artifact[name] = manifest["artifact"][name]
        pe = PE(artifact["path"])
        artifact.update(format="PE", image_base=hex(pe.image_base), rsds=pe.rsds)
        report["artifact"] = artifact
        if manifest["artifact"].get("debug") is not None:
            debug = bound_artifact(manifest["artifact"]["debug"], "debug", root)
            debug.update(pdb_identity(debug["path"]))
            if not any(r["guid"] == debug["guid"] and r["age"] == debug["age"] for r in pe.rsds):
                raise PackError("PDB GUID/Age does not match PE CodeView")
            names = {Path(r["pdb_name"].replace("\\", "/")).name for r in pe.rsds}
            if Path(debug["path"]).name not in names:
                raise PackError("PDB basename differs from PE CodeView; explicit resolver unsupported")
            if Path(debug["path"]).parent != Path(artifact["path"]).parent:
                raise PackError("Debug resolver is not identity-bound: keep the bound PDB beside this PE")
            for r in pe.rsds:
                candidate = Path(r["pdb_name"].replace("\\", "/"))
                if candidate.is_absolute() and candidate.exists() and candidate.resolve() != Path(debug["path"]):
                    raise PackError("PE embeds an existing external PDB path; resolver choice is ambiguous")
            artifact["debug"] = debug
        elif pe.rsds:
            raise PackError("PE has CodeView PDB; manifest must bind its debug path and full SHA-256")
        requests = manifest.get("requests")
        if not isinstance(requests, list) or not 1 <= len(requests) <= MAX_REQUESTS:
            raise PackError(f"requests: expected 1..{MAX_REQUESTS} items")
        normalized, ids = [], set()
        for request in requests:
            if not isinstance(request, dict):
                raise PackError("Each request must be an object")
            rid = request.get("id")
            if not isinstance(rid, str) or not re.fullmatch(r"[A-Za-z0-9_-]{1,80}", rid) or rid in ids:
                raise PackError("Request id must be unique and path-safe")
            ids.add(rid)
            rva, length = number(request.get("rva"), "rva"), number(request.get("length"), "length")
            if not 1 <= length <= MAX_RANGE:
                raise PackError(f"length: expected 1..{MAX_RANGE}")
            pe.section_for(rva, length, executable=True)
            offsets = request.get("symbolize_offsets")
            if offsets is not None:
                if not isinstance(offsets, list) or not offsets:
                    raise PackError("symbolize_offsets must be a nonempty list")
                offsets = sorted(set(number(v, "symbolize_offset") for v in offsets))
                if offsets[-1] >= length:
                    raise PackError("Symbol sample lies outside range")
            allow_missing = request.get("allow_missing_symbols", False)
            if not isinstance(allow_missing, bool):
                raise PackError("allow_missing_symbols must be boolean")
            normalized.append(dict(id=rid, rva=rva, length=length, symbolize_offsets=offsets,
                allow_missing_symbols=allow_missing))
        tools = manifest.get("tools", {})
        if not isinstance(tools, dict):
            raise PackError("tools must be an object")
        identities = {name: tool_identity(tools.get(name), "llvm-" + name, runner)
            for name in ("symbolizer", "objdump")}
        report["tools"] = identities
        pending, prepared = {}, {}
        all_keys = set()
        hits = 0
        for request in normalized:
            key_request = {k: v for k, v in request.items() if k != "id"}
            binding = dict(schema_version=SCHEMA, artifact=artifact, tools=identities, request=key_request,
                collector_sha256=digest(__file__))
            key = hashlib.sha256(canonical(binding)).hexdigest()
            all_keys.add(key)
            slot = cache / key[:2] / (key + ".json")
            if slot.exists():
                envelope = json.loads(slot.read_text())
                if not isinstance(envelope, dict) or not isinstance(envelope.get("payload"), dict):
                    raise PackError("Invalid cache envelope")
                payload = envelope.get("payload")
                if envelope.get("binding") != binding or envelope.get("payload_sha256") != hashlib.sha256(canonical(payload)).hexdigest():
                    raise PackError("Cache integrity/binding failure; remove this entry explicitly before retry")
                prepared[request["id"]] = (key, payload, True)
                hits += 1
            elif key in pending:
                pending[key]["requests"].append(request)
            else:
                pending[key] = dict(binding=binding, slot=slot, requests=[request])
        points = set()
        for key, item in pending.items():
            request = item["requests"][0]
            start = pe.image_base + request["rva"]
            argv = [identities["objdump"]["path"], "--disassemble", "--disassemble-zeroes",
                "--disassembler-color=off", f"--start-address={hex(start)}",
                f"--stop-address={hex(start + request['length'])}", artifact["path"]]
            text = runner.run(argv, "disassemble")
            instructions, overhang = parse_assembly(text, pe, request["rva"], request["length"])
            offsets = request["symbolize_offsets"]
            sample = [request["rva"] + v for v in offsets] if offsets is not None else [int(i["rva"], 16) for i in instructions]
            item.update(assembly=text, instructions=instructions, overhang=overhang, sample=sample)
            points.update(sample)
        if len(points) > MAX_POINTS:
            raise PackError("Too many symbolization points; use explicit bounded offsets or split the pack")
        symbols = {}
        if points:
            argv = [identities["symbolizer"]["path"], f"--obj={artifact['path']}", "--relative-address",
                "--inlining", "--output-style=JSON", "--no-debuginfod", "--disable-gsym"]
            if "debug" in artifact:
                argv.append(f"--fallback-debug-path={Path(artifact['debug']['path']).parent}")
            symbols = parse_symbols(runner.run(argv, "symbolize", "".join(hex(v) + "\n" for v in sorted(points))), points)
        # Detect concurrent changes before promoting new evidence/cache results.
        for identity in [artifact] + ([artifact["debug"]] if "debug" in artifact else []) + list(identities.values()):
            if digest(identity["path"]) != identity["sha256"]:
                raise PackError("Artifact/debug/tool changed while collecting; results not promoted")
        failures = 0
        for key, item in pending.items():
            request = item["requests"][0]
            rows = [symbols[v] for v in item["sample"]]
            missing = [row["Address"] for row in rows if not any(f.get("FileName") and int(f.get("Line", 0)) > 0
                and f.get("FunctionName") not in (None, "", "??") for f in row["Symbol"])]
            ok = not missing or request["allow_missing_symbols"]
            payload = dict(status="ok" if ok else "missing_symbols", range=dict(rva=hex(request["rva"]),
                length=request["length"], start_va=hex(pe.image_base + request["rva"]),
                stop_va_exclusive=hex(pe.image_base + request["rva"] + request["length"]),
                instruction_overhang_bytes=item["overhang"], function_boundary_verified=False),
                instructions=item["instructions"], symbolization=rows, missing_symbol_addresses=missing,
                raw_assembly=item["assembly"], original_target_executed=False)
            atomic_json(item["slot"], dict(binding=item["binding"], payload=payload,
                payload_sha256=hashlib.sha256(canonical(payload)).hexdigest()))
            for n, req in enumerate(item["requests"]):
                prepared[req["id"]] = (key, payload, n > 0)
                hits += n > 0
        outdir.mkdir(parents=True, exist_ok=True)
        emitted = 0
        for request in normalized:
            key, payload, hit = prepared[request["id"]]
            path = outdir / (request["id"] + ".json")
            atomic_json(path, dict(schema_version=SCHEMA, artifact=artifact, request_id=request["id"], cache_key=key, **payload))
            asm_path = outdir / (request["id"] + ".asm.txt")
            asm_path.write_text(payload["raw_assembly"])
            emitted += path.stat().st_size + asm_path.stat().st_size
            report["requests"].append(dict(id=request["id"], status=payload["status"], cache_hit=hit,
                cache_key=key, evidence_path=str(path.resolve()), assembly_path=str(asm_path.resolve()),
                sampled_addresses=len(payload["symbolization"]), instructions=len(payload["instructions"])))
            failures += payload["status"] != "ok"
        report["status"] = "ok" if failures == 0 else "partial"
        report["stats"].update(requests=len(normalized), unique_requests=len(all_keys),
            cache_hits=hits, cache_misses=len(normalized) - hits, symbolization_points=len(points),
            evidence_output_bytes=emitted)
    except (PackError, ValueError, KeyError, TypeError, OSError, struct.error) as error:
        report["error"] = str(error)
    report["stats"].update(tool_processes=len(runner.calls),
        version_processes=sum(c["phase"] == "tool_version" for c in runner.calls),
        analysis_processes=sum(c["phase"] != "tool_version" for c in runner.calls),
        tool_stdout_bytes=sum(c.get("stdout_bytes", 0) for c in runner.calls),
        tool_stderr_bytes=sum(c.get("stderr_bytes", 0) for c in runner.calls),
        elapsed_seconds=time.perf_counter() - started)
    atomic_json(outdir / "report.json", report)
    return report

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("manifest", type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--cache", required=True, type=Path)
    parser.add_argument("--timeout", type=float, default=30)
    args = parser.parse_args()
    if not 0 < args.timeout <= 300:
        parser.error("timeout must be >0 and <=300 seconds")
    report = collect(args.manifest, args.output, args.cache, args.timeout)
    print(json.dumps(dict(status=report["status"], report_path=str((args.output / "report.json").resolve()),
        stats=report["stats"], error=report.get("error")), ensure_ascii=False))
    return 0 if report["status"] == "ok" else 2

if __name__ == "__main__":
    sys.exit(main())
