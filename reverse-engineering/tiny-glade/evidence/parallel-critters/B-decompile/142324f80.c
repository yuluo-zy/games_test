
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

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
  ulonglong uVar9;
  float fVar10;
  float fVar11;
  undefined8 uVar12;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  undefined1 extraout_AL_02;
  char extraout_AL_03;
  char extraout_AL_04;
  byte bVar13;
  char extraout_AL_05;
  longlong lVar14;
  ulonglong uVar15;
  undefined8 *extraout_RAX;
  ulonglong uVar16;
  undefined8 *extraout_RAX_00;
  undefined8 *in_RCX;
  ulonglong uVar17;
  longlong lVar18;
  undefined8 *in_RDX;
  longlong lVar19;
  ulonglong uVar20;
  float *pfVar21;
  longlong *in_R8;
  longlong *in_R9;
  longlong lVar22;
  ulonglong uVar23;
  uint *puVar24;
  uint *puVar25;
  longlong lVar26;
  bool bVar27;
  bool bVar28;
  float extraout_XMM0_Da;
  float fVar29;
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
  float fVar46;
  float fVar47;
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
    fVar11 = __real_3f800000;
    fVar10 = __real_3f000000;
    local_120 = *in_R8;
    lVar26 = in_R8[1];
    local_114 = *(undefined4 *)((longlong)in_R8 + 0x14);
    puVar24 = *(uint **)(local_120 + 0xf8);
    puVar25 = puVar24 + *(longlong *)(local_120 + 0x100);
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
    fVar40 = (float)__xmm_00000000000000003f4ccccd00000000;
    uVar9 = (ulonglong)__xmm_00000000000000003f4ccccd00000000 >> 0x20;
    local_f3 = *(char *)(local_120 + 0x108);
    local_1c8 = local_190 + 0x20;
    local_1b8 = local_160 + 0x20;
    local_180 = 8;
    local_100 = 0;
    local_130 = 0;
    local_128 = 0;
    uVar23 = 0;
    uVar20 = 0;
    local_198 = puVar25;
    local_188 = lVar26;
