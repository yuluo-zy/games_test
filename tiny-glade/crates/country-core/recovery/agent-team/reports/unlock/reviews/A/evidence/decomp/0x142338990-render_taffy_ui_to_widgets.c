// addr 0x142338990 sig void __cdecl render_taffy_ui_to_widgets(void) size 4424

void __cdecl toolbar::ui_interaction::render_taffy_ui_to_widgets(void)

{
  byte *pbVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  byte bVar4;
  byte bVar5;
  byte bVar6;
  undefined1 uVar7;
  undefined4 uVar8;
  undefined4 *puVar9;
  ulonglong uVar10;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  char extraout_AL_02;
  undefined1 extraout_AL_03;
  char extraout_AL_04;
  char extraout_AL_05;
  char extraout_AL_06;
  char extraout_AL_07;
  byte extraout_AL_08;
  char extraout_AL_09;
  char extraout_AL_10;
  undefined1 extraout_AL_11;
  uint uVar11;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *plVar12;
  ulonglong extraout_RAX_02;
  undefined8 *extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  undefined8 *in_RCX;
  ulonglong uVar13;
  longlong lVar14;
  longlong lVar15;
  longlong *plVar16;
  undefined8 *in_RDX;
  ulonglong uVar17;
  longlong lVar18;
  uint *puVar19;
  longlong *in_R8;
  char *pcVar20;
  longlong *in_R9;
  undefined1 *puVar21;
  undefined8 *puVar22;
  float fVar23;
  float fVar24;
  float fVar25;
  float fVar26;
  longlong *in_stack_00000028;
  undefined8 *in_stack_00000030;
  undefined8 *in_stack_00000038;
  undefined8 *in_stack_00000040;
  longlong *in_stack_00000048;
  longlong *in_stack_00000050;
  longlong *in_stack_00000058;
  longlong in_stack_00000060;
  undefined8 *in_stack_00000068;
  undefined8 local_428;
  undefined8 uStack_420;
  undefined8 uStack_418;
  undefined8 uStack_410;
  uint *local_408;
  ulonglong local_370;
  undefined8 *local_368;
  undefined8 local_310;
  undefined8 uStack_308;
  undefined8 local_300;
  undefined *local_2f8;
  undefined8 uStack_2f0;
  undefined8 *local_2e8;
  undefined8 uStack_2e0;
  uint *local_2d8;
  uint *local_2d0;
  longlong local_2c8;
  longlong local_2b8;
  longlong *local_2a8;
  longlong *local_2a0;
  longlong local_298;
  byte local_290;
  longlong local_288;
  longlong local_280;
  longlong local_278;
  longlong local_270;
  longlong local_260;
  undefined4 local_254;
  byte local_248;
  byte local_240;
  undefined8 local_238;
  undefined8 uStack_230;
  undefined8 local_228;
  float *local_208;
  undefined8 local_200;
  undefined4 *local_1f8;
  undefined4 *local_1f0;
  int *local_1e8;
  undefined4 *local_1e0;
  longlong local_1d8;
  longlong local_1d0;
  longlong local_1c8;
  undefined8 *local_1c0;
  undefined8 local_1b8;
  undefined4 *local_1b0;
  longlong local_1a8;
  longlong local_198;
  undefined4 *local_190;
  longlong local_188;
  longlong local_180;
  uint *local_178;
  longlong local_170;
  longlong local_168;
  undefined4 local_160;
  undefined4 local_15c;
  undefined4 local_158;
  undefined4 local_154;
  char *local_150;
  longlong local_148;
  longlong local_140;
  undefined8 *local_138;
  int *local_130;
  longlong local_128;
  longlong local_120;
  longlong *local_118;
  longlong *local_110;
  longlong local_108;
  undefined4 local_fc;
  longlong local_f8;
  longlong local_f0;
  longlong local_e8;
  uint local_dc;
  byte local_d6;
  char local_d5;
  uint local_d4;
  longlong *local_d0;
  undefined1 local_c2;
  undefined1 local_c1;
  undefined8 local_c0;
  
  local_c0 = 0xfffffffffffffffe;
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    local_1a8 = 0;
  }
  else {
    uVar17 = 0x33;
    local_d0 = in_R9;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar17 != 0) {
        if (uVar17 < 0x33) {
          if ((char)(&DAT_14304da07)[uVar17] < -0x40) {
LAB_142338a47:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar17 != 0x33) goto LAB_142338a47;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar17 != 0xfffffffffffffffe)) {
        if (uVar17 + 2 < 0x33) {
          if ((char)(&UNK_14304da09)[uVar17] < -0x40) {
LAB_142338aa5:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar17 + 2 != 0x33) goto LAB_142338aa5;
      }
    }
    pcVar20 = "toolbar_button_";
    do {
      bVar4 = pcVar20[-1];
      uVar11 = (uint)(char)bVar4;
      if ((char)bVar4 < '\0') {
        bVar5 = pcVar20[-2];
        if ((char)bVar5 < -0x40) {
          bVar6 = pcVar20[-3];
          if ((char)bVar6 < -0x40) {
            pbVar1 = (byte *)(pcVar20 + -4);
            pcVar20 = pcVar20 + -4;
            uVar11 = bVar6 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            pcVar20 = pcVar20 + -3;
            uVar11 = bVar6 & 0xf;
          }
          uVar11 = bVar5 & 0x3f | uVar11 << 6;
        }
        else {
          pcVar20 = pcVar20 + -2;
          uVar11 = bVar5 & 0x1f;
        }
        uVar11 = (uint)(bVar4 & 0x3f) | uVar11 << 6;
        if (uVar11 != 0x5c) goto LAB_142338b64;
LAB_142338b79:
        if (pcVar20 == "") break;
        if (pcVar20 + -0x14304dcd7 < (char *)0x2c) {
          if (-0x41 < pcVar20[1]) break;
        }
        else if (pcVar20 + -0x14304dcd7 == (char *)0x2c) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      pcVar20 = pcVar20 + -1;
      if (uVar11 == 0x5c) goto LAB_142338b79;
LAB_142338b64:
      if (uVar11 == 0x2f) goto LAB_142338b79;
    } while (pcVar20 != "crates/systems/toolbar/src/ui_interaction.rstoolbar_button_");
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar16 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar16 = extraout_RAX_06, extraout_RAX_06 == (longlong *)0x0)) {
LAB_142339ad1:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar16 != 0) {
LAB_142339add:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar16 = -1;
    local_110 = plVar16;
    puffin::ThreadProfiler::begin_scope();
    *local_110 = *local_110 + 1;
    local_1a8 = 1;
    in_R9 = local_d0;
  }
  local_138 = (undefined8 *)*in_R8;
  local_130 = (int *)(local_138 + 2);
  country_core::systems::ui::resources::widget_params::UiFocus::is_toolbar();
  if (extraout_AL_00 != '\0') {
    lVar18 = *in_stack_00000050;
    country_core::input::action_set::DigitalActionState::pressed();
    if ((extraout_AL_01 != '\0') ||
       (fVar23 = (float)*(undefined8 *)(lVar18 + 0x148),
       fVar26 = (float)((ulonglong)*(undefined8 *)(lVar18 + 0x148) >> 0x20),
       1.0 < SQRT(fVar26 * fVar26 + fVar23 * fVar23))) {
      uVar8 = *(undefined4 *)((longlong)in_R8 + 0x1c);
      puVar9 = (undefined4 *)in_R8[2];
      uVar2 = *(undefined8 *)(local_130 + 2);
      *local_138 = *(undefined8 *)local_130;
      local_138[1] = uVar2;
      *puVar9 = uVar8;
      local_138[2] = 0;
    }
  }
  country_core::render::sparse_instance_buffer::AllocationHandle::is_null();
  local_d0 = (longlong *)*in_RCX;
  local_140 = in_RCX[1];
  local_d5 = extraout_AL_02;
  bevy_ecs::query::iter::QueryIterationCursor::
  init<tuple_<bevy_ecs::entity::Entity,ref_<country_core::components::Ui2dExtent>,ref_<country_core::components::UiMouseInteractable>,ref_<toolbar::resouces_and_components::TaffyNode>,ref_mut_<country_core::components::UiInteractionState>,enum2_<core::option::Option<ref_<country_core::components::HistoryButton>_>_>,enum2_<core::option::Option<ref_<country_core::components::UiNav>_>_>_>,tuple_<>_>
            ();
  local_118 = *(longlong **)(in_stack_00000060 + 8);
  local_148 = *in_stack_00000058;
  if (local_118 == (longlong *)0x0) {
    local_118 = *(longlong **)(in_stack_00000060 + 0x10);
  }
  local_198 = *in_stack_00000048;
  local_208 = (float *)*in_stack_00000068;
  local_200 = *in_RDX;
  local_1e8 = (int *)*in_stack_00000030;
  local_1c0 = (undefined8 *)*in_stack_00000040;
  local_1b8 = *in_stack_00000038;
  local_1e0 = (undefined4 *)in_stack_00000058[2];
  local_154 = *(undefined4 *)((longlong)in_R8 + 0x1c);
  local_1b0 = (undefined4 *)in_R8[2];
  local_158 = *(undefined4 *)((longlong)in_stack_00000058 + 0x1c);
  local_160 = *(undefined4 *)((longlong)in_R9 + 0x1c);
  local_188 = *in_R9;
  local_1f8 = (undefined4 *)in_R9[2];
  local_15c = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
  local_180 = *in_stack_00000028;
  local_1f0 = (undefined4 *)in_stack_00000028[2];
  local_d6 = local_240;
  local_178 = local_2d0;
  local_170 = local_2c8;
  local_168 = local_2b8;
  local_108 = local_298;
  local_d4 = (uint)local_290;
  local_f8 = local_288;
  local_f0 = local_280;
  local_e8 = local_278;
  local_128 = local_270;
  local_120 = local_260;
  local_fc = local_254;
  local_dc = (uint)local_248;
  local_1c8 = local_148 + 0x20;
  local_1d8 = local_188 + 0x20;
  local_1d0 = local_180 + 0x20;
  plVar16 = local_2a8;
  puVar19 = local_2d8;
  do {
    if ((local_d6 & 1) == 0) {
      if (local_2a0 == plVar16) {
        if (puVar19 == local_178) goto LAB_142339a00;
        while( true ) {
          plVar16 = *(longlong **)
                     (*(longlong *)(local_140 + 0x100) + 0x58 + (ulonglong)*puVar19 * 0xa0);
          if (plVar16 != (longlong *)0x0) break;
          puVar19 = puVar19 + 1;
          if (puVar19 == local_178) goto LAB_142339a00;
        }
        lVar14 = *(longlong *)(local_140 + 0x100) + (ulonglong)*puVar19 * 0xa0;
        lVar18 = *(longlong *)(local_140 + 0x1a8) + (ulonglong)*(uint *)(lVar14 + 0x94) * 0x48;
        uVar17 = *(ulonglong *)
                  (*(longlong *)(local_140 + 0x1a8) + 0x40 +
                  (ulonglong)*(uint *)(lVar14 + 0x94) * 0x48);
        if (((ulonglong)local_d0[0x22] < uVar17) &&
           (uVar13 = *(ulonglong *)(*(longlong *)(lVar18 + 0x38) + local_d0[0x22] * 8), uVar13 != 0)
           ) {
          local_f8 = *(longlong *)(*(longlong *)(lVar18 + 0x18) + 0x10 + ~uVar13 * 0x30);
          uVar13 = local_d0[0x23];
          if (uVar17 <= uVar13) goto LAB_1423391e6;
LAB_1423391bc:
          uVar13 = *(ulonglong *)(*(longlong *)(lVar18 + 0x38) + uVar13 * 8);
          if (uVar13 == 0) goto LAB_1423391e6;
          local_f0 = *(longlong *)(*(longlong *)(lVar18 + 0x18) + 0x10 + ~uVar13 * 0x30);
        }
        else {
          local_f8 = 0;
          uVar13 = local_d0[0x23];
          if (uVar13 < uVar17) goto LAB_1423391bc;
LAB_1423391e6:
          local_f0 = 0;
        }
        lVar15 = *(longlong *)(lVar18 + 0x38);
        if ((ulonglong)local_d0[0x24] < uVar17) {
          uVar13 = *(ulonglong *)(lVar15 + local_d0[0x24] * 8);
          lVar18 = *(longlong *)(lVar18 + 0x18);
          if (uVar13 == 0) goto LAB_142339334;
          local_e8 = *(longlong *)(lVar18 + 0x10 + ~uVar13 * 0x30);
        }
        else {
          lVar18 = *(longlong *)(lVar18 + 0x18);
LAB_142339334:
          local_e8 = 0;
        }
        uVar13 = local_d0[0x26];
        uVar10 = *(ulonglong *)(lVar14 + 0x88);
        if ((uVar13 < uVar10) && (*(longlong *)(*(longlong *)(lVar14 + 0x80) + uVar13 * 8) != 0)) {
          local_d4 = (uint)CONCAT71((int7)((ulonglong)*(longlong *)(lVar14 + 0x80) >> 8),1);
          if ((uVar17 <= uVar13) || (uVar17 = *(ulonglong *)(lVar15 + uVar13 * 8), uVar17 == 0)) {
            local_108 = 0;
            uVar17 = local_d0[0x27];
            goto joined_r0x0001423393c6;
          }
          local_108 = *(longlong *)(lVar18 + 0x10 + ~uVar17 * 0x30);
          uVar17 = local_d0[0x27];
          if (uVar10 <= uVar17) goto LAB_1423393ff;
LAB_1423393c8:
          local_dc = (uint)CONCAT71((int7)(uVar17 >> 8),
                                    *(longlong *)(*(longlong *)(lVar14 + 0x80) + uVar17 * 8) != 0);
        }
        else {
          local_d4 = 0;
          uVar17 = local_d0[0x27];
joined_r0x0001423393c6:
          if (uVar17 < uVar10) goto LAB_1423393c8;
LAB_1423393ff:
          local_dc = 0;
        }
        puVar19 = puVar19 + 1;
        lVar15 = ~*(ulonglong *)(lVar15 + local_d0[0x25] * 8) * 0x30;
        local_128 = *(longlong *)(lVar18 + 0x10 + lVar15);
        local_120 = *(longlong *)(lVar18 + 0x28 + lVar15);
        local_168 = *(longlong *)(lVar14 + 0x50);
        local_2a0 = (longlong *)0x0;
      }
      plVar12 = (longlong *)(local_168 + (longlong)local_2a0 * 0x10);
      uVar17 = (ulonglong)*(uint *)(local_168 + 8 + (longlong)local_2a0 * 0x10);
      local_110 = local_2a0;
    }
    else {
      if (local_2a0 == plVar16) {
        if (puVar19 == local_178) {
LAB_142339a00:
          if (local_1a8 != 0) {
            puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
            if (*extraout_RAX_04 == 1) {
              plVar16 = extraout_RAX_04 + 1;
            }
            else if ((*extraout_RAX_04 != 0) ||
                    (std::sys::thread_local::native::lazy::Storage::
                     initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                               (), plVar16 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0))
            goto LAB_142339ad1;
            if (*plVar16 != 0) goto LAB_142339add;
            *plVar16 = -1;
            local_d0 = plVar16;
            puffin::ThreadProfiler::end_scope();
            *local_d0 = *local_d0 + 1;
          }
          return;
        }
        while( true ) {
          plVar16 = *(longlong **)
                     (*(longlong *)(local_140 + 0x1a8) + 0x10 + (ulonglong)*puVar19 * 0x48);
          if (plVar16 != (longlong *)0x0) break;
          puVar19 = puVar19 + 1;
          if (puVar19 == local_178) goto LAB_142339a00;
        }
        lVar18 = *(longlong *)(local_140 + 0x1a8) + (ulonglong)*puVar19 * 0x48;
        uVar17 = *(ulonglong *)(lVar18 + 0x40);
        if (((ulonglong)local_d0[0x22] < uVar17) &&
           (uVar13 = *(ulonglong *)(*(longlong *)(lVar18 + 0x38) + local_d0[0x22] * 8), uVar13 != 0)
           ) {
          local_f8 = *(longlong *)(*(longlong *)(lVar18 + 0x18) + 0x10 + ~uVar13 * 0x30);
          uVar13 = local_d0[0x23];
          if (uVar17 <= uVar13) goto LAB_142339134;
LAB_14233910a:
          uVar13 = *(ulonglong *)(*(longlong *)(lVar18 + 0x38) + uVar13 * 8);
          if (uVar13 == 0) goto LAB_142339134;
          local_f0 = *(longlong *)(*(longlong *)(lVar18 + 0x18) + 0x10 + ~uVar13 * 0x30);
        }
        else {
          local_f8 = 0;
          uVar13 = local_d0[0x23];
          if (uVar13 < uVar17) goto LAB_14233910a;
LAB_142339134:
          local_f0 = 0;
        }
        lVar14 = *(longlong *)(lVar18 + 0x38);
        if ((ulonglong)local_d0[0x24] < uVar17) {
          uVar13 = *(ulonglong *)(lVar14 + local_d0[0x24] * 8);
          lVar15 = *(longlong *)(lVar18 + 0x18);
          if (uVar13 == 0) goto LAB_14233923d;
          local_e8 = *(longlong *)(lVar15 + 0x10 + ~uVar13 * 0x30);
          uVar13 = local_d0[0x26];
          if (uVar13 < uVar17) goto LAB_14233925b;
LAB_1423392a4:
          local_d4 = 0;
          uVar13 = local_d0[0x27];
          if (uVar13 < uVar17) goto LAB_1423392c1;
LAB_142339298:
          local_dc = 0;
        }
        else {
          lVar15 = *(longlong *)(lVar18 + 0x18);
LAB_14233923d:
          local_e8 = 0;
          uVar13 = local_d0[0x26];
          if (uVar17 <= uVar13) goto LAB_1423392a4;
LAB_14233925b:
          uVar13 = *(ulonglong *)(lVar14 + uVar13 * 8);
          if (uVar13 == 0) goto LAB_1423392a4;
          local_108 = *(longlong *)(lVar15 + 0x10 + ~uVar13 * 0x30);
          local_d4 = (uint)CONCAT71((int7)((ulonglong)local_108 >> 8),1);
          uVar13 = local_d0[0x27];
          if (uVar17 <= uVar13) goto LAB_142339298;
LAB_1423392c1:
          local_dc = (uint)CONCAT71((int7)(uVar17 >> 8),*(longlong *)(lVar14 + uVar13 * 8) != 0);
        }
        puVar19 = puVar19 + 1;
        lVar14 = ~*(ulonglong *)(lVar14 + local_d0[0x25] * 8) * 0x30;
        local_128 = *(longlong *)(lVar15 + 0x10 + lVar14);
        local_120 = *(longlong *)(lVar15 + 0x28 + lVar14);
        local_170 = *(longlong *)(lVar18 + 8);
        local_2a0 = (longlong *)0x0;
      }
      plVar12 = (longlong *)(local_170 + (longlong)local_2a0 * 8);
      uVar17 = (ulonglong)local_2a0 & 0xffffffff;
      local_110 = local_2a0;
    }
    puVar22 = (undefined8 *)(uVar17 * 0x10 + local_f8);
    puVar9 = (undefined4 *)(local_f0 + uVar17 * 4);
    local_428 = (undefined *)(local_e8 + uVar17 * 0x14);
    local_150 = (char *)(uVar17 * 3 + local_128);
    local_190 = (undefined4 *)(local_120 + uVar17 * 4);
    puVar21 = (undefined1 *)(uVar17 + local_108);
    if ((local_d4 & 1) == 0) {
      puVar21 = (undefined1 *)0x0;
    }
    lVar18 = *plVar12;
    *local_190 = local_fc;
    local_150[1] = '\0';
    if ((*local_130 == 1) && (local_138[3] == lVar18)) {
      *local_190 = local_fc;
      local_150[1] = '\x01';
    }
    uVar2 = puVar22[1];
    fVar23 = *(float *)(local_198 + 0xe8);
    fVar26 = *local_208;
    uStack_420 = slotmap::impl$23::fmt;
    local_2f8 = &DAT_14304dd18;
    uStack_2f0 = 1;
    local_2e8 = &local_428;
    uStack_2e0 = 1;
    local_2d8 = (uint *)0x0;
    alloc::fmt::format::format_inner();
    fVar26 = fVar23 * 10.0 * fVar26;
    local_238 = local_310;
    uStack_230 = uStack_308;
    local_228 = local_300;
    uVar3 = *puVar22;
    fVar23 = (float)uVar3;
    fVar24 = (float)((ulonglong)uVar3 >> 0x20);
    fVar25 = ((float)uVar2 + fVar26) * 0.5;
    fVar26 = ((float)((ulonglong)uVar2 >> 0x20) + fVar26) * 0.5;
    uStack_420 = (code *)CONCAT44(fVar24 - fVar26,fVar23 - fVar25);
    uStack_418 = (undefined8 *)CONCAT44(fVar26 + fVar24,fVar25 + fVar23);
    local_428 = (undefined *)CONCAT44(*puVar9,(undefined4)local_428);
    local_428 = (undefined *)CONCAT71(local_428._1_7_,3);
    local_370 = local_370 & 0xffffffffffffff00;
    local_c1 = 1;
    country_core::systems::ui::resources::widget_params::WidgetParams::new();
    local_c1 = 1;
    country_core::systems::ui::resources::widget_params::WidgetParams::no_history_recording();
    local_c1 = 1;
    country_core::systems::ui::resources::widget_params::WidgetParams::occlude_by_world();
    local_c1 = 1;
    country_core::systems::ui::resources::widget_params::WidgetParams::allow_negative_distance();
    local_c2 = 1;
    country_core::systems::ui::resources::widget_params::UiSounds::empty();
    local_c2 = 0;
    country_core::systems::ui::resources::widget_params::WidgetParams::sounds();
    local_c1 = 0;
    puVar22 = &local_238;
    country_core::systems::ui::resources::ui_state::UiState::add<alloc::string::String>();
    local_370 = extraout_RAX_02;
    local_368 = puVar22;
    if (puVar21 == (undefined1 *)0x0) {
      if (*local_1e8 != 0xd) goto LAB_142339890;
      country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active();
      if (extraout_AL_06 != '\0') {
        uVar2 = *local_1c0;
        fVar23 = *(float *)(local_198 + 0xe8);
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (extraout_RAX_03 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
          alloc::alloc::handle_alloc_error();
        }
        *extraout_RAX_03 = 1;
        extraout_RAX_03[1] = 1;
        extraout_RAX_03[2] = uVar2;
        *(float *)(extraout_RAX_03 + 3) = fVar23 * 60.0;
        country_core::systems::ui::signifiers::SignifierFactory::new();
        local_408 = local_2d8;
        uStack_418 = local_2e8;
        uStack_410 = uStack_2e0;
        local_428 = local_2f8;
        uStack_420 = (code *)uStack_2f0;
        country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write();
      }
    }
    else {
      country_core::systems::ui::resources::slider3d::impl$1::into_drag_state();
      local_2f8 = (undefined *)CONCAT71(local_2f8._1_7_,extraout_AL_03);
      country_core::systems::ui::resources::ui_state::DragState::is_some();
      if (extraout_AL_04 != '\0') {
        uVar7 = *puVar21;
        *local_1e0 = local_158;
        lVar14 = *(longlong *)(local_148 + 0x30);
        uVar2 = *(undefined8 *)(local_148 + 0x40);
        if (lVar14 == *(longlong *)(local_148 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                    ();
        }
        lVar15 = *(longlong *)(local_148 + 0x28);
        *(undefined8 *)(lVar15 + lVar14 * 0x10) = uVar2;
        *(undefined1 *)(lVar15 + 8 + lVar14 * 0x10) = uVar7;
        *(longlong *)(local_148 + 0x30) = lVar14 + 1;
        *(longlong *)(local_148 + 0x40) = *(longlong *)(local_148 + 0x40) + 1;
      }
      country_core::systems::ui::resources::slider3d::impl$1::into_drag_state();
      if (extraout_AL_05 == '\x03') {
        lVar14 = local_118[2];
        if ((ulonglong)(*local_118 - lVar14) < 0x28) {
          alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        }
        lVar15 = local_118[1];
        *(code **)(lVar15 + lVar14) =
             core::ops::function::FnOnce::
             call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<toolbar::resouces_and_components::UiPlaySelectAnimation>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
        ;
        *(code **)(lVar15 + 8 + lVar14) = bevy_ecs::error::handler::panic;
        *(longlong *)(lVar15 + 0x10 + lVar14) = lVar18;
        *(undefined8 *)(lVar15 + 0x18 + lVar14) = 0x3e4ccccd3e4ccccd;
        *(undefined4 *)(lVar15 + 0x20 + lVar14) = 0;
        *(undefined1 *)(lVar15 + 0x24 + lVar14) = 0;
        local_118[2] = lVar14 + 0x28;
      }
LAB_142339890:
      if (((local_d5 != '\0') && (*local_150 == '\x01')) &&
         (country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active(),
         extraout_AL_07 != '\0')) {
        *local_190 = local_fc;
        local_150[1] = '\x01';
        country_core::render::sparse_instance_buffer::AllocationHandle::is_null();
        if ((extraout_AL_08 & (byte)local_dc) == 1) {
          *local_1b0 = local_154;
          local_138[2] = 1;
          local_138[3] = lVar18;
        }
      }
      country_core::systems::ui::resources::ui_state::UiWidgetContext::clicked();
      if (extraout_AL_09 != '\0') {
        *local_1f8 = local_160;
        lVar14 = *(longlong *)(local_188 + 0x30);
        uVar2 = *(undefined8 *)(local_188 + 0x40);
        if (lVar14 == *(longlong *)(local_188 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                    ();
        }
        lVar15 = *(longlong *)(local_188 + 0x28);
        *(undefined8 *)(lVar15 + lVar14 * 0x10) = uVar2;
        *(longlong *)(lVar15 + 8 + lVar14 * 0x10) = lVar18;
        *(longlong *)(local_188 + 0x30) = lVar14 + 1;
        *(longlong *)(local_188 + 0x40) = *(longlong *)(local_188 + 0x40) + 1;
      }
      country_core::systems::ui::resources::slider3d::impl$1::into_drag_state();
      if (extraout_AL_10 != '\x02') {
        country_core::systems::ui::resources::slider3d::impl$1::into_drag_state();
        *local_1f0 = local_15c;
        lVar14 = *(longlong *)(local_180 + 0x30);
        uVar2 = *(undefined8 *)(local_180 + 0x40);
        if (lVar14 == *(longlong *)(local_180 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                    ();
        }
        lVar15 = *(longlong *)(local_180 + 0x28);
        *(undefined8 *)(lVar15 + lVar14 * 0x18) = uVar2;
        *(longlong *)(lVar15 + 8 + lVar14 * 0x18) = lVar18;
        *(undefined1 *)(lVar15 + 0x10 + lVar14 * 0x18) = extraout_AL_11;
        *(longlong *)(local_180 + 0x30) = lVar14 + 1;
        *(longlong *)(local_180 + 0x40) = *(longlong *)(local_180 + 0x40) + 1;
      }
    }
    local_2a0 = (longlong *)((longlong)local_110 + 1);
  } while( true );
}


// CALLEES: [{"address": "140c18130", "name": "VAL"}, {"address": "140613c10", "name": "__rust_alloc"}, {"address": "1409231f0", "name": "add<alloc::string::String>"}, {"address": "1409a8f40", "name": "allow_negative_distance"}, {"address": "140c16a00", "name": "are_scopes_on"}, {"address": "140c16d40", "name": "begin_scope"}, {"address": "14092e5f0", "name": "clicked"}, {"address": "1428a58d0", "name": "do_reserve_and_handle<alloc::alloc::Global>"}, {"address": "1409a8fb0", "name": "empty"}, {"address": "140c170f0", "name": "end_scope"}, {"address": "140d79f90", "name": "format_inner"}, {"address": "140562630", "name": "grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>"}, {"address": "140562db0", "name": "grow_one<tuple$<ref$<str$>,enum2$<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>"}, {"address": "1428d9000", "name": "handle_alloc_error"}, {"address": "142623320", "name": "init<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::UiMouseInteractable>,ref$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<country_core::components::HistoryButton>_>_>,enum2$<core::option::Option<ref$<country_core::components::UiNav>_>_>_>,tuple$<>_>"}, {"address": "1428e6250", "name": "initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>"}, {"address": "14092df10", "name": "into_drag_state"}, {"address": "14092e680", "name": "is_active"}, {"address": "140985f30", "name": "is_null"}, {"address": "14092e6b0", "name": "is_some"}, {"address": "1409a9150", "name": "is_toolbar"}, {"address": "1409a8e10", "name": "new"}, {"address": "14089d8b0", "name": "new"}, {"address": "1409a8f10", "name": "no_history_recording"}, {"address": "1409a8ee0", "name": "occlude_by_world"}, {"address": "1428d80d0", "name": "panic_access_error"}, {"address": "1428d9250", "name": "panic_already_borrowed"}, {"address": "14093fa70", "name": "pressed"}, {"address": "142338130", "name": "rfind<ref$<str$>_>"}, {"address": "1428d9940", "name": "slice_error_fail"}, {"address": "1409a8e80", "name": "sounds"}, {"address": "140b2cf90", "name": "write"}]