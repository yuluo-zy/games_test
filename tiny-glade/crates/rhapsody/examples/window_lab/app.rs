use super::{
    fixture::{self, HEIGHT, Playlist, Result, WIDTH},
    win32,
};
use ash::vk::{self, Handle};
use rhapsody::{
    raw::{
        buffer::{Buffer, BufferDesc},
        command_buffer::VulkanCommandPool,
        device::VulkanDevice,
        init::{DeviceSelectionPolicy, GraphicsInstance},
        instance::{InstanceBuilder, ValidationMessenger},
        physical_device::enumerate_physical_devices,
        swapchain::{Surface, Swapchain},
        texture::{Texture, TextureDesc},
    },
    rg::{
        executor::PassContext,
        vulkan::{AccessTable, GpuResource, ResourceSpec, VulkanGraphBackend, VulkanGraphSession},
    },
};
use serde::Serialize;
use std::{
    collections::BTreeSet,
    ffi::{CStr, CString},
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, Instant},
};

const HOST_READ: u8 = 0x26;
const HOST_WRITE: u8 = 0x35;
const TRANSFER_READ: u8 = 0x25;
const TRANSFER_WRITE: u8 = 0x34;
const TIMEOUT: u64 = 5_000_000_000;

struct Options {
    playlist: PathBuf,
    frames: Option<u64>,
    fps: u32,
    script: bool,
    report: PathBuf,
    capture: Option<PathBuf>,
    require_validation: bool,
}
impl Options {
    fn parse() -> Result<Option<Self>> {
        let mut options = Self {
            playlist: fixture::default_playlist(),
            frames: None,
            fps: 60,
            script: false,
            report: Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../target/window-lab/report.json"),
            capture: None,
            require_validation: false,
        };
        let mut args = std::env::args().skip(1);
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--playlist" => {
                    options.playlist = args.next().ok_or("missing playlist path")?.into()
                }
                "--frames" => {
                    options.frames = Some(args.next().ok_or("missing frame count")?.parse()?)
                }
                "--fps" => options.fps = args.next().ok_or("missing fps")?.parse()?,
                "--report" => options.report = args.next().ok_or("missing report path")?.into(),
                "--capture" => {
                    options.capture = Some(args.next().ok_or("missing capture path")?.into())
                }
                "--script" => options.script = true,
                "--require-validation" => options.require_validation = true,
                "--help" | "-h" => {
                    println!(
                        "Window Lab V0\n--playlist FILE --frames N --fps 1..240 --script --report FILE --capture FILE.ppm --require-validation\nKeys: Space pause, N next clip, Right step, Esc quit\nPPM/P3/P6 images are supported; no video codec is required."
                    );
                    return Ok(None);
                }
                _ => return Err(format!("unknown option: {arg}").into()),
            }
        }
        if options.fps == 0 || options.fps > 240 || options.frames == Some(0) {
            return Err("fps must be 1..=240 and frames must be positive".into());
        }
        if options.script && options.frames.is_some_and(|n| n < 180) {
            return Err("--script requires at least 180 frames".into());
        }
        Ok(Some(options))
    }
}

#[derive(Default, Serialize)]
struct Report {
    schema_version: u32,
    milestone: String,
    status: String,
    command: Vec<String>,
    git_revision_at_launch: Option<String>,
    git_dirty_at_launch: Option<bool>,
    device: String,
    driver_version_raw: u32,
    api_version_raw: u32,
    enabled_device_extensions: Vec<String>,
    validation_enabled: bool,
    validation_errors: Vec<String>,
    validation_warnings: Vec<String>,
    validation_error_count: usize,
    validation_warning_count: usize,
    fixture_hash: String,
    requested_frames: Option<u64>,
    presented_frames: u64,
    pixel_verified_frames: u64,
    source_hash: String,
    readback_hash: String,
    visited_clips: BTreeSet<String>,
    resize_events: u64,
    recreate_events: u64,
    minimized_iterations: u64,
    script_completed: bool,
    pause_actions: u64,
    next_actions: u64,
    step_actions: u64,
    cpu_frame_ms_p50: f64,
    cpu_frame_ms_p95: f64,
    cpu_frame_ms_p99: f64,
    elapsed_seconds: f64,
    gpu_time_ms: Option<f64>,
    peak_gpu_bytes: Option<u64>,
    window_visual_check: String,
    limitations: Vec<String>,
    failure: Option<String>,
}

