
void __cdecl tiny_glade::game_loop::impl$3::build(void)

{
  bevy_app::sub_app::SubApp::
  add_systems<tuple_<bevy_ecs::schedule::config::Infallible,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,void_(*)(bevy_ecs::event::reader::EventReader<tiny_glade::app_config::ApplyVideoSettingsCmd>)>_>,bevy_app::main_schedule::PostUpdate,void_(*)(bevy_ecs::event::reader::EventReader<tiny_glade::app_config::ApplyVideoSettingsCmd>)>
            ();
  return;
}

