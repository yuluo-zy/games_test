
void __cdecl country_core::components::drawable::impl$2::build(void)

{
  bevy_app::sub_app::SubApp::
  add_systems<tuple_<bevy_ecs::schedule::config::Infallible,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,void_(*)(bevy_ecs::system::query::Query<tuple_<ref_<utils::components::transform::Transform>,ref_mut_<country_core::components::drawable::PrevTransform>_>,tuple_<>_>)>_>,bevy_app::main_schedule::Update,void_(*)(bevy_ecs::system::query::Query<tuple_<ref_<utils::components::transform::Transform>,ref_mut_<country_core::components::drawable::PrevTransform>_>,tuple_<>_>)>
            ();
  return;
}

