
void __cdecl
system_clutter::autoclutter::populate_autoclutter_regions::autoclutter_subscribe_to_tabloids(void)

{
  undefined4 uVar1;
  undefined4 uVar2;
  longlong *plVar3;
  undefined4 *puVar4;
  undefined4 *puVar5;
  longlong lVar6;
  undefined8 extraout_RAX;
  undefined8 extraout_RAX_00;
  undefined8 extraout_RAX_01;
  undefined8 extraout_RAX_02;
  undefined8 extraout_RAX_03;
  undefined8 extraout_RAX_04;
  undefined8 extraout_RAX_05;
  undefined8 extraout_RAX_06;
  undefined8 extraout_RAX_07;
  undefined8 extraout_RAX_08;
  undefined8 extraout_RAX_09;
  undefined8 extraout_RAX_10;
  undefined8 extraout_RAX_11;
  longlong *in_RCX;
  longlong in_RDX;
  longlong in_R8;
  longlong in_R9;
  longlong in_stack_00000028;
  longlong in_stack_00000030;
  longlong in_stack_00000038;
  
  uVar1 = *(undefined4 *)((longlong)in_RCX + 0x1c);
  plVar3 = (longlong *)*in_RCX;
  puVar4 = (undefined4 *)in_RCX[2];
  uVar2 = *(undefined4 *)(in_RDX + 0x1c);
  puVar5 = *(undefined4 **)(in_RDX + 0x10);
  *puVar4 = uVar1;
  *puVar5 = uVar2;
  country_core::systems::collision::world::RaycastWorld::subscribe_to_changes();
  lVar6 = plVar3[2];
  if (lVar6 == *plVar3) {
    alloc::raw_vec::RawVec::
    grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
              ();
  }
  *(undefined8 *)(plVar3[1] + lVar6 * 8) = extraout_RAX;
  plVar3[2] = lVar6 + 1;
  *puVar4 = uVar1;
  *puVar5 = uVar2;
  country_core::systems::collision::world::RaycastWorld::subscribe_to_changes();
  lVar6 = plVar3[2];
  if (lVar6 == *plVar3) {
    alloc::raw_vec::RawVec::
    grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
              ();
  }
  *(undefined8 *)(plVar3[1] + lVar6 * 8) = extraout_RAX_00;
  plVar3[2] = lVar6 + 1;
  *puVar4 = uVar1;
  *puVar5 = uVar2;
  country_core::systems::collision::world::RaycastWorld::subscribe_to_changes();
  lVar6 = plVar3[2];
  if (lVar6 == *plVar3) {
    alloc::raw_vec::RawVec::
    grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
              ();
  }
  *(undefined8 *)(plVar3[1] + lVar6 * 8) = extraout_RAX_01;
  plVar3[2] = lVar6 + 1;
  *puVar4 = uVar1;
  *puVar5 = uVar2;
  country_core::systems::collision::world::RaycastWorld::subscribe_to_changes();
  lVar6 = plVar3[2];
  if (lVar6 == *plVar3) {
    alloc::raw_vec::RawVec::
    grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
              ();
  }
  *(undefined8 *)(plVar3[1] + lVar6 * 8) = extraout_RAX_02;
  plVar3[2] = lVar6 + 1;
  *puVar4 = uVar1;
  *puVar5 = uVar2;
  country_core::systems::collision::world::RaycastWorld::subscribe_to_changes();
  lVar6 = plVar3[2];
  if (lVar6 == *plVar3) {
    alloc::raw_vec::RawVec::
    grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
              ();
  }
  *(undefined8 *)(plVar3[1] + lVar6 * 8) = extraout_RAX_03;
  plVar3[2] = lVar6 + 1;
  *puVar4 = uVar1;
  *puVar5 = uVar2;
  country_core::systems::collision::world::RaycastWorld::subscribe_to_changes();
  lVar6 = plVar3[2];
  if (lVar6 == *plVar3) {
    alloc::raw_vec::RawVec::
    grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
              ();
  }
  *(undefined8 *)(plVar3[1] + lVar6 * 8) = extraout_RAX_04;
  plVar3[2] = lVar6 + 1;
  *puVar4 = uVar1;
  *puVar5 = uVar2;
  country_core::systems::collision::world::RaycastWorld::subscribe_to_changes();
  lVar6 = plVar3[2];
  if (lVar6 == *plVar3) {
    alloc::raw_vec::RawVec::
    grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
              ();
  }
  *(undefined8 *)(plVar3[1] + lVar6 * 8) = extraout_RAX_05;
  plVar3[2] = lVar6 + 1;
  *puVar4 = uVar1;
  *puVar5 = uVar2;
  country_core::systems::collision::world::RaycastWorld::subscribe_to_changes();
  lVar6 = plVar3[2];
  if (lVar6 == *plVar3) {
    alloc::raw_vec::RawVec::
    grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
              ();
  }
  *(undefined8 *)(plVar3[1] + lVar6 * 8) = extraout_RAX_06;
  plVar3[2] = lVar6 + 1;
  *puVar4 = uVar1;
  **(undefined4 **)(in_R8 + 0x10) = *(undefined4 *)(in_R8 + 0x1c);
  tabloid::Publisher::subscribe<country_core::utils::dirty_tiles::DirtyTiles>();
  lVar6 = plVar3[2];
  if (lVar6 == *plVar3) {
    alloc::raw_vec::RawVec::
    grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
              ();
  }
  *(undefined8 *)(plVar3[1] + lVar6 * 8) = extraout_RAX_07;
  plVar3[2] = lVar6 + 1;
  *puVar4 = uVar1;
  **(undefined4 **)(in_R9 + 0x10) = *(undefined4 *)(in_R9 + 0x1c);
  toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
  country_core::resources::world_raster::WorldRaster::subscribe<512>();
  lVar6 = plVar3[2];
  if (lVar6 == *plVar3) {
    alloc::raw_vec::RawVec::
    grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
              ();
  }
  *(undefined8 *)(plVar3[1] + lVar6 * 8) = extraout_RAX_08;
  plVar3[2] = lVar6 + 1;
  *puVar4 = uVar1;
  **(undefined4 **)(in_stack_00000028 + 0x10) = *(undefined4 *)(in_stack_00000028 + 0x1c);
  toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
  country_core::resources::world_raster::WorldRaster::subscribe<512>();
  lVar6 = plVar3[2];
  if (lVar6 == *plVar3) {
    alloc::raw_vec::RawVec::
    grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
              ();
  }
  *(undefined8 *)(plVar3[1] + lVar6 * 8) = extraout_RAX_09;
  plVar3[2] = lVar6 + 1;
  *puVar4 = uVar1;
  **(undefined4 **)(in_stack_00000030 + 0x10) = *(undefined4 *)(in_stack_00000030 + 0x1c);
  toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
  country_core::resources::world_raster::WorldRaster::subscribe<512>();
  lVar6 = plVar3[2];
  if (lVar6 == *plVar3) {
    alloc::raw_vec::RawVec::
    grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
              ();
  }
  *(undefined8 *)(plVar3[1] + lVar6 * 8) = extraout_RAX_10;
  plVar3[2] = lVar6 + 1;
  *puVar4 = uVar1;
  **(undefined4 **)(in_stack_00000038 + 0x10) = *(undefined4 *)(in_stack_00000038 + 0x1c);
  tabloid::Publisher::subscribe<country_core::utils::dirty_tiles::DirtyTiles>();
  lVar6 = plVar3[2];
  if (lVar6 == *plVar3) {
    alloc::raw_vec::RawVec::
    grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
              ();
  }
  *(undefined8 *)(plVar3[1] + lVar6 * 8) = extraout_RAX_11;
  plVar3[2] = lVar6 + 1;
  return;
}

