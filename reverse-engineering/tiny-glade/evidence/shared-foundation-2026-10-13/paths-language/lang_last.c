
undefined8 FUN_140b9d180(void)

{
  code *pcVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  longlong lStack_90;
  undefined4 *puStack_88;
  undefined8 uStack_80;
  longlong lStack_78;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  undefined4 uStack_68;
  undefined4 uStack_64;
  undefined8 uStack_60;
  longlong lStack_58;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  undefined4 uStack_48;
  undefined4 uStack_44;
  undefined8 uStack_40;
  longlong lStack_38;
  undefined8 *puStack_30;
  longlong lStack_28;
  undefined8 uStack_20;
  
  uStack_20 = 0xfffffffffffffffe;
  FUN_140b9dc80(&lStack_78);
  if (SBORROW8(0,lStack_78)) {
    func_0x000140d4cc30();
  }
  else {
    uStack_40 = uStack_60;
    uStack_50 = uStack_70;
    uStack_4c = uStack_6c;
    uStack_48 = uStack_68;
    uStack_44 = uStack_64;
    lStack_58 = lStack_78;
    uVar3 = FUN_140d76ed0(&lStack_58);
    FUN_140d62810(&lStack_78,uVar3,uStack_60,&UNK_142afd670,0x10);
    uVar2 = FUN_140d76ed0(&lStack_78);
    FUN_140d59140(&lStack_90,uVar2,uVar3);
    if (lStack_78 != 0) {
      func_0x000140613c20(CONCAT44(uStack_6c,uStack_70),lStack_78,1);
    }
    if (!SBORROW8(0,lStack_90)) {
      lStack_28 = lStack_90;
      if (lStack_58 != 0) {
        func_0x000140613c20(CONCAT44(uStack_4c,uStack_50),lStack_58,1);
      }
      goto LAB_140b9d2ec;
    }
    if (((uint)puStack_88 & 3) == 1) {
      lStack_38 = (longlong)puStack_88 + -1;
      lStack_28 = *(undefined8 *)((longlong)puStack_88 + -1);
      puStack_30 = *(undefined8 **)((longlong)puStack_88 + 7);
      if ((code *)*puStack_30 != (code *)0x0) {
        (*(code *)*puStack_30)(lStack_28);
      }
      if (puStack_30[1] != 0) {
        func_0x000140613c20(lStack_28,puStack_30[1],puStack_30[2]);
      }
      func_0x000140613c20(lStack_38,0x18,8);
    }
    if (lStack_58 != 0) {
      func_0x000140613c20(CONCAT44(uStack_4c,uStack_50),lStack_58,1);
    }
  }
  puStack_88 = (undefined4 *)func_0x000140613c10(5,1);
  if (puStack_88 == (undefined4 *)0x0) {
    FUN_1428d8fe3(1,5,&UNK_142afd628);
    pcVar1 = (code *)swi(3);
    uVar3 = (*pcVar1)();
    return uVar3;
  }
  lStack_28 = 5;
  *(undefined1 *)(puStack_88 + 1) = 0x53;
  *puStack_88 = 0x552d6e65;
  uStack_80 = 5;
LAB_140b9d2ec:
  puStack_30 = (undefined8 *)puStack_88;
  uVar3 = FUN_140b9c4a0(puStack_88,uStack_80);
  if (lStack_28 != 0) {
    func_0x000140613c20(puStack_30,lStack_28,1);
  }
  return uVar3;
}

