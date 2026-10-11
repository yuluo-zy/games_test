
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

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
  float fVar16;
  float fVar17;
  float fVar18;
  ulonglong uVar19;
  float *extraout_RAX;
  longlong lVar20;
  longlong lVar21;
  longlong lVar22;
  ulonglong uVar23;
  longlong lVar24;
  ulonglong uVar25;
  uint uVar26;
  longlong lVar27;
  uint *puVar28;
  longlong lVar29;
  longlong in_R9;
  longlong lVar30;
  undefined1 auVar31 [16];
  float fVar32;
  undefined1 auVar33 [16];
  float fVar34;
  float fVar35;
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
  
  fVar18 = _UNK_14304b39c;
  fVar17 = ___xmm_3f0000000000000000000000bf000000;
  fVar16 = __real_3f000000;
  lVar22 = *in_stack_00000028;
  lVar12 = in_stack_00000028[1];
  puVar28 = *(uint **)(lVar22 + 0xf8);
  puVar1 = puVar28 + *(longlong *)(lVar22 + 0x100);
  cVar8 = *(char *)(lVar22 + 0x108);
  local_218 = 8;
  local_270 = 0;
  uVar23 = 0;
  uVar25 = 0;
  do {
    if (cVar8 == '\0') {
      if (uVar25 == uVar23) {
        do {
          if (puVar28 == puVar1) goto LAB_14231e80b;
          uVar9 = *puVar28;
          puVar28 = puVar28 + 1;
          uVar23 = *(ulonglong *)(*(longlong *)(lVar12 + 0x100) + 0x58 + (ulonglong)uVar9 * 0xa0);
        } while (uVar23 == 0);
        lVar20 = *(longlong *)(lVar12 + 0x100) + (ulonglong)uVar9 * 0xa0;
        uVar25 = (ulonglong)*(uint *)(lVar20 + 0x94);
        in_R9 = uVar25 * 9;
        lVar2 = *(longlong *)(lVar12 + 0x1a8) + uVar25 * 0x48;
        if (*(ulonglong *)(lVar22 + 0x110) <
            *(ulonglong *)(*(longlong *)(lVar12 + 0x1a8) + 0x40 + uVar25 * 0x48)) {
          in_R9 = *(longlong *)(lVar2 + 0x38);
          uVar25 = *(ulonglong *)(in_R9 + *(ulonglong *)(lVar22 + 0x110) * 8);
          if (uVar25 == 0) goto LAB_14231e61b;
          in_R9 = *(longlong *)(lVar2 + 0x18);
          local_270 = *(longlong *)(in_R9 + 0x10 + ~uVar25 * 0x30);
        }
        else {
LAB_14231e61b:
          local_270 = 0;
        }
        local_218 = *(longlong *)(lVar20 + 0x50);
        uVar25 = 0;
      }
      uVar19 = (ulonglong)*(uint *)(local_218 + 8 + uVar25 * 0x10);
    }
    else {
      if (uVar25 == uVar23) {
        do {
          if (puVar28 == puVar1) goto LAB_14231e80b;
          uVar9 = *puVar28;
          puVar28 = puVar28 + 1;
          uVar23 = *(ulonglong *)(*(longlong *)(lVar12 + 0x1a8) + 0x10 + (ulonglong)uVar9 * 0x48);
        } while (uVar23 == 0);
        lVar2 = *(longlong *)(lVar12 + 0x1a8) + (ulonglong)uVar9 * 0x48;
        if ((*(ulonglong *)(lVar22 + 0x110) < *(ulonglong *)(lVar2 + 0x40)) &&
           (uVar25 = *(ulonglong *)
                      (*(longlong *)(lVar2 + 0x38) + *(ulonglong *)(lVar22 + 0x110) * 8),
           uVar25 != 0)) {
          local_270 = *(longlong *)(*(longlong *)(lVar2 + 0x18) + 0x10 + ~uVar25 * 0x30);
        }
        else {
          local_270 = 0;
        }
        uVar25 = 0;
        in_R9 = lVar22;
      }
      uVar19 = uVar25 & 0xffffffff;
    }
    lVar2 = local_270 + uVar19 * 0x28;
    country_core::resources::placement_data::PlacementDataSsbo::to_affine3a();
    if ((DEBUG_SHOW_HAVEN_BIRD_POPULATION != '\0') && (uVar9 = *(uint *)(lVar2 + 0x10), uVar9 != 0))
    {
      fVar5 = *(float *)(lVar2 + 0x14);
      fVar6 = *(float *)(lVar2 + 0x18);
      fVar7 = *(float *)(lVar2 + 0x1c);
      uVar26 = 0;
      do {
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (extraout_RAX == (float *)0x0) {
                    /* WARNING: Subroutine does not return */
          alloc::alloc::handle_alloc_error();
        }
        uVar19 = (ulonglong)uVar26;
        fVar32 = (float)uVar19 * fVar16 * 0.0;
        fVar34 = fVar7 * local_108 + fVar6 * local_118 + fVar5 * local_128 + local_f8 + fVar32;
        uVar26 = uVar26 + 1;
        fVar35 = (float)uVar19 * fVar16 +
                 fVar7 * fStack_104 + fVar6 * fStack_114 + fVar5 * fStack_124 + fStack_f4;
        fVar32 = fVar32 + fVar7 * fStack_100 + fVar6 * fStack_110 + fVar5 * fStack_120 + fStack_f0;
        *extraout_RAX = fVar34 + fVar17;
        extraout_RAX[1] = fVar35;
        extraout_RAX[2] = fVar32;
        extraout_RAX[3] = fVar34 + fVar18;
        *(ulonglong *)(extraout_RAX + 4) = CONCAT44(fVar32 + 0.0,fVar35 + 0.0);
        country_core::utils::debug_viz::DebugLine::new();
        country_core::utils::debug_viz::DebugLine::show();
      } while (uVar9 != uVar26);
    }
    uVar25 = uVar25 + 1;
  } while( true );
