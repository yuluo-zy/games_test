
void __cdecl country_core::utils::debug_viz::DebugLine::show(void)

{
  ulonglong uVar1;
  float *pfVar2;
  undefined4 uVar3;
  undefined1 uVar4;
  undefined1 uVar5;
  undefined8 uVar6;
  float *pfVar7;
  ulonglong uVar8;
  longlong lVar9;
  undefined1 auVar10 [16];
  undefined8 uVar11;
  undefined8 uVar12;
  float *pfVar13;
  undefined8 *extraout_RAX;
  void *extraout_RAX_00;
  undefined8 *in_RCX;
  void *_Dst;
  float fVar14;
  float fVar15;
  float fVar16;
  float fVar17;
  float fVar18;
  float fVar19;
  float fVar20;
  float fVar21;
  float fVar22;
  float fVar23;
  longlong local_120;
  void *local_118;
  size_t local_110;
  undefined6 uStack_66;
  
  alloc::fmt::format::format_inner();
  uVar6 = *in_RCX;
  pfVar7 = (float *)in_RCX[1];
  uVar8 = in_RCX[2];
  uVar3 = *(undefined4 *)(in_RCX + 8);
  lVar9 = in_RCX[3];
  if (SBORROW8(0,lVar9)) {
                    // WARNING: Could not recover jumptable at 0x000140b0507b. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(&DAT_142ae731c + *(int *)(&DAT_142ae731c + (ulonglong)*(uint *)(in_RCX + 6) * 4)))();
    return;
  }
  uVar11 = in_RCX[4];
  uVar12 = in_RCX[5];
  uVar4 = *(undefined1 *)((longlong)in_RCX + 0x45);
  fVar16 = 0.0;
  if (1 < uVar8) {
    fVar17 = (float)*(undefined8 *)(pfVar7 + 3);
    fVar19 = (float)((ulonglong)*(undefined8 *)(pfVar7 + 3) >> 0x20);
    fVar15 = pfVar7[5];
    fVar16 = SQRT((pfVar7[2] - fVar15) * (pfVar7[2] - fVar15) +
                  (pfVar7[1] - fVar19) * (pfVar7[1] - fVar19) +
                  (*pfVar7 - fVar17) * (*pfVar7 - fVar17));
    if (uVar8 != 2) {
      uVar1 = uVar8 * 0xc - 0x24;
      auVar10._8_8_ = 0;
      auVar10._0_8_ = uVar1;
      if ((SUB164(auVar10 * ZEXT816(0xaaaaaaaaaaaaaaab),8) >> 3 & 1) == 0) {
        pfVar13 = pfVar7 + 9;
        fVar14 = pfVar7[8];
        fVar18 = (float)*(undefined8 *)(pfVar7 + 6);
        fVar20 = (float)((ulonglong)*(undefined8 *)(pfVar7 + 6) >> 0x20);
        fVar16 = fVar16 + SQRT((fVar15 - fVar14) * (fVar15 - fVar14) +
                               (fVar19 - fVar20) * (fVar19 - fVar20) +
                               (fVar17 - fVar18) * (fVar17 - fVar18));
      }
      else {
        pfVar13 = pfVar7 + 6;
        fVar14 = fVar15;
        fVar18 = fVar17;
        fVar20 = fVar19;
      }
      if (0xb < uVar1) {
        do {
          pfVar2 = pfVar13 + 2;
          fVar15 = fVar14 - *pfVar2;
          fVar22 = (float)*(undefined8 *)pfVar13;
          fVar17 = fVar18 - fVar22;
          fVar23 = (float)((ulonglong)*(undefined8 *)pfVar13 >> 0x20);
          fVar19 = fVar20 - fVar23;
          fVar14 = pfVar13[5];
          fVar18 = (float)*(undefined8 *)(pfVar13 + 3);
          fVar20 = (float)((ulonglong)*(undefined8 *)(pfVar13 + 3) >> 0x20);
          pfVar13 = pfVar13 + 6;
          fVar21 = *pfVar2 - fVar14;
          fVar22 = fVar22 - fVar18;
          fVar23 = fVar23 - fVar20;
          fVar16 = SQRT(fVar21 * fVar21 + fVar23 * fVar23 + fVar22 * fVar22) +
                   SQRT(fVar15 * fVar15 + fVar19 * fVar19 + fVar17 * fVar17) + fVar16;
        } while (pfVar13 != pfVar7 + uVar8 * 3);
      }
    }
  }
  uVar5 = *(undefined1 *)((longlong)in_RCX + 0x44);
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  extraout_RAX[8] = CONCAT44(uVar3,fVar16);
  extraout_RAX[9] = CONCAT62(uStack_66,CONCAT11(uVar4,uVar5));
  extraout_RAX[6] = uVar11;
  extraout_RAX[7] = uVar12;
  extraout_RAX[4] = uVar8;
  extraout_RAX[5] = lVar9;
  extraout_RAX[2] = uVar6;
  extraout_RAX[3] = pfVar7;
  *extraout_RAX = 1;
  extraout_RAX[1] = 1;
  if (-1 < (longlong)local_110) {
    if (local_110 == 0) {
      _Dst = (void *)0x1;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      _Dst = extraout_RAX_00;
      if (extraout_RAX_00 == (void *)0x0) goto LAB_140b053d7;
    }
    memcpy(_Dst,local_118,local_110);
    append_only_vec::AppendOnlyVec::
    push<country_core::systems::ui::signifiers::signifier_create::UiAppendSignifier>();
    if (local_120 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    return;
  }
LAB_140b053d7:
                    // WARNING: Subroutine does not return
  alloc::raw_vec::handle_error();
}

