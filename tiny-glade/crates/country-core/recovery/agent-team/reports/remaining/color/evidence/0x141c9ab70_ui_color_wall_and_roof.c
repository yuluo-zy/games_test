
void __cdecl system_color::ui_color_wall_and_roof::ui_color_wall_and_roof(void)

{
  uint *puVar1;
  undefined8 *puVar2;
  longlong lVar3;
  longlong lVar4;
  undefined8 uVar5;
  float *pfVar6;
  longlong lVar7;
  undefined8 uVar8;
  ulonglong uVar9;
  undefined8 uVar10;
  char extraout_AL;
  byte extraout_AL_00;
  char extraout_AL_01;
  byte bVar11;
  undefined8 *in_RCX;
  uint *puVar12;
  longlong lVar13;
  undefined8 *in_RDX;
  char *pcVar14;
  undefined1 *puVar15;
  undefined8 *in_R8;
  longlong *in_R9;
  longlong lVar16;
  ulonglong uVar17;
  ulonglong uVar19;
  byte bVar20;
  char cVar21;
  longlong lVar22;
  longlong *in_stack_00000028;
  longlong *in_stack_00000030;
  longlong *in_stack_00000038;
  undefined8 *in_stack_00000048;
  undefined8 local_2e8;
  code *pcStack_2e0;
  ushort local_238;
  undefined1 local_1d8;
  byte bStack_1d7;
  undefined8 local_1d0;
  float local_1c0;
  float local_1bc;
  float local_1b8;
  int local_1a4;
  undefined *local_198;
  undefined1 *puStack_190;
  undefined8 *puStack_188;
  undefined8 uStack_180;
  undefined8 uStack_178;
  byte bStack_ef;
  undefined1 auStack_e8 [32];
  char *local_c8;
  undefined8 local_c0;
  longlong *local_b8;
  undefined8 uStack_b0;
  undefined1 local_a3;
  undefined1 uStack_a2;
  undefined1 uStack_a1;
  undefined8 local_a0;
  ulonglong uVar18;
  
  local_a0 = 0xfffffffffffffffe;
  if (*(int *)*in_RCX == 0xc) {
    puVar2 = (undefined8 *)*in_R8;
    local_c0 = *in_RDX;
    local_b8 = in_R9;
    country_core::systems::ui::resources::widget_params::WidgetVolumeShape::clearing_canvas();
    local_238 = 1;
    country_core::systems::ui::resources::widget_params::WidgetParams::new();
    local_a3 = 1;
    country_core::systems::ui::resources::widget_params::UiSounds::empty();
    local_a3 = 0;
    country_core::systems::ui::resources::widget_params::WidgetParams::sounds();
    country_core::systems::ui::resources::widget_params::WidgetParams::occlude_by_world();
    country_core::systems::ui::resources::widget_params::WidgetParams::
    signifier<system_color::ui_color_wall_and_roof::ui_color_wall_and_roof::closure_env_0>();
    pcVar14 = "color_canvascrates/systems/color/src/ui_color_wall_and_roof.rscolor_wall_";
    country_core::systems::ui::resources::ui_state::UiState::add<ref_<str_>_>();
    local_c8 = pcVar14;
    country_core::systems::ui::resources::ui_state::UiWidgetContext::clicked();
    if (extraout_AL != '\0') {
      *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
      lVar3 = *in_stack_00000028;
      lVar4 = *(longlong *)(lVar3 + 0x30);
      uVar5 = *(undefined8 *)(lVar3 + 0x40);
      if (lVar4 == *(longlong *)(lVar3 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      lVar22 = *(longlong *)(lVar3 + 0x28);
      *(undefined8 *)(lVar22 + lVar4 * 0x10) = uVar5;
      *(undefined1 *)(lVar22 + 8 + lVar4 * 0x10) = 1;
      *(longlong *)(lVar3 + 0x30) = lVar4 + 1;
      *(longlong *)(lVar3 + 0x40) = *(longlong *)(lVar3 + 0x40) + 1;
    }
    pfVar6 = (float *)*in_stack_00000048;
    country_core::systems::collision::world::RaycastWorld::raycast();
    if (local_1a4 != 2) {
      switch(local_1d8) {
      case 0:
      case 6:
      case 7:
        bVar20 = 3;
        break;
      default:
        goto LAB_141c9b305;
      case 2:
        bVar20 = bStack_1d7 & 1;
        break;
      case 8:
        bVar20 = 2;
      }
      uStack_b0 = local_1d0;
      lVar3 = *in_stack_00000030;
      lVar4 = *(longlong *)(lVar3 + 0x100);
      if (lVar4 == 0) {
        lVar22 = 0;
      }
      else {
        puVar12 = *(uint **)(lVar3 + 0xf8);
        lVar13 = in_stack_00000030[1];
        if (*(char *)(lVar3 + 0x108) == '\x01') {
          lVar16 = 0;
          lVar22 = 0;
          do {
            lVar7 = *(longlong *)
                     (*(longlong *)(lVar13 + 0x1a8) + 0x10 +
                     (ulonglong)*(uint *)((longlong)puVar12 + lVar16) * 0x48);
            if (lVar7 != 0) {
              lVar22 = *(longlong *)(lVar13 + 0x1a8) +
                       (ulonglong)*(uint *)((longlong)puVar12 + lVar16) * 0x48;
              if ((*(ulonglong *)(lVar3 + 0x110) < *(ulonglong *)(lVar22 + 0x40)) &&
                 (uVar18 = *(ulonglong *)
                            (*(longlong *)(lVar22 + 0x38) + *(ulonglong *)(lVar3 + 0x110) * 8),
                 uVar18 != 0)) {
                lVar22 = *(longlong *)(*(longlong *)(lVar22 + 0x18) + 0x10 + ~uVar18 * 0x30);
              }
              else {
                lVar22 = 0;
              }
              lVar22 = (ulonglong)((int)lVar7 - 1) * 0xa0 + lVar22;
            }
            lVar16 = lVar16 + 4;
          } while (lVar4 << 2 != lVar16);
        }
        else {
          puVar1 = puVar12 + lVar4;
          lVar4 = *(longlong *)(lVar13 + 0x100);
          lVar22 = 0;
          do {
            lVar16 = (ulonglong)*puVar12 * 0xa0;
            uVar17 = (ulonglong)*(uint *)(lVar4 + 0x94 + lVar16);
            uVar18 = *(longlong *)(lVar13 + 0x1a8) + uVar17 * 0x48;
            uVar9 = *(ulonglong *)(lVar4 + 0x58 + lVar16);
            if (*(ulonglong *)(*(longlong *)(lVar13 + 0x1a8) + 0x10 + uVar17 * 0x48) == uVar9) {
              if (uVar9 != 0) {
                if ((*(ulonglong *)(lVar3 + 0x110) < *(ulonglong *)(uVar18 + 0x40)) &&
                   (uVar17 = *(ulonglong *)
                              (*(longlong *)(uVar18 + 0x38) + *(ulonglong *)(lVar3 + 0x110) * 8),
                   uVar17 != 0)) {
                  lVar22 = *(longlong *)(*(longlong *)(uVar18 + 0x18) + 0x10 + ~uVar17 * 0x30);
                  if (uVar9 < 8) goto code_r0x000141c9b24d;
code_r0x000141c9b1ca:
                  uVar17 = 0;
                  do {
                    uVar18 = uVar17;
                    uVar17 = uVar18 + 8;
                  } while ((uVar9 & 0x7fffffffffffff8) != uVar17);
                  uVar18 = uVar18 + 7;
                }
                else {
                  lVar22 = 0;
                  if (7 < uVar9) goto code_r0x000141c9b1ca;
code_r0x000141c9b24d:
                  uVar17 = 0;
                }
                uVar19 = (ulonglong)((uint)uVar9 & 7);
                uVar9 = uVar9 & 7;
                while (uVar9 != 0) {
                  uVar19 = uVar19 - 1;
                  uVar18 = uVar17;
                  uVar17 = uVar17 + 1;
                  uVar9 = uVar19;
                }
                lVar22 = lVar22 + (uVar18 & 0xffffffff) * 0xa0;
              }
            }
            else if (uVar9 != 0) {
              if ((*(ulonglong *)(lVar3 + 0x110) < *(ulonglong *)(uVar18 + 0x40)) &&
                 (uVar17 = *(ulonglong *)
                            (*(longlong *)(uVar18 + 0x38) + *(ulonglong *)(lVar3 + 0x110) * 8),
                 uVar17 != 0)) {
                lVar22 = *(longlong *)(*(longlong *)(uVar18 + 0x18) + 0x10 + ~uVar17 * 0x30);
              }
              else {
                lVar22 = 0;
              }
              lVar22 = (ulonglong)
                       *(uint *)(uVar9 * 0x10 + -8 + *(longlong *)(lVar16 + lVar4 + 0x50)) * 0xa0 +
                       lVar22;
            }
            puVar12 = puVar12 + 1;
          } while (puVar12 != puVar1);
        }
      }
      local_2e8 = &uStack_b0;
      pcStack_2e0 = country_core::resources::walls::impl$7::fmt;
      local_198 = &UNK_142f25070;
      puStack_190 = (undefined1 *)0x1;
      uStack_180 = 1;
      uStack_178 = 0;
      puStack_188 = &local_2e8;
      alloc::fmt::format::format_inner();
      local_2e8 = (undefined8 *)
                  CONCAT44(SQRT((pfVar6[2] - local_1b8) * (pfVar6[2] - local_1b8) +
                                (pfVar6[1] - local_1bc) * (pfVar6[1] - local_1bc) +
                                (*pfVar6 - local_1c0) * (*pfVar6 - local_1c0)),(undefined4)local_2e8
                          );
      local_2e8 = (undefined8 *)CONCAT71(local_2e8._1_7_,5);
      local_238 = local_238 & 0xff00;
      uStack_a1 = 1;
      country_core::systems::ui::resources::widget_params::WidgetParams::new();
      bStack_ef = bStack_ef | 2;
      memcpy(&local_2e8,&local_198,0xb0);
      uStack_a2 = 1;
      country_core::systems::ui::resources::widget_params::UiSounds::empty();
      uStack_a2 = 1;
      country_core::systems::ui::resources::widget_params::UiSounds::on_hover_start();
      local_198 = (undefined *)CONCAT44(local_198._4_4_,8);
      uStack_a2 = 1;
      country_core::systems::ui::resources::widget_params::UiSounds::on_click();
      uStack_a2 = 0;
      country_core::systems::ui::resources::widget_params::WidgetParams::sounds();
      uStack_a1 = 1;
      country_core::systems::ui::resources::widget_params::WidgetParams::
      signifier<system_color::ui_color_wall_and_roof::ui_color_wall_and_roof::closure_env_2>();
      uStack_a1 = 0;
      puVar15 = auStack_e8;
      country_core::systems::ui::resources::ui_state::UiState::add<alloc::string::String>();
      puStack_190 = puVar15;
      country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active();
      uVar5 = uStack_b0;
      if ((lVar22 == 0 & extraout_AL_00) != 0) {
        *(undefined4 *)in_stack_00000038[2] = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
        lVar3 = *in_stack_00000038;
        lVar4 = *(longlong *)(lVar3 + 0x30);
        uVar8 = *(undefined8 *)(lVar3 + 0x40);
        if (lVar4 == *(longlong *)(lVar3 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                    ();
        }
        lVar22 = *(longlong *)(lVar3 + 0x28);
        *(undefined8 *)(lVar22 + lVar4 * 0x18) = uVar8;
        *(undefined8 *)(lVar22 + 8 + lVar4 * 0x18) = uVar5;
        *(undefined4 *)(lVar22 + 0x10 + lVar4 * 0x18) = 0x3f800000;
        *(bool *)(lVar22 + 0x14 + lVar4 * 0x18) = 1 < bVar20;
        *(longlong *)(lVar3 + 0x30) = lVar4 + 1;
        *(longlong *)(lVar3 + 0x40) = *(longlong *)(lVar3 + 0x40) + 1;
      }
      country_core::systems::ui::resources::ui_state::UiWidgetContext::clicked();
      if (extraout_AL_01 != '\0') {
        *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
        lVar3 = *in_stack_00000028;
        lVar4 = *(longlong *)(lVar3 + 0x30);
        uVar5 = *(undefined8 *)(lVar3 + 0x40);
        if (lVar4 == *(longlong *)(lVar3 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                    ();
        }
        uVar8 = uStack_b0;
        lVar22 = *(longlong *)(lVar3 + 0x28);
        *(undefined8 *)(lVar22 + lVar4 * 0x10) = uVar5;
        *(undefined1 *)(lVar22 + 8 + lVar4 * 0x10) = 0;
        *(longlong *)(lVar3 + 0x30) = lVar4 + 1;
        *(longlong *)(lVar3 + 0x40) = *(longlong *)(lVar3 + 0x40) + 1;
        bVar11 = 2;
        if ((byte)(bVar20 - 2) < 2) {
          bVar11 = bVar20 - 2;
        }
        if (bVar11 == 0) {
          cVar21 = '\b';
        }
        else if (bVar11 == 1) {
          cVar21 = '\0';
        }
        else {
          cVar21 = (bVar20 & 1) + 1;
        }
        uVar5 = *puVar2;
        *(undefined4 *)local_b8[2] = *(undefined4 *)((longlong)local_b8 + 0x1c);
        lVar3 = *local_b8;
        lVar4 = *(longlong *)(lVar3 + 0x30);
        uVar10 = *(undefined8 *)(lVar3 + 0x40);
        if (lVar4 == *(longlong *)(lVar3 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                    ();
        }
        lVar22 = *(longlong *)(lVar3 + 0x28);
        lVar13 = lVar4 * 0x20;
        *(char *)(lVar22 + lVar13) = cVar21;
        *(undefined8 *)(lVar22 + 8 + lVar13) = uVar8;
        *(undefined8 *)(lVar22 + 0x10 + lVar13) = uVar5;
        *(undefined8 *)(lVar22 + 0x18 + lVar13) = uVar10;
        *(longlong *)(lVar3 + 0x30) = lVar4 + 1;
        *(longlong *)(lVar3 + 0x40) = *(longlong *)(lVar3 + 0x40) + 1;
      }
    }
  }
LAB_141c9b305:
  return;
}

