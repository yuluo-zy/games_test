
undefined8 * FUN_140bd5bf0(undefined8 *param_1,ushort *param_2)

{
  byte bVar1;
  byte bVar2;
  byte bVar3;
  ushort uVar4;
  ushort uVar5;
  ushort uVar6;
  code *pcVar7;
  ushort uVar8;
  undefined8 uVar9;
  undefined8 *puVar10;
  char cVar11;
  ushort uVar12;
  ushort uVar13;
  ushort uVar14;
  
  bVar1 = (byte)param_2[4];
  if (*(byte *)((longlong)param_2 + 9) <= bVar1) {
    uVar9 = 0;
code_r0x000140bd5cff:
    *param_1 = uVar9;
    return param_1;
  }
  bVar2 = (byte)param_2[3];
  if (bVar2 == 0) {
    FUN_1428d99e0(&UNK_142b0fcc0);
  }
  else {
    bVar3 = *(byte *)((longlong)param_2 + 7);
    if (bVar3 != 0) {
      uVar4 = param_2[1];
      uVar5 = *param_2;
      uVar6 = param_2[2];
      uVar8 = uVar5 >> 1;
      *param_2 = uVar8;
      uVar12 = uVar4 >> 1;
      param_2[1] = uVar12;
      uVar13 = uVar6 >> 1;
      param_2[2] = uVar13;
      if (CONCAT24(uVar13,*(undefined4 *)param_2) == 0) {
        cVar11 = -1;
      }
      else {
        uVar14 = 1;
        if (1 < uVar8) {
          uVar14 = uVar8;
        }
        *param_2 = uVar14;
        if (uVar12 < 2) {
          uVar12 = 1;
        }
        param_2[1] = uVar12;
        if (uVar13 < 2) {
          uVar13 = 1;
        }
        param_2[2] = uVar13;
        cVar11 = bVar1 + 1;
      }
      *(char *)(param_2 + 4) = cVar11;
      param_1[1] = (ulonglong)(byte)param_2[5] * (ulonglong)uVar6 *
                   (ulonglong)
                   (ushort)((uVar4 / bVar3 + 1) - (ushort)((ushort)(uVar4 % (ushort)bVar3) == 0)) *
                   (ulonglong)
                   (ushort)((uVar5 / bVar2 + 1) - (ushort)((ushort)(uVar5 % (ushort)bVar2) == 0));
      *(uint *)(param_1 + 2) = (uint)uVar5;
      *(uint *)((longlong)param_1 + 0x14) = (uint)uVar4;
      *(uint *)(param_1 + 3) = (uint)uVar6;
      *(uint *)((longlong)param_1 + 0x1c) = (uint)bVar1;
      uVar9 = 1;
      goto code_r0x000140bd5cff;
    }
  }
  FUN_1428d99e0(&UNK_142b0fcd8);
  pcVar7 = (code *)swi(3);
  puVar10 = (undefined8 *)(*pcVar7)();
  return puVar10;
}

