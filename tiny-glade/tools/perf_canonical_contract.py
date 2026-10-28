"""Reviewed prospective canonical fixture contract; cold source/evidence checks only.

Original build metadata and measurement/runtime policy remain unchanged.
The raw preparation stamp is not proof of final/all-frame layout or GPU pixels.
"""
import base64
import hashlib
import json
import re
from pathlib import Path

HARNESS = 'common-cpu-workload-canonical-v2'
CONTRACT = 'canonical-common-input-closure-v2'
PREPARATION = 'canonical-brick-layout-v1'
LEGACY_HARNESS = 'common-cpu-workload-v1'
COMMON4 = {'crates/tiny-glade/tests/cpu_measurement/mod.rs': 'd2d1e71eeb425cbf5119c17bad6de718c7cd0fa699540bee1aa82e278ce4813b', 'crates/tiny-glade/tests/cpu_measurement/workload_probe.rs': 'bb357cf46e3a51e5e921f454ed4b9d41c935524a8409af76eef4ff5c970d9ee8', 'crates/tiny-glade/tests/cpu_measurement/gpu_diagnostic.rs': 'f30f54453e05ffe9b98bc147a13ae6da5b863f36aed4980b1f0a0ecc74431fa7', 'crates/tiny-glade/tests/support/performance_sparse_layout.rs': '5ccf99448777fdee307a417c1dcf43d12b0f5551938d3f783eace9a21ff14ff9'}
SHARED = {'crates/tiny-glade/tests/cpu_measurement/mod.rs': 'd2d1e71eeb425cbf5119c17bad6de718c7cd0fa699540bee1aa82e278ce4813b', 'crates/tiny-glade/tests/cpu_measurement/workload_probe.rs': 'bb357cf46e3a51e5e921f454ed4b9d41c935524a8409af76eef4ff5c970d9ee8', 'crates/tiny-glade/tests/cpu_measurement/gpu_diagnostic.rs': 'f30f54453e05ffe9b98bc147a13ae6da5b863f36aed4980b1f0a0ecc74431fa7', 'crates/tiny-glade/tests/support/performance_sparse_layout.rs': '5ccf99448777fdee307a417c1dcf43d12b0f5551938d3f783eace9a21ff14ff9', 'crates/tiny-glade/tests/support/performance_input.rs': 'ba6c70d1ce83bd1ff664f8295db05822d796e7cc55f0ecea69826376670faa4d', 'crates/tiny-glade/tests/support/performance_workload.rs': '3890c544d3f79c2d8a542acf064d70ef66cdae351a5b33d297d6bd2b6359eae4', 'crates/tiny-glade/tests/support/mod.rs': 'd4d2b5130c6375d839fce04421e252dda357fcada838dfcde21b0aba51c3c554', 'crates/tiny-glade/tests/support/upload_wait.rs': 'd5263fb610c19d996e8038bc3b41b5bdbe3ac9dc7b0ae29fd3c87c6a43ed825e'}
GETTERS = {'crates/country-core/src/render/sparse_instance_buffer.rs': ['pub struct SparseFixtureReferences {\n    pub cached_draw_snapshot: bool,\n    pub draw_snapshot_strong: usize,\n    pub draw_snapshot_weak: usize,\n    pub owned_buffer_present: bool,\n    pub owned_buffer_strong: usize,\n    pub owned_buffer_weak: usize,\n}', '    pub fn fixture_reference_state(&self) -> SparseFixtureReferences {\n        let cache = self.draw_snapshot.lock();\n        SparseFixtureReferences {\n            cached_draw_snapshot: cache.is_some(),\n            draw_snapshot_strong: cache.as_ref().map_or(0, Arc::strong_count),\n            draw_snapshot_weak: cache.as_ref().map_or(0, Arc::weak_count),\n            owned_buffer_present: self._buffer_owner.is_some(),\n            owned_buffer_strong: self._buffer_owner.as_ref().map_or(0, Arc::strong_count),\n            owned_buffer_weak: self._buffer_owner.as_ref().map_or(0, Arc::weak_count),\n        }\n    }'], 'crates/rhapsody/src/transfer/queue.rs': ['    pub(super) fn fixture_weak_history_count(&self, target: &Arc<T>) -> usize {\n        self.epochs.iter().filter(|entry| entry.target.as_ptr() == Arc::as_ptr(target)).count()\n    }'], 'crates/rhapsody/src/transfer.rs': ['    pub fn fixture_buffer_weak_history_count(&self, target: &Arc<Buffer>) -> usize {\n        self.queued.fixture_weak_history_count(target)\n    }'], 'crates/tiny-glade/src/render_upload_boundary.rs': ['    pub(crate) fn fixture_has_active_reader(&self) -> bool { self.owner.lock().is_some() }'], 'crates/tiny-glade/src/render_host.rs': ['pub struct BufferFixtureReferences {\n    pub frame_bound: bool,\n    pub active_read_lease: bool,\n    pub version_strong: usize,\n    pub version_weak: usize,\n    pub snapshot_pins: usize,\n    pub physical_strong: usize,\n    pub physical_weak: usize,\n    pub known_queue_history_weak: usize,\n    pub known_cow_retired_weak: usize,\n}', '    pub fn fixture_reference_state(&self, target: &Buffer) -> Result<BufferFixtureReferences, BackendError> {\n        let _serial = self.publication.lock();\n        let version = self.current_version(target).ok_or_else(|| BackendError::invalid_usage("fixture target is stale/foreign"))?;\n        let raw = version.buffer.shared();\n        let known_queue_history_weak = self.uploads.lock().as_ref().map_or(0,\n            |service| service.manager.fixture_buffer_weak_history_count(&raw));\n        let known_cow_retired_weak = self.cow_budget.lock().retired.iter()\n            .filter(|entry| entry.owner.as_ptr() == Arc::as_ptr(&raw)).count();\n        Ok(BufferFixtureReferences {\n            frame_bound: self.frame.is_some(),\n            active_read_lease: self.boundary().fixture_has_active_reader(),\n            version_strong: Arc::strong_count(&version) - 1,\n            version_weak: Arc::weak_count(&version),\n            snapshot_pins: version.pins.load(Ordering::Acquire),\n            physical_strong: Arc::strong_count(&raw) - 1,\n            physical_weak: Arc::weak_count(&raw),\n            known_queue_history_weak,\n            known_cow_retired_weak,\n        })\n    }']}
RICH_CALLER_SHA256 = '2d7dd64b1a36d07139d7ebe098b214e8cf1b3cc6ea9f673becf40fd5d79e23f8'

