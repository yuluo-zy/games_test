//! B gather/opportunity/finish/read_back ownership on the current renderer.
//! T13 feedback capture is excluded. See CONTRACT.md for remaining producers.
use super::{Completion, ScreenshotPurpose, ScreenshotRequest, ScreenshotSink};
use crate::{render_bridge::CoreGraphFrame, render_host::HostTables};
use bevy_ecs::{prelude::*, system::SystemParam};
use country_core::{
    game_config::{GameConfig, ScreenshotFormat},
    input::action_set::MainActionSet,
    meta_commander::MetaCommander,
    render::{
        pass_builder::{compute_pass_builder::RgBuffer, pass_resource_access::RgTexture},
        render_config::FrameRenderConfig,
    },
    resources::{
        events::EscapeMenuJustOpened,
        game_clock::GameClock,
        gui::{EscMenuState, NextEscMenuState},
        photo_mode::PhotoModeCmd,
    },
    rhapsody_types::{Texture, TextureDesc},
    systems::{
        mode_manager::AppMode,
        render_loop::{lut_gallery::LutGalleryState, menu_background::ManagedTexture},
        save::save_game::SavingGameComplete,
        steam::{SteamClientResource, SteamScreenshotRequests, SteamScreenshotSlot},
    },
};
use rhapsody::resource::ManagedBuffer;
use std::{
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread::JoinHandle,
};

type PreviewSlot = Arc<parking_lot::Mutex<Option<ManagedTexture>>>;
enum Destination {
    Menu,
    Gallery(PreviewSlot),
    Pixels(ScreenshotSink),
}
enum Capture {
    /// Original payload tag 6. No allocation has happened yet.
    Awaiting,
    Texture {
        texture: Texture,
        owner: rhapsody::resource::ManagedTexture,
        hosts: HostTables,
        published: bool,
        logical: rhapsody::rg::ResourceHandle,
        retired: Arc<AtomicBool>,
        submitted: bool,
    },
    Buffer {
        buffer: ManagedBuffer,
        logical: rhapsody::rg::ResourceHandle,
        retired: Arc<AtomicBool>,
        submitted: bool,
    },
    Done,
}
struct Request {
    purpose: u8,
    dimensions: Option<[u32; 2]>,
    destination: Destination,
    completion: Option<Completion>,
    capture: Capture,
    extent: [u32; 2],
    /// Original state @+0x50 and budget @+0x54, independent of payload tag.
    hurry: bool,
    remaining: u32,
}
impl Drop for Request {
    fn drop(&mut self) {
        // Published menu/gallery slots have their own retained host owners.
        if let Capture::Texture {
            texture,
            hosts,
            published: false,
            ..
        } = &self.capture
        {
            hosts.textures.release(texture);
        }
    }
}
#[derive(Resource, Default)]
struct Requests {
    active: Vec<Request>,
    writers: Vec<JoinHandle<()>>,
    closing: bool,
}

impl Drop for Requests {
    fn drop(&mut self) {
        // World replacement can retire an App without a shutdown frame. Its
        // captures cannot move to the incoming App's graph or lose callbacks.
        for request in &mut self.active {
            if let Some(callback) = request.completion.take() {
                callback(Err("Screenshot cancelled when its App owner was released".to_owned()));
            }
        }
        // Per-App ownership is an API adaptation of the original global writer
        // list. Unloading a gameplay World must not detach its disk writes.
        for writer in self.writers.drain(..) {
            let _ = writer.join();
        }
    }
}

pub fn install(app: &mut bevy_app::App) {
    app.init_resource::<Requests>();
    app.add_event::<EscapeMenuJustOpened>();
    // gather_screenshot_requests (0x14031af10) consumers; all are resources/events
    // of the real gameplay App, initialised here only when a host has not done so.
    app.add_event::<PhotoModeCmd>();
    app.add_event::<SavingGameComplete>();
    app.init_resource::<AppMode>();
    app.init_resource::<GameClock>();
    app.init_resource::<MainActionSet>();
    app.init_resource::<MetaCommander>();
    app.add_systems(bevy_app::PostUpdate, gather);
}

/// Independent loading Apps have no screenshot owner; hosts gate optional work
/// by installation, while a gameplay owner remains mandatory for enqueue.
/// Screenshot ownership across the render thread: the game world keeps an
/// (empty) owner so gather/enqueue work, and each hand-over moves the new
/// requests to the render world, where declaration, submission and readback
/// handle them.
pub(crate) fn render_exchange(set: &mut crate::render_thread::RenderExchange) {
    set.extract.add(|game, render, _| {
        let Some(mut source) = game.get_resource_mut::<Requests>() else {
            return;
        };
        let incoming = std::mem::take(&mut source.active);
        let closing = source.closing;
        let mut owner = render.get_resource_or_insert_with(Requests::default);
        owner.active.extend(incoming);
        owner.closing |= closing;
    });
}

pub fn installed(world: &World) -> bool {
    world.contains_resource::<Requests>()
}

pub fn enqueue(world: &mut World, request: ScreenshotRequest) -> rhapsody::Result<()> {
    let mut owner = world
        .get_resource_mut::<Requests>()
        .ok_or_else(missing_owner)?;
    if owner.closing {
        return Err(rhapsody::Error::invalid_usage(
            "screenshot owner is shutting down",
        ));
    }
    owner.active.push(pixel_request(request, 3));
    Ok(())
}

/// Native gather map closure (RNE14048dba4..dbe8): state=0, tag 6 (Awaiting),
/// remaining=3 at +0x50/+0x54. MetaCommander's direct push uses remaining=1.
fn pixel_request(request: ScreenshotRequest, remaining: u32) -> Request {
    Request {
        purpose: request.purpose.as_byte(),
        dimensions: request.dimensions,
        destination: Destination::Pixels(request.sink),
        completion: request.completion,
        capture: Capture::Awaiting,
        extent: [0; 2],
        hurry: false,
        remaining,
    }
}

/// MetaCommander screenshot @14031c38d has the same payload but budget=1.
/// Its caller still owns path/visibility selection and command consumption.
pub fn enqueue_meta_capture(world: &mut World, request: ScreenshotRequest) -> rhapsody::Result<()> {
    if request.purpose != ScreenshotPurpose::SceneWithUi {
        return Err(rhapsody::Error::invalid_usage(
            "native MetaCommander screenshot purpose must be 4",
        ));
    }
    let mut owner = world
        .get_resource_mut::<Requests>()
        .ok_or_else(missing_owner)?;
    if owner.closing {
        return Err(rhapsody::Error::invalid_usage(
            "screenshot owner is shutting down",
        ));
    }
    owner.active.push(pixel_request(request, 1));
    Ok(())
}

pub fn active_capture_purpose(world: &World) -> u8 {
    world
        .get_resource::<Requests>()
        .and_then(|owner| {
            owner
                .active
                .iter()
                .find(|r| awaiting(&r.capture))
                .map(|r| r.purpose)
        })
        .unwrap_or(7)
}
pub fn has_requests(world: &World) -> bool {
    world
        .get_resource::<Requests>()
        .is_some_and(|owner| !owner.active.is_empty())
}

