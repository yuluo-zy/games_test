// 0x14231e390 update_wrens
// SIG: void __cdecl update_wrens(void)

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl system_critters::wren::update_wrens(void)

{
  uint *puVar1;
  longlong lVar2;
  longlong *plVar3;
  longlong lVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  char cVar8;
  uint uVar9;
  undefined4 uVar10;
  undefined4 uVar11;
  longlong lVar12;
  longlong lVar13;
  longlong lVar14;
  undefined8 uVar15;
  ulonglong uVar16;
  float *extraout_RAX;
  longlong lVar17;
  longlong lVar18;
  longlong lVar19;
  ulonglong uVar20;
  longlong lVar21;
  ulonglong uVar22;
  uint uVar23;
  longlong lVar24;
  uint *puVar25;
  longlong lVar26;
  longlong in_R9;
  longlong lVar27;
  undefined1 auVar28 [16];
  float fVar29;
  undefined1 auVar30 [16];
  float fVar31;
  float fVar32;
  longlong *in_stack_00000028;
  longlong *in_stack_00000038;
  longlong *in_stack_00000040;
  longlong local_270;
  longlong local_260;
  longlong local_218;
  longlong local_1f0;
  float local_128;
  float fStack_124;
  float fStack_120;
  float local_118;
  float fStack_114;
  float fStack_110;
  float local_108;
  float fStack_104;
  float fStack_100;
  float local_f8;
  float fStack_f4;
  float fStack_f0;
  
  lVar19 = *in_stack_00000028;
  lVar12 = in_stack_00000028[1];
  puVar25 = *(uint **)(lVar19 + 0xf8);
  puVar1 = puVar25 + *(longlong *)(lVar19 + 0x100);
  cVar8 = *(char *)(lVar19 + 0x108);
  local_218 = 8;
  local_270 = 0;
  uVar20 = 0;
  uVar22 = 0;
  do {
    if (cVar8 == '\0') {
      if (uVar22 == uVar20) {
        do {
          if (puVar25 == puVar1) goto LAB_14231e80b;
          uVar9 = *puVar25;
          puVar25 = puVar25 + 1;
          uVar20 = *(ulonglong *)(*(longlong *)(lVar12 + 0x100) + 0x58 + (ulonglong)uVar9 * 0xa0);
        } while (uVar20 == 0);
        lVar17 = *(longlong *)(lVar12 + 0x100) + (ulonglong)uVar9 * 0xa0;
        uVar22 = (ulonglong)*(uint *)(lVar17 + 0x94);
        in_R9 = uVar22 * 9;
        lVar2 = *(longlong *)(lVar12 + 0x1a8) + uVar22 * 0x48;
        if (*(ulonglong *)(lVar19 + 0x110) <
            *(ulonglong *)(*(longlong *)(lVar12 + 0x1a8) + 0x40 + uVar22 * 0x48)) {
          in_R9 = *(longlong *)(lVar2 + 0x38);
          uVar22 = *(ulonglong *)(in_R9 + *(ulonglong *)(lVar19 + 0x110) * 8);
          if (uVar22 == 0) goto LAB_14231e61b;
          in_R9 = *(longlong *)(lVar2 + 0x18);
          local_270 = *(longlong *)(in_R9 + 0x10 + ~uVar22 * 0x30);
        }
        else {
LAB_14231e61b:
          local_270 = 0;
        }
        local_218 = *(longlong *)(lVar17 + 0x50);
        uVar22 = 0;
      }
      uVar16 = (ulonglong)*(uint *)(local_218 + 8 + uVar22 * 0x10);
    }
    else {
      if (uVar22 == uVar20) {
        do {
          if (puVar25 == puVar1) goto LAB_14231e80b;
          uVar9 = *puVar25;
          puVar25 = puVar25 + 1;
          uVar20 = *(ulonglong *)(*(longlong *)(lVar12 + 0x1a8) + 0x10 + (ulonglong)uVar9 * 0x48);
        } while (uVar20 == 0);
        lVar2 = *(longlong *)(lVar12 + 0x1a8) + (ulonglong)uVar9 * 0x48;
        if ((*(ulonglong *)(lVar19 + 0x110) < *(ulonglong *)(lVar2 + 0x40)) &&
           (uVar22 = *(ulonglong *)
                      (*(longlong *)(lVar2 + 0x38) + *(ulonglong *)(lVar19 + 0x110) * 8),
           uVar22 != 0)) {
          local_270 = *(longlong *)(*(longlong *)(lVar2 + 0x18) + 0x10 + ~uVar22 * 0x30);
        }
        else {
          local_270 = 0;
        }
        uVar22 = 0;
        in_R9 = lVar19;
      }
      uVar16 = uVar22 & 0xffffffff;
    }
    lVar2 = local_270 + uVar16 * 0x28;
    country_core::resources::placement_data::PlacementDataSsbo::to_affine3a();
    if ((DEBUG_SHOW_HAVEN_BIRD_POPULATION != '\0') && (uVar9 = *(uint *)(lVar2 + 0x10), uVar9 != 0))
    {
      fVar5 = *(float *)(lVar2 + 0x14);
      fVar6 = *(float *)(lVar2 + 0x18);
      fVar7 = *(float *)(lVar2 + 0x1c);
      uVar23 = 0;
      do {
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (extraout_RAX == (float *)0x0) {
                    // WARNING: Subroutine does not return
          alloc::alloc::handle_alloc_error();
        }
        uVar16 = (ulonglong)uVar23;
        fVar29 = (float)uVar16 * 0.5 * 0.0;
        fVar31 = fVar7 * local_108 + fVar6 * local_118 + fVar5 * local_128 + local_f8 + fVar29;
        uVar23 = uVar23 + 1;
        fVar32 = (float)uVar16 * 0.5 +
                 fVar7 * fStack_104 + fVar6 * fStack_114 + fVar5 * fStack_124 + fStack_f4;
        fVar29 = fVar29 + fVar7 * fStack_100 + fVar6 * fStack_110 + fVar5 * fStack_120 + fStack_f0;
        *extraout_RAX = fVar31 + -0.5;
        extraout_RAX[1] = fVar32;
        extraout_RAX[2] = fVar29;
        extraout_RAX[3] = fVar31 + 0.5;
        *(ulonglong *)(extraout_RAX + 4) = CONCAT44(fVar29 + 0.0,fVar32 + 0.0);
        country_core::utils::debug_viz::DebugLine::new();
        country_core::utils::debug_viz::DebugLine::show();
      } while (uVar9 != uVar23);
    }
    uVar22 = uVar22 + 1;
  } while( true );
LAB_14231e80b:
  lVar19 = *in_stack_00000038;
  lVar12 = in_stack_00000038[1];
  uVar10 = *(undefined4 *)((longlong)in_stack_00000038 + 0x14);
  puVar25 = *(uint **)(lVar19 + 0xf8);
  puVar1 = puVar25 + *(longlong *)(lVar19 + 0x100);
  lVar2 = *in_stack_00000040;
  lVar17 = in_stack_00000040[1];
  uVar11 = *(undefined4 *)((longlong)in_stack_00000040 + 0x14);
  if (*(char *)(lVar19 + 0x108) == '\0') {
    if (puVar25 != puVar1) {
      do {
        if (*(longlong *)(*(longlong *)(lVar12 + 0x100) + 0x58 + (ulonglong)*puVar25 * 0xa0) != 0) {
          lVar17 = *(longlong *)(lVar12 + 0x100) + (ulonglong)*puVar25 * 0xa0;
          uVar20 = (ulonglong)*(uint *)(lVar17 + 0x94);
          lVar2 = *(longlong *)(*(longlong *)(lVar12 + 0x1a8) + 0x18 + uVar20 * 0x48);
          lVar19 = ~*(ulonglong *)
                     (*(longlong *)(*(longlong *)(lVar12 + 0x1a8) + 0x38 + uVar20 * 0x48) +
                     *(longlong *)(lVar19 + 0x118) * 8) * 0x30;
          local_260 = *(longlong *)(lVar2 + 0x10 + lVar19);
          local_1f0 = *(longlong *)(lVar2 + 0x28 + lVar19);
          uVar20 = (ulonglong)*(uint *)(*(longlong *)(lVar17 + 0x50) + 8);
          goto LAB_14231eb74;
        }
        puVar25 = puVar25 + 1;
      } while (puVar25 != puVar1);
    }
  }
  else if (puVar25 != puVar1) {
    do {
      if (*(longlong *)(*(longlong *)(lVar12 + 0x1a8) + 0x10 + (ulonglong)*puVar25 * 0x48) != 0) {
        lVar12 = *(longlong *)(lVar12 + 0x1a8) + (ulonglong)*puVar25 * 0x48;
        lVar2 = *(longlong *)(lVar12 + 0x18);
        lVar19 = ~*(ulonglong *)(*(longlong *)(lVar12 + 0x38) + *(longlong *)(lVar19 + 0x118) * 8) *
                 0x30;
        local_260 = *(longlong *)(lVar2 + 0x10 + lVar19);
        local_1f0 = *(longlong *)(lVar2 + 0x28 + lVar19);
        uVar20 = 0;
LAB_14231eb74:
        *(undefined4 *)(local_1f0 + uVar20 * 4) = uVar10;
        country_core::resources::audio_resources::impl$10::set_position();
                    // WARNING: Could not recover jumptable at 0x00014231ebf6. Too many branches
                    // WARNING: Treating indirect jump as call
        (*(code *)(&DAT_14304b638 +
                  *(int *)(&DAT_14304b638 +
                          (ulonglong)*(uint *)(uVar20 * 0x50 + local_260 + 0x30) * 4)))();
        return;
      }
      puVar25 = puVar25 + 1;
    } while (puVar25 != puVar1);
  }
  _DEBUG_AIRBORNE_BIRD_COUNT = 0;
  puVar25 = *(uint **)(lVar2 + 0xf8);
  puVar1 = puVar25 + *(longlong *)(lVar2 + 0x100);
  cVar8 = *(char *)(lVar2 + 0x108);
  local_260 = 8;
  lVar26 = 0;
  uVar20 = 0;
  uVar22 = 0;
  do {
    do {
      if (cVar8 == '\0') {
        if (uVar22 == uVar20) {
          if (puVar25 == puVar1) {
            return;
          }
          while( true ) {
            uVar20 = *(ulonglong *)
                      (*(longlong *)(lVar17 + 0x100) + 0x58 + (ulonglong)*puVar25 * 0xa0);
            if (uVar20 != 0) break;
            puVar25 = puVar25 + 1;
            if (puVar25 == puVar1) {
              return;
            }
          }
          lVar18 = *(longlong *)(lVar17 + 0x100) + (ulonglong)*puVar25 * 0xa0;
          uVar22 = (ulonglong)*(uint *)(lVar18 + 0x94);
          lVar13 = *(longlong *)(*(longlong *)(lVar17 + 0x1a8) + 0x18 + uVar22 * 0x48);
          lVar24 = ~*(ulonglong *)
                     (*(longlong *)(*(longlong *)(lVar17 + 0x1a8) + 0x38 + uVar22 * 0x48) +
                     *(longlong *)(lVar2 + 0x110) * 8) * 0x30;
          lVar26 = *(longlong *)(lVar13 + 0x10 + lVar24);
          in_R9 = *(longlong *)(lVar13 + 0x28 + lVar24);
          local_260 = *(longlong *)(lVar18 + 0x50);
          puVar25 = puVar25 + 1;
          uVar22 = 0;
        }
        uVar16 = (ulonglong)*(uint *)(local_260 + 8 + uVar22 * 0x10);
      }
      else {
        if (uVar22 == uVar20) {
          if (puVar25 == puVar1) {
            return;
          }
          while( true ) {
            uVar20 = *(ulonglong *)
                      (*(longlong *)(lVar17 + 0x1a8) + 0x10 + (ulonglong)*puVar25 * 0x48);
            if (uVar20 != 0) break;
            puVar25 = puVar25 + 1;
            if (puVar25 == puVar1) {
              return;
            }
          }
          lVar26 = *(longlong *)(lVar17 + 0x1a8) + (ulonglong)*puVar25 * 0x48;
          lVar13 = *(longlong *)(lVar26 + 0x18);
          lVar18 = ~*(ulonglong *)(*(longlong *)(lVar26 + 0x38) + *(longlong *)(lVar2 + 0x110) * 8)
                   * 0x30;
          lVar26 = *(longlong *)(lVar13 + 0x10 + lVar18);
          in_R9 = *(longlong *)(lVar13 + 0x28 + lVar18);
          puVar25 = puVar25 + 1;
          uVar22 = 0;
        }
        uVar16 = uVar22 & 0xffffffff;
      }
      plVar3 = (longlong *)(lVar26 + uVar16 * 0x18);
      if (plVar3 == (longlong *)0x0) {
        return;
      }
      uVar22 = uVar22 + 1;
      *(undefined4 *)(in_R9 + uVar16 * 4) = uVar11;
      lVar13 = plVar3[1];
    } while (lVar13 == 0);
    lVar18 = *plVar3;
    lVar24 = 0;
    do {
      if (*(int *)(lVar18 + 4 + lVar24) != 0) {
        uVar16 = *(ulonglong *)(lVar18 + lVar24) & 0xffffffff;
        if ((uVar16 < *(ulonglong *)(lVar12 + 0x10)) &&
           (*(int *)(*(longlong *)(lVar12 + 8) + uVar16 * 0x14) ==
            (int)(*(ulonglong *)(lVar18 + lVar24) >> 0x20))) {
          lVar4 = *(longlong *)(lVar12 + 8) + uVar16 * 0x14;
          uVar16 = (ulonglong)*(uint *)(lVar4 + 4);
          if ((uVar16 != 0xffffffff) &&
             ((uVar16 < *(ulonglong *)(lVar19 + 0x38) &&
              ((*(ulonglong *)
                 (*(longlong *)(lVar19 + 0x28) + (ulonglong)(*(uint *)(lVar4 + 4) >> 6) * 8) >>
                (uVar16 & 0x3f) & 1) != 0)))) {
            uVar16 = (ulonglong)*(uint *)(lVar4 + 0x10);
            lVar27 = *(longlong *)
                      (*(longlong *)(lVar12 + 0x1a8) + 0x18 +
                      (ulonglong)*(uint *)(lVar4 + 0xc) * 0x48);
            lVar4 = *(longlong *)
                     (*(longlong *)(lVar12 + 0x1a8) + 0x38 +
                     (ulonglong)*(uint *)(lVar4 + 0xc) * 0x48);
            lVar21 = ~*(ulonglong *)(lVar4 + *(longlong *)(lVar19 + 0x110) * 8) * 0x30;
            lVar14 = *(longlong *)(lVar27 + 0x10 + lVar21);
            lVar21 = *(longlong *)(lVar27 + 0x28 + lVar21);
            lVar4 = *(longlong *)
                     (lVar27 + 0x10 +
                     ~*(ulonglong *)(lVar4 + *(longlong *)(lVar19 + 0x118) * 8) * 0x30);
            lVar27 = uVar16 * 0x30;
            uVar15 = *(undefined8 *)(lVar18 + 8 + lVar24);
            *(undefined4 *)(lVar14 + 0x18 + lVar27) = *(undefined4 *)(lVar18 + 0x10 + lVar24);
            *(undefined8 *)(lVar14 + 0x10 + lVar27) = uVar15;
            *(undefined4 *)(lVar14 + lVar27) = 0;
            *(undefined4 *)(lVar21 + uVar16 * 4) = uVar10;
            *(undefined4 *)(lVar14 + 8 + lVar27) = 0;
            auVar28 = *(undefined1 (*) [16])(lVar14 + lVar27);
            auVar30._0_4_ =
                 auVar28._12_4_ * auVar28._12_4_ + auVar28._4_4_ * auVar28._4_4_ +
                 auVar28._8_4_ * auVar28._8_4_ + auVar28._0_4_ * auVar28._0_4_;
            auVar30._4_4_ = auVar30._0_4_;
            auVar30._8_4_ = auVar30._0_4_;
            auVar30._12_4_ = auVar30._0_4_;
            auVar30 = sqrtps(auVar30,auVar30);
            auVar28 = divps(auVar28,auVar30);
            *(undefined4 *)(lVar21 + uVar16 * 4) = uVar10;
            *(undefined1 (*) [16])(lVar14 + lVar27) = auVar28;
            if (*(int *)(lVar4 + 0x30 + uVar16 * 0x50) == 0) goto LAB_14231f4e9;
          }
        }
        *(undefined4 *)(lVar18 + 4 + lVar24) = 0;
      }
LAB_14231f4e9:
      if (DEBUG_SHOW_BIRD_SPOTS != '\0') {
        country_core::utils::debug_viz::DebugSphere::new();
        country_core::utils::debug_viz::DebugSphere::radius();
        country_core::utils::debug_viz::DebugSphere::color();
        country_core::utils::debug_viz::DebugSphere::show();
      }
      lVar24 = lVar24 + 0x18;
    } while (lVar13 * 0x18 != lVar24);
  } while( true );
}

