
void __cdecl
toolbar::ui_interaction::
iter_subtree<tuple_<>,toolbar::ui_interaction::ui_nav_update::closure_0::closure_env_0>(void)

{
  char *pcVar1;
  undefined8 uVar2;
  longlong *plVar3;
  longlong lVar4;
  float *pfVar5;
  uint uVar6;
  uint uVar7;
  ulonglong uVar8;
  bool bVar9;
  ulonglong extraout_RAX;
  uint uVar10;
  int in_EDX;
  longlong lVar11;
  int in_R8D;
  ulonglong uVar12;
  longlong lVar13;
  longlong *in_R9;
  longlong lVar14;
  ushort uVar15;
  ulonglong uVar16;
  byte bVar17;
  char cVar18;
  char cVar23;
  float fVar19;
  char cVar21;
  char cVar22;
  undefined1 auVar20 [16];
  float fVar24;
  float fVar25;
  float fVar26;
  undefined1 auVar27 [16];
  int iVar30;
  undefined1 auVar28 [16];
  undefined1 auVar29 [16];
  int local_98;
  longlong lStack_90;
  longlong lStack_80;
  
  plVar3 = (longlong *)*in_R9;
  if (plVar3[3] == 0) {
LAB_141ea8fbd:
    taffy::node::Taffy::children();
    if (local_98 != 4) {
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    for (lVar11 = lStack_80 << 3; lVar11 != 0; lVar11 = lVar11 + -8) {
      iter_subtree<tuple_<>,toolbar::ui_interaction::ui_nav_update::closure_0::closure_env_0>();
    }
    if (lStack_90 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    return;
  }
  core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref_<slotmap::DefaultKey>_>();
  lVar11 = *plVar3;
  bVar17 = (byte)(extraout_RAX >> 0x39);
  auVar20 = ZEXT216(CONCAT11(bVar17,bVar17));
  auVar20 = pshuflw(auVar20,auVar20,0);
  lVar14 = 0;
  uVar12 = extraout_RAX;
  do {
    uVar12 = plVar3[1] & uVar12;
    pcVar1 = (char *)(lVar11 + uVar12);
    cVar18 = auVar20[0];
    auVar28[0] = -(*pcVar1 == cVar18);
    cVar21 = auVar20[1];
    auVar28[1] = -(pcVar1[1] == cVar21);
    cVar22 = auVar20[2];
    auVar28[2] = -(pcVar1[2] == cVar22);
    cVar23 = auVar20[3];
    auVar28[3] = -(pcVar1[3] == cVar23);
    auVar28[4] = -(pcVar1[4] == cVar18);
    auVar28[5] = -(pcVar1[5] == cVar21);
    auVar28[6] = -(pcVar1[6] == cVar22);
    auVar28[7] = -(pcVar1[7] == cVar23);
    auVar28[8] = -(pcVar1[8] == cVar18);
    auVar28[9] = -(pcVar1[9] == cVar21);
    auVar28[10] = -(pcVar1[10] == cVar22);
    auVar28[0xb] = -(pcVar1[0xb] == cVar23);
    auVar28[0xc] = -(pcVar1[0xc] == cVar18);
    auVar28[0xd] = -(pcVar1[0xd] == cVar21);
    auVar28[0xe] = -(pcVar1[0xe] == cVar22);
    auVar28[0xf] = -(pcVar1[0xf] == cVar23);
    uVar15 = (ushort)(SUB161(auVar28 >> 7,0) & 1) | (ushort)(SUB161(auVar28 >> 0xf,0) & 1) << 1 |
             (ushort)(SUB161(auVar28 >> 0x17,0) & 1) << 2 |
             (ushort)(SUB161(auVar28 >> 0x1f,0) & 1) << 3 |
             (ushort)(SUB161(auVar28 >> 0x27,0) & 1) << 4 |
             (ushort)(SUB161(auVar28 >> 0x2f,0) & 1) << 5 |
             (ushort)(SUB161(auVar28 >> 0x37,0) & 1) << 6 |
             (ushort)(SUB161(auVar28 >> 0x3f,0) & 1) << 7 |
             (ushort)(SUB161(auVar28 >> 0x47,0) & 1) << 8 |
             (ushort)(SUB161(auVar28 >> 0x4f,0) & 1) << 9 |
             (ushort)(SUB161(auVar28 >> 0x57,0) & 1) << 10 |
             (ushort)(SUB161(auVar28 >> 0x5f,0) & 1) << 0xb |
             (ushort)(SUB161(auVar28 >> 0x67,0) & 1) << 0xc |
             (ushort)(SUB161(auVar28 >> 0x6f,0) & 1) << 0xd |
             (ushort)(SUB161(auVar28 >> 0x77,0) & 1) << 0xe | (ushort)(auVar28[0xf] >> 7) << 0xf;
    uVar10 = (uint)uVar15;
    while (uVar15 != 0) {
      uVar6 = 0;
      for (uVar7 = uVar10; (uVar7 & 1) == 0; uVar7 = uVar7 >> 1 | 0x80000000) {
        uVar6 = uVar6 + 1;
      }
      uVar16 = uVar6 + uVar12 & plVar3[1];
      uVar2 = *(undefined8 *)(lVar11 + -0x10 + uVar16 * -0x10);
      auVar29._0_4_ = -(uint)((int)uVar2 == in_EDX);
      iVar30 = -(uint)((int)((ulonglong)uVar2 >> 0x20) == in_R8D);
      auVar29._4_4_ = auVar29._0_4_;
      auVar29._8_4_ = iVar30;
      auVar29._12_4_ = iVar30;
      iVar30 = movmskpd((int)uVar16 * 0x10,auVar29);
      if (iVar30 == 3) {
        lVar14 = ((longlong *)in_R9[1])[1];
        uVar12 = *(ulonglong *)(lVar11 + -8 + uVar16 * -0x10);
        uVar16 = uVar12 & 0xffffffff;
        if ((uVar16 < *(ulonglong *)(lVar14 + 0x10)) &&
           (*(int *)(*(longlong *)(lVar14 + 8) + uVar16 * 0x14) == (int)(uVar12 >> 0x20))) {
          lVar11 = *(longlong *)(lVar14 + 8) + uVar16 * 0x14;
          uVar16 = (ulonglong)*(uint *)(lVar11 + 4);
          if ((uVar16 != 0xffffffff) &&
             ((lVar4 = *(longlong *)in_R9[1], uVar16 < *(ulonglong *)(lVar4 + 0x38) &&
              ((*(ulonglong *)
                 (*(longlong *)(lVar4 + 0x28) + (ulonglong)(*(uint *)(lVar11 + 4) >> 6) * 8) >>
                (uVar16 & 0x3f) & 1) != 0)))) {
            if (uVar16 < *(ulonglong *)(lVar14 + 0x108)) {
              lVar13 = uVar16 * 0xa0 + *(longlong *)(lVar14 + 0x100);
            }
            else {
              lVar13 = 0;
            }
            uVar16 = *(ulonglong *)(lVar4 + 0x110);
            bVar9 = true;
            if ((uVar16 < *(ulonglong *)(lVar13 + 0x88)) &&
               (*(longlong *)(*(longlong *)(lVar13 + 0x80) + uVar16 * 8) != 0)) {
              lVar14 = (ulonglong)*(uint *)(lVar11 + 0xc) * 0x48 + *(longlong *)(lVar14 + 0x1a8);
              if ((*(ulonglong *)(lVar14 + 0x40) <= uVar16) ||
                 (uVar16 = *(ulonglong *)(*(longlong *)(lVar14 + 0x38) + uVar16 * 8), uVar16 == 0))
              {
                bVar9 = false;
                goto LAB_141ea8e5b;
              }
              lVar14 = *(longlong *)(*(longlong *)(lVar14 + 0x18) + 0x10 + ~uVar16 * 0x30);
              bVar9 = false;
            }
            else {
LAB_141ea8e5b:
              lVar14 = 0;
            }
            if (!bVar9 && lVar14 != 0) {
              uVar2 = *(undefined8 *)(lVar14 + (ulonglong)*(uint *)(lVar11 + 0x10) * 0x10);
              fVar24 = (float)uVar2 - (float)*(undefined8 *)in_R9[2];
              fVar25 = (float)((ulonglong)uVar2 >> 0x20) -
                       (float)((ulonglong)*(undefined8 *)in_R9[2] >> 0x20);
              fVar19 = SQRT(fVar25 * fVar25 + fVar24 * fVar24);
              fVar26 = 1.0 / fVar19;
              uVar10 = (uint)((int)fVar26 - 1U < 0x7fffff ||
                             (int)ABS(fVar26) - 0x800000U < 0x7f000000 && -1 < (int)fVar26);
              uVar16 = CONCAT44((int)(uVar10 << 0x1f) >> 0x1f,(int)(uVar10 << 0x1f) >> 0x1f) &
                       CONCAT44(fVar26 * fVar25,fVar26 * fVar24);
              fVar25 = (float)*(undefined8 *)in_R9[3];
              fVar26 = (float)((ulonglong)*(undefined8 *)in_R9[3] >> 0x20);
              fVar24 = 1.0 / SQRT(fVar26 * fVar26 + fVar25 * fVar25);
              uVar10 = (uint)((int)fVar24 - 1U < 0x7fffff ||
                             (int)ABS(fVar24) - 0x800000U < 0x7f000000 && -1 < (int)fVar24);
              uVar8 = CONCAT44((int)(uVar10 << 0x1f) >> 0x1f,(int)(uVar10 << 0x1f) >> 0x1f) &
                      CONCAT44(fVar24 * fVar26,fVar24 * fVar25);
              if ((0.0 < (float)(uVar8 >> 0x20) * (float)(uVar16 >> 0x20) +
                         (float)uVar8 * (float)uVar16) &&
                 (pfVar5 = (float *)in_R9[4], fVar19 < *pfVar5)) {
                *(ulonglong *)in_R9[5] = uVar12;
                *pfVar5 = fVar19;
              }
            }
          }
        }
        goto LAB_141ea8fbd;
      }
      uVar15 = (ushort)(uVar10 - 1) & (ushort)uVar10;
      uVar10 = CONCAT22((short)(uVar10 - 1 >> 0x10),uVar15);
    }
    auVar27[0] = -(*pcVar1 == -1);
    auVar27[1] = -(pcVar1[1] == -1);
    auVar27[2] = -(pcVar1[2] == -1);
    auVar27[3] = -(pcVar1[3] == -1);
    auVar27[4] = -(pcVar1[4] == -1);
    auVar27[5] = -(pcVar1[5] == -1);
    auVar27[6] = -(pcVar1[6] == -1);
    auVar27[7] = -(pcVar1[7] == -1);
    auVar27[8] = -(pcVar1[8] == -1);
    auVar27[9] = -(pcVar1[9] == -1);
    auVar27[10] = -(pcVar1[10] == -1);
    auVar27[0xb] = -(pcVar1[0xb] == -1);
    auVar27[0xc] = -(pcVar1[0xc] == -1);
    auVar27[0xd] = -(pcVar1[0xd] == -1);
    auVar27[0xe] = -(pcVar1[0xe] == -1);
    auVar27[0xf] = -(pcVar1[0xf] == -1);
    if ((((((((((((((((SUB161(auVar27 >> 7,0) & 1) != 0 || (SUB161(auVar27 >> 0xf,0) & 1) != 0) ||
                    (SUB161(auVar27 >> 0x17,0) & 1) != 0) || (SUB161(auVar27 >> 0x1f,0) & 1) != 0)
                  || (SUB161(auVar27 >> 0x27,0) & 1) != 0) || (SUB161(auVar27 >> 0x2f,0) & 1) != 0)
                || (SUB161(auVar27 >> 0x37,0) & 1) != 0) || (SUB161(auVar27 >> 0x3f,0) & 1) != 0) ||
              (SUB161(auVar27 >> 0x47,0) & 1) != 0) || (SUB161(auVar27 >> 0x4f,0) & 1) != 0) ||
            (SUB161(auVar27 >> 0x57,0) & 1) != 0) || (SUB161(auVar27 >> 0x5f,0) & 1) != 0) ||
          (SUB161(auVar27 >> 0x67,0) & 1) != 0) || (SUB161(auVar27 >> 0x6f,0) & 1) != 0) ||
        (SUB161(auVar27 >> 0x77,0) & 1) != 0) || auVar27[0xf] < '\0') goto LAB_141ea8fbd;
    uVar12 = uVar12 + lVar14 + 0x10;
    lVar14 = lVar14 + 0x10;
  } while( true );
}

