//! Actual Replay toolbar denial factory, shared ECS consumers and original UI raster pass.
#[path = "support/mod.rs"]
mod test_support;
use test_support::ShaderTestDriver as _;
use test_support::BridgeTestDriver as _;
use bevy_ecs::prelude::*;
use country_core::{
    components::{Ui2dExtent, UiInteractionState},
    game_config::GameConfig,
    game_profile::GameProfile,
    input::action_set::ActionSetStates,
    resources::{
        cursor_terrain_raycast::CursorTerrainRaycast,
        events::{EscapeMenuJustOpened, ModeJustChanged, ShutdownGameCmd},
        gui::{DebugGui, EscMenuState, TimeOfDayScreen},
        history::{EditType, History, MaybePendingSnapshotId},
        mouse_capture::{CursorCapturedPosition, OsCursorVisibilityCmd},
        mouse_terrain_raycast::CursorPositionSS,
        read_mouse_depth_value::MouseDepthValue,
        shader_refactoring_state::ShaderRefactoringState,
        toolbar::{Toolbar, ToolbarCategory},
        window::{WindowFocusEvent, WindowSize},
    },
    systems::{
        backend::input::{
            self, InputSystemSet,
            button_input::{KeyButtonInput, MouseButtonInput},
            translate_input_to_cursor_moved::InputCursorMoved,
        },
        brush_preview::OverwriteBrushPreview,
        camera_rig::MainCamera,
        mode_manager::{
            self, AppMode, AppModeChanged, ClearCanvasCmd, HistoryCommand, LoadingGameComplete,
            ModeManagerSound, ModeManagerUiContext, PreviousAppMode, RequestAppModeChange,
            RequestHistoryCommand, RequestToolbarCategoryChange, RestoreAppTo,
            UiHistoryPlaySelectAnimation,
        },
        ui::{
            hover_intersection::hover_intersection,
            resources::{
                ui_state::{UiState, WidgetId},
                widget_params::{PlayUiSound, UiNavState},
            },
            signifiers::UiSignifierStream,
            ui_update::ui_update,
        },
    },
};
use glam::Vec2;
use toolbar::{
    resouces_and_components::{
        SwitchToolbarCategoryButton, TaffyNode, UiPlaySelectAnimation, taffy_layout::TaffyLayout,
    },
    toolbar_button::{switch_app_mode_button, switch_toolbar_category_button},
    toolbar_update::ToolbarSizeMult,
    ui_interaction::{UiClickEvent, UiDragEvent, render_taffy_ui_to_widgets},
};

struct ReplayHarness {
    world: World,
    schedule: Schedule,
    entity: Entity,
    widget: WidgetId,
}