struct Lab {
    // Field drop order preserves session/resources before device and debug messenger.
    session: VulkanGraphSession,
    image: Arc<Texture>,
    upload: Arc<Buffer>,
    readback: Arc<Buffer>,
    tokens: [u64; 3],
    pool: Arc<VulkanCommandPool>,
    chain: Arc<Swapchain>,
    device: Arc<VulkanDevice>,
    messenger: ValidationMessenger,
}
impl Lab {
    fn new(window: &win32::Window, report: &mut Report, require_validation: bool) -> Result<Self> {
        let mut builder = InstanceBuilder {
            application_name: CString::new("Rhapsody Window Lab")?,
            extensions: vec![
                CString::new("VK_KHR_surface")?,
                CString::new("VK_KHR_win32_surface")?,
            ],
            ..Default::default()
        };
        report.validation_enabled = builder.enable_validation()?;
        if require_validation && !report.validation_enabled {
            return Err("Khronos validation layer is required but unavailable".into());
        }
        let instance = builder.build()?;
        let messenger = ValidationMessenger::new(&instance)?;
        let surface = unsafe {
            Surface::from_win32(instance.clone(), window.instance as _, window.handle as _)
        }?;
        let graphics = GraphicsInstance {
            devices: enumerate_physical_devices(instance.clone())?
                .into_iter()
                .map(Arc::new)
                .collect(),
            instance,
        };
        let device = graphics
            .available_for_surface(Some(surface.handle), DeviceSelectionPolicy::default())?
            .create_device(&graphics)
            .map_err(|errors| errors.join("\n"))?;
        let properties = unsafe {
            device
                .instance
                .instance
                .get_physical_device_properties(device.physical)
        };
        report.device = unsafe { CStr::from_ptr(properties.device_name.as_ptr()) }
            .to_string_lossy()
            .into_owned();
        report.driver_version_raw = properties.driver_version;
        report.api_version_raw = properties.api_version;
        report.enabled_device_extensions = device.enabled_extensions.clone();
        let (width, height) = window.extent();
        let chain = Arc::new(unsafe {
            Swapchain::new(
                device.clone(),
                surface,
                vk::Extent2D { width, height },
                true,
                vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_DST,
            )
        }?);
        let features = unsafe {
            device
                .instance
                .instance
                .get_physical_device_format_properties(device.physical, chain.format)
        };
        if !features
            .optimal_tiling_features
            .contains(vk::FormatFeatureFlags::BLIT_DST)
        {
            return Err("swapchain format does not support blitting".into());
        }
        let source_features = unsafe {
            device
                .instance
                .instance
                .get_physical_device_format_properties(device.physical, vk::Format::R8G8B8A8_UNORM)
        };
        if !source_features
            .optimal_tiling_features
            .contains(vk::FormatFeatureFlags::BLIT_SRC)
        {
            return Err("RGBA8 source does not support blitting".into());
        }
        let mut desc = TextureDesc::new_2d(WIDTH, HEIGHT, vk::Format::R8G8B8A8_UNORM);
        desc.usage = vk::ImageUsageFlags::TRANSFER_SRC | vk::ImageUsageFlags::TRANSFER_DST;
        let image = Arc::new(Texture::new(device.clone(), &desc)?);
        let bytes = u64::from(WIDTH * HEIGHT * 4);
        let upload = Arc::new(Buffer::new(
            device.clone(),
            &BufferDesc::new_cpu_to_gpu(bytes, vk::BufferUsageFlags::TRANSFER_SRC),
        )?);
        let readback = Arc::new(Buffer::new(
            device.clone(),
            &BufferDesc::new_gpu_to_cpu(bytes, vk::BufferUsageFlags::TRANSFER_DST),
        )?);
        let mut backend = VulkanGraphBackend::new(device.clone(), AccessTable::original())?;
        let tokens = [
            backend.bind_import(GpuResource::Buffer(upload.clone())),
            backend.bind_import(GpuResource::Image(image.clone())),
            backend.bind_import(GpuResource::Buffer(readback.clone())),
        ];
        let session = VulkanGraphSession::new(backend);
        let pool = VulkanCommandPool::new(device.clone())?;
        Ok(Self {
            session,
            image,
            upload,
            readback,
            tokens,
            pool,
            chain,
            device,
            messenger,
        })
    }

