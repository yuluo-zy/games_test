
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

longlong FUN_1417c0960(longlong param_1,longlong *param_2)

{
  float fVar1;
  char cVar2;
  char cVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  int iVar7;
  longlong *plVar8;
  longlong lVar9;
  longlong *plStack_70;
  char cStack_68;
  longlong *plStack_60;
  
  iVar4 = *(int *)(param_1 + 0x28);
  fVar1 = *(float *)(param_1 + 0x30);
  iVar5 = *(int *)(param_1 + 0x34);
  iVar6 = *(int *)(param_1 + 0x40);
  cVar2 = *(char *)(param_1 + 0x4b);
  cVar3 = *(char *)(param_1 + 0x4e);
  iVar7 = *(int *)(param_1 + 0x44);
  plVar8 = (longlong *)*param_2;
  lVar9 = plVar8[2];
  if (*plVar8 == lVar9) {
    plStack_60 = param_2;
    FUN_1428a9a60(plVar8,lVar9,1,1,1);
    lVar9 = plVar8[2];
    param_2 = plStack_60;
  }
  *(undefined1 *)(plVar8[1] + lVar9) = 0x7b;
  plVar8[2] = lVar9 + 1;
  cStack_68 = '\x01';
  plStack_70 = param_2;
  lVar9 = FUN_141582ea0(&plStack_70,&UNK_142ce172d,3,param_1);
  if (((((((lVar9 == 0) &&
          (lVar9 = FUN_14158a500(&plStack_70,&UNK_142ce1730,2,param_1 + 0x4a), lVar9 == 0)) &&
         (lVar9 = FUN_14158b8c0(&plStack_70,&UNK_142ce1732,4,param_1 + 0x48), lVar9 == 0)) &&
        ((iVar4 == 0 ||
         (lVar9 = FUN_141583a40(&plStack_70,&UNK_142ce1736,8,param_1 + 0x28), lVar9 == 0)))) &&
       ((lVar9 = FUN_141588c60(&plStack_70,&UNK_142ce173e,4,param_1 + 0x2c), lVar9 == 0 &&
        ((*(char *)(param_1 + 0x49) == '\0' ||
         (lVar9 = FUN_14158b9a0(&plStack_70,&UNK_142ce1742,3), lVar9 == 0)))))) &&
      ((((fVar1 == 0.0 && (!NAN(fVar1))) ||
        (lVar9 = FUN_141586090(&plStack_70,&UNK_142ce1745,8,param_1 + 0x30), lVar9 == 0)) &&
       ((((*(char *)(param_1 + 0x4c) != '\0' ||
          (lVar9 = FUN_141588150(&plStack_70,&UNK_142ce174d,0x10), lVar9 == 0)) &&
         ((iVar5 == 0 ||
          (lVar9 = FUN_141583a40(&plStack_70,&UNK_142ce175d,0x12,param_1 + 0x34), lVar9 == 0)))) &&
        ((((*(float *)(param_1 + 0x38) == _UNK_142925904 &&
           (!NAN(*(float *)(param_1 + 0x38)) && !NAN(_UNK_142925904))) ||
          (lVar9 = FUN_141586090(&plStack_70,&UNK_142ce176f,0x12), lVar9 == 0)) &&
         ((*(char *)(param_1 + 0x4d) != '\0' ||
          (lVar9 = FUN_141588150(&plStack_70,&UNK_142ce1781,0xd), lVar9 == 0)))))))))) &&
     ((((*(float *)(param_1 + 0x3c) == _UNK_142925904 &&
        (!NAN(*(float *)(param_1 + 0x3c)) && !NAN(_UNK_142925904))) ||
       (lVar9 = FUN_141586090(&plStack_70,&UNK_142ce178e,0xb), lVar9 == 0)) &&
      ((((iVar6 == 0 ||
         (lVar9 = FUN_141583a40(&plStack_70,&UNK_142ce1799,0xc,param_1 + 0x40), lVar9 == 0)) &&
        ((cVar2 == '\0' ||
         (lVar9 = FUN_141583160(&plStack_70,&UNK_142ce17a5,7,param_1 + 0x4b), lVar9 == 0)))) &&
       (((cVar3 == '\0' ||
         (lVar9 = FUN_14158cf00(&plStack_70,&UNK_142ce17ac,0xf,param_1 + 0x4e), lVar9 == 0)) &&
        ((iVar7 == 0 ||
         (lVar9 = FUN_141583a40(&plStack_70,&UNK_142ce17bb,0xb,param_1 + 0x44), lVar9 == 0))))))))))
  {
    if (cStack_68 != '\0') {
      FUN_1412f4520(*plStack_70,&UNK_142ce11e0,1,&UNK_142ce1298);
    }
    lVar9 = 0;
  }
  return lVar9;
}

