// add_wall_events
// sig: void __cdecl add_wall_events(void)
// size: 5473

void __cdecl tiny_glade::app_systems::impl$0::add_wall_events(void)

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
  byte *extraout_RAX_08;
  longlong extraout_RAX_09;
  byte *extraout_RAX_10;
  longlong extraout_RAX_11;
  byte *extraout_RAX_12;
  longlong extraout_RAX_13;
  byte *extraout_RAX_14;
  longlong extraout_RAX_15;
  byte *extraout_RAX_16;
  longlong extraout_RAX_17;
  byte *extraout_RAX_18;
  longlong extraout_RAX_19;
  longlong extraout_RAX_20;
  longlong extraout_RAX_21;
  longlong extraout_RAX_22;
  longlong extraout_RAX_23;
  longlong extraout_RAX_24;
  longlong extraout_RAX_25;
  longlong extraout_RAX_26;
  longlong extraout_RAX_27;
  longlong extraout_RAX_28;
  longlong extraout_RAX_29;
  longlong extraout_RAX_30;
  byte *extraout_RAX_31;
  longlong extraout_RAX_32;
  byte *extraout_RAX_33;
  longlong extraout_RAX_34;
  byte *extraout_RAX_35;
  longlong extraout_RAX_36;
  longlong *extraout_RAX_37;
  byte *extraout_RAX_38;
  longlong extraout_RAX_39;
  byte *extraout_RAX_40;
  longlong extraout_RAX_41;
  longlong *extraout_RAX_42;
  longlong extraout_RAX_43;
  longlong extraout_RAX_44;
  longlong extraout_RAX_45;
  longlong extraout_RAX_46;
  longlong extraout_RAX_47;
  longlong extraout_RAX_48;
  longlong extraout_RAX_49;
  longlong extraout_RAX_50;
  longlong extraout_RAX_51;
  longlong extraout_RAX_52;
  longlong extraout_RAX_53;
  longlong extraout_RAX_54;
  longlong extraout_RAX_55;
  longlong extraout_RAX_56;
  longlong extraout_RAX_57;
  longlong extraout_RAX_58;
  longlong extraout_RAX_59;
  longlong extraout_RAX_60;
  longlong extraout_RAX_61;
  longlong extraout_RAX_62;
  longlong extraout_RAX_63;
  longlong extraout_RAX_64;
  longlong extraout_RAX_65;
  longlong in_RCX;
  ulonglong uVar1;
  undefined5 *puVar2;
  undefined5 *puVar3;
  undefined5 local_2f8;
  undefined4 uStack_2f3;
  undefined4 uStack_2ef;
  undefined3 uStack_2eb;
  ulonglong local_2e8;
  undefined *puStack_2e0;
  longlong local_2d8;
  undefined5 *puStack_2d0;
  undefined *local_2c8;
  longlong lStack_2c0;
  undefined5 *local_2b8;
  undefined *puStack_2b0;
  longlong local_2a8;
  undefined5 *puStack_2a0;
  undefined *local_298;
  longlong lStack_290;
  undefined5 *local_288;
  undefined *puStack_280;
  longlong local_278;
  undefined5 *local_270;
  undefined8 uStack_268;
  longlong local_260;
  longlong lStack_258;
  undefined8 local_1d8;
  undefined *local_d8;
  undefined8 uStack_d0;
  undefined8 local_c8;
  undefined8 uStack_c0;
  longlong local_b8;
  undefined5 *puStack_b0;
  undefined *local_a8;
  undefined8 uStack_a0;
  undefined8 local_98;
  undefined8 uStack_90;
  longlong local_88;
  undefined5 *puStack_80;
  undefined8 local_70;
  
  local_70 = 0xfffffffffffffffe;
  uVar1 = 0xe720026d3fbfc97d;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX == 0) || (*(ulonglong *)(in_RCX + 0x218) <= uVar1)) ||
      (uVar1 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + uVar1 * 8), uVar1 == 0)) ||
     (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::resources::events::InitAdjWallHeightMode>();
  }
  uVar1 = 0x8042b3a7b1372c00;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_00 == 0) || (*(ulonglong *)(in_RCX + 0x218) <= uVar1)) ||
     ((uVar1 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + uVar1 * 8), uVar1 == 0 ||
      (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::resources::events::FinishAdjWallHeightMode>();
  }
  uVar1 = 0xa8a10fd8c046fe59;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_01 == 0) || (*(ulonglong *)(in_RCX + 0x218) <= uVar1)) ||
     ((uVar1 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + uVar1 * 8), uVar1 == 0 ||
      (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>
              ();
  }
  uVar1 = 0x8513c2de71b39e70;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX_02 == 0) || (*(ulonglong *)(in_RCX + 0x218) <= uVar1)) ||
      (uVar1 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + uVar1 * 8), uVar1 == 0)) ||
     (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::resources::events::FinishWall>();
  }
  uVar1 = 0x79e71d8ed7ab7fdf;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_03 == 0) || (*(ulonglong *)(in_RCX + 0x218) <= uVar1)) ||
     ((uVar1 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + uVar1 * 8), uVar1 == 0 ||
      (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::resources::events::RecordMouseInputIntoWall>();
  }
  uVar1 = 0x1b0e037e491a7be3;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_04 == 0) || (*(ulonglong *)(in_RCX + 0x218) <= uVar1)) ||
     ((uVar1 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + uVar1 * 8), uVar1 == 0 ||
      (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::systems::wall::wall_state::wall_roof_hashmap::RoofIsInvalid>();
  }
  uVar1 = 0xe25be0e1685456bf;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX_05 == 0) || (*(ulonglong *)(in_RCX + 0x218) <= uVar1)) ||
      (uVar1 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + uVar1 * 8), uVar1 == 0)) ||
     (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<system_wall_constructor::ui_arch_controls::DoUiArchControls>();
  }
  local_2f8 = 0;
  uStack_2f3 = CONCAT13(uStack_2f3._3_1_,0x800000);
  bevy_app::app::App::
  insert_resource<country_core::systems::wall::finish_wall::ContinueWallOriginalState>();
  country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::new();
  bevy_app::app::App::
  insert_resource<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>();
  bevy_app::app::App::insert_resource<country_core::systems::wall::ui_draw_wall::WallDrawState>();
  bevy_app::app::App::
  insert_resource<country_core::systems::wall::ui_continue_wall::WallContinueState>();
  uVar1 = 0x22d15b1cde734cb4;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_07 == 0) || (*(ulonglong *)(extraout_RAX_06 + 0x218) <= uVar1)) ||
     ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX_06 + 0x210) + uVar1 * 8), uVar1 == 0 ||
      (*(longlong *)(*(longlong *)(extraout_RAX_06 + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::systems::wall::calc_wall_bend::WallBendChangedEvent>();
  }
  local_2f8 = 0;
  uStack_2f3 = 0;
  puVar2 = &local_2f8;
  bevy_app::app::App::
  insert_resource<country_core::systems::wall::ui_continue_wall::SuppressContinueWall>();
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX_08 & 1) == 0) {
    std::sys::random::hashmap_random_keys();
    extraout_RAX_08[0] = 1;
    extraout_RAX_08[1] = 0;
    extraout_RAX_08[2] = 0;
    extraout_RAX_08[3] = 0;
    extraout_RAX_08[4] = 0;
    extraout_RAX_08[5] = 0;
    extraout_RAX_08[6] = 0;
    extraout_RAX_08[7] = 0;
    *(longlong *)(extraout_RAX_08 + 8) = extraout_RAX_09;
    *(undefined5 **)(extraout_RAX_08 + 0x10) = puVar2;
    local_b8 = extraout_RAX_09;
  }
  else {
    local_b8 = *(longlong *)(extraout_RAX_08 + 8);
    puVar2 = *(undefined5 **)(extraout_RAX_08 + 0x10);
  }
  *(longlong *)(extraout_RAX_08 + 8) = local_b8 + 1;
  local_c8 = 0;
  uStack_c0 = 0;
  local_d8 = &anon_c3d6e72739ce5bf70a9a302b2220936f_39_llvm_1651378587281896620;
  uStack_d0 = 0;
  puStack_b0 = puVar2;
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX_10 & 1) == 0) {
    std::sys::random::hashmap_random_keys();
    extraout_RAX_10[0] = 1;
    extraout_RAX_10[1] = 0;
    extraout_RAX_10[2] = 0;
    extraout_RAX_10[3] = 0;
    extraout_RAX_10[4] = 0;
    extraout_RAX_10[5] = 0;
    extraout_RAX_10[6] = 0;
    extraout_RAX_10[7] = 0;
    *(longlong *)(extraout_RAX_10 + 8) = extraout_RAX_11;
    *(undefined5 **)(extraout_RAX_10 + 0x10) = puVar2;
    local_88 = extraout_RAX_11;
  }
  else {
    local_88 = *(longlong *)(extraout_RAX_10 + 8);
    puVar2 = *(undefined5 **)(extraout_RAX_10 + 0x10);
  }
  *(longlong *)(extraout_RAX_10 + 8) = local_88 + 1;
  local_98 = 0;
  uStack_90 = 0;
  local_a8 = &anon_c3d6e72739ce5bf70a9a302b2220936f_39_llvm_1651378587281896620;
  uStack_a0 = 0;
  puStack_80 = puVar2;
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX_12 & 1) == 0) {
    std::sys::random::hashmap_random_keys();
    extraout_RAX_12[0] = 1;
    extraout_RAX_12[1] = 0;
    extraout_RAX_12[2] = 0;
    extraout_RAX_12[3] = 0;
    extraout_RAX_12[4] = 0;
    extraout_RAX_12[5] = 0;
    extraout_RAX_12[6] = 0;
    extraout_RAX_12[7] = 0;
    *(longlong *)(extraout_RAX_12 + 8) = extraout_RAX_13;
    *(undefined5 **)(extraout_RAX_12 + 0x10) = puVar2;
    local_278 = extraout_RAX_13;
  }
  else {
    local_278 = *(longlong *)(extraout_RAX_12 + 8);
    puVar2 = *(undefined5 **)(extraout_RAX_12 + 0x10);
  }
  *(longlong *)(extraout_RAX_12 + 8) = local_278 + 1;
  local_2d8 = local_b8;
  puStack_2d0 = puStack_b0;
  local_2e8 = local_c8;
  puStack_2e0 = (undefined *)uStack_c0;
  local_2f8 = SUB85(local_d8,0);
  uStack_2f3._0_3_ = (undefined3)((ulonglong)local_d8 >> 0x28);
  uStack_2f3._3_1_ = (undefined1)uStack_d0;
  uStack_2ef = (undefined4)((ulonglong)uStack_d0 >> 8);
  uStack_2eb = (undefined3)((ulonglong)uStack_d0 >> 0x28);
  local_2c8 = local_a8;
  lStack_2c0 = uStack_a0;
  local_2b8 = (undefined5 *)local_98;
  puStack_2b0 = (undefined *)uStack_90;
  local_2a8 = local_88;
  puStack_2a0 = puStack_80;
  local_298 = &anon_c3d6e72739ce5bf70a9a302b2220936f_39_llvm_1651378587281896620;
  lStack_290 = 0;
  local_288 = (undefined5 *)0x0;
  puStack_280 = (undefined *)0x0;
  puVar3 = &local_2f8;
  local_270 = puVar2;
  bevy_app::app::App::insert_resource<country_core::resources::walls::public_walls::PublicWalls>();
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX_14 & 1) == 0) {
    std::sys::random::hashmap_random_keys();
    extraout_RAX_14[0] = 1;
    extraout_RAX_14[1] = 0;
    extraout_RAX_14[2] = 0;
    extraout_RAX_14[3] = 0;
    extraout_RAX_14[4] = 0;
    extraout_RAX_14[5] = 0;
    extraout_RAX_14[6] = 0;
    extraout_RAX_14[7] = 0;
    *(longlong *)(extraout_RAX_14 + 8) = extraout_RAX_15;
    *(undefined5 **)(extraout_RAX_14 + 0x10) = puVar3;
    local_2d8 = extraout_RAX_15;
  }
  else {
    local_2d8 = *(longlong *)(extraout_RAX_14 + 8);
    puVar3 = *(undefined5 **)(extraout_RAX_14 + 0x10);
  }
  *(longlong *)(extraout_RAX_14 + 8) = local_2d8 + 1;
  local_2e8 = 0;
  puStack_2e0 = (undefined *)0x0;
  local_2f8 = 0x1429df030;
  uStack_2f3 = 0;
  uStack_2ef = 0;
  uStack_2eb = 0;
  puVar2 = &local_2f8;
  puStack_2d0 = puVar3;
  bevy_app::app::App::insert_resource<country_core::resources::walls::public_walls::WallEntities>();
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX_16 & 1) == 0) {
    std::sys::random::hashmap_random_keys();
    extraout_RAX_16[0] = 1;
    extraout_RAX_16[1] = 0;
    extraout_RAX_16[2] = 0;
    extraout_RAX_16[3] = 0;
    extraout_RAX_16[4] = 0;
    extraout_RAX_16[5] = 0;
    extraout_RAX_16[6] = 0;
    extraout_RAX_16[7] = 0;
    *(longlong *)(extraout_RAX_16 + 8) = extraout_RAX_17;
    *(undefined5 **)(extraout_RAX_16 + 0x10) = puVar2;
    local_2d8 = extraout_RAX_17;
  }
  else {
    local_2d8 = *(longlong *)(extraout_RAX_16 + 8);
    puVar2 = *(undefined5 **)(extraout_RAX_16 + 0x10);
  }
  *(longlong *)(extraout_RAX_16 + 8) = local_2d8 + 1;
  local_2e8 = 0;
  puStack_2e0 = (undefined *)0x0;
  local_2f8 = 0x1429df030;
  uStack_2f3 = 0;
  uStack_2ef = 0;
  uStack_2eb = 0;
  puVar3 = &local_2f8;
  puStack_2d0 = puVar2;
  bevy_app::app::App::insert_resource<country_core::resources::walls::public_walls::WallPlasters>();
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX_18 & 1) == 0) {
    std::sys::random::hashmap_random_keys();
    extraout_RAX_18[0] = 1;
    extraout_RAX_18[1] = 0;
    extraout_RAX_18[2] = 0;
    extraout_RAX_18[3] = 0;
    extraout_RAX_18[4] = 0;
    extraout_RAX_18[5] = 0;
    extraout_RAX_18[6] = 0;
    extraout_RAX_18[7] = 0;
    *(longlong *)(extraout_RAX_18 + 8) = extraout_RAX_19;
    *(undefined5 **)(extraout_RAX_18 + 0x10) = puVar3;
    local_2d8 = extraout_RAX_19;
  }
  else {
    local_2d8 = *(longlong *)(extraout_RAX_18 + 8);
    puVar3 = *(undefined5 **)(extraout_RAX_18 + 0x10);
  }
  *(longlong *)(extraout_RAX_18 + 8) = local_2d8 + 1;
  local_2e8 = 0;
  puStack_2e0 = (undefined *)0x0;
  local_2f8 = 0x1429df030;
  uStack_2f3._0_3_ = 0;
  uStack_2f3._3_1_ = 0;
  uStack_2ef = 0;
  uStack_2eb = 0;
  puStack_2d0 = puVar3;
  bevy_app::app::App::insert_resource<country_core::resources::walls::plaster::AnimatedPlasterPeel>
            ();
  country_core::resources::walls::inner_walls::InnerWalls::new();
  bevy_app::app::App::insert_resource<country_core::resources::walls::inner_walls::InnerWalls>();
  local_2f8 = 0;
  uStack_2f3 = CONCAT13(uStack_2f3._3_1_,0x800000);
  bevy_app::app::App::
  insert_resource<country_core::resources::walls::in_progress_wall::InProgressWallResource>();
  local_2e8 = local_2e8 & 0xffffffffffffff00;
  bevy_app::app::App::insert_resource<country_core::resources::walls::WallHighlight>();
  uStack_2ef = 0;
  local_2f8 = 0;
  uStack_2f3 = 0;
  bevy_app::app::App::insert_resource<country_core::resources::editing_highlight::EditingHighlight>
            ();
  uVar1 = 0x576dff9de241abe1;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_21 == 0) || (*(ulonglong *)(extraout_RAX_20 + 0x218) <= uVar1)) ||
     ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX_20 + 0x210) + uVar1 * 8), uVar1 == 0 ||
      (*(longlong *)(*(longlong *)(extraout_RAX_20 + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>
              ();
  }
  uVar1 = 0x601cb891fc59f04e;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX_22 == 0) || (*(ulonglong *)(extraout_RAX_20 + 0x218) <= uVar1)) ||
      (uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX_20 + 0x210) + uVar1 * 8), uVar1 == 0)) ||
     (*(longlong *)(*(longlong *)(extraout_RAX_20 + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
              ();
  }
  uVar1 = 0xd50d9d88448b147e;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_23 == 0) || (*(ulonglong *)(extraout_RAX_20 + 0x218) <= uVar1)) ||
     ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX_20 + 0x210) + uVar1 * 8), uVar1 == 0 ||
      (*(longlong *)(*(longlong *)(extraout_RAX_20 + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>
              ();
  }
  uVar1 = 0x8dd7de3841b9d690;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_24 == 0) || (*(ulonglong *)(extraout_RAX_20 + 0x218) <= uVar1)) ||
     ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX_20 + 0x210) + uVar1 * 8), uVar1 == 0 ||
      (*(longlong *)(*(longlong *)(extraout_RAX_20 + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateGableRoofWallHeights>
              ();
  }
  uVar1 = 0xd64c11374a7ca229;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX_25 == 0) || (*(ulonglong *)(extraout_RAX_20 + 0x218) <= uVar1)) ||
      (uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX_20 + 0x210) + uVar1 * 8), uVar1 == 0)) ||
     (*(longlong *)(*(longlong *)(extraout_RAX_20 + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallHoles>
              ();
  }
  uVar1 = 0x3f61271c4ced0f0a;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_26 == 0) || (*(ulonglong *)(extraout_RAX_20 + 0x218) <= uVar1)) ||
     ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX_20 + 0x210) + uVar1 * 8), uVar1 == 0 ||
      (*(longlong *)(*(longlong *)(extraout_RAX_20 + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>
              ();
  }
  uVar1 = 0x562171a7baf8e365;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_27 == 0) || (*(ulonglong *)(extraout_RAX_20 + 0x218) <= uVar1)) ||
     ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX_20 + 0x210) + uVar1 * 8), uVar1 == 0 ||
      (*(longlong *)(*(longlong *)(extraout_RAX_20 + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::resources::walls::wall_stone_columns::StoneColumnsChanged>();
  }
  country_core::systems::wall::snapping::SnappingMode::new();
  bevy_app::app::App::insert_resource<country_core::systems::wall::snapping::SnappingMode>();
  uVar1 = 0x585c5fad562d094d;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX_29 == 0) || (*(ulonglong *)(extraout_RAX_28 + 0x218) <= uVar1)) ||
      (uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX_28 + 0x210) + uVar1 * 8), uVar1 == 0)) ||
     (*(longlong *)(*(longlong *)(extraout_RAX_28 + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::systems::wall::merge_walls::MergeWallsCmd>();
  }
  uVar1 = 0xb42d6290ba7df1f8;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_30 == 0) || (*(ulonglong *)(extraout_RAX_28 + 0x218) <= uVar1)) ||
     ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX_28 + 0x210) + uVar1 * 8), uVar1 == 0 ||
      (*(longlong *)(*(longlong *)(extraout_RAX_28 + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::systems::wall::snapping::telegraph_walls_are_merged::PlayLineAcrossWall>
              ();
  }
  puStack_2e0 = (undefined *)((ulonglong)puStack_2e0 & 0xffffffffffffff00);
  local_2f8 = 0;
  uStack_2f3._0_3_ = 0;
  uStack_2f3._3_1_ = 4;
  uStack_2ef = 0;
  uStack_2eb = 0;
  local_2e8 = 0;
  bevy_app::app::App::insert_resource<country_core::systems::wall::draw_wall::CurveNursery>();
  country_core::resources::history::wall_compound_edit::CompoundEditBuilder::new_edit_sequence();
  bevy_app::app::App::
  insert_resource<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>
            ();
  bevy_app::app::App::insert_resource<country_core::systems::wall::eraser::EraserSizeMultiplier>();
  local_2f8 = 2;
  uStack_2f3._0_3_ = 0;
  local_1d8 = 0x8000000000000000;
  bevy_app::app::App::
  insert_resource<country_core::systems::wall::deferred_wall_history::PendingWallHistory>();
  local_2f8 = 2;
  uStack_2f3 = CONCAT13(uStack_2f3._3_1_,0x800000);
  puVar2 = &local_2f8;
  bevy_app::app::App::
  insert_resource<country_core::systems::terrain::deferred_terrain_history::PendingTerrainHistory>()
  ;
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX_31 & 1) == 0) {
    std::sys::random::hashmap_random_keys();
    extraout_RAX_31[0] = 1;
    extraout_RAX_31[1] = 0;
    extraout_RAX_31[2] = 0;
    extraout_RAX_31[3] = 0;
    extraout_RAX_31[4] = 0;
    extraout_RAX_31[5] = 0;
    extraout_RAX_31[6] = 0;
    extraout_RAX_31[7] = 0;
    *(longlong *)(extraout_RAX_31 + 8) = extraout_RAX_32;
    *(undefined5 **)(extraout_RAX_31 + 0x10) = puVar2;
    local_2d8 = extraout_RAX_32;
  }
  else {
    local_2d8 = *(longlong *)(extraout_RAX_31 + 8);
    puVar2 = *(undefined5 **)(extraout_RAX_31 + 0x10);
  }
  *(longlong *)(extraout_RAX_31 + 8) = local_2d8 + 1;
  local_2e8 = 0;
  puStack_2e0 = (undefined *)0x0;
  local_2f8 = 0x1429df030;
  uStack_2f3 = 0;
  uStack_2ef = 0;
  uStack_2eb = 0;
  puVar3 = &local_2f8;
  puStack_2d0 = puVar2;
  bevy_app::app::App::insert_resource<system_wall_constructor::WallPathSegmentationMasks>();
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX_33 & 1) == 0) {
    std::sys::random::hashmap_random_keys();
    extraout_RAX_33[0] = 1;
    extraout_RAX_33[1] = 0;
    extraout_RAX_33[2] = 0;
    extraout_RAX_33[3] = 0;
    extraout_RAX_33[4] = 0;
    extraout_RAX_33[5] = 0;
    extraout_RAX_33[6] = 0;
    extraout_RAX_33[7] = 0;
    *(longlong *)(extraout_RAX_33 + 8) = extraout_RAX_34;
    *(undefined5 **)(extraout_RAX_33 + 0x10) = puVar3;
    local_2d8 = extraout_RAX_34;
  }
  else {
    local_2d8 = *(longlong *)(extraout_RAX_33 + 8);
    puVar3 = *(undefined5 **)(extraout_RAX_33 + 0x10);
  }
  *(longlong *)(extraout_RAX_33 + 8) = local_2d8 + 1;
  local_2e8 = 0;
  puStack_2e0 = (undefined *)0x0;
  local_2f8 = 0x1429df030;
  uStack_2f3 = 0;
  uStack_2ef = 0;
  uStack_2eb = 0;
  puVar2 = &local_2f8;
  puStack_2d0 = puVar3;
  bevy_app::app::App::insert_resource<system_wall_constructor::WallPathSegmentationMasksMinusStairs>
            ();
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX_35 & 1) == 0) {
    std::sys::random::hashmap_random_keys();
    extraout_RAX_35[0] = 1;
    extraout_RAX_35[1] = 0;
    extraout_RAX_35[2] = 0;
    extraout_RAX_35[3] = 0;
    extraout_RAX_35[4] = 0;
    extraout_RAX_35[5] = 0;
    extraout_RAX_35[6] = 0;
    extraout_RAX_35[7] = 0;
    *(longlong *)(extraout_RAX_35 + 8) = extraout_RAX_36;
    *(undefined5 **)(extraout_RAX_35 + 0x10) = puVar2;
    local_88 = extraout_RAX_36;
  }
  else {
    local_88 = *(longlong *)(extraout_RAX_35 + 8);
    puVar2 = *(undefined5 **)(extraout_RAX_35 + 0x10);
  }
  *(longlong *)(extraout_RAX_35 + 8) = local_88 + 1;
  local_98 = 0;
  uStack_90 = 0;
  local_a8 = &anon_c3d6e72739ce5bf70a9a302b2220936f_39_llvm_1651378587281896620;
  uStack_a0 = 0;
  puStack_80 = puVar2;
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX_37 != (longlong *)0x0) {
    lStack_290 = *extraout_RAX_37;
    local_288 = (undefined5 *)extraout_RAX_37[1];
    *extraout_RAX_37 = *extraout_RAX_37 + 1;
    lStack_2c0 = local_88;
    local_2b8 = puStack_80;
    puStack_2d0 = (undefined5 *)local_98;
    local_2c8 = (undefined *)uStack_90;
    puStack_2e0 = local_a8;
    local_2d8 = uStack_a0;
    local_2f8 = 0;
    uStack_2f3 = 0x8000000;
    uStack_2ef = 0;
    uStack_2eb = 0;
    local_2e8 = 0;
    puStack_2b0 = &anon_e34e5d0578d4d828d43b3955f8816025_65_llvm_2456333134789696354;
    local_2a8 = 0;
    puStack_2a0 = (undefined5 *)0x0;
    local_298 = (undefined *)0x0;
    puVar2 = &local_2f8;
    bevy_app::app::App::
    insert_resource<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>
              ();
    std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
    if ((*extraout_RAX_38 & 1) == 0) {
      std::sys::random::hashmap_random_keys();
      extraout_RAX_38[0] = 1;
      extraout_RAX_38[1] = 0;
      extraout_RAX_38[2] = 0;
      extraout_RAX_38[3] = 0;
      extraout_RAX_38[4] = 0;
      extraout_RAX_38[5] = 0;
      extraout_RAX_38[6] = 0;
      extraout_RAX_38[7] = 0;
      *(longlong *)(extraout_RAX_38 + 8) = extraout_RAX_39;
      *(undefined5 **)(extraout_RAX_38 + 0x10) = puVar2;
      local_b8 = extraout_RAX_39;
    }
    else {
      local_b8 = *(longlong *)(extraout_RAX_38 + 8);
      puVar2 = *(undefined5 **)(extraout_RAX_38 + 0x10);
    }
    *(longlong *)(extraout_RAX_38 + 8) = local_b8 + 1;
    local_c8 = 0;
    uStack_c0 = 0;
    local_d8 = &anon_c3d6e72739ce5bf70a9a302b2220936f_39_llvm_1651378587281896620;
    uStack_d0 = 0;
    puStack_b0 = puVar2;
    std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
    if ((*extraout_RAX_40 & 1) == 0) {
      std::sys::random::hashmap_random_keys();
      extraout_RAX_40[0] = 1;
      extraout_RAX_40[1] = 0;
      extraout_RAX_40[2] = 0;
      extraout_RAX_40[3] = 0;
      extraout_RAX_40[4] = 0;
      extraout_RAX_40[5] = 0;
      extraout_RAX_40[6] = 0;
      extraout_RAX_40[7] = 0;
      *(longlong *)(extraout_RAX_40 + 8) = extraout_RAX_41;
      *(undefined5 **)(extraout_RAX_40 + 0x10) = puVar2;
      local_88 = extraout_RAX_41;
    }
    else {
      local_88 = *(longlong *)(extraout_RAX_40 + 8);
      puVar2 = *(undefined5 **)(extraout_RAX_40 + 0x10);
    }
    *(longlong *)(extraout_RAX_40 + 8) = local_88 + 1;
    local_98 = 0;
    uStack_90 = 0;
    local_a8 = &anon_c3d6e72739ce5bf70a9a302b2220936f_39_llvm_1651378587281896620;
    uStack_a0 = 0;
    puStack_80 = puVar2;
    core::ops::function::FnOnce::
    call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
              ();
    if (extraout_RAX_42 != (longlong *)0x0) {
      local_260 = *extraout_RAX_42;
      lStack_258 = extraout_RAX_42[1];
      *extraout_RAX_42 = *extraout_RAX_42 + 1;
      lStack_2c0 = local_b8;
      local_2b8 = puStack_b0;
      puStack_2d0 = (undefined5 *)local_c8;
      local_2c8 = (undefined *)uStack_c0;
      puStack_2e0 = local_d8;
      local_2d8 = uStack_d0;
      puStack_2b0 = local_a8;
      local_2a8 = uStack_a0;
      puStack_2a0 = (undefined5 *)local_98;
      local_298 = (undefined *)uStack_90;
      lStack_290 = local_88;
      local_288 = puStack_80;
      local_2f8 = 0;
      uStack_2f3 = 0x8000000;
      uStack_2ef = 0;
      uStack_2eb = 0;
      local_2e8 = 0;
      puStack_280 = &anon_e34e5d0578d4d828d43b3955f8816025_65_llvm_2456333134789696354;
      local_278 = 0;
      local_270 = (undefined5 *)0x0;
      uStack_268 = 0;
      bevy_app::app::App::insert_resource<system_wall_constructor::PlatformStairsStorage>();
      uVar1 = 0x8803ee35630da027;
      bevy_ecs::component::Components::get_resource_id();
      if (((extraout_RAX_44 == 0) || (*(ulonglong *)(extraout_RAX_43 + 0x218) <= uVar1)) ||
         ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX_43 + 0x210) + uVar1 * 8), uVar1 == 0 ||
          (*(longlong *)(*(longlong *)(extraout_RAX_43 + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
        bevy_ecs::event::registry::EventRegistry::
        register_event<country_core::resources::rt::UpdateRtWorldEvent>();
      }
      uVar1 = 0xb0c94e6e65358b73;
      bevy_ecs::component::Components::get_resource_id();
      if ((((extraout_RAX_45 == 0) || (*(ulonglong *)(extraout_RAX_43 + 0x218) <= uVar1)) ||
          (uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX_43 + 0x210) + uVar1 * 8), uVar1 == 0))
         || (*(longlong *)(*(longlong *)(extraout_RAX_43 + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)) {
        bevy_ecs::event::registry::EventRegistry::
        register_event<system_wall_constructor::construct_gates::systems::ConstructGateCmd>();
      }
      uVar1 = 0x3a66e0ea923b0b;
      bevy_ecs::component::Components::get_resource_id();
      if (((extraout_RAX_46 == 0) || (*(ulonglong *)(extraout_RAX_43 + 0x218) <= uVar1)) ||
         ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX_43 + 0x210) + uVar1 * 8), uVar1 == 0 ||
          (*(longlong *)(*(longlong *)(extraout_RAX_43 + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
        bevy_ecs::event::registry::EventRegistry::
        register_event<country_core::resources::walls::plaster::CreatePlasterCmd>();
      }
      uVar1 = 0x6d8612b53a242390;
      bevy_ecs::component::Components::get_resource_id();
      if (((extraout_RAX_47 == 0) || (*(ulonglong *)(extraout_RAX_43 + 0x218) <= uVar1)) ||
         ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX_43 + 0x210) + uVar1 * 8), uVar1 == 0 ||
          (*(longlong *)(*(longlong *)(extraout_RAX_43 + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
        bevy_ecs::event::registry::EventRegistry::
        register_event<country_core::resources::walls::plaster::DeletePlasterCmd>();
      }
      uVar1 = 0x8d948fbffe45eab0;
      bevy_ecs::component::Components::get_resource_id();
      if ((((extraout_RAX_48 == 0) || (*(ulonglong *)(extraout_RAX_43 + 0x218) <= uVar1)) ||
          (uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX_43 + 0x210) + uVar1 * 8), uVar1 == 0))
         || (*(longlong *)(*(longlong *)(extraout_RAX_43 + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)) {
        bevy_ecs::event::registry::EventRegistry::
        register_event<country_core::resources::walls::plaster::RecalculatePlasterCmd>();
      }
      uVar1 = 0x9d80e00ee1d342b3;
      bevy_ecs::component::Components::get_resource_id();
      if (((extraout_RAX_49 == 0) || (*(ulonglong *)(extraout_RAX_43 + 0x218) <= uVar1)) ||
         ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX_43 + 0x210) + uVar1 * 8), uVar1 == 0 ||
          (*(longlong *)(*(longlong *)(extraout_RAX_43 + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
        bevy_ecs::event::registry::EventRegistry::
        register_event<country_core::resources::walls::plaster::RecalculateHalfTimberMeshCmd>();
      }
      uVar1 = 0xc1faf1912c106825;
      bevy_ecs::component::Components::get_resource_id();
      if (((extraout_RAX_50 == 0) || (*(ulonglong *)(extraout_RAX_43 + 0x218) <= uVar1)) ||
         ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX_43 + 0x210) + uVar1 * 8), uVar1 == 0 ||
          (*(longlong *)(*(longlong *)(extraout_RAX_43 + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
        bevy_ecs::event::registry::EventRegistry::
        register_event<country_core::resources::walls::plaster::RecalculateWoodenTurretCmd>();
      }
      uVar1 = 0xec7dad02bb4e5d35;
      bevy_ecs::component::Components::get_resource_id();
      if ((((extraout_RAX_51 == 0) || (*(ulonglong *)(extraout_RAX_43 + 0x218) <= uVar1)) ||
          (uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX_43 + 0x210) + uVar1 * 8), uVar1 == 0))
         || (*(longlong *)(*(longlong *)(extraout_RAX_43 + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)) {
        bevy_ecs::event::registry::EventRegistry::
        register_event<country_core::systems::wall::wall_highlight::HighlightWallCmd>();
      }
      uVar1 = 0x861319290d3fe10f;
      bevy_ecs::component::Components::get_resource_id();
      if (((extraout_RAX_52 == 0) || (*(ulonglong *)(extraout_RAX_43 + 0x218) <= uVar1)) ||
         ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX_43 + 0x210) + uVar1 * 8), uVar1 == 0 ||
          (*(longlong *)(*(longlong *)(extraout_RAX_43 + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
        bevy_ecs::event::registry::EventRegistry::
        register_event<country_core::systems::wall::structure_outline::OutlineWallCmd>();
      }
      uVar1 = 0xf0f612171767d68f;
      bevy_ecs::component::Components::get_resource_id();
      if (((extraout_RAX_53 == 0) || (*(ulonglong *)(extraout_RAX_43 + 0x218) <= uVar1)) ||
         ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX_43 + 0x210) + uVar1 * 8), uVar1 == 0 ||
          (*(longlong *)(*(longlong *)(extraout_RAX_43 + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
        bevy_ecs::event::registry::EventRegistry::
        register_event<country_core::systems::wall::structure_outline::OutlineStairsCmd>();
      }
      uVar1 = 0xf99c083c5514d906;
      bevy_ecs::component::Components::get_resource_id();
      if ((((extraout_RAX_54 == 0) || (*(ulonglong *)(extraout_RAX_43 + 0x218) <= uVar1)) ||
          (uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX_43 + 0x210) + uVar1 * 8), uVar1 == 0))
         || (*(longlong *)(*(longlong *)(extraout_RAX_43 + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)) {
        bevy_ecs::event::registry::EventRegistry::
        register_event<country_core::resources::events::OnRectangleEdited>();
      }
      uVar1 = 0xd26f4b443c1c2c89;
      bevy_ecs::component::Components::get_resource_id();
      if (((extraout_RAX_55 == 0) || (*(ulonglong *)(extraout_RAX_43 + 0x218) <= uVar1)) ||
         ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX_43 + 0x210) + uVar1 * 8), uVar1 == 0 ||
          (*(longlong *)(*(longlong *)(extraout_RAX_43 + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
        bevy_ecs::event::registry::EventRegistry::
        register_event<country_core::systems::wall::set_wall_style::SetWallStyleCmd>();
      }
      uVar1 = 0x7fa25bb007361d49;
      bevy_ecs::component::Components::get_resource_id();
      if (((extraout_RAX_56 == 0) || (*(ulonglong *)(extraout_RAX_43 + 0x218) <= uVar1)) ||
         ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX_43 + 0x210) + uVar1 * 8), uVar1 == 0 ||
          (*(longlong *)(*(longlong *)(extraout_RAX_43 + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
        bevy_ecs::event::registry::EventRegistry::
        register_event<country_core::systems::wall::set_wall_style::SetPlasterPeelAmountCmd>();
      }
      uVar1 = 0xfddfe3d8acc1440d;
      bevy_ecs::component::Components::get_resource_id();
      if ((((extraout_RAX_57 == 0) || (*(ulonglong *)(extraout_RAX_43 + 0x218) <= uVar1)) ||
          (uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX_43 + 0x210) + uVar1 * 8), uVar1 == 0))
         || (*(longlong *)(*(longlong *)(extraout_RAX_43 + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)) {
        bevy_ecs::event::registry::EventRegistry::
        register_event<country_core::systems::wall::set_wall_style::SetFreehandHeightTypeCmd>();
      }
      uVar1 = 0x70a391b8f514e3f3;
      bevy_ecs::component::Components::get_resource_id();
      if (((extraout_RAX_58 == 0) || (*(ulonglong *)(extraout_RAX_43 + 0x218) <= uVar1)) ||
         ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX_43 + 0x210) + uVar1 * 8), uVar1 == 0 ||
          (*(longlong *)(*(longlong *)(extraout_RAX_43 + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
        bevy_ecs::event::registry::EventRegistry::
        register_event<system_wall_constructor::construct_elevation_supports::RecalculateRectangleSupports>
                  ();
      }
      uVar1 = 0x2e3306176a07e5d4;
      bevy_ecs::component::Components::get_resource_id();
      if (((extraout_RAX_59 == 0) || (*(ulonglong *)(extraout_RAX_43 + 0x218) <= uVar1)) ||
         ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX_43 + 0x210) + uVar1 * 8), uVar1 == 0 ||
          (*(longlong *)(*(longlong *)(extraout_RAX_43 + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
        bevy_ecs::event::registry::EventRegistry::
        register_event<system_wall_constructor::construct_elevation_supports::RecalculateCircleSupports>
                  ();
      }
      uVar1 = 0x9514aca8b8fc2f68;
      bevy_ecs::component::Components::get_resource_id();
      if ((((extraout_RAX_60 == 0) || (*(ulonglong *)(extraout_RAX_43 + 0x218) <= uVar1)) ||
          (uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX_43 + 0x210) + uVar1 * 8), uVar1 == 0))
         || (*(longlong *)(*(longlong *)(extraout_RAX_43 + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)) {
        bevy_ecs::event::registry::EventRegistry::
        register_event<country_core::resources::events::OnShapeMoved>();
      }
      uVar1 = 0x82e26c7dfdccd0b0;
      bevy_ecs::component::Components::get_resource_id();
      if (((extraout_RAX_61 == 0) || (*(ulonglong *)(extraout_RAX_43 + 0x218) <= uVar1)) ||
         ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX_43 + 0x210) + uVar1 * 8), uVar1 == 0 ||
          (*(longlong *)(*(longlong *)(extraout_RAX_43 + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
        bevy_ecs::event::registry::EventRegistry::
        register_event<country_core::resources::events::ReanchorDecorators>();
      }
      uVar1 = 0xfe2c62427b81ef74;
      bevy_ecs::component::Components::get_resource_id();
      if (((extraout_RAX_62 == 0) || (*(ulonglong *)(extraout_RAX_43 + 0x218) <= uVar1)) ||
         ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX_43 + 0x210) + uVar1 * 8), uVar1 == 0 ||
          (*(longlong *)(*(longlong *)(extraout_RAX_43 + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
        bevy_ecs::event::registry::EventRegistry::
        register_event<country_core::resources::events::OnFreehandWallEdited>();
      }
      uVar1 = 0xbd45698a0818db2;
      bevy_ecs::component::Components::get_resource_id();
      if ((((extraout_RAX_63 == 0) || (*(ulonglong *)(extraout_RAX_43 + 0x218) <= uVar1)) ||
          (uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX_43 + 0x210) + uVar1 * 8), uVar1 == 0))
         || (*(longlong *)(*(longlong *)(extraout_RAX_43 + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)) {
        bevy_ecs::event::registry::EventRegistry::
        register_event<country_core::resources::events::OnCircleRotated>();
      }
      uVar1 = 0x4c979a39f3a96d98;
      bevy_ecs::component::Components::get_resource_id();
      if (((extraout_RAX_64 == 0) || (*(ulonglong *)(extraout_RAX_43 + 0x218) <= uVar1)) ||
         ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX_43 + 0x210) + uVar1 * 8), uVar1 == 0 ||
          (*(longlong *)(*(longlong *)(extraout_RAX_43 + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
        bevy_ecs::event::registry::EventRegistry::
        register_event<country_core::resources::events::RecalculateHalftimberBuildings>();
      }
      uVar1 = 0xbd0c373b550b02f1;
      bevy_ecs::component::Components::get_resource_id();
      if (((extraout_RAX_65 == 0) || (*(ulonglong *)(extraout_RAX_43 + 0x218) <= uVar1)) ||
         ((uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX_43 + 0x210) + uVar1 * 8), uVar1 == 0 ||
          (*(longlong *)(*(longlong *)(extraout_RAX_43 + 0x1e0) + 0x20 + ~uVar1 * 0x60) == 0)))) {
        bevy_ecs::event::registry::EventRegistry::
        register_event<country_core::resources::events::DisplayBetaWIPWarning>();
      }
      return;
    }
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
                    // WARNING: Subroutine does not return
  std::thread::local::panic_access_error();
}