BOOT_PATH = "crates/tiny-glade/tests/original_gameplay_chain.rs"
BOOT = 'fn boot_shipped_session() -> Result<BootedSession> {\n    struct Diagnostics;\n    impl log::Log for Diagnostics {\n        fn enabled(&self, metadata: &log::Metadata<\'_>) -> bool { metadata.level() <= log::Level::Error }\n        fn log(&self, record: &log::Record<\'_>) {\n            if self.enabled(record.metadata()) { eprintln!("{} {}", record.target(), record.args()); }\n        }\n        fn flush(&self) {}\n    }\n    static DIAGNOSTICS: Diagnostics = Diagnostics;\n    if log::set_logger(&DIAGNOSTICS).is_ok() { log::set_max_level(log::LevelFilter::Error); }\n\n    let installed = std::env::var_os("TG_INSTALL_ROOT")\n        .map(PathBuf::from)\n        .unwrap_or_else(|| {\n            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../ljxsj_92385/Tiny Glade")\n        })\n        .canonicalize()?;\n    // Both shipped-session tests boot a whole App in this process: the account paths are process\n    // global, so they are initialised once and the boots are serialised.\n    static BOOT: std::sync::Mutex<()> = std::sync::Mutex::new(());\n    static ACCOUNT: std::sync::OnceLock<tempfile::TempDir> = std::sync::OnceLock::new();\n    let boot_guard = BOOT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);\n    if ACCOUNT.get().is_none() {\n        let account = tempfile::tempdir()?;\n        paths::initialize_paths_with_installation(0, Some(account.path()), &installed)?;\n        let _ = ACCOUNT.set(account);\n    }\n    std::env::set_current_dir(&installed)?;\n    utils::local_time::init_local_time();\n    tiny_glade::load_language_library::load_language_library(None, None);\n    let cli = CliArgs::parse_from(["original-gameplay-chain"]);\n    let config = get_app_config(&cli)?;\n    assert!(\n        !config.dev.demo,\n        "this acceptance requires the full original build"\n    );\n    let mut app = run_game_thread(config, &cli, {\n        // 只用于出高分辨率验证图：TG_TEST_SIZE=宽x高，默认 480x270。\n        let (w, h) = std::env::var("TG_TEST_SIZE")\n            .ok()\n            .and_then(|v| v.split_once(\'x\').and_then(|(w, h)| Some((w.parse().ok()?, h.parse().ok()?))))\n            .unwrap_or((480, 270));\n        WindowSize::new(w, h)\n    })?;\n    app.insert_resource(\n        country_core::resources::platform::RuntimePlatform::from_detection(false, false),\n    );\n    tiny_glade::game_loop::register_platform_events(&mut app);\n    tiny_glade::game_loop::register_settings_application_system(&mut app);\n    let device =\n        rhapsody::device::Device::new_headless(&rhapsody::device::DeviceConfig {\n            // Same budget as the production window path (the original allocator has no cap).\n            memory_budget_bytes: tiny_glade::production_renderer::UNBOUNDED_DEVICE_BUDGET_BYTES,\n            ..rhapsody::device::DeviceConfig::headless()\n        })\n        .context("strict gameplay physical device")?;\n    let shaders = app.world().resource::<ApplicationShaderLibrary>().0.clone();\n    let bridge = CoreRenderBridge::new(device.clone(), installed.clone(), shaders.clone())?;\n    let mut final_post = None;\n    country_core::startup::load_shaders::load_shaders(\n        &shaders,\n        bridge.shader_backend(),\n        &mut final_post,\n    );\n    // Same shader set as ProductionRenderer::new: the world tail (ui_2d, text, vermeer,\n    // painterly, blit) is part of the compiled production graph.\n    country_core::startup::load_shaders::load_ui_shaders(&shaders, bridge.shader_backend());\n    country_core::startup::startup_blit::startup_blit(&shaders, bridge.shader_backend());\n    struct TailShaders<\'a> {\n        library: &\'a country_core::asset_libraries::shader_library::AssetShaderLibrary,\n        backend: &\'a dyn country_core::render::shader::boundary::ShaderBackend,\n    }\n    impl country_core::systems::render_loop::painterly::ShaderLoader for TailShaders<\'_> {\n        fn load_shader_into_library(&mut self, path: &\'static str, name: &\'static str) {\n            country_core::startup::load_shaders::load_shader_into_library(\n                self.library,\n                self.backend,\n                path,\n                name,\n            );\n        }\n    }\n    country_core::systems::render_loop::painterly::startup_painterly(&mut TailShaders {\n        library: &shaders,\n        backend: bridge.shader_backend(),\n    });\n    country_core::systems::render_loop::vermeer::startup_vermeer(&mut TailShaders {\n        library: &shaders,\n        backend: bridge.shader_backend(),\n    });\n    bridge.install_asset_libraries(app.world_mut())?;\n    tiny_glade::gameplay_startup::install_gameplay_device_runtime(\n        &mut app,\n        device.clone(),\n        bridge.hosts().clone(),\n        installed,\n    )?;\n    // The window path uses this driver to install the same asset maintenance\n    // and canonical egui Context before its first App::update.\n    let context = egui::Context::default();\n    let mut driver = tiny_glade::game_loop::GameFrameDriver::new(app, context.clone());\n    driver\n        .app_mut()\n        .insert_resource(bevy_time::TimeUpdateStrategy::ManualDuration(\n            Duration::from_secs_f64(1.0 / 60.0),\n        ));\n    let mut loaded = false;\n    for frame in 0..600 {\n        context.begin_frame(egui::RawInput::default());\n        driver.app_mut().update();\n        let _ = context.end_frame();\n        let world = driver.app().world();\n        if frame % 60 == 0 {\n            eprintln!(\n                "gameplay App frame {frame}: session={}, walls={}",\n                world.contains_resource::<ActiveSession>(),\n                world.resource::<InnerWalls>().wall_ids().count()\n            );\n        }\n        if world.contains_resource::<ActiveSession>()\n            && world.resource::<LoadGameManager>().options.is_none()\n            && world.resource::<InnerWalls>().wall_ids().count() > 0\n        {\n            loaded = true;\n            break;\n        }\n    }\n    anyhow::ensure!(\n        loaded,\n        "shipped intro did not finish through the real gameplay schedule"\n    );\n    // Let actual asynchronous assets, constructor uploads and intro migration\n    // settle through their normal schedules before asserting visible owners.\n    for _ in 0..90 {\n        context.begin_frame(egui::RawInput::default());\n        driver.app_mut().update();\n        let _ = context.end_frame();\n    }\n    Ok(BootedSession { driver, context, bridge, shaders, device, _boot: boot_guard })\n}'

