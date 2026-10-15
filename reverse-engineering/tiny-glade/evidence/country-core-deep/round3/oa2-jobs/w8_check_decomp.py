#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""w8 worker: check decomp files validity + string assertions."""
import io, os, sys, json

BASE = r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep"

# addr -> (decomp_relpath_list, strings_to_assert, exact_strings_any)
ROWS = {
    "0x140002370": (["audio-misc/decomp/decomp-0x140002370.txt", "roofs-rt/decomp/decomp-0x140002370.txt"], [], []),
    "0x1408aa720": (["render-loop/decomp/decomp-0x1408aa720.txt"], [], []),
    "0x1408c9a30": (["render-loop/decomp/decomp-0x1408c9a30.txt"], [], []),
    "0x14090d850": (["render-loop/decomp/decomp-0x14090d850.txt"], ["Resolve to swapchain", "raster_resolve_to_swapchain", "raster_resolve_msaa_to_swapchain"], []),
    "0x14098bf60": (["render-loop/decomp/decomp-0x14098bf60.txt"], ["_Prefix merge", "/prefix_scan/inclusive_prefix_scan_merge.cs.hlsl", "/prefix_scan/inclusive_prefix_scan_subtotals.cs.hlsl"], []),
    "0x1409c40b0": (["render-loop/decomp/decomp-0x1409c40b0.txt"], [], []),
    "0x1409ccb20": (["render-loop/decomp/decomp-0x1409ccb20.txt"], [], []),
    "0x1409daaa0": (["render-loop/decomp/decomp-0x1409daaa0.txt"], ["Grass object push pass", "grass_object_push"], []),
    "0x140a0d190": (["render-loop/decomp/decomp-0x140a0d190.txt"], ["Reprojection map", "/calculate_reprojection_map.cs.hlsl", "Copy depth", "/copy_depth_to_r.cs.hlsl"], []),
    "0x140a32770": (["render-loop/decomp/decomp-0x140a32770.txt"], ["Animate buildings mask", "/rasterize_buildings_animate.cs.hlsl", "Blur buildings mask", "/gauss_blur_building_mask.cs.hlsl", "buildings blur temp"], []),
    "0x140a39e70": (["render-loop/decomp/decomp-0x140a39e70.txt"], ["Screen shadows temporal", "/apply_screen_shadows.cs.hlsl", "Shadow mask mean", "/shadow_mask_mean.cs.hlsl"], []),
    "0x140a3ba60": (["render-loop/decomp/decomp-0x140a3ba60.txt"], [], []),
    "0x140a5a570": (["render-loop/decomp/decomp-0x140a5a570.txt"], ["CoC dilated", "DoF tile offsets", "DoF tile coords", "DoF dispatch args", "Depth of field", "/dof/fused.cs.hlsl"], []),
    "0x140a66970": ([], [], []),
    "0x140a8a090": (["render-loop/decomp/decomp-0x140a8a090.txt"], [], []),
    "0x14089d500": (["roofs-rt/decomp/decomp-0x14089d500.txt"], [], []),
    "0x14089d550": (["roofs-rt/decomp/decomp-0x14089d550.txt"], [], []),
    "0x1408e30f0": (["roofs-rt/decomp/decomp-0x1408e30f0.txt"], [], []),
    "0x1408e4040": (["roofs-rt/decomp/decomp-0x1408e4040.txt"], [], []),
    "0x1408e45b0": (["roofs-rt/decomp/decomp-0x1408e45b0.txt"], [], []),
    "0x1408e45f0": (["roofs-rt/decomp/decomp-0x1408e45f0.txt"], [], []),
    "0x140940d80": (["roofs-rt/decomp/decomp-0x140940d80.txt"], [], []),
    "0x140a38460": (["roofs-rt/decomp/decomp-0x140a38460.txt"], [], []),
    "0x140a385a0": (["roofs-rt/decomp/decomp-0x140a385a0.txt"], [], []),
    "0x140ae6570": (["roofs-rt/decomp/decomp-0x140ae6570.txt"], [], []),
}

out = []
for addr, (files, must, anyw) in ROWS.items():
    entry = {"addr": addr, "files": []}
    for rel in files:
        p = os.path.join(BASE, rel.replace("\\", os.sep))
        rec = {"rel": rel, "exists": os.path.exists(p)}
        if rec["exists"]:
            with io.open(p, "r", encoding="utf-8", errors="replace") as f:
                txt = f.read()
            rec["size"] = len(txt)
            rec["head"] = txt[:240].replace("\n", "\\n")
            low = txt.lower()
            rec["notfound"] = ("notfound" in low) or ("no function at" in low) or ("error" in low[:400].lower())
        entry["files"].append(rec)
    # assertion grep on first file
    if files:
        p0 = os.path.join(BASE, files[0].replace("\\", os.sep))
        if os.path.exists(p0):
            with io.open(p0, "r", encoding="utf-8", errors="replace") as f:
                txt = f.read()
            entry["asserts"] = {s: (s in txt) for s in must}
    out.append(entry)

with io.open(sys.argv[1], "w", encoding="utf-8") as f:
    json.dump(out, f, ensure_ascii=False, indent=1)
print("done", len(out))