LAB_1423251a2:
    if (local_f3 == '\0') {
      if (uVar20 == uVar23) {
        do {
          if (puVar24 == puVar25) {
            return;
          }
          uVar6 = *puVar24;
          puVar24 = puVar24 + 1;
          uVar23 = *(ulonglong *)(*(longlong *)(lVar26 + 0x100) + 0x58 + (ulonglong)uVar6 * 0xa0);
        } while (uVar23 == 0);
        lVar14 = *(longlong *)(lVar26 + 0x100) + (ulonglong)uVar6 * 0xa0;
        uVar20 = (ulonglong)*(uint *)(lVar14 + 0x94);
        lVar19 = *(longlong *)(lVar26 + 0x1a8);
        lVar22 = *(longlong *)(lVar19 + 0x18 + uVar20 * 0x48);
        lVar18 = *(longlong *)(lVar19 + 0x38 + uVar20 * 0x48);
        if ((*(ulonglong *)(local_120 + 0x118) < *(ulonglong *)(lVar19 + 0x40 + uVar20 * 0x48)) &&
           (uVar20 = *(ulonglong *)(lVar18 + *(ulonglong *)(local_120 + 0x118) * 8), uVar20 != 0)) {
          local_100 = *(longlong *)(lVar22 + 0x10 + ~uVar20 * 0x30);
        }
        else {
          local_100 = 0;
        }
        lVar19 = ~*(ulonglong *)(lVar18 + *(longlong *)(local_120 + 0x110) * 8) * 0x30;
        local_130 = *(longlong *)(lVar22 + 0x10 + lVar19);
        local_148 = *(longlong *)(lVar22 + 0x28 + lVar19);
        lVar19 = ~*(ulonglong *)(lVar18 + *(longlong *)(local_120 + 0x120) * 8) * 0x30;
        local_128 = *(longlong *)(lVar22 + 0x10 + lVar19);
        local_140 = *(longlong *)(lVar22 + 0x28 + lVar19);
        local_180 = *(longlong *)(lVar14 + 0x50);
        uVar20 = 0;
      }
      uVar15 = (ulonglong)*(uint *)(local_180 + 8 + uVar20 * 0x10);
    }
    else {
      if (uVar20 == uVar23) {
        do {
          if (puVar24 == puVar25) {
            return;
          }
          uVar6 = *puVar24;
          puVar24 = puVar24 + 1;
          uVar23 = *(ulonglong *)(*(longlong *)(lVar26 + 0x1a8) + 0x10 + (ulonglong)uVar6 * 0x48);
        } while (uVar23 == 0);
        lVar19 = *(longlong *)(lVar26 + 0x1a8) + (ulonglong)uVar6 * 0x48;
        lVar22 = *(longlong *)(lVar19 + 0x18);
        lVar18 = *(longlong *)(lVar19 + 0x38);
        if ((*(ulonglong *)(local_120 + 0x118) < *(ulonglong *)(lVar19 + 0x40)) &&
           (uVar20 = *(ulonglong *)(lVar18 + *(ulonglong *)(local_120 + 0x118) * 8), uVar20 != 0)) {
          local_100 = *(longlong *)(lVar22 + 0x10 + ~uVar20 * 0x30);
        }
        else {
          local_100 = 0;
        }
        lVar19 = ~*(ulonglong *)(lVar18 + *(longlong *)(local_120 + 0x110) * 8) * 0x30;
        local_130 = *(longlong *)(lVar22 + 0x10 + lVar19);
        local_148 = *(longlong *)(lVar22 + 0x28 + lVar19);
        lVar19 = ~*(ulonglong *)(lVar18 + *(longlong *)(local_120 + 0x120) * 8) * 0x30;
        local_128 = *(longlong *)(lVar22 + 0x10 + lVar19);
        local_140 = *(longlong *)(lVar22 + 0x28 + lVar19);
        uVar20 = 0;
      }
      uVar15 = uVar20 & 0xffffffff;
    }
    local_150 = uVar15 * 0x68 + local_130;
    local_1a8 = (undefined4 *)(local_148 + uVar15 * 4);
    pfVar21 = (float *)(uVar15 * 0x30 + local_100);
    local_1b0 = uVar15 * 0x38 + local_128;
    local_1a0 = (undefined4 *)(local_140 + uVar15 * 4);
    uVar20 = uVar20 + 1;
    country_core::systems::camera_rig::MainCamera::mode();
    if (extraout_AL_00 == '\x03') goto code_r0x000142325448;
    goto LAB_1423254b2;
  }
  return;
code_r0x000142325448:
  country_core::systems::camera_rig::MainCamera::eye_position();
  if (__real_40a00000 <
      SQRT(((float)uStack_2a0 - pfVar21[6]) * ((float)uStack_2a0 - pfVar21[6]) +
           (local_2a8._4_4_ - pfVar21[5]) * (local_2a8._4_4_ - pfVar21[5]) +
           ((float)local_2a8 - pfVar21[4]) * ((float)local_2a8 - pfVar21[4]))) goto LAB_1423251a2;
