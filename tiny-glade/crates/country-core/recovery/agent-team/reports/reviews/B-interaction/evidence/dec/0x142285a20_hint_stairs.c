
void __cdecl system_onboarding::hint_stairs::hint_stairs(void)

{
  undefined8 *puVar1;
  float fVar2;
  undefined4 uVar3;
  ulonglong *puVar4;
  ulonglong uVar5;
  ulonglong *puVar6;
  undefined4 *puVar7;
  int *piVar8;
  ulonglong uVar9;
  undefined8 uVar10;
  undefined2 uVar11;
  undefined2 uVar12;
  undefined2 extraout_AX;
  undefined2 extraout_AX_00;
  undefined2 extraout_AX_01;
  undefined2 extraout_AX_02;
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  undefined8 *in_RCX;
  longlong lVar13;
  int *piVar14;
  longlong *in_RDX;
  ulonglong uVar15;
  ulonglong uVar16;
  ulonglong uVar17;
  longlong lVar18;
  int *piVar19;
  undefined8 *in_R8;
  ulonglong uVar20;
  int *piVar21;
  longlong *in_R9;
  int *piVar22;
  int *piVar23;
  ulonglong uVar24;
  longlong lVar25;
  longlong lVar26;
  longlong lVar27;
  bool bVar28;
  bool bVar29;
  bool bVar30;
  bool bVar31;
  longlong *in_stack_00000028;
  undefined8 *in_stack_00000030;
  undefined8 *in_stack_00000038;
  undefined8 *in_stack_00000040;
  undefined8 *in_stack_00000048;
  undefined8 *in_stack_00000050;
  undefined8 *in_stack_00000058;
  undefined8 *in_stack_00000060;
  undefined8 *in_stack_00000068;
  longlong *in_stack_00000070;
  undefined4 local_138;
  undefined2 local_134;
  undefined6 uStack_132;
  undefined2 uStack_12c;
  undefined2 uStack_12a;
  undefined6 uStack_128;
  undefined2 uStack_122;
  undefined6 uStack_120;
  undefined2 uStack_11a;
  undefined6 uStack_118;
  undefined2 uStack_112;
  undefined8 uStack_110;
  undefined8 local_108;
  undefined8 uStack_100;
  undefined8 local_f8;
  undefined4 local_ee;
  undefined2 local_ea;
  undefined2 uStack_d4;
  undefined2 uStack_d2;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  undefined8 local_c8;
  undefined4 uStack_c0;
  undefined6 local_b8;
  undefined6 uStack_a0;
  undefined2 uStack_90;
  undefined4 uStack_8e;
  undefined2 uStack_8a;
  undefined8 local_88;
  undefined2 uStack_5e;
  undefined4 uStack_5c;
  
  uVar12 = uStack_d2;
  puVar4 = (ulonglong *)*in_stack_00000030;
  lVar18 = in_stack_00000030[1];
  uVar5 = *puVar4;
  uVar20 = 0;
  uVar16 = uVar5 - *(ulonglong *)(lVar18 + 0x18);
  if (uVar5 < *(ulonglong *)(lVar18 + 0x18)) {
    uVar16 = uVar20;
  }
  uVar15 = uVar5 - *(ulonglong *)(lVar18 + 0x38);
  if (uVar5 < *(ulonglong *)(lVar18 + 0x38)) {
    uVar15 = uVar20;
  }
  lVar25 = uVar16 * 8 + *(longlong *)(lVar18 + 8);
  uVar5 = *(ulonglong *)(lVar18 + 0x10) - uVar16;
  if (*(ulonglong *)(lVar18 + 0x10) < uVar16) {
    lVar25 = 8;
    uVar5 = uVar20;
  }
  lVar27 = uVar15 * 8 + *(longlong *)(lVar18 + 0x28);
  uVar16 = *(ulonglong *)(lVar18 + 0x30) - uVar15;
  if (*(ulonglong *)(lVar18 + 0x30) < uVar15) {
    lVar27 = 8;
    uVar16 = uVar20;
  }
  *puVar4 = (*(longlong *)(lVar18 + 0x40) - uVar16) - uVar5;
  lVar18 = lVar25 + uVar5 * 8;
  lVar26 = lVar27 + uVar16 * 8;
  uVar3 = *(undefined4 *)((longlong)in_R8 + 0x1c);
  puVar6 = (ulonglong *)*in_R8;
  puVar7 = (undefined4 *)in_R8[2];
  do {
    if ((lVar25 == 0) || (lVar25 == lVar18)) {
      if ((lVar27 == 0) || (lVar27 == lVar26)) break;
      lVar27 = lVar27 + 8;
      lVar25 = 0;
    }
    else {
      lVar25 = lVar25 + 8;
    }
    *puVar4 = *puVar4 + 1;
    *puVar7 = uVar3;
    *puVar6 = *puVar6 + 1;
  } while( true );
  piVar8 = (int *)*in_RCX;
  if ((*piVar8 == 0xb) && ((char)piVar8[1] == '?')) {
    fVar2 = *(float *)(*in_stack_00000070 + 0x28);
    *puVar7 = uVar3;
    *(float *)(puVar6 + 1) = fVar2 + *(float *)(puVar6 + 1);
  }
  lVar18 = *in_RDX;
  uStack_d2 = (undefined2)uStack_132;
  uVar11 = uStack_d2;
  uStack_5e = (undefined2)uStack_118;
  uStack_5c = (undefined4)((uint6)uStack_118 >> 0x10);
  if (*(char *)(lVar18 + 0x205) == '\0') {
    puVar4 = (ulonglong *)*in_stack_00000050;
    lVar25 = in_stack_00000050[1];
    uVar5 = *puVar4;
    uVar20 = 0;
    uVar16 = uVar5 - *(ulonglong *)(lVar25 + 0x18);
    if (uVar5 < *(ulonglong *)(lVar25 + 0x18)) {
      uVar16 = uVar20;
    }
    uVar15 = uVar5 - *(ulonglong *)(lVar25 + 0x38);
    if (uVar5 < *(ulonglong *)(lVar25 + 0x38)) {
      uVar15 = uVar20;
    }
    uVar5 = *(ulonglong *)(lVar25 + 0x30);
    uVar24 = uVar5 - uVar15;
    if (uVar5 < uVar15) {
      uVar24 = uVar20;
    }
    uVar9 = *(ulonglong *)(lVar25 + 0x10);
    uVar17 = uVar9 - uVar16;
    if (uVar9 < uVar16) {
      uVar17 = uVar20;
    }
    uVar20 = *(ulonglong *)(lVar25 + 0x40);
    *puVar4 = uVar20 - (uVar17 + uVar24);
    if ((uVar9 < uVar16 || uVar9 - uVar16 == 0) && (uVar5 < uVar15 || uVar5 - uVar15 == 0)) {
      puVar4 = (ulonglong *)*in_stack_00000058;
      lVar25 = in_stack_00000058[1];
      uVar5 = *puVar4;
      uVar20 = 0;
      uVar16 = uVar5 - *(ulonglong *)(lVar25 + 0x18);
      if (uVar5 < *(ulonglong *)(lVar25 + 0x18)) {
        uVar16 = uVar20;
      }
      uVar15 = uVar5 - *(ulonglong *)(lVar25 + 0x38);
      if (uVar5 < *(ulonglong *)(lVar25 + 0x38)) {
        uVar15 = uVar20;
      }
      uVar5 = *(ulonglong *)(lVar25 + 0x30);
      uVar24 = uVar5 - uVar15;
      if (uVar5 < uVar15) {
        uVar24 = uVar20;
      }
      uVar9 = *(ulonglong *)(lVar25 + 0x10);
      uVar17 = uVar9 - uVar16;
      if (uVar9 < uVar16) {
        uVar17 = uVar20;
      }
      uVar20 = *(ulonglong *)(lVar25 + 0x40);
      *puVar4 = uVar20 - (uVar17 + uVar24);
      if (uVar9 >= uVar16 && uVar9 - uVar16 != 0 || uVar5 >= uVar15 && uVar5 - uVar15 != 0) {
        *puVar4 = uVar20;
        country_core::resources::steam::impl$31::from();
        hint_orchestrator::BindingHintDisplay::
        alternate<enum2_<core::option::Option<enum2_<bevy_input::keyboard::KeyCode>_>_>_>();
        local_138 = 6;
        *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
        lVar25 = *in_stack_00000028;
        uVar10 = *(undefined8 *)(lVar25 + 0x40);
        uStack_8e = local_ee;
        uStack_8a = local_ea;
        uStack_cc = CONCAT22(uStack_12a,uStack_12c);
        local_c8 = CONCAT26(uStack_122,uStack_128);
        uStack_d0 = (undefined4)((uint6)uStack_132 >> 0x10);
        lVar27 = *(longlong *)(lVar25 + 0x30);
        if (lVar27 == *(longlong *)(lVar25 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<gpu_profiler::backend::ash::ProfilerOutputBuffer<rhapsody::raw::profiler::ProfilerBuffer>,alloc::alloc::Global>
                    ();
        }
        lVar26 = *(longlong *)(lVar25 + 0x28);
        lVar13 = lVar27 * 0x58;
        *(undefined8 *)(lVar26 + 0x50 + lVar13) = uVar10;
        puVar1 = (undefined8 *)(lVar26 + 0x40 + lVar13);
        *puVar1 = local_f8;
        puVar1[1] = CONCAT26(local_ea,CONCAT42(local_ee,0x1100));
        puVar1 = (undefined8 *)(lVar26 + 0x30 + lVar13);
        *puVar1 = local_108;
        puVar1[1] = uStack_100;
        puVar1 = (undefined8 *)(lVar26 + 0x20 + lVar13);
        *puVar1 = CONCAT26(uStack_112,uStack_118);
        puVar1[1] = uStack_110;
        puVar1 = (undefined8 *)(lVar26 + 0x10 + lVar13);
        *puVar1 = local_c8;
        puVar1[1] = CONCAT26(uStack_11a,uStack_120);
        *(undefined8 *)(lVar26 + lVar13) = CONCAT26(uStack_d2,CONCAT24(local_134,6));
        ((undefined8 *)(lVar26 + lVar13))[1] = CONCAT44(uStack_cc,uStack_d0);
        *(longlong *)(lVar25 + 0x30) = lVar27 + 1;
        *(longlong *)(lVar25 + 0x40) = *(longlong *)(lVar25 + 0x40) + 1;
        uVar12 = uStack_d2;
      }
      goto LAB_142285d0a;
    }
    *puVar4 = uVar20;
    *(undefined4 *)in_R9[2] = *(undefined4 *)((longlong)in_R9 + 0x1c);
    lVar25 = *in_R9;
    lVar27 = *(longlong *)(lVar25 + 0x30);
    uVar10 = *(undefined8 *)(lVar25 + 0x40);
    if (lVar27 == *(longlong *)(lVar25 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                ();
    }
    lVar26 = *(longlong *)(lVar25 + 0x28);
    *(undefined8 *)(lVar26 + lVar27 * 0x10) = uVar10;
    *(undefined1 *)(lVar26 + 8 + lVar27 * 0x10) = 0x11;
    *(longlong *)(lVar25 + 0x30) = lVar27 + 1;
    *(longlong *)(lVar25 + 0x40) = *(longlong *)(lVar25 + 0x40) + 1;
    uStack_d2 = uVar12;
    if (*(char *)(lVar18 + 0x201) == '\0') goto LAB_142286018;
LAB_142285d18:
    if (*(char *)(lVar18 + 0x202) == '\0') {
      lVar25 = in_stack_00000068[1];
      uVar5 = *(ulonglong *)*in_stack_00000068;
      piVar21 = (int *)0x0;
      piVar19 = (int *)(uVar5 - *(ulonglong *)(lVar25 + 0x18));
      if (uVar5 < *(ulonglong *)(lVar25 + 0x18)) {
        piVar19 = piVar21;
      }
      piVar22 = (int *)(uVar5 - *(ulonglong *)(lVar25 + 0x38));
      if (uVar5 < *(ulonglong *)(lVar25 + 0x38)) {
        piVar22 = piVar21;
      }
      lVar27 = (longlong)piVar19 * 0x28 + *(longlong *)(lVar25 + 8);
      lVar26 = (longlong)piVar22 * 0x28 + *(longlong *)(lVar25 + 0x28);
      bVar28 = *(int **)(lVar25 + 0x10) < piVar19;
      piVar19 = (int *)((longlong)*(int **)(lVar25 + 0x10) - (longlong)piVar19);
      bVar30 = piVar19 == (int *)0x0;
      if (bVar28) {
        piVar19 = piVar21;
      }
      if (bVar28) {
        lVar27 = 8;
      }
      piVar23 = (int *)(lVar27 + -0x28 + (longlong)piVar19 * 0x28);
      if (bVar28 || bVar30) {
        piVar23 = piVar21;
      }
      bVar29 = *(int **)(lVar25 + 0x30) < piVar22;
      piVar22 = (int *)((longlong)*(int **)(lVar25 + 0x30) - (longlong)piVar22);
      bVar31 = piVar22 == (int *)0x0;
      if (bVar29) {
        lVar26 = 8;
        piVar22 = piVar21;
      }
      piVar14 = (int *)(lVar26 + -0x28 + (longlong)piVar22 * 0x28);
      if (bVar29 || bVar31) {
        piVar14 = piVar23;
      }
      piVar19 = (int *)((longlong)piVar22 + (longlong)piVar19);
      if (!bVar28 && !bVar30 || !bVar29 && !bVar31) {
        piVar19 = piVar21;
      }
      *(ulonglong *)*in_stack_00000068 = *(longlong *)(lVar25 + 0x40) - (longlong)piVar19;
      if ((!bVar28 && !bVar30 || !bVar29 && !bVar31) && (*puVar6 != 0)) {
        puVar4 = (ulonglong *)*in_stack_00000060;
        lVar25 = in_stack_00000060[1];
        uVar5 = *puVar4;
        uVar20 = 0;
        uVar16 = uVar5 - *(ulonglong *)(lVar25 + 0x18);
        if (uVar5 < *(ulonglong *)(lVar25 + 0x18)) {
          uVar16 = uVar20;
        }
        uVar15 = uVar5 - *(ulonglong *)(lVar25 + 0x38);
        if (uVar5 < *(ulonglong *)(lVar25 + 0x38)) {
          uVar15 = uVar20;
        }
        uVar5 = *(ulonglong *)(lVar25 + 0x30);
        uVar24 = uVar5 - uVar15;
        if (uVar5 < uVar15) {
          uVar24 = uVar20;
        }
        uVar9 = *(ulonglong *)(lVar25 + 0x10);
        uVar17 = uVar9 - uVar16;
        if (uVar9 < uVar16) {
          uVar17 = uVar20;
        }
        uVar20 = *(ulonglong *)(lVar25 + 0x40);
        *puVar4 = uVar20 - (uVar17 + uVar24);
        if ((uVar9 >= uVar16 && uVar9 - uVar16 != 0 || uVar5 >= uVar15 && uVar5 - uVar15 != 0) &&
           (*puVar4 = uVar20, *piVar14 == 0)) {
          *(undefined4 *)in_R9[2] = *(undefined4 *)((longlong)in_R9 + 0x1c);
          lVar25 = *in_R9;
          lVar27 = *(longlong *)(lVar25 + 0x30);
          uVar10 = *(undefined8 *)(lVar25 + 0x40);
          if (lVar27 == *(longlong *)(lVar25 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                      ();
          }
          lVar26 = *(longlong *)(lVar25 + 0x28);
          *(undefined8 *)(lVar26 + lVar27 * 0x10) = uVar10;
          *(undefined1 *)(lVar26 + 8 + lVar27 * 0x10) = 0xe;
          *(longlong *)(lVar25 + 0x30) = lVar27 + 1;
          *(longlong *)(lVar25 + 0x40) = *(longlong *)(lVar25 + 0x40) + 1;
        }
      }
      if (((*piVar8 == 0xb) && ((char)piVar8[1] == '?')) && (2 < *puVar6)) {
        country_core::resources::steam::impl$31::from();
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
          alloc::alloc::handle_alloc_error();
        }
        *extraout_RAX = &DAT_143038ed8;
        extraout_RAX[1] = 10;
        extraout_RAX[2] = "hint/rmb";
        extraout_RAX[3] = 8;
        hint_orchestrator::BindingHintDisplay::
        alternate<system_onboarding::hint_orchestrator::HintGraphicParams>();
        *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
        lVar25 = *in_stack_00000028;
        local_88 = *(undefined8 *)(lVar25 + 0x40);
        uStack_c0 = local_138;
        uStack_a0 = (undefined6)(CONCAT44(uStack_5c,CONCAT22(uStack_5e,uStack_11a)) >> 0x10);
        local_b8 = (undefined6)(CONCAT26(uStack_12c,uStack_132) >> 0x10);
        uStack_90 = 0xe00;
        lVar27 = *(longlong *)(lVar25 + 0x30);
        uStack_d4 = extraout_AX;
        if (lVar27 == *(longlong *)(lVar25 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<gpu_profiler::backend::ash::ProfilerOutputBuffer<rhapsody::raw::profiler::ProfilerBuffer>,alloc::alloc::Global>
                    ();
        }
LAB_142286199:
        lVar26 = *(longlong *)(lVar25 + 0x28);
        lVar13 = lVar27 * 0x58;
        *(undefined8 *)(lVar26 + 0x50 + lVar13) = local_88;
        puVar1 = (undefined8 *)(lVar26 + 0x40 + lVar13);
        *puVar1 = uStack_110;
        puVar1[1] = CONCAT26(uStack_8a,CONCAT42(uStack_8e,uStack_90));
        puVar1 = (undefined8 *)(lVar26 + 0x30 + lVar13);
        *puVar1 = CONCAT26(uStack_11a,uStack_120);
        puVar1[1] = CONCAT26(uStack_112,uStack_a0);
        puVar1 = (undefined8 *)(lVar26 + 0x20 + lVar13);
        *puVar1 = CONCAT26(uStack_12a,local_b8);
        puVar1[1] = CONCAT26(uStack_122,uStack_128);
        puVar1 = (undefined8 *)(lVar26 + 0x10 + lVar13);
        *puVar1 = local_c8;
        puVar1[1] = CONCAT26(uVar11,CONCAT24(local_134,uStack_c0));
        *(undefined8 *)(lVar26 + lVar13) = CONCAT26(uStack_d2,CONCAT24(uStack_d4,6));
        ((undefined8 *)(lVar26 + lVar13))[1] = CONCAT44(uStack_cc,uStack_d0);
        local_138 = uStack_c0;
LAB_1422861d6:
        *(longlong *)(lVar25 + 0x30) = lVar27 + 1;
        *(longlong *)(lVar25 + 0x40) = *(longlong *)(lVar25 + 0x40) + 1;
      }
    }
  }
  else {
LAB_142285d0a:
    uStack_d2 = uVar12;
    if (*(char *)(lVar18 + 0x201) != '\0') goto LAB_142285d18;
LAB_142286018:
    if (*puVar6 != 0) {
      *(undefined4 *)in_R9[2] = *(undefined4 *)((longlong)in_R9 + 0x1c);
      lVar25 = *in_R9;
      lVar27 = *(longlong *)(lVar25 + 0x30);
      uVar10 = *(undefined8 *)(lVar25 + 0x40);
      if (lVar27 == *(longlong *)(lVar25 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      lVar26 = *(longlong *)(lVar25 + 0x28);
      *(undefined8 *)(lVar26 + lVar27 * 0x10) = uVar10;
      *(undefined1 *)(lVar26 + 8 + lVar27 * 0x10) = 0xd;
      goto LAB_1422861d6;
    }
    if (((*piVar8 == 0xb) && ((char)piVar8[1] == '?')) && (1.0 < *(float *)(puVar6 + 1))) {
      country_core::resources::steam::impl$31::from();
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_00 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
        alloc::alloc::handle_alloc_error();
      }
      *extraout_RAX_00 = &DAT_143038ed8;
      extraout_RAX_00[1] = 10;
      extraout_RAX_00[2] = &DAT_143038ee2;
      extraout_RAX_00[3] = 8;
      hint_orchestrator::BindingHintDisplay::
      alternate<system_onboarding::hint_orchestrator::HintGraphicParams>();
      *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
      lVar25 = *in_stack_00000028;
      local_88 = *(undefined8 *)(lVar25 + 0x40);
      uStack_c0 = local_138;
      uStack_a0 = (undefined6)(CONCAT44(uStack_5c,CONCAT22(uStack_5e,uStack_11a)) >> 0x10);
      local_b8 = (undefined6)(CONCAT26(uStack_12c,uStack_132) >> 0x10);
      uStack_90 = 0xd00;
      lVar27 = *(longlong *)(lVar25 + 0x30);
      uStack_d4 = extraout_AX_00;
      if (lVar27 == *(longlong *)(lVar25 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<gpu_profiler::backend::ash::ProfilerOutputBuffer<rhapsody::raw::profiler::ProfilerBuffer>,alloc::alloc::Global>
                  ();
      }
      goto LAB_142286199;
    }
  }
  if (*(char *)(lVar18 + 0x203) != '\0') goto LAB_1422862b6;
  if (*piVar8 == 4) {
    *(undefined4 *)in_R9[2] = *(undefined4 *)((longlong)in_R9 + 0x1c);
    lVar25 = *in_R9;
    lVar27 = *(longlong *)(lVar25 + 0x30);
    uVar10 = *(undefined8 *)(lVar25 + 0x40);
    if (lVar27 == *(longlong *)(lVar25 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                ();
    }
    lVar26 = *(longlong *)(lVar25 + 0x28);
    *(undefined8 *)(lVar26 + lVar27 * 0x10) = uVar10;
    *(undefined1 *)(lVar26 + 8 + lVar27 * 0x10) = 0xf;
  }
  else {
    if ((*(char *)(lVar18 + 0x201) == '\0') || (*(char *)(lVar18 + 0x202) != '\x01'))
    goto LAB_1422862b6;
    puVar4 = (ulonglong *)*in_stack_00000038;
    lVar25 = in_stack_00000038[1];
    uVar5 = *puVar4;
    uVar20 = 0;
    uVar16 = uVar5 - *(ulonglong *)(lVar25 + 0x18);
    if (uVar5 < *(ulonglong *)(lVar25 + 0x18)) {
      uVar16 = uVar20;
    }
    uVar15 = uVar5 - *(ulonglong *)(lVar25 + 0x38);
    if (uVar5 < *(ulonglong *)(lVar25 + 0x38)) {
      uVar15 = uVar20;
    }
    uVar5 = *(ulonglong *)(lVar25 + 0x30);
    uVar24 = uVar5 - uVar15;
    if (uVar5 < uVar15) {
      uVar24 = uVar20;
    }
    uVar9 = *(ulonglong *)(lVar25 + 0x10);
    uVar17 = uVar9 - uVar16;
    if (uVar9 < uVar16) {
      uVar17 = uVar20;
    }
    uVar20 = *(ulonglong *)(lVar25 + 0x40);
    *puVar4 = uVar20 - (uVar17 + uVar24);
    if ((uVar9 < uVar16 || uVar9 - uVar16 == 0) && (uVar5 < uVar15 || uVar5 - uVar15 == 0))
    goto LAB_1422862b6;
    *puVar4 = uVar20;
    country_core::resources::steam::impl$31::from();
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_02 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    *extraout_RAX_02 = &DAT_143038ed8;
    extraout_RAX_02[1] = 10;
    extraout_RAX_02[2] = "hint/rmb";
    extraout_RAX_02[3] = 8;
    hint_orchestrator::BindingHintDisplay::
    alternate<system_onboarding::hint_orchestrator::HintGraphicParams>();
    *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
    lVar25 = *in_stack_00000028;
    uVar10 = *(undefined8 *)(lVar25 + 0x40);
    uStack_a0 = (undefined6)(CONCAT44(uStack_5c,CONCAT22(uStack_5e,uStack_11a)) >> 0x10);
    local_b8 = (undefined6)(CONCAT26(uStack_12c,uStack_132) >> 0x10);
    lVar27 = *(longlong *)(lVar25 + 0x30);
    if (lVar27 == *(longlong *)(lVar25 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<gpu_profiler::backend::ash::ProfilerOutputBuffer<rhapsody::raw::profiler::ProfilerBuffer>,alloc::alloc::Global>
                ();
    }
    lVar26 = *(longlong *)(lVar25 + 0x28);
    lVar13 = lVar27 * 0x58;
    *(undefined8 *)(lVar26 + 0x50 + lVar13) = uVar10;
    puVar1 = (undefined8 *)(lVar26 + 0x40 + lVar13);
    *puVar1 = uStack_110;
    puVar1[1] = CONCAT26(uStack_8a,CONCAT42(uStack_8e,0x100));
    puVar1 = (undefined8 *)(lVar26 + 0x30 + lVar13);
    *puVar1 = CONCAT26(uStack_11a,uStack_120);
    puVar1[1] = CONCAT26(uStack_112,uStack_a0);
    puVar1 = (undefined8 *)(lVar26 + 0x20 + lVar13);
    *puVar1 = CONCAT26(uStack_12a,local_b8);
    puVar1[1] = CONCAT26(uStack_122,uStack_128);
    puVar1 = (undefined8 *)(lVar26 + 0x10 + lVar13);
    *puVar1 = local_c8;
    puVar1[1] = CONCAT26(uVar11,CONCAT24(local_134,local_138));
    *(undefined8 *)(lVar26 + lVar13) = CONCAT26(uStack_d2,CONCAT24(extraout_AX_02,6));
    ((undefined8 *)(lVar26 + lVar13))[1] = CONCAT44(uStack_cc,uStack_d0);
  }
  *(longlong *)(lVar25 + 0x30) = lVar27 + 1;
  *(longlong *)(lVar25 + 0x40) = *(longlong *)(lVar25 + 0x40) + 1;
LAB_1422862b6:
  if (*(char *)(lVar18 + 0x204) == '\0') {
    puVar4 = (ulonglong *)*in_stack_00000040;
    lVar18 = in_stack_00000040[1];
    uVar5 = *puVar4;
    uVar20 = 0;
    uVar16 = uVar5 - *(ulonglong *)(lVar18 + 0x18);
    if (uVar5 < *(ulonglong *)(lVar18 + 0x18)) {
      uVar16 = uVar20;
    }
    uVar15 = uVar5 - *(ulonglong *)(lVar18 + 0x38);
    if (uVar5 < *(ulonglong *)(lVar18 + 0x38)) {
      uVar15 = uVar20;
    }
    uVar5 = *(ulonglong *)(lVar18 + 0x30);
    uVar24 = uVar5 - uVar15;
    if (uVar15 > uVar5) {
      uVar24 = uVar20;
    }
    uVar9 = *(ulonglong *)(lVar18 + 0x10);
    uVar17 = uVar9 - uVar16;
    if (uVar16 > uVar9) {
      uVar17 = uVar20;
    }
    uVar20 = *(ulonglong *)(lVar18 + 0x40);
    *puVar4 = uVar20 - (uVar17 + uVar24);
    if (uVar16 <= uVar9 && uVar9 - uVar16 != 0 || uVar15 <= uVar5 && uVar5 - uVar15 != 0) {
      *puVar4 = uVar20;
      *(undefined4 *)in_R9[2] = *(undefined4 *)((longlong)in_R9 + 0x1c);
      lVar18 = *in_R9;
      lVar25 = *(longlong *)(lVar18 + 0x30);
      uVar10 = *(undefined8 *)(lVar18 + 0x40);
      if (lVar25 == *(longlong *)(lVar18 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      lVar27 = *(longlong *)(lVar18 + 0x28);
      *(undefined8 *)(lVar27 + lVar25 * 0x10) = uVar10;
      *(undefined1 *)(lVar27 + 8 + lVar25 * 0x10) = 0x10;
    }
    else {
      puVar4 = (ulonglong *)*in_stack_00000048;
      lVar18 = in_stack_00000048[1];
      uVar5 = *puVar4;
      uVar20 = 0;
      uVar16 = uVar5 - *(ulonglong *)(lVar18 + 0x18);
      if (uVar5 < *(ulonglong *)(lVar18 + 0x18)) {
        uVar16 = uVar20;
      }
      uVar15 = uVar5 - *(ulonglong *)(lVar18 + 0x38);
      if (uVar5 < *(ulonglong *)(lVar18 + 0x38)) {
        uVar15 = uVar20;
      }
      uVar5 = *(ulonglong *)(lVar18 + 0x30);
      uVar24 = uVar5 - uVar15;
      if (uVar5 < uVar15) {
        uVar24 = uVar20;
      }
      uVar9 = *(ulonglong *)(lVar18 + 0x10);
      uVar17 = uVar9 - uVar16;
      if (uVar9 < uVar16) {
        uVar17 = uVar20;
      }
      uVar20 = *(ulonglong *)(lVar18 + 0x40);
      *puVar4 = uVar20 - (uVar17 + uVar24);
      if ((uVar9 < uVar16 || uVar9 - uVar16 == 0) && (uVar5 < uVar15 || uVar5 - uVar15 == 0)) {
        return;
      }
      *puVar4 = uVar20;
      country_core::resources::steam::impl$31::from();
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_01 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
        alloc::alloc::handle_alloc_error();
      }
      *extraout_RAX_01 = &DAT_143038ed8;
      extraout_RAX_01[1] = 10;
      extraout_RAX_01[2] = &DAT_143038ee2;
      extraout_RAX_01[3] = 8;
      hint_orchestrator::BindingHintDisplay::
      alternate<system_onboarding::hint_orchestrator::HintGraphicParams>();
      *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
      lVar18 = *in_stack_00000028;
      uVar10 = *(undefined8 *)(lVar18 + 0x40);
      uStack_a0 = (undefined6)(CONCAT44(uStack_5c,CONCAT22(uStack_5e,uStack_11a)) >> 0x10);
      local_b8 = (undefined6)(CONCAT26(uStack_12c,uStack_132) >> 0x10);
      lVar25 = *(longlong *)(lVar18 + 0x30);
      if (lVar25 == *(longlong *)(lVar18 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<gpu_profiler::backend::ash::ProfilerOutputBuffer<rhapsody::raw::profiler::ProfilerBuffer>,alloc::alloc::Global>
                  ();
      }
      lVar27 = *(longlong *)(lVar18 + 0x28);
      lVar26 = lVar25 * 0x58;
      *(undefined8 *)(lVar27 + 0x50 + lVar26) = uVar10;
      puVar1 = (undefined8 *)(lVar27 + 0x40 + lVar26);
      *puVar1 = uStack_110;
      puVar1[1] = CONCAT26(uStack_8a,CONCAT42(uStack_8e,0x1000));
      puVar1 = (undefined8 *)(lVar27 + 0x30 + lVar26);
      *puVar1 = CONCAT26(uStack_11a,uStack_120);
      puVar1[1] = CONCAT26(uStack_112,uStack_a0);
      puVar1 = (undefined8 *)(lVar27 + 0x20 + lVar26);
      *puVar1 = CONCAT26(uStack_12a,local_b8);
      puVar1[1] = CONCAT26(uStack_122,uStack_128);
      puVar1 = (undefined8 *)(lVar27 + 0x10 + lVar26);
      *puVar1 = local_c8;
      puVar1[1] = CONCAT26(uVar11,CONCAT24(local_134,local_138));
      *(undefined8 *)(lVar27 + lVar26) = CONCAT26(uStack_d2,CONCAT24(extraout_AX_01,6));
      ((undefined8 *)(lVar27 + lVar26))[1] = CONCAT44(uStack_cc,uStack_d0);
    }
    *(longlong *)(lVar18 + 0x30) = lVar25 + 1;
    *(longlong *)(lVar18 + 0x40) = *(longlong *)(lVar18 + 0x40) + 1;
  }
  return;
}

