
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140a29b10(undefined8 param_1,longlong param_2)

{
  int *piVar1;
  char *pcVar2;
  code *pcVar3;
  uint uVar4;
  uint uVar5;
  int iVar6;
  longlong lVar7;
  ulonglong uVar8;
  longlong lVar9;
  ulonglong uVar10;
  undefined8 *puVar11;
  longlong lVar12;
  uint uVar13;
  ushort uVar14;
  ulonglong uVar15;
  bool bVar16;
  undefined1 auVar17 [16];
  undefined1 auVar18 [16];
  undefined1 auVar19 [16];
  undefined1 auVar20 [16];
  undefined *puStack_3b8;
  undefined8 uStack_3b0;
  undefined8 uStack_3a8;
  longlong lStack_2c0;
  longlong lStack_2b8;
  undefined1 uStack_2b0;
  undefined1 auStack_2a8 [38];
  char cStack_282;
  longlong lStack_280;
  undefined8 auStack_220 [20];
  undefined8 uStack_180;
  longlong lStack_e0;
  longlong lStack_d8;
  longlong lStack_d0;
  longlong lStack_c8;
  undefined **ppuStack_c0;
  longlong lStack_b8;
  longlong lStack_b0;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined1 auStack_90 [16];
  undefined1 auStack_80 [16];
  undefined1 auStack_70 [16];
  undefined8 uStack_60;
  undefined1 uStack_58;
  undefined3 uStack_53;
  undefined1 auStack_50 [4];
  undefined4 uStack_4c;
  undefined8 uStack_48;
  undefined8 uStack_40;
  undefined1 uStack_32;
  undefined1 uStack_31;
  undefined8 uStack_30;
  
  uStack_30 = 0xfffffffffffffffe;
  puVar11 = (undefined8 *)0x65cfda6a55a841ac;
  if (*(longlong *)(param_2 + 0x88) != 0) {
    lVar7 = *(longlong *)(param_2 + 0x70);
    uVar10 = *(ulonglong *)(param_2 + 0x78);
    uVar8 = uVar10 & 0x65cfda6a55a841ac;
    lVar12 = 0;
    while( true ) {
      pcVar2 = (char *)(lVar7 + uVar8);
      auVar19[0] = -(*pcVar2 == UNK_142950410);
      auVar19[1] = -(pcVar2[1] == UNK_142950410._1_1_);
      auVar19[2] = -(pcVar2[2] == UNK_142950410._2_1_);
      auVar19[3] = -(pcVar2[3] == UNK_142950410._3_1_);
      auVar19[4] = -(pcVar2[4] == UNK_142950410._4_1_);
      auVar19[5] = -(pcVar2[5] == UNK_142950410._5_1_);
      auVar19[6] = -(pcVar2[6] == UNK_142950410._6_1_);
      auVar19[7] = -(pcVar2[7] == UNK_142950410._7_1_);
      auVar19[8] = -(pcVar2[8] == UNK_142950410._8_1_);
      auVar19[9] = -(pcVar2[9] == UNK_142950410._9_1_);
      auVar19[10] = -(pcVar2[10] == UNK_142950410._10_1_);
      auVar19[0xb] = -(pcVar2[0xb] == UNK_142950410._11_1_);
      auVar19[0xc] = -(pcVar2[0xc] == UNK_142950410._12_1_);
      auVar19[0xd] = -(pcVar2[0xd] == UNK_142950410._13_1_);
      auVar19[0xe] = -(pcVar2[0xe] == UNK_142950410._14_1_);
      auVar19[0xf] = -(pcVar2[0xf] == UNK_142950410._15_1_);
      uVar14 = (ushort)(SUB161(auVar19 >> 7,0) & 1) | (ushort)(SUB161(auVar19 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar19 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar19 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar19 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar19 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar19 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar19 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar19 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar19 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar19 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar19 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar19 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar19 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar19 >> 0x77,0) & 1) << 0xe | (ushort)(auVar19[0xf] >> 7) << 0xf;
      uVar13 = (uint)uVar14;
      while (uVar14 != 0) {
        uVar4 = 0;
        for (uVar5 = uVar13; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
          uVar4 = uVar4 + 1;
        }
        uVar15 = uVar4 + uVar8 & uVar10;
        if ((*(longlong *)(lVar7 + -0x18 + uVar15 * -0x18) == -0x7dd9338791986cab) &&
           (*(longlong *)(lVar7 + -0x10 + uVar15 * -0x18) == 0x65cfda6a55a841ac)) {
          puVar11 = *(undefined8 **)(lVar7 + uVar15 * -0x18 + -8);
          if (*(undefined8 **)(param_2 + 0x218) <= puVar11) goto code_r0x000140a2a05d;
          goto code_r0x000140a29cfc;
        }
        uVar14 = (ushort)(uVar13 - 1) & (ushort)uVar13;
        uVar13 = CONCAT22((short)(uVar13 - 1 >> 0x10),uVar14);
      }
      auVar17[0] = -(*pcVar2 == -1);
      auVar17[1] = -(pcVar2[1] == -1);
      auVar17[2] = -(pcVar2[2] == -1);
      auVar17[3] = -(pcVar2[3] == -1);
      auVar17[4] = -(pcVar2[4] == -1);
      auVar17[5] = -(pcVar2[5] == -1);
      auVar17[6] = -(pcVar2[6] == -1);
      auVar17[7] = -(pcVar2[7] == -1);
      auVar17[8] = -(pcVar2[8] == -1);
      auVar17[9] = -(pcVar2[9] == -1);
      auVar17[10] = -(pcVar2[10] == -1);
      auVar17[0xb] = -(pcVar2[0xb] == -1);
      auVar17[0xc] = -(pcVar2[0xc] == -1);
      auVar17[0xd] = -(pcVar2[0xd] == -1);
      auVar17[0xe] = -(pcVar2[0xe] == -1);
      auVar17[0xf] = -(pcVar2[0xf] == -1);
      if ((((((((((((((((SUB161(auVar17 >> 7,0) & 1) != 0 || (SUB161(auVar17 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar17 >> 0x17,0) & 1) != 0) || (SUB161(auVar17 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar17 >> 0x27,0) & 1) != 0) || (SUB161(auVar17 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar17 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar17 >> 0x3f,0) & 1) != 0) || (SUB161(auVar17 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar17 >> 0x4f,0) & 1) != 0) || (SUB161(auVar17 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar17 >> 0x5f,0) & 1) != 0) || (SUB161(auVar17 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar17 >> 0x6f,0) & 1) != 0) || (SUB161(auVar17 >> 0x77,0) & 1) != 0) ||
          auVar17[0xf] < '\0') break;
      lVar9 = uVar8 + lVar12;
      lVar12 = lVar12 + 0x10;
      uVar8 = lVar9 + 0x10U & uVar10;
    }
  }
  uVar13 = *(uint *)(param_2 + 0x90);
  if (uVar13 < 0x3ffffffe) {
    LOCK();
    bVar16 = uVar13 == *(uint *)(param_2 + 0x90);
    if (bVar16) {
      *(uint *)(param_2 + 0x90) = uVar13 + 1;
    }
    UNLOCK();
    if (bVar16) {
      lVar7 = *(longlong *)(param_2 + 0xf0);
      goto joined_r0x000140a2a03d;
    }
  }
  FUN_1428d8a30(param_2 + 0x90);
  lVar7 = *(longlong *)(param_2 + 0xf0);
joined_r0x000140a2a03d:
  if (lVar7 == 0) {
code_r0x000140a2a043:
    LOCK();
    piVar1 = (int *)(param_2 + 0x90);
    iVar6 = *piVar1;
    *piVar1 = *piVar1 + -1;
    UNLOCK();
    if (SBORROW4(0,iVar6 - 1U & 0xbfffffff)) {
      FUN_1428d8d40(param_2 + 0x90);
      FUN_1428d9310(&UNK_142acac90);
code_r0x000140a2a0dd:
      uStack_32 = 1;
      FUN_1428d9310(&UNK_142acace8);
    }
    else {
code_r0x000140a2a05d:
      FUN_1428d9310(&UNK_142acac90);
code_r0x000140a2a069:
      auStack_220[0] = 0;
      FUN_1428cab41(0,&puStack_3b8,puVar11 + 2,auStack_220,&UNK_142c2e180);
code_r0x000140a2a099:
      FUN_1428cf200();
    }
                    /* WARNING: Does not return */
    pcVar3 = (code *)invalidInstructionException();
    (*pcVar3)();
  }
  lVar7 = *(longlong *)(param_2 + 0xd8);
  uVar10 = *(ulonglong *)(param_2 + 0xe0);
  uVar8 = uVar10 & 0x65cfda6a55a841ac;
  lVar12 = 0;
code_r0x000140a29c51:
  auVar19 = *(undefined1 (*) [16])(lVar7 + uVar8);
  auVar20[0] = -(auVar19[0] == UNK_142950410);
  auVar20[1] = -(auVar19[1] == UNK_142950410._1_1_);
  auVar20[2] = -(auVar19[2] == UNK_142950410._2_1_);
  auVar20[3] = -(auVar19[3] == UNK_142950410._3_1_);
  auVar20[4] = -(auVar19[4] == UNK_142950410._4_1_);
  auVar20[5] = -(auVar19[5] == UNK_142950410._5_1_);
  auVar20[6] = -(auVar19[6] == UNK_142950410._6_1_);
  auVar20[7] = -(auVar19[7] == UNK_142950410._7_1_);
  auVar20[8] = -(auVar19[8] == UNK_142950410._8_1_);
  auVar20[9] = -(auVar19[9] == UNK_142950410._9_1_);
  auVar20[10] = -(auVar19[10] == UNK_142950410._10_1_);
  auVar20[0xb] = -(auVar19[0xb] == UNK_142950410._11_1_);
  auVar20[0xc] = -(auVar19[0xc] == UNK_142950410._12_1_);
  auVar20[0xd] = -(auVar19[0xd] == UNK_142950410._13_1_);
  auVar20[0xe] = -(auVar19[0xe] == UNK_142950410._14_1_);
  auVar20[0xf] = -(auVar19[0xf] == UNK_142950410._15_1_);
  uVar14 = (ushort)(SUB161(auVar20 >> 7,0) & 1) | (ushort)(SUB161(auVar20 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar20 >> 0x17,0) & 1) << 2 |
           (ushort)(SUB161(auVar20 >> 0x1f,0) & 1) << 3 |
           (ushort)(SUB161(auVar20 >> 0x27,0) & 1) << 4 |
           (ushort)(SUB161(auVar20 >> 0x2f,0) & 1) << 5 |
           (ushort)(SUB161(auVar20 >> 0x37,0) & 1) << 6 |
           (ushort)(SUB161(auVar20 >> 0x3f,0) & 1) << 7 |
           (ushort)(SUB161(auVar20 >> 0x47,0) & 1) << 8 |
           (ushort)(SUB161(auVar20 >> 0x4f,0) & 1) << 9 |
           (ushort)(SUB161(auVar20 >> 0x57,0) & 1) << 10 |
           (ushort)(SUB161(auVar20 >> 0x5f,0) & 1) << 0xb |
           (ushort)(SUB161(auVar20 >> 0x67,0) & 1) << 0xc |
           (ushort)(SUB161(auVar20 >> 0x6f,0) & 1) << 0xd |
           (ushort)(SUB161(auVar20 >> 0x77,0) & 1) << 0xe | (ushort)(auVar20[0xf] >> 7) << 0xf;
  uVar13 = (uint)uVar14;
  while( true ) {
    if (uVar14 == 0) goto code_r0x000140a29c68;
    uVar4 = 0;
    for (uVar5 = uVar13; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
      uVar4 = uVar4 + 1;
    }
    lVar9 = (uVar4 + uVar8 & uVar10) * -0x88;
    if ((*(longlong *)(lVar7 + -0x88 + lVar9) == -0x7dd9338791986cab) &&
       (*(longlong *)(lVar7 + -0x80 + lVar9) == 0x65cfda6a55a841ac)) break;
    uVar14 = (ushort)(uVar13 - 1) & (ushort)uVar13;
    uVar13 = CONCAT22((short)(uVar13 - 1 >> 0x10),uVar14);
  }
  puVar11 = *(undefined8 **)(lVar7 + lVar9 + -8);
  LOCK();
  piVar1 = (int *)(param_2 + 0x90);
  iVar6 = *piVar1;
  *piVar1 = *piVar1 + -1;
  UNLOCK();
  if (!SBORROW4(0,iVar6 - 1U & 0xbfffffff)) {
    if (puVar11 < *(undefined8 **)(param_2 + 0x218)) goto code_r0x000140a29cfc;
    goto code_r0x000140a2a05d;
  }
  FUN_1428d8d40(param_2 + 0x90);
  if (*(undefined8 **)(param_2 + 0x218) <= puVar11) goto code_r0x000140a2a05d;
code_r0x000140a29cfc:
  uVar10 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + (longlong)puVar11 * 8);
  if (uVar10 != 0) {
    lVar7 = ~uVar10 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar7) == 0)
    goto code_r0x000140a2a05d;
    puVar11 = (undefined8 *)
              FUN_140969720(*(undefined8 *)(*(longlong *)(param_2 + 0x1e0) + lVar7 + 0x10),
                            &UNK_142c2e3f8);
    if (puVar11 == (undefined8 *)0x0) {
code_r0x000140a2a0f2:
      FUN_1428d9310(&UNK_142acaca8);
      pcVar3 = (code *)swi(3);
      (*pcVar3)();
      return;
    }
    puStack_3b8 = &UNK_142a9e3cd;
    uStack_3b0 = 0x35;
    if ((puVar11[3] != 0x35) ||
       (iVar6 = func_0x000142923bf0(&UNK_142a9e3cd,puVar11[2],0x35), iVar6 != 0))
    goto code_r0x000140a2a069;
    lVar7 = (**(code **)(puVar11[1] + 0x18))(*puVar11);
    if (lVar7 == 0) goto code_r0x000140a2a0f2;
    FUN_141659f00(&puStack_3b8,param_2);
    FUN_141658290(&puStack_3b8,param_2);
    if (puStack_3b8 == *(undefined **)(param_2 + 0x5a0)) {
      lStack_b0 = lStack_2c0 + lStack_2b8 * 4;
      auStack_70 = (undefined1  [16])0x0;
      auStack_80 = (undefined1  [16])0x0;
      auStack_90 = (undefined1  [16])0x0;
      uStack_60 = 0;
      lStack_c8 = param_2 + 0xf8;
      lStack_d0 = param_2 + 0x1a0;
      lStack_b8 = lStack_2c0;
      uStack_a8 = 8;
      uStack_a0 = 0;
      uStack_98 = 8;
      uStack_58 = uStack_2b0;
      uStack_53 = (undefined3)uStack_4c;
      auStack_220[0] = 0;
      uStack_180 = 0;
      lStack_e0 = lVar7;
      lStack_d8 = param_2;
      ppuStack_c0 = &puStack_3b8;
      FUN_14130d8e0(auStack_50,auStack_220,&UNK_142aca530);
      uStack_31 = 1;
      FUN_1405d0be0(&puStack_3b8);
      uStack_31 = 1;
      FUN_1416032a0(auStack_220,&UNK_142acacc0,10,&UNK_142acacca,0x1b,param_2,param_1);
      uStack_31 = 1;
      FUN_14160c800(&puStack_3b8,auStack_220,0xc3483cd70105de2f);
      uStack_32 = 1;
      uVar10 = 0x4bc362195b80f729;
      lVar7 = FUN_1400dd6f0(param_2 + 0x38,0x4bc362195b80f729,0x982798ce80cbcefb);
      if (((lVar7 != 0) && (uVar10 < *(ulonglong *)(param_2 + 0x218))) &&
         (uVar10 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar10 * 8), uVar10 != 0)) {
        lVar7 = ~uVar10 * 0x60;
        if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar7) != 0) {
          if (cStack_282 != '\b') {
            FUN_14096ba30(auStack_2a8,lStack_280 + 0xe8,0x601a45ce43af912f,
                          *(undefined4 *)
                           (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + lVar7 + 0x10) + 0x28));
          }
          func_0x000142923c00(auStack_220,&puStack_3b8,400);
          uStack_3a8 = uStack_40;
          uStack_3b0 = uStack_48;
          uStack_31 = 0;
          FUN_14160aaa0(auStack_220,&puStack_3b8);
          return;
        }
      }
      goto code_r0x000140a2a0dd;
    }
    goto code_r0x000140a2a099;
  }
  goto code_r0x000140a2a05d;
code_r0x000140a29c68:
  auVar18[0] = -(auVar19[0] == -1);
  auVar18[1] = -(auVar19[1] == -1);
  auVar18[2] = -(auVar19[2] == -1);
  auVar18[3] = -(auVar19[3] == -1);
  auVar18[4] = -(auVar19[4] == -1);
  auVar18[5] = -(auVar19[5] == -1);
  auVar18[6] = -(auVar19[6] == -1);
  auVar18[7] = -(auVar19[7] == -1);
  auVar18[8] = -(auVar19[8] == -1);
  auVar18[9] = -(auVar19[9] == -1);
  auVar18[10] = -(auVar19[10] == -1);
  auVar18[0xb] = -(auVar19[0xb] == -1);
  auVar18[0xc] = -(auVar19[0xc] == -1);
  auVar18[0xd] = -(auVar19[0xd] == -1);
  auVar18[0xe] = -(auVar19[0xe] == -1);
  auVar18[0xf] = -(auVar19[0xf] == -1);
  if ((((((((((((((((SUB161(auVar18 >> 7,0) & 1) != 0 || (SUB161(auVar18 >> 0xf,0) & 1) != 0) ||
                  (SUB161(auVar18 >> 0x17,0) & 1) != 0) || (SUB161(auVar18 >> 0x1f,0) & 1) != 0) ||
                (SUB161(auVar18 >> 0x27,0) & 1) != 0) || (SUB161(auVar18 >> 0x2f,0) & 1) != 0) ||
              (SUB161(auVar18 >> 0x37,0) & 1) != 0) || (SUB161(auVar18 >> 0x3f,0) & 1) != 0) ||
            (SUB161(auVar18 >> 0x47,0) & 1) != 0) || (SUB161(auVar18 >> 0x4f,0) & 1) != 0) ||
          (SUB161(auVar18 >> 0x57,0) & 1) != 0) || (SUB161(auVar18 >> 0x5f,0) & 1) != 0) ||
        (SUB161(auVar18 >> 0x67,0) & 1) != 0) || (SUB161(auVar18 >> 0x6f,0) & 1) != 0) ||
      (SUB161(auVar18 >> 0x77,0) & 1) != 0) || auVar18[0xf] < '\0') goto code_r0x000140a2a043;
  lVar9 = uVar8 + lVar12;
  lVar12 = lVar12 + 0x10;
  uVar8 = lVar9 + 0x10U & uVar10;
  goto code_r0x000140a29c51;
}

