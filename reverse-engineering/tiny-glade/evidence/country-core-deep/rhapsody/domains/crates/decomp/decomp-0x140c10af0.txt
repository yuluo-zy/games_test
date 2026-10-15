
int * FUN_140c10af0(int *param_1,longlong *param_2)

{
  longlong lVar1;
  char cVar2;
  char cVar3;
  longlong lVar4;
  code *pcVar5;
  uint uVar6;
  int *piVar7;
  uint uVar8;
  uint uVar9;
  longlong lVar10;
  uint uVar11;
  int iVar12;
  uint uVar13;
  uint uVar14;
  longlong lVar15;
  uint uStack_8c;
  uint uStack_88;
  uint uStack_84;
  longlong *plStack_80;
  ulonglong uStack_78;
  int *piStack_70;
  longlong lStack_68;
  longlong lStack_60;
  
  lVar1 = param_2[5];
  lStack_68 = param_2[4];
  lVar4 = param_2[1];
  plStack_80 = param_2;
  if (lVar4 == 0) {
    uStack_78 = 0;
    uVar13 = 0;
    uVar14 = 0;
    lVar4 = param_2[3];
  }
  else {
    lVar15 = *param_2;
    if (*(char *)((longlong)param_2 + 0x46) == '\x01') {
      lVar10 = 0;
      uVar14 = 0;
      uVar13 = 0;
      do {
        cVar2 = *(char *)(lVar15 + lVar10);
        func_0x000140c10dd0(&uStack_8c,cVar2);
        uVar6 = 0;
        if ((byte)(cVar2 - 0x28U) < 0x10) {
          uVar6 = uStack_88;
        }
        uVar13 = uVar13 | uVar6;
        uVar14 = uVar14 | uStack_8c;
        lVar10 = lVar10 + 1;
      } while (lVar4 != lVar10);
      uStack_78 = 0;
      lVar4 = plStack_80[3];
    }
    else {
      if (*(char *)((longlong)param_2 + 0x44) == '\0') {
        lVar10 = 0;
        uVar14 = 0;
        uVar13 = 0;
        do {
          cVar2 = *(char *)(lVar15 + lVar10);
          func_0x000140c10dd0(&uStack_8c,cVar2);
          uVar6 = 0;
          if ((byte)(cVar2 - 0x28U) < 0x10) {
            uVar6 = uStack_88;
          }
          uVar13 = uVar13 | uVar6;
          uVar14 = uVar14 | uStack_8c;
          lVar10 = lVar10 + 1;
        } while (lVar4 != lVar10);
        uStack_78 = (ulonglong)uStack_84;
      }
      else {
        if (*(char *)((longlong)param_2 + 0x44) != '\x01') {
code_r0x000140c10daa:
          FUN_1428d9430(&UNK_142b1a658,0xf,&UNK_142b1a6b0);
          pcVar5 = (code *)swi(3);
          piVar7 = (int *)(*pcVar5)();
          return piVar7;
        }
        lVar10 = 0;
        uVar14 = 0;
        uVar13 = 0;
        do {
          cVar2 = *(char *)(lVar15 + lVar10);
          func_0x000140c10dd0(&uStack_8c,cVar2);
          uVar6 = 0;
          if ((byte)(cVar2 - 0x28U) < 0x10) {
            uVar6 = uStack_88;
          }
          uVar13 = uVar13 | uVar6;
          uVar14 = uVar14 | uStack_8c;
          lVar10 = lVar10 + 1;
        } while (lVar4 != lVar10);
        uStack_78 = 1;
        if (cVar2 == '\'') {
          uStack_78 = 0x3b9acdea;
        }
      }
      lVar4 = plStack_80[3];
    }
  }
  piStack_70 = param_1;
  if (lVar4 == 0) {
    iVar12 = (int)uStack_78;
    uVar8 = 0x2000;
    uVar6 = 0;
    uVar11 = 0;
  }
  else {
    lStack_60 = plStack_80[2];
    cVar2 = *(char *)((longlong)plStack_80 + 0x45);
    lVar15 = 0;
    uVar9 = 0;
    uVar11 = 0;
    do {
      while( true ) {
        cVar3 = *(char *)(lStack_60 + lVar15);
        func_0x000140c10dd0(&uStack_8c,cVar3);
        uVar9 = uVar9 | uStack_8c;
        uVar11 = uVar11 | uStack_88;
        if (cVar2 == '\0') break;
        if (cVar2 != '\x01') {
          FUN_1428d9430(&UNK_142b1a658,0xf,&UNK_142b1a698);
          goto code_r0x000140c10daa;
        }
        uVar6 = 1;
        if (cVar3 == '\'') {
          uVar6 = 0x3b9acdea;
        }
        lVar15 = lVar15 + 1;
        if (lVar4 == lVar15) goto code_r0x000140c10c80;
      }
      lVar15 = lVar15 + 1;
      uVar6 = uStack_84;
    } while (lVar4 != lVar15);
code_r0x000140c10c80:
    uVar8 = 0x2000;
    if (uVar9 != 0) {
      uVar8 = uVar9;
    }
    iVar12 = (int)uStack_78;
  }
  piStack_70[0x12] = (int)plStack_80[8];
  lVar4 = plStack_80[7];
  *(longlong *)(piStack_70 + 0xe) = plStack_80[6];
  *(longlong *)(piStack_70 + 0x10) = lVar4;
  *piStack_70 = uVar14 + (uVar14 == 0);
  piStack_70[1] = uVar8;
  piStack_70[2] = 0x2d;
  piStack_70[4] = 0;
  piStack_70[5] = 0;
  piStack_70[6] = uVar13;
  piStack_70[7] = uVar11;
  piStack_70[8] = iVar12;
  piStack_70[9] = uVar6;
  *(longlong *)(piStack_70 + 10) = lVar1;
  *(longlong *)(piStack_70 + 0xc) = lStack_68;
  return piStack_70;
}

