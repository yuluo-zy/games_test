// name=add_roof_events_and_resources
// sig=void __cdecl add_roof_events_and_resources(void)
// size=1067

void __cdecl system_roof::impl$0::add_roof_events_and_resources(void)

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
  longlong extraout_RAX_08;
  longlong extraout_RAX_09;
  longlong extraout_RAX_10;
  ulonglong uVar1;
  
  bevy_app::app::App::insert_resource<system_roof::ui_edit_roof::RoofParamsCacheResource>();
  bevy_app::app::App::insert_resource<country_core::resources::roofs::roof_highlight::RoofHighlight>
            ();
  uVar1 = 0x76f812fb58de4f70;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX_00 == 0) || (*(ulonglong *)(extraout_RAX + 0x218) <= uVar1)) ||
      (uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX + 0x210) + uVar1 * 8), uVar1 == 0)) ||
     (*(longlong *)(*(longlong *)(extraout_RAX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::resources::roofs::commands::CreateRoofCmd>();
  }
  uVar1 = 0x7df16b212f058a4e;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_01 == 0) || (*(ulonglong *)(extraout_RAX + 0x218) <= uVar1)) ||
     ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX + 0x210) + uVar1 * 8), uVar1 == 0 ||
      (*(longlong *)(*(longlong *)(extraout_RAX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::resources::roofs::commands::DeleteRoofCmd>();
  }
  uVar1 = 0xc9d6b47311723910;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_02 == 0) || (*(ulonglong *)(extraout_RAX + 0x218) <= uVar1)) ||
     ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX + 0x210) + uVar1 * 8), uVar1 == 0 ||
      (*(longlong *)(*(longlong *)(extraout_RAX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::resources::roofs::commands::EditRoofCmd>();
  }
  uVar1 = 0x79512746800fddf4;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX_03 == 0) || (*(ulonglong *)(extraout_RAX + 0x218) <= uVar1)) ||
      (uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX + 0x210) + uVar1 * 8), uVar1 == 0)) ||
     (*(longlong *)(*(longlong *)(extraout_RAX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::resources::roofs::commands::RecalculateRoofCmd>();
  }
  uVar1 = 0xd72e82ceeaa2d715;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_04 == 0) || (*(ulonglong *)(extraout_RAX + 0x218) <= uVar1)) ||
     ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX + 0x210) + uVar1 * 8), uVar1 == 0 ||
      (*(longlong *)(*(longlong *)(extraout_RAX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::resources::roofs::roof_sounds::PlayRoofSound>();
  }
  uVar1 = 0x9b513f367da46e41;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_05 == 0) || (*(ulonglong *)(extraout_RAX + 0x218) <= uVar1)) ||
     ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX + 0x210) + uVar1 * 8), uVar1 == 0 ||
      (*(longlong *)(*(longlong *)(extraout_RAX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::resources::roofs::commands::HighlightRoofCmd>();
  }
  uVar1 = 0x44704b19ba545721;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX_06 == 0) || (*(ulonglong *)(extraout_RAX + 0x218) <= uVar1)) ||
      (uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX + 0x210) + uVar1 * 8), uVar1 == 0)) ||
     (*(longlong *)(*(longlong *)(extraout_RAX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::resources::roofs::commands::SwitchRoofTypeCmd>();
  }
  uVar1 = 0x30d267c659e8762b;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_07 == 0) || (*(ulonglong *)(extraout_RAX + 0x218) <= uVar1)) ||
     ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX + 0x210) + uVar1 * 8), uVar1 == 0 ||
      (*(longlong *)(*(longlong *)(extraout_RAX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>();
  }
  uVar1 = 0x79130f3bd6fd4f66;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_08 == 0) || (*(ulonglong *)(extraout_RAX + 0x218) <= uVar1)) ||
     ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX + 0x210) + uVar1 * 8), uVar1 == 0 ||
      (*(longlong *)(*(longlong *)(extraout_RAX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<system_roof::deferred_record_roof_history::RecordRoofHistoryCmd>();
  }
  uVar1 = 0xe148524d31b4003a;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX_09 == 0) || (*(ulonglong *)(extraout_RAX + 0x218) <= uVar1)) ||
      (uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX + 0x210) + uVar1 * 8), uVar1 == 0)) ||
     (*(longlong *)(*(longlong *)(extraout_RAX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>();
  }
  uVar1 = 0x4e76346e82415192;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_10 == 0) || (*(ulonglong *)(extraout_RAX + 0x218) <= uVar1)) ||
     ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX + 0x210) + uVar1 * 8), uVar1 == 0 ||
      (*(longlong *)(*(longlong *)(extraout_RAX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>();
  }
  return;
}

