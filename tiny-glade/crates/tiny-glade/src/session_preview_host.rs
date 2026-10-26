//! B session-selection GPU/file host. The existing core algorithm owns pool
//! assignment, refresh retry, native guards, all draws and their ordering.
use crate::{
    render_bridge::CoreGraphFrame,
    render_host::{HostTables, TextureHost},
};
use bevy_ecs::{
    change_detection::{DetectChangesMut, Mut},
    prelude::Resource,
    schedule::IntoScheduleConfigs,
    world::World,
};
use country_core::{
    asset_libraries::{
        shader_library::AssetShaderLibrary,
        texture_library::{AssetTextureLibrary, TextureLibraryDevice, TextureUpdate},
    },
    game_config::GameConfig,
    render::{
        load_texture::{boundary::TextureDevice, parse_compiled_image},
        pass_builder::{
            draw_builder::{BindTexture, DrawBuilder},
            draw_stream::{DrawPacket, PassHost},
            framebuffer_binding::AttachmentTexture,
        },
    },
    resources::{
        gui::{EscMenuState, LoadMenuThumbnailMetrics},
        session::SavedSessionList,
        texture_presets::TextureFromDisk,
    },
    rhapsody_contract::{Handle, ManagedHandle},
    rhapsody_types::{Texture, TextureDesc},
    systems::render_loop::{
        draw_passes::{
            d_main_support::{ColorSpec, DepthSpec, DrawPassCtx},
            ui_2d_pass::Vec2Uniform,
        },
        session_selection::{self as native, *},
    },
};
use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
    sync::{Arc, Mutex, OnceLock},
};

#[derive(Resource, Clone)]
struct PreviewGpu(Arc<PreviewGpuOwner>);
struct PreviewGpuOwner {
    textures: Arc<TextureHost>,
    owned: Vec<Texture>,
}
impl Drop for PreviewGpuOwner {
    fn drop(&mut self) {
        for texture in &self.owned {
            self.textures.release(texture);
        }
    }
}

pub fn install(app: &mut bevy_app::App, hosts: HostTables) {
    app.init_resource::<ThumbnailTextures>();
    app.add_systems(
        bevy_app::PreStartup,
        (move |world: &mut World| {
            initialize(world, &hosts).expect("original SessionThumbnailsPlugin startup");
        })
        .after(crate::gameplay_startup::GameplayGpuPreStartup::Inputs),
    );
}

fn initialize(world: &mut World, hosts: &HostTables) -> anyhow::Result<()> {
    use country_core::render::load_texture::AlignedFile;
    anyhow::ensure!(
        !world.contains_resource::<PreviewGpu>(),
        "session thumbnail startup ran twice"
    );
    let shaders = world
        .resource::<crate::run_game::ApplicationShaderLibrary>()
        .0
        .clone();
    for (uri, name) in STARTUP_SHADERS {
        country_core::startup::load_shaders::load_shader_into_library(
            &shaders,
            hosts.shaders.as_ref(),
            uri,
            name,
        );
    }
    // Actual NameHash-to-packaged-name matches recorded in round12/render.
    // Keep this producer independent of the common texture whitelist.
    let root = paths::installed().join("compiled-assets/textures");
    for name in [
        "menu/craft_paper",
        "menu/new_glade_tile",
        "cloth_ferris",
        "brush/2",
        "menu/back",
    ] {
        let hash = country_core::name_hash::NameHash::compute(name);
        if world
            .resource::<AssetTextureLibrary>()
            .get_handle_and_texture_by_name(hash)
            .is_some()
        {
            continue;
        }
        let path = root.join(name).with_extension("texture");
        let file = AlignedFile::read(&path)?;
        let image = parse_compiled_image(file.bytes(), &path);
        let texture = hosts
            .textures
            .create_texture(name, &image.desc, image.data)?;
        world
            .resource_mut::<AssetTextureLibrary>()
            .add(Some(name.to_owned()), texture);
    }
    let mut owner = PreviewGpuOwner {
        textures: hosts.textures.clone(),
        owned: Vec::new(),
    };
    let mut pool = world.resource_mut::<ThumbnailTextures>();
    for _ in 0..THUMBNAIL_POOL_SIZE {
        let desc = TextureDesc::new_2d(THUMBNAIL_EXTENT, THUMBNAIL_FORMAT).sampled();
        match hosts
            .textures
            .create_texture(THUMBNAIL_TEXTURE_NAME, &desc, &[])
        {
            Ok(texture) => {
                pool.pool.push(PooledThumbnail {
                    path: None,
                    modified: 0,
                    texture: country_core::systems::render_loop::menu_background::ManagedTexture(
                        texture.id,
                    ),
                    assigned: false,
                });
                owner.owned.push(texture);
            }
            Err(error) => {
                log::error!(target:"country_core::systems::render_loop::session_selection", "{CREATE_FAILED_MESSAGE}{error}")
            }
        }
    }
    world.insert_resource(PreviewGpu(Arc::new(owner)));
    Ok(())
}

