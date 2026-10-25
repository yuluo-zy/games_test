# -*- coding: utf-8 -*-
"""Cross-check SOURCE_MAP business functions vs chapter rows for group A domains."""
import io, sys, re, os, json
sys.stdout.reconfigure(encoding='utf-8')
BASE = r'D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep'

with io.open(os.path.join(BASE, 'round3/REVIEW-UNEXPLORED/groupA_rows.json'), encoding='utf-8') as f:
    rows = json.load(f)
with io.open(os.path.join(BASE, 'SOURCE_MAP.json'), encoding='utf-8') as f:
    smap = json.load(f)

DOMAIN_FILES = {
 'history': [
   'resources/history/mod.rs','resources/history/auto_clutter_edit.rs','resources/history/color_edit.rs',
   'resources/history/decorator_edit.rs','resources/history/path_edit.rs','resources/history/replay_cmd_buffer.rs',
   'resources/history/roof_edit.rs','resources/history/stairs_history_edit.rs','resources/history/terrain_edit.rs',
   'resources/history/tree_edit.rs','resources/history/wall_compound_edit.rs','resources/history/wall_history.rs',
   'systems/history/finish_replay.rs','systems/history/history_process.rs','systems/history/history_recording.rs',
   'systems/history/mod.rs','systems/history/replay_auto_clutter.rs','systems/history/replay_garden.rs',
   'systems/history/replay_mode.rs','systems/history/replay_path.rs','systems/history/replay_pop_bucket.rs',
   'systems/history/replay_raster.rs','systems/history/replay_stairs.rs','systems/history/replay_terrain.rs',
   'systems/history/replay_wall.rs','systems/history/replay_water.rs','systems/history/signifier_replay_active.rs',
 ],
 'session-save': [
   'resources/session.rs','resources/logo_animation.rs','resources/loading_screen.rs','resources/prefab.rs',
   'systems/prefab.rs','systems/save/mod.rs','systems/save/load_completion_checker.rs','systems/save/load_game.rs',
   'systems/save/save_backup_list.rs','systems/save/save_backup_metronome.rs','systems/save/save_file_identity.rs',
   'systems/save/save_game.rs','systems/save/save_thread.rs','systems/save/shadow_file.rs','systems/save/snapshot.rs',
   'systems/save/snapshot_gc.rs',
 ],
 'camera-time': [
   'resources/daily.rs','resources/glade_settings.rs','resources/time.rs','resources/time_of_day.rs','resources/weather.rs',
   'systems/arrow_gizmo_animation.rs','systems/camera_rig.rs','systems/camera_rig/butterfly.rs',
   'systems/camera_rig/camera_body.rs','systems/camera_rig/free.rs','systems/camera_rig/normal.rs',
   'systems/camera_rig/walk.rs','systems/clear_canvas.rs','systems/despawn.rs','systems/look_at_camera.rs',
   'systems/main_camera.rs','systems/time_of_day.rs','systems/time_update.rs','systems/timelapse_replay.rs',
   'systems/ui_camera_mode_gizmos.rs','systems/weather_system.rs',
 ],
}

# chapter VA set per domain + name-based fallback matching
for dom in ['history', 'session-save', 'camera-time']:
    va_set = set()
    for r in rows[dom]:
        a = r['addr'].strip()
        if a.startswith('0x'):
            va_set.add(a.lower())
    missing = []
    no_va = []
    for f in DOMAIN_FILES[dom]:
        ent = smap.get(f)
        if not ent:
            continue
        for fn in ent.get('functions', []):
            if fn.get('kind') != 'business':
                continue
            name = fn.get('name')
            vas = [v.lower() for v in fn.get('va') or []]
            if not vas:
                no_va.append((f, name, fn.get('line')))
                continue
            uncovered = [v for v in vas if v not in va_set]
            if uncovered:
                missing.append((f, name, vas, uncovered, fn.get('line')))
    print('='*20, dom)
    print('-- business with VA but NO chapter row: %d' % len(missing))
    for f, name, vas, unc, line in missing:
        print('   %s | %s | va=%s | uncovered=%s | line=%s' % (f, name[:100], vas, unc, line))
    print('-- business with no VA (inline): %d' % len(no_va))
    for f, name, line in no_va:
        print('   %s | %s | line=%s' % (f, name[:110], line))
