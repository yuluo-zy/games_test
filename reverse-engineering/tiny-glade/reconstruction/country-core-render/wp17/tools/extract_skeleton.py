#!/usr/bin/env python3
"""WP17 — condense skeleton bundles (enriched.json.gz + pdb.jsonl.gz) for the
history replay domain. Read-only; writes a per-module summary text file.

Usage: python3 -I extract_skeleton.py <skel_dir> <modules...> --out <outdir>
"""
import gzip, json, sys, os

def load_gz(path):
    with gzip.open(path, "rt") as f:
        return json.load(f)

def load_jsonl_gz(path):
    out = []
    with gzip.open(path, "rt") as f:
        for line in f:
            line = line.strip()
            if line:
                out.append(json.loads(line))
    return out

def rva_hex(v):
    try:
        return hex(v)
    except Exception:
        return str(v)

def main():
    args = sys.argv[1:]
    skel_dir = args[0]
    mods = []
    outdir = "."
    for a in args[1:]:
        if a == "--out":
            outdir = args[args.index("--out") + 1]
            break
        mods.append(a)
    os.makedirs(outdir, exist_ok=True)
    for mod in mods:
        lines = []
        lines.append(f"===== MODULE {mod} =====")
        # enriched
        ep = os.path.join(skel_dir, mod + ".rs.enriched.json.gz")
        if os.path.exists(ep):
            e = load_gz(ep)
            lines.append("## FUNCTIONS")
            for fn in e.get("functions", []):
                name = fn.get("name", "?")
                ci = fn.get("compiled_instances", [])
                rvas = [i.get("rva") for i in ci if i.get("rva")]
                ih = fn.get("inline_hosts", [])
                sl = fn.get("source_line", "")
                lines.append(f"  {name} | compiled={rvas} | hosts={len(ih)} | line={sl}")
                for h in ih:
                    lines.append(f"      host: {h.get('host_name')} @ {rva_hex(h.get('host_rva'))}")
            lines.append("## FILE INTERFACE CANDIDATES")
            for c in e.get("file_interface_candidates", []):
                lines.append(f"  [{c.get('candidate_id')}] {c.get('function_type_spelling')}")
        # pdb jsonl
        pp = os.path.join(skel_dir, mod + ".rs.pdb.jsonl.gz")
        if os.path.exists(pp):
            recs = load_jsonl_gz(pp)
            lines.append("## PDB RECORDS")
            for r in recs:
                if not isinstance(r, dict):
                    continue
                lines.append("  " + json.dumps(r, ensure_ascii=False)[:400])
        with open(os.path.join(outdir, mod + ".txt"), "w") as f:
            f.write("\n".join(lines))
        print(f"wrote {mod}.txt ({len(lines)} lines)")

if __name__ == "__main__":
    main()
