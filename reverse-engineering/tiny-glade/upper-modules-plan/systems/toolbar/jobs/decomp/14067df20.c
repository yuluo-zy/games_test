// 0x14067df20 decorate_hovered
// signature: void __cdecl decorate_hovered(void)

void __cdecl toolbar::mei::widgets::decorate::impl$0::decorate_hovered(void)

{
  char *pcVar1;
  undefined8 *puVar2;
  longlong lVar3;
  longlong *plVar4;
  uint uVar5;
  uint uVar6;
  ulonglong extraout_RAX;
  ulonglong extraout_RAX_00;
  longlong extraout_RAX_01;
  ulonglong extraout_RAX_02;
  longlong extraout_RAX_03;
  ulonglong extraout_RAX_04;
  longlong *in_RCX;
  longlong *in_RDX;
  ulonglong uVar7;
  longlong lVar8;
  ulonglong uVar9;
  longlong lVar10;
  longlong lVar11;
  uint uVar12;
  ushort uVar13;
  longlong lVar14;
  byte bVar15;
  char cVar16;
  char cVar18;
  char cVar19;
  char cVar20;
  undefined1 auVar17 [16];
  undefined1 auVar21 [16];
  undefined1 auVar22 [16];
  undefined1 auVar23 [16];
  undefined1 auVar24 [16];
  undefined1 auVar25 [16];
  undefined1 auVar26 [16];
  longlong local_58;
  longlong local_50;
  
  lVar3 = *in_RDX;
  if (*(longlong *)(lVar3 + 0x1b8) != 0) {
    ::core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    lVar11 = *(longlong *)(lVar3 + 0x1a0);
    bVar15 = (byte)(extraout_RAX >> 0x39);
    auVar17 = ZEXT216(CONCAT11(bVar15,bVar15));
    auVar17 = pshuflw(auVar17,auVar17,0);
    lVar8 = in_RDX[1];
    lVar10 = 0;
    uVar9 = extraout_RAX;
    while( true ) {
      uVar9 = *(ulonglong *)(lVar3 + 0x1a8) & uVar9;
      pcVar1 = (char *)(lVar11 + uVar9);
      cVar16 = auVar17[0];
      auVar24[0] = -(*pcVar1 == cVar16);
      cVar18 = auVar17[1];
      auVar24[1] = -(pcVar1[1] == cVar18);
      cVar19 = auVar17[2];
      auVar24[2] = -(pcVar1[2] == cVar19);
      cVar20 = auVar17[3];
      auVar24[3] = -(pcVar1[3] == cVar20);
      auVar24[4] = -(pcVar1[4] == cVar16);
      auVar24[5] = -(pcVar1[5] == cVar18);
      auVar24[6] = -(pcVar1[6] == cVar19);
      auVar24[7] = -(pcVar1[7] == cVar20);
      auVar24[8] = -(pcVar1[8] == cVar16);
      auVar24[9] = -(pcVar1[9] == cVar18);
      auVar24[10] = -(pcVar1[10] == cVar19);
      auVar24[0xb] = -(pcVar1[0xb] == cVar20);
      auVar24[0xc] = -(pcVar1[0xc] == cVar16);
      auVar24[0xd] = -(pcVar1[0xd] == cVar18);
      auVar24[0xe] = -(pcVar1[0xe] == cVar19);
      auVar24[0xf] = -(pcVar1[0xf] == cVar20);
      uVar13 = (ushort)(SUB161(auVar24 >> 7,0) & 1) | (ushort)(SUB161(auVar24 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar24 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar24 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar24 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar24 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar24 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar24 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar24 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar24 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar24 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar24 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar24 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar24 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar24 >> 0x77,0) & 1) << 0xe | (ushort)(auVar24[0xf] >> 7) << 0xf;
      uVar12 = (uint)uVar13;
      while (uVar13 != 0) {
        uVar5 = 0;
        for (uVar6 = uVar12; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
          uVar5 = uVar5 + 1;
        }
        lVar14 = (uVar5 + uVar9 & *(ulonglong *)(lVar3 + 0x1a8)) * -0xc0;
        if (lVar8 == *(longlong *)(lVar11 + -0xc0 + lVar14)) {
          if ((*(byte *)(lVar11 + -2 + lVar14) & 1) == 0) goto LAB_14067e363;
          ::utils::calculate_hash<tuple_<country_core::systems::ui::GlobalId,u32>_>();
          plVar4 = (longlong *)*in_RDX;
          uVar9 = extraout_RAX_00 ^ 0xb1695320004fb443;
          core::MeiContext::reuse_or_create_node();
          *(undefined1 *)(extraout_RAX_01 + 0x5d) = 0;
          if ((uVar9 & 1) == 0) {
            lVar3 = plVar4[2];
            if (lVar3 == *plVar4) {
              alloc::raw_vec::RawVec::grow_one<enum2_<iced_core::event::Event>,alloc::alloc::Global>
                        ();
            }
            lVar11 = plVar4[1];
            lVar10 = lVar3 * 0x38;
            *(undefined8 *)(lVar11 + lVar10) = 3;
            *(longlong *)(lVar11 + 0x10 + lVar10) = lVar8;
            puVar2 = (undefined8 *)(lVar11 + 0x18 + lVar10);
            *puVar2 = 0xc0a00000;
            puVar2[1] = 0x80000000bf800000;
            *(longlong *)(lVar11 + 0x28 + lVar10) = SUB168(ZEXT416(0x3f800000),0);
            *(ulonglong *)(lVar11 + 0x30 + lVar10) = extraout_RAX_00 ^ 0xb1695320004fb443;
            plVar4[2] = lVar3 + 1;
          }
          widgets::impl$26::image<ref_<str_>_>();
          if (*(longlong *)(local_58 + 0x1b8) == 0) goto LAB_14067e34f;
          ::core::hash::BuildHasher::
          hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
          lVar3 = *(longlong *)(local_58 + 0x1a0);
          bVar15 = (byte)(extraout_RAX_02 >> 0x39);
          auVar17 = ZEXT216(CONCAT11(bVar15,bVar15));
          auVar17 = pshuflw(auVar17,auVar17,0);
          lVar11 = 0;
          uVar9 = extraout_RAX_02;
          goto LAB_14067e152;
        }
        uVar13 = (ushort)(uVar12 - 1) & (ushort)uVar12;
        uVar12 = CONCAT22((short)(uVar12 - 1 >> 0x10),uVar13);
      }
      auVar21[0] = -(*pcVar1 == -1);
      auVar21[1] = -(pcVar1[1] == -1);
      auVar21[2] = -(pcVar1[2] == -1);
      auVar21[3] = -(pcVar1[3] == -1);
      auVar21[4] = -(pcVar1[4] == -1);
      auVar21[5] = -(pcVar1[5] == -1);
      auVar21[6] = -(pcVar1[6] == -1);
      auVar21[7] = -(pcVar1[7] == -1);
      auVar21[8] = -(pcVar1[8] == -1);
      auVar21[9] = -(pcVar1[9] == -1);
      auVar21[10] = -(pcVar1[10] == -1);
      auVar21[0xb] = -(pcVar1[0xb] == -1);
      auVar21[0xc] = -(pcVar1[0xc] == -1);
      auVar21[0xd] = -(pcVar1[0xd] == -1);
      auVar21[0xe] = -(pcVar1[0xe] == -1);
      auVar21[0xf] = -(pcVar1[0xf] == -1);
      if ((((((((((((((((SUB161(auVar21 >> 7,0) & 1) != 0 || (SUB161(auVar21 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar21 >> 0x17,0) & 1) != 0) || (SUB161(auVar21 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar21 >> 0x27,0) & 1) != 0) || (SUB161(auVar21 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar21 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar21 >> 0x3f,0) & 1) != 0) || (SUB161(auVar21 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar21 >> 0x4f,0) & 1) != 0) || (SUB161(auVar21 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar21 >> 0x5f,0) & 1) != 0) || (SUB161(auVar21 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar21 >> 0x6f,0) & 1) != 0) || (SUB161(auVar21 >> 0x77,0) & 1) != 0) ||
          auVar21[0xf] < '\0') break;
      uVar9 = uVar9 + lVar10 + 0x10;
      lVar10 = lVar10 + 0x10;
    }
  }
                    // WARNING: Subroutine does not return
  ::core::option::expect_failed();
LAB_14067e152:
  uVar9 = *(ulonglong *)(local_58 + 0x1a8) & uVar9;
  pcVar1 = (char *)(lVar3 + uVar9);
  cVar16 = auVar17[0];
  auVar25[0] = -(*pcVar1 == cVar16);
  cVar18 = auVar17[1];
  auVar25[1] = -(pcVar1[1] == cVar18);
  cVar19 = auVar17[2];
  auVar25[2] = -(pcVar1[2] == cVar19);
  cVar20 = auVar17[3];
  auVar25[3] = -(pcVar1[3] == cVar20);
  auVar25[4] = -(pcVar1[4] == cVar16);
  auVar25[5] = -(pcVar1[5] == cVar18);
  auVar25[6] = -(pcVar1[6] == cVar19);
  auVar25[7] = -(pcVar1[7] == cVar20);
  auVar25[8] = -(pcVar1[8] == cVar16);
  auVar25[9] = -(pcVar1[9] == cVar18);
  auVar25[10] = -(pcVar1[10] == cVar19);
  auVar25[0xb] = -(pcVar1[0xb] == cVar20);
  auVar25[0xc] = -(pcVar1[0xc] == cVar16);
  auVar25[0xd] = -(pcVar1[0xd] == cVar18);
  auVar25[0xe] = -(pcVar1[0xe] == cVar19);
  auVar25[0xf] = -(pcVar1[0xf] == cVar20);
  uVar13 = (ushort)(SUB161(auVar25 >> 7,0) & 1) | (ushort)(SUB161(auVar25 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar25 >> 0x17,0) & 1) << 2 |
           (ushort)(SUB161(auVar25 >> 0x1f,0) & 1) << 3 |
           (ushort)(SUB161(auVar25 >> 0x27,0) & 1) << 4 |
           (ushort)(SUB161(auVar25 >> 0x2f,0) & 1) << 5 |
           (ushort)(SUB161(auVar25 >> 0x37,0) & 1) << 6 |
           (ushort)(SUB161(auVar25 >> 0x3f,0) & 1) << 7 |
           (ushort)(SUB161(auVar25 >> 0x47,0) & 1) << 8 |
           (ushort)(SUB161(auVar25 >> 0x4f,0) & 1) << 9 |
           (ushort)(SUB161(auVar25 >> 0x57,0) & 1) << 10 |
           (ushort)(SUB161(auVar25 >> 0x5f,0) & 1) << 0xb |
           (ushort)(SUB161(auVar25 >> 0x67,0) & 1) << 0xc |
           (ushort)(SUB161(auVar25 >> 0x6f,0) & 1) << 0xd |
           (ushort)(SUB161(auVar25 >> 0x77,0) & 1) << 0xe | (ushort)(auVar25[0xf] >> 7) << 0xf;
  uVar12 = (uint)uVar13;
  while (uVar13 != 0) {
    uVar5 = 0;
    for (uVar6 = uVar12; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
      uVar5 = uVar5 + 1;
    }
    lVar10 = (uVar5 + uVar9 & *(ulonglong *)(local_58 + 0x1a8)) * -0xc0;
    if (local_50 == *(longlong *)(lVar3 + -0xc0 + lVar10)) {
      *(undefined4 *)(lVar3 + -0x10 + lVar10) = 0x42c80000;
      plVar4 = (longlong *)*in_RDX;
      uVar7 = extraout_RAX_00 + 1 ^ 0xb1695320004fb443;
      uVar9 = uVar7;
      core::MeiContext::reuse_or_create_node();
      *(undefined1 *)(extraout_RAX_03 + 0x5d) = 0;
      if ((uVar9 & 1) == 0) {
        lVar3 = plVar4[2];
        if (lVar3 == *plVar4) {
          alloc::raw_vec::RawVec::grow_one<enum2_<iced_core::event::Event>,alloc::alloc::Global>();
        }
        lVar11 = plVar4[1];
        lVar10 = lVar3 * 0x38;
        *(undefined8 *)(lVar11 + lVar10) = 3;
        *(longlong *)(lVar11 + 0x10 + lVar10) = lVar8;
        puVar2 = (undefined8 *)(lVar11 + 0x18 + lVar10);
        *puVar2 = 0x40a00000;
        puVar2[1] = 0x3f800000;
        *(undefined8 *)(lVar11 + 0x28 + lVar10) = 0x80000000bf800000;
        *(ulonglong *)(lVar11 + 0x30 + lVar10) = uVar7;
        plVar4[2] = lVar3 + 1;
      }
      widgets::impl$26::image<ref_<str_>_>();
      if (*(longlong *)(local_58 + 0x1b8) == 0) goto LAB_14067e34f;
      ::core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      lVar3 = *(longlong *)(local_58 + 0x1a0);
      bVar15 = (byte)(extraout_RAX_04 >> 0x39);
      auVar17 = ZEXT216(CONCAT11(bVar15,bVar15));
      auVar17 = pshuflw(auVar17,auVar17,0);
      lVar11 = 0;
      uVar9 = extraout_RAX_04;
      goto LAB_14067e2f0;
    }
    uVar13 = (ushort)(uVar12 - 1) & (ushort)uVar12;
    uVar12 = CONCAT22((short)(uVar12 - 1 >> 0x10),uVar13);
  }
  auVar22[0] = -(*pcVar1 == -1);
  auVar22[1] = -(pcVar1[1] == -1);
  auVar22[2] = -(pcVar1[2] == -1);
  auVar22[3] = -(pcVar1[3] == -1);
  auVar22[4] = -(pcVar1[4] == -1);
  auVar22[5] = -(pcVar1[5] == -1);
  auVar22[6] = -(pcVar1[6] == -1);
  auVar22[7] = -(pcVar1[7] == -1);
  auVar22[8] = -(pcVar1[8] == -1);
  auVar22[9] = -(pcVar1[9] == -1);
  auVar22[10] = -(pcVar1[10] == -1);
  auVar22[0xb] = -(pcVar1[0xb] == -1);
  auVar22[0xc] = -(pcVar1[0xc] == -1);
  auVar22[0xd] = -(pcVar1[0xd] == -1);
  auVar22[0xe] = -(pcVar1[0xe] == -1);
  auVar22[0xf] = -(pcVar1[0xf] == -1);
  if ((((((((((((((((SUB161(auVar22 >> 7,0) & 1) != 0 || (SUB161(auVar22 >> 0xf,0) & 1) != 0) ||
                  (SUB161(auVar22 >> 0x17,0) & 1) != 0) || (SUB161(auVar22 >> 0x1f,0) & 1) != 0) ||
                (SUB161(auVar22 >> 0x27,0) & 1) != 0) || (SUB161(auVar22 >> 0x2f,0) & 1) != 0) ||
              (SUB161(auVar22 >> 0x37,0) & 1) != 0) || (SUB161(auVar22 >> 0x3f,0) & 1) != 0) ||
            (SUB161(auVar22 >> 0x47,0) & 1) != 0) || (SUB161(auVar22 >> 0x4f,0) & 1) != 0) ||
          (SUB161(auVar22 >> 0x57,0) & 1) != 0) || (SUB161(auVar22 >> 0x5f,0) & 1) != 0) ||
        (SUB161(auVar22 >> 0x67,0) & 1) != 0) || (SUB161(auVar22 >> 0x6f,0) & 1) != 0) ||
      (SUB161(auVar22 >> 0x77,0) & 1) != 0) || auVar22[0xf] < '\0') goto LAB_14067e34f;
  uVar9 = uVar9 + lVar11 + 0x10;
  lVar11 = lVar11 + 0x10;
  goto LAB_14067e152;
LAB_14067e2f0:
  uVar9 = *(ulonglong *)(local_58 + 0x1a8) & uVar9;
  pcVar1 = (char *)(lVar3 + uVar9);
  cVar16 = auVar17[0];
  auVar26[0] = -(*pcVar1 == cVar16);
  cVar18 = auVar17[1];
  auVar26[1] = -(pcVar1[1] == cVar18);
  cVar19 = auVar17[2];
  auVar26[2] = -(pcVar1[2] == cVar19);
  cVar20 = auVar17[3];
  auVar26[3] = -(pcVar1[3] == cVar20);
  auVar26[4] = -(pcVar1[4] == cVar16);
  auVar26[5] = -(pcVar1[5] == cVar18);
  auVar26[6] = -(pcVar1[6] == cVar19);
  auVar26[7] = -(pcVar1[7] == cVar20);
  auVar26[8] = -(pcVar1[8] == cVar16);
  auVar26[9] = -(pcVar1[9] == cVar18);
  auVar26[10] = -(pcVar1[10] == cVar19);
  auVar26[0xb] = -(pcVar1[0xb] == cVar20);
  auVar26[0xc] = -(pcVar1[0xc] == cVar16);
  auVar26[0xd] = -(pcVar1[0xd] == cVar18);
  auVar26[0xe] = -(pcVar1[0xe] == cVar19);
  auVar26[0xf] = -(pcVar1[0xf] == cVar20);
  uVar13 = (ushort)(SUB161(auVar26 >> 7,0) & 1) | (ushort)(SUB161(auVar26 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar26 >> 0x17,0) & 1) << 2 |
           (ushort)(SUB161(auVar26 >> 0x1f,0) & 1) << 3 |
           (ushort)(SUB161(auVar26 >> 0x27,0) & 1) << 4 |
           (ushort)(SUB161(auVar26 >> 0x2f,0) & 1) << 5 |
           (ushort)(SUB161(auVar26 >> 0x37,0) & 1) << 6 |
           (ushort)(SUB161(auVar26 >> 0x3f,0) & 1) << 7 |
           (ushort)(SUB161(auVar26 >> 0x47,0) & 1) << 8 |
           (ushort)(SUB161(auVar26 >> 0x4f,0) & 1) << 9 |
           (ushort)(SUB161(auVar26 >> 0x57,0) & 1) << 10 |
           (ushort)(SUB161(auVar26 >> 0x5f,0) & 1) << 0xb |
           (ushort)(SUB161(auVar26 >> 0x67,0) & 1) << 0xc |
           (ushort)(SUB161(auVar26 >> 0x6f,0) & 1) << 0xd |
           (ushort)(SUB161(auVar26 >> 0x77,0) & 1) << 0xe | (ushort)(auVar26[0xf] >> 7) << 0xf;
  uVar12 = (uint)uVar13;
  while (uVar13 != 0) {
    uVar5 = 0;
    for (uVar6 = uVar12; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
      uVar5 = uVar5 + 1;
    }
    lVar8 = (uVar5 + uVar9 & *(ulonglong *)(local_58 + 0x1a8)) * -0xc0;
    if (local_50 == *(longlong *)(lVar3 + -0xc0 + lVar8)) {
      *(undefined4 *)(lVar3 + -0x10 + lVar8) = 0x42c80000;
LAB_14067e363:
      in_RCX[2] = in_RDX[2];
      lVar3 = in_RDX[1];
      *in_RCX = *in_RDX;
      in_RCX[1] = lVar3;
      return;
    }
    uVar13 = (ushort)(uVar12 - 1) & (ushort)uVar12;
    uVar12 = CONCAT22((short)(uVar12 - 1 >> 0x10),uVar13);
  }
  auVar23[0] = -(*pcVar1 == -1);
  auVar23[1] = -(pcVar1[1] == -1);
  auVar23[2] = -(pcVar1[2] == -1);
  auVar23[3] = -(pcVar1[3] == -1);
  auVar23[4] = -(pcVar1[4] == -1);
  auVar23[5] = -(pcVar1[5] == -1);
  auVar23[6] = -(pcVar1[6] == -1);
  auVar23[7] = -(pcVar1[7] == -1);
  auVar23[8] = -(pcVar1[8] == -1);
  auVar23[9] = -(pcVar1[9] == -1);
  auVar23[10] = -(pcVar1[10] == -1);
  auVar23[0xb] = -(pcVar1[0xb] == -1);
  auVar23[0xc] = -(pcVar1[0xc] == -1);
  auVar23[0xd] = -(pcVar1[0xd] == -1);
  auVar23[0xe] = -(pcVar1[0xe] == -1);
  auVar23[0xf] = -(pcVar1[0xf] == -1);
  if ((((((((((((((((SUB161(auVar23 >> 7,0) & 1) != 0 || (SUB161(auVar23 >> 0xf,0) & 1) != 0) ||
                  (SUB161(auVar23 >> 0x17,0) & 1) != 0) || (SUB161(auVar23 >> 0x1f,0) & 1) != 0) ||
                (SUB161(auVar23 >> 0x27,0) & 1) != 0) || (SUB161(auVar23 >> 0x2f,0) & 1) != 0) ||
              (SUB161(auVar23 >> 0x37,0) & 1) != 0) || (SUB161(auVar23 >> 0x3f,0) & 1) != 0) ||
            (SUB161(auVar23 >> 0x47,0) & 1) != 0) || (SUB161(auVar23 >> 0x4f,0) & 1) != 0) ||
          (SUB161(auVar23 >> 0x57,0) & 1) != 0) || (SUB161(auVar23 >> 0x5f,0) & 1) != 0) ||
        (SUB161(auVar23 >> 0x67,0) & 1) != 0) || (SUB161(auVar23 >> 0x6f,0) & 1) != 0) ||
      (SUB161(auVar23 >> 0x77,0) & 1) != 0) || auVar23[0xf] < '\0') {
LAB_14067e34f:
                    // WARNING: Subroutine does not return
    ::core::option::unwrap_failed();
  }
  uVar9 = uVar9 + lVar11 + 0x10;
  lVar11 = lVar11 + 0x10;
  goto LAB_14067e2f0;
}

