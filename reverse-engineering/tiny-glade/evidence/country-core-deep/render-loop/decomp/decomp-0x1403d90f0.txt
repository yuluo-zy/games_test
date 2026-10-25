
void FUN_1403d90f0(undefined8 param_1,longlong param_2,undefined4 param_3)

{
  undefined4 uVar1;
  code *pcVar2;
  longlong lVar3;
  longlong lVar4;
  undefined8 uVar5;
  ulonglong uVar6;
  int *piStack_3c0;
  undefined *puStack_3b8;
  undefined *puStack_230;
  undefined8 uStack_228;
  int **ppiStack_220;
  undefined8 uStack_218;
  undefined8 uStack_210;
  undefined1 auStack_120 [38];
  char cStack_fa;
  longlong lStack_f8;
  longlong lStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined *puStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined **ppuStack_70;
  undefined *puStack_68;
  int iStack_5c;
  longlong lStack_58;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  uVar6 = 0x6dc90dcfd23786fa;
  lVar3 = FUN_1400dd6f0(param_2 + 0x38,0x6dc90dcfd23786fa,0xed952bd348f66c04);
  if (((lVar3 != 0) && (uVar6 < *(ulonglong *)(param_2 + 0x218))) &&
     (uVar6 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar6 * 8), uVar6 != 0)) {
    lVar3 = ~uVar6 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar3) != 0) {
      lVar3 = *(longlong *)(param_2 + 0x1e0) + lVar3;
      uVar5 = *(undefined8 *)(lVar3 + 0x10);
      uVar1 = *(undefined4 *)(param_2 + 0x5b0);
      lVar4 = func_0x00014067dd80(uVar5);
      if (*(char *)(lVar4 + 0x29a) != '\0') {
        FUN_140a59890(&puStack_230,param_3);
        iStack_5c = (int)uStack_228;
        if ((int)uStack_228 == 0x4c) {
          uStack_80 = 4;
          puStack_88 = &UNK_1429d361f;
        }
        else {
          if ((int)uStack_228 != 0x61) goto code_r0x0001403d9415;
          uStack_80 = 7;
          puStack_88 = &UNK_1429d3618;
        }
        *(undefined4 *)(lVar3 + 0x5c) = uVar1;
        lVar3 = func_0x00014067dd80(uVar5);
        *(undefined2 *)(lVar3 + 0x29a) = 0x100;
        puStack_68 = &UNK_140109e90;
        puStack_230 = &UNK_1429d3640;
        uStack_228 = 1;
        uStack_218 = 1;
        uStack_210 = 0;
        ppiStack_220 = (int **)&ppuStack_70;
        ppuStack_70 = &puStack_88;
        FUN_140d79f90(&piStack_3c0,&puStack_230);
        FUN_1405876a0(&puStack_230,&UNK_1429d3650,0x11,&piStack_3c0,param_2,param_1);
        FUN_1405889f0(&piStack_3c0,&puStack_230,0x2f52917136b0d5bb);
        puStack_68 = &UNK_140109e90;
        puStack_230 = &UNK_1429d3670;
        uStack_228 = 1;
        uStack_218 = 1;
        uStack_210 = 0;
        ppiStack_220 = (int **)&ppuStack_70;
        ppuStack_70 = &puStack_88;
        FUN_140d79f90(&lStack_a0,&puStack_230);
        lStack_58 = lStack_a0;
        uStack_49 = 1;
        uStack_78 = uStack_98;
        uVar5 = FUN_140c87660(uStack_98,uStack_90);
        ppuStack_70 = (undefined **)CONCAT71(ppuStack_70._1_7_,(char)param_3);
        puStack_230 = (undefined *)CONCAT44(puStack_230._4_4_,2);
        FUN_140363db0(&ppuStack_70,&piStack_3c0,uVar5,0x32,&puStack_230);
        func_0x000142923c00(&puStack_230,&piStack_3c0,400);
        if (cStack_fa != '\b') {
          FUN_14048f570(auStack_120,lStack_f8 + 0xe8,0xb66175286d72dcb9,0x200);
        }
        func_0x000142923c00(&piStack_3c0,&puStack_230,400);
        FUN_14096ec80(&piStack_3c0,0x20000000080);
        if (lStack_58 != 0) {
          func_0x000140613c20(uStack_78,lStack_58,1);
        }
      }
      return;
    }
  }
  FUN_1428d9310(&UNK_1429d3600);
code_r0x0001403d9415:
  piStack_3c0 = &iStack_5c;
  puStack_3b8 = &UNK_140c87d80;
  puStack_230 = &UNK_1429d3698;
  uStack_228 = 1;
  uStack_210 = 0;
  ppiStack_220 = &piStack_3c0;
  uStack_218 = 1;
  FUN_1428d9390(&puStack_230,&UNK_1429d36a8);
  pcVar2 = (code *)swi(3);
  (*pcVar2)();
  return;
}

