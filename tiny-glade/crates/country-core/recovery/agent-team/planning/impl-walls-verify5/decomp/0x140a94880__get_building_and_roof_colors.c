// get_building_and_roof_colors  size=1101
// void __cdecl get_building_and_roof_colors(void)

void __cdecl
country_core::systems::wall::shapes::rectangle::create_rectangle::get_building_and_roof_colors(void)

{
  char *pcVar1;
  longlong lVar2;
  ulonglong uVar3;
  longlong lVar4;
  undefined8 uVar5;
  uint uVar6;
  uint uVar7;
  ulonglong extraout_RAX;
  longlong lVar8;
  ulonglong extraout_RAX_00;
  ulonglong extraout_RAX_01;
  undefined4 *in_RCX;
  ulonglong uVar9;
  uint uVar10;
  ulonglong in_RDX;
  undefined1 uVar11;
  undefined4 uVar12;
  ulonglong in_R8;
  ushort uVar13;
  longlong in_R9;
  longlong lVar14;
  ulonglong uVar15;
  undefined1 uVar16;
  undefined4 uVar17;
  byte bVar18;
  char cVar19;
  char cVar21;
  char cVar22;
  char cVar23;
  undefined1 auVar20 [16];
  undefined1 auVar24 [16];
  undefined1 auVar25 [16];
  undefined1 auVar26 [16];
  undefined1 auVar27 [16];
  undefined1 auVar28 [16];
  undefined1 auVar29 [16];
  longlong in_stack_00000028;
  longlong *in_stack_00000030;
  longlong *in_stack_00000038;
  longlong in_stack_00000040;
  undefined4 local_6c;
  undefined7 local_60;
  undefined4 uStack_59;
  
  uVar12 = *(undefined4 *)(in_stack_00000040 + 0x1c);
  local_6c = *(undefined4 *)(in_stack_00000040 + 0x20);
  uVar16 = *(undefined1 *)(in_stack_00000040 + 0x42);
  uVar17 = *(undefined4 *)(in_stack_00000040 + 0x24);
  if ((in_RDX & 1) != 0) {
    local_60 = (undefined7)in_R8;
    uStack_59._0_1_ = (undefined1)(in_R8 >> 0x38);
    if (in_stack_00000030[3] != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      lVar2 = *in_stack_00000030;
      uVar3 = in_stack_00000030[1];
      bVar18 = (byte)(extraout_RAX >> 0x39);
      auVar20 = ZEXT216(CONCAT11(bVar18,bVar18));
      auVar20 = pshuflw(auVar20,auVar20,0);
      lVar8 = 0;
      uVar9 = extraout_RAX;
      while( true ) {
        uVar9 = uVar3 & uVar9;
        pcVar1 = (char *)(lVar2 + uVar9);
        cVar19 = auVar20[0];
        auVar27[0] = -(*pcVar1 == cVar19);
        cVar21 = auVar20[1];
        auVar27[1] = -(pcVar1[1] == cVar21);
        cVar22 = auVar20[2];
        auVar27[2] = -(pcVar1[2] == cVar22);
        cVar23 = auVar20[3];
        auVar27[3] = -(pcVar1[3] == cVar23);
        auVar27[4] = -(pcVar1[4] == cVar19);
        auVar27[5] = -(pcVar1[5] == cVar21);
        auVar27[6] = -(pcVar1[6] == cVar22);
        auVar27[7] = -(pcVar1[7] == cVar23);
        auVar27[8] = -(pcVar1[8] == cVar19);
        auVar27[9] = -(pcVar1[9] == cVar21);
        auVar27[10] = -(pcVar1[10] == cVar22);
        auVar27[0xb] = -(pcVar1[0xb] == cVar23);
        auVar27[0xc] = -(pcVar1[0xc] == cVar19);
        auVar27[0xd] = -(pcVar1[0xd] == cVar21);
        auVar27[0xe] = -(pcVar1[0xe] == cVar22);
        auVar27[0xf] = -(pcVar1[0xf] == cVar23);
        uVar13 = (ushort)(SUB161(auVar27 >> 7,0) & 1) | (ushort)(SUB161(auVar27 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar27 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar27 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar27 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar27 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar27 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar27 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar27 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar27 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar27 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar27 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar27 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar27 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar27 >> 0x77,0) & 1) << 0xe | (ushort)(auVar27[0xf] >> 7) << 0xf
        ;
        uVar10 = (uint)uVar13;
        while (uVar13 != 0) {
          uVar6 = 0;
          for (uVar7 = uVar10; (uVar7 & 1) == 0; uVar7 = uVar7 >> 1 | 0x80000000) {
            uVar6 = uVar6 + 1;
          }
          lVar14 = (uVar6 + uVar9 & uVar3) * -0x2a8;
          if (*(ulonglong *)(lVar2 + -0x2a8 + lVar14) == in_R8) {
            if ((*(ulonglong *)(lVar2 + -0x290 + lVar14) < 2) ||
               (*(float *)(lVar2 + -0x270 + lVar14) <= 0.0)) {
                    // WARNING: Subroutine does not return
              core::panicking::panic();
            }
            if (1 < *(uint *)(lVar14 + lVar2 + -0x48)) {
              uVar11 = 0;
              goto LAB_140a9497f;
            }
            if (*(ulonglong *)(in_R9 + 0x10) <= in_R8) {
                    // WARNING: Subroutine does not return
              core::panicking::panic_bounds_check();
            }
            if (*(ulonglong *)(in_stack_00000028 + 0x10) <= in_R8) {
                    // WARNING: Subroutine does not return
              core::panicking::panic_bounds_check();
            }
            uVar12 = *(undefined4 *)(*(longlong *)(in_R9 + 8) + in_R8 * 4);
            uVar17 = *(undefined4 *)(*(longlong *)(in_stack_00000028 + 8) + in_R8 * 4);
            if (in_stack_00000030[9] == 0) goto LAB_140a94ba8;
            core::hash::BuildHasher::
            hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
            lVar8 = in_stack_00000030[6];
            bVar18 = (byte)(extraout_RAX_00 >> 0x39);
            auVar20 = ZEXT216(CONCAT11(bVar18,bVar18));
            auVar20 = pshuflw(auVar20,auVar20,0);
            lVar14 = 0;
            uVar9 = extraout_RAX_00;
            goto LAB_140a94a6e;
          }
          uVar13 = (ushort)(uVar10 - 1) & (ushort)uVar10;
          uVar10 = CONCAT22((short)(uVar10 - 1 >> 0x10),uVar13);
        }
        auVar24[0] = -(*pcVar1 == -1);
        auVar24[1] = -(pcVar1[1] == -1);
        auVar24[2] = -(pcVar1[2] == -1);
        auVar24[3] = -(pcVar1[3] == -1);
        auVar24[4] = -(pcVar1[4] == -1);
        auVar24[5] = -(pcVar1[5] == -1);
        auVar24[6] = -(pcVar1[6] == -1);
        auVar24[7] = -(pcVar1[7] == -1);
        auVar24[8] = -(pcVar1[8] == -1);
        auVar24[9] = -(pcVar1[9] == -1);
        auVar24[10] = -(pcVar1[10] == -1);
        auVar24[0xb] = -(pcVar1[0xb] == -1);
        auVar24[0xc] = -(pcVar1[0xc] == -1);
        auVar24[0xd] = -(pcVar1[0xd] == -1);
        auVar24[0xe] = -(pcVar1[0xe] == -1);
        auVar24[0xf] = -(pcVar1[0xf] == -1);
        if ((((((((((((((((SUB161(auVar24 >> 7,0) & 1) != 0 || (SUB161(auVar24 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar24 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar24 >> 0x1f,0) & 1) != 0) || (SUB161(auVar24 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar24 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar24 >> 0x37,0) & 1) != 0) || (SUB161(auVar24 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar24 >> 0x47,0) & 1) != 0) || (SUB161(auVar24 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar24 >> 0x57,0) & 1) != 0) || (SUB161(auVar24 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar24 >> 0x67,0) & 1) != 0) || (SUB161(auVar24 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar24 >> 0x77,0) & 1) != 0) || auVar24[0xf] < '\0') break;
        uVar9 = uVar9 + lVar8 + 0x10;
        lVar8 = lVar8 + 0x10;
      }
    }
  }
  uVar11 = 0;
LAB_140a9497f:
  in_RCX[6] = uStack_59;
  *(ulonglong *)((longlong)in_RCX + 0x11) = CONCAT17((undefined1)uStack_59,local_60);
  *in_RCX = uVar12;
  in_RCX[1] = local_6c;
  *(undefined1 *)(in_RCX + 3) = uVar16;
  in_RCX[2] = uVar17;
  *(undefined1 *)(in_RCX + 4) = uVar11;
  return;
LAB_140a94a6e:
  uVar9 = in_stack_00000030[7] & uVar9;
  pcVar1 = (char *)(lVar8 + uVar9);
  cVar19 = auVar20[0];
  auVar28[0] = -(*pcVar1 == cVar19);
  cVar21 = auVar20[1];
  auVar28[1] = -(pcVar1[1] == cVar21);
  cVar22 = auVar20[2];
  auVar28[2] = -(pcVar1[2] == cVar22);
  cVar23 = auVar20[3];
  auVar28[3] = -(pcVar1[3] == cVar23);
  auVar28[4] = -(pcVar1[4] == cVar19);
  auVar28[5] = -(pcVar1[5] == cVar21);
  auVar28[6] = -(pcVar1[6] == cVar22);
  auVar28[7] = -(pcVar1[7] == cVar23);
  auVar28[8] = -(pcVar1[8] == cVar19);
  auVar28[9] = -(pcVar1[9] == cVar21);
  auVar28[10] = -(pcVar1[10] == cVar22);
  auVar28[0xb] = -(pcVar1[0xb] == cVar23);
  auVar28[0xc] = -(pcVar1[0xc] == cVar19);
  auVar28[0xd] = -(pcVar1[0xd] == cVar21);
  auVar28[0xe] = -(pcVar1[0xe] == cVar22);
  auVar28[0xf] = -(pcVar1[0xf] == cVar23);
  uVar13 = (ushort)(SUB161(auVar28 >> 7,0) & 1) | (ushort)(SUB161(auVar28 >> 0xf,0) & 1) << 1 |
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
  uVar10 = (uint)uVar13;
  while (uVar13 != 0) {
    uVar6 = 0;
    for (uVar7 = uVar10; (uVar7 & 1) == 0; uVar7 = uVar7 >> 1 | 0x80000000) {
      uVar6 = uVar6 + 1;
    }
    uVar15 = uVar6 + uVar9 & in_stack_00000030[7];
    if (*(ulonglong *)(lVar8 + -0x10 + uVar15 * -0x10) == in_R8) {
      uVar9 = *(ulonglong *)(lVar8 + -8 + uVar15 * -0x10);
      if (uVar9 >> 0x20 == 0) goto LAB_140a94ba8;
      lVar8 = in_stack_00000038[1];
      uVar15 = uVar9 & 0xffffffff;
      if ((*(ulonglong *)(lVar8 + 0x10) <= uVar15) ||
         (*(int *)(*(longlong *)(lVar8 + 8) + uVar15 * 0x14) != (int)(uVar9 >> 0x20)))
      goto LAB_140a94ba8;
      lVar14 = *(longlong *)(lVar8 + 8) + uVar15 * 0x14;
      uVar9 = (ulonglong)*(uint *)(lVar14 + 4);
      if ((uVar9 == 0xffffffff) ||
         ((lVar4 = *in_stack_00000038, *(ulonglong *)(lVar4 + 0x38) <= uVar9 ||
          ((*(ulonglong *)
             (*(longlong *)(lVar4 + 0x28) + (ulonglong)(*(uint *)(lVar14 + 4) >> 6) * 8) >>
            (uVar9 & 0x3f) & 1) == 0)))) goto LAB_140a94ba8;
      lVar8 = *(longlong *)
               (*(longlong *)
                 (*(longlong *)(lVar8 + 0x1a8) + 0x18 + (ulonglong)*(uint *)(lVar14 + 0xc) * 0x48) +
                0x10 + ~*(ulonglong *)
                         (*(longlong *)
                           (*(longlong *)(lVar8 + 0x1a8) + 0x38 +
                           (ulonglong)*(uint *)(lVar14 + 0xc) * 0x48) +
                         *(longlong *)(lVar4 + 0x110) * 8) * 0x30);
      lVar14 = (ulonglong)*(uint *)(lVar14 + 0x10) * 0x58;
      local_6c = *(undefined4 *)(lVar8 + 0x50 + lVar14);
      uVar11 = *(undefined1 *)(lVar8 + 0x40 + lVar14);
      uVar5 = *(undefined8 *)(lVar8 + 0x41 + lVar14);
      local_60 = (undefined7)uVar5;
      uStack_59._1_3_ = (undefined3)((uint)*(undefined4 *)(lVar8 + 0x48 + lVar14) >> 8);
      uStack_59._0_1_ = (undefined1)((ulonglong)uVar5 >> 0x38);
      goto LAB_140a94baa;
    }
    uVar13 = (ushort)(uVar10 - 1) & (ushort)uVar10;
    uVar10 = CONCAT22((short)(uVar10 - 1 >> 0x10),uVar13);
  }
  auVar25[0] = -(*pcVar1 == -1);
  auVar25[1] = -(pcVar1[1] == -1);
  auVar25[2] = -(pcVar1[2] == -1);
  auVar25[3] = -(pcVar1[3] == -1);
  auVar25[4] = -(pcVar1[4] == -1);
  auVar25[5] = -(pcVar1[5] == -1);
  auVar25[6] = -(pcVar1[6] == -1);
  auVar25[7] = -(pcVar1[7] == -1);
  auVar25[8] = -(pcVar1[8] == -1);
  auVar25[9] = -(pcVar1[9] == -1);
  auVar25[10] = -(pcVar1[10] == -1);
  auVar25[0xb] = -(pcVar1[0xb] == -1);
  auVar25[0xc] = -(pcVar1[0xc] == -1);
  auVar25[0xd] = -(pcVar1[0xd] == -1);
  auVar25[0xe] = -(pcVar1[0xe] == -1);
  auVar25[0xf] = -(pcVar1[0xf] == -1);
  if ((((((((((((((((SUB161(auVar25 >> 7,0) & 1) != 0 || (SUB161(auVar25 >> 0xf,0) & 1) != 0) ||
                  (SUB161(auVar25 >> 0x17,0) & 1) != 0) || (SUB161(auVar25 >> 0x1f,0) & 1) != 0) ||
                (SUB161(auVar25 >> 0x27,0) & 1) != 0) || (SUB161(auVar25 >> 0x2f,0) & 1) != 0) ||
              (SUB161(auVar25 >> 0x37,0) & 1) != 0) || (SUB161(auVar25 >> 0x3f,0) & 1) != 0) ||
            (SUB161(auVar25 >> 0x47,0) & 1) != 0) || (SUB161(auVar25 >> 0x4f,0) & 1) != 0) ||
          (SUB161(auVar25 >> 0x57,0) & 1) != 0) || (SUB161(auVar25 >> 0x5f,0) & 1) != 0) ||
        (SUB161(auVar25 >> 0x67,0) & 1) != 0) || (SUB161(auVar25 >> 0x6f,0) & 1) != 0) ||
      (SUB161(auVar25 >> 0x77,0) & 1) != 0) || auVar25[0xf] < '\0') {
LAB_140a94ba8:
    uVar11 = 0;
LAB_140a94baa:
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    bVar18 = (byte)(extraout_RAX_01 >> 0x39);
    auVar20 = ZEXT216(CONCAT11(bVar18,bVar18));
    auVar20 = pshuflw(auVar20,auVar20,0);
    lVar8 = 0;
    uVar9 = extraout_RAX_01;
    do {
      uVar9 = uVar3 & uVar9;
      pcVar1 = (char *)(lVar2 + uVar9);
      cVar19 = auVar20[0];
      auVar29[0] = -(*pcVar1 == cVar19);
      cVar21 = auVar20[1];
      auVar29[1] = -(pcVar1[1] == cVar21);
      cVar22 = auVar20[2];
      auVar29[2] = -(pcVar1[2] == cVar22);
      cVar23 = auVar20[3];
      auVar29[3] = -(pcVar1[3] == cVar23);
      auVar29[4] = -(pcVar1[4] == cVar19);
      auVar29[5] = -(pcVar1[5] == cVar21);
      auVar29[6] = -(pcVar1[6] == cVar22);
      auVar29[7] = -(pcVar1[7] == cVar23);
      auVar29[8] = -(pcVar1[8] == cVar19);
      auVar29[9] = -(pcVar1[9] == cVar21);
      auVar29[10] = -(pcVar1[10] == cVar22);
      auVar29[0xb] = -(pcVar1[0xb] == cVar23);
      auVar29[0xc] = -(pcVar1[0xc] == cVar19);
      auVar29[0xd] = -(pcVar1[0xd] == cVar21);
      auVar29[0xe] = -(pcVar1[0xe] == cVar22);
      auVar29[0xf] = -(pcVar1[0xf] == cVar23);
      uVar13 = (ushort)(SUB161(auVar29 >> 7,0) & 1) | (ushort)(SUB161(auVar29 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar29 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar29 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar29 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar29 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar29 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar29 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar29 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar29 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar29 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar29 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar29 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar29 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar29 >> 0x77,0) & 1) << 0xe | (ushort)(auVar29[0xf] >> 7) << 0xf;
      uVar10 = (uint)uVar13;
      while (uVar13 != 0) {
        uVar6 = 0;
        for (uVar7 = uVar10; (uVar7 & 1) == 0; uVar7 = uVar7 >> 1 | 0x80000000) {
          uVar6 = uVar6 + 1;
        }
        lVar14 = (uVar6 + uVar9 & uVar3) * -0x2a8;
        if (*(ulonglong *)(lVar2 + -0x2a8 + lVar14) == in_R8) {
          if ((*(ulonglong *)(lVar2 + -0x290 + lVar14) < 2) ||
             (*(float *)(lVar2 + -0x270 + lVar14) <= 0.0)) {
                    // WARNING: Subroutine does not return
            core::panicking::panic();
          }
          uVar16 = *(undefined1 *)(lVar2 + lVar14 + -7);
          goto LAB_140a9497f;
        }
        uVar13 = (ushort)(uVar10 - 1) & (ushort)uVar10;
        uVar10 = CONCAT22((short)(uVar10 - 1 >> 0x10),uVar13);
      }
      auVar26[0] = -(*pcVar1 == -1);
      auVar26[1] = -(pcVar1[1] == -1);
      auVar26[2] = -(pcVar1[2] == -1);
      auVar26[3] = -(pcVar1[3] == -1);
      auVar26[4] = -(pcVar1[4] == -1);
      auVar26[5] = -(pcVar1[5] == -1);
      auVar26[6] = -(pcVar1[6] == -1);
      auVar26[7] = -(pcVar1[7] == -1);
      auVar26[8] = -(pcVar1[8] == -1);
      auVar26[9] = -(pcVar1[9] == -1);
      auVar26[10] = -(pcVar1[10] == -1);
      auVar26[0xb] = -(pcVar1[0xb] == -1);
      auVar26[0xc] = -(pcVar1[0xc] == -1);
      auVar26[0xd] = -(pcVar1[0xd] == -1);
      auVar26[0xe] = -(pcVar1[0xe] == -1);
      auVar26[0xf] = -(pcVar1[0xf] == -1);
      if ((((((((((((((((SUB161(auVar26 >> 7,0) & 1) != 0 || (SUB161(auVar26 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar26 >> 0x17,0) & 1) != 0) || (SUB161(auVar26 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar26 >> 0x27,0) & 1) != 0) || (SUB161(auVar26 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar26 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar26 >> 0x3f,0) & 1) != 0) || (SUB161(auVar26 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar26 >> 0x4f,0) & 1) != 0) || (SUB161(auVar26 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar26 >> 0x5f,0) & 1) != 0) || (SUB161(auVar26 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar26 >> 0x6f,0) & 1) != 0) || (SUB161(auVar26 >> 0x77,0) & 1) != 0) ||
          auVar26[0xf] < '\0') {
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      uVar9 = uVar9 + lVar8 + 0x10;
      lVar8 = lVar8 + 0x10;
    } while( true );
  }
  uVar9 = uVar9 + lVar14 + 0x10;
  lVar14 = lVar14 + 0x10;
  goto LAB_140a94a6e;
}



// CALLEES: [{'name': 'hash_one<std::hash::random::RandomState,ref$<country_core::systems::ui::GlobalId>_>', 'address': '1405b0d70'}, {'name': 'panic', 'address': '1428d9430'}, {'name': 'panic_bounds_check', 'address': '1428d9518'}, {'name': 'unwrap_failed', 'address': '1428d9310'}]