
/* WARNING: Type propagation algorithm not settling */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
void asset__inheritable__impl$1__inherit_from(uint *param_1,uint *param_2)
{
  byte bVar1;
  char cVar2;
  code *pcVar3;
  uint uVar4;
  byte bVar5;
  uint uVar6;
  uint in_R10D;
  uint uVar7;
  byte in_R11B;
  undefined **ppuStack_b0;
  undefined *puStack_a8;
  undefined *puStack_a0;
  ulonglong uStack_98;
  undefined *puStack_90;
  uint uStack_88;
  undefined *puStack_80;
  uint uStack_78;
  undefined **ppuStack_70;
  uint uStack_68;
  undefined8 uStack_60;
  uint uStack_58;
  undefined8 uStack_50;
  uint uStack_48;
  undefined *puStack_40;
  ulonglong uStack_38;
  undefined ***pppuStack_30;
  undefined8 uStack_28;
  undefined8 uStack_20;
  uVar4 = *param_1;
  uVar6 = uVar4 - 4;
  if (1 < uVar6) {
    uVar6 = 2;
  }
  if (uVar6 != 0) {
    if (uVar6 == 2) {
      uVar7 = *param_2 - 4;
      uVar6 = 2;
      if (uVar7 < 2) {
        uVar6 = uVar7;
      }
      if (uVar6 == 2) {
        if (uVar4 != 3 && *param_2 != 3) {
          asset__prefab__config__impl$6__inherit_from();
          uVar4 = *param_1;
        }
        if (uVar4 == 3) {
          return;
        }
        if ((char)param_1[0x10] == '\0') {
          *(undefined1 *)(param_1 + 0x10) = 1;
        }
        if (uVar4 == 0) {
          *param_1 = 2;
          *(undefined8 *)(param_1 + 1) = _DAT_14292e6b0;
          param_1[3] = 0x3f800000;
          uVar6 = param_1[4];
        }
        else {
          uVar6 = param_1[4];
        }
        if (uVar6 == 0) {
          param_1[4] = 2;
          *(undefined8 *)(param_1 + 5) = _DAT_14292e6b0;
          param_1[7] = 0x3f800000;
          uVar6 = param_1[8];
        }
        else {
          uVar6 = param_1[8];
        }
        if (uVar6 == 0) {
          param_1[8] = 2;
          *(undefined8 *)(param_1 + 9) = _DAT_14292e6b0;
          param_1[0xb] = 0x3f800000;
          uVar6 = param_1[0xc];
        }
        else {
          uVar6 = param_1[0xc];
        }
        if (uVar6 == 0) {
          param_1[0xc] = 2;
          *(undefined8 *)(param_1 + 0xd) = _DAT_14292e6b0;
          param_1[0xf] = 0x3f800000;
          cVar2 = *(char *)((longlong)param_1 + 0x41);
        }
        else {
          cVar2 = *(char *)((longlong)param_1 + 0x41);
        }
        if (cVar2 == '\x02') {
          *(undefined1 *)((longlong)param_1 + 0x41) = 0;
        }
        return;
      }
      if (uVar6 != 1) goto LAB_14268a6b3;
      puStack_a0 = &UNK_14315120a;
      uStack_98 = 0x3b;
      ppuStack_b0 = &puStack_a0;
      puStack_a8 = &UNK_140109e90;
      puStack_40 = &UNK_143150340;
      uStack_38 = 2;
      uStack_20 = 0;
      pppuStack_30 = &ppuStack_b0;
      uStack_28 = 1;
      FUN_1428d9390(&puStack_40,&UNK_143150420);
    }
    FUN_1428d9430(&UNK_1431503c0,0x28,&UNK_143150408);
LAB_14268a6b3:
    puStack_40 = &UNK_1431502f8;
    uStack_38 = 1;
    pppuStack_30 = (undefined ***)0x8;
    uStack_28 = 0;
    uStack_20 = 0;
    FUN_1428d9390(&puStack_40,&UNK_143150420);
    pcVar3 = (code *)swi(3);
    (*pcVar3)();
    return;
  }
  uVar4 = *param_2;
  uVar6 = uVar4 - 4;
  uVar7 = 2;
  if (uVar6 < 2) {
    uVar7 = uVar6;
  }
  if (uVar7 == 0) {
    uVar4 = 4;
    goto LAB_14268a3cd;
  }
  if (uVar7 != 2) {
    uVar4 = 5;
    goto LAB_14268a3cd;
  }
  if (uVar4 == 3) {
    uVar4 = 3;
    goto LAB_14268a3cd;
  }
  if (uVar4 < 2) {
    uVar7 = param_2[4];
    if (uVar7 < 2) goto LAB_14268a4e6;
LAB_14268a565:
    puStack_a8 = (undefined *)(ulonglong)param_2[7];
    ppuStack_b0 = *(undefined ***)(param_2 + 5);
    uVar6 = param_2[8];
    if (1 < uVar6) goto LAB_14268a585;
LAB_14268a4f4:
    in_R10D = param_2[0xc];
    puStack_80 = puStack_a0;
    ppuStack_70 = ppuStack_b0;
    uStack_60 = uStack_50;
    uStack_58 = uStack_48;
    uStack_68 = (uint)puStack_a8;
  }
  else {
    uStack_48 = param_2[3];
    uStack_50 = *(undefined8 *)(param_2 + 1);
    uVar7 = param_2[4];
    if (1 < uVar7) goto LAB_14268a565;
LAB_14268a4e6:
    uVar6 = param_2[8];
    if (uVar6 < 2) goto LAB_14268a4f4;
LAB_14268a585:
    uStack_98 = (ulonglong)param_2[0xb];
    puStack_80 = *(undefined **)(param_2 + 9);
    in_R10D = param_2[0xc];
    ppuStack_70 = ppuStack_b0;
    uStack_60 = uStack_50;
    uStack_58 = uStack_48;
    uStack_68 = (uint)puStack_a8;
  }
  uStack_78 = (uint)uStack_98;
  if (1 < in_R10D) {
    uStack_38 = (ulonglong)param_2[0xf];
    puStack_40 = *(undefined **)(param_2 + 0xd);
  }
  bVar1 = *(byte *)((longlong)param_2 + 0x41) - 2;
  bVar5 = 2;
  if (bVar1 < 2) {
    bVar5 = bVar1;
  }
  if (bVar5 == 0) {
    in_R11B = 2;
  }
  else if (bVar5 == 2) {
    in_R11B = *(byte *)((longlong)param_2 + 0x41) & 1;
  }
  else {
    in_R11B = 3;
  }
  param_2 = (uint *)(ulonglong)(byte)param_2[0x10];
  puStack_90 = puStack_40;
  uStack_88 = (uint)uStack_38;
LAB_14268a3cd:
  *param_1 = uVar4;
  *(undefined8 *)(param_1 + 1) = uStack_60;
  param_1[3] = uStack_58;
  param_1[4] = uVar7;
  *(undefined ***)(param_1 + 5) = ppuStack_70;
  param_1[7] = uStack_68;
  param_1[8] = uVar6;
  *(undefined **)(param_1 + 9) = puStack_80;
  param_1[0xb] = uStack_78;
  param_1[0xc] = in_R10D;
  *(undefined **)(param_1 + 0xd) = puStack_90;
  param_1[0xf] = uStack_88;
  *(char *)(param_1 + 0x10) = (char)param_2;
  *(byte *)((longlong)param_1 + 0x41) = in_R11B;
  return;
}
