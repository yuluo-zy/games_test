#!/usr/bin/env python3
"""Dump PDB function names from skeleton enriched json.gz bundles (read-only)."""
import gzip, json, sys, glob

root = "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/B/crates/country-core/src/"

def load(path):
    d = json.loads(gzip.open(path, "rt").read())
    return d.get("functions", [])

def main(patterns):
    seen = set()
    for pat in patterns:
        for p in sorted(glob.glob(root + pat)):
            try:
                funcs = load(p)
            except Exception as e:
                print(f"# ERROR {p}: {e}", file=sys.stderr)
                continue
            rel = p[len(root):]
            print(f"### {rel} ({len(funcs)} functions)")
            for f in funcs:
                name = f.get("name", "")
                if name in seen:
                    continue
                seen.add(name)
                # caller info
                callers = f.get("reference_callers") or []
                hosts = []
                for ih in (f.get("inline_hosts") or []):
                    hosts.append(f"{ih.get('host_name')}@{ih.get('host_rva')}")
                print(f"  {name}" + (f"  [callers:{len(callers)}]" if callers else "") + (f"  [inline_host:{hosts[0]}]" if hosts else ""))

if __name__ == "__main__":
    patterns = sys.argv[1:] or ["input/*.rs", "startup/*.rs", "startup/glade_themes/*.rs", "systems/ui/*.rs", "systems/ui/signifiers/*.rs", "systems/ui/resources/*.rs", "geometry/*.rs", "utils/*.rs", "app_loading_state.rs", "game_config.rs", "game_profile.rs", "game_key_bindings.rs", "meta_commander.rs", "lib.rs"]
    main(patterns)