    fn graph(&mut self, pixels: &[u8], first: bool, output: &mut [u8]) -> Result<()> {
        // V0 is deliberately serial: the previous presentation fence has completed here.
        self.upload.write_bytes(0, pixels)?;
        let mut graph = self.session.begin_frame()?;
        let upload = graph.import_unowned(
            "lab.upload",
            ResourceSpec::Buffer(self.upload.desc).descriptor(),
            self.tokens[0],
            HOST_WRITE,
        )?;
        let image = graph.import_unowned(
            "lab.image",
            ResourceSpec::Image(self.image.desc).descriptor(),
            self.tokens[1],
            if first { 0 } else { TRANSFER_READ },
        )?;
        let readback = graph.import_unowned(
            "lab.readback",
            ResourceSpec::Buffer(self.readback.desc).descriptor(),
            self.tokens[2],
            HOST_READ,
        )?;
        {
            let device = self.device.clone();
            let source = self.upload.clone();
            let target = self.image.clone();
            let mut pass = graph.add_pass("upload fixture");
            pass.add_resource_access(upload, TRANSFER_READ, 0)?;
            pass.add_resource_access(image, TRANSFER_WRITE, 0)?;
            pass.render_with_context(move |ctx: PassContext<'_>| {
                ctx.keep_alive(source.clone());
                ctx.keep_alive(target.clone());
                unsafe {
                    device.device.cmd_copy_buffer_to_image(
                        vk::CommandBuffer::from_raw(ctx.command.handle()),
                        source.handle,
                        target.handle,
                        vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                        &[copy_region()],
                    );
                }
                true
            });
        }
        {
            let device = self.device.clone();
            let source = self.image.clone();
            let target = self.readback.clone();
            let mut pass = graph.add_pass("verify fixture");
            pass.add_resource_access(image, TRANSFER_READ, 0)?;
            pass.add_resource_access(readback, TRANSFER_WRITE, 0)?;
            pass.render_with_context(move |ctx: PassContext<'_>| {
                ctx.keep_alive(source.clone());
                ctx.keep_alive(target.clone());
                unsafe {
                    device.device.cmd_copy_image_to_buffer(
                        vk::CommandBuffer::from_raw(ctx.command.handle()),
                        source.handle,
                        vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                        target.handle,
                        &[copy_region()],
                    );
                }
                true
            });
        }
        graph.export(image, TRANSFER_READ)?;
        graph.export(readback, HOST_READ)?;
        let receipt = self.session.execute_frame(graph.build())?;
        self.session.wait_frame(receipt.frame, TIMEOUT)?;
        // Readback is exclusively owned by this loop and this frame's HOST_READ export has completed.
        unsafe { self.readback.read_bytes_after_completion(0, output) }?;
        if output != pixels {
            return Err("GPU readback differs from the deterministic CPU fixture".into());
        }
        Ok(())
    }

    fn present(&mut self, report: &mut Report) -> Result<bool> {
        let before = self.chain.handle;
        let Some(frame) = Swapchain::validate_swapchain_and_acquire_image(&mut self.chain)? else {
            return Ok(false);
        };
        if before != self.chain.handle {
            report.recreate_events += 1;
        }
        let mut command = self.pool.allocate()?;
        command.begin()?;
        command.keep_alive(self.image.clone());
        let range = vk::ImageSubresourceRange::default()
            .aspect_mask(vk::ImageAspectFlags::COLOR)
            .level_count(1)
            .layer_count(1);
        let layers = vk::ImageSubresourceLayers::default()
            .aspect_mask(vk::ImageAspectFlags::COLOR)
            .layer_count(1);
        let to_dst = vk::ImageMemoryBarrier::default()
            .image(frame.image())
            .old_layout(vk::ImageLayout::UNDEFINED)
            .new_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
            .dst_access_mask(vk::AccessFlags::TRANSFER_WRITE)
            .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .subresource_range(range);
        let to_present = vk::ImageMemoryBarrier::default()
            .image(frame.image())
            .old_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
            .new_layout(vk::ImageLayout::PRESENT_SRC_KHR)
            .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
            .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .subresource_range(range);
        let extent = frame.extent();
        let blit = vk::ImageBlit::default()
            .src_subresource(layers)
            .dst_subresource(layers)
            .src_offsets([
                vk::Offset3D::default(),
                vk::Offset3D {
                    x: WIDTH as i32,
                    y: HEIGHT as i32,
                    z: 1,
                },
            ])
            .dst_offsets([
                vk::Offset3D::default(),
                vk::Offset3D {
                    x: extent.width as i32,
                    y: extent.height as i32,
                    z: 1,
                },
            ]);
        unsafe {
            self.device.device.cmd_pipeline_barrier(
                command.handle,
                vk::PipelineStageFlags::TOP_OF_PIPE,
                vk::PipelineStageFlags::TRANSFER,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &[to_dst],
            );
            self.device.device.cmd_blit_image(
                command.handle,
                self.image.handle,
                vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                frame.image(),
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &[blit],
                vk::Filter::NEAREST,
            );
            self.device.device.cmd_pipeline_barrier(
                command.handle,
                vk::PipelineStageFlags::TRANSFER,
                vk::PipelineStageFlags::BOTTOM_OF_PIPE,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &[to_present],
            );
        }
        command.end()?;
        let pending = frame.submit_and_present(command, vk::PipelineStageFlags::TRANSFER)?;
        pending.wait(TIMEOUT)?;
        drop(pending);
        Ok(true)
    }
    fn drain_messages(&self, report: &mut Report) {
        for message in self.messenger.collector.all() {
            let text = format!("{}: {}", message.id_name, message.message);
            if message
                .severity
                .contains(vk::DebugUtilsMessageSeverityFlagsEXT::ERROR)
            {
                report.validation_error_count += 1;
                if report.validation_errors.len() < 32 {
                    report.validation_errors.push(text);
                }
            } else {
                report.validation_warning_count += 1;
                if report.validation_warnings.len() < 32 {
                    report.validation_warnings.push(text);
                }
            }
        }
        self.messenger.collector.clear();
    }
}
fn copy_region() -> vk::BufferImageCopy {
    vk::BufferImageCopy::default()
        .image_subresource(
            vk::ImageSubresourceLayers::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .layer_count(1),
        )
        .image_extent(vk::Extent3D {
            width: WIDTH,
            height: HEIGHT,
            depth: 1,
        })
}
fn ensure_parent(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent)?;
    }
    Ok(())
}

