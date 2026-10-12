
void __cdecl system_roof::create_roof::create_roof(void)

{
  undefined8 *puVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  ulonglong *puVar6;
  longlong lVar7;
  undefined4 *puVar8;
  longlong lVar9;
  undefined4 *puVar10;
  longlong lVar11;
  undefined4 *puVar12;
  undefined8 uVar13;
  longlong lVar14;
  undefined8 uVar15;
  longlong lVar16;
  undefined4 uVar17;
  char extraout_AL;
  char extraout_AL_00;
  uint extraout_EAX;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  undefined8 *in_RCX;
  longlong lVar18;
  undefined1 uVar19;
  ulonglong uVar20;
  ulonglong uVar21;
  longlong *in_R8;
  longlong *in_R9;
  ulonglong uVar22;
  ulonglong uVar23;
  ulonglong uVar24;
  longlong *in_stack_00000028;
  undefined8 local_378;
  undefined4 uStack_368;
  undefined4 uStack_364;
  undefined4 uStack_360;
  undefined8 local_250;
  undefined4 uStack_1b4;
  undefined8 local_1b0;
  undefined8 uStack_1a8;
  undefined4 uStack_148;
  ulonglong local_c0;
  ulonglong local_b8;
  
  puVar6 = (ulonglong *)*in_RCX;
  lVar7 = in_RCX[1];
  uVar21 = *puVar6;
  uVar22 = 0;
  uVar23 = uVar21 - *(ulonglong *)(lVar7 + 0x18);
  if (uVar21 < *(ulonglong *)(lVar7 + 0x18)) {
    uVar23 = uVar22;
  }
  uVar20 = uVar21 - *(ulonglong *)(lVar7 + 0x38);
  if (uVar21 < *(ulonglong *)(lVar7 + 0x38)) {
    uVar20 = uVar22;
  }
  local_b8 = uVar23 * 0x40 + *(longlong *)(lVar7 + 8);
  uVar21 = *(ulonglong *)(lVar7 + 0x10) - uVar23;
  if (*(ulonglong *)(lVar7 + 0x10) < uVar23) {
    local_b8 = 8;
    uVar21 = uVar22;
  }
  local_c0 = uVar20 * 0x40 + *(longlong *)(lVar7 + 0x28);
  uVar23 = *(ulonglong *)(lVar7 + 0x30) - uVar20;
  if (*(ulonglong *)(lVar7 + 0x30) < uVar20) {
    local_c0 = 8;
    uVar23 = uVar22;
  }
  *puVar6 = (*(longlong *)(lVar7 + 0x40) - uVar23) - uVar21;
  uVar22 = uVar21 * 0x40 + local_b8;
  uVar23 = uVar23 * 0x40 + local_c0;
  uVar2 = *(undefined4 *)((longlong)in_R9 + 0x1c);
  lVar7 = *in_R9;
  puVar8 = (undefined4 *)in_R9[2];
  uVar3 = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
  lVar9 = *in_stack_00000028;
  puVar10 = (undefined4 *)in_stack_00000028[2];
  uVar4 = *(undefined4 *)((longlong)in_R8 + 0x1c);
  lVar11 = *in_R8;
  puVar12 = (undefined4 *)in_R8[2];
  uVar21 = uVar23;
  while( true ) {
    if ((local_b8 == 0) || (local_b8 == uVar22)) {
      if ((local_c0 == 0) || (local_c0 == uVar23)) {
        return;
      }
      local_b8 = 0;
      uVar20 = local_c0;
      local_c0 = local_c0 + 0x40;
    }
    else {
      uVar20 = local_b8;
      local_b8 = local_b8 + 0x40;
    }
    *puVar6 = *puVar6 + 1;
    country_core::resources::walls::public_walls::PublicWalls::get();
    if (extraout_RAX == 0) {
                    /* WARNING: Subroutine does not return */
      core::option::unwrap_failed();
    }
    country_core::resources::walls::public_walls::PublicWalls::get();
    if (extraout_RAX_00 == 0) break;
    if (1 < *(uint *)(extraout_RAX_00 + 600)) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_fmt();
    }
    country_core::resources::walls::public_wall_state::PublicWallState::max_y();
    country_core::resources::walls::inner_wall_state::WallStyle::is_halftimbered();
    uVar5 = *(undefined4 *)(uVar20 + 0x1c);
    uVar13 = *(undefined8 *)(uVar20 + 0x14);
    country_core::resources::roofs::roof_shape::Roof::new();
    country_core::resources::roofs::roof_shape::Roof::ty();
    country_core::resources::roofs::roof_shape::Roof::is_gable();
    uVar24 = uVar21;
    if (extraout_AL_00 != '\0') {
      *puVar8 = uVar2;
      lVar14 = *(longlong *)(lVar7 + 0x30);
      uVar15 = *(undefined8 *)(lVar7 + 0x40);
      if (lVar14 == *(longlong *)(lVar7 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      lVar16 = *(longlong *)(lVar7 + 0x28);
      *(undefined8 *)(lVar16 + lVar14 * 0x10) = uVar15;
      *(undefined8 *)(lVar16 + 8 + lVar14 * 0x10) = local_250;
      *(longlong *)(lVar7 + 0x30) = lVar14 + 1;
      *(longlong *)(lVar7 + 0x40) = *(longlong *)(lVar7 + 0x40) + 1;
      uVar24 = uVar21 & 0xffffffff;
    }
    country_core::resources::roofs::roof_tile_sim::RoofTileSimulation::new();
    country_core::resources::roofs::roof_trigger_volume::impl$0::default();
    country_core::resources::roofs::roof_animation::RoofAnimation::new();
    _ZN100__LT_country_core__components__drawable__WireframePass_u20_as_u20_bevy_ecs__component__Component_GT_14clone_behavior17h98754d446e2f5bc0E
              ();
    country_core::resources::walls::decorator::DecoratorRank::new();
    country_core::components::drawable::impl$1::from();
    bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
    uVar17 = DAT_142ff76e4;
    uStack_148 = (undefined4)
                 ((ulonglong)anon_e4cb94d12ad201d370fa6509c913f15a_2_llvm_10471519311377478581 >>
                 0x20);
    uStack_368 = (undefined4)local_1b0;
    uStack_364 = (undefined4)((ulonglong)local_1b0 >> 0x20);
    uStack_360 = (undefined4)uStack_1a8;
    bevy_ecs::system::commands::Commands::
    spawn<tuple_<country_core::resources::roofs::roof_shape::Roof,country_core::resources::roofs::roof_tile_sim::RoofTileSimulation,country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,country_core::resources::roofs::roof_animation::RoofAnimation,country_core::resources::roofs::roof_tiles::RoofTiles,system_roof::utils::components::RoofSpire,system_roof::utils::components::RoofSupportBeams,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StoneFloorInstanceData>_>_>
              ();
    if (extraout_AL == '\0') {
      country_core::resources::walls::inner_wall_state::WallStyle::is_halftimbered();
      uVar24 = (ulonglong)extraout_EAX;
      uVar19 = 6;
    }
    else {
      uVar19 = 0;
    }
    country_core::systems::wall::valid_enclosure::ShapeParameters::center();
    ::utils::swizzlers::impl$1::xvy();
    uVar21 = uVar24 & 0xffffffff;
    local_378._2_6_ = (undefined6)((ulonglong)uVar13 >> 0x10);
    local_378._0_2_ = CONCAT11((char)uVar24,uVar19);
    *puVar10 = uVar3;
    lVar14 = *(longlong *)(lVar9 + 0x30);
    uVar13 = *(undefined8 *)(lVar9 + 0x40);
    if (lVar14 == *(longlong *)(lVar9 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                ();
    }
    lVar16 = *(longlong *)(lVar9 + 0x28);
    *(undefined8 *)(lVar16 + lVar14 * 0x28) = uVar13;
    puVar1 = (undefined8 *)(lVar16 + 0x14 + lVar14 * 0x28);
    *puVar1 = CONCAT44(uStack_368,uStack_1b4);
    puVar1[1] = CONCAT44(uStack_360,uStack_364);
    puVar1 = (undefined8 *)(lVar16 + 8 + lVar14 * 0x28);
    *puVar1 = local_378;
    puVar1[1] = CONCAT44(uStack_1b4,uVar5);
    *(longlong *)(lVar9 + 0x30) = lVar14 + 1;
    *(longlong *)(lVar9 + 0x40) = *(longlong *)(lVar9 + 0x40) + 1;
    uVar13 = *(undefined8 *)(uVar20 + 8);
    *puVar12 = uVar4;
    lVar14 = *(longlong *)(lVar11 + 0x30);
    uVar15 = *(undefined8 *)(lVar11 + 0x40);
    if (lVar14 == *(longlong *)(lVar11 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                ();
    }
    lVar16 = *(longlong *)(lVar11 + 0x28);
    lVar18 = lVar14 * 0x20;
    *(undefined8 *)(lVar16 + lVar18) = uVar15;
    *(ulonglong *)(lVar16 + 8 + lVar18) = CONCAT44(uVar17,uStack_148);
    *(undefined8 *)(lVar16 + 0x10 + lVar18) = uVar13;
    *(undefined1 *)(lVar16 + 0x18 + lVar18) = 0;
    *(longlong *)(lVar11 + 0x30) = lVar14 + 1;
    *(longlong *)(lVar11 + 0x40) = *(longlong *)(lVar11 + 0x40) + 1;
  }
                    /* WARNING: Subroutine does not return */
  core::option::unwrap_failed();
}

