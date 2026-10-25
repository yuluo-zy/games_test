#!/usr/bin/env python3
"""Batch decompile runner for rhapsody domains (RE-MCP).

Usage: python scripts/run-rhapsody-decomp.py <domain> <dbid> [--lock]

- Reads evidence/country-core-deep/rhapsody/domains/<domain>.json (symbols w/ VA)
- Builds a RE-MCP job (open/wait/decompile*/close, allow_error on decompiles)
- Invokes scripts/mcp-job.mjs, writes decomp-<va>.txt per function to
  evidence/country-core-deep/rhapsody/domains/<domain>/decomp/ + summary.json
"""
import json, os, subprocess, sys, time

ROOT = r"D:/game/reverse-engineering/tiny-glade"
BIN = os.path.join(ROOT, "workspace/input/tiny-glade.exe")
LOCK = os.path.join(ROOT, "evidence/country-core-deep/.mcp-batch.lock")
DOM = os.path.join(ROOT, "evidence/country-core-deep/rhapsody/domains")

def acquire_lock(no_lock, timeout=10800):
    if no_lock:
        return None
    start = time.time()
    while True:
        try:
            fd = os.open(LOCK, os.O_CREAT | os.O_EXCL | os.O_WRONLY)
            os.write(fd, f"{os.getpid()} {time.time()}\n".encode())
            os.close(fd)
            return fd
        except FileExistsError:
            if time.time() - start > timeout:
                print("LOCK TIMEOUT", file=sys.stderr)
                sys.exit(1)
            time.sleep(10)

def release_lock():
    try:
        os.unlink(LOCK)
    except OSError:
        pass

def main():
    domain, dbid = sys.argv[1], sys.argv[2]
    no_lock = "--lock" not in sys.argv
    only_summary = "--only-summary" in sys.argv
    mf = os.path.join(DOM, f"{domain}.json")
    rows = json.load(open(mf, encoding="utf-8"))
    # prefer compiled_procedure w/ VA; fall back to any symbol with VA (dedup)
    seen, steps = set(), []
    for r in sorted(rows, key=lambda x: x["name"]):
        va = r.get("va")
        if not va or va in seen:
            continue
        if r.get("form") == "inline_instance" and domain != "crates":
            # inline instances often fold onto a compiled body; still worth one
            # decompile attempt, but low priority.
            pass
        seen.add(va)
        steps.append({"tool": "decompile_function",
                      "args": {"database": dbid, "address": va},
                      "timeout_ms": 240000, "allow_error": True,
                      "_note": f"{r['name'][:110]}"})
    job = [
        {"tool": "open_database",
         "args": {"file_path": BIN, "run_auto_analysis": False, "database_id": dbid,
                  "keep_open": True}, "timeout_ms": 600000},
        {"tool": "wait_for_analysis", "args": {"database": dbid}, "timeout_ms": 600000},
    ] + steps + [
        {"tool": "close_database", "args": {"database": dbid, "save": False},
         "timeout_ms": 180000},
    ]
    outdir = os.path.join(DOM, domain)
    os.makedirs(os.path.join(outdir, "decomp"), exist_ok=True)
    jobpath = os.path.join(outdir, "batch-job.json")
    outpath = os.path.join(outdir, "batch-report.json")
    json.dump(job, open(jobpath, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print(f"[{domain}] {len(steps)} decompile steps -> {jobpath}")
    if only_summary:
        return
    acquire_lock(no_lock)
    try:
        r = subprocess.run(["node", os.path.join(ROOT, "scripts/mcp-job.mjs"),
                            "re_mcp_ghidra", jobpath, outpath], cwd=ROOT,
                           capture_output=True, text=True)
    finally:
        release_lock()
    print(r.stdout[-1500:])
    if r.returncode != 0:
        print("RUNNER FAILED:", r.stderr[-1500:])
        sys.exit(1)
    report = json.load(open(outpath, encoding="utf-8"))
    summary = []
    for res in report["results"]:
        if res["tool"] != "decompile_function":
            continue
        addr = res["args"]["address"]
        note = res.get("_note", "")
        content = res["result"].get("content")
        text, is_err = "", bool(res["result"].get("isError"))
        if content:
            text = content[0].get("text", "")
            try:
                j = json.loads(text)
                if "decompiled_code" in j:
                    text = j.get("decompiled_code", "")
                elif "disassembly" in j:
                    text = j.get("disassembly", "")
            except Exception:
                pass
        fn = os.path.join(outdir, "decomp", f"decomp-{addr}.txt")
        open(fn, "w", encoding="utf-8").write((note + "\n" if note else "") + text)
        summary.append({"address": addr, "bytes": len(text), "error": is_err, "note": note})
    json.dump(summary, open(os.path.join(outdir, "summary.json"), "w", encoding="utf-8"),
              ensure_ascii=False, indent=1)
    ok = sum(1 for s in summary if not s["error"])
    print(f"[{domain}] done: {ok}/{len(summary)} ok; decomp/ + summary.json written")
    for s in summary:
        if s["error"]:
            print("  ERR", s["address"], s["note"][:70])

if __name__ == "__main__":
    main()
