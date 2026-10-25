#!/usr/bin/env python3
"""Dump a skeleton bundle's enriched/pdb/structures evidence in compact form.

Usage:
  dump_bundle.py <bundle_stem> [--functions|--structures|--pdb]
  bundle_stem e.g. .../resources/tree_storage.rs  (all sidecars share this stem)

Run with -I (no script-dir on sys.path) against extracted bundle data.
"""
import gzip
import json
import sys

def load(stem, suffix):
    path = stem + "." + suffix
    if path.endswith(".gz"):
        with gzip.open(path, "rt") as f:
            return json.load(f)
    with open(path) as f:
        return json.load(f)

def dump_functions(data):
    for fn in data.get("functions", []):
        print(f"### {fn['id']} {fn['name']} type={fn.get('type_index')}")
        print(f"    compiled_instances={fn.get('compiled_instances')}")
        for host in fn.get("inline_hosts", []):
            print(f"    inline host: {host['kind']} {host['host_name']} rva=0x{host['host_rva']:x} line={host.get('source_line')} host_src={host.get('host_source_files')}")

def dump_structures(data):
    for s in data.get("structures", []):
        print(f"### struct {s.get('name')} size={s.get('size_bytes')} align={s.get('alignment')}")
        for fld in s.get("fields", []):
            print(f"    field {fld.get('name')} off={fld.get('offset')} size={fld.get('size')} type={fld.get('type')}")

def dump_pdb(data):
    # metadata records are in jsonl.gz; pdb.json is a summary
    print(json.dumps({k: v for k, v in data.items() if k != 'metadata_file'}, indent=1))

def main():
    stem = sys.argv[1]
    mode = sys.argv[2] if len(sys.argv) > 2 else "functions"
    if mode == "functions":
        dump_functions(load(stem, "enriched.json.gz"))
    elif mode == "structures":
        dump_structures(load(stem, "structures.json.gz"))
    elif mode == "pdb":
        dump_pdb(load(stem, "pdb.json"))
    elif mode == "pdbjsonl":
        path = stem + ".pdb.jsonl.gz"
        with gzip.open(path, "rt") as f:
            for line in f:
                rec = json.loads(line)
                print(json.dumps(rec))
    else:
        print("unknown mode", mode)
        sys.exit(2)

if __name__ == "__main__":
    main()
