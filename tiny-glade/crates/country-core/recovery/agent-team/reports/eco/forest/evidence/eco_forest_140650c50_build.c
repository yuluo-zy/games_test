// signature: void __cdecl build(void)
// size: 207
// locals: []
// callees: [{"name": "_ZN8bevy_ecs8schedule6config19IntoScheduleConfigs21chain_ignore_deferred17hdee9057d98cfc71cE", "address": "140ea68d0"}, {"name": "add_systems<tuple$<>,bevy_app::main_schedule::Update,enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<>_>,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>_>_>", "address": "140ea7c90"}, {"name": "in_set_inner<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<>_>,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>", "address": "140ea7c00"}, {"name": "insert_resource<country_core::resources::tree_storage::InteractableForestPlacements>", "address": "140ea7ff0"}, {"name": "intern<dyn$<bevy_ecs::schedule::set::SystemSet>_>", "address": "140ea74a0"}]

void __cdecl forest::impl$0::build(void)

{
  _ZN8bevy_ecs8schedule6config19IntoScheduleConfigs21chain_ignore_deferred17hdee9057d98cfc71cE();
  bevy_ecs::intern::Interner::intern<dyn_<bevy_ecs::schedule::set::SystemSet>_>();
  bevy_ecs::schedule::config::ScheduleConfigs::
  in_set_inner<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>
            ();
  bevy_app::sub_app::SubApp::
  add_systems<tuple_<>,bevy_app::main_schedule::Update,enum2_<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>_>_>
            ();
  bevy_app::app::App::
  insert_resource<country_core::resources::tree_storage::InteractableForestPlacements>();
  return;
}