#[derive(Debug)]
enum Command {
    U2(u64, [u32; 2]),
    F32(u64, f32),
    V2(u64, [f32; 2]),
    U32(u64, u32),
    Bool(u64, bool),
    Texture(u64, u64, u8),
    Disk(u64, u64, u8),
    State(u8, u8, u8),
}
#[derive(Debug)]
struct Draw {
    shader: u64,
    commands: Vec<Command>,
    vertices: Option<u32>,
}
struct Host<'a> {
    mode: SelectionModeView,
    sessions: Vec<SessionRow>,
    thumbnails: Mut<'a, ThumbnailTextures>,
    shaders: Arc<AssetShaderLibrary>,
    gpu: Arc<PreviewGpuOwner>,
    demo: bool,
    extent: [u32; 2],
    draws: Vec<Draw>,
    pass_started: bool,
    rendered: bool,
}
impl Host<'_> {
    fn command(&mut self, command: Command) {
        self.draws
            .last_mut()
            .expect("native draw began")
            .commands
            .push(command);
    }
}
impl SessionPreviewHost for Host<'_> {
    fn app_mode(&self) -> SelectionModeView {
        self.mode.clone()
    }
    fn raster_shader(&self, name: &str) -> u64 {
        self.shaders
            .get_raster_handle_by_name(name)
            .expect("original session shader")
            .id()
            .raw()
    }
    fn sessions(&self) -> Vec<SessionRow> {
        self.sessions.clone()
    }
    fn thumbnails(&mut self) -> &mut ThumbnailTextures {
        &mut self.thumbnails
    }
    fn mark_thumbnails_changed(&mut self) {
        self.thumbnails.set_changed();
    }
    fn demo_mode_override(&self) -> bool {
        self.demo
    }
    fn modified_filetime(&self, path: &Path) -> Result<i64, RefreshError> {
        modified_filetime(path)
    }
    fn decode_image(&self, path: &Path) -> DecodeOutcome {
        decode_image(path)
    }
    fn update_texture(
        &mut self,
        texture: &country_core::systems::render_loop::menu_background::ManagedTexture,
        image: &RgbaImage,
    ) -> Result<(), String> {
        let texture = self
            .gpu
            .owned
            .iter()
            .find(|owner| owner.id == texture.0)
            .ok_or_else(|| "thumbnail texture does not belong to this App's pool".to_owned())?;
        self.gpu
            .textures
            .update_texture(
                texture,
                &TextureUpdate {
                    data: image.pixels.clone(),
                    buffer_offset: 0,
                    mip_level: 0,
                    extent: [image.width, image.height, 1],
                    array_layers: 1,
                    access: 0x22,
                },
            )
            .map_err(|error| error.to_string())
    }
    fn path_tail(&self, path: &Path) -> String {
        let mut components: Vec<_> = path.components().rev().take(3).collect();
        components.reverse();
        components
            .into_iter()
            .collect::<PathBuf>()
            .display()
            .to_string()
    }
    fn insert_error_key(&mut self, key: String) -> bool {
        static ERRORS: OnceLock<Mutex<BTreeSet<String>>> = OnceLock::new();
        ERRORS
            .get_or_init(|| Mutex::new(BTreeSet::new()))
            .lock()
            .expect("Mutex was poisonned")
            .insert(key)
    }
    fn log_error(&mut self, message: &str) {
        log::error!(target:"country_core::systems::render_loop::session_selection","{message}");
    }
    fn log_debug(&mut self, message: &str) {
        log::debug!(target:"country_core::systems::render_loop::session_selection","{message}");
    }
    fn target_extent(&self) -> [u32; 2] {
        self.extent
    }
    fn begin_pass(&mut self, name: &str) {
        assert_eq!(name, PASS_NAME);
        assert!(!self.pass_started);
        self.pass_started = true;
    }
    fn begin_draw(&mut self, shader: u64) {
        self.draws.push(Draw {
            shader,
            commands: Vec::new(),
            vertices: None,
        });
    }
    fn optional_uniform_u32x2(&mut self, k: u64, v: [u32; 2]) {
        self.command(Command::U2(k, v));
    }
    fn uniform_f32(&mut self, k: u64, v: f32) {
        self.command(Command::F32(k, v));
    }
    fn uniform_vec2(&mut self, k: u64, v: [f32; 2]) {
        self.command(Command::V2(k, v));
    }
    fn uniform_u32(&mut self, k: u64, v: u32) {
        self.command(Command::U32(k, v));
    }
    fn uniform_bool(&mut self, k: u64, v: bool) {
        self.command(Command::Bool(k, v));
    }
    fn bind_texture(
        &mut self,
        t: &country_core::systems::render_loop::menu_background::ManagedTexture,
        k: u64,
        a: u8,
    ) {
        self.command(Command::Texture(t.0, k, a));
    }
    fn bind_disk_texture(&mut self, h: u64, k: u64, a: u8) {
        self.command(Command::Disk(h, k, a));
    }
    fn edit_raster_state(&mut self, a: u8, b: u8, c: u8) {
        self.command(Command::State(a, b, c));
    }
    fn draw_no_vao(&mut self, n: u32) {
        let draw = self.draws.last_mut().expect("native draw");
        assert!(draw.vertices.replace(n).is_none());
    }
    fn render_pass(&mut self) {
        assert!(self.pass_started);
        self.rendered = true;
    }
}

