
void FUN_140907cc0(longlong param_1,longlong param_2,undefined8 param_3,undefined8 param_4,
                  undefined8 param_5)

{
  undefined4 uVar1;
  longlong lVar2;
  code *pcVar3;
  longlong lStack_208;
  undefined1 auStack_200 [176];
  longlong alStack_150 [28];
  longlong *plStack_70;
  undefined8 uStack_58;
  undefined8 uStack_50;
  undefined4 uStack_44;
  longlong lStack_40;
  undefined1 uStack_31;
  undefined8 uStack_30;
  
  uStack_30 = 0xfffffffffffffffe;
  lVar2 = *(longlong *)(param_1 + 0xe0);
  uStack_31 = 1;
  lStack_40 = param_1;
  func_0x00014003bb80(2);
  uVar1 = *(undefined4 *)(param_2 + 0x10);
  uStack_31 = 1;
  FUN_1409dca90(lVar2 + 0x90,uVar1,2,1);
  uStack_58 = param_4;
  uStack_50 = param_3;
  uStack_44 = uVar1;
  if (*(int *)(lStack_40 + 0x40) == 4) {
    if (*(longlong *)(lStack_40 + 0x48) != 0) {
      func_0x000140613c20(*(undefined8 *)(lStack_40 + 0x50),*(longlong *)(lStack_40 + 0x48) << 3,4);
    }
  }
  else if (*(int *)(lStack_40 + 0x40) == 1) {
    FUN_140be24b0(lStack_40 + 0x48);
  }
  lVar2 = lStack_40;
  *(undefined4 *)(lStack_40 + 0x40) = 7;
  *(undefined4 *)(lStack_40 + 0x44) = uStack_44;
  *(undefined8 *)(lStack_40 + 0x48) = uStack_50;
  *(undefined8 *)(lStack_40 + 0x50) = param_5;
  *(undefined8 *)(lStack_40 + 0x58) = uStack_58;
  *(undefined4 *)(lStack_40 + 0x60) = 0;
  func_0x000142923c00(alStack_150,lStack_40,0xf8);
  lStack_208 = alStack_150[0];
  alStack_150[0] = 2;
  if (lStack_208 != 2) {
    func_0x000142923c00(auStack_200,lVar2 + 8,0xb0);
    lVar2 = plStack_70[2];
    if (lVar2 == *plStack_70) {
      FUN_140563300(plStack_70,&UNK_142aaaa00);
    }
    func_0x000142923c10(lVar2 * 0xb8 + plStack_70[1],&lStack_208,0xb8);
    plStack_70[2] = lVar2 + 1;
    uStack_31 = 0;
    FUN_1408935c0(alStack_150);
    return;
  }
  FUN_1428d9310(&UNK_142c81680);
                    /* WARNING: Does not return */
  pcVar3 = (code *)invalidInstructionException();
  (*pcVar3)();
}

