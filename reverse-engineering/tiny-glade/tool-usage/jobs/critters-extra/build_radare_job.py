#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Build radare2 RNE-oracle job for critters-extra: entry disassembly at each
new function plus a float-constant read set (added after reva disasm pinpoints
__real_* addresses)."""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "critters-extra-radare.json")

RNE = "/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne"

OPEN = {"tool": "open_file", "args": {"file_path": RNE}, "timeout_seconds": 120}
INFO = {"tool": "show_info", "args": {}, "timeout_seconds": 120}
CLOSE = {"tool": "close_file", "args": {}, "timeout_seconds": 120}

SPOTS = [
    ("0x142326a80", "sim_wren_boids head"),
    ("0x141dc3510", "Boid_frame_update_wren head"),
    ("0x141dc2540", "BoidFlock_remove_wren head"),
    ("0x141dc2170", "Boid_random_vel_change head"),
    ("0x140f90cd0", "fish head"),
    ("0x140f909f0", "FishSpawnLocations_new head"),
    ("0x140f90c10", "despawn_fish head"),
    ("0x141dc2740", "Boid_frame_update_fish head"),
    ("0x141dc2340", "BoidFlock_remove_fish head"),
    ("0x141d84d40", "update_butterfly_states head"),
    ("0x141d86060", "update_butterfly_goals head"),
    ("0x141d867b0", "set_butterfly_velocities head"),
    ("0x141d86eb0", "update_butterfly_audio head"),
    ("0x142316940", "update_firefly_goals head"),
    ("0x142317040", "set_firefly_velocities head"),
    ("0x140f8ee90", "DuckSpawnLocations_new head"),
    ("0x140f8f210", "spawn_ducks head"),
    ("0x140f8f8b0", "despawn_ducks head"),
    ("0x1409d49a0", "startup_critters head"),
    ("0x141dc42f0", "wren_boids_closure3 head"),
]

STEPS = [OPEN, INFO]
for addr, label in SPOTS:
    STEPS.append({
        "tool": "disassemble",
        "args": {"address": addr, "num_instructions": 20},
        "timeout_seconds": 120,
    })
STEPS.append(CLOSE)

with open(OUT, "w") as fh:
    json.dump(STEPS, fh, indent=2)
    fh.write("\n")
print("written", OUT, "steps:", len(STEPS))