pub fn run() -> Result<()> {
    let Some(options) = Options::parse()? else {
        return Ok(());
    };
    let git = |args: &[&str]| -> Option<String> {
        let output = std::process::Command::new("git")
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .args(["-c", "safe.directory=D:/game"])
            .args(args)
            .output()
            .ok()?;
        output
            .status
            .success()
            .then(|| String::from_utf8_lossy(&output.stdout).trim().to_owned())
    };
    let mut report = Report { schema_version: 1, milestone: "M0 / baseline fixture player".into(),
        git_revision_at_launch: git(&["rev-parse", "HEAD"]),
        git_dirty_at_launch: git(&["status", "--porcelain", "--untracked-files=normal", "--", "."]).map(|s| !s.is_empty()),
        command: std::env::args().collect(), requested_frames: options.frames,
        window_visual_check: "pending; offscreen readback is verified separately".into(),
        limitations: vec!["V0 uses serial offscreen graph + separate legacy presentation submission".into(),
            "Per-frame CPU fixture generation and GPU readback are correctness checks, not renderer benchmarks".into(),
            "Present completion protocol remains the current backend's conservative implementation; M2 is not certified".into(),
            "GPU timestamps and memory budget counters are not instrumented in V0".into()], ..Default::default() };
    let started = Instant::now();
    let result = play(&options, &mut report);
    report.elapsed_seconds = started.elapsed().as_secs_f64();
    if let Err(error) = &result {
        report.failure = Some(error.to_string());
    }
    report.status = if result.is_ok() { "pass" } else { "fail" }.into();
    ensure_parent(&options.report)?;
    std::fs::write(&options.report, serde_json::to_vec_pretty(&report)?)?;
    println!(
        "Window Lab {}: {} presented / {} verified; validation errors: {}; report: {}",
        report.status,
        report.presented_frames,
        report.pixel_verified_frames,
        report.validation_error_count,
        options.report.display()
    );
    result
}

