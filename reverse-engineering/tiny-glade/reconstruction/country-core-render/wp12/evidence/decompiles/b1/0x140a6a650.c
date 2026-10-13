// snapping_signifiers addr=0x140a6a650 size=1676 sig=void __cdecl snapping_signifiers(void)

void __cdecl country_core::systems::wall::snapping::snapping_maintain::snapping_signifiers(void)

{
  char *pcVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  uint *puVar4;
  longlong *plVar5;
  uint uVar6;
  uint uVar7;
  bool bVar8;
  ulonglong extraout_RAX;
  undefined8 *in_RCX;
  longlong *in_RDX;
  ulonglong uVar9;
  longlong lVar10;
  longlong lVar11;
  longlong lVar12;
  longlong *in_R8;
  uint *puVar13;
  longlong lVar14;
  uint uVar15;
  longlong *in_R9;
  ulonglong uVar16;
  undefined4 *puVar17;
  ushort uVar18;
  longlong lVar19;
  longlong lVar20;
  longlong lVar21;
  undefined4 *puVar22;
  byte bVar23;
  char cVar24;
  char cVar26;
  char cVar27;
  char cVar28;
  undefined1 auVar25 [16];
  undefined1 auVar29 [16];
  undefined1 auVar30 [16];
  undefined8 local_e8;
  undefined4 uStack_e0;
  longlong local_58;
  
  lVar12 = *in_R8;
  lVar14 = in_R8[1];
  puVar4 = *(uint **)(lVar12 + 0xf8);
  lVar19 = *(longlong *)(lVar12 + 0x100);
  pcVar1 = (char *)(lVar12 + 0x108);
  lVar10 = lVar19 * 4;
  lVar11 = 0;
  puVar13 = puVar4;
  if (*pcVar1 == '\0') {
    do {
      if (lVar10 == 0) goto LAB_140a6a7d9;
      uVar15 = *puVar13;
      puVar13 = puVar13 + 1;
      lVar21 = *(longlong *)(*(longlong *)(lVar14 + 0x100) + 0x58 + (ulonglong)uVar15 * 0xa0);
      lVar10 = lVar10 + -4;
    } while (lVar21 == 0);
    lVar11 = *(longlong *)(lVar14 + 0x100) + (ulonglong)uVar15 * 0xa0;
    uVar9 = (ulonglong)*(uint *)(lVar11 + 0x94);
    lVar10 = *(longlong *)(*(longlong *)(lVar14 + 0x1a8) + 0x18 + uVar9 * 0x48);
    uVar16 = (ulonglong)*(uint *)(*(longlong *)(lVar11 + 0x50) + 8);
    lVar11 = ~*(ulonglong *)
               (*(longlong *)(*(longlong *)(lVar14 + 0x1a8) + 0x38 + uVar9 * 0x48) +
               *(longlong *)(lVar12 + 0x118) * 8) * 0x30;
    lVar12 = uVar16 * 0x30 + *(longlong *)(lVar10 + 0x10 + lVar11);
    puVar17 = (undefined4 *)(uVar16 * 4 + *(longlong *)(lVar10 + 0x28 + lVar11));
  }
  else {
    do {
      if (lVar10 == 0) goto LAB_140a6a7d9;
      uVar15 = *puVar13;
      puVar13 = puVar13 + 1;
      lVar21 = *(longlong *)(*(longlong *)(lVar14 + 0x1a8) + 0x10 + (ulonglong)uVar15 * 0x48);
      lVar10 = lVar10 + -4;
    } while (lVar21 == 0);
    lVar11 = (ulonglong)uVar15 * 0x48 + *(longlong *)(lVar14 + 0x1a8);
    lVar10 = *(longlong *)(lVar11 + 0x18);
    lVar11 = ~*(ulonglong *)(*(longlong *)(lVar11 + 0x38) + *(longlong *)(lVar12 + 0x118) * 8) *
             0x30;
    lVar12 = *(longlong *)(lVar10 + 0x10 + lVar11);
    puVar17 = *(undefined4 **)(lVar10 + 0x28 + lVar11);
  }
  lVar11 = 1;
  bVar8 = false;
  uVar2 = *(undefined4 *)((longlong)in_R8 + 0x14);
joined_r0x000140a6a7ea:
  if (*pcVar1 == '\0') {
    if (lVar11 == lVar21) {
      do {
        if (puVar13 == puVar4 + lVar19) goto LAB_140a6a84d;
        uVar15 = *puVar13;
        puVar13 = puVar13 + 1;
      } while (*(longlong *)(*(longlong *)(lVar14 + 0x100) + 0x58 + (ulonglong)uVar15 * 0xa0) == 0);
    }
  }
  else if (lVar11 == lVar21) {
    do {
      if (puVar13 == puVar4 + lVar19) goto LAB_140a6a84d;
      uVar15 = *puVar13;
      puVar13 = puVar13 + 1;
    } while (*(longlong *)(*(longlong *)(lVar14 + 0x1a8) + 0x10 + (ulonglong)uVar15 * 0x48) == 0);
  }
  goto LAB_140a6aa28;
LAB_140a6a7d9:
  puVar17 = (undefined4 *)0x0;
  bVar8 = true;
  lVar21 = 0;
  uVar2 = *(undefined4 *)((longlong)in_R8 + 0x14);
  goto joined_r0x000140a6a7ea;
LAB_140a6a84d:
  if (bVar8) goto LAB_140a6aa28;
  *puVar17 = uVar2;
  *(undefined4 *)(lVar12 + 0x24) = 0;
  *(undefined8 *)(lVar12 + 0x1c) = 0;
  lVar14 = *in_RDX;
  lVar19 = in_RDX[1];
  puVar4 = *(uint **)(lVar14 + 0xf8);
  lVar11 = *(longlong *)(lVar14 + 0x100);
  pcVar1 = (char *)(lVar14 + 0x108);
  lVar10 = lVar11 * 4;
  lVar21 = 0;
  puVar13 = puVar4;
  if (*pcVar1 == '\0') {
    do {
      if (lVar10 == 0) goto LAB_140a6a9c9;
      uVar15 = *puVar13;
      puVar13 = puVar13 + 1;
      lVar20 = *(longlong *)(*(longlong *)(lVar19 + 0x100) + 0x58 + (ulonglong)uVar15 * 0xa0);
      lVar10 = lVar10 + -4;
    } while (lVar20 == 0);
    lVar21 = *(longlong *)(lVar19 + 0x100) + (ulonglong)uVar15 * 0xa0;
    uVar9 = (ulonglong)*(uint *)(lVar21 + 0x94);
    lVar10 = *(longlong *)(*(longlong *)(lVar19 + 0x1a8) + 0x18 + uVar9 * 0x48);
    uVar16 = (ulonglong)*(uint *)(*(longlong *)(lVar21 + 0x50) + 8);
    lVar21 = ~*(ulonglong *)
               (*(longlong *)(*(longlong *)(lVar19 + 0x1a8) + 0x38 + uVar9 * 0x48) +
               *(longlong *)(lVar14 + 0x118) * 8) * 0x30;
    lVar14 = uVar16 * 0x30 + *(longlong *)(lVar10 + 0x10 + lVar21);
    puVar22 = (undefined4 *)(uVar16 * 4 + *(longlong *)(lVar10 + 0x28 + lVar21));
  }
  else {
    do {
      if (lVar10 == 0) goto LAB_140a6a9c9;
      uVar15 = *puVar13;
      puVar13 = puVar13 + 1;
      lVar20 = *(longlong *)(*(longlong *)(lVar19 + 0x1a8) + 0x10 + (ulonglong)uVar15 * 0x48);
      lVar10 = lVar10 + -4;
    } while (lVar20 == 0);
    lVar10 = *(longlong *)(lVar19 + 0x1a8) + (ulonglong)uVar15 * 0x48;
    lVar21 = *(longlong *)(lVar10 + 0x18);
    lVar10 = ~*(ulonglong *)(*(longlong *)(lVar10 + 0x38) + *(longlong *)(lVar14 + 0x118) * 8) *
             0x30;
    lVar14 = *(longlong *)(lVar21 + 0x10 + lVar10);
    puVar22 = *(undefined4 **)(lVar21 + 0x28 + lVar10);
  }
  lVar21 = 1;
  bVar8 = false;
  uVar3 = *(undefined4 *)((longlong)in_RDX + 0x14);
  goto joined_r0x000140a6a9d9;
LAB_140a6aa4a:
  if (!bVar8) {
    *puVar22 = uVar3;
    *(undefined4 *)(lVar14 + 0x24) = 0;
    *(undefined8 *)(lVar14 + 0x1c) = 0;
    bVar23 = *(byte *)*in_RCX;
    if (1 < bVar23) {
      if (bVar23 == 2) {
        *puVar17 = uVar2;
        *(undefined8 *)(lVar12 + 0x1c) = 0x3fc000003fc00000;
        *(undefined4 *)(lVar12 + 0x24) = 0x3fc00000;
        ::utils::swizzlers::impl$1::xvy();
        *puVar17 = uVar2;
        *(undefined4 *)(lVar12 + 0x18) = uStack_e0;
        *(undefined8 *)(lVar12 + 0x10) = local_e8;
      }
      else {
        plVar5 = (longlong *)*in_R9;
        lVar10 = *(longlong *)((byte *)*in_RCX + 0x10);
        if (plVar5[3] != 0) {
          core::hash::BuildHasher::
          hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
          lVar12 = *plVar5;
          bVar23 = (byte)(extraout_RAX >> 0x39);
          auVar25 = ZEXT216(CONCAT11(bVar23,bVar23));
          auVar25 = pshuflw(auVar25,auVar25,0);
          lVar14 = 0;
          uVar9 = extraout_RAX;
          while( true ) {
            uVar9 = plVar5[1] & uVar9;
            pcVar1 = (char *)(lVar12 + uVar9);
            cVar24 = auVar25[0];
            auVar30[0] = -(*pcVar1 == cVar24);
            cVar26 = auVar25[1];
            auVar30[1] = -(pcVar1[1] == cVar26);
            cVar27 = auVar25[2];
            auVar30[2] = -(pcVar1[2] == cVar27);
            cVar28 = auVar25[3];
            auVar30[3] = -(pcVar1[3] == cVar28);
            auVar30[4] = -(pcVar1[4] == cVar24);
            auVar30[5] = -(pcVar1[5] == cVar26);
            auVar30[6] = -(pcVar1[6] == cVar27);
            auVar30[7] = -(pcVar1[7] == cVar28);
            auVar30[8] = -(pcVar1[8] == cVar24);
            auVar30[9] = -(pcVar1[9] == cVar26);
            auVar30[10] = -(pcVar1[10] == cVar27);
            auVar30[0xb] = -(pcVar1[0xb] == cVar28);
            auVar30[0xc] = -(pcVar1[0xc] == cVar24);
            auVar30[0xd] = -(pcVar1[0xd] == cVar26);
            auVar30[0xe] = -(pcVar1[0xe] == cVar27);
            auVar30[0xf] = -(pcVar1[0xf] == cVar28);
            uVar18 = (ushort)(SUB161(auVar30 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar30 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar30 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar30 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar30 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar30 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar30 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar30 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar30 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar30 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar30 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar30 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar30 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar30 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar30 >> 0x77,0) & 1) << 0xe |
                     (ushort)(auVar30[0xf] >> 7) << 0xf;
            uVar15 = (uint)uVar18;
            while (uVar18 != 0) {
              uVar6 = 0;
              for (uVar7 = uVar15; (uVar7 & 1) == 0; uVar7 = uVar7 >> 1 | 0x80000000) {
                uVar6 = uVar6 + 1;
              }
              lVar19 = (uVar6 + uVar9 & plVar5[1]) * -0x2a8;
              if (*(longlong *)(lVar12 + -0x2a8 + lVar19) == lVar10) {
                if ((1 < *(ulonglong *)(lVar12 + -0x290 + lVar19)) &&
                   (0.0 < *(float *)(lVar12 + -0x270 + lVar19))) {
                  ::utils::swizzlers::impl$1::xvy();
                  shapes::shape_placement_signifiers::preview_snapping_line();
                  if (SBORROW8(0,local_58)) {
                    return;
                  }
                  country_core::utils::debug_viz::DebugLine::show();
                  country_core::utils::debug_viz::DebugSphere::show();
                  return;
                }
                    // WARNING: Subroutine does not return
                core::panicking::panic();
              }
              uVar18 = (ushort)(uVar15 - 1) & (ushort)uVar15;
              uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar18);
            }
            auVar29[0] = -(*pcVar1 == -1);
            auVar29[1] = -(pcVar1[1] == -1);
            auVar29[2] = -(pcVar1[2] == -1);
            auVar29[3] = -(pcVar1[3] == -1);
            auVar29[4] = -(pcVar1[4] == -1);
            auVar29[5] = -(pcVar1[5] == -1);
            auVar29[6] = -(pcVar1[6] == -1);
            auVar29[7] = -(pcVar1[7] == -1);
            auVar29[8] = -(pcVar1[8] == -1);
            auVar29[9] = -(pcVar1[9] == -1);
            auVar29[10] = -(pcVar1[10] == -1);
            auVar29[0xb] = -(pcVar1[0xb] == -1);
            auVar29[0xc] = -(pcVar1[0xc] == -1);
            auVar29[0xd] = -(pcVar1[0xd] == -1);
            auVar29[0xe] = -(pcVar1[0xe] == -1);
            auVar29[0xf] = -(pcVar1[0xf] == -1);
            if ((((((((((((((((SUB161(auVar29 >> 7,0) & 1) != 0 ||
                             (SUB161(auVar29 >> 0xf,0) & 1) != 0) ||
                            (SUB161(auVar29 >> 0x17,0) & 1) != 0) ||
                           (SUB161(auVar29 >> 0x1f,0) & 1) != 0) ||
                          (SUB161(auVar29 >> 0x27,0) & 1) != 0) ||
                         (SUB161(auVar29 >> 0x2f,0) & 1) != 0) ||
                        (SUB161(auVar29 >> 0x37,0) & 1) != 0) ||
                       (SUB161(auVar29 >> 0x3f,0) & 1) != 0) || (SUB161(auVar29 >> 0x47,0) & 1) != 0
                      ) || (SUB161(auVar29 >> 0x4f,0) & 1) != 0) ||
                    (SUB161(auVar29 >> 0x57,0) & 1) != 0) || (SUB161(auVar29 >> 0x5f,0) & 1) != 0)
                  || (SUB161(auVar29 >> 0x67,0) & 1) != 0) || (SUB161(auVar29 >> 0x6f,0) & 1) != 0)
                || (SUB161(auVar29 >> 0x77,0) & 1) != 0) || auVar29[0xf] < '\0') break;
            uVar9 = uVar9 + lVar14 + 0x10;
            lVar14 = lVar14 + 0x10;
          }
        }
      }
    }
    return;
  }
  goto LAB_140a6aa28;
LAB_140a6a9c9:
  puVar22 = (undefined4 *)0x0;
  bVar8 = true;
  lVar20 = 0;
  uVar3 = *(undefined4 *)((longlong)in_RDX + 0x14);
joined_r0x000140a6a9d9:
  if (*pcVar1 == '\0') {
    if (lVar21 == lVar20) {
      do {
        if (puVar13 == puVar4 + lVar11) goto LAB_140a6aa4a;
        uVar15 = *puVar13;
        puVar13 = puVar13 + 1;
      } while (*(longlong *)(*(longlong *)(lVar19 + 0x100) + 0x58 + (ulonglong)uVar15 * 0xa0) == 0);
    }
  }
  else if (lVar21 == lVar20) {
    do {
      if (puVar13 == puVar4 + lVar11) goto LAB_140a6aa4a;
      uVar15 = *puVar13;
      puVar13 = puVar13 + 1;
    } while (*(longlong *)(*(longlong *)(lVar19 + 0x1a8) + 0x10 + (ulonglong)uVar15 * 0x48) == 0);
  }
LAB_140a6aa28:
                    // WARNING: Subroutine does not return
  core::result::unwrap_failed();
}