/// `Local<[u8;2]>` of gather_screenshot_requests: `[hold active, frame counter]`.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct PhotoHold {
    active: u8,
    counter: u8,
}

/// Effects of one photo-mode hold step (0x14031b2a0..b313, b582).
#[derive(Default, Debug, PartialEq, Eq)]
pub(crate) struct HoldStep {
    /// Enter LAB_14031b313: build the main screenshot request now.
    capture: bool,
    /// Fall through to LAB_14031b582: check the grading-reference key.
    check_grading: bool,
    /// `FrameRenderConfig.flag_20 = v` (+ changed tick), when written.
    flag_20: Option<bool>,
    /// GameClock byte +0x31 (`paused`) `= v` (+ changed tick), when written.
    clock_paused: Option<bool>,
}

/// Bit-exact branch table of the original hold state machine.
/// `trigger` = Steam placeholder | PhotoModeCmd tag 4 | screenshot key.
pub(crate) fn photo_hold_step(hold: &mut PhotoHold, trigger: bool, photomode: bool) -> HoldStep {
    if hold.active == 0 {
        if trigger && photomode {
            // b2a0..b2c2: clock +0x31=1, state=[1,1]; do not capture this frame.
            *hold = PhotoHold { active: 1, counter: 1 };
            return HoldStep {
                check_grading: true,
                clock_paused: Some(true),
                ..HoldStep::default()
            };
        }
        // b30a: a trigger outside Photomode captures immediately.
        return HoldStep {
            capture: trigger,
            check_grading: !trigger,
            ..HoldStep::default()
        };
    }
    match hold.counter {
        0..=0x0f => {
            hold.counter += 1;
            HoldStep {
                check_grading: true,
                flag_20: Some(true),
                ..HoldStep::default()
            }
        }
        0x10 => {
            hold.counter = 0x11;
            HoldStep {
                capture: true,
                ..HoldStep::default()
            }
        }
        _ => {
            // counter > 0x10: release (counter itself is left at 0x11).
            hold.active = 0;
            HoldStep {
                capture: trigger,
                check_grading: !trigger,
                flag_20: Some(false),
                clock_paused: Some(false),
            }
        }
    }
}

/// purpose = 4 - (AppMode == Photomode) (0x14031b73a..b744), extension from
/// `GameConfig.screenshot_format` (+0x120, 0x14031b32f), quality png=0x64 / jpg=0x55.
pub(crate) fn main_capture_params(
    mode: AppMode,
    format: ScreenshotFormat,
) -> (ScreenshotPurpose, &'static str, u8) {
    let purpose = if mode == AppMode::Photomode {
        ScreenshotPurpose::SceneBeforeUi
    } else {
        ScreenshotPurpose::SceneWithUi
    };
    match format {
        ScreenshotFormat::Jpeg => (purpose, "jpg", 0x55),
        ScreenshotFormat::Png => (purpose, "png", 0x64),
    }
}

/// `paths::config()/"screenshots"/<date_time_string_for_filename>.<ext>`.
pub(crate) fn main_capture_path(config_dir: &Path, stamp: &str, ext: &str) -> PathBuf {
    config_dir.join("screenshots").join(format!("{stamp}.{ext}"))
}

/// `dims=[w*64/max(h,1) (>=1), 64]` of the loading_background request (0x14031c0f8+).
pub(crate) fn loading_background_width(extent_8: [u32; 2]) -> u32 {
    let ratio = (extent_8[0] as f32 * 64.0) / extent_8[1].max(1) as f32;
    ratio.max(1.0) as u32
}

/// Original binding slots (`MainActionSet.digital` byte offsets 0xe8/0xf0 / 8).
const SCREENSHOT_SLOT: usize = 29;
const GRADING_REFERENCE_SLOT: usize = 30;

fn file_request(
    purpose: ScreenshotPurpose,
    dimensions: Option<[u32; 2]>,
    path: PathBuf,
    jpeg_quality: u8,
    completion: Option<Completion>,
) -> ScreenshotRequest {
    ScreenshotRequest {
        purpose,
        dimensions,
        sink: ScreenshotSink::File { path, jpeg_quality },
        completion,
    }
}

/// Steam half of the gather parameters: both must exist (the original resource is
/// only present with a Steam client; this host always inits the request list).
#[derive(SystemParam)]
struct SteamParams<'w> {
    requests: Option<Res<'w, SteamScreenshotRequests>>,
    client: Option<Res<'w, SteamClientResource>>,
}

/// Completion of a Steam-visible screenshot (vtable 0x1429b1280 -> 0x1402ce120): fill the
/// shared slot with `Ok((path, dims))` or the error.
fn steam_completion(path: PathBuf, slot: Arc<SteamScreenshotSlot>) -> Completion {
    Arc::new(move |result| {
        let value = match result {
            Ok(dims) => Ok((path.clone(), dims)),
            Err(message) => Err(message.into()),
        };
        *slot.lock() = Some(value);
    })
}

