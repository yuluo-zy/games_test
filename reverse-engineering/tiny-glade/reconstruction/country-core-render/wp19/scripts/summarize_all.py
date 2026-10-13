#!/usr/bin/env python3
"""Summarize all WP19 skeleton bundles into one text file.

Usage: python summarize_all.py <skeleton-root> <outfile>
skeleton-root = skeleton/B/crates/country-core/src
"""
import gzip, json, sys, os

BUNDLES = [
    ("systems/render_loop/painterly", "mod.rs"),
    ("systems/render_loop", "vermeer.rs"),
    ("systems/render_loop", "menu_background.rs"),
    ("systems/render_loop/session_selection", "mod.rs"),
    ("systems/render_loop", "lut_gallery.rs"),
    ("systems/render_loop", "indirect_params.rs"),
    ("systems/render_loop/resources", "critters.rs"),
    ("resources", "gpu_lut.rs"),
    ("resources", "grading_lut.rs"),
    ("resources", "loading_screen.rs"),
    ("resources", "logo_animation.rs"),
]

def load_gz(p):
    with gzip.open(p, 'rt') as f:
        return json.load(f)

def summarize(d, base, out):
    enrich = os.path.join(d, base + '.enriched.json.gz')
    pdbg = os.path.join(d, base + '.pdb.json.gz')
    data = load_gz(enrich)
    src = data.get('source', {})
    out.write("=" * 100 + "\n")
    out.write(f"SOURCE: {src.get('path')} | modules={src.get('modules')}\n")
    out.write("=" * 100 + "\n")
    funcs = data.get('functions', [])
    out.write(f"\n## FUNCTIONS ({len(funcs)})\n")
    for f in funcs:
        name = f.get('name', '')
        insts = f.get('compiled_instances', [])
        hosts = f.get('inline_hosts', [])
        inst_info = ""
        if insts:
            for inst in insts[:4]:
                inst_info += f" rva={inst.get('rva')} size={inst.get('size')} mod={inst.get('module')}"
        host_info = ""
        if hosts:
            host_info = " inline_hosts=" + ",".join(f"{h.get('host_name')}@{h.get('source_line')}" for h in hosts[:4])
        out.write(f"  - {name}  inst={len(insts)}{inst_info} hosts={len(hosts)}{host_info}\n")
        ss = f.get('system_interfaces_from_own_symbol', [])
        if ss:
            out.write(f"      system_interfaces_from_own_symbol: {json.dumps(ss, ensure_ascii=False)[:500]}\n")
        ptypes = f.get('parameter_type_candidates', [])
        if ptypes:
            for p in ptypes[:4]:
                out.write(f"      param_candidate: {json.dumps(p, ensure_ascii=False)[:300]}\n")
    ents = data.get('named_symbol_entities', [])
    out.write(f"\n## NAMED SYMBOL ENTITIES ({len(ents)})\n")
    for e in ents:
        out.write(f"  - {e.get('name')} | {e.get('kind')} | size={e.get('size_bytes')} def_status={e.get('definition_status')}\n")
        procs = e.get('procedure_occurrences', [])
        seen = set()
        for p in procs:
            pn = p.get('name')
            key = (pn, p.get('rva'))
            if key in seen:
                continue
            seen.add(key)
            if len(seen) > 10:
                break
            out.write(f"        proc: {pn} rva={p.get('rva')}\n")
    # structures: named_symbol_entities + observed_game_structures
    structp = os.path.join(d, base + '.structures.json.gz')
    if os.path.exists(structp):
        st = load_gz(structp)
        sents = st.get('named_symbol_entities', [])
        obs = st.get('observed_game_structures', [])
        out.write(f"\n## STRUCTURES named_symbol_entities ({len(sents)})\n")
        for e in sents:
            out.write(f"  - {e.get('name')} | size={e.get('size_bytes')} fields={e.get('fields')}\n")
        out.write(f"\n## STRUCTURES observed_game_structures ({len(obs)})\n")
        for o in obs:
            out.write(f"  - {json.dumps(o, ensure_ascii=False)[:400]}\n")
    # PDB procedures with rva from pdb.jsonl.gz
    if os.path.exists(pdbg):
        plines = load_gz(pdbg)
        proc_names = set()
        if isinstance(plines, list):
            for ln in plines:
                if isinstance(ln, dict) and ln.get('event') == 'symbol' and ln.get('class') == 'procedure':
                    proc_names.add((ln.get('name'), ln.get('rva'), ln.get('code_size'), ln.get('module')))
        if proc_names:
            out.write(f"\n## PDB PROCEDURES ({len(proc_names)})\n")
            for nm, rva, sz, mod in sorted(proc_names, key=lambda x: (x[0] is None, x[0] or '')):
                out.write(f"  - {nm} rva={rva} size={sz} module={mod}\n")
    out.write("\n\n")

def main():
    root, outpath = sys.argv[1], sys.argv[2]
    with open(outpath, 'w') as out:
        for d, base in BUNDLES:
            full = os.path.join(root, d)
            if not os.path.exists(os.path.join(full, base + '.enriched.json.gz')):
                out.write(f"### MISSING {d}/{base}\n\n")
                continue
            summarize(full, base, out)
    print("wrote", outpath)

if __name__ == '__main__':
    main()
