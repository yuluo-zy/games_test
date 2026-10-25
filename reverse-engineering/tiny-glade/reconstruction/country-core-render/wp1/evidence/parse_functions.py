import gzip, json, sys
from pathlib import Path

root = Path("/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/B/crates/country-core/src/render/pass_builder")
mods = sys.argv[1:] if len(sys.argv)>1 else ["compute_pass_builder","pass_resource_access","draw_builder","bind_resources","pass_resources","uniform_binder","framebuffer_binding","draw_list","draw_stream","shader_uri"]
for m in mods:
    p = root / f"{m}.rs.enriched.json.gz"
    print("="*100)
    print(f"### {m}")
    with gzip.open(p, "rt") as f:
        data = json.load(f)
    funcs = data.get("functions", [])
    print(f"functions count: {len(funcs)}")
    for fn in funcs:
        name = fn.get("name")
        ti = fn.get("type_index")
        cinst = fn.get("compiled_instances", [])
        inst_desc = ""
        if cinst:
            inst_desc = " | compiled_instances: " + json.dumps(cinst)[:500]
        ih = fn.get("inline_hosts", [])
        nih = len(ih) if ih else 0
        print(f"  {name} [type_index={ti}] inline_hosts={nih}{inst_desc}")
    # metadata_records from pdb.json
    pdbf = root / f"{m}.rs.pdb.json"
    if pdbf.exists():
        pd = json.loads(pdbf.read_text())
        mr = pd.get("metadata_records", [])
        print(f"  pdb.json keys: {list(pd.keys())}")
        print(f"  metadata_records: {len(mr)}")
        for rec in mr[:3]:
            print("   MR:", json.dumps(rec)[:400])