#[allow(clippy::too_many_arguments)]
fn gather(
    mut opened: EventReader<EscapeMenuJustOpened>,
    menu: Res<EscMenuState>,
    mut next: ResMut<NextEscMenuState>,
    mut config: ResMut<FrameRenderConfig>,
    mut gallery: ResMut<LutGalleryState>,
    mut requests: ResMut<Requests>,
    steam: SteamParams,
    app_mode: Res<AppMode>,
    game_config: Res<GameConfig>,
    mut hold: Local<PhotoHold>,
    mut clock: ResMut<GameClock>,
    actions: Res<MainActionSet>,
    mut photo_cmds: EventReader<PhotoModeCmd>,
    mut saved: EventReader<SavingGameComplete>,
    mut commander: ResMut<MetaCommander>,
) {
    // Steam: Vec::retain_mut(add_captured_screenshots_to_steam_library) @14031b1d4.
    let steam = steam.requests.as_deref().zip(
        steam
            .client
            .as_deref()
            .and_then(|resource| resource.client.clone()),
    );
    if let Some((steam_requests, client)) = &steam {
        super::add_captured_screenshots_to_steam_library(steam_requests, client.as_ref());
    }
    reap_writers(&mut requests.writers);
    if requests.closing {
        return;
    }
    let mut gathered: Vec<Request> = Vec::new();
    // Trigger sources, in original evaluation order and short-circuit: a Steam
    // placeholder, else PhotoModeCmd tag 4 (reader stops at the match), else the key.
    let trigger = steam
        .as_ref()
        .is_some_and(|(steam_requests, _)| steam_requests.take_requested())
        || photo_cmds.read().any(|cmd| cmd.0 == 4)
        || actions.digital[SCREENSHOT_SLOT].just_pressed();
    let step = photo_hold_step(&mut hold, trigger, *app_mode == AppMode::Photomode);
    if let Some(paused) = step.clock_paused {
        clock.paused = paused;
    }
    if let Some(flag) = step.flag_20 {
        config.flag_20 = flag;
    }
    if step.capture {
        let (purpose, ext, quality) = main_capture_params(*app_mode, game_config.game.screenshot_format);
        let path = main_capture_path(
            paths::config(),
            &utils::file::date_time_string_for_filename(),
            ext,
        );
        let completion = steam.as_ref().map(|(steam_requests, _)| {
            let slot = Arc::new(SteamScreenshotSlot::new(None));
            steam_requests.push_captured(slot.clone());
            steam_completion(path.clone(), slot)
        });
        gathered.push(pixel_request(
            file_request(purpose, None, path, quality, completion),
            3,
        ));
    } else if step.check_grading && actions.digital[GRADING_REFERENCE_SLOT].just_pressed() {
        // purpose 0: relative `grading_reference.png` (.rdata 0x1429bc800), quality 0x64.
        gathered.push(pixel_request(
            file_request(
                ScreenshotPurpose::GradingReference,
                None,
                PathBuf::from("grading_reference.png"),
                0x64,
                None,
            ),
            3,
        ));
    }
    // purpose 6 (T13 feedback capture) is excluded by the user and not produced.
    if matches!(*menu, EscMenuState::Photomode(false)) {
        let side = ((config.extent_8[1] as f32 * 0.4) as u32).max(1);
        let slot = Arc::new(parking_lot::Mutex::new(None));
        gallery.preview = Some(slot.clone());
        next.set(EscMenuState::Photomode(true));
        gathered.push(shared(1, Some([side, side]), Destination::Gallery(slot)));
    }
    if opened.read().count() != 0 {
        gathered.push(shared(2, None, Destination::Menu));
    }
    // SavingGameComplete: the LAST event of the frame (0x14031bf30..bfd7) -> two files.
    if let Some(done) = saved.read().last() {
        gathered.push(pixel_request(
            file_request(
                ScreenshotPurpose::MenuBackground,
                Some([0x200, 0x200]),
                done.path.join("screenshot.jpg"),
                0x55,
                None,
            ),
            3,
        ));
        gathered.push(pixel_request(
            file_request(
                ScreenshotPurpose::MenuBackground,
                Some([loading_background_width(config.extent_8), 0x40]),
                done.path.join("loading_background.jpg"),
                0x5f,
                None,
            ),
            3,
        ));
    }
    // Original local list order (main/grading, LUT, menu, session files) is the
    // order appended to the global request vector.
    requests.active.extend(gathered);
    // MetaCommander Screenshot{path}: direct push, purpose 4, q=0x55, dims=None,
    // remaining=1, no completion (0x14031c38d).
    if let Some(path) = commander.consume_screenshot() {
        requests.active.push(pixel_request(
            file_request(ScreenshotPurpose::SceneWithUi, None, path, 0x55, None),
            1,
        ));
    }
}
fn shared(purpose: u8, dimensions: Option<[u32; 2]>, destination: Destination) -> Request {
    Request {
        purpose,
        dimensions,
        destination,
        completion: None,
        capture: Capture::Awaiting,
        extent: [0; 2],
        hurry: false,
        remaining: 3,
    }
}
fn missing_owner() -> rhapsody::Error {
    rhapsody::Error::invalid_usage("missing original screenshot request owner")
}

/// All matching awaiting requests are captured, preserving native list order.
/// Purpose equality is equivalent to the native normalized 2..6 category test
/// plus exact-byte comparison for the fallback category.
pub fn opportunity(
    frame: &mut CoreGraphFrame<'_>,
    world: &mut World,
    purpose: u8,
    source: RgTexture,
    samples: u8,
) -> rhapsody::Result<usize> {
    let mut requests = world
        .remove_resource::<Requests>()
        .ok_or_else(missing_owner)?;
    let result = (|| {
        if !matches!(samples, 1 | 4) {
            return Err(rhapsody::Error::invalid_usage(
                "screenshot source must have 1 or 4 samples",
            ));
        }
        reset_cancelled(&mut requests);
        let mut count = 0;
        for request in &mut requests.active {
            if request.purpose != purpose || !matches!(request.capture, Capture::Awaiting) {
                continue;
            }
            let extent = request.dimensions.unwrap_or(source.desc.extent_2d());
            if extent.contains(&0) {
                return Err(rhapsody::Error::invalid_usage("zero screenshot extent"));
            }
            let mut capture = match &request.destination {
                Destination::Pixels(_) => capture_buffer(frame, world, source, extent, samples)?,
                _ => capture_texture(frame, world, source, extent, samples)?,
            };
            if let Capture::Texture {
                texture,
                hosts,
                published,
                ..
            } = &mut capture
            {
                let publication = match &request.destination {
                    Destination::Menu => {
                        crate::menu_background_host::publish(world, hosts, texture.clone())
                    }
                    Destination::Gallery(slot) => {
                        crate::world_postprocess::publish_lut_gallery_preview_to_slot(
                            world,
                            hosts,
                            slot,
                            texture.clone(),
                        )
                    }
                    Destination::Pixels(_) => unreachable!(),
                };
                if let Err(error) = publication {
                    hosts.textures.release(texture);
                    return Err(error);
                }
                // Native side effect is visible to later passes in THIS graph.
                *published = true;
            }
            request.extent = extent;
            request.capture = capture;
            count += 1;
        }
        Ok(count)
    })();
    world.insert_resource(requests);
    result
}

// Callback runs on cancellation or successful graph retirement. A receipt-backed
// submitted=true request remains captured; only never-submitted retirements reset.
fn retirement_flag(frame: &mut CoreGraphFrame<'_>) -> rhapsody::Result<Arc<AtomicBool>> {
    let retired = Arc::new(AtomicBool::new(false));
    let notify = retired.clone();
    frame.with_graph(|graph| {
        graph.on_retire(move || notify.store(true, Ordering::Release));
        Ok(())
    })?;
    Ok(retired)
}
fn awaiting(capture: &Capture) -> bool {
    match capture {
        Capture::Awaiting => true,
        Capture::Texture {
            submitted: false,
            retired,
            ..
        }
        | Capture::Buffer {
            submitted: false,
            retired,
            ..
        } => retired.load(Ordering::Acquire),
        _ => false,
    }
}
fn reset_cancelled(owner: &mut Requests) {
    for request in &mut owner.active {
        if !matches!(request.capture, Capture::Awaiting) && awaiting(&request.capture) {
            if let Capture::Texture {
                texture,
                hosts,
                published: false,
                ..
            } = &request.capture
            {
                hosts.textures.release(texture);
            }
            request.capture = Capture::Awaiting;
            request.extent = [0; 2];
        }
    }
}