impl ReplayHarness {
    fn new() -> Self {
        bevy_tasks::ComputeTaskPool::get_or_init(|| {
            bevy_tasks::TaskPoolBuilder::new().num_threads(2).build()
        });
        let mut world = World::new();
        world.insert_resource(AppMode::Path);
        let mut toolbar = Toolbar::default();
        toolbar.switch_mode(&AppMode::Path);
        world.insert_resource(toolbar);
        input::register_input_events(&mut world);
        // These are owned by the real input installer, not fabricated by this
        // fixture. Assert ownership so a future registrar regression is visible.
        assert!(world.contains_resource::<MouseDepthValue>());
        assert!(world.contains_resource::<PreviousAppMode>());
        assert!(world.contains_resource::<Events<AppModeChanged>>());
        assert_eq!(world.resource::<PreviousAppMode>().0, AppMode::Path);
        assert_eq!(
            std::any::TypeId::of::<ModeJustChanged>(),
            std::any::TypeId::of::<AppModeChanged>(),
            "requires the canonical full event alias, never a tool-only bridge"
        );
        assert_eq!(
            std::any::TypeId::of::<UiInteractionState>(),
            std::any::TypeId::of::<toolbar::resouces_and_components::UiInteractionState>()
        );
        assert_eq!(
            std::any::TypeId::of::<country_core::components::UiMouseInteractable>(),
            std::any::TypeId::of::<toolbar::resouces_and_components::UiMouseInteractable>()
        );
        world.init_resource::<DebugGui>();
        world.init_resource::<GameProfile>();
        world.init_resource::<GameConfig>();
        world.init_resource::<ShaderRefactoringState>();
        world.init_resource::<EscMenuState>();
        world.init_resource::<TimeOfDayScreen>();
        world.init_resource::<CursorCapturedPosition>();
        world.init_resource::<CursorTerrainRaycast>();
        world.insert_resource(CursorPositionSS(Vec2::ZERO));
        world.insert_resource(WindowSize::new(960, 540));
        world.insert_resource(MainCamera::new(1.0));
        world.insert_resource(UiState::at_startup());
        world.init_resource::<UiNavState>();
        world.init_resource::<UiSignifierStream>();
        world.insert_resource(ToolbarSizeMult { multiplier: 1.0 });
        world.insert_resource(History::<MaybePendingSnapshotId>::empty_glade());
        world.init_resource::<RestoreAppTo>();
        world.insert_resource(ModeManagerUiContext::from_ui_state(
            world.resource::<UiState>(),
        ));
        world.init_resource::<Events<WindowFocusEvent>>();
        world.init_resource::<Events<EscapeMenuJustOpened>>();
        world.init_resource::<Events<ShutdownGameCmd>>();
        world.init_resource::<Events<OverwriteBrushPreview>>();
        world.init_resource::<Events<OsCursorVisibilityCmd>>();
        world.init_resource::<Events<PlayUiSound>>();
        world.init_resource::<Events<UiClickEvent>>();
        world.init_resource::<Events<UiDragEvent>>();
        world.init_resource::<Events<ModeJustChanged>>();
        world.init_resource::<Events<RequestAppModeChange>>();
        world.init_resource::<Events<RequestToolbarCategoryChange>>();
        world.init_resource::<Events<RequestHistoryCommand>>();
        world.init_resource::<Events<LoadingGameComplete>>();
        world.init_resource::<Events<HistoryCommand>>();
        world.init_resource::<Events<UiHistoryPlaySelectAnimation>>();
        world.init_resource::<Events<ModeManagerSound>>();
        world.init_resource::<Events<ClearCanvasCmd>>();
        world.init_resource::<toolbar::MeiContext>();
        world.init_resource::<Events<bevy_input::mouse::MouseWheel>>();
        world.init_resource::<Events<country_core::systems::audio::PlayMenuUiSound>>();

        // Register required components exactly as the application does.
        toolbar::resouces_and_components::register_local_components(&mut world);
        world.init_resource::<TaffyLayout>();
        world.init_resource::<country_core::resources::gui::PhotomodeScreen>();
        world.init_resource::<country_core::startup::init_photomode_state::PhotomodeState>();
        world.init_resource::<country_core::startup::init_photomode_state::SteamdeckScreenshotMsgScreen>();
        world.insert_resource(country_core::resources::onboarding::WelcomeScreenState::Closed);
        let mut startup = Schedule::default();
        startup.add_systems(toolbar::toolbar_startup::toolbar_setup);
        startup.run(&mut world);
        world.flush();
        let entity = world
            .query::<(Entity, &SwitchToolbarCategoryButton)>()
            .iter(&world)
            .find(|(_, button)| button.category == ToolbarCategory::Water)
            .expect("real full-edition Water category")
            .0;
        let node = world.get::<TaffyNode>(entity).unwrap().node;
        let widget = WidgetId::from_name(&format!("toolbar_button_{node:?}"));
        let mut schedule = Schedule::default();
        // Reuse the existing input_pipeline fixture's real installer. Its
        // Consume slot runs after translate/resolve/record/derive and before
        // replay advancement. The relative consumer order matches host 6/7/8/10.
        input::add_input_cpu_consumers(&mut schedule);
        schedule.add_systems(
            (
                hover_intersection,
                ui_update,
                toolbar::mei::systems::mei_pre_update,
                toolbar::ui_interaction::hide_ui,
                render_taffy_ui_to_widgets,
                toolbar::ui_interaction::ui_focus_parent_if_self_in_focus,
                toolbar::ui_interaction::ui_show_if_parent_in_focus,
                (
                    toolbar::ui_interaction::ui_animation_state_update,
                    toolbar::ui_presentation::ui_focus_animation,
                    toolbar::ui_presentation::ui_appear_animation,
                )
                    .chain_ignore_deferred(),
                toolbar::ui_interaction::ui_nav_update,
                toolbar::toolbar_systems::toolbar_steal_mouse_input,
                switch_toolbar_category_button,
                switch_app_mode_button,
                mode_manager::synchronize_mode_manager_ui_context,
                mode_manager::mode_manager::<EditType, MaybePendingSnapshotId>,
                mode_manager::send_mode_just_changed_event,
                toolbar::toolbar_systems::toolbar_alignment,
                toolbar::taffy_update::taffy_finalize_element_size,
                toolbar::taffy_update::taffy_perform_layout,
                toolbar::taffy_update::taffy_update_drawables,
            )
                .chain_ignore_deferred()
                .in_set(InputSystemSet::Consume),
        );
        Self {
            world,
            schedule,
            entity,
            widget,
        }
    }

