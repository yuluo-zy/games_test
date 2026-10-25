#!/usr/bin/env python3
"""Build the WP19 ghidra read job: compute VAs from PDB rva or public-symbol VA.

VA = 0x140000000 + (pdb_rva - 0x1000)  [since section_offset = pdb_rva - 0x1000,
 and VA = 0x140000000 + section_offset]
"""
import json, os

BASE = 0x140000000

# functions known from public-symbols (exact)
pub = {
 "painterly_has_target": "0x1408a63d0",
 "painterly_pass": "0x1408a6410",
 "startup_painterly": "0x1408a7fe0",
 "vermeer_activate": "0x1408a8060",
 "vermeer_deactivate": "0x1408a8140",
 "vermeer_into_disable_hold": "0x1408a81e0",
 "vermeer_is_active": "0x1408a8260",
 "vermeer_should_freeze_camera": "0x1408a8270",
 "vermeer_pass": "0x1408a82f0",
 "vermeer_render_from_target": "0x1408a8d70",
 "startup_vermeer": "0x1408aa720",
 "render_menu_bg": "0x1408c9a30",
 "thumbnails_startup": "0x1408f6900",
 "render_session_selection_previews": "0x1408f6c00",
 "render_photomode_lut_gallery": "0x1408b0560",
 "gpu_lut_render_luts": "0x14090e280",
 "gpu_lut_plugin_build": "0x14090e080",
 "grading_flattened": "0x140b00b20",
 "grading_load_unwrapped": "0x140b00bc0",
 "grading_load_cube": "0x140b01050",
 "grading_luts_load": "0x140b01df0",
 "startup_loading_screen": "0x140a1a1e0",
 "logo_play": "0x140b19d00",
 "logo_is_waiting": "0x140b19d10",
 "logo_is_finished": "0x140b19d20",
 "logo_finish_wait": "0x140b19d30",
 "startup_animated_logo": "0x140b19d90",
 "animate_logo": "0x140b1a6b0",
}

# functions with pdb rva only -> compute
# Verified mapping (painterly_pass): pdb_rva 9069584 = 0x8a6410, public VA 0x1408a6410
# => VA = 0x140000000 + pdb_rva   (pdb rva is the file-image VA minus image base)
def from_rva(rva):
    return hex(BASE + rva)

# sanity checks against public-symbols VAs
check = {
 9069584: 0x1408a6410,  # painterly_pass
 9069520: 0x1408a63d0,  # has_target
 11533088+0x1000: 0x140b00b20,  # flattened  (pub offset 11533088 -> pdb rva +0x1000)
}
for rva, va in check.items():
    assert from_rva(rva) == hex(va), f"{rva} -> {from_rva(rva)} != {hex(va)}"
print("address mapping sanity checks passed")

rva_only = {
 "painterly_impl1_build": 9069536,          # impl$1::build
 "vermeer_impl1_build": 9077376,            # impl$1::build
 "menu_background_impl0_build": 9214400,    # impl$0::build
 "session_impl0_build": 9398464,            # impl$0::build
 "lut_gallery_impl0_build": 9110720,        # impl$0::build
 "gpu_lut_compute_impl": 9495312,           # impl$3::render_luts::impl$0::compute_impl
 "gpu_lut_build_closure0": 10950336,        # impl$0::build::closure$0 (also FnOnce call_once)
 "gpu_lut_build_closure1": 10951040,        # impl$0::build::closure$1
 "loading_screen_impl0_build": 10592656,    # impl$0::build
 "logo_impl2_build": 11640128,              # impl$2::build
 "grading_parse_rgb": 11541696,             # impl$2::load_cube::parse_rgb
}

funcs = {}
for k, v in pub.items():
    funcs[k] = v
for k, v in rva_only.items():
    funcs[k] = from_rva(v)

print("=== computed addresses ===")
for k in rva_only:
    print(f"{k}: {funcs[k]}")

# organize into batches of <= 20
items = list(funcs.items())
batches = [items[i:i+20] for i in range(0, len(items), 20)]
steps = []
for b in batches:
    steps.append({
        "tool": "get_functions",
        "args": {
            "program": "tiny-glade.exe",
            "functions": ",".join(v for _, v in b),
            "fields": "signature,decompiled_code,callees",
            "include_call_context": False
        },
        "timeout_seconds": 300
    })
job = steps
out = "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/reconstruction/country-core-render/wp19/scripts/job-wp19.json"
os.makedirs(os.path.dirname(out), exist_ok=True)
with open(out, 'w') as f:
    json.dump(job, f, indent=2)
print("wrote", out, "batches:", len(steps))
for i, b in enumerate(batches):
    print(f"batch{i}: {[k for k,_ in b]}")
