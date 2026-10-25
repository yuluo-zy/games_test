"""Lock-respecting runner for remaining wall decompiles (tg_a2 domain).

Uses the same lock protocol as scripts/run-decomp-batch.py
(evidence/country-core-deep/.mcp-batch.lock, O_CREAT|O_EXCL), runs one
RE-MCP session for remain-job.json, releases the lock, then extracts
decomp-<va>.txt files the same way the batch runner does.
"""
import json, os, subprocess, sys, time

ROOT = r"D:/game/reverse-engineering/tiny-glade"
HERE = os.path.join(ROOT, "evidence", "country-core-deep", "wall")
LOCK = os.path.join(ROOT, "evidence", "country-core-deep", ".mcp-batch.lock")
NODE = os.path.join(ROOT, "scripts", "mcp-job.mjs")


def acquire(timeout=5400):
    start = time.time()
    while True:
        try:
            fd = os.open(LOCK, os.O_CREAT | os.O_EXCL | os.O_WRONLY)
            os.write(fd, f"{os.getpid()} {time.time()} wall-remaining\n".encode())
            os.close(fd)
            print("LOCK acquired", flush=True)
            return
        except FileExistsError:
            if time.time() - start > timeout:
                print("LOCK TIMEOUT", file=sys.stderr)
                sys.exit(1)
            time.sleep(10)


def release():
    try:
        os.unlink(LOCK)
    except OSError:
        pass


def extract(report_path):
    vamap = json.load(open(os.path.join(HERE, "va-map.json"), encoding="utf-8"))
    rep = json.load(open(report_path, encoding="utf-8"))
    n_ok = n_err = 0
    for res in rep.get("results", []):
        if res.get("tool") != "decompile_function":
            continue
        addr = res["args"]["address"]
        iserr = bool(res.get("result", {}).get("isError"))
        text = ""
        content = res.get("result", {}).get("content") or []
        if content:
            t = content[0].get("text", "")
            try:
                j = json.loads(t)
                text = j.get("decompiled_code") or j.get("disassembly") or t
            except Exception:
                text = t
        note = vamap.get(addr, {}).get("name", "") + " :: " + vamap.get(addr, {}).get("file", "")
        open(os.path.join(HERE, "decomp", f"decomp-{addr}.txt"), "w",
             encoding="utf-8").write((note + "\n" if note else "") + text)
        if iserr:
            n_err += 1
        else:
            n_ok += 1
    print(f"extracted ok={n_ok} err={n_err}", flush=True)
    return n_ok, n_err


def main():
    acquire()
    try:
        r = subprocess.run(["node", NODE, "re_mcp_ghidra",
                            os.path.join(HERE, "extra-job.json"),
                            os.path.join(HERE, "extra-report.json")],
                           cwd=ROOT, capture_output=True, text=True)
        print("node exit", r.returncode)
        print(r.stdout[-800:])
        if r.returncode != 0:
            print("STDERR:", r.stderr[-800:])
    finally:
        release()
    extract(os.path.join(HERE, "extra-report.json"))


if __name__ == "__main__":
    main()