fn capture_buffer(
    frame: &mut CoreGraphFrame<'_>,
    world: &World,
    source: RgTexture,
    extent: [u32; 2],
    samples: u8,
) -> rhapsody::Result<Capture> {
    let hosts = frame.hosts();
    let bytes = byte_size(extent)?;
    // Native usage=STORAGE_BUFFER, memory=GpuToCpu. Current safe read_bytes
    // retains invalidation and waits for actual overlapping writes.
    let buffer = hosts.buffers.graphics_device().create_buffer(
        &rhapsody::resource::BufferDesc::new_gpu_to_cpu(
            bytes,
            ash::vk::BufferUsageFlags::STORAGE_BUFFER,
        ),
        "screenshot buffer",
        (),
    )?;
    let target = frame.import_native_buffer(buffer.shared(), 0x00)?;
    crate::scene_compute::with_env(frame, world, |env| {
        let (shader, binding) = if samples == 1 {
            (
                "/screenshot/capture_screenshot_buffer.cs.hlsl:sample1",
                0xd15b_e69a_f90f_5947,
            )
        } else {
            (
                "/screenshot/capture_screenshot_buffer.cs.hlsl:sample4",
                0x681c_a9c9_3557_dc9c,
            )
        };
        // Current typed RG binding retains the same buffer owner and declares
        // its actual write; original bind_external_storage_buffer hash below.
        let pass = env
            .try_new_pass("Capture screenshot buffer", &shader)?
            .bind_texture(binding, &&source, 0x19)
            .write_rg_storage_buffer(
                0x0f68_04da_1cec_c30e,
                &RgBuffer {
                    resource: target.raw(),
                },
            )
            .uniform(0xca50_2818_f423_c41d, extent)
            .dispatch_2d(extent[0], extent[1]);
        env.record(pass);
        Ok(())
    })?;
    frame.with_graph(|graph| {
        graph.export(target, 0x26)?;
        Ok(())
    })?;
    let retired = retirement_flag(frame)?;
    Ok(Capture::Buffer {
        buffer,
        logical: target,
        retired,
        submitted: false,
    })
}

fn capture_texture(
    frame: &mut CoreGraphFrame<'_>,
    world: &World,
    source: RgTexture,
    extent: [u32; 2],
    sample_count: u8,
) -> rhapsody::Result<Capture> {
    let hosts = frame.hosts();
    use country_core::render::load_texture::boundary::TextureDevice;
    let desc = TextureDesc::new_2d(extent, 0x25).storage().sampled();
    let texture = hosts
        .textures
        .create_texture("screenshot target", &desc, &[])
        .map_err(|error| rhapsody::Error::invalid_usage(error.to_string()))?;
    let built = (|| {
        // TextureHost initializes sampled images to AnyShaderRead. Storage
        // writes belong to the capture pass, not the image's initial state.
        let target_id = frame.import_texture(&texture, 0x22)?;
        let target = RgTexture {
            resource: target_id.raw(),
            desc,
        };
        crate::scene_compute::with_env(frame, world, |env| {
            let mut input = source;
            let mut samples = sample_count;
            for _ in 0..downsample_count(source.desc.extent_2d()[1], extent[1]) {
                let half = country_core::systems::render_loop::compute_passes::pass_kit::half_res(
                    input.desc,
                );
                let out = env.create_texture(
                    "blur downsample out",
                    TextureDesc::new_2d(half.extent_2d(), 0x25),
                );
                let (shader, binding) = if samples == 1 {
                    ("/blur_downsample.cs.hlsl", 0x11ba_5697_cc40_b3dd)
                } else {
                    (
                        "/blur_downsample.cs.hlsl:cs_main_multisampled",
                        0xd03d_c8b4_a9aa_ab46,
                    )
                };
                let pass = env
                    .try_new_pass("Blur downsample", &shader)?
                    .bind_texture(binding, &&input, 0x19)
                    .bind_texture(0x8821_9a3a_e698_92b2, &&out, 0x32)
                    .dispatch_2d(half.extent_2d()[0], half.extent_2d()[1]);
                env.record(pass);
                input = out;
                samples = 1;
            }
            let (shader, binding) = if samples == 1 {
                (
                    "/screenshot/capture_screenshot_texture.cs.hlsl:sample1",
                    0xd15b_e69a_f90f_5947,
                )
            } else {
                (
                    "/screenshot/capture_screenshot_texture.cs.hlsl:sample4",
                    0x681c_a9c9_3557_dc9c,
                )
            };
            let pass = env
                .try_new_pass("Capture screenshot texture", &shader)?
                .bind_texture(binding, &&input, 0x19)
                .bind_texture(0x94f8_b4d9_d73f_8eea, &&target, 0x32)
                .uniform(0xca50_2818_f423_c41d, extent)
                .dispatch_2d(extent[0], extent[1]);
            env.record(pass);
            Ok(())
        })?;
        frame.with_graph(|graph| {
            graph.export(target_id, 0x22)?;
            Ok(())
        })?;
        let retired = retirement_flag(frame)?;
        let owner = hosts.textures.get(&texture).ok_or_else(|| {
            rhapsody::Error::invalid_usage("new screenshot texture missing from its host")
        })?;
        Ok(Capture::Texture {
            texture: texture.clone(),
            owner,
            hosts: hosts.clone(),
            published: false,
            logical: target_id,
            retired,
            submitted: false,
        })
    })();
    if built.is_err() {
        hosts.textures.release(&texture);
    }
    built
}

/// Required host hook: call immediately after this main graph successfully
/// executes, before presentation acquisition. Never call after a failed submit.
pub fn submitted(
    world: &mut World,
    bridge: &mut crate::render_bridge::CoreRenderBridge,
    graph_id: rhapsody::rg::GraphId,
    receipt: &rhapsody::rg::SessionFrameReceipt,
) -> rhapsody::Result<()> {
    let mut owner = world
        .remove_resource::<Requests>()
        .ok_or_else(missing_owner)?;
    let result = (|| {
        for request in &mut owner.active {
            let (logical, submitted) = match &request.capture {
                Capture::Texture {
                    logical, submitted, ..
                }
                | Capture::Buffer {
                    logical, submitted, ..
                } => (*logical, *submitted),
                _ => continue,
            };
            if submitted || logical.graph_id() != graph_id {
                continue;
            }
            if receipt.receipts.is_empty()
                || !u16::try_from(logical.raw())
                    .is_ok_and(|index| receipt.snapshot.exports.contains(&index))
            {
                return Err(rhapsody::Error::invalid_usage(
                    "screenshot has no successful exported submission",
                ));
            }
            let mapping = receipt
                .snapshot
                .physical_mapping
                .iter()
                .find(|m| m.logical == logical.raw())
                .ok_or_else(|| {
                    rhapsody::Error::invalid_usage(
                        "screenshot export absent from executed main graph",
                    )
                })?;
            let token = mapping.token.ok_or_else(|| {
                rhapsody::Error::invalid_usage("screenshot export has no physical owner")
            })?;
            let actual = bridge.with_session(|session| session.backend.resource_clone(token));
            // GraphId is carried from the same BuiltCoreGraph/CompiledCoreGraph
            // guard that produced this receipt. Actual Arc identity additionally
            // rejects wrong-App receipts and reused logical indices.
            let same = match (&request.capture, actual) {
                (
                    Capture::Buffer { buffer, .. },
                    Some(rhapsody::rg::GpuResource::Buffer(actual)),
                ) => Arc::ptr_eq(&buffer.shared(), &actual),
                (
                    Capture::Texture { owner, .. },
                    Some(rhapsody::rg::GpuResource::Image(actual)),
                ) => Arc::ptr_eq(&owner.shared(), &actual),
                _ => false,
            };
            if !same {
                return Err(rhapsody::Error::invalid_usage(
                    "screenshot receipt belongs to a different GPU owner",
                ));
            }
            match &mut request.capture {
                Capture::Texture { submitted, .. } | Capture::Buffer { submitted, .. } => {
                    *submitted = true
                }
                _ => unreachable!(),
            }
        }
        Ok(())
    })();
    world.insert_resource(owner);
    result
}