REQUIRED = tuple(dict.fromkeys((*SHARED, *GETTERS, BOOT_PATH)))
OBLIGATIONS = (
    'fresh_boot_before_first_gameplay_warmup_checked',
    'known_owners_complete_unknown_owners_refused_checked',
    'chunk_segments_full_tail_bytes_visibility_preserved_checked',
    'escaped_offsets_source_audit_and_retained_arc_weak_checked',
    'once_attempt_preflight_before_mutation_failure_not_retried_checked',
    'readonly_getters_no_default_algorithm_or_hot_path_change_checked',
    'transitive_fixture_dependencies_reviewed_checked',
)


def require(ok, message):
    if not ok:
        raise ValueError(message)


def signature(value):
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(',', ':'),
        ensure_ascii=False, allow_nan=False).encode()).hexdigest()


def bytes_sha(raw):
    return hashlib.sha256(raw).hexdigest()


def _source_contract(source_files, raw_files, shared, harness, schema):
    """Check known input closure and own getter files, never infer a missing version.

    All full bytes are bound to source.json. The original test file may contain
    different unrelated tests; its exact audited boot body must still agree.
    These literal checks are narrow reviewed capability checks, not a Rust parser.
    """
    require(set(raw_files) == set(REQUIRED), 'canonical source archive closure incomplete/extra')
    require(all(source_files.get(name) == expected for name, expected in shared.items()),
            'canonical shared input source bytes differ; explicit new contract required')
    texts = {}
    for name in REQUIRED:
        raw = raw_files[name]
        require(type(raw) is bytes and bytes_sha(raw) == source_files.get(name),
                'canonical archived source differs from frozen manifest: '+name)
        texts[name] = raw.decode('utf-8').replace('\r\n', '\n')
    for name, blocks in GETTERS.items():
        require(all(texts[name].count(part) == 1 for part in blocks),
                'unreviewed/missing canonical readonly getter body: '+name)
    require(texts[BOOT_PATH].count(BOOT) == 1, 'canonical fresh-boot body differs from source review')
    common = {name: shared[name] for name in COMMON4}
    return {'schema': schema, 'harness': harness, 'preparation_schema': PREPARATION,
        'common_source_sha256': common, 'shared_input_closure_sha256': dict(shared),
        'own_getter_source_sha256': {name: source_files[name] for name in GETTERS},
        'readonly_getter_body_sha256': signature(GETTERS),
        'own_boot_source_sha256': source_files[BOOT_PATH], 'boot_body_sha256': bytes_sha(BOOT.encode()),
        'reviewed_source_sha256': {name: source_files[name] for name in REQUIRED}}


