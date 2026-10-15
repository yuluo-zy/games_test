
void FUN_140907f90(longlong param_1,longlong param_2,undefined8 param_3,undefined8 param_4,
                  longlong param_5,undefined8 param_6,undefined8 param_7)

{
  undefined4 uVar1;
  undefined4 uVar2;
  code *pcVar3;
  longlong lVar4;
  longlong lStack_210;
  undefined1 auStack_208 [176];
  longlong alStack_158 [28];
  longlong *plStack_78;
  undefined8 uStack_60;
  undefined8 uStack_58;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  longlong lStack_48;
  undefined1 uStack_39;
  undefined8 uStack_38;
  
  uStack_38 = 0xfffffffffffffffe;
  lVar4 = *(longlong *)(param_1 + 0xe0) + 0x90;
  uStack_39 = 1;
  lStack_48 = param_1;
  func_0x00014003bb80(2);
  uVar1 = *(undefined4 *)(param_2 + 0x10);
  uStack_39 = 1;
  FUN_1409dca90(lVar4,uVar1,2,1);
  uStack_39 = 1;
  func_0x00014003bb80(2);
  uVar2 = *(undefined4 *)(param_5 + 0x10);
  uStack_39 = 1;
  FUN_1409dca90(lVar4,uVar2,2,1);
  uStack_60 = param_4;
  uStack_58 = param_3;
  uStack_50 = uVar1;
  uStack_4c = uVar2;
  if (*(int *)(lStack_48 + 0x40) == 4) {
    if (*(longlong *)(lStack_48 + 0x48) != 0) {
      func_0x000140613c20(*(undefined8 *)(lStack_48 + 0x50),*(longlong *)(lStack_48 + 0x48) << 3,4);
    }
  }
  else if (*(int *)(lStack_48 + 0x40) == 1) {
    FUN_140be24b0(lStack_48 + 0x48);
  }
  lVar4 = lStack_48;
  *(undefined4 *)(lStack_48 + 0x40) = 8;
  *(undefined4 *)(lStack_48 + 0x44) = uStack_50;
  *(undefined4 *)(lStack_48 + 0x48) = uStack_4c;
  *(undefined8 *)(lStack_48 + 0x50) = uStack_58;
  *(undefined8 *)(lStack_48 + 0x58) = uStack_60;
  *(undefined8 *)(lStack_48 + 0x60) = param_6;
  *(undefined8 *)(lStack_48 + 0x68) = param_7;
  func_0x000142923c00(alStack_158,lStack_48,0xf8);
  lStack_210 = alStack_158[0];
  alStack_158[0] = 2;
  if (lStack_210 != 2) {
    func_0x000142923c00(auStack_208,lVar4 + 8,0xb0);
    lVar4 = plStack_78[2];
    if (lVar4 == *plStack_78) {
      FUN_140563300(plStack_78,&UNK_142aaaa00);
    }
    func_0x000142923c10(lVar4 * 0xb8 + plStack_78[1],&lStack_210,0xb8);
    plStack_78[2] = lVar4 + 1;
    uStack_39 = 0;
    FUN_1408935c0(alStack_158);
    return;
  }
  FUN_1428d9310(&UNK_142c81680);
                    /* WARNING: Does not return */
  pcVar3 = (code *)invalidInstructionException();
  (*pcVar3)();
}

