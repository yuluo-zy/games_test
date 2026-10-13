#!/usr/bin/env python3
"""WP14 audio-domain skeleton extractor.

Reads the B skeleton bundles for crates/country-core/src/systems/audio/* (plus the
audio_trigger asset library) and writes a compact per-module text summary into
reconstruction/country-core-render/wp14/_analysis/ so the bundles can be
analyzed without carrying megabytes of JSON in the context.

Read-only against the skeleton; writes ONLY under the wp14 work dir.
Run with -I to avoid planting local modules.
"""
import gzip
import json
import os
import sys

ROOT = "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade"
SKEL = os.path.join(ROOT, "skeleton", "B", "crates", "country-core", "src")
OUT = os.path.join(ROOT, "reconstruction", "country-core-render", "wp14", "_analysis")
os.makedirs(OUT, exist_ok=True)


def read_gz(path):
    with gzip.open(path, "rt", encoding="utf-8") as fh:
        return json.load(fh)


def read_json(path):
    with open(path, "r", encoding="utf-8") as fh:
        return json.load(fh)


def dump_module(rel_dir, name, out):
    base = os.path.join(SKEL, rel_dir, name + ".rs")
    lines = []
    lines.append(f"==== {rel_dir}/{name}.rs ====")

    # ---- pdb.json summary ----
    pdbp = base + ".pdb.json"
    if os.path.exists(pdbp):
        d = read_json(pdbp)
        lines.append(f"metadata_records={d.get('metadata_records')} "
                     f"symbol_pairs={d.get('logical_symbol_and_type_pairs')} "
                     f"line_blocks={d.get('line_blocks')} "
                     f"inline_sites={d.get('inline_sites')} "
                     f"proc_instances={d.get('procedure_instances')}")

    # ---- structures.json.gz : named entities + definitions ----
    sp = base + ".structures.json.gz"
    if os.path.exists(sp):
        d = read_gz(sp)
        lines.append("-- named_symbol_entities --")
        for ent in d.get("named_symbol_entities", []):
            fields = ent.get("fields", "unknown")
            if isinstance(fields, list):
                fdesc = "; ".join(str(x) for x in fields)
            else:
                fdesc = str(fields)
            lines.append(f"  {ent.get('name')} :: {ent.get('definition_status')} "
                         f"size={ent.get('size_bytes')} align={ent.get('alignment')} "
                         f"enum={ent.get('enum_values')} :: {fdesc}")
        lines.append("-- definitions --")
        for dd in d.get("definitions", []):
            lines.append("  " + json.dumps(dd, ensure_ascii=False)[:800])

    # ---- enriched.json.gz : functions ----
    ep = base + ".enriched.json.gz"
    if os.path.exists(ep):
        d = read_gz(ep)
        for fn in d.get("functions", []):
            name = fn.get("name")
            tpi = fn.get("tpi_type_record")
            tpi_s = ""
            if tpi and "Procedure" in tpi:
                p = tpi["Procedure"]
                tpi_s = f"ret={p.get('ReturnType')} nparams={p.get('ParameterCount')}"
            lines.append(f"-- FUNC {name} :: {tpi_s}")
            for ci in fn.get("compiled_instances", []):
                lines.append(f"    compiled rva={ci}")
            for ih in fn.get("inline_hosts", []):
                lines.append(f"    inline_into {ih.get('host_name')} rva={ih.get('host_rva')} line={ih.get('source_line')}")
            for rc in fn.get("reference_callers", []):
                lines.append(f"    called_by {rc}")
            for si in fn.get("system_interfaces_from_own_symbol", []):
                lines.append(f"    own_system_iface {si}")
            for pte in fn.get("parameter_type_candidates", []):
                lines.append(f"    param_cand {pte}")
            for cid in fn.get("candidate_interface_ids", []):
                lines.append(f"    cand_iface_id {cid}")

    # ---- dependencies.json : imports + call graph ----
    dp = base + ".dependencies.json"
    if os.path.exists(dp):
        d = read_json(dp)
        crates = sorted({imp.get("crate") for imp in d.get("imports", [])})
        lines.append("imports: " + ", ".join(crates))
        for fn in d.get("functions", []):
            lines.append(f"-- DEPS {fn.get('name')}")
            for dep in fn.get("dependencies", []):
                rel = dep.get("relationship", "")
                f = dep.get("function", "")
                sf = dep.get("source_file", "")
                lines.append(f"    [{rel}] {f}  ({sf})")

    text = "\n".join(lines)
    with open(out, "w", encoding="utf-8") as fh:
        fh.write(text)
    return len(lines)


def main():
    targets = [
        ("systems/audio", "audio_update"),
        ("systems/audio", "building_audio"),
        ("systems/audio", "clutter_audio"),
        ("systems/audio", "decorator_audio"),
        ("systems/audio", "elevation_supports_audio"),
        ("systems/audio", "firstperson_steps"),
        ("systems/audio", "forest_audio"),
        ("systems/audio", "game_loading_audio"),
        ("systems/audio", "garden_audio"),
        ("systems/audio", "ivy_audio"),
        ("systems/audio", "menu_audio"),
        ("systems/audio", "mod"),
        ("systems/audio", "path_audio"),
        ("systems/audio", "playermade_stairs_audio"),
        ("systems/audio", "replay_audio"),
        ("systems/audio", "roof_audio"),
        ("systems/audio", "sheep_audio"),
        ("systems/audio", "signifier_audio"),
        ("systems/audio", "terrain_audio"),
        ("systems/audio", "terrain_stairs_audio"),
        ("systems/audio", "timberframe_audio"),
        ("systems/audio", "tree_audio"),
        ("systems/audio", "trigger_audio"),
        ("systems/audio", "wall_audio_fmod"),
        ("systems/audio", "wall_audio_orchestrator"),
        ("systems/audio", "wall_audio_path"),
        ("systems/audio", "wall_audio_ui"),
        ("systems/audio", "wall_audio_utils"),
        ("systems/audio", "water_ambience_audio"),
        ("systems/audio", "water_audio"),
        ("systems/audio", "weather_audio"),
        ("systems/audio/music", "music_player"),
        ("systems/audio/music", "music_system"),
        ("systems/audio/music", "music_debug_gui"),
        ("asset_libraries", "audio_trigger_library"),
        ("resources", "audio_resources"),
        ("resources", "audio_settings"),
        ("startup", "load_audio"),
        ("startup", "load_glade_audio"),
        ("utils", "audio_utils"),
        ("systems/wall/snapping", "snapping_audio"),
    ]
    total = 0
    for rel, name in targets:
        n = dump_module(rel, name, os.path.join(OUT, f"{rel.replace('/','__')}__{name}.txt"))
        print(f"{rel}/{name}: {n} lines")
        total += n
    print(f"TOTAL {total} lines -> {OUT}")


if __name__ == "__main__":
    main()
