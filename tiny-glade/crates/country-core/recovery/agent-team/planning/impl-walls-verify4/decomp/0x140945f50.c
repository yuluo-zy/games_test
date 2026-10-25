
void __cdecl country_core::systems::wall::shapes::impl$0::add_wall_shape_ux(void)

{
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong extraout_RAX_04;
  longlong extraout_RAX_05;
  longlong extraout_RAX_06;
  longlong extraout_RAX_07;
  ulonglong uVar1;
  
  _ZN8bevy_ecs8schedule6config19IntoScheduleConfigs21chain_ignore_deferred17hb19bd34d494346c0E();
  _ZN8bevy_ecs8schedule6config19IntoScheduleConfigs5chain17h23a51ffd143e1e27E();
  _ZN8bevy_ecs8schedule6config19IntoScheduleConfigs21chain_ignore_deferred17hffe5f522185a65e1E();
  bevy_ecs::schedule::config::IntoScheduleConfigs::
  chain_ignore_deferred<tuple_<enum2_<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>_>,enum2_<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>_>_>,alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>,tuple_<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple_<>,tuple_<>_>_>
            ();
  bevy_ecs::intern::Interner::intern<dyn_<bevy_ecs::schedule::set::SystemSet>_>();
  bevy_ecs::schedule::config::ScheduleConfigs::
  in_set_inner<bevy_ecs::intern::Interned<dyn_<bevy_ecs::schedule::set::SystemSet>_>_>();
  bevy_ecs::schedule::config::impl$3::
  run_if<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,bool_(*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState>_>)>,bool_(*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState>_>)>
            ();
  bevy_app::sub_app::SubApp::
  add_systems<tuple_<>,bevy_app::main_schedule::Update,enum2_<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>_>_>
            ();
  bevy_app::app::App::
  insert_resource<enum2_<country_core::systems::wall::shapes::rectangle::WipRectangle>_>();
  bevy_app::app::App::
  insert_resource<country_core::systems::wall::shapes::shape_placement::ShapePlacement>();
  bevy_app::app::App::
  insert_resource<country_core::systems::wall::shapes::shape_placement::SnapMode>();
  uVar1 = 0xc38a6a0c228dcd67;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX_00 == 0) || (*(ulonglong *)(extraout_RAX + 0x218) <= uVar1)) ||
      (uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX + 0x210) + uVar1 * 8), uVar1 == 0)) ||
     (*(longlong *)(*(longlong *)(extraout_RAX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::systems::wall::shapes::rectangle::CreateRectangleCmd>();
  }
  uVar1 = 0x1a0858bd8956ea1;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_01 == 0) || (*(ulonglong *)(extraout_RAX + 0x218) <= uVar1)) ||
     ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX + 0x210) + uVar1 * 8), uVar1 == 0 ||
      (*(longlong *)(*(longlong *)(extraout_RAX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::systems::wall::shapes::rectangle::EditRectangleCmd>();
  }
  uVar1 = 0xdcbb5a7f787a5734;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_02 == 0) || (*(ulonglong *)(extraout_RAX + 0x218) <= uVar1)) ||
     ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX + 0x210) + uVar1 * 8), uVar1 == 0 ||
      (*(longlong *)(*(longlong *)(extraout_RAX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::systems::wall::shapes::rectangle::FinishEditRectangleCmd>();
  }
  uVar1 = 0x4a1db5837d11c93f;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX_03 == 0) || (*(ulonglong *)(extraout_RAX + 0x218) <= uVar1)) ||
      (uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX + 0x210) + uVar1 * 8), uVar1 == 0)) ||
     (*(longlong *)(*(longlong *)(extraout_RAX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::systems::wall::shapes::circle::create_circle::CreateCircleCmd>();
  }
  uVar1 = 0x58adde1121b70c78;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_04 == 0) || (*(ulonglong *)(extraout_RAX + 0x218) <= uVar1)) ||
     ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX + 0x210) + uVar1 * 8), uVar1 == 0 ||
      (*(longlong *)(*(longlong *)(extraout_RAX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::systems::wall::shapes::shape_placement::DontRecalculateShapePlacement>
              ();
  }
  uVar1 = 0xf4e90bdaaff68cb;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_05 == 0) || (*(ulonglong *)(extraout_RAX + 0x218) <= uVar1)) ||
     ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX + 0x210) + uVar1 * 8), uVar1 == 0 ||
      (*(longlong *)(*(longlong *)(extraout_RAX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::systems::ui_raise_shape::HandleRaiseShapeUiInteraction>();
  }
  uVar1 = 0xfbdd85d7ac4b26cb;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX_06 == 0) || (*(ulonglong *)(extraout_RAX + 0x218) <= uVar1)) ||
      (uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX + 0x210) + uVar1 * 8), uVar1 == 0)) ||
     (*(longlong *)(*(longlong *)(extraout_RAX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::DoUiEditRectangleDims>
              ();
  }
  uVar1 = 0x23ea1ac5bdb3f981;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_07 == 0) || (*(ulonglong *)(extraout_RAX + 0x218) <= uVar1)) ||
     ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX + 0x210) + uVar1 * 8), uVar1 == 0 ||
      (*(longlong *)(*(longlong *)(extraout_RAX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::DoUiEditRectangleRotate>
              ();
  }
  bevy_app::app::App::
  insert_resource<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource>
            ();
  bevy_app::app::App::
  insert_resource<country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::UiEditRectangleActiveWidgetResource>
            ();
  bevy_app::app::App::
  insert_resource<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle>
            ();
  bevy_app::app::App::
  insert_resource<country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle>();
  bevy_app::app::App::
  insert_resource<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::ShapesDragMouseMoved>
            ();
  bevy_app::app::App::
  insert_resource<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>
            ();
  return;
}