fn play(options: &Options, report: &mut Report) -> Result<()> {
    let playlist = Playlist::load(&options.playlist)?;
    report.fixture_hash = playlist.fingerprint.clone();
    let window = win32::Window::new()?;
    let mut lab = Lab::new(&window, report, options.require_validation)?;
    let mut pixels = vec![0; (WIDTH * HEIGHT * 4) as usize];
    let mut output = pixels.clone();
    let mut timings = Vec::new();
    let (mut clip, mut frame) = (0usize, 0u32);
    let mut paused = false;
    let mut extent = window.extent();
    let mut script_phase = 0;
    let mut restore_at = None;
    let start = Instant::now();
    let mut last_progress = Instant::now();
    let result = (|| -> Result<()> {
        while let Some(keys) = window.poll() {
            if options.frames.is_some_and(|n| report.presented_frames >= n) {
                break;
            }
            if options.frames.is_some() && last_progress.elapsed() > Duration::from_secs(30) {
                return Err("no presented frame for 30 seconds".into());
            }
            if keys & win32::PAUSE != 0 {
                paused = !paused;
                report.pause_actions += 1;
            }
            if keys & win32::NEXT != 0 {
                clip = (clip + 1) % playlist.clips.len();
                frame = 0;
                report.next_actions += 1;
            }
            if keys & win32::STEP != 0 {
                report.step_actions += 1;
            }
            if options.script {
                match script_phase {
                    0 if report.presented_frames >= 30 => {
                        window.resize(800, 500)?;
                        script_phase = 1;
                    }
                    1 if report.presented_frames >= 60 => {
                        window.minimize();
                        restore_at = Some(Instant::now() + Duration::from_millis(250));
                        script_phase = 2;
                    }
                    2 if restore_at.is_some_and(|at| Instant::now() >= at) => {
                        window.restore();
                        script_phase = 3;
                    }
                    3 if report.presented_frames >= 120 => {
                        window.resize(1100, 680)?;
                        script_phase = 4;
                    }
                    _ => {}
                }
            }
            let current_extent = window.extent();
            if current_extent != extent {
                extent = current_extent;
                lab.chain.resize(vk::Extent2D {
                    width: extent.0,
                    height: extent.1,
                })?;
                report.resize_events += 1;
            }
            if extent.0 == 0 || extent.1 == 0 {
                report.minimized_iterations += 1;
                std::thread::sleep(Duration::from_millis(10));
                continue;
            }
            let frame_start = Instant::now();
            playlist.render(clip, frame, &mut pixels);
            lab.graph(&pixels, report.pixel_verified_frames == 0, &mut output)?;
            report.pixel_verified_frames += 1;
            if !lab.present(report)? {
                continue;
            }
            report.presented_frames += 1;
            last_progress = Instant::now();
            report
                .visited_clips
                .insert(playlist.clips[clip].name.clone());
            lab.drain_messages(report);
            if report.validation_error_count > 0 {
                return Err("Vulkan validation reported errors (see report)".into());
            }
            // Bound samples even for an indefinitely open interactive player.
            if timings.len() == 10000 {
                timings.drain(..5000);
            }
            timings.push(frame_start.elapsed().as_secs_f64() * 1000.0);
            if report.presented_frames.is_multiple_of(15) {
                window.title(&format!("Rhapsody Window Lab V0 | {} | {} | clip {} frame {} | {:.1} fps | validation {} | Space/N/Right/Esc",
                    playlist.clips[clip].name, if paused { "PAUSED" } else { "PLAY" }, frame, report.presented_frames,
                    report.presented_frames as f64 / start.elapsed().as_secs_f64(), report.validation_enabled));
            }
            if !paused || keys & win32::STEP != 0 {
                frame += 1;
                if frame >= playlist.clips[clip].frames {
                    clip = (clip + 1) % playlist.clips.len();
                    frame = 0;
                }
            }
            let target = Duration::from_secs_f64(1.0 / options.fps as f64);
            if let Some(wait) = target.checked_sub(frame_start.elapsed()) {
                std::thread::sleep(wait);
            }
        }
        if options.frames.is_some_and(|n| report.presented_frames != n) {
            return Err("window closed before requested frame count".into());
        }
        report.script_completed = options.script
            && script_phase == 4
            && report.minimized_iterations > 0
            && report.resize_events >= 3;
        if options.script && !report.script_completed {
            return Err("window script did not finish".into());
        }
        if report.presented_frames == 0 {
            return Err("no frames presented".into());
        }
        if let Some(path) = &options.capture {
            ensure_parent(path)?;
            fixture::write_ppm(path, &output)?;
        }
        Ok(())
    })();
    // Collect diagnostics through teardown, while preserving the original failure.
    let collector = lab.messenger.collector.clone();
    let cleanup = lab
        .session
        .wait_submits_done(TIMEOUT)
        .map_err(|e| e.to_string());
    lab.drain_messages(report);
    drop(lab);
    for error in collector.errors() {
        report.validation_error_count += 1;
        if report.validation_errors.len() < 32 {
            report
                .validation_errors
                .push(format!("{}: {}", error.id_name, error.message));
        }
    }
    report.source_hash = format!("{:016x}", fixture::hash(&pixels));
    report.readback_hash = format!("{:016x}", fixture::hash(&output));
    timings.sort_by(f64::total_cmp);
    if !timings.is_empty() {
        let percentile = |p: f64| timings[((timings.len() - 1) as f64 * p).ceil() as usize];
        report.cpu_frame_ms_p50 = percentile(0.50);
        report.cpu_frame_ms_p95 = percentile(0.95);
        report.cpu_frame_ms_p99 = percentile(0.99);
    }
    result?;
    cleanup?;
    if report.validation_error_count > 0 {
        return Err("Vulkan validation reported teardown errors".into());
    }
    Ok(())
}