LAB_14231e80b:
  lVar22 = *in_stack_00000038;
  lVar12 = in_stack_00000038[1];
  uVar10 = *(undefined4 *)((longlong)in_stack_00000038 + 0x14);
  puVar28 = *(uint **)(lVar22 + 0xf8);
  puVar1 = puVar28 + *(longlong *)(lVar22 + 0x100);
  lVar2 = *in_stack_00000040;
  lVar20 = in_stack_00000040[1];
  uVar11 = *(undefined4 *)((longlong)in_stack_00000040 + 0x14);
  if (*(char *)(lVar22 + 0x108) == '\0') {
    if (puVar28 != puVar1) {
      do {
        if (*(longlong *)(*(longlong *)(lVar12 + 0x100) + 0x58 + (ulonglong)*puVar28 * 0xa0) != 0) {
          lVar20 = *(longlong *)(lVar12 + 0x100) + (ulonglong)*puVar28 * 0xa0;
          uVar23 = (ulonglong)*(uint *)(lVar20 + 0x94);
          lVar2 = *(longlong *)(*(longlong *)(lVar12 + 0x1a8) + 0x18 + uVar23 * 0x48);
          lVar22 = ~*(ulonglong *)
                     (*(longlong *)(*(longlong *)(lVar12 + 0x1a8) + 0x38 + uVar23 * 0x48) +
                     *(longlong *)(lVar22 + 0x118) * 8) * 0x30;
          local_260 = *(longlong *)(lVar2 + 0x10 + lVar22);
          local_1f0 = *(longlong *)(lVar2 + 0x28 + lVar22);
          uVar23 = (ulonglong)*(uint *)(*(longlong *)(lVar20 + 0x50) + 8);
          goto LAB_14231eb74;
        }
        puVar28 = puVar28 + 1;
      } while (puVar28 != puVar1);
    }
  }
  else if (puVar28 != puVar1) {
    do {
      if (*(longlong *)(*(longlong *)(lVar12 + 0x1a8) + 0x10 + (ulonglong)*puVar28 * 0x48) != 0) {
        lVar12 = *(longlong *)(lVar12 + 0x1a8) + (ulonglong)*puVar28 * 0x48;
        lVar2 = *(longlong *)(lVar12 + 0x18);
        lVar22 = ~*(ulonglong *)(*(longlong *)(lVar12 + 0x38) + *(longlong *)(lVar22 + 0x118) * 8) *
                 0x30;
        local_260 = *(longlong *)(lVar2 + 0x10 + lVar22);
        local_1f0 = *(longlong *)(lVar2 + 0x28 + lVar22);
        uVar23 = 0;
LAB_14231eb74:
        *(undefined4 *)(local_1f0 + uVar23 * 4) = uVar10;
        country_core::resources::audio_resources::impl$10::set_position();
                    /* WARNING: Could not recover jumptable at 0x00014231ebf6. Too many branches */
                    /* WARNING: Treating indirect jump as call */
        (*(code *)(&DAT_14304b638 +
                  *(int *)(&DAT_14304b638 +
                          (ulonglong)*(uint *)(uVar23 * 0x50 + local_260 + 0x30) * 4)))();
        return;
      }
      puVar28 = puVar28 + 1;
    } while (puVar28 != puVar1);
  }
  _DEBUG_AIRBORNE_BIRD_COUNT = 0;
  puVar28 = *(uint **)(lVar2 + 0xf8);
  puVar1 = puVar28 + *(longlong *)(lVar2 + 0x100);
  cVar8 = *(char *)(lVar2 + 0x108);
  local_260 = 8;
  lVar29 = 0;
  uVar23 = 0;
  uVar25 = 0;
  do {
    do {
      if (cVar8 == '\0') {
        if (uVar25 == uVar23) {
          if (puVar28 == puVar1) {
            return;
          }
          while( true ) {
            uVar23 = *(ulonglong *)
                      (*(longlong *)(lVar20 + 0x100) + 0x58 + (ulonglong)*puVar28 * 0xa0);
            if (uVar23 != 0) break;
            puVar28 = puVar28 + 1;
            if (puVar28 == puVar1) {
              return;
            }
          }
          lVar21 = *(longlong *)(lVar20 + 0x100) + (ulonglong)*puVar28 * 0xa0;
          uVar25 = (ulonglong)*(uint *)(lVar21 + 0x94);
          lVar13 = *(longlong *)(*(longlong *)(lVar20 + 0x1a8) + 0x18 + uVar25 * 0x48);
          lVar27 = ~*(ulonglong *)
                     (*(longlong *)(*(longlong *)(lVar20 + 0x1a8) + 0x38 + uVar25 * 0x48) +
                     *(longlong *)(lVar2 + 0x110) * 8) * 0x30;
          lVar29 = *(longlong *)(lVar13 + 0x10 + lVar27);
          in_R9 = *(longlong *)(lVar13 + 0x28 + lVar27);
          local_260 = *(longlong *)(lVar21 + 0x50);
          puVar28 = puVar28 + 1;
          uVar25 = 0;
        }
        uVar19 = (ulonglong)*(uint *)(local_260 + 8 + uVar25 * 0x10);
      }
      else {
        if (uVar25 == uVar23) {
          if (puVar28 == puVar1) {
            return;
          }
          while( true ) {
            uVar23 = *(ulonglong *)
                      (*(longlong *)(lVar20 + 0x1a8) + 0x10 + (ulonglong)*puVar28 * 0x48);
            if (uVar23 != 0) break;
            puVar28 = puVar28 + 1;
            if (puVar28 == puVar1) {
              return;
            }
          }
          lVar29 = *(longlong *)(lVar20 + 0x1a8) + (ulonglong)*puVar28 * 0x48;
          lVar13 = *(longlong *)(lVar29 + 0x18);
          lVar21 = ~*(ulonglong *)(*(longlong *)(lVar29 + 0x38) + *(longlong *)(lVar2 + 0x110) * 8)
                   * 0x30;
          lVar29 = *(longlong *)(lVar13 + 0x10 + lVar21);
          in_R9 = *(longlong *)(lVar13 + 0x28 + lVar21);
          puVar28 = puVar28 + 1;
          uVar25 = 0;
        }
        uVar19 = uVar25 & 0xffffffff;
      }
      plVar3 = (longlong *)(lVar29 + uVar19 * 0x18);
      if (plVar3 == (longlong *)0x0) {
        return;
      }
      uVar25 = uVar25 + 1;
      *(undefined4 *)(in_R9 + uVar19 * 4) = uVar11;
      lVar13 = plVar3[1];
    } while (lVar13 == 0);
    lVar21 = *plVar3;
    lVar27 = 0;
    do {
      if (*(int *)(lVar21 + 4 + lVar27) != 0) {
        uVar19 = *(ulonglong *)(lVar21 + lVar27) & 0xffffffff;
        if ((uVar19 < *(ulonglong *)(lVar12 + 0x10)) &&
           (*(int *)(*(longlong *)(lVar12 + 8) + uVar19 * 0x14) ==
            (int)(*(ulonglong *)(lVar21 + lVar27) >> 0x20))) {
          lVar4 = *(longlong *)(lVar12 + 8) + uVar19 * 0x14;
          uVar19 = (ulonglong)*(uint *)(lVar4 + 4);
          if ((uVar19 != 0xffffffff) &&
             ((uVar19 < *(ulonglong *)(lVar22 + 0x38) &&
              ((*(ulonglong *)
                 (*(longlong *)(lVar22 + 0x28) + (ulonglong)(*(uint *)(lVar4 + 4) >> 6) * 8) >>
                (uVar19 & 0x3f) & 1) != 0)))) {
            uVar19 = (ulonglong)*(uint *)(lVar4 + 0x10);
            lVar30 = *(longlong *)
                      (*(longlong *)(lVar12 + 0x1a8) + 0x18 +
                      (ulonglong)*(uint *)(lVar4 + 0xc) * 0x48);
            lVar4 = *(longlong *)
                     (*(longlong *)(lVar12 + 0x1a8) + 0x38 +
                     (ulonglong)*(uint *)(lVar4 + 0xc) * 0x48);
            lVar24 = ~*(ulonglong *)(lVar4 + *(longlong *)(lVar22 + 0x110) * 8) * 0x30;
            lVar14 = *(longlong *)(lVar30 + 0x10 + lVar24);
            lVar24 = *(longlong *)(lVar30 + 0x28 + lVar24);
            lVar4 = *(longlong *)
                     (lVar30 + 0x10 +
                     ~*(ulonglong *)(lVar4 + *(longlong *)(lVar22 + 0x118) * 8) * 0x30);
            lVar30 = uVar19 * 0x30;
            uVar15 = *(undefined8 *)(lVar21 + 8 + lVar27);
            *(undefined4 *)(lVar14 + 0x18 + lVar30) = *(undefined4 *)(lVar21 + 0x10 + lVar27);
            *(undefined8 *)(lVar14 + 0x10 + lVar30) = uVar15;
            *(undefined4 *)(lVar14 + lVar30) = 0;
            *(undefined4 *)(lVar24 + uVar19 * 4) = uVar10;
            *(undefined4 *)(lVar14 + 8 + lVar30) = 0;
            auVar31 = *(undefined1 (*) [16])(lVar14 + lVar30);
            auVar33._0_4_ =
                 auVar31._12_4_ * auVar31._12_4_ + auVar31._4_4_ * auVar31._4_4_ +
                 auVar31._8_4_ * auVar31._8_4_ + auVar31._0_4_ * auVar31._0_4_;
            auVar33._4_4_ = auVar33._0_4_;
            auVar33._8_4_ = auVar33._0_4_;
            auVar33._12_4_ = auVar33._0_4_;
            auVar33 = sqrtps(auVar33,auVar33);
            auVar31 = divps(auVar31,auVar33);
            *(undefined4 *)(lVar24 + uVar19 * 4) = uVar10;
            *(undefined1 (*) [16])(lVar14 + lVar30) = auVar31;
            if (*(int *)(lVar4 + 0x30 + uVar19 * 0x50) == 0) goto LAB_14231f4e9;
          }
        }
        *(undefined4 *)(lVar21 + 4 + lVar27) = 0;
      }
LAB_14231f4e9:
      if (DEBUG_SHOW_BIRD_SPOTS != '\0') {
        country_core::utils::debug_viz::DebugSphere::new();
        country_core::utils::debug_viz::DebugSphere::radius();
        country_core::utils::debug_viz::DebugSphere::color();
        country_core::utils::debug_viz::DebugSphere::show();
      }
      lVar27 = lVar27 + 0x18;
    } while (lVar13 * 0x18 != lVar27);
  } while( true );
}

