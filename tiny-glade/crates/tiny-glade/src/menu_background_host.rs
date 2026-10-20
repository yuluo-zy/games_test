//! Original menu-background consumer on current per-App texture owners.
//! Native state/slot replacement is delegated to core, including early returns.
use crate::{
    render_bridge::CoreGraphFrame,
    render_host::{HostTables, TextureHost},
};
use bevy_ecs::{
    change_detection::{DetectChangesMut, Mut},
    prelude::Resource,
    world::World,
};
use country_core::{
    render::pass_builder::{
        draw_builder::{BindTexture, DrawBuilder},
        draw_stream::{DrawPacket, PassHost},
        framebuffer_binding::AttachmentTexture,
    },
    resources::gui::EscMenuState,
    rhapsody_contract::{Handle, ManagedHandle},
    rhapsody_types::Texture,
    systems::render_loop::{
        draw_passes::d_main_support::{ColorSpec, DepthSpec, DrawPassCtx, ShaderLookup},
        menu_background::{
            self as native, AppModeView, ManagedTexture, MenuBackgroundHost,
            MenuBackgroundResources,
        },
    },
};
use std::{collections::BTreeMap, sync::Arc};

struct CaptureOwner {
    host: Arc<TextureHost>,
    textures: parking_lot::Mutex<BTreeMap<u64, Texture>>,
}
impl CaptureOwner {
    fn release(&self, id: u64) {
        if let Some(texture) = self.textures.lock().remove(&id) {
            self.host.release(&texture);
        }
    }
}
impl Drop for CaptureOwner {
    fn drop(&mut self) {
        for texture in self.textures.get_mut().values() {
            self.host.release(texture);
        }
    }
}
#[derive(Resource, Clone)]
struct MenuCaptureGpu(Arc<CaptureOwner>);

pub fn install(app: &mut bevy_app::App, hosts: HostTables) {
    app.init_resource::<MenuBackgroundResources>();
    app.insert_resource(MenuCaptureGpu(Arc::new(CaptureOwner {
        host: hosts.textures,
        textures: parking_lot::Mutex::new(BTreeMap::new()),
    })));
    app.add_systems(
        bevy_app::PreStartup,
        move |library: bevy_ecs::prelude::Res<crate::run_game::ApplicationShaderLibrary>| {
            country_core::startup::load_shaders::load_shader_into_library(
                &library.0,
                hosts.shaders.as_ref(),
                "/blit/blit_stretch_decode_srgb.raster.hlsl",
                native::BLIT_SHADER_NAME,
            );
        },
    );
}

/// Transfer a real completed capture to the original shared pending slot.
/// The renderer consumes it only when the original menu gate permits drawing.
pub fn publish(world: &mut World, hosts: &HostTables, texture: Texture) -> rhapsody::Result<()> {
    let owner = world
        .get_resource::<MenuCaptureGpu>()
        .ok_or_else(|| missing("MenuBackgroundPlugin"))?
        .0
        .clone();
    if !Arc::ptr_eq(&owner.host, &hosts.textures) {
        return Err(missing("capture in the same App's TextureHost"));
    }
    let actual = hosts
        .textures
        .get(&texture)
        .ok_or_else(|| missing("actual capture texture"))?;
    if actual.desc.sample_count != 1
        || !actual
            .desc
            .usage
            .contains(ash::vk::ImageUsageFlags::SAMPLED)
    {
        return Err(missing("single-sample sampled capture"));
    }
    let id = texture.id;
    // Native ManagedTexture cloning retains the allocation. Publishing the
    // same namespace entry again must not retire the still-current owner.
    if world.resource::<MenuBackgroundResources>().current.as_ref().is_some_and(|current| current.0 == id) {
        return Ok(());
    }
    owner.textures.lock().insert(id, texture);
    let pending = world.resource::<MenuBackgroundResources>().shared.clone();
    let old = pending.lock().replace(ManagedTexture(id));
    if let Some(old) = old {
        if old.0 != id {
            owner.release(old.0);
        }
    }
    Ok(())
}

