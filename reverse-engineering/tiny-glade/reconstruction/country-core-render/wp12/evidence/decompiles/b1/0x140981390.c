// wall_highlight addr=0x140981390 size=572 sig=void __cdecl wall_highlight(void)

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl country_core::systems::wall::wall_highlight::wall_highlight(void)

{
  char *pcVar1;
  char cVar2;
  undefined4 *puVar3;
  undefined4 *puVar4;
  ulonglong *puVar5;
  ulonglong uVar6;
  longlong *plVar7;
  uint uVar8;
  uint uVar9;
  ulonglong extraout_RAX;
  longlong *in_RCX;
  ulonglong uVar10;
  longlong lVar11;
  longlong *in_RDX;
  ushort uVar12;
  undefined8 *in_R8;
  ulonglong uVar13;
  ulonglong uVar14;
  longlong *in_R9;
  longlong lVar15;
  longlong lVar16;
  uint uVar17;
  ulonglong uVar18;
  ulonglong uVar19;
  ulonglong uVar20;
  byte bVar21;
  char cVar22;
  char cVar27;
  undefined4 uVar23;
  char cVar25;
  char cVar26;
  undefined1 auVar24 [16];
  undefined1 auVar28 [16];
  undefined1 auVar29 [16];
  
  uVar23 = *(undefined4 *)((longlong)in_RCX + 0x1c);
  puVar3 = (undefined4 *)in_RCX[2];
  *puVar3 = uVar23;
  puVar4 = (undefined4 *)*in_RCX;
  *(undefined1 *)(puVar4 + 4) = 0;
  puVar5 = (ulonglong *)*in_R8;
  lVar11 = in_R8[1];
  uVar14 = *puVar5;
  uVar20 = 0;
  uVar18 = uVar14 - *(ulonglong *)(lVar11 + 0x18);
  if (uVar14 < *(ulonglong *)(lVar11 + 0x18)) {
    uVar18 = uVar20;
  }
  uVar10 = uVar14 - *(ulonglong *)(lVar11 + 0x38);
  if (uVar14 < *(ulonglong *)(lVar11 + 0x38)) {
    uVar10 = uVar20;
  }
  uVar14 = *(ulonglong *)(lVar11 + 0x10);
  uVar6 = *(ulonglong *)(lVar11 + 0x30);
  uVar13 = uVar6 - uVar10;
  if (uVar6 < uVar10) {
    uVar13 = uVar20;
  }
  uVar19 = uVar14 - uVar18;
  if (uVar14 < uVar18) {
    uVar19 = uVar20;
  }
  uVar20 = *(ulonglong *)(lVar11 + 0x40);
  lVar15 = *(longlong *)(lVar11 + 8);
  lVar11 = *(longlong *)(lVar11 + 0x28);
  *puVar5 = (uVar20 - uVar13) - uVar19;
  if (uVar14 >= uVar18 && uVar14 - uVar18 != 0 || uVar6 >= uVar10 && uVar6 - uVar10 != 0) {
    lVar15 = lVar15 + uVar18 * 0x18;
    if (uVar14 < uVar18) {
      lVar15 = 8;
    }
    lVar16 = 0;
    if (uVar18 < uVar14) {
      lVar16 = lVar15 + -0x18 + uVar19 * 0x18;
    }
    lVar11 = lVar11 + uVar10 * 0x18;
    if (uVar6 < uVar10) {
      lVar11 = 8;
    }
    lVar11 = lVar11 + -0x18 + uVar13 * 0x18;
    if (uVar6 <= uVar10) {
      lVar11 = lVar16;
    }
    *puVar5 = uVar20;
    lVar15 = *(longlong *)(lVar11 + 8);
    cVar2 = *(char *)(lVar11 + 0x10);
    plVar7 = (longlong *)*in_R9;
    if (plVar7[3] != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      lVar11 = *plVar7;
      bVar21 = (byte)(extraout_RAX >> 0x39);
      auVar24 = ZEXT216(CONCAT11(bVar21,bVar21));
      auVar24 = pshuflw(auVar24,auVar24,0);
      lVar16 = 0;
      uVar14 = extraout_RAX;
      while( true ) {
        uVar14 = plVar7[1] & uVar14;
        pcVar1 = (char *)(lVar11 + uVar14);
        cVar22 = auVar24[0];
        auVar29[0] = -(*pcVar1 == cVar22);
        cVar25 = auVar24[1];
        auVar29[1] = -(pcVar1[1] == cVar25);
        cVar26 = auVar24[2];
        auVar29[2] = -(pcVar1[2] == cVar26);
        cVar27 = auVar24[3];
        auVar29[3] = -(pcVar1[3] == cVar27);
        auVar29[4] = -(pcVar1[4] == cVar22);
        auVar29[5] = -(pcVar1[5] == cVar25);
        auVar29[6] = -(pcVar1[6] == cVar26);
        auVar29[7] = -(pcVar1[7] == cVar27);
        auVar29[8] = -(pcVar1[8] == cVar22);
        auVar29[9] = -(pcVar1[9] == cVar25);
        auVar29[10] = -(pcVar1[10] == cVar26);
        auVar29[0xb] = -(pcVar1[0xb] == cVar27);
        auVar29[0xc] = -(pcVar1[0xc] == cVar22);
        auVar29[0xd] = -(pcVar1[0xd] == cVar25);
        auVar29[0xe] = -(pcVar1[0xe] == cVar26);
        auVar29[0xf] = -(pcVar1[0xf] == cVar27);
        uVar12 = (ushort)(SUB161(auVar29 >> 7,0) & 1) | (ushort)(SUB161(auVar29 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar29 >> 0x17,0) & 1) << 2 |
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
                 (ushort)(SUB161(auVar29 >> 0x77,0) & 1) << 0xe | (ushort)(auVar29[0xf] >> 7) << 0xf
        ;
        uVar17 = (uint)uVar12;
        while (uVar12 != 0) {
          uVar8 = 0;
          for (uVar9 = uVar17; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
            uVar8 = uVar8 + 1;
          }
          uVar18 = uVar8 + uVar14 & plVar7[1];
          if (*(longlong *)(lVar11 + -0x10 + uVar18 * -0x10) == lVar15) {
            uVar14 = *(ulonglong *)(lVar11 + -8 + uVar18 * -0x10);
            if (uVar14 >> 0x20 == 0) {
              return;
            }
            *puVar3 = uVar23;
            *puVar4 = (int)uVar14;
            puVar4[1] = (int)(uVar14 >> 0x20);
            *(longlong *)(puVar4 + 2) = lVar15;
            *(char *)(puVar4 + 4) = cVar2;
            uVar23 = 0;
            if (cVar2 != '\x01') {
              uVar23 = 0x3d8f5c29;
            }
            *(undefined4 *)in_RDX[2] = *(undefined4 *)((longlong)in_RDX + 0x1c);
            uVar17 = (uint)(cVar2 == '\x01');
            uVar14 = CONCAT44((int)(uVar17 << 0x1f) >> 0x1f,(int)(uVar17 << 0x1f) >> 0x1f);
            puVar5 = (ulonglong *)*in_RDX;
            *puVar5 = uVar14 & __xmm_00000000000000003cf5c2903dcccccd |
                      ~uVar14 & (ulonglong)__xmm_00000000000000003da3d70b3d8f5c29;
            *(undefined4 *)(puVar5 + 1) = uVar23;
            return;
          }
          uVar12 = (ushort)(uVar17 - 1) & (ushort)uVar17;
          uVar17 = CONCAT22((short)(uVar17 - 1 >> 0x10),uVar12);
        }
        auVar28[0] = -(*pcVar1 == -1);
        auVar28[1] = -(pcVar1[1] == -1);
        auVar28[2] = -(pcVar1[2] == -1);
        auVar28[3] = -(pcVar1[3] == -1);
        auVar28[4] = -(pcVar1[4] == -1);
        auVar28[5] = -(pcVar1[5] == -1);
        auVar28[6] = -(pcVar1[6] == -1);
        auVar28[7] = -(pcVar1[7] == -1);
        auVar28[8] = -(pcVar1[8] == -1);
        auVar28[9] = -(pcVar1[9] == -1);
        auVar28[10] = -(pcVar1[10] == -1);
        auVar28[0xb] = -(pcVar1[0xb] == -1);
        auVar28[0xc] = -(pcVar1[0xc] == -1);
        auVar28[0xd] = -(pcVar1[0xd] == -1);
        auVar28[0xe] = -(pcVar1[0xe] == -1);
        auVar28[0xf] = -(pcVar1[0xf] == -1);
        if ((((((((((((((((SUB161(auVar28 >> 7,0) & 1) != 0 || (SUB161(auVar28 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar28 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar28 >> 0x1f,0) & 1) != 0) || (SUB161(auVar28 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar28 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar28 >> 0x37,0) & 1) != 0) || (SUB161(auVar28 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar28 >> 0x47,0) & 1) != 0) || (SUB161(auVar28 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar28 >> 0x57,0) & 1) != 0) || (SUB161(auVar28 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar28 >> 0x67,0) & 1) != 0) || (SUB161(auVar28 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar28 >> 0x77,0) & 1) != 0) || auVar28[0xf] < '\0') break;
        uVar14 = uVar14 + lVar16 + 0x10;
        lVar16 = lVar16 + 0x10;
      }
    }
  }
                    // WARNING: Read-only address (ram,0x000142ab6640) is written
                    // WARNING: Read-only address (ram,0x000142ab6650) is written
  return;
}

