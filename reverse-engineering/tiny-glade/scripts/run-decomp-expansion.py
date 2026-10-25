"""Batch decompile runner for the SOURCE_MAP_EXPANSION modules
(components / startup / utils / input / root config).

Usage:
  python scripts/run-decomp-expansion.py <batch> <dbid> [--lock] [--only-summary]

- <batch> is a key in BATCHES below (file-substring filter over
  evidence/country-core-deep/SOURCE_MAP_EXPANSION.json).
- Decompiles every business function VA to
  evidence/country-core-deep/expansion-<batch>/decomp/decomp-<va>.txt

Same lock protocol as run-decomp-batch.py (machine RAM constraint).
"""
import json, os, subprocess, sys, time

ROOT = r"D:/game/reverse-engineering/tiny-glade"
IDX = os.path.join(ROOT, "evidence/country-core-deep/SOURCE_MAP_EXPANSION.json")
BIN = os.path.join(ROOT, "workspace/input/tiny-glade.exe")
LOCK = os.path.join(ROOT, "evidence/country-core-deep/.mcp-batch.lock")

BATCHES = {
    "b1": ["components/", "utils/"],
    "b2": ["startup/", "input/", "game_config.rs", "game_key_bindings.rs", "game_profile.rs"],
}


def acquire_lock(timeout=7200):
    start = time.time()
    while True:
        try:
            fd = os.open(LOCK, os.O_CREAT | os.O_EXCL | os.O_WRONLY)
            os.write(fd, f"{os.getpid()} {time.time()}\n".encode())
            os.close(fd)
            return
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
    batch, dbid = sys.argv[1], sys.argv[2]
    only_summary = "--only-summary" in sys.argv
    no_lock = "--lock" not in sys.argv

    pats = BATCHES[batch]
    idx = json.load(open(IDX, encoding="utf-8"))
    steps, seen = [], set()
    for f in sorted(idx):
        if not any(p in f for p in pats):
            continue
        for fn in idx[f]["functions"]:
            if fn["kind"] != "business":
                continue
            for va in fn["va"]:
                if va in seen:
                    continue
                seen.add(va)
                steps.append({"tool": "decompile_function",
                              "args": {"database": dbid, "address": va},
                              "timeout_ms": 240000, "allow_error": True,
                              "_note": f"{f} :: {fn['name'].split('country_core::')[-1][:90]}"})

    job = [
        {"tool": "open_database",
         "args": {"file_path": BIN, "run_auto_analysis": False, "database_id": dbid,
                  "keep_open": True}, "timeout_ms": 600000},
        {"tool": "wait_for_analysis", "args": {"database": dbid}, "timeout_ms": 600000},
    ] + steps + [
        {"tool": "close_database", "args": {"database": dbid, "save": False},
         "timeout_ms": 180000},
    ]
    outdir = os.path.join(ROOT, "evidence/country-core-deep", f"expansion-{batch}")
    os.makedirs(os.path.join(outdir, "decomp"), exist_ok=True)
    jobpath = os.path.join(outdir, "batch-job.json")
    outpath = os.path.join(outdir, "batch-report.json")
    json.dump(job, open(jobpath, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print(f"[{batch}] {len(steps)} decompile steps -> {jobpath}")
    if only_summary:
        return

    acquire_lock() if not no_lock else None
    try:
        r = subprocess.run(["node", os.path.join(ROOT, "scripts/mcp-job.mjs"),
                            "re_mcp_ghidra", jobpath, outpath], cwd=ROOT,
                           capture_output=True, text=True)
    finally:
        release_lock()
    print(r.stdout[-1500:])
    if r.returncode != 0:
        print("RUNNER FAILED:", r.stderr[-2000:])
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
                text = j.get("decompiled_code") or j.get("disassembly") or text
            except Exception:
                pass
        fn = os.path.join(outdir, "decomp", f"decomp-{addr}.txt")
        open(fn, "w", encoding="utf-8").write((note + "\n" if note else "") + text)
        summary.append({"address": addr, "bytes": len(text), "error": is_err, "note": note})
    json.dump(summary, open(os.path.join(outdir, "summary.json"), "w", encoding="utf-8"),
              ensure_ascii=False, indent=1)
    ok = sum(1 for s in summary if not s["error"])
    print(f"[{batch}] done: {ok}/{len(summary)} ok; decomp/ + summary.json written")
    for s in summary:
        if s["error"]:
            print("  ERR", s["address"], s["note"][:70])


if __name__ == "__main__":
    main()
