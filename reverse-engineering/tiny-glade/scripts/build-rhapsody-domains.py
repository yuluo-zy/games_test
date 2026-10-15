#!/usr/bin/env python3
"""Build per-domain symbol manifests for rhapsody deep-dive.

Merges two sources:
  - skeleton/B/crates/rhapsody/**/*.rs  (PDB metadata: symbol names + record form)
  - evidence/render-architecture/public-symbols.json  (S_PUB32: demangled + VA)

Emits evidence/country-core-deep/rhapsody/domains/<domain>.json:
  [{ "name", "form" (compiled_procedure|inline_instance|proc_macro|unknown),
     "line", "va" (or null), "spub32_name" }]

Domains follow the skeleton file layout:
  rg        src/rg/*.rs
  raw-core  src/raw/{device,buffer,texture,swapchain,shader,command_buffer,barrier,
                     submit,descriptor_set,binding,render_pass,framebuffer,
                     dynamic_constants,debug,profiler,sampler,semaphore,stats,
                     surface,temp_list}.rs
  raw-init  src/raw/{init,instance,physical_device,format,allocator,crash,disposal,
                     error,bytes,workarounds,ray_tracing}.rs + src/leek.rs + src/lib.rs
  managed   src/raw/managed/*.rs
  crates    crates/{shader-pipeline,gpu-profiler,vk-sync-rs,aftermath-rs,
                    dxgi-enumerate-gpus}/src/**/*.rs
"""
import json, os, re, sys

ROOT = r"D:/game/reverse-engineering/tiny-glade"
SKEL = os.path.join(ROOT, "skeleton/B/crates/rhapsody")
PUB = os.path.join(ROOT, "evidence/render-architecture/public-symbols.json")
OUT = os.path.join(ROOT, "evidence/country-core-deep/rhapsody/domains")

DOMAINS = {
    "rg": ["src/rg"],
    "raw-core": ["src/raw/device.rs", "src/raw/buffer.rs", "src/raw/texture.rs",
                 "src/raw/swapchain.rs", "src/raw/shader.rs", "src/raw/command_buffer.rs",
                 "src/raw/barrier.rs", "src/raw/submit.rs", "src/raw/descriptor_set.rs",
                 "src/raw/binding.rs", "src/raw/render_pass.rs", "src/raw/framebuffer.rs",
                 "src/raw/dynamic_constants.rs", "src/raw/debug.rs", "src/raw/profiler.rs",
                 "src/raw/sampler.rs", "src/raw/semaphore.rs", "src/raw/stats.rs",
                 "src/raw/surface.rs", "src/raw/temp_list.rs"],
    "raw-init": ["src/raw/init.rs", "src/raw/instance.rs", "src/raw/physical_device.rs",
                 "src/raw/format.rs", "src/raw/allocator.rs", "src/raw/crash.rs",
                 "src/raw/disposal.rs", "src/raw/error.rs", "src/raw/bytes.rs",
                 "src/raw/workarounds.rs", "src/raw/ray_tracing.rs", "src/leek.rs", "src/lib.rs"],
    "managed": ["src/raw/managed"],
    "crates": ["crates/shader-pipeline", "crates/gpu-profiler", "crates/vk-sync-rs",
               "crates/aftermath-rs", "crates/dxgi-enumerate-gpus"],
}