/// Required host cancellation hook for a graph that was never executed.
/// Reset only declarations from that graph; preserve older submitted requests.
pub fn cancel_unsubmitted(world: &mut World, graph_id: rhapsody::rg::GraphId) {
    if let Some(mut owner) = world.get_resource_mut::<Requests>() {
        for request in &mut owner.active {
            let reset = match &request.capture {
                Capture::Texture {
                    logical, submitted, ..
                }
                | Capture::Buffer {
                    logical, submitted, ..
                } => !submitted && logical.graph_id() == graph_id,
                _ => false,
            };
            if reset {
                if let Capture::Texture {
                    texture,
                    hosts,
                    published: false,
                    ..
                } = &request.capture
                {
                    hosts.textures.release(texture);
                }
                request.capture = Capture::Awaiting;
                request.extent = [0; 2];
            }
        }
    }
}

pub fn finish(world: &mut World) -> rhapsody::Result<()> {
    let mut owner = world
        .remove_resource::<Requests>()
        .ok_or_else(missing_owner)?;
    let result = finish_owned(&mut owner);
    world.insert_resource(owner);
    result
}
fn finish_owned(owner: &mut Requests) -> rhapsody::Result<()> {
    reset_cancelled(owner);
    reap_writers(&mut owner.writers);
    for request in &mut owner.active {
        let submitted = match request.capture {
            Capture::Texture { submitted, .. } | Capture::Buffer { submitted, .. } => submitted,
            _ => false,
        };
        if !submitted {
            continue;
        }
        if !request.hurry && request.remaining != 0 {
            request.remaining -= 1;
            continue;
        }
        match &request.capture {
            Capture::Texture { hosts, .. } => {
                if request.hurry {
                    hosts.buffers.graphics_device().wait_idle()?;
                }
                if let Some(callback) = request.completion.take() {
                    callback(Ok(request.extent));
                }
            }
            Capture::Buffer { buffer, .. } => {
                if request.hurry {
                    buffer.device().wait_idle()?;
                }
                let mut rgba = vec![0; byte_size(request.extent)? as usize];
                buffer.read_bytes(0, &mut rgba)?;
                let Destination::Pixels(sink) = &request.destination else {
                    unreachable!()
                };
                let sink = sink.clone();
                let extent = request.extent;
                let completion = request.completion.clone();
                if request.hurry {
                    deliver(sink, extent, rgba, completion);
                    request.completion = None;
                } else {
                    // Keep callback ownership in request until spawn succeeds.
                    // Native normal readback is asynchronous; hurry is synchronous.
                    let writer = std::thread::Builder::new()
                        .name("Screenshot image writer".into())
                        .spawn(move || deliver(sink, extent, rgba, completion))
                        .map_err(|error| {
                            rhapsody::Error::invalid_usage(format!(
                                "failed to spawn screenshot writer: {error}"
                            ))
                        })?;
                    owner.writers.push(writer);
                    request.completion = None;
                }
            }
            _ => unreachable!(),
        }
        request.capture = Capture::Done;
    }
    // B retain@1402b5c00 retains state != 2, independent of payload tag.
    owner.active.retain(|r| !matches!(r.capture, Capture::Done));
    Ok(())
}

fn deliver(sink: ScreenshotSink, extent: [u32; 2], rgba: Vec<u8>, completion: Option<Completion>) {
    match sink {
        ScreenshotSink::File { path, jpeg_quality } => {
            super::write_image_with_callback_impl(
                &path,
                extent[0],
                extent[1],
                &rgba,
                jpeg_quality,
                &|result| {
                    if let Err(error) = &result {
                        log::warn!("Error writing screenshot: {error}");
                    }
                    if let Some(callback) = &completion {
                        callback(result);
                    }
                },
            );
        }
        ScreenshotSink::Rgba(callback) => {
            callback(extent[0], extent[1], rgba);
            if let Some(completion) = completion {
                completion(Ok(extent));
            }
        }
    }
}
fn reap_writers(writers: &mut Vec<JoinHandle<()>>) {
    let mut index = 0;
    while index < writers.len() {
        if writers[index].is_finished() {
            let _ = writers.remove(index).join();
        } else {
            index += 1;
        }
    }
}

pub fn shutdown(world: &mut World) -> rhapsody::Result<()> {
    let mut owner = world
        .remove_resource::<Requests>()
        .ok_or_else(missing_owner)?;
    owner.closing = true;
    for request in &mut owner.active {
        request.hurry = true;
    }
    let result = finish_owned(&mut owner);
    // No later graph can satisfy an unsubmitted request after shutdown. Settle
    // its callback exactly once instead of silently dropping it with the World.
    let message = result.as_ref().err().map(|error| format!("Screenshot shutdown failed: {error}"))
        .unwrap_or_else(|| "Screenshot cancelled before submission during shutdown".to_owned());
    for mut request in owner.active.drain(..) {
        if let Some(callback) = request.completion.take() { callback(Err(message.clone())); }
    }
    // Always join all previously started writer threads even if readback failed.
    for writer in owner.writers.drain(..) {
        let _ = writer.join();
    }
    world.insert_resource(owner);
    result
}
fn byte_size(extent: [u32; 2]) -> rhapsody::Result<u64> {
    u64::from(extent[0])
        .checked_mul(u64::from(extent[1]))
        .and_then(|n| n.checked_mul(4))
        .filter(|n| usize::try_from(*n).is_ok())
        .ok_or_else(|| rhapsody::Error::invalid_usage("screenshot buffer size overflow"))
}
fn downsample_count(source_height: u32, output_height: u32) -> u64 {
    (source_height as f32 / output_height as f32).log2().round() as u64
}