fn modified_filetime(path: &Path) -> Result<i64, RefreshError> {
    let metadata = std::fs::metadata(path).map_err(|error| RefreshError {
        context: "Reading metadata",
        source: error.to_string(),
    })?;
    let modified = metadata.modified().map_err(|error| RefreshError {
        context: "Checking modification time",
        source: error.to_string(),
    })?;
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        let _ = modified;
        Ok(metadata.last_write_time() as i64)
    }
    #[cfg(not(windows))]
    {
        let delta = match modified.duration_since(std::time::UNIX_EPOCH) {
            Ok(d) => i128::from(d.as_secs()) * 10_000_000 + i128::from(d.subsec_nanos() / 100),
            Err(e) => {
                let d = e.duration();
                -(i128::from(d.as_secs()) * 10_000_000 + i128::from(d.subsec_nanos() / 100))
            }
        };
        i64::try_from(i128::from(FILETIME_UNIX_EPOCH) + delta).map_err(|error| RefreshError {
            context: "Checking modification time",
            source: error.to_string(),
        })
    }
}
fn decode_image(path: &Path) -> DecodeOutcome {
    let decoded = image::io::Reader::open(path)
        .map_err(image::ImageError::IoError)
        .and_then(|reader| reader.decode());
    match decoded {
        Ok(image) => {
            let image = image.to_rgba8();
            DecodeOutcome::Image(RgbaImage {
                width: image.width(),
                height: image.height(),
                pixels: image.into_raw(),
            })
        }
        // B 0x1408f7ab2 compares Result<DynamicImage> Err tag 10 and
        // ImageError::Decoding tag 0, then retries next frame without logging.
        Err(image::ImageError::Decoding(_)) => DecodeOutcome::Skip,
        Err(error) => DecodeOutcome::Failed(error.to_string()),
    }
}
fn carousel(metrics: LoadMenuThumbnailMetrics) -> CarouselState {
    CarouselState {
        unobserved_14: metrics
            .highlighted_index
            .map_or(0, |index| 1u64 | (u64::from(index) << 32)),
        origin: metrics.origin.to_array(),
        size: metrics.cell_size.to_array(),
        spacing: metrics.step.x,
        unobserved_30: metrics.step.y.to_bits(),
        scroll: metrics.scroll_offset.x,
        unobserved_38: [metrics.scroll_offset.y.to_bits(), metrics.cell_count],
        mode_tag: metrics.cells_per_line,
        active: metrics.column_major,
    }
}


