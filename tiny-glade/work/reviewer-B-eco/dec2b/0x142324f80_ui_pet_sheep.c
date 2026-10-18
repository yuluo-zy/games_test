
void __cdecl system_critters::sheep::ui_pet_sheep::ui_pet_sheep(void)

{
  float fVar1;
  float fVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  int iVar5;
  uint uVar6;
  ulonglong uVar7;
  ulonglong uVar8;
  float fVar9;
  undefined8 uVar10;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  undefined1 extraout_AL_02;
  char extraout_AL_03;
  char extraout_AL_04;
  byte bVar11;
  char extraout_AL_05;
  longlong lVar12;
  ulonglong uVar13;
  undefined8 *extraout_RAX;
  ulonglong uVar14;
  undefined8 *extraout_RAX_00;
  undefined8 *in_RCX;
  ulonglong uVar15;
  longlong lVar16;
  undefined8 *in_RDX;
  longlong lVar17;
  ulonglong uVar18;
  float *pfVar19;
  longlong *in_R8;
  longlong *in_R9;
  longlong lVar20;
  ulonglong uVar21;
  uint *puVar22;
  uint *puVar23;
  longlong lVar24;
  bool bVar25;
  bool bVar26;
  float extraout_XMM0_Da;
  float fVar27;
  float fVar28;
  float fVar29;
  float fVar30;
  float fVar31;
  float fVar32;
  float fVar33;
  float fVar34;
  float fVar35;
  undefined8 *in_stack_00000028;
  longlong *in_stack_00000030;
  longlong *in_stack_00000038;
  float *in_stack_00000040;
  undefined8 *in_stack_00000048;
  undefined8 *in_stack_00000050;
  longlong *in_stack_00000058;
  undefined8 local_388;
  code *pcStack_380;
  undefined8 *local_378;
  undefined8 uStack_370;
  undefined8 local_368;
  longlong local_2d8;
  undefined8 local_2d0;
  undefined8 local_2a8;
  undefined8 uStack_2a0;
  undefined8 *local_298;
  undefined8 uStack_290;
  undefined8 local_288;
  float fStack_280;
  float fStack_27c;
  undefined8 *local_1f8;
  undefined8 local_1f0;
  undefined8 *local_1e8;
  longlong local_1e0;
  undefined4 *local_1d8;
  longlong local_1d0;
  longlong local_1c8;
  undefined4 *local_1c0;
  longlong local_1b8;
  longlong local_1b0;
  undefined4 *local_1a8;
  undefined4 *local_1a0;
  uint *local_198;
  longlong local_190;
  longlong local_188;
  longlong local_180;
  undefined8 local_178;
  ulonglong *local_170;
  undefined8 local_168;
  longlong local_160;
  undefined4 local_158;
  undefined4 local_154;
  longlong local_150;
  longlong local_148;
  longlong local_140;
  longlong local_138;
  longlong local_130;
  longlong local_128;
  longlong local_120;
  undefined4 local_114;
  longlong local_110;
  ulonglong local_108;
  longlong local_100;
  char local_f3;
  undefined1 local_f2;
  undefined1 local_f1;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  iVar5 = *(int *)*in_RCX;
  if (iVar5 == 0x13) {
    country_core::systems::camera_rig::MainCamera::mode();
    if (extraout_AL == '\x03') goto LAB_142325044;
  }
  else if ((iVar5 != 0xd) && ((iVar5 != 6 || (1 < (byte)((char)((int *)*in_RCX)[1] - 1U))))) {
LAB_142325044:
    local_120 = *in_R8;
    lVar24 = in_R8[1];
    local_114 = *(undefined4 *)((longlong)in_R8 + 0x14);
    puVar22 = *(uint **)(local_120 + 0xf8);
    puVar23 = puVar22 + *(longlong *)(local_120 + 0x100);
    local_110 = *in_stack_00000030;
    local_1f0 = *in_RDX;
    local_1e8 = (undefined8 *)*in_stack_00000048;
    local_178 = *in_stack_00000028;
    local_1e0 = *in_stack_00000038 + 0x2c;
    local_158 = *(undefined4 *)((longlong)in_R9 + 0x1c);
    local_190 = *in_R9;
    local_1d8 = (undefined4 *)in_R9[2];
    local_170 = (ulonglong *)*in_stack_00000050;
    local_1d0 = in_stack_00000050[1];
    local_154 = *(undefined4 *)((longlong)in_stack_00000058 + 0x1c);
    local_160 = *in_stack_00000058;
    local_1c0 = (undefined4 *)in_stack_00000058[2];
    local_f3 = *(char *)(local_120 + 0x108);
    local_1c8 = local_190 + 0x20;
    local_1b8 = local_160 + 0x20;
    local_180 = 8;
    local_100 = 0;
    local_130 = 0;
    local_128 = 0;
    uVar21 = 0;
    uVar18 = 0;
    local_198 = puVar23;
    local_188 = lVar24;
LAB_1423251a2:
    if (local_f3 == '\0') {
      if (uVar18 == uVar21) {
        do {
          if (puVar22 == puVar23) {
            return;
          }
          uVar6 = *puVar22;
          puVar22 = puVar22 + 1;
          uVar21 = *(ulonglong *)(*(longlong *)(lVar24 + 0x100) + 0x58 + (ulonglong)uVar6 * 0xa0);
        } while (uVar21 == 0);
        lVar12 = *(longlong *)(lVar24 + 0x100) + (ulonglong)uVar6 * 0xa0;
        uVar18 = (ulonglong)*(uint *)(lVar12 + 0x94);
        lVar17 = *(longlong *)(lVar24 + 0x1a8);
        lVar20 = *(longlong *)(lVar17 + 0x18 + uVar18 * 0x48);
        lVar16 = *(longlong *)(lVar17 + 0x38 + uVar18 * 0x48);
        if ((*(ulonglong *)(local_120 + 0x118) < *(ulonglong *)(lVar17 + 0x40 + uVar18 * 0x48)) &&
           (uVar18 = *(ulonglong *)(lVar16 + *(ulonglong *)(local_120 + 0x118) * 8), uVar18 != 0)) {
          local_100 = *(longlong *)(lVar20 + 0x10 + ~uVar18 * 0x30);
        }
        else {
          local_100 = 0;
        }
        lVar17 = ~*(ulonglong *)(lVar16 + *(longlong *)(local_120 + 0x110) * 8) * 0x30;
        local_130 = *(longlong *)(lVar20 + 0x10 + lVar17);
        local_148 = *(longlong *)(lVar20 + 0x28 + lVar17);
        lVar17 = ~*(ulonglong *)(lVar16 + *(longlong *)(local_120 + 0x120) * 8) * 0x30;
        local_128 = *(longlong *)(lVar20 + 0x10 + lVar17);
        local_140 = *(longlong *)(lVar20 + 0x28 + lVar17);
        local_180 = *(longlong *)(lVar12 + 0x50);
        uVar18 = 0;
      }
      uVar13 = (ulonglong)*(uint *)(local_180 + 8 + uVar18 * 0x10);
    }
    else {
      if (uVar18 == uVar21) {
        do {
          if (puVar22 == puVar23) {
            return;
          }
          uVar6 = *puVar22;
          puVar22 = puVar22 + 1;
          uVar21 = *(ulonglong *)(*(longlong *)(lVar24 + 0x1a8) + 0x10 + (ulonglong)uVar6 * 0x48);
        } while (uVar21 == 0);
        lVar17 = *(longlong *)(lVar24 + 0x1a8) + (ulonglong)uVar6 * 0x48;
        lVar20 = *(longlong *)(lVar17 + 0x18);
        lVar16 = *(longlong *)(lVar17 + 0x38);
        if ((*(ulonglong *)(local_120 + 0x118) < *(ulonglong *)(lVar17 + 0x40)) &&
           (uVar18 = *(ulonglong *)(lVar16 + *(ulonglong *)(local_120 + 0x118) * 8), uVar18 != 0)) {
          local_100 = *(longlong *)(lVar20 + 0x10 + ~uVar18 * 0x30);
        }
        else {
          local_100 = 0;
        }
        lVar17 = ~*(ulonglong *)(lVar16 + *(longlong *)(local_120 + 0x110) * 8) * 0x30;
        local_130 = *(longlong *)(lVar20 + 0x10 + lVar17);
        local_148 = *(longlong *)(lVar20 + 0x28 + lVar17);
        lVar17 = ~*(ulonglong *)(lVar16 + *(longlong *)(local_120 + 0x120) * 8) * 0x30;
        local_128 = *(longlong *)(lVar20 + 0x10 + lVar17);
        local_140 = *(longlong *)(lVar20 + 0x28 + lVar17);
        uVar18 = 0;
      }
      uVar13 = uVar18 & 0xffffffff;
    }
    local_150 = uVar13 * 0x68 + local_130;
    local_1a8 = (undefined4 *)(local_148 + uVar13 * 4);
    pfVar19 = (float *)(uVar13 * 0x30 + local_100);
    local_1b0 = uVar13 * 0x38 + local_128;
    local_1a0 = (undefined4 *)(local_140 + uVar13 * 4);
    uVar18 = uVar18 + 1;
    country_core::systems::camera_rig::MainCamera::mode();
    if (extraout_AL_00 == '\x03') goto code_r0x000142325448;
    goto LAB_1423254b2;
  }
  return;
code_r0x000142325448:
  country_core::systems::camera_rig::MainCamera::eye_position();
  if (5.0 < SQRT(((float)uStack_2a0 - pfVar19[6]) * ((float)uStack_2a0 - pfVar19[6]) +
                 (local_2a8._4_4_ - pfVar19[5]) * (local_2a8._4_4_ - pfVar19[5]) +
                 ((float)local_2a8 - pfVar19[4]) * ((float)local_2a8 - pfVar19[4])))
  goto LAB_1423251a2;
LAB_1423254b2:
  local_388 = (undefined **)(local_150 + 0x20);
  pcStack_380 = core::fmt::num::imp::impl$9::fmt;
  local_2a8 = &PTR_s_pet_sheep__14304bde8;
  uStack_2a0 = 1;
  uStack_290 = 1;
  local_288 = 0;
  local_298 = &local_388;
  local_1f8 = local_388;
  local_108 = uVar18;
  alloc::fmt::format::format_inner();
  local_388 = (undefined **)CONCAT44(0x3f800000,(float)local_388);
  pcStack_380 = (code *)CONCAT44((float)((ulonglong)*(undefined8 *)(pfVar19 + 4) >> 0x20) + 0.8,
                                 (float)*(undefined8 *)(pfVar19 + 4) + 0.0);
  local_378 = (undefined8 *)CONCAT44(local_378._4_4_,pfVar19[6] + 0.0);
  local_388 = (undefined **)((ulonglong)local_388 & 0xffffffffffffff00);
  local_f1 = 1;
  country_core::systems::ui::resources::widget_params::WidgetParams::new();
  local_f2 = 1;
  country_core::systems::ui::resources::widget_params::UiSounds::empty();
  local_f2 = 0;
  country_core::systems::ui::resources::widget_params::WidgetParams::sounds();
  local_f1 = 1;
  country_core::systems::ui::resources::widget_params::WidgetParams::no_history_recording();
  local_f1 = 0;
  country_core::systems::ui::resources::ui_state::UiState::add<alloc::string::String>();
  country_core::systems::ui::resources::ui_state::UiWidgetContext::intersecting();
  fVar27 = fStack_27c;
  fVar1 = fStack_280;
  if ((float)local_2a8 != 2.8026e-45) {
    fVar34 = local_288._4_4_;
    country_core::systems::camera_rig::MainCamera::eye_position();
    *in_stack_00000040 =
         SQRT((fVar27 - pcStack_380._0_4_) * (fVar27 - pcStack_380._0_4_) +
              (fVar1 - local_388._4_4_) * (fVar1 - local_388._4_4_) +
              (fVar34 - (float)local_388) * (fVar34 - (float)local_388));
  }
  uVar18 = local_108;
  country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active();
  lVar17 = local_110;
  if (extraout_AL_01 == '\0') goto LAB_142325a08;
  fVar1 = *in_stack_00000040;
  ::utils::geometry::ws_to_cs();
  fVar27 = extraout_XMM0_Da;
  if (extraout_XMM0_Da <= 1e-20) {
    fVar27 = 1e-20;
  }
  fVar27 = 0.075 / fVar27;
  if (1.0 <= fVar27) {
    fVar27 = 1.0;
  }
  fVar35 = *(float *)(lVar17 + 0xc4);
  fVar32 = *(float *)(lVar17 + 200);
  fVar2 = *(float *)(lVar17 + 0xd0);
  fVar9 = *(float *)(lVar17 + 0xd8);
  fVar29 = *(float *)(lVar17 + 0xe0);
  fVar28 = *(float *)(lVar17 + 0xe4);
  fVar34 = *(float *)(lVar17 + 0xe8);
  if (fVar34 <= 0.0) {
    fVar30 = *(float *)(lVar17 + 0xd4) - *(float *)(lVar17 + 0xc0);
    if (0.0 < fVar30) {
      fVar29 = fVar29 - fVar32;
      fVar32 = fVar2 + fVar35;
      fVar30 = (1.0 - fVar34) + fVar30;
      fVar35 = fVar9 + fVar28;
      fVar34 = fVar30;
      goto LAB_1423257dd;
    }
    fVar30 = (1.0 - fVar34) - fVar30;
    fVar31 = 0.5 / SQRT(fVar30);
    fVar33 = fVar31 * fVar30;
    fVar34 = fVar31 * (fVar35 + fVar2);
    fVar35 = fVar31 * (fVar32 + fVar29);
    fVar30 = fVar31 * (fVar9 - fVar28);
  }
  else {
    fVar30 = *(float *)(lVar17 + 0xd4) + *(float *)(lVar17 + 0xc0);
    if (fVar30 <= 0.0) {
      fVar32 = fVar32 + fVar29;
      fVar30 = (fVar34 + 1.0) - fVar30;
      fVar29 = fVar35 - fVar2;
      fVar35 = fVar30;
      fVar34 = fVar9 + fVar28;
LAB_1423257dd:
      fVar30 = 0.5 / SQRT(fVar30);
      fVar33 = fVar30 * fVar32;
      fVar34 = fVar30 * fVar34;
      fVar35 = fVar30 * fVar35;
      fVar30 = fVar30 * fVar29;
    }
    else {
      fVar30 = fVar34 + 1.0 + fVar30;
      fVar33 = 0.5 / SQRT(fVar30);
      fVar33 = fVar33 * (fVar9 - fVar28);
      fVar34 = fVar33 * (fVar29 - fVar32);
      fVar35 = fVar33 * (fVar35 - fVar2);
      fVar30 = fVar33 * fVar30;
    }
  }
  uVar3 = *local_1e8;
  fVar32 = *(float *)((longlong)local_1e8 + 0x14);
  fVar2 = *(float *)(local_1e8 + 1);
  uVar4 = *(undefined8 *)((longlong)local_1e8 + 0xc);
  local_388 = (undefined **)local_1e0;
  pcStack_380 = core::fmt::num::imp::impl$6::fmt;
  local_2a8 = (undefined **)&DAT_14304be00;
  uStack_2a0 = 1;
  uStack_290 = 1;
  local_288 = 0;
  local_298 = &local_388;
  alloc::fmt::format::format_inner();
  uVar10 = local_2d0;
  local_138 = local_2d8;
  std::alloc::__default_lib_allocator::__rust_alloc();
  local_168 = uVar10;
  if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  *extraout_RAX = 1;
  extraout_RAX[1] = 1;
  *(float *)(extraout_RAX + 2) =
       fVar35 * 1.0 * -1.0 + fVar34 * 0.0 * 1.0 + fVar33 * -4.371139e-08 * 1.0 + fVar30 * 0.0;
  *(float *)((longlong)extraout_RAX + 0x14) =
       fVar35 * 0.0 * 1.0 + fVar34 * -4.371139e-08 * 1.0 + fVar33 * 0.0 * -1.0 + fVar30 * 1.0;
  *(float *)(extraout_RAX + 3) =
       fVar35 * -4.371139e-08 * 1.0 + fVar34 * 0.0 * -1.0 + fVar33 * 1.0 * 1.0 + fVar30 * 0.0;
  *(float *)((longlong)extraout_RAX + 0x1c) =
       fVar35 * 0.0 * -1.0 + fVar34 * 1.0 * -1.0 + fVar33 * 0.0 * -1.0 + fVar30 * -4.371139e-08;
  extraout_RAX[4] =
       CONCAT44((float)((ulonglong)uVar3 >> 0x20) + fVar1 * (float)((ulonglong)uVar4 >> 0x20),
                (float)uVar3 + fVar1 * (float)uVar4);
  *(float *)(extraout_RAX + 5) = fVar32 * fVar1 + fVar2;
  *(float *)((longlong)extraout_RAX + 0x2c) = fVar27 * 0.8;
  country_core::systems::ui::signifiers::SignifierFactory::new();
  local_368 = local_288;
  local_378 = local_298;
  uStack_370 = uStack_290;
  local_388 = local_2a8;
  pcStack_380 = (code *)uStack_2a0;
  country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write();
  uVar18 = local_108;
  lVar24 = local_188;
  if (local_138 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
LAB_142325a08:
  country_core::systems::ui::resources::slider3d::impl$1::into_drag_state();
  local_2a8 = (undefined **)CONCAT71(local_2a8._1_7_,extraout_AL_02);
  country_core::systems::ui::resources::ui_state::DragState::is_some();
  if (extraout_AL_03 != '\0') {
    uVar3 = *local_1f8;
    *local_1d8 = local_158;
    lVar24 = *(longlong *)(local_190 + 0x30);
    uVar4 = *(undefined8 *)(local_190 + 0x40);
    if (lVar24 == *(longlong *)(local_190 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                ();
    }
    lVar17 = *(longlong *)(local_190 + 0x28);
    *(undefined8 *)(lVar17 + lVar24 * 0x10) = uVar4;
    *(undefined8 *)(lVar17 + 8 + lVar24 * 0x10) = uVar3;
    *(longlong *)(local_190 + 0x30) = lVar24 + 1;
    *(longlong *)(local_190 + 0x40) = *(longlong *)(local_190 + 0x40) + 1;
    uVar18 = local_108;
    lVar24 = local_188;
  }
  country_core::systems::ui::resources::slider3d::impl$1::into_drag_state();
  puVar23 = local_198;
  lVar17 = local_1b0;
  bVar11 = extraout_AL_04 - 2;
  if (2 < bVar11) {
    bVar11 = 3;
  }
  if (bVar11 == 1) {
    if (0.5 < *(float *)(local_1b0 + 0x34)) {
      country_core::resources::audio_resources::AudioInstance::start();
      *local_1a0 = local_114;
      *(undefined4 *)(lVar17 + 0x34) = 0;
    }
  }
  else if (bVar11 == 2) {
    uVar13 = *local_170;
    uVar15 = uVar13 - *(ulonglong *)(local_1d0 + 0x18);
    if (uVar13 < *(ulonglong *)(local_1d0 + 0x18)) {
      uVar15 = 0;
    }
    uVar14 = uVar13 - *(ulonglong *)(local_1d0 + 0x38);
    if (uVar13 < *(ulonglong *)(local_1d0 + 0x38)) {
      uVar14 = 0;
    }
    uVar13 = *(ulonglong *)(local_1d0 + 0x30);
    lVar17 = uVar13 - uVar14;
    bVar25 = lVar17 != 0;
    if (uVar13 < uVar14) {
      lVar17 = 0;
    }
    uVar7 = *(ulonglong *)(local_1d0 + 0x10);
    lVar20 = uVar7 - uVar15;
    bVar26 = lVar20 != 0;
    if (uVar7 < uVar15) {
      lVar20 = 0;
    }
    uVar8 = *(ulonglong *)(local_1d0 + 0x40);
    *local_170 = uVar8 - (lVar20 + lVar17);
    if (uVar7 >= uVar15 && bVar26 || uVar13 >= uVar14 && bVar25) {
      *local_170 = uVar8;
      country_core::systems::camera_rig::MainCamera::mode();
      if (extraout_AL_05 == '\x03') {
        uStack_2a0 = CONCAT44(uStack_2a0._4_4_,pfVar19[6]);
        local_2a8 = *(undefined ***)(pfVar19 + 4);
        *local_1c0 = local_154;
        lVar17 = *(longlong *)(local_160 + 0x30);
        uVar3 = *(undefined8 *)(local_160 + 0x40);
        if (lVar17 == *(longlong *)(local_160 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                    ();
        }
        lVar20 = *(longlong *)(local_160 + 0x28);
        lVar16 = lVar17 * 0x20;
        *(undefined8 *)(lVar20 + lVar16) = uVar3;
        *(undefined ***)(lVar20 + 8 + lVar16) = local_2a8;
        *(float *)(lVar20 + 0x10 + lVar16) = (float)uStack_2a0;
        *(undefined4 *)(lVar20 + 0x14 + lVar16) = 0x3f800000;
        *(undefined1 *)(lVar20 + 0x18 + lVar16) = 4;
        *(longlong *)(local_160 + 0x30) = lVar17 + 1;
        *(longlong *)(local_160 + 0x40) = *(longlong *)(local_160 + 0x40) + 1;
        uVar18 = local_108;
      }
    }
    fVar27 = pfVar19[7];
    fVar34 = pfVar19[8];
    fVar35 = pfVar19[9];
    fVar32 = *pfVar19;
    fVar2 = pfVar19[1];
    fVar9 = pfVar19[2];
    fVar29 = pfVar19[3];
    fVar1 = pfVar19[4];
    uVar3 = *(undefined8 *)(pfVar19 + 5);
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_00 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    fVar33 = fVar9 + fVar9;
    fVar31 = fVar2 + fVar2;
    fVar28 = fVar29 * (fVar32 + fVar32);
    fVar30 = (fVar32 + fVar32) * fVar32;
    *extraout_RAX_00 = 1;
    extraout_RAX_00[1] = 1;
    extraout_RAX_00[2] =
         CONCAT44((fVar33 * fVar2 - fVar28) * fVar35 * -1.15 +
                  (1.0 - (fVar9 * fVar33 + fVar30)) * fVar34 * 2.17 +
                  (fVar31 * fVar32 + fVar29 * fVar33) * fVar27 * 0.0 + (float)uVar3,
                  (fVar29 * fVar31 + fVar33 * fVar32) * fVar35 * -1.15 +
                  (fVar31 * fVar32 - fVar29 * fVar33) * fVar34 * 2.17 +
                  (1.0 - (fVar31 * fVar2 + fVar9 * fVar33)) * fVar27 * 0.0 + fVar1);
    *(float *)(extraout_RAX_00 + 3) =
         (1.0 - (fVar30 + fVar31 * fVar2)) * fVar35 * -1.15 +
         (fVar28 + fVar33 * fVar2) * fVar34 * 2.17 +
         (fVar33 * fVar32 - fVar29 * fVar31) * fVar27 * 0.0 + (float)((ulonglong)uVar3 >> 0x20);
    country_core::systems::ui::signifiers::SignifierFactory::new();
    country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write();
    *local_1a8 = local_114;
    *(undefined8 *)(local_150 + 8) = 0x40e0000000000001;
  }
  goto LAB_1423251a2;
}

