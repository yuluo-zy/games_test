// 0x141a31280 run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void_(*)(bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::roof_highlight::RoofHighlight>),void_(*)(bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::roof_highlight::RoofHighlight>)>_> :: void __cdecl run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void_(*)(bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::roof_highlight::RoofHighlight>),void_(*)(bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::roof_highlight::RoofHighlight>)>_>(void)

void __cdecl
bevy_ecs::system::schedule_system::impl$1::
run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void_(*)(bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::roof_highlight::RoofHighlight>),void_(*)(bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::HighlightRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::roof_highlight::RoofHighlight>)>_>
          (void)

{
  int *piVar1;
  int iVar2;
  undefined1 uVar3;
  ulonglong uVar4;
  undefined8 *puVar5;
  undefined8 *puVar6;
  undefined8 uVar7;
  longlong lVar8;
  longlong lVar9;
  byte *in_RCX;
  longlong in_RDX;
  longlong lVar10;
  ulonglong uVar11;
  ulonglong uVar12;
  ulonglong uVar13;
  longlong lVar14;
  longlong lVar15;
  char *local_b0;
  undefined8 local_a8;
  byte *local_a0;
  code *local_98;
  char **local_90;
  code *local_88;
  undefined *local_80;
  undefined8 local_78;
  byte **local_70;
  undefined8 local_68;
  undefined8 local_60;
  longlong local_50;
  longlong local_48;
  
  LOCK();
  piVar1 = (int *)(in_RDX + 0x5b0);
  iVar2 = *piVar1;
  *piVar1 = *piVar1 + 1;
  UNLOCK();
  if ((*in_RCX & 1) == 0) {
                    // WARNING: Subroutine does not return
    core::option::expect_failed();
  }
  if ((*(ulonglong *)(in_RCX + 0x18) < *(ulonglong *)(in_RDX + 0x218)) &&
     (uVar4 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + *(ulonglong *)(in_RCX + 0x18) * 8),
     uVar4 != 0)) {
    lVar8 = ~uVar4 * 0x60;
    if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar8) != 0) {
      local_48 = *(longlong *)(in_RDX + 0x1e0) + lVar8;
      puVar5 = *(undefined8 **)(local_48 + 0x10);
      if (*(ulonglong *)(in_RDX + 0x218) <= *(ulonglong *)(in_RCX + 0x10)) {
LAB_141a314cb:
        local_a0 = in_RCX + 200;
        local_b0 = 
        "bevy_ecs::event::collections::Events<country_core::resources::roofs::commands::HighlightRoofCmd>Resource requested by  does not exist: "
        ;
        local_a8 = 0x60;
        local_98 = alloc::string::impl$23::fmt;
        local_90 = &local_b0;
        local_88 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
        local_80 = &DAT_142eb8188;
        local_78 = 2;
        local_60 = 0;
        local_70 = &local_a0;
        local_68 = 2;
                    // WARNING: Subroutine does not return
        core::panicking::panic_fmt();
      }
      uVar4 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + *(ulonglong *)(in_RCX + 0x10) * 8);
      if (uVar4 == 0) goto LAB_141a314cb;
      lVar8 = *(longlong *)(in_RDX + 0x1e0);
      lVar9 = ~uVar4 * 0x60;
      if (*(longlong *)(lVar8 + 0x20 + lVar9) == 0) goto LAB_141a314cb;
      if (((*(ulonglong *)(in_RCX + 0x20) < *(ulonglong *)(in_RDX + 0x218)) &&
          (uVar4 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + *(ulonglong *)(in_RCX + 0x20) * 8)
          , uVar4 != 0)) && (lVar14 = ~uVar4 * 0x60, *(longlong *)(lVar8 + 0x20 + lVar14) != 0)) {
        lVar9 = *(longlong *)(lVar9 + lVar8 + 0x10);
        lVar14 = lVar14 + lVar8;
        puVar6 = *(undefined8 **)(lVar14 + 0x10);
        *(int *)(lVar14 + 0x5c) = iVar2;
        *(undefined1 *)(puVar6 + 1) = 0;
        uVar4 = *(ulonglong *)(in_RCX + 8);
        uVar11 = 0;
        uVar13 = uVar4 - *(ulonglong *)(lVar9 + 0x18);
        if (uVar4 < *(ulonglong *)(lVar9 + 0x18)) {
          uVar13 = uVar11;
        }
        uVar12 = uVar4 - *(ulonglong *)(lVar9 + 0x38);
        if (uVar4 < *(ulonglong *)(lVar9 + 0x38)) {
          uVar12 = uVar11;
        }
        lVar8 = uVar13 * 0x18 + *(longlong *)(lVar9 + 8);
        uVar4 = *(ulonglong *)(lVar9 + 0x10) - uVar13;
        if (*(ulonglong *)(lVar9 + 0x10) < uVar13) {
          lVar8 = 8;
          uVar4 = uVar11;
        }
        lVar10 = uVar12 * 0x18 + *(longlong *)(lVar9 + 0x28);
        uVar13 = *(ulonglong *)(lVar9 + 0x30) - uVar12;
        if (*(ulonglong *)(lVar9 + 0x30) < uVar12) {
          lVar10 = 8;
          uVar13 = uVar11;
        }
        *(ulonglong *)(in_RCX + 8) = (*(longlong *)(lVar9 + 0x40) - uVar13) - uVar4;
        local_50 = uVar13 * 0x18 + lVar10;
        lVar9 = lVar8;
        do {
          if ((lVar9 == 0) || (lVar9 == lVar8 + uVar4 * 0x18)) {
            if ((lVar10 == 0) || (lVar10 == local_50)) {
              *(int *)(in_RCX + 0x160) = iVar2;
              return;
            }
            lVar15 = 0;
            lVar9 = lVar10;
            lVar10 = lVar10 + 0x18;
          }
          else {
            lVar15 = lVar9 + 0x18;
          }
          *(longlong *)(in_RCX + 8) = *(longlong *)(in_RCX + 8) + 1;
          uVar7 = *(undefined8 *)(lVar9 + 8);
          uVar3 = *(undefined1 *)(lVar9 + 0x10);
          *(int *)(lVar14 + 0x5c) = iVar2;
          *puVar6 = uVar7;
          *(undefined1 *)(puVar6 + 1) = uVar3;
          country_core::resources::walls::HighlightColor::to_rgb();
          *(int *)(local_48 + 0x5c) = iVar2;
          *(undefined4 *)(puVar5 + 1) = (undefined4)local_78;
          *puVar5 = local_80;
          lVar9 = lVar15;
        } while( true );
      }
      local_b0 = "country_core::resources::roofs::roof_highlight::RoofHighlight";
      local_a8 = 0x3d;
      goto LAB_141a3156c;
    }
  }
  local_b0 = 
  "country_core::resources::editing_highlight::EditingHighlightSystem built with a different world than the one it was added to."
  ;
  local_a8 = 0x3c;
LAB_141a3156c:
  local_a0 = in_RCX + 200;
  local_98 = alloc::string::impl$23::fmt;
  local_90 = &local_b0;
  local_88 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
  local_80 = &DAT_142eb8188;
  local_78 = 2;
  local_60 = 0;
  local_70 = &local_a0;
  local_68 = 2;
                    // WARNING: Subroutine does not return
  core::panicking::panic_fmt();
}


