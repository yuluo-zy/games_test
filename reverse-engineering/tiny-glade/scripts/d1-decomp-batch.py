"""D1 restorer batch decompiler for system-decorator decorator_visual functions.

Usage: python scripts/d1-decomp-batch.py <va-list.json> [--dbid tg_d1]
- va-list.json: [{"va": "0x1407bbee0", "note": "file :: fn"}]
- Writes decomp/<va>.txt + batch-report.json under
  evidence/country-core-deep/systems-recovery/groupD/decorator_visual/
"""
import json, os, subprocess, sys, time

ROOT = r"D:/game/reverse-engineering/tiny-glade"
BIN = os.path.join(ROOT, "workspace/input/tiny-glade.exe")
OUTDIR = os.path.join(
    ROOT,
    "evidence/country-core-deep/systems-recovery/groupD/decorator_visual",
)
LOCK = os.path.join(ROOT, "evidence/country-core-deep/.mcp-batch.lock")


def acquire_lock(timeout=7200):
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
    va_list = sys.argv[1]
    dbid = "tg_d1"
    if "--dbid" in sys.argv:
        dbid = sys.argv[sys.argv.index("--dbid") + 1]
    funcs = json.load(open(va_list, encoding="utf-8"))
    steps = [
        {
            "tool": "decompile_function",
            "args": {"database": dbid, "address": f["va"]},
            "timeout_ms": 240000,
            "allow_error": True,
            "_note": f["note"],
        }
        for f in funcs
    ]
    job = [
        {
            "tool": "open_database",
            "args": {
                "file_path": BIN,
                "run_auto_analysis": False,
                "database_id": dbid,
                "keep_open": True,
            },
            "timeout_ms": 600000,
        },
        {"tool": "wait_for_analysis", "args": {"database": dbid}, "timeout_ms": 600000},
    ] + steps + [
        {"tool": "close_database", "args": {"database": dbid, "save": False},
         "timeout_ms": 180000},
    ]
    os.makedirs(os.path.join(OUTDIR, "decomp"), exist_ok=True)
    jobpath = os.path.join(OUTDIR, "batch-job.json")
    outpath = os.path.join(OUTDIR, "batch-report.json")
    json.dump(job, open(jobpath, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print(f"[d1] {len(steps)} decompile steps -> {jobpath}")
    acquire_lock()
    print("[d1] MCP lock acquired, running batch...")
    try:
        r = subprocess.run(
            ["node", os.path.join(ROOT, "scripts/mcp-job.mjs"),
             "re_mcp_ghidra", jobpath, outpath],
            cwd=ROOT, capture_output=True, text=True,
        )
    finally:
        release_lock()
    print(r.stdout[-2000:])
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
                if "decompiled_code" in j:
                    text = j.get("decompiled_code", "")
                elif "disassembly" in j:
                    text = j.get("disassembly", "")
            except Exception:
                pass
        fn = os.path.join(OUTDIR, "decomp", f"decomp-{addr}.txt")
        open(fn, "w", encoding="utf-8").write((note + "\n" if note else "") + text)
        summary.append({"address": addr, "bytes": len(text), "error": is_err, "note": note})
    json.dump(summary, open(os.path.join(OUTDIR, "summary.json"), "w", encoding="utf-8"),
              ensure_ascii=False, indent=1)
    ok = sum(1 for s in summary if not s["error"])
    print(f"[d1] done: {ok}/{len(summary)} ok; decomp/ + summary.json written")
    for s in summary:
        if s["error"]:
            print("  ERR", s["address"], s["note"][:70])


if __name__ == "__main__":
    main()
