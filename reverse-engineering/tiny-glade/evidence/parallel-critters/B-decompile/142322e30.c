
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

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
  uint uVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  float fVar13;
  undefined8 uVar14;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  char extraout_AL_02;
  char cVar15;
  char extraout_AL_03;
  char extraout_AL_04;
  byte bVar16;
  uint *puVar17;
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *in_RCX;
  ulonglong uVar18;
  undefined8 *puVar19;
  undefined8 *in_RDX;
  longlong lVar20;
  longlong lVar21;
  longlong lVar22;
  longlong lVar23;
  longlong unaff_RSI;
  longlong lVar24;
  ulonglong uVar25;
  longlong unaff_RDI;
  ulonglong uVar26;
  longlong lVar27;
  longlong *in_R8;
  undefined8 *in_R9;
  float *pfVar28;
  bool bVar29;
  float fVar30;
  float fVar31;
  float fVar32;
  float fVar33;
  float fVar34;
  float fVar35;
  float fVar36;
  float fVar37;
  float fVar38;
  float fVar39;
  float fVar40;
  float fVar41;
  float fVar42;
  float fVar43;
  float fVar44;
  float fVar45;
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
  
  fVar10 = __real_3ecccccd;
  fVar8 = __real_3f800000;
  uVar9 = ___xmm_80000000800000008000000080000000;
  local_f0 = 0xfffffffffffffffe;
  lVar21 = *in_R8;
  local_118 = in_R8[1];
  local_f8 = *(undefined4 *)((longlong)in_R8 + 0x14);
  puVar17 = *(uint **)(lVar21 + 0xf8);
  puVar1 = puVar17 + *(longlong *)(lVar21 + 0x100);
  local_140 = *in_stack_00000028;
  if (*(char *)(lVar21 + 0x108) == '\0') {
    lVar20 = 8;
    lVar22 = 0;
    lVar23 = 0;
    lVar24 = 0;
    do {
      if (lVar24 == lVar23) {
        if (puVar17 == puVar1) goto LAB_14232318c;
        while( true ) {
          lVar23 = *(longlong *)
                    (*(longlong *)(local_118 + 0x100) + 0x58 + (ulonglong)*puVar17 * 0xa0);
          if (lVar23 != 0) break;
          puVar17 = puVar17 + 1;
          if (puVar17 == puVar1) goto LAB_14232318c;
        }
        lVar24 = *(longlong *)(local_118 + 0x100) + (ulonglong)*puVar17 * 0xa0;
        uVar25 = (ulonglong)*(uint *)(lVar24 + 0x94);
        lVar20 = *(longlong *)(*(longlong *)(local_118 + 0x1a8) + 0x18 + uVar25 * 0x48);
        lVar27 = ~*(ulonglong *)
                   (*(longlong *)(*(longlong *)(local_118 + 0x1a8) + 0x38 + uVar25 * 0x48) +
                   *(longlong *)(lVar21 + 0x110) * 8) * 0x30;
        lVar22 = *(longlong *)(lVar20 + 0x10 + lVar27);
        unaff_RDI = *(longlong *)(lVar20 + 0x28 + lVar27);
        lVar20 = *(longlong *)(lVar24 + 0x50);
        puVar17 = puVar17 + 1;
        lVar24 = 0;
      }
      uVar25 = (ulonglong)*(uint *)(lVar20 + 8 + lVar24 * 0x10);
      lVar27 = lVar22 + uVar25 * 0x100;
      lVar24 = lVar24 + 1;
      fVar40 = *(float *)(local_140 + 0x28);
      bVar29 = *(char *)(lVar22 + 0xfc + uVar25 * 0x100) == '\0';
      if (bVar29) {
        fVar40 = (float)((uint)fVar40 ^ uVar9);
      }
      if ((!bVar29) && (fVar8 <= *(float *)(lVar27 + 0xf8))) {
        *(undefined4 *)(unaff_RDI + uVar25 * 4) = local_f8;
        *(undefined1 *)(lVar27 + 0xfc) = 0;
      }
      *(undefined4 *)(unaff_RDI + uVar25 * 4) = local_f8;
      fVar39 = fVar40 / fVar10 + *(float *)(lVar27 + 0xf8);
      fVar40 = 0.0;
      if (0.0 <= fVar39) {
        fVar40 = fVar39;
      }
      fVar39 = fVar8;
      if (fVar40 <= fVar8) {
        fVar39 = fVar40;
      }
      *(float *)(lVar27 + 0xf8) = fVar39;
    } while( true );
  }
  lVar23 = 0;
  uVar25 = 0;
  uVar26 = 0;
  do {
    if (uVar26 == uVar25) {
      if (puVar17 == puVar1) break;
      while( true ) {
        uVar25 = *(ulonglong *)
                  (*(longlong *)(local_118 + 0x1a8) + 0x10 + (ulonglong)*puVar17 * 0x48);
        if (uVar25 != 0) break;
        puVar17 = puVar17 + 1;
        if (puVar17 == puVar1) goto LAB_14232318c;
      }
      lVar23 = *(longlong *)(local_118 + 0x1a8) + (ulonglong)*puVar17 * 0x48;
      lVar22 = *(longlong *)(lVar23 + 0x18);
      lVar20 = ~*(ulonglong *)(*(longlong *)(lVar23 + 0x38) + *(longlong *)(lVar21 + 0x110) * 8) *
               0x30;
      lVar23 = *(longlong *)(lVar22 + 0x10 + lVar20);
      unaff_RSI = *(longlong *)(lVar22 + 0x28 + lVar20);
      puVar17 = puVar17 + 1;
      uVar26 = 0;
    }
    uVar18 = uVar26 & 0xffffffff;
    cVar15 = *(char *)(lVar23 + 0xfc + uVar18 * 0x100);
    fVar40 = *(float *)(local_140 + 0x28);
    if (cVar15 == '\0') {
      fVar40 = (float)((uint)fVar40 ^ uVar9);
    }
    lVar22 = uVar18 * 0x100 + lVar23;
    if ((cVar15 == '\x01') && (fVar8 <= *(float *)(lVar22 + 0xf8))) {
      *(undefined4 *)(unaff_RSI + uVar18 * 4) = local_f8;
      *(undefined1 *)(lVar22 + 0xfc) = 0;
    }
    uVar26 = uVar26 + 1;
    *(undefined4 *)(unaff_RSI + uVar18 * 4) = local_f8;
    fVar39 = fVar40 / fVar10 + *(float *)(lVar22 + 0xf8);
    fVar40 = 0.0;
    if (0.0 <= fVar39) {
      fVar40 = fVar39;
    }
    fVar39 = fVar8;
    if (fVar40 <= fVar8) {
      fVar39 = fVar40;
    }
    *(float *)(lVar22 + 0xf8) = fVar39;
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
    fVar13 = _UNK_142c2910c;
    fVar12 = _UNK_142c29108;
    fVar11 = _UNK_142c29104;
    fVar39 = ___xmm_b33bbd2e000000003f80000000000000;
    fVar40 = __real_40c00000;
    fVar10 = __real_3f800000;
    fVar8 = __real_3f000000;
    puVar17 = *(uint **)(lVar21 + 0xf8);
    puVar1 = puVar17 + *(longlong *)(lVar21 + 0x100);
    local_f3 = *(char *)(lVar21 + 0x108);
    local_128 = *in_stack_00000030;
    local_198 = *in_RDX;
    local_190 = (undefined8 *)*in_stack_00000040;
    local_170 = *local_138;
    local_140 = local_140 + 0x2c;
    local_130 = 0;
    local_108 = 0;
    local_100 = 0;
    uVar25 = 0;
    local_138 = (undefined8 *)0x0;
    lVar23 = 8;
    lVar22 = 8;
    local_178 = lVar21;
LAB_1423232de:
    do {
      if (local_f3 == '\0') {
        if (local_138 == (undefined8 *)uVar25) {
          if (puVar17 == puVar1) {
            return;
          }
          while( true ) {
            uVar25 = *(ulonglong *)
                      (*(longlong *)(local_118 + 0x100) + 0x58 + (ulonglong)*puVar17 * 0xa0);
            if (uVar25 != 0) break;
            puVar17 = puVar17 + 1;
            if (puVar17 == puVar1) {
              return;
            }
          }
          lVar20 = *(longlong *)(local_118 + 0x100) + (ulonglong)*puVar17 * 0xa0;
          uVar18 = (ulonglong)*(uint *)(lVar20 + 0x94);
          lVar21 = *(longlong *)(local_118 + 0x1a8);
          uVar26 = *(ulonglong *)(lVar21 + 0x40 + uVar18 * 0x48);
          lVar22 = *(longlong *)(lVar21 + 0x18 + uVar18 * 0x48);
          lVar21 = *(longlong *)(lVar21 + 0x38 + uVar18 * 0x48);
          if ((*(ulonglong *)(local_178 + 0x118) < uVar26) &&
             (uVar18 = *(ulonglong *)(lVar21 + *(ulonglong *)(local_178 + 0x118) * 8), uVar18 != 0))
          {
            local_108 = *(longlong *)(lVar22 + 0x10 + ~uVar18 * 0x30);
            uVar18 = *(ulonglong *)(local_178 + 0x120);
            if (uVar26 <= uVar18) goto LAB_1423234f2;
LAB_142323483:
            uVar26 = *(ulonglong *)(lVar21 + uVar18 * 8);
            if (uVar26 == 0) goto LAB_1423234f2;
            local_100 = *(longlong *)(lVar22 + 0x10 + ~uVar26 * 0x30);
          }
          else {
            local_108 = 0;
            uVar18 = *(ulonglong *)(local_178 + 0x120);
            if (uVar18 < uVar26) goto LAB_142323483;
LAB_1423234f2:
            local_100 = 0;
          }
          lVar21 = ~*(ulonglong *)(lVar21 + *(longlong *)(local_178 + 0x110) * 8) * 0x30;
          local_130 = *(longlong *)(lVar22 + 0x10 + lVar21);
          local_160 = *(longlong *)(lVar22 + 0x28 + lVar21);
          puVar17 = puVar17 + 1;
          lVar22 = *(longlong *)(lVar20 + 0x50);
          local_138 = (undefined8 *)0x0;
        }
        puVar19 = (undefined8 *)(lVar22 + (longlong)local_138 * 0x10);
        uVar26 = (ulonglong)*(uint *)(lVar22 + 8 + (longlong)local_138 * 0x10);
      }
      else {
        if (local_138 == (undefined8 *)uVar25) {
          if (puVar17 == puVar1) {
            return;
          }
          while( true ) {
            uVar25 = *(ulonglong *)
                      (*(longlong *)(local_118 + 0x1a8) + 0x10 + (ulonglong)*puVar17 * 0x48);
            if (uVar25 != 0) break;
            puVar17 = puVar17 + 1;
            if (puVar17 == puVar1) {
              return;
            }
          }
          lVar21 = *(longlong *)(local_118 + 0x1a8) + (ulonglong)*puVar17 * 0x48;
          uVar26 = *(ulonglong *)(lVar21 + 0x40);
          lVar23 = *(longlong *)(lVar21 + 0x18);
          lVar20 = *(longlong *)(lVar21 + 0x38);
          if ((*(ulonglong *)(local_178 + 0x118) < uVar26) &&
             (uVar18 = *(ulonglong *)(lVar20 + *(ulonglong *)(local_178 + 0x118) * 8), uVar18 != 0))
          {
            local_108 = *(longlong *)(lVar23 + 0x10 + ~uVar18 * 0x30);
            uVar18 = *(ulonglong *)(local_178 + 0x120);
            if (uVar26 <= uVar18) goto LAB_1423234a5;
LAB_14232344a:
            uVar26 = *(ulonglong *)(lVar20 + uVar18 * 8);
            if (uVar26 == 0) goto LAB_1423234a5;
            local_100 = *(longlong *)(lVar23 + 0x10 + ~uVar26 * 0x30);
          }
          else {
            local_108 = 0;
            uVar18 = *(ulonglong *)(local_178 + 0x120);
            if (uVar18 < uVar26) goto LAB_14232344a;
LAB_1423234a5:
            local_100 = 0;
          }
          lVar20 = ~*(ulonglong *)(lVar20 + *(longlong *)(local_178 + 0x110) * 8) * 0x30;
          local_130 = *(longlong *)(lVar23 + 0x10 + lVar20);
          local_160 = *(longlong *)(lVar23 + 0x28 + lVar20);
          puVar17 = puVar17 + 1;
          lVar23 = *(longlong *)(lVar21 + 8);
          local_138 = (undefined8 *)0x0;
        }
        puVar19 = (undefined8 *)(lVar23 + (longlong)local_138 * 8);
        uVar26 = (ulonglong)local_138 & 0xffffffff;
      }
      local_110 = uVar26 * 0x100 + local_130;
      local_120 = (undefined4 *)(local_160 + uVar26 * 4);
      pfVar28 = (float *)(uVar26 * 0x30 + local_108);
      local_180 = *puVar19;
      local_138 = (undefined8 *)((longlong)local_138 + 1);
    } while (*(char *)(uVar26 + local_100) != '\x02');
    country_core::systems::camera_rig::MainCamera::mode();
    if (extraout_AL_00 == '\x03') {
      country_core::systems::camera_rig::MainCamera::eye_position();
      if (fVar40 < SQRT(((float)uStack_240 - pfVar28[6]) * ((float)uStack_240 - pfVar28[6]) +
                        (local_248._4_4_ - pfVar28[5]) * (local_248._4_4_ - pfVar28[5]) +
                        ((float)local_248 - pfVar28[4]) * ((float)local_248 - pfVar28[4])))
      goto LAB_1423232de;
    }
    local_348 = (undefined **)&local_180;
    pcStack_340 = bevy_ecs::entity::impl$10::fmt;
    local_248 = &PTR_s_pet_duck__14304bc58;
    uStack_240 = 1;
    uStack_230 = 1;
    local_228 = 0;
    local_238 = &local_348;
    local_158 = lVar23;
    local_150 = lVar22;
    alloc::fmt::format::format_inner();
    local_338 = (undefined8 *)CONCAT44(local_338._4_4_,pfVar28[6]);
    pcStack_340 = *(code **)(pfVar28 + 4);
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
    fVar44 = fStack_21c;
    fVar42 = fStack_220;
    if ((float)local_248 == 2.8026e-45) {
      country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active();
      cVar15 = extraout_AL_01;
    }
    else {
      fVar35 = local_228._4_4_;
      country_core::systems::camera_rig::MainCamera::eye_position();
      *in_stack_00000038 =
           SQRT((fVar44 - pcStack_340._0_4_) * (fVar44 - pcStack_340._0_4_) +
                (fVar42 - local_348._4_4_) * (fVar42 - local_348._4_4_) +
                (fVar35 - (float)local_348) * (fVar35 - (float)local_348));
      country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active();
      cVar15 = extraout_AL_02;
    }
    if (cVar15 != '\0') {
      fVar44 = *(float *)(local_128 + 0xc4);
      fVar35 = *(float *)(local_128 + 200);
      fVar2 = *(float *)(local_128 + 0xd0);
      fVar3 = *(float *)(local_128 + 0xd8);
      fVar31 = *(float *)(local_128 + 0xe0);
      fVar33 = *(float *)(local_128 + 0xe4);
      fVar42 = *(float *)(local_128 + 0xe8);
      if (fVar42 <= 0.0) {
        fVar36 = *(float *)(local_128 + 0xd4) - *(float *)(local_128 + 0xc0);
        if (0.0 < fVar36) {
          fVar31 = fVar31 - fVar35;
          fVar35 = fVar2 + fVar44;
          fVar36 = (fVar10 - fVar42) + fVar36;
          fVar44 = fVar3 + fVar33;
          fVar42 = fVar36;
          goto LAB_1423238fa;
        }
        fVar36 = (fVar10 - fVar42) - fVar36;
        fVar30 = fVar8 / SQRT(fVar36);
        fVar32 = fVar30 * fVar36;
        fVar42 = fVar30 * (fVar44 + fVar2);
        fVar44 = fVar30 * (fVar35 + fVar31);
        fVar36 = fVar30 * (fVar3 - fVar33);
      }
      else {
        fVar36 = *(float *)(local_128 + 0xd4) + *(float *)(local_128 + 0xc0);
        if (fVar36 <= 0.0) {
          fVar35 = fVar35 + fVar31;
          fVar36 = (fVar42 + fVar10) - fVar36;
          fVar31 = fVar44 - fVar2;
          fVar44 = fVar36;
          fVar42 = fVar3 + fVar33;
LAB_1423238fa:
          fVar36 = fVar8 / SQRT(fVar36);
          fVar32 = fVar36 * fVar35;
          fVar42 = fVar36 * fVar42;
          fVar44 = fVar36 * fVar44;
          fVar36 = fVar36 * fVar31;
        }
        else {
          fVar36 = fVar42 + fVar10 + fVar36;
          fVar32 = fVar8 / SQRT(fVar36);
          fVar32 = fVar32 * (fVar3 - fVar33);
          fVar42 = fVar32 * (fVar31 - fVar35);
          fVar44 = fVar32 * (fVar44 - fVar2);
          fVar36 = fVar32 * fVar36;
        }
      }
      fVar35 = *in_stack_00000038;
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
      uVar14 = local_270;
      local_148 = local_278;
      local_188 = local_268;
      std::alloc::__default_lib_allocator::__rust_alloc();
      local_168 = uVar14;
      if (extraout_RAX == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::alloc::handle_alloc_error();
      }
      uVar4 = *(undefined4 *)(&__real_3e4ccccd3f000000 + (ulonglong)(extraout_AL_03 == '\x03') * 4);
      fVar34 = fVar32 * ___xmm_000000003f80000000000000b33bbd2e *
               ___xmm_bf8000003f800000bf8000003f800000;
      fVar37 = fVar32 * _UNK_142c29114 * _UNK_1429cd174;
      fVar38 = fVar32 * _UNK_142c29118 * _UNK_1429cd178;
      fVar32 = fVar32 * _UNK_142c2911c * _UNK_1429cd17c;
      fVar31 = fVar42 * ___xmm_3f80000000000000b33bbd2e00000000 *
               ___xmm_bf800000bf8000003f8000003f800000;
      fVar33 = fVar42 * _UNK_142c29124 * _UNK_1429cd184;
      fVar30 = fVar42 * _UNK_142c29128 * _UNK_1429cd188;
      fVar42 = fVar42 * _UNK_142c2912c * _UNK_1429cd18c;
      fVar41 = fVar44 * ___xmm_00000000b33bbd2e000000003f800000 *
               ___xmm_bf8000003f8000003f800000bf800000;
      fVar43 = fVar44 * _UNK_142c29134 * _UNK_1429cd194;
      fVar45 = fVar44 * _UNK_142c29138 * _UNK_1429cd198;
      fVar44 = fVar44 * _UNK_142c2913c * _UNK_1429cd19c;
      *extraout_RAX = 1;
      extraout_RAX[1] = 1;
      *(float *)(extraout_RAX + 2) = fVar41 + fVar31 + fVar34 + fVar36 * fVar39;
      *(float *)((longlong)extraout_RAX + 0x14) = fVar43 + fVar33 + fVar37 + fVar36 * fVar11;
      *(float *)(extraout_RAX + 3) = fVar45 + fVar30 + fVar38 + fVar36 * fVar12;
      *(float *)((longlong)extraout_RAX + 0x1c) = fVar44 + fVar42 + fVar32 + fVar36 * fVar13;
      extraout_RAX[4] =
           CONCAT44((float)((ulonglong)uVar6 >> 0x20) + (float)((ulonglong)uVar5 >> 0x20) * fVar35,
                    (float)uVar6 + (float)uVar5 * fVar35);
      *(float *)(extraout_RAX + 5) = fVar2 * fVar35 + fVar3;
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
    bVar16 = extraout_AL_04 - 2;
    if (2 < bVar16) {
      bVar16 = 3;
    }
    if (bVar16 == 1) {
      *local_120 = local_f8;
      *(undefined1 *)(local_110 + 0xfc) = 1;
      country_core::resources::audio_resources::AudioInstance::start_if_not_playing();
      lVar23 = local_158;
      lVar22 = local_150;
    }
    else {
      lVar23 = local_158;
      lVar22 = local_150;
      if (bVar16 == 2) {
        fVar44 = pfVar28[7];
        fVar35 = pfVar28[8];
        fVar2 = pfVar28[9];
        fVar3 = *pfVar28;
        fVar31 = pfVar28[1];
        fVar33 = pfVar28[2];
        fVar36 = pfVar28[3];
        fVar42 = pfVar28[4];
        uVar6 = *(undefined8 *)(pfVar28 + 5);
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
                    /* WARNING: Subroutine does not return */
          alloc::alloc::handle_alloc_error();
        }
        fVar41 = fVar33 + fVar33;
        fVar38 = fVar31 + fVar31;
        fVar30 = fVar36 * (fVar3 + fVar3);
        fVar37 = (fVar3 + fVar3) * fVar3;
        fVar32 = (fVar38 * fVar3 - fVar36 * fVar41) * fVar35 *
                 ___xmm_000000003f4ccccd3f4ccccd3f4ccccd;
        fVar34 = (fVar10 - (fVar33 * fVar41 + fVar37)) * fVar35 * _UNK_14304bbd4;
        fVar35 = (fVar30 + fVar41 * fVar31) * fVar35 * _UNK_14304bbd8;
        *extraout_RAX_00 = 1;
        extraout_RAX_00[1] = 1;
        extraout_RAX_00[2] =
             CONCAT44((fVar41 * fVar31 - fVar30) * fVar2 * 0.0 +
                      fVar34 + (fVar38 * fVar3 + fVar36 * fVar41) * fVar44 * 0.0 + (float)uVar6,
                      (fVar36 * fVar38 + fVar41 * fVar3) * fVar2 * 0.0 +
                      fVar32 + (fVar10 - (fVar38 * fVar31 + fVar33 * fVar41)) * fVar44 * 0.0 +
                      fVar42);
        *(float *)(extraout_RAX_00 + 3) =
             (fVar10 - (fVar37 + fVar38 * fVar31)) * fVar2 * 0.0 +
             fVar35 + (fVar41 * fVar3 - fVar36 * fVar38) * fVar44 * 0.0 +
             (float)((ulonglong)uVar6 >> 0x20);
        country_core::systems::ui::signifiers::SignifierFactory::new();
        local_328 = local_228;
        local_338 = local_238;
        uStack_330 = uStack_230;
        local_348 = local_248;
        pcStack_340 = (code *)uStack_240;
        country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write();
        lVar22 = local_150;
        lVar23 = local_158;
        if (local_110 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
      }
    }
    goto LAB_1423232de;
  }
  return;
}