def parse_skeleton_files(paths):
    """Yield (relpath, list_of_symbols) for each skeleton file under the domain paths."""
    for base in paths:
        abs_base = os.path.join(SKEL, base)
        if os.path.isfile(abs_base):
            files = [base]
        elif os.path.isdir(abs_base):
            files = []
            for dp, _, fns in os.walk(abs_base):
                for fn in sorted(fns):
                    if fn.endswith(".rs"):
                        files.append(os.path.relpath(os.path.join(dp, fn), SKEL).replace("\\", "/"))
        else:
            print(f"  [skip] {base} not found", file=sys.stderr)
            continue
        for rel in files:
            path = os.path.join(SKEL, rel)
            with open(path, encoding="utf-8", errors="replace") as f:
                text = f.read()
            syms = []
            cur = None
            for line in text.splitlines():
                m = re.match(r"^// 函数符号: (.+)$", line)
                if m:
                    cur = {"name": m.group(1).strip(), "form": "unknown",
                           "line": None, "va": None, "spub32_name": None}
                    syms.append(cur)
                    continue
                if cur is not None:
                    m = re.match(r"^// 记录形式: (\w+)", line)
                    if m:
                        cur["form"] = m.group(1)
                    m = re.match(r"^// 源码行: (\d+)", line)
                    if m:
                        cur["line"] = int(m.group(1))
                    if re.match(r"^// [^ ]|^// BEGIN|^// END|^$", line):
                        # keep scanning block until next symbol / comment block ends
                        pass
            yield rel, syms

def build_spub32_map():
    d = json.load(open(PUB, encoding="utf-8"))
    by_va = {}
    by_name = {}
    for e in d:
        va = e.get("va")
        dm = e.get("demangled", "")
        if not isinstance(dm, str) or not dm:
            continue
        if "rhapsody" not in dm and not dm.startswith("_$LT$") and ".." not in dm:
            continue
        # heuristically accept rhapsody workspace symbols: rhapsody::, or crates modules
        if not any(k in dm for k in ("rhapsody", "shader_pipeline", "gpu_profiler",
                                     "vk_sync", "aftermath", "dxgi")):
            continue
        if va:
            by_va[va] = dm
        clean = re.sub(r"::h[0-9a-f]{16}$", "", dm)
        by_name.setdefault(clean, dm)
    return by_va, by_name

def match_va(symbol_name, by_name, by_va):
    """Try to find a VA for a skeleton symbol name (fuzzy, best-effort)."""
    n = symbol_name
    # exact or demangled-with-hash
    if n in by_name:
        return by_name[n], None
    # hash-stripped demangled forms are stored keyed by stripped name; skeleton may
    # carry trailing instance suffix like (function::closure$0) or impl$N. Try contains.
    best = None
    for key, dm in by_name.items():
        if key.startswith(n + "::") or n.startswith(key + "::") or key == n:
            best = dm
            break
    return best, None

def main():
    os.makedirs(OUT, exist_ok=True)
    by_va, by_name = build_spub32_map()
    # reverse: demangled-name -> va
    name_to_va = {}
    d = json.load(open(PUB, encoding="utf-8"))
    for e in d:
        dm = e.get("demangled", "")
        va = e.get("va")
        if isinstance(dm, str) and dm and va:
            name_to_va.setdefault(re.sub(r"::h[0-9a-f]{16}$", "", dm), va)
    summary = {}
    for dom, paths in DOMAINS.items():
        rows = []
        for rel, syms in parse_skeleton_files(paths):
            for s in syms:
                va = name_to_va.get(s["name"])
                row = {"name": s["name"], "form": s["form"], "line": s["line"],
                       "va": va, "spub32_name": None, "file": rel}
                if va:
                    row["spub32_name"] = s["name"]
                rows.append(row)
        comp = [r for r in rows if r["form"] == "compiled_procedure"]
        withva = [r for r in rows if r["va"]]
        summary[dom] = {"symbols": len(rows), "compiled": len(comp), "with_va": len(withva)}
        json.dump(rows, open(os.path.join(OUT, f"{dom}.json"), "w", encoding="utf-8"),
                  ensure_ascii=False, indent=0)
        print(f"[{dom}] symbols={len(rows)} compiled={len(comp)} with_va={len(withva)}")
    json.dump(summary, open(os.path.join(OUT, "SUMMARY.json"), "w", encoding="utf-8"),
              ensure_ascii=False, indent=1)
    print("written to", OUT)

if __name__ == "__main__":
    main()
