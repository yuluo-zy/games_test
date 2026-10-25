// build  0x1408b04c0
// signature: void __cdecl build(void)
// callees: [{"name": "__rust_alloc", "address": "140613c10"}, {"name": "add_systems<tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void_(*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)>_>,bevy_app::main_schedule::PreStartup,void_(*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)>", "address": "140b67880"}, {"name": "handle_alloc_error", "address": "1428d9000"}, {"name": "insert_resource<country_core::resources::grading_lut::LutGalleryState>", "address": "14153f7c0"}]


void __cdecl country_core::systems::render_loop::lut_gallery::impl$0::build(void)

{
  undefined8 *extraout_RAX;
  uint uStack_38;
  
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX != (undefined8 *)0x0) {
    *extraout_RAX = 1;
    extraout_RAX[1] = 1;
    *(undefined1 *)(extraout_RAX + 2) = 0;
    *(undefined4 *)((longlong)extraout_RAX + 0x14) = 0;
    extraout_RAX[3] = (ulonglong)uStack_38;
    extraout_RAX[4] = 0;
    extraout_RAX[5] = 0;
    extraout_RAX[6] = 0;
    *(undefined8 *)((longlong)extraout_RAX + 0x35) = 0;
    *(undefined8 *)((longlong)extraout_RAX + 0x3d) = 0;
    bevy_app::app::App::insert_resource<country_core::resources::grading_lut::LutGalleryState>();
    bevy_app::sub_app::SubApp::
    add_systems<tuple_<bevy_ecs::schedule::config::Infallible,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,void_(*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)>_>,bevy_app::main_schedule::PreStartup,void_(*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)>
              ();
    return;
  }
                    // WARNING: Subroutine does not return
  alloc::alloc::handle_alloc_error();
}

