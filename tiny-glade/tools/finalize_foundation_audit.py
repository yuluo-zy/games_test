"""Refresh current hashes and record selected small-system CPU body projections."""
from pathlib import Path
import hashlib
import json
import subprocess

ROOT = Path(__file__).resolve().parents[2]
PROJECT = ROOT / 'tiny-glade'
RECON = 'reverse-engineering/tiny-glade/reconstruction/'

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

audit_path = PROJECT / 'provenance/foundation-audit.json'
audit = json.loads(audit_path.read_text(encoding='utf-8'))
source_paths = sorted({f['source_path'] for f in audit['files']})
git_status = subprocess.check_output(['git', 'status', '--porcelain', '--', *source_paths], cwd=ROOT, text=True)
dirty = {line[3:].replace('\\', '/') for line in git_status.splitlines()}
for record in audit['files']:
    destination = PROJECT / record['path']
    current = sha(destination)
    if current != record['sha256']:
        record['transformations'].append('rustfmt, local GFN fixture path, Windows fixture write-access, or MSRV-compatible RGB chunk iteration; see integration fixes')
    record['sha256'] = current
    record['source_worktree_modified'] = record['source_path'] in dirty
audit['source_git_status'] = git_status
audit['integration_fixes'] = [
    'Localize Rust oracle includes, their child JSON/Rust fixtures and GFN C/build fixtures.',
    'Physically remove speculative IndexCondenser insert, Vec3SliceView2 projection and 2D global spline queries plus their hypothesis tests/feature. Preserve all verified bodies; original research tree is unchanged.',
    'Select committed geometry/native_tests.rs SHA497528a3 matching accepted numeric-review input. Preserve unselected unverified worktree hash; do not change expected values or tolerances.',
    'Windows asset-pipe mtime preservation test requests write access on its own fixture file; production algorithm unchanged.',
    'tg-text RGB loop uses chunks_exact(3), equivalent to as_chunks::<3>().0, to retain workspace MSRV.',
    'GUI native-gui remains opt-in. Original fork dependencies are used when enabled.',
    'Retain latest paths/lang-lib/tabloid owned product repairs; native evidence does not cover changed policies.',
    'Borrow owned Arc<str> localization values in integrity/crash native-GUI replacement strings.',
    'With formal MSRV 1.88, collapse equivalent conditional chains, use integer is_multiple_of, and borrow a one-sprite GPU example slice instead of redundant Arc clones.',
]
audit['validation'] = {
    'platform': 'Windows x86_64',
    'foundation_packages': 18,
    'default_test_status': 'All package targets passed across full suite and affected-target reruns after integration fixes.',
    'utils_default_and_all_features': {'passed': 87, 'failed': 0, 'ignored': 0},
    'utils_post_pruning_all_targets_all_features': {'check_exit_code': 0, 'test_passed': 87, 'test_failed': 0, 'clippy_exit_code': 0, 'new_execution_after_user_stop': False},
    'utils_historical_experimental_before_removal': {'passed': 96, 'original_acceptance_upgraded': False, 'current_package_feature': False},
    'strict_all_targets_clippy': 'passed for all 18 default-feature packages',
    'all_targets_all_features_check': 'passed for all 18 packages, including real native-gui and tg-text gpu sources',
    'strict_all_features_clippy': '17 foundation packages passed including native-gui; tg-text passed with --no-deps (rhapsody/shader dependencies linted by their owner and root matrix)',
    'gfn_default_ignored': 1,
    'gfn_native_fixture': {'test_passed': 1, 'fixture_combinations': 14, 'real_nvidia_sdk': False},
    'readme_links': 'passed',
    'complete_original_modules_accepted': False,
}
audit_path.write_text(json.dumps(audit, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')

selected = [
    {
        'module': 'tree',
        'original_owner': 'system-tree/process_tree_cmds.rs:despawn_trees',
        'source_path': RECON + 'standalone/tg-tree/src/despawn.rs',
        'functions': ['DESPAWN_THRESHOLD', 'despawn_trees resolved deletion predicate'],
        'evidence': 'evidence/multi-baseline/tree-tool-rest-analysis.md section 1; B 0x1422686d0',
        'transformations': ['Project Bevy Query/EventWriter into should_despawn_tree(marker, animation_value); no lifecycle/event payload ABI assumed.'],
        'input_contract': 'Animation component/query presence is resolved by caller. Strict ordered <0.01 preserves NaN rejection.',
    },
    {
        'module': 'water',
        'original_owner': 'system-water/water_audio_grid.rs + country-core/world_raster rasterize_stroke callbacks',
        'source_path': RECON + 'standalone/tg-critters-water/src/water/audio_grid.rs',
        'additional_source_paths': [RECON + 'standalone/tg-critters-water/src/water/stroke.rs'],
        'functions': ['frame_bit', 'apply_bit', 'select_write_mode', 'coverage_to_alpha', 'brush_fixed', 'apply_write'],
        'evidence': 'evidence/multi-baseline/water-extra-analysis.md sections 2 and 5; B 0x141baefe0, 0x142262221..0x1422622c8, 0x142828b30/0x142821d90/0x142823e50',
        'transformations': ['Flatten mask/counters from WaterAudioGridCell into explicit scalar inputs; present supplied by real caller.', 'coverage becomes unsigned u32: no guessed negative clamp.', 'Illegal mode is reported as None by pure projection; original entry panics.'],
        'input_contract': 'bit in 0..64 from frame_bit; nonnegative coverage; finite brush; weight in 0..=65536. No guessed coverage scale, grid/world distance or raycast producer.',
    },
    {
        'module': 'critters',
        'original_owner': 'system-critters raw CritterLocomotion collision branch predicates',
        'source_path': RECON + 'standalone/tg-critters-water/src/critters/components.rs',
        'functions': ['samples_rock_mask', 'skips_water_sample', 'skips_public_walls', 'wall_hash_only'],
        'evidence': 'evidence/multi-baseline/critters-extra-analysis.md section 6',
        'transformations': ['Only raw u32 input/predicates selected; remove Bevy component/bundle and speculative species/default producers.'],
        'input_contract': 'raw tag is preserved; no eight-byte enum layout or species mapping asserted.',
    },
    {
        'module': 'audio',
        'original_owner': 'country-core/systems/audio wall_audio_fmod + music_audio_system scalar leaves',
        'source_path': RECON + 'clutter-audio/crates/tg-clutter-audio/src/audio/wall_kinds.rs',
        'additional_source_paths': [RECON + 'clutter-audio/crates/tg-clutter-audio/src/audio/music.rs'],
        'functions': ['WallKind', 'WALL_KIND_TABLE', 'wall_kind', 'snap_off_boundaries', 'compute_new_glade_time'],
        'evidence': 'evidence/multi-baseline/audio-extra-analysis.md sections 1 and 4; RNE jump table 0x142ab4998 + MusicRichness/NewGladeTime constants',
        'transformations': ['Select only enum/table/getter before placeholder FMOD helpers.', 'NewGladeTime state+0x1c8 projected into mutable f32 accumulator; name advance_new_glade_time.'],
        'input_contract': 'Wall tags retain original kind24+ ignore guard; music finite scalar inputs only. No MusicPlayer/lerp stub or event backend imported.',
    },
    {
        'module': 'clutter',
        'original_owner': 'country-core/resources/history/auto_clutter_edit.rs',
        'source_path': RECON + 'clutter-audio/crates/tg-clutter-audio/src/clutter/auto_clutter_edit.rs',
        'functions': ['AutoClutterEdit::Delete externally tagged serde data contract'],
        'evidence': 'evidence/multi-baseline/clutter-extra-analysis.md; B serialize 0x14003d300/0x140a18320, deserialize 0x140a18500',
        'transformations': ['Select Delete{id:u64} schema only; serde derived, not counted as hand-written decoder.'],
        'input_contract': 'Observed wire key/value types only; no history/picking/replay implementation inferred.',
    },
]
files = []
for item in selected:
    destination = f'crates/country-core/src/recovered_systems/{item["module"]}.rs'
    item['formal_path'] = destination
    sources = [item['source_path']] + item.get('additional_source_paths', [])
    item['sources'] = []
    for source in sources:
        source_file = ROOT / source
        commit = subprocess.check_output(['git', 'log', '-1', '--format=%H', '--', source], cwd=ROOT, text=True).strip()
        item['sources'].append({'path': source, 'sha256': sha(source_file), 'source_commit': commit})
    files.append({'path': destination, 'sha256': sha(PROJECT / destination), 'source_path': item['source_path'],
                  'additional_source_paths': item.get('additional_source_paths', []), 'source_sha256': item['sources'][0]['sha256'],
                  'source_commit': item['sources'][0]['source_commit'], 'kind': 'recovered_cpu_projection',
                  'selected_functions': item['functions'], 'transformations': item['transformations']})
for name in ['mod.rs', 'README.md']:
    path = 'crates/country-core/src/recovered_systems/' + name
    files.append({'path': path, 'sha256': sha(PROJECT / path), 'kind': 'integration_glue'})
small = {
    'date': '2026-10-14',
    'scope': 'Function-level audit and migration of completed CPU leaves from standalone/clutter-audio drafts; no whole draft/plugin imported.',
    'selected': selected,
    'files': files,
    'excluded': [
        {'source': RECON + 'standalone/tg-tree/src/storage.rs:insert', 'reason': 'saturating_add overflow policy lacks original boundary evidence; raw entry/generation policy and reset hash keys are not fully recovered.'},
        {'source': RECON + 'standalone/tg-tree/src/{process_cmds,plugin,replay,convert,recalc,bend}.rs', 'reason': 'Unknown prefab/event payloads, initial animation, +65 bounds, bend constants/rounding, replay divisors, conversion gates and guessed scheduler edges. No whole-system adoption.'},
        {'source': RECON + 'standalone/tg-critters-water/src/water/{raster,tool,replay,plugin}.rs + stroke::coverage_at/rasterize_stroke + audio_grid::update_grid', 'reason': 'Unknown initial raster, half-cell origin, coverage scale/width, zeno replaced with capsule AA, producer/layout/selector/occlusion/distance and replay easing assumptions.'},
        {'source': RECON + 'standalone/tg-critters-water/src/critters/{boids,duck,sheep,sheep_terrain,wren,fish,butterfly,firefly,shared,plugin}.rs', 'reason': 'UNKNOWN sampler/axis/trajectory/state/timer/producer branches and guessed schedules. Whole modules and demo tests are not original acceptance. Firefly original-instruction recovery is audited separately by parent.'},
        {'source': RECON + 'standalone/tg-audio/src/{backend,events,plugin,music,update,wall_system,stairs,menu,sheep,clutter,building}.rs', 'reason': 'Guessed/empty event payloads, FMOD recording adapter, identity music lerp, unimplemented MusicPlayer, inferred event prefixes and unknown parameter/input/listener/scheduling fields.'},
        {'source': RECON + 'standalone/tg-audio/src/math.rs:inverse_lerp_clamped', 'reason': 'Common scalar arithmetic already available via formal utils; do not duplicate dependency/math helper solely for the audio draft.'},
        {'source': RECON + 'clutter-audio/crates/tg-clutter-audio/src/audio/music.rs:compute_music_richness/music_audio_system_step', 'reason': 'compute_music_richness explicitly uses smoothed=target placeholder; FMOD/player operations are comments. Only separate boundary/NewGladeTime leaves selected.'},
        {'source': RECON + 'clutter-audio/crates/tg-clutter-audio/src/audio/wall_kinds.rs:play_wall_kind/wall_frame_tail/wall_event_position', 'reason': 'FMOD helpers are no-op comments, wall_event_position returns fabricated ZERO. Pure table/getter selected without these bodies.'},
        {'source': RECON + 'clutter-audio/crates/tg-clutter-audio/src/clutter/auto_clutter_edit.rs:History/replay_auto_clutter/UI', 'reason': 'History::add and start_input_recording empty; flag!=0 replay branch calls unimplemented erase_decorator_pair; picking/UI path has unknown(). Schema only selected.'},
        {'source': RECON + 'clutter-audio/crates/tg-clutter-audio/src/{utils,tabloid,glue}.rs', 'reason': 'Public/library substitutes and game-type stubs. Use formal utils/tabloid and actual verified consumer contracts instead.'},
    ],
    'validation': {'native_equivalence_newly_run': False, 'test_contracts': 'Existing source cases projected into pure APIs: tree threshold/NaN, raw predicate bits, PADDW/PSUBW, fixed callbacks, RNE wall-table cases, music bounds, serde wire string.', 'country_core_cpu_projection_tests': {'passed': 10, 'failed': 0, 'ignored': 0}, 'complete_systems_accepted': False},
}
(PROJECT / 'provenance/small-system-audit.json').write_text(json.dumps(small, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
print(f'Refreshed {len(audit["files"])} foundation records; wrote {len(files)} small-system file records')
