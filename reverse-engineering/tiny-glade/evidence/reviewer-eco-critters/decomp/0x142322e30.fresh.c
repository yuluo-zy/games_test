// ui_pet_ducks 0x142322e30 sig=void __cdecl ui_pet_ducks(void)

void __cdecl system_critters::ui_pet_ducks::ui_pet_ducks(void)

{
  uint *puVar1;
  float fVar2;
  float fVar3;
  undefined4 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  int iVar7;
  float fVar8;
  undefined8 uVar9;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  char extraout_AL_02;
  char cVar10;
  char extraout_AL_03;
  char extraout_AL_04;
  byte bVar11;
  uint *puVar12;
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *in_RCX;
  ulonglong uVar13;
  undefined8 *puVar14;
  undefined8 *in_RDX;
  longlong lVar15;
  longlong lVar16;
  longlong lVar17;
  longlong lVar18;
  longlong unaff_RSI;
  longlong lVar19;
  ulonglong uVar20;
  longlong unaff_RDI;
  ulonglong uVar21;
  longlong lVar22;
  longlong *in_R8;
  undefined8 *in_R9;
  float *pfVar23;
  bool bVar24;
  float fVar25;
  float fVar26;
  float fVar27;
  float fVar28;
  float fVar29;
  float fVar30;
  float fVar31;
  float fVar32;
  float fVar33;
  longlong *in_stack_00000028;
  longlong *in_stack_00000030;
  float *in_stack_00000038;
  undefined8 *in_stack_00000040;
  undefined8 local_348;
  code *pcStack_340;
  undefined8 *local_338;
  undefined8 uStack_330;
  undefined8 local_328;
  longlong local_290;
  undefined8 local_288;
  longlong local_278;
  undefined8 local_270;
  undefined8 local_268;
  undefined8 local_248;
  undefined8 uStack_240;
  undefined8 *local_238;
  undefined8 uStack_230;
  undefined8 local_228;
  float fStack_220;
  float fStack_21c;
  undefined8 local_198;
  undefined8 *local_190;
  undefined8 local_188;
  undefined8 local_180;
  longlong local_178;
  undefined8 local_170;
  undefined8 local_168;
  longlong local_160;
  longlong local_158;
  longlong local_150;
  longlong local_148;
  longlong local_140;
  undefined8 *local_138;
  longlong local_130;
  longlong local_128;
  undefined4 *local_120;
  longlong local_118;
  longlong local_110;
  longlong local_108;
  longlong local_100;
  undefined4 local_f8;
  char local_f3;
  undefined1 local_f2;
  undefined1 local_f1;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  lVar16 = *in_R8;
  local_118 = in_R8[1];
  local_f8 = *(undefined4 *)((longlong)in_R8 + 0x14);
  puVar12 = *(uint **)(lVar16 + 0xf8);
  puVar1 = puVar12 + *(longlong *)(lVar16 + 0x100);
  local_140 = *in_stack_00000028;
  if (*(char *)(lVar16 + 0x108) == '\0') {
    lVar15 = 8;
    lVar17 = 0;
    lVar18 = 0;
    lVar19 = 0;
    do {
      if (lVar19 == lVar18) {
        if (puVar12 == puVar1) goto LAB_14232318c;
        while( true ) {
          lVar18 = *(longlong *)
                    (*(longlong *)(local_118 + 0x100) + 0x58 + (ulonglong)*puVar12 * 0xa0);
          if (lVar18 != 0) break;
          puVar12 = puVar12 + 1;
          if (puVar12 == puVar1) goto LAB_14232318c;
        }
        lVar19 = *(longlong *)(local_118 + 0x100) + (ulonglong)*puVar12 * 0xa0;
        uVar20 = (ulonglong)*(uint *)(lVar19 + 0x94);
        lVar15 = *(longlong *)(*(longlong *)(local_118 + 0x1a8) + 0x18 + uVar20 * 0x48);
        lVar22 = ~*(ulonglong *)
                   (*(longlong *)(*(longlong *)(local_118 + 0x1a8) + 0x38 + uVar20 * 0x48) +
                   *(longlong *)(lVar16 + 0x110) * 8) * 0x30;
        lVar17 = *(longlong *)(lVar15 + 0x10 + lVar22);
        unaff_RDI = *(longlong *)(lVar15 + 0x28 + lVar22);
        lVar15 = *(longlong *)(lVar19 + 0x50);
        puVar12 = puVar12 + 1;
        lVar19 = 0;
      }
      uVar20 = (ulonglong)*(uint *)(lVar15 + 8 + lVar19 * 0x10);
      lVar22 = lVar17 + uVar20 * 0x100;
      lVar19 = lVar19 + 1;
      fVar32 = *(float *)(local_140 + 0x28);
      bVar24 = *(char *)(lVar17 + 0xfc + uVar20 * 0x100) == '\0';
      if (bVar24) {
        fVar32 = -fVar32;
      }
      if ((!bVar24) && (1.0 <= *(float *)(lVar22 + 0xf8))) {
        *(undefined4 *)(unaff_RDI + uVar20 * 4) = local_f8;
        *(undefined1 *)(lVar22 + 0xfc) = 0;
      }
      *(undefined4 *)(unaff_RDI + uVar20 * 4) = local_f8;
      fVar33 = fVar32 / 0.4 + *(float *)(lVar22 + 0xf8);
      fVar32 = 0.0;
      if (0.0 <= fVar33) {
        fVar32 = fVar33;
      }
      fVar33 = 1.0;
      if (fVar32 <= 1.0) {
        fVar33 = fVar32;
      }
      *(float *)(lVar22 + 0xf8) = fVar33;
    } while( true );
  }
  lVar18 = 0;
  uVar20 = 0;
  uVar21 = 0;
  do {
    if (uVar21 == uVar20) {
      if (puVar12 == puVar1) break;
      while( true ) {
        uVar20 = *(ulonglong *)
                  (*(longlong *)(local_118 + 0x1a8) + 0x10 + (ulonglong)*puVar12 * 0x48);
        if (uVar20 != 0) break;
        puVar12 = puVar12 + 1;
        if (puVar12 == puVar1) goto LAB_14232318c;
      }
      lVar18 = *(longlong *)(local_118 + 0x1a8) + (ulonglong)*puVar12 * 0x48;
      lVar17 = *(longlong *)(lVar18 + 0x18);
      lVar15 = ~*(ulonglong *)(*(longlong *)(lVar18 + 0x38) + *(longlong *)(lVar16 + 0x110) * 8) *
               0x30;
      lVar18 = *(longlong *)(lVar17 + 0x10 + lVar15);
      unaff_RSI = *(longlong *)(lVar17 + 0x28 + lVar15);
      puVar12 = puVar12 + 1;
      uVar21 = 0;
    }
    uVar13 = uVar21 & 0xffffffff;
    cVar10 = *(char *)(lVar18 + 0xfc + uVar13 * 0x100);
    fVar32 = *(float *)(local_140 + 0x28);
    if (cVar10 == '\0') {
      fVar32 = -fVar32;
    }
    lVar17 = uVar13 * 0x100 + lVar18;
    if ((cVar10 == '\x01') && (1.0 <= *(float *)(lVar17 + 0xf8))) {
      *(undefined4 *)(unaff_RSI + uVar13 * 4) = local_f8;
      *(undefined1 *)(lVar17 + 0xfc) = 0;
    }
    uVar21 = uVar21 + 1;
    *(undefined4 *)(unaff_RSI + uVar13 * 4) = local_f8;
    fVar33 = fVar32 / 0.4 + *(float *)(lVar17 + 0xf8);
    fVar32 = 0.0;
    if (0.0 <= fVar33) {
      fVar32 = fVar33;
    }
    fVar33 = 1.0;
    if (fVar32 <= 1.0) {
      fVar33 = fVar32;
    }
    *(float *)(lVar17 + 0xf8) = fVar33;
  } while( true );
LAB_14232318c:
  iVar7 = *(int *)*in_RCX;
  if (iVar7 == 0x13) {
    local_138 = in_R9;
    country_core::systems::camera_rig::MainCamera::mode();
    if (extraout_AL == '\x03') goto LAB_1423231d0;
  }
  else if ((iVar7 != 0xd) &&
          ((local_138 = in_R9, iVar7 != 6 || (1 < (byte)((char)((int *)*in_RCX)[1] - 1U))))) {
LAB_1423231d0:
    puVar12 = *(uint **)(lVar16 + 0xf8);
    puVar1 = puVar12 + *(longlong *)(lVar16 + 0x100);
    local_f3 = *(char *)(lVar16 + 0x108);
    local_128 = *in_stack_00000030;
    local_198 = *in_RDX;
    local_190 = (undefined8 *)*in_stack_00000040;
    local_170 = *local_138;
    local_140 = local_140 + 0x2c;
    local_130 = 0;
    local_108 = 0;
    local_100 = 0;
    uVar20 = 0;
    local_138 = (undefined8 *)0x0;
    lVar18 = 8;
    lVar17 = 8;
    local_178 = lVar16;
LAB_1423232de:
    do {
      if (local_f3 == '\0') {
        if (local_138 == (undefined8 *)uVar20) {
          if (puVar12 == puVar1) {
            return;
          }
          while( true ) {
            uVar20 = *(ulonglong *)
                      (*(longlong *)(local_118 + 0x100) + 0x58 + (ulonglong)*puVar12 * 0xa0);
            if (uVar20 != 0) break;
            puVar12 = puVar12 + 1;
            if (puVar12 == puVar1) {
              return;
            }
          }
          lVar15 = *(longlong *)(local_118 + 0x100) + (ulonglong)*puVar12 * 0xa0;
          uVar13 = (ulonglong)*(uint *)(lVar15 + 0x94);
          lVar16 = *(longlong *)(local_118 + 0x1a8);
          uVar21 = *(ulonglong *)(lVar16 + 0x40 + uVar13 * 0x48);
          lVar17 = *(longlong *)(lVar16 + 0x18 + uVar13 * 0x48);
          lVar16 = *(longlong *)(lVar16 + 0x38 + uVar13 * 0x48);
          if ((*(ulonglong *)(local_178 + 0x118) < uVar21) &&
             (uVar13 = *(ulonglong *)(lVar16 + *(ulonglong *)(local_178 + 0x118) * 8), uVar13 != 0))
          {
            local_108 = *(longlong *)(lVar17 + 0x10 + ~uVar13 * 0x30);
            uVar13 = *(ulonglong *)(local_178 + 0x120);
            if (uVar21 <= uVar13) goto LAB_1423234f2;
LAB_142323483:
            uVar21 = *(ulonglong *)(lVar16 + uVar13 * 8);
            if (uVar21 == 0) goto LAB_1423234f2;
            local_100 = *(longlong *)(lVar17 + 0x10 + ~uVar21 * 0x30);
          }
          else {
            local_108 = 0;
            uVar13 = *(ulonglong *)(local_178 + 0x120);
            if (uVar13 < uVar21) goto LAB_142323483;
LAB_1423234f2:
            local_100 = 0;
          }
          lVar16 = ~*(ulonglong *)(lVar16 + *(longlong *)(local_178 + 0x110) * 8) * 0x30;
          local_130 = *(longlong *)(lVar17 + 0x10 + lVar16);
          local_160 = *(longlong *)(lVar17 + 0x28 + lVar16);
          puVar12 = puVar12 + 1;
          lVar17 = *(longlong *)(lVar15 + 0x50);
          local_138 = (undefined8 *)0x0;
        }
        puVar14 = (undefined8 *)(lVar17 + (longlong)local_138 * 0x10);
        uVar21 = (ulonglong)*(uint *)(lVar17 + 8 + (longlong)local_138 * 0x10);
      }
      else {
        if (local_138 == (undefined8 *)uVar20) {
          if (puVar12 == puVar1) {
            return;
          }
          while( true ) {
            uVar20 = *(ulonglong *)
                      (*(longlong *)(local_118 + 0x1a8) + 0x10 + (ulonglong)*puVar12 * 0x48);
            if (uVar20 != 0) break;
            puVar12 = puVar12 + 1;
            if (puVar12 == puVar1) {
              return;
            }
          }
          lVar16 = *(longlong *)(local_118 + 0x1a8) + (ulonglong)*puVar12 * 0x48;
          uVar21 = *(ulonglong *)(lVar16 + 0x40);
          lVar18 = *(longlong *)(lVar16 + 0x18);
          lVar15 = *(longlong *)(lVar16 + 0x38);
          if ((*(ulonglong *)(local_178 + 0x118) < uVar21) &&
             (uVar13 = *(ulonglong *)(lVar15 + *(ulonglong *)(local_178 + 0x118) * 8), uVar13 != 0))
          {
            local_108 = *(longlong *)(lVar18 + 0x10 + ~uVar13 * 0x30);
            uVar13 = *(ulonglong *)(local_178 + 0x120);
            if (uVar21 <= uVar13) goto LAB_1423234a5;
LAB_14232344a:
            uVar21 = *(ulonglong *)(lVar15 + uVar13 * 8);
            if (uVar21 == 0) goto LAB_1423234a5;
            local_100 = *(longlong *)(lVar18 + 0x10 + ~uVar21 * 0x30);
          }
          else {
            local_108 = 0;
            uVar13 = *(ulonglong *)(local_178 + 0x120);
            if (uVar13 < uVar21) goto LAB_14232344a;
LAB_1423234a5:
            local_100 = 0;
          }
          lVar15 = ~*(ulonglong *)(lVar15 + *(longlong *)(local_178 + 0x110) * 8) * 0x30;
          local_130 = *(longlong *)(lVar18 + 0x10 + lVar15);
          local_160 = *(longlong *)(lVar18 + 0x28 + lVar15);
          puVar12 = puVar12 + 1;
          lVar18 = *(longlong *)(lVar16 + 8);
          local_138 = (undefined8 *)0x0;
        }
        puVar14 = (undefined8 *)(lVar18 + (longlong)local_138 * 8);
        uVar21 = (ulonglong)local_138 & 0xffffffff;
      }
      local_110 = uVar21 * 0x100 + local_130;
      local_120 = (undefined4 *)(local_160 + uVar21 * 4);
      pfVar23 = (float *)(uVar21 * 0x30 + local_108);
      local_180 = *puVar14;
      local_138 = (undefined8 *)((longlong)local_138 + 1);
    } while (*(char *)(uVar21 + local_100) != '\x02');
    country_core::systems::camera_rig::MainCamera::mode();
    if (extraout_AL_00 == '\x03') {
      country_core::systems::camera_rig::MainCamera::eye_position();
      if (6.0 < SQRT(((float)uStack_240 - pfVar23[6]) * ((float)uStack_240 - pfVar23[6]) +
                     (local_248._4_4_ - pfVar23[5]) * (local_248._4_4_ - pfVar23[5]) +
                     ((float)local_248 - pfVar23[4]) * ((float)local_248 - pfVar23[4])))
      goto LAB_1423232de;
    }
    local_348 = (undefined **)&local_180;
    pcStack_340 = bevy_ecs::entity::impl$10::fmt;
    local_248 = &PTR_s_pet_duck__14304bc58;
    uStack_240 = 1;
    uStack_230 = 1;
    local_228 = 0;
    local_238 = &local_348;
    local_158 = lVar18;
    local_150 = lVar17;
    alloc::fmt::format::format_inner();
    local_338 = (undefined8 *)CONCAT44(local_338._4_4_,pfVar23[6]);
    pcStack_340 = *(code **)(pfVar23 + 4);
    local_348 = (undefined **)CONCAT44(0x3f000000,(float)local_348);
    local_348 = (undefined **)((ulonglong)local_348 & 0xffffffffffffff00);
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
    fVar33 = fStack_21c;
    fVar32 = fStack_220;
    if ((float)local_248 == 2.8026e-45) {
      country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active();
      cVar10 = extraout_AL_01;
    }
    else {
      fVar31 = local_228._4_4_;
      country_core::systems::camera_rig::MainCamera::eye_position();
      *in_stack_00000038 =
           SQRT((fVar33 - pcStack_340._0_4_) * (fVar33 - pcStack_340._0_4_) +
                (fVar32 - local_348._4_4_) * (fVar32 - local_348._4_4_) +
                (fVar31 - (float)local_348) * (fVar31 - (float)local_348));
      country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active();
      cVar10 = extraout_AL_02;
    }
    if (cVar10 != '\0') {
      fVar33 = *(float *)(local_128 + 0xc4);
      fVar31 = *(float *)(local_128 + 200);
      fVar2 = *(float *)(local_128 + 0xd0);
      fVar3 = *(float *)(local_128 + 0xd8);
      fVar26 = *(float *)(local_128 + 0xe0);
      fVar8 = *(float *)(local_128 + 0xe4);
      fVar32 = *(float *)(local_128 + 0xe8);
      if (fVar32 <= 0.0) {
        fVar27 = *(float *)(local_128 + 0xd4) - *(float *)(local_128 + 0xc0);
        if (0.0 < fVar27) {
          fVar26 = fVar26 - fVar31;
          fVar31 = fVar2 + fVar33;
          fVar27 = (1.0 - fVar32) + fVar27;
          fVar33 = fVar3 + fVar8;
          fVar32 = fVar27;
          goto LAB_1423238fa;
        }
        fVar27 = (1.0 - fVar32) - fVar27;
        fVar25 = 0.5 / SQRT(fVar27);
        fVar28 = fVar25 * fVar27;
        fVar32 = fVar25 * (fVar33 + fVar2);
        fVar33 = fVar25 * (fVar31 + fVar26);
        fVar27 = fVar25 * (fVar3 - fVar8);
      }
      else {
        fVar27 = *(float *)(local_128 + 0xd4) + *(float *)(local_128 + 0xc0);
        if (fVar27 <= 0.0) {
          fVar31 = fVar31 + fVar26;
          fVar27 = (fVar32 + 1.0) - fVar27;
          fVar26 = fVar33 - fVar2;
          fVar33 = fVar27;
          fVar32 = fVar3 + fVar8;
LAB_1423238fa:
          fVar27 = 0.5 / SQRT(fVar27);
          fVar28 = fVar27 * fVar31;
          fVar32 = fVar27 * fVar32;
          fVar33 = fVar27 * fVar33;
          fVar27 = fVar27 * fVar26;
        }
        else {
          fVar27 = fVar32 + 1.0 + fVar27;
          fVar28 = 0.5 / SQRT(fVar27);
          fVar28 = fVar28 * (fVar3 - fVar8);
          fVar32 = fVar28 * (fVar26 - fVar31);
          fVar33 = fVar28 * (fVar33 - fVar2);
          fVar27 = fVar28 * fVar27;
        }
      }
      fVar31 = *in_stack_00000038;
      uVar6 = *local_190;
      uVar5 = *(undefined8 *)((longlong)local_190 + 0xc);
      fVar2 = *(float *)((longlong)local_190 + 0x14);
      fVar3 = *(float *)(local_190 + 1);
      country_core::systems::camera_rig::MainCamera::mode();
      local_348 = (undefined **)local_140;
      pcStack_340 = core::fmt::num::imp::impl$6::fmt;
      local_248 = (undefined **)&DAT_14304bc70;
      uStack_240 = 1;
      local_238 = &local_348;
      uStack_230 = 1;
      local_228 = 0;
      alloc::fmt::format::format_inner();
      uVar9 = local_270;
      local_148 = local_278;
      local_188 = local_268;
      std::alloc::__default_lib_allocator::__rust_alloc();
      local_168 = uVar9;
      if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
        alloc::alloc::handle_alloc_error();
      }
      uVar4 = *(undefined4 *)(&__real_3e4ccccd3f000000 + (ulonglong)(extraout_AL_03 == '\x03') * 4);
      *extraout_RAX = 1;
      extraout_RAX[1] = 1;
      *(float *)(extraout_RAX + 2) =
           fVar33 * 1.0 * -1.0 + fVar32 * 0.0 * 1.0 + fVar28 * -4.371139e-08 * 1.0 + fVar27 * 0.0;
      *(float *)((longlong)extraout_RAX + 0x14) =
           fVar33 * 0.0 * 1.0 + fVar32 * -4.371139e-08 * 1.0 + fVar28 * 0.0 * -1.0 + fVar27 * 1.0;
      *(float *)(extraout_RAX + 3) =
           fVar33 * -4.371139e-08 * 1.0 + fVar32 * 0.0 * -1.0 + fVar28 * 1.0 * 1.0 + fVar27 * 0.0;
      *(float *)((longlong)extraout_RAX + 0x1c) =
           fVar33 * 0.0 * -1.0 + fVar32 * 1.0 * -1.0 + fVar28 * 0.0 * -1.0 + fVar27 * -4.371139e-08;
      extraout_RAX[4] =
           CONCAT44((float)((ulonglong)uVar6 >> 0x20) + (float)((ulonglong)uVar5 >> 0x20) * fVar31,
                    (float)uVar6 + (float)uVar5 * fVar31);
      *(float *)(extraout_RAX + 5) = fVar2 * fVar31 + fVar3;
      *(undefined4 *)((longlong)extraout_RAX + 0x2c) = uVar4;
      country_core::systems::ui::signifiers::SignifierFactory::new();
      local_328 = local_228;
      local_338 = local_238;
      uStack_330 = uStack_230;
      local_348 = local_248;
      pcStack_340 = (code *)uStack_240;
      country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write();
      if (local_148 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
    country_core::systems::ui::resources::slider3d::impl$1::into_drag_state();
    bVar11 = extraout_AL_04 - 2;
    if (2 < bVar11) {
      bVar11 = 3;
    }
    if (bVar11 == 1) {
      *local_120 = local_f8;
      *(undefined1 *)(local_110 + 0xfc) = 1;
      country_core::resources::audio_resources::AudioInstance::start_if_not_playing();
      lVar18 = local_158;
      lVar17 = local_150;
    }
    else {
      lVar18 = local_158;
      lVar17 = local_150;
      if (bVar11 == 2) {
        fVar33 = pfVar23[7];
        fVar31 = pfVar23[8];
        fVar2 = pfVar23[9];
        fVar3 = *pfVar23;
        fVar26 = pfVar23[1];
        fVar8 = pfVar23[2];
        fVar27 = pfVar23[3];
        fVar32 = pfVar23[4];
        uVar6 = *(undefined8 *)(pfVar23 + 5);
        local_348 = (undefined **)&local_180;
        pcStack_340 = bevy_ecs::entity::impl$10::fmt;
        local_248 = (undefined **)&DAT_14304bcc0;
        uStack_240 = 1;
        uStack_230 = 1;
        local_228 = 0;
        local_238 = &local_348;
        alloc::fmt::format::format_inner();
        uVar5 = local_288;
        local_110 = local_290;
        std::alloc::__default_lib_allocator::__rust_alloc();
        local_120 = (undefined4 *)uVar5;
        if (extraout_RAX_00 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
          alloc::alloc::handle_alloc_error();
        }
        fVar30 = fVar8 + fVar8;
        fVar29 = fVar26 + fVar26;
        fVar25 = fVar27 * (fVar3 + fVar3);
        fVar28 = (fVar3 + fVar3) * fVar3;
        *extraout_RAX_00 = 1;
        extraout_RAX_00[1] = 1;
        extraout_RAX_00[2] =
             CONCAT44((fVar30 * fVar26 - fVar25) * fVar2 * 0.0 +
                      (1.0 - (fVar8 * fVar30 + fVar28)) * fVar31 * 0.8 +
                      (fVar29 * fVar3 + fVar27 * fVar30) * fVar33 * 0.0 + (float)uVar6,
                      (fVar27 * fVar29 + fVar30 * fVar3) * fVar2 * 0.0 +
                      (fVar29 * fVar3 - fVar27 * fVar30) * fVar31 * 0.8 +
                      (1.0 - (fVar29 * fVar26 + fVar8 * fVar30)) * fVar33 * 0.0 + fVar32);
        *(float *)(extraout_RAX_00 + 3) =
             (1.0 - (fVar28 + fVar29 * fVar26)) * fVar2 * 0.0 +
             (fVar25 + fVar30 * fVar26) * fVar31 * 0.8 +
             (fVar30 * fVar3 - fVar27 * fVar29) * fVar33 * 0.0 + (float)((ulonglong)uVar6 >> 0x20);
        country_core::systems::ui::signifiers::SignifierFactory::new();
        local_328 = local_228;
        local_338 = local_238;
        uStack_330 = uStack_230;
        local_348 = local_248;
        pcStack_340 = (code *)uStack_240;
        country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write();
        lVar17 = local_150;
        lVar18 = local_158;
        if (local_110 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
      }
    }
    goto LAB_1423232de;
  }
  return;
}

