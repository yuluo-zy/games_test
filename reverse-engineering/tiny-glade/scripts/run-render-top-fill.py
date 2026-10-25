"""Run the render-top decompile fill and the 25 dangling refs.

Usage: python scripts/run-render-top-fill.py <dbid>
- reads round3/render-top-job.json (702 steps), runs in N concurrent MCP sessions
  (chunks), saves decomp files into evidence/country-core-deep/render-top/decomp/
- appends the 25 dangling refs (round3/dangling-refs.json) to the same batch
"""
import json, os, re, subprocess, sys, concurrent.futures

ROOT = r"D:/game/reverse-engineering/tiny-glade"
BIN = os.path.join(ROOT, "workspace/input/tiny-glade.exe")
OUT = os.path.join(ROOT, "evidence/country-core-deep/render-top")
os.makedirs(os.path.join(OUT, "decomp"), exist_ok=True)

steps = json.load(open(os.path.join(ROOT, "evidence/country-core-deep/round3/render-top-job.json"), encoding="utf-8"))
dangling = json.load(open(os.path.join(ROOT, "evidence/country-core-deep/round3/dangling-refs.json"), encoding="utf-8"))
# add dangling VAs not already in steps
have = {s["args"]["address"].lower() for s in steps}
for d in dangling:
    va = d["va"].lower()
    if va not in have:
        steps.append({"tool":"decompile_function","args":{"database":"DBID","address":d["va"]},
                      "timeout_ms":180000,"allow_error":True,"_note":d["chapter"]+" :: "+d["fn"][:60]})
print(f"total steps: {len(steps)}")

dbid = sys.argv[1]
chunks = 4
csteps = [steps[i::chunks] for i in range(chunks)]

def run(ci):
    cdbid = f"{dbid}_c{ci}"
    cs = [{**s, "args": {**s["args"], "database": cdbid}} for s in csteps[ci]]
    job = [
        {"tool":"open_database","args":{"file_path":BIN,"run_auto_analysis":False,"database_id":cdbid,"keep_open":True},"timeout_ms":600000},
        {"tool":"wait_for_analysis","args":{"database":cdbid},"timeout_ms":600000},
    ] + cs + [
        {"tool":"close_database","args":{"database":cdbid,"save":False},"timeout_ms":180000},
    ]
    jp = os.path.join(OUT, f"chunk-{ci}-job.json")
    op = os.path.join(OUT, f"chunk-{ci}-report.json")
    json.dump(job, open(jp, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    r = subprocess.run(["node", os.path.join(ROOT, "scripts/mcp-job.mjs"), "re_mcp_ghidra", jp, op],
                       cwd=ROOT, capture_output=True, text=True)
    return ci, op, r.returncode, r.stdout[-300:], r.stderr[-300:]

results = {}
with concurrent.futures.ThreadPoolExecutor(max_workers=chunks) as ex:
    for ci, op, rc, so, se in ex.map(run, range(chunks)):
        print(f"[chunk {ci}] exit={rc} {so[-150:]}")
        if rc != 0:
            print("CHUNK FAILED", se[-500:]); sys.exit(1)
        results[ci] = op

# merge reports, extract decomp files
merged = {"results": []}
for ci in sorted(results):
    rep = json.load(open(results[ci], encoding="utf-8"))
    merged["results"].extend(rep["results"])
json.dump(merged, open(os.path.join(OUT, "fill-report.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)

summary = []
for res in merged["results"]:
    if res["tool"] != "decompile_function": continue
    addr = res["args"]["address"]; note = res.get("_note", "")
    content = res["result"].get("content")
    text, is_err = "", bool(res["result"].get("isError"))
    if content:
        text = content[0].get("text", "")
        try:
            j = json.loads(text)
            if "decompiled_code" in j: text = j.get("decompiled_code", "")
            elif "disassembly" in j: text = j.get("disassembly", "")
        except Exception: pass
    fn = os.path.join(OUT, "decomp", f"decomp-{addr.lower()}.txt")
    open(fn, "w", encoding="utf-8").write((note + "\n" if note else "") + text)
    summary.append({"address": addr, "bytes": len(text), "error": is_err, "note": note})
json.dump(summary, open(os.path.join(OUT, "fill-summary.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
ok = sum(1 for s in summary if not s["error"])
print(f"render-top fill done: {ok}/{len(summary)} ok, {len(summary)-ok} NotFound (need hand-decode)")
for s in summary:
    if s["error"]: print("  ERR", s["address"], s["note"][:70])
