"""Supplementary single-function batch decompile (lock-serialized, terrain-water owner).

Usage: python scripts/run-supp-decomp.py <job.json> <out-dir>
Reuses run-decomp-batch's global lock so Ghidra MCP sessions stay serialized.
"""
import importlib.util, json, os, subprocess, sys

ROOT = r"D:/game/reverse-engineering/tiny-glade"
spec = importlib.util.spec_from_file_location(
    "rdb", os.path.join(ROOT, "scripts", "run-decomp-batch.py"))
rdb = importlib.util.module_from_spec(spec)
spec.loader.exec_module(rdb)  # reuse LOCK + acquire/release

def main():
    jobpath, outdir = sys.argv[1], sys.argv[2]
    os.makedirs(outdir, exist_ok=True)
    rdb.acquire_lock(False)
    try:
        out = os.path.join(outdir, "supp-report.json")
        r = subprocess.run(["node", os.path.join(ROOT, "scripts/mcp-job.mjs"),
                            "re_mcp_ghidra", jobpath, out], cwd=ROOT,
                           capture_output=True, text=True)
        print(r.stdout[-1500:])
        if r.returncode != 0:
            print("RUNNER FAILED:", r.stderr[-1500:])
            sys.exit(1)
        report = json.load(open(out, encoding="utf-8"))
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
            fn = os.path.join(outdir, f"decomp-{addr}.txt")
            open(fn, "w", encoding="utf-8").write((note + "\n" if note else "") + text)
            print(("ERR " if is_err else "OK  "), addr, len(text), note[:70])
    finally:
        rdb.release_lock()

if __name__ == "__main__":
    main()
