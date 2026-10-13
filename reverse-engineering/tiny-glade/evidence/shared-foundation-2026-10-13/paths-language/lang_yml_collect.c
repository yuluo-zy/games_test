
void FUN_1424950a0(undefined8 *param_1,undefined8 param_2)

{
  longlong lVar1;
  char *pcVar2;
  undefined8 uVar3;
  code *pcVar4;
  longlong unaff_RBX;
  longlong lVar5;
  char *pcVar6;
  longlong lVar7;
  longlong lStack_78;
  longlong lStack_70;
  longlong lStack_68;
  undefined8 uStack_60;
  longlong lStack_58;
  longlong lStack_50;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  lVar7 = param_1[2];
  if (lVar7 != 0) {
    pcVar6 = (char *)*param_1;
    uStack_60 = param_2;
    do {
      if (*pcVar6 != '\x02') {
        FUN_1428d9310(&UNK_1430d2540);
LAB_1424951f9:
        FUN_1428d9310(&UNK_1430d2558);
        goto LAB_14249524e;
      }
      lVar7 = lVar7 + -1;
      lVar1 = *(longlong *)(pcVar6 + 0x18);
      if (lVar1 < 0) {
        FUN_1428d8fe3(0,lStack_50,&UNK_1430d2500);
LAB_14249521d:
        pcVar6 = (char *)0x0;
        lVar5 = unaff_RBX;
LAB_14249523c:
        FUN_1428d8fe3(pcVar6,lVar5,&UNK_1430d2500);
LAB_14249524e:
                    /* WARNING: Does not return */
        pcVar4 = (code *)invalidInstructionException();
        (*pcVar4)();
      }
      pcVar2 = *(char **)(pcVar6 + 0x70);
      lVar5 = *(longlong *)(pcVar6 + 0x10);
      if (lVar1 != 0) {
        lStack_50 = func_0x000140613c10(lVar1,1);
        if (lStack_50 != 0) goto LAB_142495185;
        lStack_50 = lVar1;
        FUN_1428d8fe3(1,lVar1,&UNK_1430d2500);
        goto LAB_14249523c;
      }
      lStack_50 = 1;
LAB_142495185:
      lStack_58 = lVar1;
      func_0x000142923c00(lStack_50,lVar5);
      if (pcVar6[0x38] != '\x02') goto LAB_1424951f9;
      lVar5 = *(longlong *)(pcVar6 + 0x50);
      if (lVar5 < 0) goto LAB_14249521d;
      uVar3 = *(undefined8 *)(pcVar6 + 0x48);
      if (lVar5 == 0) {
        unaff_RBX = 1;
      }
      else {
        pcVar6 = (char *)0x1;
        unaff_RBX = func_0x000140613c10(lVar5,1);
        if (unaff_RBX == 0) goto LAB_14249523c;
      }
      func_0x000142923c00(unaff_RBX,uVar3,lVar5);
      lStack_78 = lStack_58;
      lStack_70 = lStack_50;
      lStack_68 = lStack_58;
      FUN_14212cc80(uStack_60,&lStack_78,unaff_RBX,lVar5);
      pcVar6 = pcVar2;
    } while (lVar7 != 0);
  }
  return;
}

