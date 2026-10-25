
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_141e4c6c0(longlong param_1,undefined4 param_2,undefined4 param_3,float param_4)

{
  byte bVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  longlong lVar4;
  longlong lVar5;
  float *pfVar6;
  float *pfVar7;
  float fVar8;
  uint uVar9;
  undefined *puVar10;
  ulonglong uVar11;
  longlong lVar12;
  float fVar13;
  float fVar14;
  undefined4 uVar16;
  undefined8 *puVar15;
  undefined8 uStack_d8;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  int iStack_b0;
  undefined4 uStack_ac;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined4 uStack_90;
  undefined4 auStack_8c [19];
  
  lVar4 = *(longlong *)(param_1 + 0xa0);
  uVar11 = *(ulonglong *)(param_1 + 0xa8);
  uStack_d8 = &iStack_b0;
  uStack_90 = param_2;
  auStack_8c[0] = param_3;
  if (uVar11 < 2) {
    if (uVar11 == 0) {
      return;
    }
  }
  else {
    if (0x14 < uVar11) goto LAB_141e4ca14;
    FUN_140fe4110(lVar4,uVar11,1,&uStack_d8);
  }
LAB_141e4c732:
  uVar9 = _DAT_1429258f0;
  fVar8 = _DAT_142925870;
  lVar5 = uVar11 * 0x28;
  lVar12 = 0;
  do {
    bVar1 = *(byte *)(lVar4 + 0x24 + lVar12);
    uVar11 = (ulonglong)bVar1;
    if (bVar1 == 4) {
      FUN_142376120(param_1,*(undefined4 *)(lVar4 + 0x10 + lVar12),
                    *(undefined4 *)(lVar4 + 0x14 + lVar12),&uStack_90,auStack_8c);
      uStack_d8 = (int *)CONCAT44(_UNK_142f8f51c,_DAT_142f8f518);
      uStack_d0 = _UNK_142f8f520;
      uStack_cc = _UNK_142f8f524;
      puVar15 = &uStack_d8;
      FUN_140fe7e40(&iStack_b0,param_1,*(undefined4 *)(lVar4 + 0x10 + lVar12),
                    *(undefined4 *)(lVar4 + 0x14 + lVar12),puVar15);
      uVar16 = (undefined4)((ulonglong)puVar15 >> 0x20);
      if (iStack_b0 != 4) break;
      uVar2 = *(undefined4 *)(lVar4 + 0x10 + lVar12);
      uVar3 = *(undefined4 *)(lVar4 + 0x14 + lVar12);
      fVar14 = *(float *)(lVar4 + 8 + lVar12);
      fVar13 = *(float *)(lVar4 + 0xc + lVar12);
    }
    else {
      uVar2 = *(undefined4 *)(lVar4 + 0x18 + lVar12);
      uVar3 = *(undefined4 *)(lVar4 + 0x1c + lVar12);
      fVar13 = *(float *)(lVar4 + 0x20 + lVar12);
      uStack_d8 = (int *)CONCAT44(_UNK_142f8f51c,_DAT_142f8f518);
      uStack_d0 = _UNK_142f8f520;
      uStack_cc = _UNK_142f8f524;
      puVar15 = &uStack_d8;
      FUN_140fe7e40(&iStack_b0,param_1,*(undefined4 *)(lVar4 + 0x10 + lVar12),
                    *(undefined4 *)(lVar4 + 0x14 + lVar12),puVar15);
      uVar16 = (undefined4)((ulonglong)puVar15 >> 0x20);
      if (iStack_b0 != 4) {
        uStack_d8 = (int *)CONCAT44(uStack_ac,iStack_b0);
        puVar10 = &UNK_142f8f540;
        goto LAB_141e4c9f2;
      }
      FUN_140fe7d00(&iStack_b0,param_1,uVar2,uVar3);
      if (iStack_b0 != 4) {
        uStack_d8 = (int *)CONCAT44(uStack_ac,iStack_b0);
        puVar10 = &UNK_142f8f558;
        goto LAB_141e4c9f2;
      }
      pfVar6 = (float *)CONCAT44(uStack_a4,uStack_a8);
      FUN_140fe7d00(&iStack_b0,param_1,*(undefined4 *)(lVar4 + 0x10 + lVar12),
                    *(undefined4 *)(lVar4 + 0x14 + lVar12));
      if (iStack_b0 != 4) {
        uStack_d8 = (int *)CONCAT44(uStack_ac,iStack_b0);
        puVar10 = &UNK_142f8f570;
        goto LAB_141e4c9f2;
      }
      pfVar7 = (float *)CONCAT44(uStack_a4,uStack_a8);
      if (bVar1 < 2) {
        fVar14 = *pfVar6 * fVar8 - *pfVar7 * fVar8;
        if (bVar1 == 0) {
          fVar13 = (float)((uint)pfVar7[1] ^ uVar9) - fVar13 * param_4;
        }
        else {
          fVar13 = pfVar6[1] + fVar13 * param_4;
        }
      }
      else {
        if (bVar1 == 2) {
          fVar14 = (float)((uint)*pfVar7 ^ uVar9) - fVar13 * param_4;
        }
        else {
          fVar14 = *pfVar6 + fVar13 * param_4;
        }
        fVar13 = pfVar6[1] * fVar8 - pfVar7[1] * fVar8;
      }
      fVar14 = fVar14 + pfVar6[2] + *(float *)(lVar4 + 8 + lVar12);
      fVar13 = fVar13 + pfVar6[3] + *(float *)(lVar4 + 0xc + lVar12);
      uVar2 = *(undefined4 *)(lVar4 + 0x10 + lVar12);
      uVar3 = *(undefined4 *)(lVar4 + 0x14 + lVar12);
    }
    FUN_142374e10(param_1,uVar2,uVar3,fVar14,CONCAT44(uVar16,fVar13));
    lVar12 = lVar12 + 0x28;
    if (lVar5 == lVar12) {
      return;
    }
  } while( true );
  uStack_d8 = (int *)CONCAT44(uStack_ac,iStack_b0);
  puVar10 = &UNK_142f8f528;
LAB_141e4c9f2:
  uStack_d0 = uStack_a8;
  uStack_cc = uStack_a4;
  FUN_1428d9760(&UNK_142f8f358,0x2b,&uStack_d8,&UNK_142f8f338,puVar10);
LAB_141e4ca14:
  FUN_140fe4c00(lVar4,uVar11,&uStack_d8);
  goto LAB_141e4c732;
}

