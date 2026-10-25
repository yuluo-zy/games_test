"""D2 restorer batch 2: private helpers + dispatch handlers (tg_d2).
"""
import json, os, subprocess, sys
import concurrent.futures as cf

ROOT = r"D:/game/reverse-engineering/tiny-glade"
WORK = os.path.join(ROOT, "evidence/country-core-deep/systems-recovery/groupD/ui_systems/_work")
BIN = os.path.join(ROOT, "workspace/input/tiny-glade.exe")
DBID = "tg_d2"
CHUNKS = 1

TARGETS = {
    "0x141276e30": "shared op/apply helper (chimney/move/place/focus/splitting callers)",
    "0x14127fdd0": "show_decorator_remove_signifier builder",
    "0x14127cab0": "raycast_with_ss_offset helper",
    "0x14125cf90": "splitting/focus helper A",
    "0x14125cef0": "splitting/focus helper B",
    "0x14125e560": "ui_move_decorator helper",
    "0x14125de50": "ui_place_decorator helper",
    "0x14127a480": "handle_moving_out_wall_decorator_part inner",
    "0x14127c7f0": "handle_moving_out_roof_decorator_part inner",
    "0x1412803f0": "SystemParam init glue A",
    "0x1412811c0": "SystemParam init glue B",
    "0x141280b50": "SystemParam init glue C",
    "0x1412804e0": "SystemParam init glue D",
    "0x1407a91bb": "rotate wall handler",
    "0x1407a8766": "rotate roof handler",
    "0x1407a87a6": "rotate terrain handler",
    "0x1407a8814": "rotate stair handler",
    "0x1407adba7": "stair-height wall handler",
    "0x1407ad91a": "stair-height roof handler",
    "0x1407ad998": "stair-height terrain handler",
    "0x14079b860": "fn after ui_decorator_inactive",
}

def main():
    steps = [{"tool": "decompile_function",
              "args": {"database": DBID, "address": va},
              "timeout_ms": 240000, "allow_error": True,
              "_note": name} for va, name in sorted(TARGETS.items())]
    chunk_steps = [steps[i::CHUNKS] for i in range(CHUNKS)]
    outdir = os.path.join(ROOT, "evidence/country-core-deep/systems-recovery/groupD/ui_systems/decomp")
    os.makedirs(outdir, exist_ok=True)

    def run_chunk(ci):
        csteps = chunk_steps[ci]
        cdbid = f"{DBID}_b2"
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
        cjpath = os.path.join(WORK, f"chunk-b2-job.json")
        copath = os.path.join(WORK, f"chunk-b2-report.json")
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
    outpath = os.path.join(WORK, "batch-b2-report.json")
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
                    text = j["decompiled_code"]
            except Exception:
                pass
        status = "ERR" if is_err else "OK"
        if not is_err:
            dp = os.path.join(outdir, f"decomp-{addr}.txt")
            with open(dp, "w", encoding="utf-8") as f:
                f.write(f"// {note}\n")
                f.write(text)
        summary.append({"va": addr, "note": note, "status": status, "chars": len(text)})
    with open(os.path.join(WORK, "batch-b2-summary.json"), "w", encoding="utf-8") as f:
        json.dump(summary, f, ensure_ascii=False, indent=1)
    for s in summary:
        print(f"{s['va']} {s['status']} {s['chars']} {s['note']}")

if __name__ == "__main__":
    main()