/// Render-thread extraction: what this module reads from the World.
pub fn render_extract(m: &mut impl country_core::render::extract::RenderExtract) {
    m.resource::<ThumbnailTextures>();
    m.resource::<PreviewGpu>();
    m.resource::<crate::run_game::ApplicationShaderLibrary>();
    m.asset_library::<AssetTextureLibrary>();
    m.resource::<EscMenuState>();
    m.resource::<SavedSessionList>();
    m.resource::<GameConfig>();
    m.resource::<bevy_time::Time>();
}

#[cfg(test)]
mod file_tests {
    use super::*;
    #[test]
    fn disk_decode_retains_extent_and_metadata_uses_real_filetime() {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("preview.png");
        let pixels = image::RgbaImage::from_raw(3, 2, (0..24).collect()).unwrap();
        pixels.save(&path).unwrap();
        let DecodeOutcome::Image(decoded) = decode_image(&path) else {
            panic!("real PNG decode");
        };
        assert_eq!((decoded.width, decoded.height), (3, 2));
        assert_eq!(decoded.pixels, pixels.into_raw());
        let time = modified_filetime(&path).unwrap();
        #[cfg(windows)]
        {
            use std::os::windows::fs::MetadataExt;
            assert_eq!(
                time,
                std::fs::metadata(&path).unwrap().last_write_time() as i64
            );
        }
        assert!(time > FILETIME_UNIX_EPOCH);
        let missing = root.path().join("missing.jpg");
        assert!(matches!(decode_image(&missing), DecodeOutcome::Failed(_)));
        assert_eq!(
            modified_filetime(&missing).unwrap_err().context,
            "Reading metadata"
        );
    }
    #[test]
    fn carousel_projects_the_actual_shared_gui_record() {
        let metrics = LoadMenuThumbnailMetrics {
            highlighted_index: Some(4),
            origin: glam::Vec2::new(0.4, 0.5),
            cell_size: glam::Vec2::splat(0.25),
            step: glam::Vec2::splat(0.3),
            scroll_offset: glam::Vec2::new(2., 0.),
            cell_count: 9,
            cells_per_line: 1,
            column_major: true,
        };
        let state = carousel(metrics);
        assert_eq!(state.unobserved_14, 1 | 4 << 32);
        assert_eq!(state.mode_tag, 1);
        assert!(state.active);
        assert_eq!(state.scroll, 2.);
        assert_eq!(state.spacing, 0.3);
    }
}

