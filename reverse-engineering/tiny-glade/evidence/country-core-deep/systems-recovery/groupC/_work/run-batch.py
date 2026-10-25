"""GroupC re-analyst batch decompiler (tg_xC1).

Decompiles business VAs of system-wall-constructor / system-roof that lack
prior .c/.asm evidence. Writes evidence to
systems-recovery/groupC/<crate>/decomp/decomp-<va>.txt
"""
import json, os, re, subprocess, sys, time

ROOT = r"D:/game/reverse-engineering/tiny-glade"
WORK = os.path.join(ROOT, "evidence/country-core-deep/systems-recovery/groupC/_work")
BIN = os.path.join(ROOT, "workspace/input/tiny-glade.exe")
DBID = "tg_xc1"
CHUNKS = 2

def main():
    st = json.load(open(os.path.join(WORK, "target-status.json"), encoding="utf-8"))
    todo = [(va, v) for va, v in st.items() if v[2] == "-"]
    # drop the 3 with evidence/native .c
    native = {"0x1421aaea0", "0x1421a7710", "0x1421a6d40"}
    todo = [(va, v) for va, v in todo if va not in native]
    print(f"[groupC] {len(todo)} fresh decompile targets")
    steps = [{"tool": "decompile_function",
              "args": {"database": DBID, "address": va},
              "timeout_ms": 240000, "allow_error": True,
              "_note": f"{v[1]} :: {v[0]}"} for va, v in sorted(todo)]
    import concurrent.futures as cf
    chunk_steps = [steps[i::CHUNKS] for i in range(CHUNKS)]
    outdir = os.path.join(ROOT, "evidence/country-core-deep/systems-recovery/groupC")

    def run_chunk(ci):
        csteps = chunk_steps[ci]
        cdbid = f"{DBID}_c{ci}"
        csteps = [{**s, "args": {**s["args"], "database": cdbid}} for s in csteps]
        cjob = ([
            {"tool": "open_database",
             "args": {"file_path": BIN, "run_auto_analysis": False,
                      "database_id": cdbid, "keep_open": True},
             "timeout_ms": 600000},
            {"tool": "wait_for_analysis", "args": {"database": cdbid},
             "timeout_ms": 600000},
        ] + csteps + [
            {"tool": "close_database", "args": {"database": cdbid, "save": False},
             "timeout_ms": 180000},
        ])
        cjpath = os.path.join(WORK, f"chunk-{ci}-job.json")
        copath = os.path.join(WORK, f"chunk-{ci}-report.json")
        json.dump(cjob, open(cjpath, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
        r = subprocess.run(["node", os.path.join(ROOT, "scripts/mcp-job.mjs"),
                            "re_mcp_ghidra", cjpath, copath], cwd=ROOT,
                           capture_output=True, text=True, timeout=7200)
        return ci, copath, r.returncode, r.stdout[-300:], r.stderr[-300:]

    reports = {}
    with cf.ThreadPoolExecutor(max_workers=CHUNKS) as ex:
        for ci, copath, rc, so, se in ex.map(run_chunk, range(CHUNKS)):
            print(f"[chunk {ci}] exit={rc} {so[-200:]}")
            if rc != 0:
                print("CHUNK FAILED", se[-600:])
                sys.exit(1)
            reports[ci] = copath
    merged = {"results": []}
    for ci in sorted(reports):
        rep = json.load(open(reports[ci], encoding="utf-8"))
        merged["results"].extend(rep["results"])
    outpath = os.path.join(WORK, "batch-report.json")
    json.dump(merged, open(outpath, "w", encoding="utf-8"), ensure_ascii=False, indent=1)

    summary = []
    for res in merged["results"]:
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
        # determine crate dir from note
        crate = "roof" if note.startswith("roof") else "wall-constructor"
        d = os.path.join(outdir, crate, "decomp")
        os.makedirs(d, exist_ok=True)
        fn = os.path.join(d, f"decomp-{addr}.txt")
        open(fn, "w", encoding="utf-8").write((note + "\n" if note else "") + text)
        summary.append({"va": addr, "note": note, "bytes": len(text), "error": is_err})
    json.dump(summary, open(os.path.join(WORK, "batch-summary.json"), "w", encoding="utf-8"), indent=1)
    ok = [s for s in summary if not s["error"] and s["bytes"] > 40]
    print(f"OK {len(ok)} / {len(summary)}; errors: {[s['va'] for s in summary if s['error'] or s['bytes'] <= 40]}")

if __name__ == "__main__":
    main()
