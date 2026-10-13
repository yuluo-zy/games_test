
void asset__prefab__scan__PrefabDependencies__add_file_to_prefab_dependency(longlong *param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                  undefined8 param_5)
{
  char *pcVar1;
  longlong lVar2;
  ulonglong uVar3;
  longlong lVar4;
  code *pcVar5;
  uint uVar6;
  uint uVar7;
  int iVar8;
  int iVar9;
  int iVar10;
  int iVar11;
  char cVar12;
  char cVar13;
  char cVar14;
  char cVar15;
  char cVar16;
  char cVar17;
  char cVar18;
  char cVar19;
  char cVar20;
  char cVar21;
  char cVar22;
  char cVar23;
  char cVar24;
  char cVar25;
  char cVar26;
  char cVar27;
  char cVar28;
  ushort uVar29;
  ulonglong uVar30;
  undefined8 uVar31;
  undefined8 *puVar32;
  longlong lVar33;
  longlong lVar34;
  uint uVar35;
  byte bVar36;
  undefined1 auVar37 [16];
  char cVar38;
  char cVar39;
  char cVar40;
  char cVar41;
  undefined1 unaff_XMM7_Ba;
  undefined1 unaff_XMM7_Bb;
  undefined1 unaff_XMM7_Bc;
  undefined1 unaff_XMM7_Bd;
  undefined1 unaff_XMM7_Be;
  undefined1 unaff_XMM7_Bf;
  undefined1 unaff_XMM7_Bg;
  undefined1 unaff_XMM7_Bh;
  undefined1 unaff_XMM7_Bi;
  undefined1 unaff_XMM7_Bj;
  undefined1 unaff_XMM7_Bk;
  undefined1 unaff_XMM7_Bl;
  undefined1 unaff_XMM7_Bm;
  undefined1 unaff_XMM7_Bn;
  undefined1 unaff_XMM7_Bo;
  undefined1 unaff_XMM7_Bp;
  undefined1 auVar42 [16];
  longlong alStack_108 [3];
  undefined8 uStack_f0;
  undefined8 *puStack_e8;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  longlong lStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined8 *puStack_90;
  undefined1 uStack_81;
  undefined8 uStack_80;
  uStack_80 = 0xfffffffffffffffe;
  uStack_b0 = param_5;
  uStack_b8 = param_4;
  lVar34 = lStack_a8;
  uStack_98 = param_4;
  if (param_1[3] != 0) {
    uStack_81 = 1;
    uStack_a0 = param_2;
    puStack_90 = (undefined8 *)param_3;
    uVar30 = core__hash__BuildHasher__hash_one(param_1 + 4);
    lVar2 = *param_1;
    uVar3 = param_1[1];
    bVar36 = (byte)(uVar30 >> 0x39);
    auVar37 = ZEXT216(CONCAT11(bVar36,bVar36));
    auVar37 = pshuflw(auVar37,auVar37,0);
    cVar38 = auVar37[0];
    cVar39 = auVar37[1];
    cVar40 = auVar37[2];
    cVar41 = auVar37[3];
    lVar33 = 0;
    iVar8 = -(uint)(CONCAT13(unaff_XMM7_Bd,
                             CONCAT12(unaff_XMM7_Bc,CONCAT11(unaff_XMM7_Bb,unaff_XMM7_Ba))) ==
                   CONCAT13(unaff_XMM7_Bd,
                            CONCAT12(unaff_XMM7_Bc,CONCAT11(unaff_XMM7_Bb,unaff_XMM7_Ba))));
    iVar9 = -(uint)(CONCAT13(unaff_XMM7_Bh,
                             CONCAT12(unaff_XMM7_Bg,CONCAT11(unaff_XMM7_Bf,unaff_XMM7_Be))) ==
                   CONCAT13(unaff_XMM7_Bh,
                            CONCAT12(unaff_XMM7_Bg,CONCAT11(unaff_XMM7_Bf,unaff_XMM7_Be))));
    iVar10 = -(uint)(CONCAT13(unaff_XMM7_Bl,
                              CONCAT12(unaff_XMM7_Bk,CONCAT11(unaff_XMM7_Bj,unaff_XMM7_Bi))) ==
                    CONCAT13(unaff_XMM7_Bl,
                             CONCAT12(unaff_XMM7_Bk,CONCAT11(unaff_XMM7_Bj,unaff_XMM7_Bi))));
    iVar11 = -(uint)(CONCAT13(unaff_XMM7_Bp,
                              CONCAT12(unaff_XMM7_Bo,CONCAT11(unaff_XMM7_Bn,unaff_XMM7_Bm))) ==
                    CONCAT13(unaff_XMM7_Bp,
                             CONCAT12(unaff_XMM7_Bo,CONCAT11(unaff_XMM7_Bn,unaff_XMM7_Bm))));
    uVar31 = uStack_a0;
    puVar32 = puStack_90;
    while( true ) {
      uVar30 = uVar3 & uVar30;
      pcVar1 = (char *)(lVar2 + uVar30);
      cVar12 = *pcVar1;
      cVar13 = pcVar1[1];
      cVar14 = pcVar1[2];
      cVar15 = pcVar1[3];
      cVar16 = pcVar1[4];
      cVar17 = pcVar1[5];
      cVar18 = pcVar1[6];
      cVar19 = pcVar1[7];
      cVar20 = pcVar1[8];
      cVar21 = pcVar1[9];
      cVar22 = pcVar1[10];
      cVar23 = pcVar1[0xb];
      cVar24 = pcVar1[0xc];
      cVar25 = pcVar1[0xd];
      cVar26 = pcVar1[0xe];
      cVar27 = pcVar1[0xf];
      auVar37[0] = -(cVar12 == cVar38);
      auVar37[1] = -(cVar13 == cVar39);
      auVar37[2] = -(cVar14 == cVar40);
      auVar37[3] = -(cVar15 == cVar41);
      auVar37[4] = -(cVar16 == cVar38);
      auVar37[5] = -(cVar17 == cVar39);
      auVar37[6] = -(cVar18 == cVar40);
      auVar37[7] = -(cVar19 == cVar41);
      auVar37[8] = -(cVar20 == cVar38);
      auVar37[9] = -(cVar21 == cVar39);
      auVar37[10] = -(cVar22 == cVar40);
      auVar37[0xb] = -(cVar23 == cVar41);
      auVar37[0xc] = -(cVar24 == cVar38);
      auVar37[0xd] = -(cVar25 == cVar39);
      auVar37[0xe] = -(cVar26 == cVar40);
      auVar37[0xf] = -(cVar27 == cVar41);
      uVar29 = (ushort)(SUB161(auVar37 >> 7,0) & 1) | (ushort)(SUB161(auVar37 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar37 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar37 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar37 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar37 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar37 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar37 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar37 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar37 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar37 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar37 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar37 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar37 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar37 >> 0x77,0) & 1) << 0xe | (ushort)(auVar37[0xf] >> 7) << 0xf;
      uVar35 = (uint)uVar29;
      lVar34 = lStack_a8;
      while (lStack_a8 = lVar33, uVar29 != 0) {
        uVar6 = 0;
        for (uVar7 = uVar35; (uVar7 & 1) == 0; uVar7 = uVar7 >> 1 | 0x80000000) {
          uVar6 = uVar6 + 1;
        }
        lVar34 = (uVar6 + uVar30 & uVar3) * -0x38;
        uStack_81 = 1;
        cVar28 = hashbrown__impl$0__equivalent(uVar31,puVar32,lVar2 + -0x38 + lVar34);
        if (cVar28 != '\0') {
          uStack_d8 = uStack_98;
          uStack_d0 = param_5;
          lVar33 = *(longlong *)(lVar2 + -8 + lVar34);
          if (lVar33 == *(longlong *)(lVar2 + -0x18 + lVar34)) {
            alloc__raw_vec__RawVec__grow_one(lVar2 + lVar34 + -0x18,&UNK_142fce640);
          }
          lVar4 = *(longlong *)(lVar2 + lVar34 + -0x10);
          *(undefined8 *)(lVar4 + lVar33 * 0x10) = uStack_98;
          *(undefined8 *)(lVar4 + 8 + lVar33 * 0x10) = param_5;
          *(longlong *)(lVar2 + lVar34 + -8) = lVar33 + 1;
          return;
        }
        uVar29 = (ushort)(uVar35 - 1) & (ushort)uVar35;
        uVar35 = CONCAT22((short)(uVar35 - 1 >> 0x10),uVar29);
        uVar31 = uStack_a0;
        puVar32 = puStack_90;
        lVar33 = lStack_a8;
        lVar34 = lStack_a8;
      }
      auVar42[0] = -(cVar12 == (char)iVar8);
      auVar42[1] = -(cVar13 == (char)((uint)iVar8 >> 8));
      auVar42[2] = -(cVar14 == (char)((uint)iVar8 >> 0x10));
      auVar42[3] = -(cVar15 == (char)((uint)iVar8 >> 0x18));
      auVar42[4] = -(cVar16 == (char)iVar9);
      auVar42[5] = -(cVar17 == (char)((uint)iVar9 >> 8));
      auVar42[6] = -(cVar18 == (char)((uint)iVar9 >> 0x10));
      auVar42[7] = -(cVar19 == (char)((uint)iVar9 >> 0x18));
      auVar42[8] = -(cVar20 == (char)iVar10);
      auVar42[9] = -(cVar21 == (char)((uint)iVar10 >> 8));
      auVar42[10] = -(cVar22 == (char)((uint)iVar10 >> 0x10));
      auVar42[0xb] = -(cVar23 == (char)((uint)iVar10 >> 0x18));
      auVar42[0xc] = -(cVar24 == (char)iVar11);
      auVar42[0xd] = -(cVar25 == (char)((uint)iVar11 >> 8));
      auVar42[0xe] = -(cVar26 == (char)((uint)iVar11 >> 0x10));
      auVar42[0xf] = -(cVar27 == (char)((uint)iVar11 >> 0x18));
      if ((((((((((((((((SUB161(auVar42 >> 7,0) & 1) != 0 || (SUB161(auVar42 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar42 >> 0x17,0) & 1) != 0) || (SUB161(auVar42 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar42 >> 0x27,0) & 1) != 0) || (SUB161(auVar42 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar42 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar42 >> 0x3f,0) & 1) != 0) || (SUB161(auVar42 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar42 >> 0x4f,0) & 1) != 0) || (SUB161(auVar42 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar42 >> 0x5f,0) & 1) != 0) || (SUB161(auVar42 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar42 >> 0x6f,0) & 1) != 0) || (SUB161(auVar42 >> 0x77,0) & 1) != 0) ||
          auVar42[0xf] < '\0') break;
      uVar30 = uVar30 + lStack_a8 + 0x10;
      lVar33 = lStack_a8 + 0x10;
      lStack_a8 = lVar34;
    }
  }
  lStack_a8 = lVar34;
  uStack_81 = 1;
  FUN_140d61b00(&uStack_d8);
  puStack_e8 = (undefined8 *)func_0x000140613c10(0x10,8);
  puStack_90 = puStack_e8;
  if (puStack_e8 != (undefined8 *)0x0) {
    *puStack_e8 = uStack_98;
    puStack_e8[1] = param_5;
    uStack_f0 = 1;
    uStack_e0 = 1;
    hashbrown__map__HashMap__insert(alStack_108,param_1,&uStack_d8,&uStack_f0);
    if (!SBORROW8(0,alStack_108[0])) {
      uStack_81 = 0;
      core__ptr__drop_in_place(alStack_108);
    }
    return;
  }
  FUN_1428d9000(8,0x10);
                    /* WARNING: Does not return */
  pcVar5 = (code *)invalidInstructionException();
  (*pcVar5)();
}