    fn frame(&mut self, pressed: bool, position: Vec2) {
        let mut mouse = self.world.resource_mut::<MouseButtonInput>();
        mouse.clear(); // Same per-frame edge clearing as the existing device harness.
        if pressed {
            mouse.press(0);
        } else {
            mouse.release(0);
        }
        self.world.send_event(InputCursorMoved::Absolute(position));
        self.world
            .resource_mut::<bevy_time::Time>()
            .advance_by(std::time::Duration::from_millis(16));
        self.schedule.run(&mut self.world);
        self.world.flush();
        assert_eq!(self.world.resource::<CursorPositionSS>().0, position);
    }

    fn position(&self) -> Vec2 {
        let extent = self.world.get::<Ui2dExtent>(self.entity).unwrap();
        assert!(extent.size.x > 0.0 && extent.size.y > 0.0);
        extent.pos
    }

    fn key_frame(&mut self, binding: Option<usize>) {
        let code = binding.map(|slot| {
            let key = self.world.resource::<GameProfile>().key_bindings.bindings[slot]
                .expect("default navigation binding");
            country_core::game_profile::desktop_keycode_index(key).unwrap()
        });
        let mut keyboard = self.world.resource_mut::<KeyButtonInput>();
        keyboard.release_all();
        keyboard.clear();
        if let Some(code) = code {
            keyboard.press(code);
        }
        self.frame(false, Vec2::ZERO);
    }

    fn requests(&self) -> Vec<AppMode> {
        let events = self.world.resource::<Events<RequestAppModeChange>>();
        events.get_cursor().read(events).map(|e| e.0).collect()
    }
}

