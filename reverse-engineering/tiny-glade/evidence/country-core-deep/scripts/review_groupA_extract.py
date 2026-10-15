# -*- coding: utf-8 -*-
"""Extract chapter rows for group A domains (history/session-save/camera-time)."""
import io, sys, re, os, json
from collections import Counter
sys.stdout.reconfigure(encoding='utf-8')
BASE = r'D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters'

HISTORY_FILES = [os.path.normpath(os.path.join(BASE, 'resources/history', f)) for f in
    ['mod.rs.md','auto_clutter_edit.rs.md','color_edit.rs.md','decorator_edit.rs.md','path_edit.rs.md',
     'replay_cmd_buffer.rs.md','roof_edit.rs.md','stairs_history_edit.rs.md','terrain_edit.rs.md',
     'tree_edit.rs.md','wall_compound_edit.rs.md','wall_history.rs.md']]
HISTORY_FILES += [os.path.normpath(os.path.join(BASE, 'systems/history', f)) for f in
    ['finish_replay.rs.md','history_process.rs.md','history_recording.rs.md','mod.rs.md',
     'replay_auto_clutter.rs.md','replay_garden.rs.md','replay_mode.rs.md','replay_path.rs.md',
     'replay_pop_bucket.rs.md','replay_raster.rs.md','replay_stairs.rs.md','replay_terrain.rs.md',
     'replay_wall.rs.md','replay_water.rs.md','signifier_replay_active.rs.md']]

SESSION_FILES = [os.path.normpath(os.path.join(BASE, 'resources', f)) for f in
    ['session.rs.md','logo_animation.rs.md','loading_screen.rs.md','prefab.rs.md']]
SESSION_FILES += [os.path.normpath(os.path.join(BASE, 'systems', 'save', f)) for f in
    ['load_completion_checker.rs.md','load_game.rs.md','mod.rs.md','save_backup_list.rs.md',
     'save_backup_metronome.rs.md','save_file_identity.rs.md','save_game.rs.md','save_thread.rs.md',
     'shadow_file.rs.md','snapshot.rs.md','snapshot_gc.rs.md']]
SESSION_FILES.append(os.path.normpath(os.path.join(BASE, 'systems', 'prefab.rs.md')))

CAMERA_FILES = [os.path.normpath(os.path.join(BASE, 'resources', f)) for f in
    ['time_of_day.rs.md','daily.rs.md','glade_settings.rs.md','time.rs.md','weather.rs.md']]
CAMERA_FILES += [os.path.normpath(os.path.join(BASE, 'systems', 'camera_rig', f)) for f in
    ['butterfly.rs.md','camera_body.rs.md','free.rs.md','normal.rs.md','walk.rs.md']]
CAMERA_FILES += [os.path.normpath(os.path.join(BASE, 'systems', f)) for f in
    ['camera_rig.rs.md','main_camera.rs.md','look_at_camera.rs.md','time_of_day.rs.md',
     'time_update.rs.md','weather_system.rs.md','timelapse_replay.rs.md','ui_camera_mode_gizmos.rs.md',
     'arrow_gizmo_animation.rs.md','clear_canvas.rs.md','despawn.rs.md']]

domains = {'history': HISTORY_FILES, 'session-save': SESSION_FILES, 'camera-time': CAMERA_FILES}

row_re = re.compile(r"^\|\s*`{1,2}([^`]+)`{1,2}(?:[^|]*?)\|\s*(0x[0-9a-fA-F]+|\(数据型\)|[^|]*?)\s*\|\s*(\d+)?\s*\|\s*([^|]*?)\s*\|\s*([^|]*?)\s*\|\s*([^|]*?)\s*\|")
concl_re = re.compile(r"^\s*([OIU])(?:[：:\s(（\-—|]|$)")

out = {}
for dom, files in domains.items():
    rows = []
    for fp in files:
        if not os.path.exists(fp):
            print('MISSING FILE:', fp); continue
        with io.open(fp, encoding='utf-8', errors='replace') as f:
            lines = f.readlines()
        for ln, line in enumerate(lines, 1):
            m = row_re.match(line.strip())
            if not m:
                continue
            fn, addr, src_line, role, concl_raw, evid = m.groups()
            c = concl_raw.strip()
            cm = concl_re.match(c)
            concl = cm.group(1) if cm else '?'
            rel = os.path.relpath(fp, BASE).replace(os.sep, '/')
            rows.append({'file': rel, 'line': ln, 'fn': fn,
                         'addr': addr.strip(), 'src_line': (src_line or '').strip(), 'role': role.strip(),
                         'concl': concl, 'concl_raw': c, 'evid': evid.strip()})
    out[dom] = rows
    print('== %s: %d rows' % (dom, len(rows)), Counter(r['concl'] for r in rows))
    for r in rows:
        if r['concl'] == '?':
            print('   UNCLASS:', r['file'], r['line'], r['concl_raw'][:60])

with io.open(r'D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/round3/REVIEW-UNEXPLORED/groupA_rows.json', 'w', encoding='utf-8') as f:
    json.dump(out, f, ensure_ascii=False, indent=1)
print('saved')
