//! B gather/opportunity/finish/read_back ownership on the current renderer.
//! T13 feedback capture is excluded. See CONTRACT.md for remaining producers.
use super::{Completion, ScreenshotRequest, ScreenshotSink};
use crate::{render_bridge::CoreGraphFrame, render_host::HostTables};
use bevy_ecs::prelude::*;
use country_core::{
    render::{
        pass_builder::{compute_pass_builder::RgBuffer, pass_resource_access::RgTexture},
        render_config::FrameRenderConfig,
    },
    resources::{
        events::EscapeMenuJustOpened,
        gui::{EscMenuState, NextEscMenuState},
    },
    rhapsody_types::{Texture, TextureDesc},
    systems::render_loop::{lut_gallery::LutGalleryState, menu_background::ManagedTexture},
};
use rhapsody::resource::ManagedBuffer;
use std::{
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
    app.add_event::<country_core::resources::photo_mode::PhotoModeCmd>();
    app.add_event::<country_core::systems::save::save_game::SavingGameComplete>();
    app.add_systems(bevy_app::PostUpdate, gather);
}

/// Independent loading Apps have no screenshot owner; hosts gate optional work
/// by installation, while a gameplay owner remains mandatory for enqueue.
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
    owner.active.push(Request {
        purpose: request.purpose.as_byte(),
        dimensions: request.dimensions,
        destination: Destination::Pixels(request.sink),
        completion: request.completion,
        capture: Capture::Awaiting,
        extent: [0; 2],
        hurry: false,
        // RNE14048dba4..dbe8: store 0x00000003_00000000 at +0x50.
        remaining: 3,
    });
    Ok(())
}

