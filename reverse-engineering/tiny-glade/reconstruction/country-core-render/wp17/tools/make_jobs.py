#!/usr/bin/env python3
"""WP17 — write Ghidra get_functions batch jobs for the replay domain."""
import json, sys, os

def batch(name, funcs, fields="signature,decompiled_code", outdir="tools/jobs"):
    os.makedirs(outdir, exist_ok=True)
    job = [{
        "tool": "get_metadata",
        "args": {"program": "tiny-glade.exe"},
        "timeout_seconds": 120,
    }, {
        "tool": "get_functions",
        "args": {
            "program": "tiny-glade.exe",
            "functions": ",".join(funcs),
            "fields": fields,
            "include_call_context": False,
        },
        "timeout_seconds": 300,
    }]
    p = os.path.join(outdir, name + ".json")
    with open(p, "w") as f:
        json.dump(job, f)
    print(p)

if __name__ == "__main__":
    batch("wp17-b1-orchestration", [
        "0x140949170",  # ReplayBuffer::len
        "0x1409f18f0",  # ReplayBuffer::pop_front
        "0x1409f1a80",  # ReplayBuffer::front_mut_bucket
        "0x1409f1ab0",  # ReplayBuffer::make_animations_instant
        "0x140af6170",  # history_process::replay_edit
        "0x140af5b30",  # history_process
        "0x140a7c550",  # replay_mode
        "0x1408b36a0",  # finish_replay
        "0x140b38250",  # replay_pop_bucket
        "0x140b1b860",  # signifier_replay_active
        "0x140b1d680",  # register_required_components
    ])
    batch("wp17-b2-domain", [
        "0x140951b80",  # replay_wall
        "0x140952570",  # _deprecated_morph_curve
        "0x140abfb30",  # WallReplayEditCmd::process
        "0x140995150",  # replay_terrain
        "0x140994f30",  # TerrainReplayCmd::forward
        "0x140995040",  # TerrainReplayCmd::backward
        "0x140aa4a60",  # replay_auto_clutter
        "0x140af75b0",  # replay_stairs
        "0x1409febf0",  # replay_raster<Path>
        "0x1401700430",  # RasterReplayCmd::forward
        "0x1401700620",  # RasterReplayCmd::backward
        "0x14016286d0",  # generate_path_replay_cmds
    ])
    batch("wp17-b3-resources", [
        "0x140a83300",  # wall_history ev_new_wall
        "0x140a83f80",  # ev_height_changed
        "0x140a83fb0",  # EditType::new_height_changed
        "0x140858d30",  # TerrainEdit::revert
        "0x1408592d0",  # TerrainEdit::reapply
        "0x140859740",  # TerrainEdit::update_terrain_curve
        "0x140858c80",  # TerrainEdit impl$0::fmt
        "0x140a82d50",  # CompoundEditBuilder::build
        "0x140a82b40",  # CompoundEditBuilder::next
        "0x140a82b20",  # CompoundEditBuilder::new_edit_sequence
        "0x140a87970",  # wall_compound_edit impl$5::fmt
        "0x140b31220",  # decorator_edit convert_before_after_states_to_changes
        "0x14017bdae0",  # decorator_edit send_state_diff_events
        "0x140a976b0",  # path_edit impl$1::from
    ])
