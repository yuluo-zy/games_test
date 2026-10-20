
longlong FUN_1404f4c30(longlong param_1,longlong *param_2)

{
  longlong lVar1;
  longlong lVar2;
  longlong lVar3;
  longlong lVar4;
  float fVar5;
  char cVar6;
  char cVar7;
  int iVar8;
  longlong *plVar9;
  char cVar10;
  longlong lVar11;
  longlong *plStack_80;
  char cStack_78;
  int iStack_6c;
  int iStack_68;
  int iStack_64;
  longlong lStack_60;
  
  iVar8 = *(int *)(param_1 + 0x28);
  lVar1 = param_1 + 0x49;
  func_0x000140833230(lVar1);
  fVar5 = *(float *)(param_1 + 0x30);
  lVar2 = param_1 + 0x4c;
  func_0x0001408c5680(lVar2);
  iStack_64 = *(int *)(param_1 + 0x34);
  lVar3 = param_1 + 0x38;
  func_0x000140b3f910(lVar3);
  lVar4 = param_1 + 0x4d;
  func_0x0001408c5680(lVar4);
  lStack_60 = param_1 + 0x3c;
  func_0x000140b3f910();
  iStack_68 = *(int *)(param_1 + 0x40);
  iStack_6c = *(int *)(param_1 + 0x44);
  cVar6 = *(char *)(param_1 + 0x4b);
  cVar7 = *(char *)(param_1 + 0x4e);
  plVar9 = (longlong *)*param_2;
  lVar11 = plVar9[2];
  if (*plVar9 == lVar11) {
    FUN_1428a29e0(plVar9,lVar11,1,1,1);
    lVar11 = plVar9[2];
  }
  *(undefined1 *)(plVar9[1] + lVar11) = 0x7b;
  plVar9[2] = lVar11 + 1;
  cStack_78 = '\x01';
  plStack_80 = param_2;
  lVar11 = FUN_1404c0220(&plStack_80,&UNK_1429ec6e2,3,param_1);
  if ((((((((lVar11 == 0) &&
           (lVar11 = FUN_1404c10c0(&plStack_80,&UNK_1429ec6e5,2,param_1 + 0x4a), lVar11 == 0)) &&
          (lVar11 = FUN_1404bd3f0(&plStack_80,&UNK_1429ec6e7,4,param_1 + 0x48), lVar11 == 0)) &&
         ((iVar8 == 0 ||
          (lVar11 = FUN_1404baab0(&plStack_80,&UNK_1429ec6eb,8,param_1 + 0x28), lVar11 == 0)))) &&
        ((lVar11 = FUN_1404ba220(&plStack_80,&UNK_1429ec6f3,4,param_1 + 0x2c), lVar11 == 0 &&
         ((cVar10 = func_0x000140833230(lVar1), cVar10 != '\0' ||
          (lVar11 = FUN_1404b8720(&plStack_80,&UNK_1429ec6f7,3,lVar1), lVar11 == 0)))))) &&
       (((fVar5 == 0.0 && (!NAN(fVar5))) ||
        (lVar11 = FUN_1404bdc80(&plStack_80,&UNK_1429ec6fa,8,param_1 + 0x30), lVar11 == 0)))) &&
      ((((((cVar10 = func_0x0001408c5680(lVar2), cVar10 != '\0' ||
           (lVar11 = FUN_1404bc390(&plStack_80,&UNK_1429ec702,0x10,lVar2), lVar11 == 0)) &&
          ((iStack_64 == 0 ||
           (lVar11 = FUN_1404baab0(&plStack_80,&UNK_1429ec712,0x12,param_1 + 0x34), lVar11 == 0))))
         && (((cVar10 = func_0x000140b3f910(lVar3), cVar10 != '\0' ||
              (lVar11 = FUN_1404bdc80(&plStack_80,&UNK_1429ec724,0x12,lVar3), lVar11 == 0)) &&
             ((cVar10 = func_0x0001408c5680(lVar4), cVar10 != '\0' ||
              (lVar11 = FUN_1404bc390(&plStack_80,&UNK_1429ec736,0xd,lVar4), lVar11 == 0)))))) &&
        ((((lVar1 = lStack_60, cVar10 = func_0x000140b3f910(lStack_60), cVar10 != '\0' ||
           (lVar11 = FUN_1404bdc80(&plStack_80,&UNK_1429ec743,0xb,lVar1), lVar11 == 0)) &&
          ((iStack_68 == 0 ||
           (lVar11 = FUN_1404baab0(&plStack_80,&UNK_1429ec74e,0xc,param_1 + 0x40), lVar11 == 0))))
         && ((cVar6 == '\0' ||
             (lVar11 = FUN_1404b7780(&plStack_80,&UNK_1429ec75a,7,param_1 + 0x4b), lVar11 == 0))))))
       && ((cVar7 == '\0' ||
           (lVar11 = FUN_1404be900(&plStack_80,&UNK_1429ec761,0xf,param_1 + 0x4e), lVar11 == 0))))))
     && ((iStack_6c == 0 ||
         (lVar11 = FUN_1404baab0(&plStack_80,&UNK_1429ec770,0xb,param_1 + 0x44), lVar11 == 0)))) {
    if (cStack_78 != '\0') {
      FUN_1402b5870(*plStack_80,&UNK_1429ec2e8,1,&UNK_1429ec8e8);
    }
    lVar11 = 0;
  }
  return lVar11;
}