/// MetaCommander screenshot @14031c38d has the same payload but budget=1.
/// Its caller still owns path/visibility selection and command consumption.
pub fn enqueue_meta_capture(world: &mut World, request: ScreenshotRequest) -> rhapsody::Result<()> {
    if request.purpose != super::ScreenshotPurpose::SceneWithUi {
        return Err(rhapsody::Error::invalid_usage(
            "native MetaCommander screenshot purpose must be 4",
        ));
    }
    enqueue(world, request)?;
    world
        .resource_mut::<Requests>()
        .active
        .last_mut()
        .expect("just inserted")
        .remaining = 1;
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

#[derive(Default)]
enum BeautyShotState {
    #[default]
    Idle,
    Settling(u8),
}

/// Original screenshot producer family. The feedback/T13 branch is deliberately
/// excluded. All remaining canonical resources are mandatory, not defaults.
fn gather(
    mut events: (
        EventReader<country_core::systems::save::save_game::SavingGameComplete>,
        EventReader<EscapeMenuJustOpened>,
        EventReader<country_core::resources::photo_mode::PhotoModeCmd>,
    ),
    actions: Res<country_core::input::action_set::ActionSetStates>,
    mode: Res<country_core::systems::mode_manager::AppMode>,
    menu: Res<EscMenuState>,
    settings: Res<country_core::game_config::GameConfig>,
    mut next: ResMut<NextEscMenuState>,
    mut config: ResMut<FrameRenderConfig>,
    mut clock: ResMut<country_core::resources::time::GameTime>,
    mut gallery: ResMut<LutGalleryState>,
    mut commander: ResMut<country_core::meta_commander::MetaCommander>,
    mut beauty: Local<BeautyShotState>,
    mut requests: ResMut<Requests>,
    steam_requests: Option<Res<country_core::systems::steam::SteamScreenshotRequests>>,
    steam_client: Option<Res<country_core::systems::steam::SteamClientResource>>,
) -> bevy_ecs::error::Result {
    reap_writers(&mut requests.writers);
    if requests.closing {
        return Ok(());
    }
    // Original PhotoModeCmd reader stops at the first tag4. Later events remain
    // pending for its next read, so do not replace this with any/count.
    let mut photo_requested = false;
    for command in events.2.read() {
        if command.0 == 4 {
            photo_requested = true;
            break;
        }
    }
    let direct_request = photo_requested || actions.digital[25].just_pressed();
    let capture = match *beauty {
        BeautyShotState::Idle if direct_request && mode.native_tag() == 19 => {
            clock.paused = true;
            *beauty = BeautyShotState::Settling(1);
            false
        }
        BeautyShotState::Idle => direct_request,
        BeautyShotState::Settling(n) if n < 16 => {
            config.flag_20 = true;
            *beauty = BeautyShotState::Settling(n + 1);
            false
        }
        BeautyShotState::Settling(16) => {
            *beauty = BeautyShotState::Settling(17);
            true
        }
        BeautyShotState::Settling(_) => {
            clock.paused = false;
            config.flag_20 = false;
            *beauty = BeautyShotState::Idle;
            direct_request
        }
    };
    if capture {
        use country_core::game_config::ScreenshotFormat;
        let (extension, quality) = match settings.game.screenshot_format {
            ScreenshotFormat::Jpeg => ("jpg", 85),
            ScreenshotFormat::Png => ("png", 100),
        };
        let path = paths::config().join("screenshots").join(format!(
            "{}.{}",
            utils::file::date_time_string_for_filename(),
            extension
        ));
        let purpose = if mode.native_tag() == 19 { 3 } else { 4 };
        push_file(&mut requests, path, purpose, None, quality, 3);
    }
    if actions.digital[26].just_pressed() {
        push_file(
            &mut requests,
            std::path::PathBuf::from("grading_reference.png"),
            0,
            None,
            100,
            3,
        );
    }
    // Native gather next contains feedback capture/state production; excluded.
    // LUT side reads FrameRenderConfig +4 = extent_0.height (not extent_8).
    if matches!(*menu, EscMenuState::Photomode(false)) {
        let side = ((config.extent_0[1] as f32 * 0.4) as u32).max(1);
        let slot = Arc::new(parking_lot::Mutex::new(None));
        gallery.preview = Some(slot.clone());
        next.set(EscMenuState::Photomode(true));
        requests
            .active
            .push(shared(1, Some([side, side]), Destination::Gallery(slot)));
    }
    if events.1.read().count() != 0 {
        requests.active.push(shared(2, None, Destination::Menu));
    }
    // Original EventReader::last consumes the complete batch, then only the
    // last saved folder gets both images. No active-session fallback.
    if let Some(saved) = events.0.read().last() {
        push_file(
            &mut requests,
            saved.path.join("screenshot.jpg"),
            2,
            Some([512, 512]),
            85,
            3,
        );
        let [width, height] = config.extent_0;
        let thumbnail_width = ((width as f32 * 64.0 / height.max(1) as f32).max(1.0)) as u32;
        push_file(
            &mut requests,
            saved.path.join("loading_background.jpg"),
            2,
            Some([thumbnail_width, 64]),
            95,
            3,
        );
    }
    // Same canonical Meta queue. Its update already created parent directories
    // and pushed the command back; here it is consumed before pushing active.
    if let Some(path) = commander.consume_owned_screenshot()? {
        push_file(&mut requests, path, 4, None, 85, 1);
    }
    // The current Steam resource exposes only a pending bool, unlike B's owned
    // request/result Vec. Keep this request pending and expose the real SDK gap.
    // The file/beauty/save/Meta producers above still perform their actual work.
    if steam_requests.is_some_and(|r| r.pending)
        || capture && steam_client.is_some_and(|r| r.client.is_some())
    {
        return Err(anyhow::anyhow!("original Steam screenshot request/result queue and SDK upload consumer are not recovered").into());
    }
    Ok(())
}
fn push_file(
    owner: &mut Requests,
    path: std::path::PathBuf,
    purpose: u8,
    dimensions: Option<[u32; 2]>,
    jpeg_quality: u8,
    remaining: u32,
) {
    owner.active.push(Request {
        purpose,
        dimensions,
        destination: Destination::Pixels(ScreenshotSink::File { path, jpeg_quality }),
        completion: None,
        capture: Capture::Awaiting,
        extent: [0; 2],
        hurry: false,
        remaining,
    });
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
        let target_id = frame.import_texture(&texture, 0x11)?;
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
