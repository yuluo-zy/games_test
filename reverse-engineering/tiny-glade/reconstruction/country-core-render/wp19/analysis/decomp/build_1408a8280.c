// build  0x1408a8280
// signature: void __cdecl build(void)
// callees: [{"name": "add_systems<tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void_(*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)>_>,bevy_app::main_schedule::PreStartup,void_(*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)>", "address": "140b674e0"}, {"name": "add_systems<tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void_(*)(bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState>_>)>_>,bevy_state::state::transitions::OnExit<country_core::app_loading_state::AppLoadingState>,country_core::systems::render_loop::vermeer::impl$1::build::closure_env$0>", "address": "140b66d60"}, {"name": "insert_resource<country_core::systems::render_loop::vermeer::VermeerDebugControl>", "address": "141549170"}, {"name": "insert_resource<enum2$<country_core::systems::render_loop::vermeer::VermeerState>_>", "address": "14153c5d0"}]


void __cdecl country_core::systems::render_loop::vermeer::impl$1::build(void)

{
  bevy_app::app::App::
  insert_resource<enum2_<country_core::systems::render_loop::vermeer::VermeerState>_>();
  bevy_app::app::App::
  insert_resource<country_core::systems::render_loop::vermeer::VermeerDebugControl>();
  bevy_app::sub_app::SubApp::
  add_systems<tuple_<bevy_ecs::schedule::config::Infallible,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,void_(*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)>_>,bevy_app::main_schedule::PreStartup,void_(*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)>
            ();
  bevy_app::sub_app::SubApp::
  add_systems<tuple_<bevy_ecs::schedule::config::Infallible,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,void_(*)(bevy_ecs::change_detection::ResMut<enum2_<country_core::systems::render_loop::vermeer::VermeerState>_>)>_>,bevy_state::state::transitions::OnExit<country_core::app_loading_state::AppLoadingState>,country_core::systems::render_loop::vermeer::impl_1::build::closure_env_0>
            ();
  return;
}

