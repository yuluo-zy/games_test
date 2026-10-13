#!/usr/bin/env python3
"""Summarize a skeleton bundle (enriched.json.gz) for one .rs source file.

Usage: python summarize_bundle.py <path-to-dir-containing-X.rs.enriched.json.gz> <base-name>
Example: python summarize_bundle.py <skel>/systems/render_loop/painterly mod.rs
  -> looks for <skel>/systems/render_loop/painterly/mod.rs.enriched.json.gz
Prints: named symbol entities (types), functions (procedures w/ rva/size),
interface catalog, and PDB symbol names from pdb.jsonl.gz.
"""
import gzip, json, sys, os

def load_gz(p):
    with gzip.open(p, 'rt') as f:
        return json.load(f)

def main():
    if len(sys.argv) < 3:
        print("usage: summarize_bundle.py <dir> <base>"); sys.exit(1)
    d, base = sys.argv[1], sys.argv[2]
    enrich = os.path.join(d, base + '.enriched.json.gz')
    data = load_gz(enrich)
    src = data.get('source', {})
    print("=" * 100)
    print("SOURCE:", src.get('path'), "| sha:", str(src.get('source_sha256'))[:12])
    print("=" * 100)
    funcs = data.get('functions', [])
    print(f"\n## FUNCTIONS ({len(funcs)})")
    for f in funcs:
        name = f.get('name', '')
        insts = f.get('compiled_instances', [])
        hosts = f.get('inline_hosts', [])
        inst_info = ""
        if insts:
            for inst in insts[:4]:
                inst_info += f" rva={inst.get('rva')} size={inst.get('size')} mod={inst.get('module')}"
        tpi = f.get('tpi_type_record')
        host_info = ""
        if hosts:
            host_info = " inline_hosts=" + ",".join(f"{h.get('host_name')}@{h.get('source_line')}" for h in hosts[:4])
        print(f"  - {name}  inst={len(insts)}{inst_info} hosts={len(hosts)}{host_info}")
        ss = f.get('system_interfaces_from_own_symbol', [])
        if ss:
            print(f"      system_interfaces_from_own_symbol: {json.dumps(ss, ensure_ascii=False)[:400]}")
        ptypes = f.get('parameter_type_candidates', [])
        if ptypes:
            for p in ptypes[:4]:
                print(f"      param_candidate: {json.dumps(p, ensure_ascii=False)[:220]}")
    ents = data.get('named_symbol_entities', [])
    print(f"\n## NAMED SYMBOL ENTITIES ({len(ents)})")
    for e in ents:
        print(f"  - {e.get('name')} | {e.get('kind')} | size={e.get('size_bytes')} def_status={e.get('definition_status')}")
        procs = e.get('procedure_occurrences', [])
        for p in procs[:6]:
            print(f"        proc: {p.get('name')} rva={p.get('rva')}")
    cat = data.get('interface_catalog', {})
    print(f"\n## INTERFACE CATALOG keys: {list(cat.keys()) if isinstance(cat, dict) else type(cat)}")
    if isinstance(cat, dict):
        for k, v in cat.items():
            if isinstance(v, list):
                print(f"  {k}: {len(v)} items")
                for it in v[:10]:
                    print("      ", json.dumps(it, ensure_ascii=False)[:240])
            else:
                print(f"  {k}: {json.dumps(v, ensure_ascii=False)[:240]}")
    fics = data.get('file_interface_candidates', [])
    print(f"\n## FILE INTERFACE CANDIDATES ({len(fics) if isinstance(fics, list) else 'n/a'})")
    if isinstance(fics, list):
        for it in fics[:20]:
            print("  ", json.dumps(it, ensure_ascii=False)[:300])

if __name__ == '__main__':
    main()
