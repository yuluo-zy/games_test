import json
from pathlib import Path

def batch(name, addrs, fields="signature,decompiled_code"):
    job = [{
        "tool": "get_functions",
        "args": {
            "program": "tiny-glade.exe",
            "functions": ",".join(addrs),
            "fields": fields,
            "include_call_context": False,
        },
        "timeout_seconds": 300,
    }]
    Path(f"jobs_{name}.json").write_text(json.dumps(job, indent=1))
    print(name, len(addrs), fields)

def hexl(dec):
    return hex(0x140000000 + dec)

# batch 1: backend/input
batch("b1_input", [
    "0x1416feeb0",  # calculate_input_delta
    "0x1417cd0a0",  # translate_input
    "0x140b17a80",  # translate_input_to_cursor_moved
    "0x140942b00",  # reset_input_state
    "0x140941a30",  # control_input_replay
    "0x140942280",  # advance_input_replay_frame
    "0x1409423d0",  # input_record_replay
    "0x14083c620",  # record_and_replay_input_for_trailers
    "0x14083cdd0",  # do_input_replay_ui
])

# batch 2: camera_rig
batch("b2_camera", [
    "0x140a0d090",  # butterfly::update
    "0x140992f10",  # free::get_position
    "0x140993010",  # free::get_yaw_pitch
    "0x140993100",  # free::set
    "0x140993750",  # rotate_yaw_pitch (free/walk shared)
    "0x1409938b0",  # free::translate_screen_space
    "0x140993d30",  # free/walk update
    "0x140949cc0",  # normal::rotate_yaw_pitch
    "0x140949dc0",  # normal::constrain_to_be_above_terrain
    "0x140949e70",  # normal::translate_screen_space
    "0x140949fe0",  # normal::update
    "0x140993e20",  # walk::new
    "0x140994130",  # walk::teleport
    "0x140994320",  # walk::constrain_to_be_above_terrain
    "0x140994700",  # walk::translate_screen_space
])

# batch 3: main_camera + look_at_camera
batch("b3_camera", [
    "0x140911e40",  # camera_snapshot
    "0x1409125e0",  # build
    "0x140912a40",  # reset_camera
    "0x140912c20",  # main_camera_update
    "0x1409145a0",  # paused_main_camera_update
    "0x140873de0",  # look_at_camera
    "0x1408756e0",  # register_required_components (look_at)
])

# batch 4: time/weather/world
batch("b4_time", [
    "0x140a73460",  # time_of_day
    "0x140a74790",  # remap_hour_to_fmod_hour
    "0x140922c30",  # time_update
    "0x1408e5b00",  # weather_system
    "0x140a86b60",  # mode_manager
    "0x140a875e0",  # send_mode_just_changed_event
    "0x140957990",  # world_raster::record_raster_history
    "0x140957bf0",  # world_raster::upload_world_raster_buffers_to_staging
    "0x140a06d30",  # timelapse::start_timelapse
    "0x140a06e80",  # timelapse::finish_timelapse
    "0x140a06fb0",  # timelapse::cleanup_timelapse
])

# batch 5: ui systems (large -> decompiled too)
batch("b5_ui", [
    "0x14092a100",  # ui_2d_follow_mouse
    "0x1409527a0",  # ui_camera_mode_gizmos
    "0x1409771f0",  # ui_edit_focus
    "0x140977f50",  # geo_src_to_edit_subject
    "0x140978000",  # geo_src_to_edit_subject_w_deco_clutter
    "0x140ac98e0",  # ui_edit_freehand_walls
    "0x140a623e0",  # ui_focus_mode_shape_height
    "0x1409781d0",  # ui_move_freehand_walls
    "0x14097a060",  # ui_move_shapes
    "0x14097d280",  # move_gizmo_and_trigger_vol
    "0x140952e70",  # ui_raise_shape
    "0x140955650",  # impl_raise_shape
])

# batch 6: ui_clutter
batch("b6_clutter", [
    "0x14099ca60",  # ui_convert_autoclutter_to_decoclutter
    "0x1409a92c0",  # ui_focus_decoclutter
    "0x1409a1f50",  # debug_spawn_all_clutter
    "0x1412357c0",  # decoclutter_show_pins
    "0x140976ca0",  # handle_rmb_autoclutter
    "0x1409aa3e0",  # history_record_auto_clutter
])

# batch 7: misc
batch("b7_misc", [
    "0x140a92f80",  # arrow_gizmo_animation
    "0x140865050",  # brush_preview
    "0x140a7abd0",  # clear_canvas
    "0x1408b3510",  # despawn_glade_entities
    "0x140a68430",  # mesh_follow_terrain
    "0x140945ed0",  # add_stages
    "0x14175ada0",  # line_x_shape_overlap
    "0x140af9000",  # update_mod_key_state
])

# batch 8: prefab + world_raster draw_raster (water instance) + mode_manager serde signature-only
batch("b8_prefab", [
    "0x140840430",  # prefab::build
    "0x140840610",  # spawn_prefabs
    "0x140841550",  # create_prefab_rt
    "0x140841b00",  # create_prefab_gpu_instances
    "0x140841d60",  # update_prefabs
    "0x140842d60",  # despawn_prefabs
    "0x140843390",  # despawn_prefab_contents
    "0x140843740",  # hot_reload_prefabs
    "0x1408446b0",  # register_required_components (prefab)
    "0x142260ae0",  # world_raster draw_raster<WaterRaster> instance
])
batch("b8b_serde", [
    "0x1400368a0", "0x14003ac90", "0x14003b6a0",
    "0x140afbba0", "0x140afc140", "0x140afc7c0", "0x140afd8d0", "0x140afde20",
], fields="signature")