#[derive(Default)]
struct Draw {
    shader: u64,
    texture: Option<(u64, u64, u8)>,
    mask: u8,
    vertices: u32,
}
struct Host<'a> {
    mode: AppModeView,
    resources: Mut<'a, MenuBackgroundResources>,
    owner: Arc<CaptureOwner>,
    shaders: Arc<country_core::asset_libraries::shader_library::AssetShaderLibrary>,
    draw: Option<Draw>,
    rendered: bool,
}
impl MenuBackgroundHost for Host<'_> {
    fn app_mode(&self) -> AppModeView {
        self.mode
    }
    fn menu_background(&mut self) -> &mut MenuBackgroundResources {
        self.resources.bypass_change_detection()
    }
    fn mark_menu_background_changed(&mut self) {
        self.resources.set_changed();
    }
    fn on_current_replaced(&mut self, old: ManagedTexture) {
        if !self.resources.current.as_ref().is_some_and(|current| current.0 == old.0) {
            self.owner.release(old.0);
        }
    }
    fn raster_shader(&self, name: &str) -> u64 {
        self.shaders
            .get_raster_handle_by_name(name)
            .expect("original menu background shader")
            .id()
            .raw()
    }
    fn begin_pass(&mut self, name: &str) {
        assert_eq!(name, native::PASS_NAME);
    }
    fn begin_draw(&mut self, shader: u64) {
        self.draw = Some(Draw {
            shader,
            mask: 0xff,
            ..Default::default()
        });
    }
    fn bind_texture(&mut self, texture: &ManagedTexture, key: u64, access: u8) {
        self.draw.as_mut().expect("original draw").texture = Some((texture.0, key, access));
    }
    fn clear_raster_state_byte0(&mut self, mask: u8) {
        self.draw.as_mut().expect("original draw").mask &= mask;
    }
    fn draw_no_vao(&mut self, vertices: u32) {
        self.draw.as_mut().expect("original draw").vertices = vertices;
    }
    fn render_pass(&mut self) {
        self.rendered = true;
    }
}

pub fn render(
    frame: &mut CoreGraphFrame<'_>,
    world: &mut World,
    target: AttachmentTexture,
) -> rhapsody::Result<()> {
    let menu = world
        .get_resource::<EscMenuState>()
        .ok_or_else(|| missing("EscMenuState"))?
        .view();
    let mode = AppModeView {
        state: u64::from(menu.tag) | (u64::from(menu.payload) << 32),
        flag: menu.flag,
    };
    if !native::menu_bg_mode_allows(mode) {
        return Ok(());
    }
    let owner = world
        .get_resource::<MenuCaptureGpu>()
        .ok_or_else(|| missing("MenuBackgroundPlugin GPU owner"))?
        .0
        .clone();
    if !Arc::ptr_eq(&owner.host, &frame.hosts().textures) {
        return Err(missing("same-App texture namespace"));
    }
    let shaders = frame.shader_library();
    let (draw, rendered) = world.resource_scope(|_, resources: Mut<MenuBackgroundResources>| {
        let mut host = Host {
            mode,
            resources,
            owner: owner.clone(),
            shaders: shaders.clone(),
            draw: None,
            rendered: false,
        };
        native::render_menu_bg(&mut host);
        (host.draw, host.rendered)
    });
    let Some(draw) = draw else {
        return Ok(());
    };
    assert!(rendered);
    let (id, key, access) = draw.texture.expect("original source binding");
    let texture = owner
        .textures
        .lock()
        .get(&id)
        .cloned()
        .ok_or_else(|| missing("current capture ownership"))?;
    if owner.host.get(&texture).is_none() {
        return Err(missing("live capture allocation"));
    }
    let imported = frame.import_texture(&texture, 0x22)?;
    let desc = AttachmentTexture {
        id: imported.raw(),
        ..AttachmentTexture::from(&texture)
    };
    struct Lookup<'a>(&'a country_core::asset_libraries::shader_library::AssetShaderLibrary);
    impl ShaderLookup for Lookup<'_> {
        fn raster_handle_by_name(
            &self,
            name: &str,
        ) -> Option<Handle<country_core::render::shader::RasterShaderProgram>> {
            self.0
                .get_raster_handle_by_name(name)
                .map(|h| Handle(ManagedHandle(h.id().raw()), std::marker::PhantomData))
        }
    }
    let elapsed = world
        .resource::<country_core::resources::time::GameTime>()
        .elapsed;
    frame.with_raster(|host| {
        let state = host.default_raster_state();
        let mut factory = || -> Box<dyn PassHost> { Box::new(host.sibling()) };
        let lookup = Lookup(&shaders);
        let mut ctx = DrawPassCtx::new(world, elapsed, &mut factory, &lookup);
        let mut pass = ctx.raster_pass(
            native::PASS_NAME,
            &[ColorSpec::tag2(target)],
            DepthSpec::None,
            target,
        );
        let packet = DrawPacket::new(
            Handle(ManagedHandle(draw.shader), std::marker::PhantomData),
            [0, 0],
            state,
        );
        let mut builder = DrawBuilder::new(&mut pass, packet, 0);
        builder.packet_mut().state.0 &= 0xffff_ff00 | u32::from(draw.mask);
        builder
            .bind_texture_to_draw_builder(BindTexture::RgHandle(&desc), key, access, None)
            .draw_no_vao(draw.vertices);
        pass.render();
        Ok(())
    })
}
fn missing(name: &str) -> rhapsody::Error {
    rhapsody::Error::invalid_usage(format!("original menu background: missing {name}"))
}