fn render_cross_pixels(
    bridge: &mut tiny_glade::render_bridge::CoreRenderBridge,
    world: &mut World,
) -> Vec<u8> {
    use ash::vk::{self, Handle as _};
    use country_core::systems::ui::signifiers::SignifierId;
    use country_core::{
        render::{
            load_texture::boundary::TextureDevice,
            pass_builder::{draw_stream::PassHost, framebuffer_binding::AttachmentTexture},
            ssbo::rhapsody_boundary::{BufferDesc, GraphicsDevice},
        },
        rhapsody_types::TextureDesc,
        systems::render_loop::draw_passes::{
            d_main_support::{DrawPassCtx, ShaderLookup},
            ui_2d_pass::{self, Ui2dDrawElement},
        },
    };
    let elements: Vec<_> = world
        .query_filtered::<(&Ui2dExtent, &country_core::components::Ui2dImage), With<SignifierId>>()
        .iter(world)
        .map(|(extent, image)| Ui2dDrawElement {
            extent: [extent.pos.x, extent.pos.y, extent.size.x, extent.size.y],
            image: image.into(),
            icon: None,
            interaction: ui_2d_pass::UiInteractionStateRead { flag: false },
            order: None,
            inactive: false,
        })
        .collect();
    let hosts = bridge.hosts().clone();
    let target = hosts
        .textures
        .create_texture(
            "Replay cross validation",
            &TextureDesc::new_2d([960, 540], 37)
                .color_attachment()
                .sampled(),
            &[],
        )
        .unwrap();
    let readback = GraphicsDevice::create_buffer(
        hosts.buffers.as_ref(),
        &BufferDesc::new(960 * 540 * 4, 0x22, 2),
        "Replay cross pixels",
        None,
    )
    .unwrap();
    let shaders = bridge.shader_library().clone();
    struct Lookup<'a>(&'a country_core::asset_libraries::shader_library::AssetShaderLibrary);
    impl ShaderLookup for Lookup<'_> {
        fn raster_handle_by_name(
            &self,
            name: &str,
        ) -> Option<
            country_core::rhapsody_contract::Handle<
                country_core::render::shader::RasterShaderProgram,
            >,
        > {
            self.0.get_raster_handle_by_name(name).map(|handle| {
                country_core::rhapsody_contract::Handle(
                    country_core::rhapsody_contract::ManagedHandle(handle.id().raw()),
                    core::marker::PhantomData,
                )
            })
        }
    }
    let (_, built) = bridge
        .build_frame(|frame| {
            tiny_glade::frame_setup::with_frame_resources(frame, world, |frame, world| {
            let color = frame.import_texture(&target, 0)?;
            let buffer = frame.import_buffer(&readback, 0)?;
            let color_index = color.raw();
            let buffer_index = buffer.raw();
            let device = hosts.buffers.graphics_device().clone();
            frame.with_graph(|graph| {
                let mut pass = graph.add_pass("Replay pixel test clear");
                pass.add_resource_access(color, 0x34, 0)?;
                pass.render_with_context(move |context| {
                    let image = context.managed.unwrap().texture_for(color_index).unwrap();
                    unsafe {
                        device.raw_device().cmd_clear_color_image(
                            vk::CommandBuffer::from_raw(context.command.handle()),
                            image.raw(),
                            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                            &vk::ClearColorValue { float32: [0.0; 4] },
                            &[vk::ImageSubresourceRange {
                                aspect_mask: vk::ImageAspectFlags::COLOR,
                                base_mip_level: 0,
                                level_count: 1,
                                base_array_layer: 0,
                                layer_count: 1,
                            }],
                        );
                    }
                    true
                });
                Ok(())
            })?;
            frame.with_raster(|host| {
                let mut sibling = || -> Box<dyn PassHost> { Box::new(host.sibling()) };
                let lookup = Lookup(&shaders);
                let mut ctx =
                    DrawPassCtx::new(world, std::time::Duration::ZERO, &mut sibling, &lookup);
                ui_2d_pass::ui_2d_pass(
                    &mut ctx,
                    &AttachmentTexture {
                        id: color_index,
                        extent: (960, 540),
                        format: 37,
                        sample_count: 1,
                    },
                    (960, 540),
                    &elements,
                    &[],
                );
                Ok(())
            })?;
            let device = hosts.buffers.graphics_device().clone();
            frame.with_graph(|graph| {
                let mut pass = graph.add_pass("Replay pixel test readback");
                pass.add_resource_access(color, 0x25, 0)?;
                pass.add_resource_access(buffer, 0x34, 0)?;
                pass.render_with_context(move |context| {
                    let managed = context.managed.unwrap();
                    let image = managed.texture_for(color_index).unwrap();
                    let destination = managed.buffer_for(buffer_index).unwrap();
                    unsafe {
                        device.raw_device().cmd_copy_image_to_buffer(
                            vk::CommandBuffer::from_raw(context.command.handle()),
                            image.raw(),
                            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                            destination.raw(),
                            &[vk::BufferImageCopy {
                                image_subresource: vk::ImageSubresourceLayers {
                                    aspect_mask: vk::ImageAspectFlags::COLOR,
                                    mip_level: 0,
                                    base_array_layer: 0,
                                    layer_count: 1,
                                },
                                image_extent: vk::Extent3D {
                                    width: 960,
                                    height: 540,
                                    depth: 1,
                                },
                                ..Default::default()
                            }],
                        );
                    }
                    true
                });
                graph.export(buffer, 0x26)?;
                Ok(())
            })?;
            Ok(())
            })
        })
        .unwrap();
    bridge.shader_backend().prepare_test_pipelines(std::time::Duration::from_secs(10)).unwrap();
    bridge.execute_test_graph(built).unwrap();
    bridge.wait_submits_done(10_000_000_000).unwrap();
    let mut pixels = vec![0; 960 * 540 * 4];
    unsafe {
        hosts
            .buffers
            .get_by_id(readback.id)
            .unwrap()
            .shared()
            .read_bytes_after_completion(0, &mut pixels)
            .unwrap();
    }
    hosts.textures.release(&target);
    hosts.buffers.release(&readback);
    pixels
}