LAB_1423254b2:
  local_388 = (undefined **)(local_150 + 0x20);
  pcStack_380 = core::fmt::num::imp::impl$9::fmt;
  local_2a8 = &PTR_s_pet_sheep__14304bde8;
  uStack_2a0 = 1;
  uStack_290 = 1;
  local_288 = 0;
  local_298 = &local_388;
  local_1f8 = local_388;
  local_108 = uVar20;
  alloc::fmt::format::format_inner();
  local_388 = (undefined **)CONCAT44(0x3f800000,(float)local_388);
  pcStack_380 = (code *)CONCAT44((float)((ulonglong)*(undefined8 *)(pfVar21 + 4) >> 0x20) +
                                 (float)uVar9,(float)*(undefined8 *)(pfVar21 + 4) + fVar40);
  local_378 = (undefined8 *)CONCAT44(local_378._4_4_,pfVar21[6] + 0.0);
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
  fVar43 = fStack_27c;
  fVar1 = fStack_280;
  if ((float)local_2a8 != 2.8026e-45) {
    fVar47 = local_288._4_4_;
    country_core::systems::camera_rig::MainCamera::eye_position();
    *in_stack_00000040 =
         SQRT((fVar43 - pcStack_380._0_4_) * (fVar43 - pcStack_380._0_4_) +
              (fVar1 - local_388._4_4_) * (fVar1 - local_388._4_4_) +
              (fVar47 - (float)local_388) * (fVar47 - (float)local_388));
  }
  uVar20 = local_108;
  country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active();
  lVar19 = local_110;
  if (extraout_AL_01 == '\0') goto LAB_142325a08;
  fVar1 = *in_stack_00000040;
  ::utils::geometry::ws_to_cs();
  fVar43 = extraout_XMM0_Da;
  if (extraout_XMM0_Da <= __real_1e3ce508) {
    fVar43 = __real_1e3ce508;
  }
  fVar47 = __real_3d99999a / fVar43;
  if (fVar11 <= __real_3d99999a / fVar43) {
    fVar47 = fVar11;
  }
  fVar45 = *(float *)(lVar19 + 0xc4);
  fVar39 = *(float *)(lVar19 + 200);
  fVar2 = *(float *)(lVar19 + 0xd0);
  fVar29 = *(float *)(lVar19 + 0xd8);
  fVar30 = *(float *)(lVar19 + 0xe0);
  fVar31 = *(float *)(lVar19 + 0xe4);
  fVar43 = *(float *)(lVar19 + 0xe8);
  if (fVar43 <= 0.0) {
    fVar38 = *(float *)(lVar19 + 0xd4) - *(float *)(lVar19 + 0xc0);
    if (0.0 < fVar38) {
      fVar30 = fVar30 - fVar39;
      fVar39 = fVar2 + fVar45;
      fVar38 = (fVar11 - fVar43) + fVar38;
      fVar45 = fVar29 + fVar31;
      fVar43 = fVar38;
      goto LAB_1423257dd;
    }
    fVar38 = (fVar11 - fVar43) - fVar38;
    fVar32 = fVar10 / SQRT(fVar38);
    fVar41 = fVar32 * fVar38;
    fVar43 = fVar32 * (fVar45 + fVar2);
    fVar45 = fVar32 * (fVar39 + fVar30);
    fVar38 = fVar32 * (fVar29 - fVar31);
  }
  else {
    fVar38 = *(float *)(lVar19 + 0xd4) + *(float *)(lVar19 + 0xc0);
    if (fVar38 <= 0.0) {
      fVar39 = fVar39 + fVar30;
      fVar38 = (fVar43 + fVar11) - fVar38;
      fVar30 = fVar45 - fVar2;
      fVar45 = fVar38;
      fVar43 = fVar29 + fVar31;
LAB_1423257dd:
      fVar38 = fVar10 / SQRT(fVar38);
      fVar41 = fVar38 * fVar39;
      fVar43 = fVar38 * fVar43;
      fVar45 = fVar38 * fVar45;
      fVar38 = fVar38 * fVar30;
    }
    else {
      fVar38 = fVar43 + fVar11 + fVar38;
      fVar41 = fVar10 / SQRT(fVar38);
      fVar41 = fVar41 * (fVar29 - fVar31);
      fVar43 = fVar41 * (fVar30 - fVar39);
      fVar45 = fVar41 * (fVar45 - fVar2);
      fVar38 = fVar41 * fVar38;
    }
  }
  uVar3 = *local_1e8;
  fVar39 = *(float *)((longlong)local_1e8 + 0x14);
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
  uVar12 = local_2d0;
  local_138 = local_2d8;
  std::alloc::__default_lib_allocator::__rust_alloc();
  local_168 = uVar12;
  if (extraout_RAX == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
    alloc::alloc::handle_alloc_error();
  }
  fVar47 = fVar47 * __real_3f4ccccd;
  fVar29 = fVar38 * ___xmm_b33bbd2e000000003f80000000000000;
  fVar31 = fVar38 * _UNK_142c29104;
  fVar33 = fVar38 * _UNK_142c29108;
  fVar38 = fVar38 * _UNK_142c2910c;
  fVar34 = fVar41 * ___xmm_000000003f80000000000000b33bbd2e *
           ___xmm_bf8000003f800000bf8000003f800000;
  fVar35 = fVar41 * _UNK_142c29114 * _UNK_1429cd174;
  fVar36 = fVar41 * _UNK_142c29118 * _UNK_1429cd178;
  fVar37 = fVar41 * _UNK_142c2911c * _UNK_1429cd17c;
  fVar30 = fVar43 * ___xmm_3f80000000000000b33bbd2e00000000 *
           ___xmm_bf800000bf8000003f8000003f800000;
  fVar32 = fVar43 * _UNK_142c29124 * _UNK_1429cd184;
  fVar41 = fVar43 * _UNK_142c29128 * _UNK_1429cd188;
  fVar43 = fVar43 * _UNK_142c2912c * _UNK_1429cd18c;
  fVar42 = fVar45 * ___xmm_00000000b33bbd2e000000003f800000 *
           ___xmm_bf8000003f8000003f800000bf800000;
  fVar44 = fVar45 * _UNK_142c29134 * _UNK_1429cd194;
  fVar46 = fVar45 * _UNK_142c29138 * _UNK_1429cd198;
  fVar45 = fVar45 * _UNK_142c2913c * _UNK_1429cd19c;
  *extraout_RAX = 1;
  extraout_RAX[1] = 1;
  *(float *)(extraout_RAX + 2) = fVar42 + fVar30 + fVar34 + fVar29;
  *(float *)((longlong)extraout_RAX + 0x14) = fVar44 + fVar32 + fVar35 + fVar31;
  *(float *)(extraout_RAX + 3) = fVar46 + fVar41 + fVar36 + fVar33;
  *(float *)((longlong)extraout_RAX + 0x1c) = fVar45 + fVar43 + fVar37 + fVar38;
  extraout_RAX[4] =
       CONCAT44((float)((ulonglong)uVar3 >> 0x20) + fVar1 * (float)((ulonglong)uVar4 >> 0x20),
                (float)uVar3 + fVar1 * (float)uVar4);
  *(float *)(extraout_RAX + 5) = fVar39 * fVar1 + fVar2;
  *(float *)((longlong)extraout_RAX + 0x2c) = fVar47;
  country_core::systems::ui::signifiers::SignifierFactory::new();
  local_368 = local_288;
  local_378 = local_298;
  uStack_370 = uStack_290;
  local_388 = local_2a8;
  pcStack_380 = (code *)uStack_2a0;
  country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write();
  uVar20 = local_108;
  lVar26 = local_188;
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
    lVar26 = *(longlong *)(local_190 + 0x30);
    uVar4 = *(undefined8 *)(local_190 + 0x40);
    if (lVar26 == *(longlong *)(local_190 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                ();
    }
    lVar19 = *(longlong *)(local_190 + 0x28);
    *(undefined8 *)(lVar19 + lVar26 * 0x10) = uVar4;
    *(undefined8 *)(lVar19 + 8 + lVar26 * 0x10) = uVar3;
    *(longlong *)(local_190 + 0x30) = lVar26 + 1;
    *(longlong *)(local_190 + 0x40) = *(longlong *)(local_190 + 0x40) + 1;
    uVar20 = local_108;
    lVar26 = local_188;
  }
  country_core::systems::ui::resources::slider3d::impl$1::into_drag_state();
  puVar25 = local_198;
  lVar19 = local_1b0;
  bVar13 = extraout_AL_04 - 2;
  if (2 < bVar13) {
    bVar13 = 3;
  }
  if (bVar13 == 1) {
    if (fVar10 < *(float *)(local_1b0 + 0x34)) {
      country_core::resources::audio_resources::AudioInstance::start();
      *local_1a0 = local_114;
      *(undefined4 *)(lVar19 + 0x34) = 0;
    }
  }
  else if (bVar13 == 2) {
    uVar15 = *local_170;
    uVar17 = uVar15 - *(ulonglong *)(local_1d0 + 0x18);
    if (uVar15 < *(ulonglong *)(local_1d0 + 0x18)) {
      uVar17 = 0;
    }
    uVar16 = uVar15 - *(ulonglong *)(local_1d0 + 0x38);
    if (uVar15 < *(ulonglong *)(local_1d0 + 0x38)) {
      uVar16 = 0;
    }
    uVar15 = *(ulonglong *)(local_1d0 + 0x30);
    lVar19 = uVar15 - uVar16;
    bVar27 = lVar19 != 0;
    if (uVar15 < uVar16) {
      lVar19 = 0;
    }
    uVar7 = *(ulonglong *)(local_1d0 + 0x10);
    lVar22 = uVar7 - uVar17;
    bVar28 = lVar22 != 0;
    if (uVar7 < uVar17) {
      lVar22 = 0;
    }
    uVar8 = *(ulonglong *)(local_1d0 + 0x40);
    *local_170 = uVar8 - (lVar22 + lVar19);
    if (uVar7 >= uVar17 && bVar28 || uVar15 >= uVar16 && bVar27) {
      *local_170 = uVar8;
      country_core::systems::camera_rig::MainCamera::mode();
      if (extraout_AL_05 == '\x03') {
        uStack_2a0 = CONCAT44(uStack_2a0._4_4_,pfVar21[6]);
        local_2a8 = *(undefined ***)(pfVar21 + 4);
        *local_1c0 = local_154;
        lVar19 = *(longlong *)(local_160 + 0x30);
        uVar3 = *(undefined8 *)(local_160 + 0x40);
        if (lVar19 == *(longlong *)(local_160 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                    ();
        }
        lVar22 = *(longlong *)(local_160 + 0x28);
        lVar18 = lVar19 * 0x20;
        *(undefined8 *)(lVar22 + lVar18) = uVar3;
        *(undefined ***)(lVar22 + 8 + lVar18) = local_2a8;
        *(float *)(lVar22 + 0x10 + lVar18) = (float)uStack_2a0;
        *(undefined4 *)(lVar22 + 0x14 + lVar18) = 0x3f800000;
        *(undefined1 *)(lVar22 + 0x18 + lVar18) = 4;
        *(longlong *)(local_160 + 0x30) = lVar19 + 1;
        *(longlong *)(local_160 + 0x40) = *(longlong *)(local_160 + 0x40) + 1;
        uVar20 = local_108;
      }
    }
    fVar43 = pfVar21[7];
    fVar47 = pfVar21[8];
    fVar45 = pfVar21[9];
    fVar39 = *pfVar21;
    fVar2 = pfVar21[1];
    fVar29 = pfVar21[2];
    fVar30 = pfVar21[3];
    fVar1 = pfVar21[4];
    uVar3 = *(undefined8 *)(pfVar21 + 5);
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_00 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error();
    }
    fVar41 = fVar29 + fVar29;
    fVar32 = fVar2 + fVar2;
    fVar31 = fVar30 * (fVar39 + fVar39);
    fVar38 = (fVar39 + fVar39) * fVar39;
    fVar33 = (fVar11 - (fVar32 * fVar2 + fVar29 * fVar41)) * fVar43 *
             __xmm_00000000000000000000000000000000;
    fVar34 = (fVar32 * fVar39 + fVar30 * fVar41) * fVar43 * DAT_1429259a4;
    fVar35 = (fVar41 * fVar39 - fVar30 * fVar32) * fVar43 * DAT_1429259a8;
    fVar43 = (fVar32 * fVar39 - fVar30 * fVar41) * fVar47 * ___xmm_00000000400ae148400ae148400ae148;
    fVar29 = (fVar11 - (fVar29 * fVar41 + fVar38)) * fVar47 * _UNK_14304bd54;
    fVar47 = (fVar31 + fVar41 * fVar2) * fVar47 * _UNK_14304bd58;
    fVar39 = (fVar30 * fVar32 + fVar41 * fVar39) * fVar45 * ___xmm_00000000bf933333bf933333bf933333;
    fVar30 = (fVar41 * fVar2 - fVar31) * fVar45 * _UNK_14304bd64;
    fVar45 = (fVar11 - (fVar38 + fVar32 * fVar2)) * fVar45 * _UNK_14304bd68;
    *extraout_RAX_00 = 1;
    extraout_RAX_00[1] = 1;
    extraout_RAX_00[2] =
         CONCAT44(fVar30 + fVar29 + fVar34 + (float)uVar3,fVar39 + fVar43 + fVar33 + fVar1);
    *(float *)(extraout_RAX_00 + 3) = fVar45 + fVar47 + fVar35 + (float)((ulonglong)uVar3 >> 0x20);
    country_core::systems::ui::signifiers::SignifierFactory::new();
    country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write();
    *local_1a8 = local_114;
    *(undefined8 *)(local_150 + 8) = 0x40e0000000000001;
  }
  goto LAB_1423251a2;
}

