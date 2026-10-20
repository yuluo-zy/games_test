
void FUN_140b0e0f0(undefined8 *param_1,longlong param_2)

{
  longlong lVar1;
  code *pcVar2;
  undefined4 *puVar3;
  undefined4 uVar4;
  longlong lVar5;
  longlong *plVar6;
  undefined1 uStack_c9;
  undefined8 uStack_c8;
  undefined4 uStack_c0;
  undefined8 *puStack_b8;
  code *pcStack_b0;
  undefined8 uStack_a8;
  undefined4 uStack_a0;
  undefined8 uStack_98;
  undefined4 *puStack_90;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined4 uStack_78;
  undefined4 uStack_74;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  ulonglong uStack_68;
  undefined *puStack_60;
  undefined8 uStack_58;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  undefined *puStack_48;
  undefined8 uStack_40;
  undefined8 **ppuStack_38;
  undefined8 uStack_30;
  undefined8 uStack_28;
  
  lVar5 = param_2;
  uStack_c8 = FUN_140d635e0();
  uVar4 = (undefined4)lVar5;
  *(undefined4 *)param_1[2] = *(undefined4 *)((longlong)param_1 + 0x1c);
  uStack_c0 = uVar4;
  FUN_140a57100(*param_1);
  if (2 < *puRam000000014362c3a8) {
    uStack_a8 = FUN_140d63910(&uStack_c8);
    puStack_b8 = &uStack_a8;
    pcStack_b0 = FUN_140d8a550;
    uStack_a0 = uVar4;
    puVar3 = (undefined4 *)func_0x00014067dd80(&UNK_142ae7d98);
    uStack_78 = *puVar3;
    uStack_74 = puVar3[1];
    uStack_70 = puVar3[2];
    uStack_6c = puVar3[3];
    uStack_4c = puVar3[4];
    uStack_68 = 3;
    puStack_60 = &UNK_142ae7db0;
    uStack_58 = 0x21;
    puStack_48 = &UNK_142ae7d78;
    uStack_40 = 2;
    ppuStack_38 = &puStack_b8;
    uStack_30 = 1;
    uStack_28 = 0;
    uStack_98 = 0;
    puStack_90 = (undefined4 *)&UNK_142ae7db0;
    uStack_88 = 0x21;
    uStack_80 = 0;
    uStack_50 = 1;
    func_0x000140d4a330(&uStack_c9,&uStack_98);
  }
  puVar3 = (undefined4 *)func_0x000140613c10(0x1c,4);
  if (puVar3 != (undefined4 *)0x0) {
    *puVar3 = 0;
    puVar3[6] = 0;
    uStack_98 = 1;
    uStack_88 = 1;
    uStack_80 = 1;
    puStack_90 = puVar3;
    FUN_1412df330(param_2,&uStack_98,&UNK_142ae7dd8);
    plVar6 = *(longlong **)(param_2 + 8);
    if (plVar6 == (longlong *)0x0) {
      plVar6 = *(longlong **)(param_2 + 0x10);
    }
    lVar5 = plVar6[2];
    if ((ulonglong)(*plVar6 - lVar5) < 0xc) {
      FUN_1428a9a60(plVar6,lVar5,0xc,1,1);
    }
    lVar1 = plVar6[1];
    *(code **)(lVar1 + lVar5) = FUN_1412dccf0;
    *(undefined4 *)(lVar1 + 8 + lVar5) = 0;
    plVar6[2] = lVar5 + 0xc;
    uStack_98 = 2;
    uStack_68 = uStack_68 & 0xffffffff00000000;
    FUN_1412e0fa0(param_2,&uStack_98,&UNK_142ae7df0);
    lVar5 = plVar6[2];
    if ((ulonglong)(*plVar6 - lVar5) < 8) {
      FUN_1428a9a60(plVar6,lVar5,8,1,1);
    }
    *(code **)(plVar6[1] + lVar5) = FUN_1412d91b0;
    plVar6[2] = lVar5 + 8;
    return;
  }
  FUN_1428d8fe3(4,0x1c,&UNK_142c6d8c8);
  pcVar2 = (code *)swi(3);
  (*pcVar2)();
  return;
}