pub fn render(
    frame: &mut CoreGraphFrame<'_>,
    world: &mut World,
    target: AttachmentTexture,
) -> rhapsody::Result<()> {
    let menu = world
        .get_resource::<EscMenuState>()
        .ok_or_else(|| missing("EscMenuState"))?;
    let view = menu.view();
    if mode_hides_selection(u64::from(view.tag)) {
        return Ok(());
    }
    let EscMenuState::Load(menu) = menu else {
        return Err(rhapsody::Error::unsupported(
            "native session preview active menu variant has no canonical load-menu projection",
        ));
    };
    let mode = SelectionModeView {
        state: u64::from(view.tag),
        selected_slot: menu.hovered_index.unwrap_or(0) as u64,
        // Native copies the canonical locked metrics to its stack and releases
        // the lock. This Arc is an adapter for that one immutable snapshot only.
        carousel: Arc::new(parking_lot::Mutex::new(carousel(
            *menu.thumbnail_metrics.lock(),
        ))),
    };
    let sessions = world
        .get_resource::<SavedSessionList>()
        .ok_or_else(|| missing("SavedSessionList"))?
        .sessions
        .iter()
        .map(|session| SessionRow {
            screenshot_path: session.screenshot_path(),
            flag: !session.readiness.is_ok(),
        })
        .collect();
    let gpu = world
        .get_resource::<PreviewGpu>()
        .ok_or_else(|| missing("SessionThumbnailsPlugin startup"))?
        .0
        .clone();
    if !Arc::ptr_eq(&gpu.textures, &frame.hosts().textures) {
        return Err(missing("same-App thumbnail TextureHost"));
    }
    let shaders = frame.shader_library();
    for name in RENDER_SHADER_LOOKUP_ORDER {
        if shaders.get_raster_handle_by_name(name).is_none() {
            return Err(missing(name));
        }
    }
    let demo = world.resource::<GameConfig>().dev.demo;
    let (draws, pass_started, rendered) =
        world.resource_scope(|_, thumbnails: Mut<ThumbnailTextures>| {
            let mut host = Host {
                mode,
                sessions,
                thumbnails,
                shaders: shaders.clone(),
                gpu: gpu.clone(),
                demo,
                extent: [target.extent.0, target.extent.1],
                draws: Vec::new(),
                pass_started: false,
                rendered: false,
            };
            native::render_session_selection_previews(&mut host);
            (host.draws, host.pass_started, host.rendered)
        });
    if !pass_started {
        return Ok(());
    }
    if !rendered {
        return Err(missing("SessionPreviewHost::render_pass"));
    }
    let elapsed = world
        .get_resource::<bevy_time::Time>()
        .map(|time| time.elapsed())
        .unwrap_or_default();
    frame.with_raster(|host| {
        let default_state = host.default_raster_state();
        let mut factory = || -> Box<dyn PassHost> { Box::new(host.sibling()) };
        let lookup = super::LibraryShaders(shaders.as_ref());
        let mut ctx = DrawPassCtx::new(world, elapsed, &mut factory, &lookup);
        let mut pass = ctx.raster_pass(
            PASS_NAME,
            // B 0x1408f83b0: color tag0, carried in r9 at 0x1408f8404.
            // Preview quads blend over the preceding menu/world composition.
            &[ColorSpec::tag0(target)],
            DepthSpec::None,
            target,
        );
        for draw in draws {
            let shader = Handle(ManagedHandle(draw.shader), std::marker::PhantomData);
            let packet = DrawPacket::new(shader, [0, 0], default_state);
            let mut builder = DrawBuilder::new(&mut pass, packet, 0);
            for command in draw.commands {
                builder = match command {
                    Command::U2(k, v) => builder.optional_uniform(k, v),
                    Command::F32(k, v) => builder.uniform(k, v),
                    Command::V2(k, v) => builder.uniform(k, Vec2Uniform(glam::Vec2::from_array(v))),
                    Command::U32(k, v) => builder.uniform(k, v),
                    Command::Bool(k, v) => builder.uniform(k, v),
                    Command::Disk(h, k, a) => builder.bind_texture_to_draw_builder(
                        BindTexture::FromDisk(TextureFromDisk(h)),
                        k,
                        a,
                        None,
                    ),
                    Command::Texture(id, k, a) => {
                        let owner = gpu
                            .owned
                            .iter()
                            .find(|texture| texture.id == id)
                            .expect("native pool owns draw texture");
                        let handle = ManagedHandle(id);
                        let desc = AttachmentTexture::from(owner);
                        builder.bind_texture_to_draw_builder(
                            BindTexture::Raw {
                                texture: &handle,
                                desc: &desc,
                            },
                            k,
                            a,
                            None,
                        )
                    }
                    Command::State(mask, keep, set) => {
                        let state = &mut builder.packet_mut().state.0;
                        let bytes = state.to_le_bytes();
                        *state = u32::from_le_bytes([
                            bytes[0] & mask,
                            bytes[1],
                            bytes[2],
                            (bytes[3] & keep) | set,
                        ]);
                        builder
                    }
                };
            }
            builder.draw_no_vao(draw.vertices.expect("native draw terminator"));
        }
        pass.render();
        Ok(())
    })
}
fn missing(name: &str) -> rhapsody::Error {
    rhapsody::Error::invalid_usage(format!("original session preview: missing {name}"))
}
