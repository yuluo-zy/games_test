
undefined8 *
FUN_1400315e0(undefined8 *param_1,longlong param_2,undefined8 *param_3,undefined8 *param_4)

{
  code *pcVar1;
  undefined *puVar2;
  undefined8 *puVar3;
  int iStack_80;
  undefined4 uStack_7c;
  undefined4 uStack_78;
  undefined4 uStack_74;
  undefined8 uStack_70;
  int iStack_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined8 uStack_58;
  byte abStack_48 [4];
  undefined4 uStack_44;
  undefined8 uStack_40;
  int iStack_38;
  undefined4 uStack_34;
  undefined4 uStack_30;
  undefined4 uStack_2c;
  undefined8 uStack_28;
  
  func_0x000140698320(&iStack_80,*(longlong *)*param_3 + 0x10);
  if (iStack_80 == 0xc) {
    iStack_68 = CONCAT31(iStack_68._1_3_,(undefined1)uStack_7c);
    puVar2 = &UNK_142928fb0;
  }
  else {
    uStack_28 = uStack_70;
    iStack_38 = iStack_80;
    uStack_34 = uStack_7c;
    uStack_30 = uStack_78;
    uStack_2c = uStack_74;
    FUN_1406982e0(&iStack_80,*(longlong *)*param_4 + 0x10);
    if (iStack_80 != 0xf) {
      uStack_58 = uStack_70;
      iStack_68 = iStack_80;
      uStack_64 = uStack_7c;
      uStack_60 = uStack_78;
      uStack_5c = uStack_74;
      FUN_140c0f820(abStack_48,param_2,param_2 + 0x30,&iStack_38,&iStack_68,0);
      if ((abStack_48[0] & 1) == 0) {
        FUN_140c876e0(param_1,param_2,param_2 + 0x30);
        param_1[6] = uStack_40;
      }
      else {
        *(undefined4 *)(param_1 + 1) = uStack_44;
        *param_1 = 0;
      }
      return param_1;
    }
    iStack_68 = CONCAT31(iStack_68._1_3_,(undefined1)uStack_7c);
    puVar2 = &UNK_142928fc8;
  }
  FUN_1428d9760(&UNK_142928ce0,0x2b,&iStack_68,&UNK_142928cc0,puVar2);
  pcVar1 = (code *)swi(3);
  puVar3 = (undefined8 *)(*pcVar1)();
  return puVar3;
}

