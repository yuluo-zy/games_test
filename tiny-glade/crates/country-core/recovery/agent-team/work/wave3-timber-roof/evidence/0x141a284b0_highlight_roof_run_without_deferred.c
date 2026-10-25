// 0x141a284b0 highlight_roof_run_without_deferred
// sig: void __cdecl run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void_(*)(bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::roof_highlight::RoofHighlight>),void_(*)(bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::roof_highlight::RoofHighlight>)>_>_>(void)
// size: 876


void __cdecl
bevy_ecs::system::system::System::
run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void_(*)(bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::roof_highlight::RoofHighlight>),void_(*)(bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::roof_highlight::RoofHighlight>)>_>_>
          (void)

{
  int *piVar1;
  int iVar2;
  undefined1 uVar3;
  uint uVar4;
  ulonglong uVar5;
  longlong *plVar6;
  longlong lVar7;
  undefined8 *puVar8;
  undefined8 uVar9;
  undefined8 *puVar10;
  longlong lVar11;
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  byte *in_RCX;
  longlong in_RDX;
  longlong lVar12;
  byte *local_a0;
  code *local_98;
  undefined **local_90;
  code *local_88;
  undefined *local_80;
  undefined8 local_78;
  undefined **local_70;
  undefined8 local_68;
  byte **local_60;
  undefined8 local_58;
  undefined8 uStack_50;
  
  if ((*in_RCX & 1) == 0) {
                    // WARNING: Subroutine does not return
    core::option::expect_failed();
  }
  local_a0 = *(byte **)(in_RDX + 0x5a0);
  if (*(byte **)(in_RCX + 0x28) != local_a0) {
    local_70 = &anon_d7edb72707ef88fc7604984178925be0_56_llvm_9683733247967176420;
    local_68 = 1;
    local_60 = (byte **)0x8;
    local_58 = 0;
    uStack_50 = 0;
                    // WARNING: Subroutine does not return
    core::panicking::
    assert_failed<bevy_ecs::world::identifier::WorldId,bevy_ecs::world::identifier::WorldId>();
  }
  uVar5 = *(ulonglong *)(in_RDX + 0x108);
  uVar4 = *(uint *)(in_RCX + 0x168);
  *(int *)(in_RCX + 0x168) = (int)uVar5;
  if (uVar5 < uVar4) {
                    // WARNING: Subroutine does not return
    core::slice::index::slice_start_index_len_fail();
  }
  LOCK();
  piVar1 = (int *)(in_RDX + 0x5b0);
  iVar2 = *piVar1;
  *piVar1 = *piVar1 + 1;
  UNLOCK();
  if ((*(ulonglong *)(in_RCX + 0x18) < *(ulonglong *)(in_RDX + 0x218)) &&
     (uVar5 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + *(ulonglong *)(in_RCX + 0x18) * 8),
     uVar5 != 0)) {
    lVar11 = ~uVar5 * 0x60;
    if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar11) != 0) {
      lVar11 = *(longlong *)(in_RDX + 0x1e0) + lVar11;
      plVar6 = *(longlong **)(lVar11 + 0x10);
      if (*(ulonglong *)(in_RDX + 0x218) <= *(ulonglong *)(in_RCX + 0x10)) {
LAB_141a286fa:
        local_a0 = in_RCX + 200;
        local_80 = &DAT_142eb1cd1;
        local_78 = 0x60;
        local_98 = alloc::string::impl$23::fmt;
        local_90 = &local_80;
        local_88 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
        local_70 = &PTR_s_Resource_requested_by_does_not_e_142eb1798;
        local_68 = 2;
        uStack_50 = 0;
        local_60 = &local_a0;
        local_58 = 2;
                    // WARNING: Subroutine does not return
        core::panicking::panic_fmt();
      }
      uVar5 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + *(ulonglong *)(in_RCX + 0x10) * 8);
      if ((uVar5 == 0) ||
         (lVar7 = *(longlong *)(in_RDX + 0x1e0), *(longlong *)(lVar7 + 0x20 + ~uVar5 * 0x60) == 0))
      goto LAB_141a286fa;
      if (((*(ulonglong *)(in_RCX + 0x20) < *(ulonglong *)(in_RDX + 0x218)) &&
          (uVar5 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + *(ulonglong *)(in_RCX + 0x20) * 8)
          , uVar5 != 0)) && (lVar12 = ~uVar5 * 0x60, *(longlong *)(lVar7 + 0x20 + lVar12) != 0)) {
        lVar12 = lVar12 + lVar7;
        puVar8 = *(undefined8 **)(lVar12 + 0x10);
        *(int *)(lVar12 + 0x5c) = iVar2;
        *(undefined1 *)(puVar8 + 1) = 0;
        event::iterators::EventIteratorWithId::
        new<country_core::resources::roofs::commands::HighlightRoofCmd>();
        event::iterators::impl$0::
        next<system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>();
        puVar10 = extraout_RAX;
        while (puVar10 != (undefined8 *)0x0) {
          uVar9 = *puVar10;
          uVar3 = *(undefined1 *)(puVar10 + 1);
          *(int *)(lVar12 + 0x5c) = iVar2;
          *puVar8 = uVar9;
          *(undefined1 *)(puVar8 + 1) = uVar3;
          country_core::resources::walls::HighlightColor::to_rgb();
          *(int *)(lVar11 + 0x5c) = iVar2;
          *(undefined4 *)(plVar6 + 1) = local_98._0_4_;
          *plVar6 = (longlong)local_a0;
          event::iterators::impl$0::
          next<system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>();
          puVar10 = extraout_RAX_00;
        }
        *(int *)(in_RCX + 0x160) = iVar2;
        return;
      }
      local_80 = &DAT_142eb2414;
      local_78 = 0x3d;
      goto LAB_141a2879b;
    }
  }
  local_80 = &DAT_142eb2591;
  local_78 = 0x3c;
LAB_141a2879b:
  local_a0 = in_RCX + 200;
  local_98 = alloc::string::impl$23::fmt;
  local_90 = &local_80;
  local_88 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
  local_70 = &PTR_s_Resource_requested_by_does_not_e_142eb1798;
  local_68 = 2;
  uStack_50 = 0;
  local_60 = &local_a0;
  local_58 = 2;
                    // WARNING: Subroutine does not return
  core::panicking::panic_fmt();
}

