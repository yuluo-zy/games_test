// snapping_init addr=0x140ada600 size=668 sig=void __cdecl snapping_init(void)

void __cdecl country_core::systems::wall::snapping::snapping_init::snapping_init(void)

{
  char *pcVar1;
  ulonglong uVar2;
  ulonglong uVar3;
  undefined4 uVar4;
  undefined4 *puVar5;
  undefined1 *puVar6;
  longlong *plVar7;
  longlong lVar8;
  ulonglong *puVar9;
  uint uVar10;
  uint uVar11;
  ulonglong extraout_RAX;
  undefined8 *in_RCX;
  longlong *in_RDX;
  longlong *in_R8;
  ulonglong uVar12;
  longlong *in_R9;
  longlong lVar13;
  longlong lVar14;
  uint uVar15;
  ushort uVar16;
  longlong lVar17;
  byte bVar18;
  char cVar19;
  float fVar20;
  char cVar26;
  ulonglong uVar21;
  char cVar24;
  char cVar25;
  undefined1 auVar22 [16];
  float fVar27;
  undefined1 auVar23 [16];
  ulonglong uVar28;
  undefined1 auVar29 [16];
  float fVar30;
  float fVar33;
  undefined1 auVar31 [16];
  undefined1 auVar32 [16];
  float fVar34;
  undefined1 auVar35 [16];
  float fVar36;
  float fVar37;
  float fVar38;
  float fVar39;
  
  uVar4 = *(undefined4 *)((longlong)in_RCX + 0x1c);
  puVar5 = (undefined4 *)in_RCX[2];
  *puVar5 = uVar4;
  puVar6 = (undefined1 *)*in_RCX;
  *puVar6 = 0;
  if (*(char *)(*in_R9 + 0xfd) != '\0') {
    return;
  }
  if (SBORROW8(0,*(longlong *)*in_R8)) {
    return;
  }
  plVar7 = (longlong *)*in_RDX;
  lVar8 = ((longlong *)*in_R8)[0x10];
  if (plVar7[3] != 0) {
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    lVar14 = *plVar7;
    bVar18 = (byte)(extraout_RAX >> 0x39);
    auVar22 = ZEXT216(CONCAT11(bVar18,bVar18));
    auVar22 = pshuflw(auVar22,auVar22,0);
    lVar13 = 0;
    uVar12 = extraout_RAX;
    while( true ) {
      uVar12 = plVar7[1] & uVar12;
      pcVar1 = (char *)(lVar14 + uVar12);
      cVar19 = auVar22[0];
      auVar35[0] = -(*pcVar1 == cVar19);
      cVar24 = auVar22[1];
      auVar35[1] = -(pcVar1[1] == cVar24);
      cVar25 = auVar22[2];
      auVar35[2] = -(pcVar1[2] == cVar25);
      cVar26 = auVar22[3];
      auVar35[3] = -(pcVar1[3] == cVar26);
      auVar35[4] = -(pcVar1[4] == cVar19);
      auVar35[5] = -(pcVar1[5] == cVar24);
      auVar35[6] = -(pcVar1[6] == cVar25);
      auVar35[7] = -(pcVar1[7] == cVar26);
      auVar35[8] = -(pcVar1[8] == cVar19);
      auVar35[9] = -(pcVar1[9] == cVar24);
      auVar35[10] = -(pcVar1[10] == cVar25);
      auVar35[0xb] = -(pcVar1[0xb] == cVar26);
      auVar35[0xc] = -(pcVar1[0xc] == cVar19);
      auVar35[0xd] = -(pcVar1[0xd] == cVar24);
      auVar35[0xe] = -(pcVar1[0xe] == cVar25);
      auVar35[0xf] = -(pcVar1[0xf] == cVar26);
      uVar16 = (ushort)(SUB161(auVar35 >> 7,0) & 1) | (ushort)(SUB161(auVar35 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar35 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar35 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar35 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar35 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar35 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar35 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar35 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar35 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar35 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar35 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar35 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar35 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar35 >> 0x77,0) & 1) << 0xe | (ushort)(auVar35[0xf] >> 7) << 0xf;
      uVar15 = (uint)uVar16;
      while (uVar16 != 0) {
        uVar10 = 0;
        for (uVar11 = uVar15; (uVar11 & 1) == 0; uVar11 = uVar11 >> 1 | 0x80000000) {
          uVar10 = uVar10 + 1;
        }
        lVar17 = (uVar10 + uVar12 & plVar7[1]) * -0x78;
        if (*(longlong *)(lVar14 + -0x78 + lVar17) == lVar8) {
          if (*(float *)(lVar14 + -0x40 + lVar17) <= 1.6500001) {
            return;
          }
          puVar9 = *(ulonglong **)(lVar14 + lVar17 + -0x68);
          uVar12 = *(ulonglong *)(lVar14 + lVar17 + -0x60);
          uVar21 = 0;
          uVar28 = 0;
          if (uVar12 < 2) goto LAB_140ada7de;
          uVar28 = *puVar9;
          auVar32._8_8_ = 0;
          auVar32._0_8_ = uVar28;
          uVar21 = puVar9[1];
          auVar22._8_8_ = 0;
          auVar22._0_8_ = uVar21;
          fVar20 = (float)uVar28 - (float)uVar21;
          fVar27 = (float)(uVar28 >> 0x20) - (float)(uVar21 >> 0x20);
          fVar20 = fVar27 * fVar27 + fVar20 * fVar20;
          if (fVar20 <= 0.0) {
            auVar22 = ZEXT416(0);
            auVar32 = ZEXT416(0);
          }
          uVar21 = auVar22._0_8_;
          uVar28 = auVar32._0_8_;
          if (uVar12 == 2) goto LAB_140ada7de;
          if (fVar20 <= 0.0) {
            fVar20 = 0.0;
          }
          lVar8 = uVar12 * 8 + -0x10;
          lVar14 = 0;
          goto LAB_140ada796;
        }
        uVar16 = (ushort)(uVar15 - 1) & (ushort)uVar15;
        uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar16);
      }
      auVar31[0] = -(*pcVar1 == -1);
      auVar31[1] = -(pcVar1[1] == -1);
      auVar31[2] = -(pcVar1[2] == -1);
      auVar31[3] = -(pcVar1[3] == -1);
      auVar31[4] = -(pcVar1[4] == -1);
      auVar31[5] = -(pcVar1[5] == -1);
      auVar31[6] = -(pcVar1[6] == -1);
      auVar31[7] = -(pcVar1[7] == -1);
      auVar31[8] = -(pcVar1[8] == -1);
      auVar31[9] = -(pcVar1[9] == -1);
      auVar31[10] = -(pcVar1[10] == -1);
      auVar31[0xb] = -(pcVar1[0xb] == -1);
      auVar31[0xc] = -(pcVar1[0xc] == -1);
      auVar31[0xd] = -(pcVar1[0xd] == -1);
      auVar31[0xe] = -(pcVar1[0xe] == -1);
      auVar31[0xf] = -(pcVar1[0xf] == -1);
      if ((((((((((((((((SUB161(auVar31 >> 7,0) & 1) != 0 || (SUB161(auVar31 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar31 >> 0x17,0) & 1) != 0) || (SUB161(auVar31 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar31 >> 0x27,0) & 1) != 0) || (SUB161(auVar31 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar31 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar31 >> 0x3f,0) & 1) != 0) || (SUB161(auVar31 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar31 >> 0x4f,0) & 1) != 0) || (SUB161(auVar31 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar31 >> 0x5f,0) & 1) != 0) || (SUB161(auVar31 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar31 >> 0x6f,0) & 1) != 0) || (SUB161(auVar31 >> 0x77,0) & 1) != 0) ||
          auVar31[0xf] < '\0') break;
      uVar12 = uVar12 + lVar13 + 0x10;
      lVar13 = lVar13 + 0x10;
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
  while( true ) {
    uVar2 = *(ulonglong *)((longlong)puVar9 + lVar14 + 8);
    auVar29._8_8_ = 0;
    auVar29._0_8_ = uVar2;
    uVar3 = *(ulonglong *)((longlong)puVar9 + lVar14 + 0x10);
    auVar23._8_8_ = 0;
    auVar23._0_8_ = uVar3;
    fVar27 = (float)uVar2 - (float)uVar3;
    fVar37 = (float)(uVar2 >> 0x20) - (float)(uVar3 >> 0x20);
    fVar27 = fVar37 * fVar37 + fVar27 * fVar27;
    if (fVar27 <= fVar20) {
      fVar27 = fVar20;
      uVar2 = uVar28;
      uVar3 = uVar21;
      auVar23 = auVar22;
      auVar29 = auVar32;
    }
    auVar32 = auVar29;
    auVar22 = auVar23;
    uVar21 = uVar3;
    uVar28 = uVar2;
    fVar20 = fVar27;
    lVar14 = lVar14 + 8;
    if (lVar8 == lVar14) break;
LAB_140ada796:
    uVar21 = auVar22._0_8_;
    uVar28 = auVar32._0_8_;
    if ((uVar12 * 8 + -8 == lVar14) || (lVar8 == lVar14)) break;
  }
LAB_140ada7de:
  fVar20 = (float)(uVar21 >> 0x20);
  fVar27 = (float)uVar28;
  fVar30 = (float)uVar21 - fVar27;
  fVar37 = (float)(uVar28 >> 0x20);
  fVar33 = fVar20 - fVar37;
  fVar34 = fVar33 * fVar33 + fVar30 * fVar30;
  if (fVar34 <= 1e-20) {
    fVar34 = 1e-20;
  }
  lVar8 = 0;
  do {
    lVar14 = lVar8;
    if (uVar12 << 3 == lVar14) break;
    fVar36 = (float)*(undefined8 *)((longlong)puVar9 + lVar14);
    fVar38 = (float)((ulonglong)*(undefined8 *)((longlong)puVar9 + lVar14) >> 0x20);
    fVar39 = ((fVar38 - fVar37) * fVar33 + (fVar36 - fVar27) * fVar30) / fVar34;
    fVar36 = fVar36 - (fVar39 * (float)uVar21 + (1.0 - fVar39) * fVar27);
    fVar38 = fVar38 - (fVar39 * fVar20 + (1.0 - fVar39) * fVar37);
    lVar8 = lVar14 + 8;
  } while (SQRT(fVar38 * fVar38 + fVar36 * fVar36) <= 1.5);
  *puVar5 = uVar4;
  *puVar6 = 1;
  puVar6[1] = uVar12 << 3 != lVar14;
  return;
}