def source_contract(source_files, raw_files):
    return _source_contract(source_files, raw_files, SHARED, HARNESS, CONTRACT)


def rich_source_contract(source_files, raw_files):
    """Separate optional-rich diagnosis; never a formal common-v2 capability.

    The frozen rich caller explicitly opts in at runtime. Its raw report still
    must prove that opt-in; source capability by itself does not establish it.
    """
    shared = dict(SHARED)
    shared['crates/tiny-glade/tests/cpu_measurement/mod.rs'] = RICH_CALLER_SHA256
    return _source_contract(source_files, raw_files, shared, None,
                            'rich-optional-canonical-source-diagnostic-v1')


def archive(root, source):
    raw = {name: (Path(root)/name).read_bytes() for name in REQUIRED}
    contract = source_contract(source['files'], raw)
    return {'schema': 'canonical-source-archive-v2', 'source_commit': source['commit'],
        'source_content_sha256': source['content_sha256'], 'contract': contract,
        'files': {name: {'sha256': bytes_sha(value), 'raw_base64': base64.b64encode(value).decode('ascii')}
                  for name, value in raw.items()}}


def validate_archive(value, source):
    require(value.get('schema') == 'canonical-source-archive-v2'
            and value['source_commit'] == source['commit']
            and value['source_content_sha256'] == source['content_sha256'],
            'canonical archive source identity differs')
    require(set(value['files']) == set(REQUIRED), 'canonical source archive closure incomplete/extra')
    raw = {}
    for name, ref in value['files'].items():
        try:
            raw[name] = base64.b64decode(ref['raw_base64'], validate=True)
        except (ValueError, TypeError) as error:
            raise ValueError('invalid canonical source archive encoding') from error
        require(bytes_sha(raw[name]) == ref['sha256'], 'canonical archive bytes changed: '+name)
    contract = source_contract(source['files'], raw)
    require(value['contract'] == contract, 'canonical archive contract differs')
    return contract


