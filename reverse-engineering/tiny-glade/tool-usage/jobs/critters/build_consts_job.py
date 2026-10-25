#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Read float constants used by critter systems and confirm callee names."""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "critters-consts.json")

FLOATS = [
    "0x142925904",
    "0x14295a0e8",
    "0x142934694",
    "0x142ad58e8",
    "0x142a7b658",
    "0x1429258f4",
    "0x1429258f8",
    "0x1429258fc",
    "0x142925870",
    "0x142a7c88c",
    "0x1429edb90",
    "0x1429cd030",
]

CALLEES = [
    "0x141246fb0",  # terrain height sample
    "0x140a14df0",  # bilinear_sample_at_world_xz<RasterBufferRef<512>>
    "0x14261f0c0",  # bilinear_sample_at_world_xz<WaterAudioGrid>
    "0x141625ae0",  # perchance uniform_f32
    "0x140c90700",  # uniform_sample_disk
    "0x140c90770",  # uniform_sample_circle
    "0x140c90940",  # golden_spiral
    "0x1408cc680",  # legacy_raycast
    "0x1408ccc90",  # legacy_spherecast
    "0x14090e070",  # developer_hotkeys_enabled
    "0x14261ed20",  # just_pressed
    "0x140911df0",  # eye_forward
    "0x140911dd0",  # eye_position
    "0x140911230",  # camera mode
    "0x140a12650",  # cursor raycast position
    "0x14261ef90",  # curve new_for_point_count
    "0x14230eb00",  # move_ducks query init
    "0x14230ecf0",  # move_critters query init
    "0x14230e780",  # avoid_collisions / duck_ripples query init
]

code = '''
import json
prog = currentProgram
out = {}
for a in %r:
    try:
        addr = prog.getAddressFactory().getAddress(a)
        out[a + "_float"] = prog.getMemory().getFloat(addr)
        buf = bytearray(8)
        prog.getMemory().getBytes(addr, buf)
        out[a + "_hex"] = "".join("%%02x" %% (b & 0xFF) for b in buf)
    except Exception as e:
        out[a] = "ERR:" + str(e)[:80]
for a in %r:
    try:
        addr = prog.getAddressFactory().getAddress(a)
        f = prog.getFunctionManager().getFunctionAt(addr)
        out[a + "_fn"] = str(f.getName()) if f else "no-func"
    except Exception as e:
        out[a] = "ERR:" + str(e)[:80]
print(json.dumps(out))
''' % (FLOATS, CALLEES)

JOB = [
    {
        "tool": "run-script",
        "args": {
            "programPath": "/tiny-glade.exe",
            "code": code,
            "timeoutSeconds": 60,
        },
        "timeout_seconds": 180,
    }
]

with open(OUT, "w") as fh:
    json.dump(JOB, fh, indent=2)
    fh.write("\n")
print("written", OUT)
