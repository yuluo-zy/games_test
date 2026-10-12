
void FUN_1417e78e0(longlong *param_1,longlong param_2,longlong *param_3)

{
  undefined8 *puVar1;
  undefined1 auVar2 [16];
  ushort uVar3;
  undefined8 *puVar4;
  ulonglong uVar5;
  code *pcVar6;
  int iVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  longlong lVar14;
  uint uVar15;
  longlong lVar16;
  undefined1 (*pauVar17) [16];
  undefined *puVar18;
  undefined8 uStack_108;
  undefined8 uStack_100;
  undefined4 uStack_f8;
  undefined4 uStack_f4;
  undefined4 uStack_f0;
  undefined4 uStack_ec;
  undefined4 uStack_e8;
  undefined4 uStack_e4;
  undefined4 uStack_e0;
  undefined4 uStack_dc;
  undefined4 uStack_d8;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  longlong lStack_c8;
  longlong *aplStack_c0 [8];
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  undefined8 uStack_60;
  undefined8 uStack_58;
  undefined8 uStack_50;
  undefined8 uStack_48;
  
  puVar4 = (undefined8 *)*param_3;
  uVar3 = *(ushort *)(param_1 + 3);
joined_r0x0001417e7921:
  if (uVar3 != 0) goto code_r0x0001417e793c;
code_r0x0001417e7950:
  if (param_2 == 0) {
    return;
  }
  lVar14 = *param_1;
  pauVar17 = (undefined1 (*) [16])param_1[1];
  do {
    auVar2 = *pauVar17;
    lVar14 = lVar14 + -0x20;
    pauVar17 = pauVar17 + 1;
    uVar15 = (uint)(ushort)((ushort)(SUB161(auVar2 >> 7,0) & 1) |
                            (ushort)(SUB161(auVar2 >> 0xf,0) & 1) << 1 |
                            (ushort)(SUB161(auVar2 >> 0x17,0) & 1) << 2 |
                            (ushort)(SUB161(auVar2 >> 0x1f,0) & 1) << 3 |
                            (ushort)(SUB161(auVar2 >> 0x27,0) & 1) << 4 |
                            (ushort)(SUB161(auVar2 >> 0x2f,0) & 1) << 5 |
                            (ushort)(SUB161(auVar2 >> 0x37,0) & 1) << 6 |
                            (ushort)(SUB161(auVar2 >> 0x3f,0) & 1) << 7 |
                            (ushort)(SUB161(auVar2 >> 0x47,0) & 1) << 8 |
                            (ushort)(SUB161(auVar2 >> 0x4f,0) & 1) << 9 |
                            (ushort)(SUB161(auVar2 >> 0x57,0) & 1) << 10 |
                            (ushort)(SUB161(auVar2 >> 0x5f,0) & 1) << 0xb |
                            (ushort)(SUB161(auVar2 >> 0x67,0) & 1) << 0xc |
                            (ushort)(SUB161(auVar2 >> 0x6f,0) & 1) << 0xd |
                            (ushort)(SUB161(auVar2 >> 0x77,0) & 1) << 0xe |
                           (ushort)(byte)(auVar2[0xf] >> 7) << 0xf);
  } while (uVar15 == 0xffff);
  uVar15 = ~uVar15;
  *param_1 = lVar14;
  param_1[1] = (longlong)pauVar17;
  do {
    uVar3 = (ushort)uVar15;
    iVar7 = 0;
    for (; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
      iVar7 = iVar7 + 1;
    }
    *(ushort *)(param_1 + 3) = uVar3 - 1 & uVar3;
    puVar18 = (undefined *)(ulonglong)*(ushort *)((lVar14 - (ulonglong)(uint)(iVar7 * 2)) + -2);
    if (*(undefined **)(puVar4[1] + 0x10) <= puVar18) {
code_r0x0001417e7ab6:
      FUN_1428d9518(puVar18);
      pcVar6 = (code *)swi(3);
      (*pcVar6)();
      return;
    }
    uVar3 = *(ushort *)(*(longlong *)(puVar4[1] + 8) + (longlong)puVar18 * 4);
    uVar5 = *(ulonglong *)(puVar4[2] + 0x10);
    if (uVar5 <= uVar3) {
      puVar18 = &UNK_142ce7b28;
      FUN_1428d9518((ulonglong)uVar3,uVar5,&UNK_142ce7b28);
      goto code_r0x0001417e7ab6;
    }
    lVar14 = *(longlong *)(puVar4[2] + 8);
    lVar16 = (ulonglong)uVar3 * 0x40;
    uStack_108 = *(undefined8 *)(lVar14 + lVar16);
    uStack_100 = ((undefined8 *)(lVar14 + lVar16))[1];
    puVar1 = (undefined8 *)(lVar14 + 0x10 + lVar16);
    uVar8 = *puVar1;
    uVar9 = puVar1[1];
    puVar1 = (undefined8 *)(lVar14 + 0x20 + lVar16);
    uVar10 = *puVar1;
    uVar11 = puVar1[1];
    puVar1 = (undefined8 *)(lVar14 + 0x30 + lVar16);
    uVar12 = *puVar1;
    uVar13 = puVar1[1];
    *(undefined8 *)(lVar14 + lVar16) = 5;
    uStack_70._0_4_ = (undefined4)uVar8;
    uStack_70._4_4_ = (undefined4)((ulonglong)uVar8 >> 0x20);
    uStack_68._0_4_ = (undefined4)uVar9;
    uStack_68._4_4_ = (undefined4)((ulonglong)uVar9 >> 0x20);
    uStack_60._0_4_ = (undefined4)uVar10;
    uStack_60._4_4_ = (undefined4)((ulonglong)uVar10 >> 0x20);
    uStack_58._0_4_ = (undefined4)uVar11;
    uStack_58._4_4_ = (undefined4)((ulonglong)uVar11 >> 0x20);
    uStack_50._0_4_ = (undefined4)uVar12;
    uStack_50._4_4_ = (undefined4)((ulonglong)uVar12 >> 0x20);
    uStack_48._0_4_ = (undefined4)uVar13;
    uStack_48._4_4_ = (undefined4)((ulonglong)uVar13 >> 0x20);
    uStack_d8 = (undefined4)uStack_50;
    uStack_d4 = uStack_50._4_4_;
    uStack_d0 = (undefined4)uStack_48;
    uStack_cc = uStack_48._4_4_;
    uStack_e8 = (undefined4)uStack_60;
    uStack_e4 = uStack_60._4_4_;
    uStack_e0 = (undefined4)uStack_58;
    uStack_dc = uStack_58._4_4_;
    uStack_f8 = (undefined4)uStack_70;
    uStack_f4 = uStack_70._4_4_;
    uStack_f0 = (undefined4)uStack_68;
    uStack_ec = uStack_68._4_4_;
    uStack_80 = uStack_108;
    uStack_78 = uStack_100;
    uStack_70 = uVar8;
    uStack_68 = uVar9;
    uStack_60 = uVar10;
    uStack_58 = uVar11;
    uStack_50 = uVar12;
    uStack_48 = uVar13;
    FUN_1418ae210(&lStack_c8,*puVar4,puVar18,&uStack_108);
    switch(lStack_c8) {
    case 0:
    case 1:
    case 2:
    case 3:
      FUN_140be24b0(aplStack_c0);
      param_2 = param_2 + -1;
      uVar3 = *(ushort *)(param_1 + 3);
      goto joined_r0x0001417e7921;
    case 4:
      LOCK();
      *aplStack_c0[0] = *aplStack_c0[0] + -1;
      UNLOCK();
      if (*aplStack_c0[0] == 0) {
        FUN_140bdd1b0(aplStack_c0);
      }
    }
    param_2 = param_2 + -1;
    uVar3 = *(ushort *)(param_1 + 3);
    if (uVar3 == 0) goto code_r0x0001417e7950;
code_r0x0001417e793c:
    uVar15 = (uint)uVar3;
    lVar14 = *param_1;
  } while( true );
}

