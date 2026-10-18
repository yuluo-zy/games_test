
void __cdecl country_core::systems::save::impl$1::add_save_load_systems(void)

{
  _ZN8bevy_ecs8schedule6config19IntoScheduleConfigs6in_set17h7150512c6ad02e88E();
  bevy_ecs::schedule::config::impl$3::
  run_if<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,bool_(*)(enum2_<core::option::Option<bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState>_>_>_>)>,bevy_state::condition::in_state::closure_env_0<country_core::app_loading_state::AppLoadingState>_>
            ();
  _ZN8bevy_ecs8schedule6config19IntoScheduleConfigs6run_if17hff5506707d951704E();
  bevy_ecs::intern::Interner::intern<dyn_<bevy_ecs::schedule::set::SystemSet>_>();
  bevy_ecs::schedule::config::ScheduleConfigs::
  in_set_inner<bevy_ecs::intern::Interned<dyn_<bevy_ecs::schedule::set::SystemSet>_>_>();
  bevy_ecs::schedule::config::IntoScheduleConfigs::
  run_if<void_(*)(bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>),alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>,tuple_<bevy_ecs::schedule::config::Infallible,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,void_(*)(bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>)>_>,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,bool_(*)(bevy_ecs::event::reader::EventReader<country_core::game_profile::SaveGameProfileCmd>)>,bool_(*)(bevy_ecs::event::reader::EventReader<country_core::game_profile::SaveGameProfileCmd>)>
            ();
  bevy_ecs::intern::Interner::intern<dyn_<bevy_ecs::schedule::set::SystemSet>_>();
  bevy_ecs::schedule::config::ScheduleConfigs::
  in_set_inner<bevy_ecs::intern::Interned<dyn_<bevy_ecs::schedule::set::SystemSet>_>_>();
  bevy_ecs::schedule::config::IntoScheduleConfigs::
  run_if<void_(*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>),alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>,tuple_<bevy_ecs::schedule::config::Infallible,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,void_(*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>)>_>,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,bool_(*)(bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)>,bool_(*)(bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)>
            ();
  bevy_ecs::intern::Interner::intern<dyn_<bevy_ecs::schedule::set::SystemSet>_>();
  bevy_ecs::schedule::config::ScheduleConfigs::
  in_set_inner<bevy_ecs::intern::Interned<dyn_<bevy_ecs::schedule::set::SystemSet>_>_>();
  bevy_app::sub_app::SubApp::
  add_systems<tuple_<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple_<>,tuple_<>,tuple_<>,tuple_<>_>,bevy_app::main_schedule::Update,tuple_<enum2_<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>_>,enum2_<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>_>,enum2_<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>_>,enum2_<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>_>_>_>
            ();
  return;
}

