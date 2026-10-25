#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Build radare2 RNE-oracle job: spot-check branch heads of the small critter
functions plus data constants referenced by the behaviour chain.
"""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "critters-radare.json")

RNE = "/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne"

OPEN = {"tool": "open_file", "args": {"file_path": RNE}, "timeout_seconds": 120}
INFO = {"tool": "show_info", "args": {}, "timeout_seconds": 120}
CLOSE = {"tool": "close_file", "args": {}, "timeout_seconds": 120}

# (address, label) -> disassemble 24 instructions at each spot
SPOTS = [
    ("0x14230ff00", "plan_duck_trajectories head"),
    ("0x142312200", "move_ducks_along_trajectories head"),
    ("0x142313b30", "update_duck_goals head"),
    ("0x142314470", "set_duck_velocities_towards_goals head"),
    ("0x142314b70", "update_duck_ripples head"),
    ("0x1423179a0", "critters_avoid_collisions head"),
    ("0x142318c10", "smooth_critter_velocities head"),
    ("0x142318fa0", "move_critters head"),
    ("0x14231bd80", "sheep_walk_on_terrain head"),
    ("0x142315050", "update_sheep_goals head"),
]

STEPS = [OPEN, INFO]
for addr, label in SPOTS:
    STEPS.append({
        "tool": "disassemble",
        "args": {"address": addr, "num_instructions": 16},
        "timeout_seconds": 120,
    })
STEPS.append(CLOSE)

with open(OUT, "w") as fh:
    json.dump(STEPS, fh, indent=2)
    fh.write("\n")
print("written", OUT, "steps:", len(STEPS))
