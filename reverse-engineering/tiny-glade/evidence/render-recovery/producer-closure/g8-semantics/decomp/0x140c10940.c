
int * FUN_140c10940(int *param_1,longlong *param_2)

{
  longlong lVar1;
  char cVar2;
  undefined1 uVar3;
  longlong lVar4;
  longlong lVar5;
  longlong lVar6;
  undefined1 *puVar7;
  uint uVar8;
  longlong lVar9;
  longlong lVar10;
  uint uVar11;
  undefined1 *puVar12;
  uint uVar13;
  uint uVar14;
  int iVar15;
  uint uStack_84;
  uint uStack_80;
  int *piStack_78;
  longlong lStack_70;
  
  lVar1 = param_2[7];
  lVar5 = param_2[4];
  lVar6 = param_2[5];
  lStack_70 = param_2[6];
  lVar4 = param_2[1];
  if (lVar4 == 0) {
    uVar14 = 0;
    if (param_2[3] == 0) {
      uVar11 = 0;
      goto code_r0x000140c10a98;
    }
    puVar12 = (undefined1 *)param_2[2];
    puVar7 = puVar12 + param_2[3];
code_r0x000140c10a54:
    uVar11 = 0;
    uVar8 = 0;
    do {
      uVar3 = *puVar12;
      puVar12 = puVar12 + 1;
      func_0x000140c10dd0(&uStack_84,uVar3);
      uVar8 = uVar8 | uStack_84;
    } while (puVar12 != puVar7);
    uVar13 = 0;
    piStack_78 = param_1;
  }
  else {
    lVar10 = *param_2;
    lVar9 = 0;
    uVar14 = 0;
    uVar11 = 0;
    piStack_78 = param_1;
    do {
      cVar2 = *(char *)(lVar10 + lVar9);
      func_0x000140c10dd0(&uStack_84,cVar2);
      uVar13 = 0;
      if ((byte)(cVar2 - 0x28U) < 0x10) {
        uVar13 = uStack_80;
      }
      uVar11 = uVar11 | uVar13;
      uVar14 = uVar14 | uStack_84;
      lVar9 = lVar9 + 1;
    } while (lVar4 != lVar9);
    lVar4 = param_2[3];
    param_1 = piStack_78;
    if (lVar4 == 0) {
code_r0x000140c10a98:
      iVar15 = uVar14 + (uVar14 == 0);
      uVar13 = 0;
      uVar14 = 0x2000;
      goto code_r0x000140c10aa8;
    }
    puVar12 = (undefined1 *)param_2[2];
    if (uVar11 == 0) {
      puVar7 = puVar12 + lVar4;
      goto code_r0x000140c10a54;
    }
    lVar10 = 0;
    uVar8 = 0;
    uVar13 = 0;
    do {
      func_0x000140c10dd0(&uStack_84,puVar12[lVar10]);
      uVar8 = uVar8 | uStack_84;
      uVar13 = uVar13 | uStack_80;
      lVar10 = lVar10 + 1;
    } while (lVar4 != lVar10);
  }
  iVar15 = uVar14 + (uVar14 == 0);
  param_1 = piStack_78;
  uVar14 = 0x2000;
  if (uVar8 != 0) {
    uVar14 = uVar8;
  }
code_r0x000140c10aa8:
  *param_1 = iVar15;
  param_1[1] = uVar14;
  param_1[2] = 0x2c;
  param_1[4] = 0;
  param_1[5] = 0;
  param_1[6] = uVar11;
  param_1[7] = uVar13;
  *(longlong *)(param_1 + 8) = lVar1;
  *(longlong *)(param_1 + 10) = lVar5;
  *(longlong *)(param_1 + 0xc) = lVar6;
  *(longlong *)(param_1 + 0xe) = lStack_70;
  return param_1;
}

