
longlong FUN_140fac1f0(longlong param_1,undefined4 param_2,undefined1 param_3,longlong param_4)

{
  code *pcVar1;
  longlong lVar2;
  undefined8 unaff_RBX;
  undefined8 auStack_380 [4];
  undefined8 uStack_360;
  undefined1 auStack_348 [320];
  undefined4 uStack_208;
  undefined8 uStack_200;
  undefined8 uStack_1f8;
  undefined1 uStack_1f0;
  undefined4 uStack_1e8;
  undefined8 uStack_1e0;
  undefined8 uStack_1d8;
  undefined8 uStack_1d0;
  undefined4 uStack_1c8;
  undefined8 uStack_1c0;
  undefined8 uStack_1b8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  longlong lStack_58;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  longlong lStack_48;
  undefined8 uStack_40;
  
  uStack_40 = 0xfffffffffffffffe;
  lStack_58 = param_4;
  lStack_48 = param_4;
  uStack_50 = FUN_140b906c0(param_2);
  uStack_4c = param_2;
  lVar2 = FUN_140b91fa0(lStack_48 + 0x10,
                        CONCAT71((int7)((ulonglong)unaff_RBX >> 8),param_3) & 0xffffffff);
  if (lVar2 != 0) {
    func_0x000140b92090(&uStack_70,lVar2,&uStack_50);
    FUN_140e15190(&uStack_1e0,uStack_60,uStack_5c);
    func_0x000142923c00(auStack_348,&uStack_1e0,0x140);
    uStack_200 = 0;
    uStack_1f8 = 0;
    uStack_1f0 = 0;
    uStack_208 = 0;
    auStack_380[0] = 0;
    uStack_1e8 = 4;
    uStack_360 = 0x8000000000000000;
    uStack_1e0 = 0;
    uStack_1d8 = uStack_70;
    uStack_1d0 = uStack_68;
    uStack_1c8 = 0;
    uStack_1c0 = 0;
    uStack_1b8 = 0x400019000000000;
    FUN_141804bd0(&uStack_a0,&uStack_1e0);
    *(undefined8 *)(param_1 + 0x1e8) = 0x8000000000000000;
    func_0x000142923c00(param_1,auStack_380,0x1a0);
    *(undefined1 *)(param_1 + 0x254) = 0;
    *(undefined8 *)(param_1 + 0x1a0) = 0;
    *(undefined8 *)(param_1 + 0x1a8) = 8;
    *(undefined8 *)(param_1 + 0x1b0) = 0;
    *(undefined8 *)(param_1 + 0x200) = uStack_a0;
    *(undefined8 *)(param_1 + 0x208) = uStack_98;
    *(undefined8 *)(param_1 + 0x210) = uStack_90;
    *(undefined8 *)(param_1 + 0x218) = uStack_88;
    *(undefined8 *)(param_1 + 0x220) = uStack_80;
    *(undefined8 *)(param_1 + 0x228) = uStack_78;
    *(undefined4 *)(param_1 + 0x248) = 0x7f7fffff;
    *(undefined1 *)(param_1 + 0x256) = 5;
    *(undefined8 *)(param_1 + 0x24c) = 0;
    *(undefined8 *)(param_1 + 0x1b8) = 0;
    *(undefined8 *)(param_1 + 0x1c0) = 4;
    *(undefined8 *)(param_1 + 0x1c8) = 0;
    *(undefined8 *)(param_1 + 0x1d0) = 0;
    *(undefined8 *)(param_1 + 0x1d8) = 4;
    *(undefined8 *)(param_1 + 0x1e0) = 0;
    *(undefined8 *)(param_1 + 0x238) = 0;
    *(longlong *)(param_1 + 0x230) = lStack_48;
    *(undefined1 *)(param_1 + 0x255) = param_3;
    *(ulonglong *)(param_1 + 0x240) = CONCAT44(uStack_4c,uStack_50);
    return param_1;
  }
  FUN_1428d9310(&UNK_142bb0ed0);
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

