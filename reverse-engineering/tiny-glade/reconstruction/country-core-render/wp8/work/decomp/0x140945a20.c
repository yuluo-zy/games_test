
void __cdecl country_core::systems::terrain::impl$0::add_terrain_events(void)

{
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong extraout_RAX_04;
  longlong in_RCX;
  ulonglong uVar1;
  
  uVar1 = 0x4cf482123ffcebbb;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX == 0) || (*(ulonglong *)(in_RCX + 0x218) <= uVar1)) ||
      (uVar1 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + uVar1 * 8), uVar1 == 0)) ||
     (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>();
  }
  uVar1 = 0xc1e128444a73c6fc;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_00 == 0) || (*(ulonglong *)(in_RCX + 0x218) <= uVar1)) ||
     ((uVar1 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + uVar1 * 8), uVar1 == 0 ||
      (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>();
  }
  uVar1 = 0xc26ebbeefbfc3261;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_01 == 0) || (*(ulonglong *)(in_RCX + 0x218) <= uVar1)) ||
     ((uVar1 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + uVar1 * 8), uVar1 == 0 ||
      (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::systems::render_loop::compute_passes::highlight_active_terrain_stroke::AddTerrainHighlightCmd>
              ();
  }
  uVar1 = 0xcda6af2f6afb130e;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX_02 == 0) || (*(ulonglong *)(in_RCX + 0x218) <= uVar1)) ||
      (uVar1 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + uVar1 * 8), uVar1 == 0)) ||
     (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd>
              ();
  }
  uVar1 = 0xbd2e81353f745816;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_03 == 0) || (*(ulonglong *)(in_RCX + 0x218) <= uVar1)) ||
     ((uVar1 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + uVar1 * 8), uVar1 == 0 ||
      (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::systems::terrain::delayed_terrain_changed_msg::OnTerrainChange>();
  }
  uVar1 = 0x2965d932542eefc5;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_04 == 0) || (*(ulonglong *)(in_RCX + 0x218) <= uVar1)) ||
     ((uVar1 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + uVar1 * 8), uVar1 == 0 ||
      (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>();
  }
  return;
}


