#!/usr/bin/env python3
"""Summarize a PDB skeleton bundle set for roof recovery.

Usage: summarize_bundle.py <dir> <base-name>
Reads <base>.rs.structures.json.gz / .enriched.json.gz / .dependencies.json / .pdb.json
Prints a compact human/agent-readable summary to stdout.
"""
import gzip, json, sys, os

def load_gz(p):
    with gzip.open(p, 'rt') as f:
        return json.load(f)

def load(p):
    with open(p, 'rt') as f:
        return json.load(f)

def main():
    d, base = sys.argv[1], sys.argv[2]
    print(f"=== {base} ===")

    # pdb.json metadata
    pdbp = os.path.join(d, f"{base}.rs.pdb.json")
    if os.path.exists(pdbp):
        p = load(pdbp)
        print(f"[pdb] lines={p.get('line_blocks')} inline={p.get('inline_sites')} "
              f"proc_instances={p.get('procedure_instances')} types={p.get('type_indices')} "
              f"modules={p.get('modules')}")
        print(f"[pdb] source_sha256={p.get('source_sha256')}")

    # structures.json.gz
    sp = os.path.join(d, f"{base}.rs.structures.json.gz")
    if os.path.exists(sp):
        s = load_gz(sp)
        print("--- structures ---")
        for e in s.get('named_symbol_entities', []):
            print(f"  N {e['name']} | {e.get('definition_status')} | size={e.get('size_bytes')} "
                  f"align={e.get('alignment')} fields={e.get('fields')}")
        for e in s.get('observed_game_structures', []):
            print(f"  O {e.get('name')} | {e.get('definition_status')} | size={e.get('size_bytes')}")
            for fl in e.get('fields', []) or []:
                if isinstance(fl, dict):
                    print(f"      field {fl.get('name')} off={fl.get('offset')} ty={fl.get('type')} "
                          f"bits={fl.get('bits')}")
        for e in s.get('definitions', []):
            if isinstance(e, dict) and e.get('name'):
                print(f"  D {e['name']} | {e.get('kind')}")

    # enriched.json.gz functions
    ep = os.path.join(d, f"{base}.rs.enriched.json.gz")
    if os.path.exists(ep):
        e = load_gz(ep)
        print("--- functions ---")
        for f in e.get('functions', []):
            name = f['name']
            inst = f.get('compiled_instances', [])
            rvas = [i.get('rva') for i in inst]
            src_lines = [i.get('source_line') for i in inst]
            unk = f.get('unknowns', {})
            sig = unk.get('complete_rust_signature', '?')
            params = [p for p in f.get('parameter_type_candidates', [])]
            ci = f.get('candidate_interface_ids', [])
            print(f"  F {name}")
            print(f"      rva={rvas} line={src_lines} params_cand={len(params)} cand_iface={len(ci)}")
            print(f"      unknown sig={sig}")
            for p in params[:6]:
                print(f"        param: {json.dumps(p)[:220]}")

    # dependencies.json functions list (call graph)
    dp = os.path.join(d, f"{base}.rs.dependencies.json")
    if os.path.exists(dp):
        deps = load(dp)
        print("--- imports (first 6) ---")
        for i in deps.get('imports', [])[:6]:
            print(f"  use {i.get('use_candidate')} [{i.get('ownership_evidence')}]")
        print(f"  imports_total={len(deps.get('imports', []))}")
        print("--- functions in call graph ---")
        for f in deps.get('functions', []):
            nm = f.get('name', '?')
            called = f.get('called_functions') or f.get('callees') or []
            print(f"  {nm} callees={len(called)}")

if __name__ == '__main__':
    main()
