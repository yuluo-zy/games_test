
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl
system_clutter::inter_shape_stitches::detect_intra_shape_corners::WallAndRoofChanges::dirty_walls
          (void)

{
  ulonglong uVar1;
  ulonglong uVar2;
  byte *extraout_RAX;
  longlong extraout_RAX_00;
  undefined8 *in_RCX;
  undefined8 *in_RDX;
  longlong lVar3;
  longlong lVar4;
  longlong lVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  longlong lVar8;
  longlong lVar9;
  ulonglong local_210 [19];
  undefined8 local_178;
  ulonglong *local_170;
  longlong local_168;
  longlong local_160;
  longlong local_158;
  longlong local_150;
  longlong local_148;
  ulonglong *local_140;
  longlong local_138;
  longlong local_130;
  longlong local_128;
  longlong local_120;
  longlong local_118;
  ulonglong *local_110;
  longlong local_108;
  longlong local_100;
  longlong local_f8;
  longlong local_f0;
  longlong local_e8;
  ulonglong *local_e0;
  longlong local_d8;
  longlong local_d0;
  longlong local_c8;
  longlong local_c0;
  longlong local_b8;
  ulonglong *local_b0;
  longlong local_a8;
  longlong local_a0;
  longlong local_98;
  ulonglong *local_90;
  longlong local_88;
  undefined8 *local_80;
  undefined *local_78;
  undefined8 uStack_70;
  ulonglong local_68;
  undefined8 uStack_60;
  longlong local_58;
  longlong lStack_50;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  local_e0 = (ulonglong *)*in_RDX;
  lVar3 = in_RDX[1];
  uVar1 = *local_e0;
  uVar2 = 0;
  uVar7 = uVar1 - *(ulonglong *)(lVar3 + 0x18);
  if (uVar1 < *(ulonglong *)(lVar3 + 0x18)) {
    uVar7 = uVar2;
  }
  uVar6 = uVar1 - *(ulonglong *)(lVar3 + 0x38);
  if (uVar1 < *(ulonglong *)(lVar3 + 0x38)) {
    uVar6 = uVar2;
  }
  local_d8 = uVar7 * 0x10 + *(longlong *)(lVar3 + 8);
  uVar1 = *(ulonglong *)(lVar3 + 0x10) - uVar7;
  if (*(ulonglong *)(lVar3 + 0x10) < uVar7) {
    local_d8 = 8;
    uVar1 = uVar2;
  }
  local_c0 = uVar6 * 0x10 + *(longlong *)(lVar3 + 0x28);
  uVar7 = *(ulonglong *)(lVar3 + 0x30) - uVar6;
  if (*(ulonglong *)(lVar3 + 0x30) < uVar6) {
    local_c0 = 8;
    uVar7 = uVar2;
  }
  local_b8 = uVar7 + uVar1;
  *local_e0 = *(longlong *)(lVar3 + 0x40) - local_b8;
  local_c8 = uVar1 * 0x10 + local_d8;
  local_d0 = uVar7 * 0x10 + local_c0;
  local_b0 = (ulonglong *)in_RDX[5];
  lVar3 = in_RDX[6];
  uVar1 = *local_b0;
  uVar7 = uVar1 - *(ulonglong *)(lVar3 + 0x18);
  if (uVar1 < *(ulonglong *)(lVar3 + 0x18)) {
    uVar7 = uVar2;
  }
  uVar6 = uVar1 - *(ulonglong *)(lVar3 + 0x38);
  if (uVar1 < *(ulonglong *)(lVar3 + 0x38)) {
    uVar6 = uVar2;
  }
  local_a8 = uVar7 * 0x10 + *(longlong *)(lVar3 + 8);
  uVar1 = *(ulonglong *)(lVar3 + 0x10) - uVar7;
  if (*(ulonglong *)(lVar3 + 0x10) < uVar7) {
    local_a8 = 8;
    uVar1 = uVar2;
  }
  local_a0 = uVar6 * 0x10 + *(longlong *)(lVar3 + 0x28);
  uVar7 = *(ulonglong *)(lVar3 + 0x30) - uVar6;
  if (*(ulonglong *)(lVar3 + 0x30) < uVar6) {
    local_a0 = 8;
    uVar7 = uVar2;
  }
  local_98 = uVar1 + uVar7;
  *local_b0 = *(longlong *)(lVar3 + 0x40) - local_98;
  lVar5 = uVar1 * 0x10 + local_a8;
  lVar8 = uVar7 * 0x10 + local_a0;
  local_90 = (ulonglong *)in_RDX[10];
  lVar3 = in_RDX[0xb];
  uVar1 = *local_90;
  uVar7 = uVar1 - *(ulonglong *)(lVar3 + 0x18);
  if (uVar1 < *(ulonglong *)(lVar3 + 0x18)) {
    uVar7 = uVar2;
  }
  uVar6 = uVar1 - *(ulonglong *)(lVar3 + 0x38);
  if (uVar1 < *(ulonglong *)(lVar3 + 0x38)) {
    uVar6 = uVar2;
  }
  lVar9 = uVar7 * 0x20 + *(longlong *)(lVar3 + 8);
  uVar1 = *(ulonglong *)(lVar3 + 0x10) - uVar7;
  if (*(ulonglong *)(lVar3 + 0x10) < uVar7) {
    lVar9 = 8;
    uVar1 = uVar2;
  }
  lVar4 = uVar6 * 0x20 + *(longlong *)(lVar3 + 0x28);
  uVar7 = *(ulonglong *)(lVar3 + 0x30) - uVar6;
  if (*(ulonglong *)(lVar3 + 0x30) < uVar6) {
    lVar4 = 8;
    uVar7 = uVar2;
  }
  local_88 = uVar7 + uVar1;
  *local_90 = *(longlong *)(lVar3 + 0x40) - local_88;
  local_80 = in_RCX;
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX & 1) == 0) {
    std::sys::random::hashmap_random_keys();
    extraout_RAX[0] = 1;
    extraout_RAX[1] = 0;
    extraout_RAX[2] = 0;
    extraout_RAX[3] = 0;
    extraout_RAX[4] = 0;
    extraout_RAX[5] = 0;
    extraout_RAX[6] = 0;
    extraout_RAX[7] = 0;
    *(longlong *)(extraout_RAX + 8) = extraout_RAX_00;
    *(longlong *)(extraout_RAX + 0x10) = lVar3;
    local_58 = extraout_RAX_00;
  }
  else {
    local_58 = *(longlong *)(extraout_RAX + 8);
    lVar3 = *(longlong *)(extraout_RAX + 0x10);
  }
  *(longlong *)(extraout_RAX + 8) = local_58 + 1;
  local_68 = _DAT_142f33320;
  uStack_60 = _UNK_142f33328;
  local_78 = anon_2fc37c2e165e4335d861dbcc334b2b56_1_llvm_2532611877472227773;
  uStack_70 = _UNK_142f33318;
  local_178 = 1;
  local_170 = local_e0;
  local_168 = local_d8;
  local_160 = local_c8;
  local_158 = local_c0;
  local_150 = local_d0;
  local_148 = local_b8;
  local_140 = local_b0;
  local_138 = local_a8;
  local_128 = local_a0;
  local_118 = local_98;
  local_110 = local_90;
  local_e8 = local_88;
  local_130 = lVar5;
  local_120 = lVar8;
  local_108 = lVar9;
  local_100 = uVar1 * 0x20 + lVar9;
  local_f8 = lVar4;
  local_f0 = uVar7 * 0x20 + lVar4;
  lStack_50 = lVar3;
  core::iter::adapters::map::impl$2::
  size_hint<tuple_<country_core::resources::walls::WallId,tuple_<>_>,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<bevy_ecs::event::iterators::EventIterator<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,system_clutter::inter_shape_stitches::detect_intra_shape_corners::impl_1::dirty_walls::closure_env_0>,core::iter::adapters::map::Map<bevy_ecs::event::iterators::EventIterator<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,system_clutter::inter_shape_stitches::detect_intra_shape_corners::impl_1::dirty_walls::closure_env_1>_>,core::iter::adapters::map::Map<bevy_ecs::event::iterators::EventIterator<country_core::resources::roofs::commands::RecalculateRoofCmd>,system_clutter::inter_shape_stitches::detect_intra_shape_corners::impl_1::dirty_walls::closure_env_2>_>,hashbrown::set::impl_10::extend::closure_env_0<country_core::resources::walls::WallId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<bevy_ecs::event::iterators::EventIterator<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,system_clutter::inter_shape_stitches::detect_intra_shape_corners::impl_1::dirty_walls::closure_env_0>,core::iter::adapters::map::Map<bevy_ecs::event::iterators::EventIterator<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,system_clutter::inter_shape_stitches::detect_intra_shape_corners::impl_1::dirty_walls::closure_env_1>_>,core::iter::adapters::map::Map<bevy_ecs::event::iterators::EventIterator<country_core::resources::roofs::commands::RecalculateRoofCmd>,system_clutter::inter_shape_stitches::detect_intra_shape_corners::impl_1::dirty_walls::closure_env_2>_>_>_>
            ();
  if (local_68 < local_210[0]) {
    hashbrown::raw::RawTable::
    reserve_rehash<tuple_<country_core::resources::history::HistorySnapshotId,tuple_<>_>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::history::HistorySnapshotId,tuple_<>,std::hash::random::RandomState>_>
              ();
  }
  memcpy(local_210,&local_178,0x98);
  _ZN106__LT_core__iter__adapters__chain__Chain_LT_A_C_B_GT__u20_as_u20_core__iter__traits__iterator__Iterator_GT_4fold17h27c3af5177ae99e1E
            ();
  local_80[4] = local_58;
  local_80[5] = lStack_50;
  local_80[2] = local_68;
  local_80[3] = uStack_60;
  *local_80 = local_78;
  local_80[1] = uStack_70;
  return;
}

