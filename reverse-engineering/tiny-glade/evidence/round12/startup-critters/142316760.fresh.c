
void FUN_142316760(longlong *param_1,undefined8 *param_2)

{
  code *pcVar1;
  int iVar2;
  undefined8 *puVar3;
  longlong lVar4;
  undefined8 auStack_f0 [6];
  longlong lStack_c0;
  longlong lStack_b8;
  longlong lStack_b0;
  longlong lStack_a8;
  longlong lStack_a0;
  longlong lStack_98;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  undefined8 uStack_60;
  undefined8 uStack_58;
  undefined1 uStack_50;
  undefined *puStack_48;
  undefined8 uStack_40;
  longlong lStack_38;
  undefined8 uStack_30;
  longlong lStack_28;
  undefined8 uStack_20;
  
  uStack_20 = 0xfffffffffffffffe;
  lStack_a8 = *param_1;
  lStack_c0 = param_1[1];
  lStack_b8 = lStack_c0 + 0x1a0;
  lStack_b0 = lStack_c0 + 0xf8;
  lStack_a0 = *(longlong *)(lStack_a8 + 0xf8);
  lStack_98 = lStack_a0 + *(longlong *)(lStack_a8 + 0x100) * 4;
  uStack_50 = *(undefined1 *)(lStack_a8 + 0x108);
  uStack_90 = 8;
  uStack_88 = 0;
  uStack_80 = 8;
  uStack_78 = 0;
  uStack_70 = 0;
  uStack_68 = 0;
  uStack_60 = 0;
  uStack_58 = 0;
  FUN_141dbed60(&lStack_38,&lStack_c0,&UNK_14304ad98);
  if (lStack_28 != 0) {
    if (lStack_28 == 0x60) {
      puVar3 = (undefined8 *)FUN_14230e780(*param_2,&UNK_14304a250);
      if (puVar3 == (undefined8 *)0x0) goto LAB_14231688c;
      puStack_48 = &UNK_14304a455;
      uStack_40 = 0x44;
      if (puVar3[3] == 0x44) {
        iVar2 = func_0x000142923bf0(&UNK_14304a455,puVar3[2],0x44);
        if (iVar2 == 0) {
          lVar4 = (**(code **)(puVar3[1] + 0x18))(*puVar3);
          if (lVar4 != 0) {
            FUN_14230f1e0(lVar4,uStack_30,0x60);
          }
          goto LAB_14231688c;
        }
      }
      auStack_f0[0] = 0;
      FUN_1428cab41(0,&puStack_48,puVar3 + 2,auStack_f0,&UNK_14304a2c0);
    }
    else {
      FUN_1428d9430(&UNK_14304adb0,0x37,&UNK_14304ae10);
    }
                    /* WARNING: Does not return */
    pcVar1 = (code *)invalidInstructionException();
    (*pcVar1)();
  }
LAB_14231688c:
  if (lStack_38 != 0) {
    func_0x000140613c20(uStack_30,lStack_38 << 4,4);
  }
  return;
}