/// Render-thread extraction: what this module reads from the World.
pub fn render_extract(m: &mut impl country_core::render::extract::RenderExtract) {
    m.resource::<AppMode>();
    m.resource::<GameClock>();
    m.resource::<MainActionSet>();
    m.resource::<MetaCommander>();
}

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::{
        input::action_set::{DigitalActionPhase, DigitalActionState},
        resources::steam::{AnalogActionData, DigitalActionData, SteamSdk},
    };
    use std::sync::Mutex;

    #[test]
    fn shutdown_settles_unsubmitted_requests_after_render_ownership_transfer_once() {
        let mut game = World::new();
        game.init_resource::<Requests>();
        let (send, receive) = std::sync::mpsc::channel();
        enqueue(&mut game, ScreenshotRequest {
            purpose: ScreenshotPurpose::FinalFrame,
            dimensions: None,
            sink: ScreenshotSink::Rgba(Arc::new(|_, _, _| panic!("cancelled request delivered pixels"))),
            completion: Some(Arc::new(move |result| { send.send(result).unwrap(); })),
        }).unwrap();
        let mut exchange = crate::render_thread::RenderExchange::default();
        render_exchange(&mut exchange);
        let mut render = World::new();
        exchange.extract.run(&mut game, &mut render);
        shutdown(&mut game).unwrap();
        assert!(receive.try_recv().is_err(), "the request moved to the render owner");
        shutdown(&mut render).unwrap();
        assert!(receive.recv().unwrap().unwrap_err().contains("cancelled before submission"));
        shutdown(&mut render).unwrap();
        assert!(receive.try_recv().is_err());
        assert!(!has_requests(&render));
    }

    #[test]
    fn replacing_render_world_cancels_only_outgoing_app_requests_once() {
        let (send, receive) = std::sync::mpsc::channel();
        let mut exchange = crate::render_thread::RenderExchange::default();
        render_exchange(&mut exchange);
        let mut render = World::new();
        let mut apps = [World::new(), World::new()];
        for (index, game) in apps.iter_mut().enumerate() {
            game.init_resource::<Requests>();
            let send = send.clone();
            enqueue(game, ScreenshotRequest {
                purpose: ScreenshotPurpose::FinalFrame,
                dimensions: None,
                sink: ScreenshotSink::Rgba(Arc::new(|_, _, _| panic!("cancelled request delivered pixels"))),
                completion: Some(Arc::new(move |result| { send.send((index, result)).unwrap(); })),
            }).unwrap();
        }
        exchange.extract.run(&mut apps[0], &mut render);
        render = World::new();
        let (index, result) = receive.try_recv().unwrap();
        assert_eq!(index, 0);
        assert!(result.unwrap_err().contains("App owner"));
        exchange.extract.run(&mut apps[1], &mut render);
        assert!(has_requests(&render));
        assert!(receive.try_recv().is_err());
        shutdown(&mut render).unwrap();
        assert_eq!(receive.try_recv().unwrap().0, 1);
        drop(render);
        drop(apps);
        assert!(receive.try_recv().is_err());
    }

    /// Slot ids are the byte offsets 0xe8/0xf0/0xf8 of the original resource / 8.
    #[test]
    fn binding_slots_are_the_original_offsets() {
        use country_core::input::action_set::DIGITAL_NAMES;
        assert_eq!(SCREENSHOT_SLOT * 8, 0xe8);
        assert_eq!(GRADING_REFERENCE_SLOT * 8, 0xf0);
        assert_eq!(DIGITAL_NAMES[SCREENSHOT_SLOT], "screenshot");
        assert_eq!(DIGITAL_NAMES[GRADING_REFERENCE_SLOT], "screenshot_grading_reference");
    }

    #[test]
    fn purpose_extension_and_quality_follow_app_mode_and_format() {
        use ScreenshotFormat::{Jpeg, Png};
        assert_eq!(
            main_capture_params(AppMode::Photomode, Jpeg),
            (ScreenshotPurpose::SceneBeforeUi, "jpg", 0x55)
        );
        assert_eq!(
            main_capture_params(AppMode::Photomode, Png),
            (ScreenshotPurpose::SceneBeforeUi, "png", 0x64)
        );
        for mode in [AppMode::default(), AppMode::Terrain, AppMode::Water, AppMode::Replay] {
            assert_eq!(main_capture_params(mode, Jpeg), (ScreenshotPurpose::SceneWithUi, "jpg", 0x55));
            assert_eq!(main_capture_params(mode, Png), (ScreenshotPurpose::SceneWithUi, "png", 0x64));
        }
        assert_eq!(
            main_capture_path(Path::new("/cfg"), "2026-10-20 01-02-03", "png"),
            Path::new("/cfg/screenshots/2026-10-20 01-02-03.png")
        );
    }

    #[test]
    fn loading_background_is_aspect_scaled_to_64_high_and_at_least_one() {
        assert_eq!(loading_background_width([1920, 1080]), 113); // 1920*64/1080 = 113.77
        assert_eq!(loading_background_width([10, 0]), 640); // max(h,1)
        assert_eq!(loading_background_width([0, 100]), 1);
    }

    #[test]
    fn hold_state_machine_matches_every_original_branch() {
        // idle, no trigger: only the grading key is consulted.
        let mut hold = PhotoHold::default();
        assert_eq!(
            photo_hold_step(&mut hold, false, true),
            HoldStep { check_grading: true, ..HoldStep::default() }
        );
        // idle, trigger outside Photomode: capture at once (b30a -> b313).
        assert_eq!(
            photo_hold_step(&mut hold, true, false),
            HoldStep { capture: true, ..HoldStep::default() }
        );
        assert_eq!(hold, PhotoHold::default());
        // idle, trigger in Photomode: clock frozen, state [1,1], no capture, grading checked.
        assert_eq!(
            photo_hold_step(&mut hold, true, true),
            HoldStep { check_grading: true, clock_paused: Some(true), ..HoldStep::default() }
        );
        assert_eq!(hold, PhotoHold { active: 1, counter: 1 });
        // counter 1..=15: flag_20=1 every frame, even with another trigger (not consumed).
        for expected in 2..=16u8 {
            let step = photo_hold_step(&mut hold, true, true);
            assert_eq!(
                step,
                HoldStep { check_grading: true, flag_20: Some(true), ..HoldStep::default() }
            );
            assert_eq!(hold.counter, expected);
        }
        // counter == 0x10: counter = 0x11 and capture; flag_20 untouched this frame.
        assert_eq!(
            photo_hold_step(&mut hold, false, true),
            HoldStep { capture: true, ..HoldStep::default() }
        );
        assert_eq!(hold, PhotoHold { active: 1, counter: 0x11 });
        // counter > 0x10, no trigger: release, grading checked.
        let mut after = hold;
        assert_eq!(
            photo_hold_step(&mut after, false, true),
            HoldStep {
                check_grading: true,
                flag_20: Some(false),
                clock_paused: Some(false),
                ..HoldStep::default()
            }
        );
        assert_eq!(after, PhotoHold { active: 0, counter: 0x11 });
        // counter > 0x10 with a trigger on the same frame: release AND capture at once.
        assert_eq!(
            photo_hold_step(&mut hold, true, true),
            HoldStep {
                capture: true,
                flag_20: Some(false),
                clock_paused: Some(false),
                ..HoldStep::default()
            }
        );
        assert_eq!(hold.active, 0);
    }

    #[derive(Default)]
    struct FakeSdk {
        added: Mutex<Vec<(PathBuf, u32, u32)>>,
        library_error: bool,
    }
    impl SteamSdk for FakeSdk {
        fn user_steam_id(&self) -> u64 { 0 }
        fn run_frame(&self) {}
        fn connected_controllers(&self) -> Vec<u64> { Vec::new() }
        fn action_set_handle(&self, _: &str) -> Option<u64> { None }
        fn analog_action_handle(&self, _: &str) -> Option<u64> { None }
        fn digital_action_handle(&self, _: &str) -> Option<u64> { None }
        fn analog_action_data(&self, _: u64, _: u64) -> AnalogActionData { AnalogActionData::default() }
        fn digital_action_data(&self, _: u64, _: u64) -> DigitalActionData { DigitalActionData::default() }
        fn activate_action_set(&self, _: u64, _: u64) {}
        fn init_input(&self) -> bool { true }
        fn is_steam_running_on_steam_deck(&self) -> bool { false }
        fn show_binding_panel(&self, _: u64) {}
        fn activate_game_overlay_to_web_page(&self, _: &str) {}
        fn hook_screenshots(&self, _: bool) {}
        fn show_floating_gamepad_text_input(&self, _: u32, _: i32, _: i32, _: i32, _: i32) {}
        fn analog_action_origins(&self, _: u64, _: u64, _: u64) -> Vec<u64> { Vec::new() }
        fn digital_action_origins(&self, _: u64, _: u64, _: u64) -> Vec<u64> { Vec::new() }
        fn glyph_for_action_origin(&self, _: u64) -> String { String::new() }
        fn add_screenshot_to_library(&self, path: &Path, width: u32, height: u32) -> Result<(), String> {
            if self.library_error {
                return Err("rejected".into());
            }
            self.added.lock().unwrap().push((path.to_owned(), width, height));
            Ok(())
        }
    }

    fn init_test_paths() -> PathBuf {
        use std::sync::OnceLock;
        static ROOT: OnceLock<PathBuf> = OnceLock::new();
        ROOT.get_or_init(|| {
            let root = tempfile::tempdir().expect("tempdir").keep();
            let installed = root.join("installed");
            std::fs::create_dir_all(&installed).unwrap();
            // Another test of this process may already own the global paths.
            let _ = paths::initialize_paths_with_installation(0, Some(&root), &installed);
            paths::config().to_owned()
        })
        .clone()
    }

    fn pressed() -> DigitalActionState {
        DigitalActionState { phase: DigitalActionPhase::JustPressed, timer_seconds: 0.0 }
    }

    fn world(app_mode: AppMode, format: ScreenshotFormat) -> World {
        let mut world = World::new();
        world.init_resource::<Events<EscapeMenuJustOpened>>();
        world.init_resource::<Events<PhotoModeCmd>>();
        world.init_resource::<Events<SavingGameComplete>>();
        world.init_resource::<EscMenuState>();
        world.init_resource::<NextEscMenuState>();
        world.init_resource::<LutGalleryState>();
        world.init_resource::<Requests>();
        world.init_resource::<GameClock>();
        world.init_resource::<MainActionSet>();
        world.init_resource::<MetaCommander>();
        let mut game_config = GameConfig::default();
        game_config.game.screenshot_format = format;
        world.insert_resource(game_config);
        world.insert_resource(app_mode);
        world.insert_resource(FrameRenderConfig {
            extent_0: [1920, 1080],
            extent_8: [1920, 1080],
            extent_10: [1920, 1080],
            world_resolution_scale: 1.0,
            tgsr_resolution_scale: 1.0,
            flag_20: false,
        });
        world
    }

    fn run(world: &mut World, schedule: &mut Schedule) {
        schedule.run(world);
        world.clear_trackers();
    }

    fn schedule() -> Schedule {
        let mut schedule = Schedule::default();
        schedule.add_systems(gather);
        schedule
    }

    fn requests_of(world: &World) -> Vec<(u8, Option<[u32; 2]>, Option<(PathBuf, u8)>, u32, bool)> {
        world
            .resource::<Requests>()
            .active
            .iter()
            .map(|r| {
                let file = match &r.destination {
                    Destination::Pixels(ScreenshotSink::File { path, jpeg_quality }) => {
                        Some((path.clone(), *jpeg_quality))
                    }
                    _ => None,
                };
                (r.purpose, r.dimensions, file, r.remaining, r.completion.is_some())
            })
            .collect()
    }

    #[test]
    fn screenshot_key_outside_photomode_queues_a_purpose_4_file_with_the_configured_format() {
        let config_dir = init_test_paths();
        for (format, ext, quality) in [(ScreenshotFormat::Jpeg, "jpg", 0x55), (ScreenshotFormat::Png, "png", 0x64)] {
            let mut world = world(AppMode::default(), format);
            let mut schedule = schedule();
            world.resource_mut::<MainActionSet>().digital[SCREENSHOT_SLOT] = pressed();
            run(&mut world, &mut schedule);
            let queued = requests_of(&world);
            assert_eq!(queued.len(), 1);
            let (purpose, dims, file, remaining, completion) = queued[0].clone();
            assert_eq!((purpose, dims, remaining, completion), (4, None, 3, false));
            let (path, q) = file.unwrap();
            assert_eq!(q, quality);
            assert_eq!(path.parent().unwrap(), config_dir.join("screenshots"));
            assert_eq!(path.extension().unwrap(), ext);
            // A pure capture does not touch the photo-hold side effects.
            assert!(!world.resource::<GameClock>().paused);
            assert!(!world.resource::<FrameRenderConfig>().flag_20);
        }
    }

    #[test]
    fn photo_mode_holds_16_frames_then_captures_purpose_3_and_releases() {
        init_test_paths();
        let mut world = world(AppMode::Photomode, ScreenshotFormat::Jpeg);
        let mut schedule = schedule();
        // Frame 0: the key starts the hold; clock frozen, nothing queued.
        world.resource_mut::<MainActionSet>().digital[SCREENSHOT_SLOT] = pressed();
        run(&mut world, &mut schedule);
        world.resource_mut::<MainActionSet>().digital[SCREENSHOT_SLOT] = DigitalActionState::default();
        assert!(world.resource::<GameClock>().paused);
        assert!(requests_of(&world).is_empty());
        assert!(!world.resource::<FrameRenderConfig>().flag_20);
        // Frames 1..=15: flag_20 held on, still nothing queued.
        for _ in 1..=15 {
            run(&mut world, &mut schedule);
            assert!(world.resource::<FrameRenderConfig>().flag_20);
            assert!(requests_of(&world).is_empty());
        }
        // Frame 16: the photo is taken (purpose 3 in Photomode), flag still on.
        run(&mut world, &mut schedule);
        let queued = requests_of(&world);
        assert_eq!(queued.len(), 1);
        assert_eq!(queued[0].0, 3);
        assert!(world.resource::<FrameRenderConfig>().flag_20);
        // Frame 17: released.
        run(&mut world, &mut schedule);
        assert!(!world.resource::<FrameRenderConfig>().flag_20);
        assert!(!world.resource::<GameClock>().paused);
        assert_eq!(requests_of(&world).len(), 1);
    }

    #[test]
    fn flag_20_and_clock_writes_set_change_ticks_only_where_the_original_does() {
        init_test_paths();
        let mut world = world(AppMode::Photomode, ScreenshotFormat::Jpeg);
        let mut schedule = schedule();
        world.clear_trackers();
        // Hold start writes the clock byte (+0x31) but not flag_20.
        world.resource_mut::<MainActionSet>().digital[SCREENSHOT_SLOT] = pressed();
        schedule.run(&mut world);
        assert!(world.is_resource_changed::<GameClock>());
        assert!(!world.is_resource_changed::<FrameRenderConfig>());
        // The next frame writes flag_20 (unconditionally, so it is flagged changed).
        world.clear_trackers();
        world.resource_mut::<MainActionSet>().digital[SCREENSHOT_SLOT] = DigitalActionState::default();
        schedule.run(&mut world);
        assert!(world.is_resource_changed::<FrameRenderConfig>());
        assert!(!world.is_resource_changed::<GameClock>());
    }

    #[test]
    fn photo_mode_cmd_tag_4_triggers_but_camera_tags_do_not() {
        init_test_paths();
        let mut world = world(AppMode::default(), ScreenshotFormat::Jpeg);
        let mut schedule = schedule();
        world.send_event(PhotoModeCmd(1));
        run(&mut world, &mut schedule);
        assert!(requests_of(&world).is_empty());
        world.send_event(PhotoModeCmd(2));
        world.send_event(PhotoModeCmd(4));
        run(&mut world, &mut schedule);
        assert_eq!(requests_of(&world).len(), 1);
    }

    #[test]
    fn grading_reference_key_queues_a_relative_png_purpose_0_and_is_skipped_by_a_capture() {
        init_test_paths();
        let mut world = world(AppMode::default(), ScreenshotFormat::Jpeg);
        let mut schedule = schedule();
        world.resource_mut::<MainActionSet>().digital[GRADING_REFERENCE_SLOT] = pressed();
        run(&mut world, &mut schedule);
        assert_eq!(
            requests_of(&world),
            vec![(0, None, Some((PathBuf::from("grading_reference.png"), 0x64)), 3, false)]
        );
        // Same frame as a capture trigger: the capture path never evaluates purpose 0.
        let mut world = self::world(AppMode::default(), ScreenshotFormat::Jpeg);
        let mut schedule = self::schedule();
        world.resource_mut::<MainActionSet>().digital[SCREENSHOT_SLOT] = pressed();
        world.resource_mut::<MainActionSet>().digital[GRADING_REFERENCE_SLOT] = pressed();
        run(&mut world, &mut schedule);
        let queued = requests_of(&world);
        assert_eq!(queued.len(), 1);
        assert_eq!(queued[0].0, 4);
    }

    #[test]
    fn saving_game_complete_queues_the_two_purpose_2_session_files_from_the_last_event() {
        init_test_paths();
        let mut world = world(AppMode::default(), ScreenshotFormat::Png);
        let mut schedule = schedule();
        world.send_event(SavingGameComplete { path: PathBuf::from("/saves/old") });
        world.send_event(SavingGameComplete { path: PathBuf::from("/saves/new") });
        run(&mut world, &mut schedule);
        assert_eq!(
            requests_of(&world),
            vec![
                (2, Some([512, 512]), Some((PathBuf::from("/saves/new/screenshot.jpg"), 0x55)), 3, false),
                (2, Some([113, 64]), Some((PathBuf::from("/saves/new/loading_background.jpg"), 0x5f)), 3, false),
            ]
        );
    }

    #[test]
    fn meta_screenshot_is_purpose_4_quality_0x55_budget_1_without_completion() {
        init_test_paths();
        let mut world = world(AppMode::default(), ScreenshotFormat::Png);
        let mut schedule = schedule();
        let path = std::env::temp_dir().join(format!("tg-gather-meta-{}", std::process::id())).join("s.png");
        world.resource_mut::<MetaCommander>().push_screenshot(path.clone());
        run(&mut world, &mut schedule);
        assert_eq!(requests_of(&world), vec![(4, None, Some((path, 0x55)), 1, false)]);
        assert!(world.resource::<MetaCommander>().is_empty());
    }

    #[test]
    fn steam_placeholder_triggers_a_capture_whose_completion_feeds_the_library_once() {
        init_test_paths();
        let sdk = Arc::new(FakeSdk::default());
        let mut world = world(AppMode::default(), ScreenshotFormat::Jpeg);
        let steam = SteamScreenshotRequests::default();
        world.insert_resource(steam.clone());
        world.insert_resource(SteamClientResource { client: Some(sdk.clone()) });
        let mut schedule = schedule();
        // Steam callback puts an empty placeholder; gather takes it and captures.
        steam.on_screenshot_requested();
        run(&mut world, &mut schedule);
        let completion = world.resource::<Requests>().active[0].completion.clone().expect("steam completion");
        let path = match &world.resource::<Requests>().active[0].destination {
            Destination::Pixels(ScreenshotSink::File { path, .. }) => path.clone(),
            _ => unreachable!(),
        };
        assert_eq!(steam.requests.read().len(), 1); // Captured(slot), placeholder gone
        // Not completed yet: kept; completed with a real file: handed to Steam once.
        run(&mut world, &mut schedule);
        assert_eq!(steam.requests.read().len(), 1);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, b"x").unwrap();
        completion(Ok([640, 360]));
        run(&mut world, &mut schedule);
        assert!(steam.requests.read().is_empty());
        assert_eq!(*sdk.added.lock().unwrap(), vec![(std::path::absolute(&path).unwrap(), 640, 360)]);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn steam_failures_are_dropped_without_fabricating_a_library_entry() {
        let sdk = FakeSdk { library_error: true, ..FakeSdk::default() };
        let requests = SteamScreenshotRequests::default();
        // capture error / missing file / SDK rejection: all removed, none added.
        let err_slot = Arc::new(SteamScreenshotSlot::new(Some(Err("gpu lost".into()))));
        let missing = Arc::new(SteamScreenshotSlot::new(Some(Ok((PathBuf::from("/definitely/missing.jpg"), [1, 1])))));
        let empty = Arc::new(SteamScreenshotSlot::new(None));
        requests.on_screenshot_requested();
        requests.push_captured(err_slot);
        requests.push_captured(missing);
        requests.push_captured(empty.clone());
        super::super::add_captured_screenshots_to_steam_library(&requests, &sdk);
        let left = requests.requests.read();
        assert_eq!(left.len(), 2); // placeholder + the still-empty slot
        assert!(sdk.added.lock().unwrap().is_empty());
    }

    #[test]
    fn without_a_steam_client_a_capture_has_no_completion_and_no_slot_is_created() {
        init_test_paths();
        let mut world = world(AppMode::default(), ScreenshotFormat::Jpeg);
        // The host always inits the request list; without a client it must stay unused.
        let steam = SteamScreenshotRequests::default();
        world.insert_resource(steam.clone());
        world.insert_resource(SteamClientResource::default());
        let mut schedule = schedule();
        world.resource_mut::<MainActionSet>().digital[SCREENSHOT_SLOT] = pressed();
        run(&mut world, &mut schedule);
        assert!(!requests_of(&world)[0].4);
        assert!(steam.requests.read().is_empty());
    }
}