def capability_contract(cap, review, source_files):
    files = cap['source_files']
    require(all(name in files for name in REQUIRED), 'canonical reviewed source closure incomplete')
    raw = {name: Path(files[name]['path']).read_bytes() for name in REQUIRED}
    contract = source_contract(source_files, raw)
    require(cap.get('canonical_source_contract') == review.get('canonical_source_contract') == contract,
            'canonical capability/source review contract differs')
    require(all(review.get(name) is True for name in OBLIGATIONS),
            'canonical fresh-boot/owner/reference/transitive source review obligations incomplete')
    return contract


def session_preparation(session):
    require(session.get('harness') == HARNESS and
            session.get('canonical_layout_input_policy') == PREPARATION,
            'missing canonical runtime source policy; historical runs cannot be upgraded')
    return preparation_stamp(session.get('canonical_brick_layout'),
                             session.get('input_before'), session.get('input_after'))


def preparation_stamp(prep, input_before, input_after):
    """Shared raw-stamp check only; caller chooses formal/diagnostic policy."""
    require(isinstance(prep, dict) and prep.get('schema') == PREPARATION
            and prep.get('phase') == 'fresh_boot_before_first_gameplay_warmup',
            'missing canonical preparation stamp')
    for name in ('owner_count', 'chunk_count', 'used_records'):
        require(type(prep.get(name)) is int and 0 < prep[name] <= (1 << 64)-1,
                'invalid canonical preparation count: '+name)
    # Empty live owner allocations are legitimate; owner_count need not be
    # bounded by occupied chunks. The report does not count nonempty owners.
    require(prep['chunk_count'] <= 0x10000 and prep['used_records'] <= 0x200000
            and prep['used_records'] <= prep['chunk_count']*32,
            'invalid canonical typed chunk/record capacities')
    for name in ('owner_record_bytes_blake3', 'owner_full_chunk_bytes_blake3',
                 'physical_layout_blake3', 'free_stack_blake3'):
        require(type(prep.get(name)) is str and re.fullmatch('[0-9a-f]{64}', prep[name]),
                'invalid canonical declared BLAKE3 encoding: '+name)
    require(type(prep.get('raw_index_escape_scope')) is str and prep['raw_index_escape_scope'],
            'canonical escaped-reference scope absent')
    ranges = prep.get('normal_ranges')
    require(type(ranges) is list and ranges, 'canonical prepared draw ranges missing')
    end = 0
    for pair in ranges:
        require(type(pair) is list and len(pair) == 2 and
                all(type(x) is int for x in pair) and pair[0] >= end and pair[1] > 0
                and pair[0]+pair[1] <= 0x200000, 'canonical prepared draw ranges invalid')
        end = pair[0]+pair[1]
    require(sum(pair[1] for pair in ranges) <= prep['used_records'],
            'visible prepared ranges exceed owned used records')
    require(all(isinstance(value, dict) and value.get('canonical_brick_layout') == prep
                for value in (input_before, input_after)),
            'canonical preparation/input endpoint association differs')
    # Only encoding and association are checked here. Python has no BLAKE3;
    # exact raw values are paired by the existing input/workload comparison.
    return prep