#[test]
fn physical_replay_denial_builds_follows_rasterizes_and_unloads_the_real_cross() {
    use country_core::{
        asset_libraries::shader_library::{AssetShaderLibrary, SharedAssetShaderLibrary},
        render::load_texture::{AlignedFile, boundary::TextureDevice, parse_compiled_image},
        resources::time::GameTime,
        systems::ui::signifiers::{
            SignifierId,
            animation::{signifier_animation, signifier_animation_t},
            signifier_create::signifier_create,
            signifier_mark_for_delete::{despawn_marked, signfier_mark_for_delete},
        },
    };
    use std::{path::PathBuf, sync::Arc};
    let installed = std::env::var_os("TG_INSTALL_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../ljxsj_92385/Tiny Glade")
        })
        .canonicalize()
        .unwrap();
    let device =
        rhapsody::device::Device::new_headless(&rhapsody::device::DeviceConfig::headless())
            .unwrap();
    let shaders = Arc::new(AssetShaderLibrary::new());
    let mut bridge = tiny_glade::render_bridge::CoreRenderBridge::new(
        device,
        installed.clone(),
        shaders.clone(),
    )
    .unwrap();
    country_core::startup::load_shaders::load_shader_into_library(
        &shaders,
        bridge.shader_backend(),
        "/vertex_color.raster.hlsl",
        "vertex_color_shader",
    );
    country_core::startup::load_shaders::load_ui_shaders(&shaders, bridge.shader_backend());
    let mut h = ReplayHarness::new();
    bridge.install_asset_libraries(&mut h.world).unwrap();
    h.world.init_resource::<country_core::render::frame_constants::FrameConstants>();
    h.world.init_resource::<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>();
    let mut watch = country_core::asset_libraries::configwatch::ConfigWatch::new(false);
    let palette = country_core::startup::init_color_library::init_color_library(
        bridge.hosts().buffers.as_ref(),
        h.world.resource::<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>(),
        &mut watch,
    );
    h.world.insert_resource(palette);
    h.world.insert_resource(watch);
    h.world.insert_resource(SharedAssetShaderLibrary(shaders));
    h.world.insert_resource(GameTime {
        delta: 1.0 / 60.0,
        ..Default::default()
    });
    let path = installed.join("compiled-assets/textures/icon_cross.texture");
    let bytes = AlignedFile::read(&path).unwrap();
    let image = parse_compiled_image(bytes.bytes(), &path);
    let texture = bridge
        .hosts()
        .textures
        .create_texture("icon_cross", &image.desc, image.data)
        .unwrap();
    h.world
        .resource_mut::<country_core::asset_libraries::texture_library::AssetTextureLibrary>()
        .add(Some("icon_cross".to_owned()), texture);
    h.schedule.add_systems(
        (
            signifier_create,
            signfier_mark_for_delete,
            signifier_animation_t,
            signifier_animation,
            despawn_marked,
        )
            .chain_ignore_deferred()
            .in_set(InputSystemSet::Consume)
            .after(ui_update)
            .before(toolbar::mei_pre_update),
    );
    h.schedule.add_systems(
        country_core::systems::ui_2d_follow_mouse::ui_2d_follow_mouse
            .in_set(InputSystemSet::Consume)
            .after(signifier_animation),
    );
    for _ in 0..24 {
        h.frame(false, Vec2::ZERO);
    }
    let baseline = render_cross_pixels(&mut bridge, &mut h.world);
    assert!(baseline.iter().all(|value| *value == 0));
    // Authoritative replay mode is the session starting condition; actual
    // mouse input, picker, toolbar denial producer and consumers remain real.
    h.world.insert_resource(AppMode::Replay);
    for _ in 0..4 {
        h.frame(false, h.position());
    }
    h.frame(true, h.position());
    h.frame(false, h.position());
    let moved_cursor = h.position() + Vec2::new(4.0, 0.0);
    h.frame(false, moved_cursor);
    assert!(h.world.resource::<Events<UiClickEvent>>().is_empty());
    assert_eq!(*h.world.resource::<AppMode>(), AppMode::Replay);
    let crosses: Vec<_> = h
        .world
        .query_filtered::<Entity, With<SignifierId>>()
        .iter(&h.world)
        .collect();
    assert!(!crosses.is_empty());
    let transient_meshes: Vec<_> = crosses
        .iter()
        .map(|entity| {
            h.world
                .get::<country_core::asset_libraries::transient_mesh::TransientMesh>(*entity)
                .unwrap()
                .mesh_handle()
        })
        .collect();
    for entity in &crosses {
        assert_eq!(
            h.world
                .get::<country_core::components::Ui2dImage>(*entity)
                .unwrap()
                .name,
            "icon_cross"
        );
        assert!(
            h.world
                .get::<country_core::components::Ui2DFollowMouse>(*entity)
                .is_some()
        );
        assert_eq!(
            h.world.get::<Ui2dExtent>(*entity).unwrap().pos,
            moved_cursor
        );
    }
    let pixels = render_cross_pixels(&mut bridge, &mut h.world);
    assert!(
        pixels
            .chunks_exact(4)
            .any(|pixel| pixel[..3].iter().any(|value| *value != 0)),
        "real original UI pass must draw nonzero cross pixels"
    );
    let mut exit = Schedule::new(bevy_state::state::OnExit(
        country_core::app_loading_state::AppLoadingState::Game,
    ));
    exit.add_systems(
        (
            tiny_glade::toolbar_plugin::clear_toolbar_session,
            country_core::startup::clearing::despawn_glade_entities,
        )
            .chain(),
    );
    h.world.add_schedule(exit);
    h.world.run_schedule(bevy_state::state::OnExit(
        country_core::app_loading_state::AppLoadingState::Game,
    ));
    h.world.flush();
    assert!(
        crosses
            .iter()
            .all(|entity| h.world.get_entity(*entity).is_err())
    );
    h.world.resource_scope(
        |world, mut meshes: Mut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>| {
            let mut vaos =
                world.resource_mut::<country_core::asset_libraries::vao_library::AssetVAOLibrary>();
            country_core::asset_libraries::transient_mesh::delete_dropped_transient_meshes(
                bridge.hosts().buffers.as_ref(),
                &mut meshes,
                &mut vaos,
            );
        },
    );
    let meshes = h
        .world
        .resource::<country_core::asset_libraries::mesh_library::AssetMeshLibrary>();
    assert!(
        transient_meshes
            .iter()
            .all(|handle| meshes.get(*handle).is_none())
    );
    assert!(meshes.transient_meshes().lock().unwrap().is_empty());
    assert_eq!(h.world.resource::<UiSignifierStream>().committed_len(), 0);
    let unloaded = render_cross_pixels(&mut bridge, &mut h.world);
    assert!(
        unloaded.iter().all(|value| *value == 0),
        "unload must remove the actual GPU cross output"
    );
}
