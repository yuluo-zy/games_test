
void FUN_140907990(longlong param_1,uint param_2,undefined8 param_3,undefined8 param_4)

{
  longlong lVar1;
  code *pcVar2;
  longlong lStack_218;
  undefined1 auStack_210 [176];
  longlong alStack_160 [2];
  undefined4 uStack_150;
  longlong *plStack_80;
  undefined8 uStack_68;
  undefined8 uStack_60;
  longlong lStack_58;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  longlong lStack_48;
  undefined1 uStack_39;
  undefined8 uStack_38;
  
  uStack_38 = 0xfffffffffffffffe;
  lVar1 = *(longlong *)(param_1 + 0xe0);
  uStack_39 = 1;
  lStack_48 = param_1;
  FUN_1412d5ad0(alStack_160,*(undefined8 *)(lVar1 + 0x1b8),*(undefined8 *)(lVar1 + 0xd0));
  uStack_39 = 1;
  func_0x00014003bb80(2);
  uStack_39 = 1;
  FUN_1409dca90(lVar1 + 0x90,uStack_150,2,1);
  uStack_50 = uStack_150;
  lStack_58 = (ulonglong)param_2 * 0x14;
  uStack_4c = *(undefined4 *)(&UNK_142aaaa78 + lStack_58);
  uStack_68 = param_4;
  uStack_60 = param_3;
  if (*(int *)(lStack_48 + 0x40) == 4) {
    if (*(longlong *)(lStack_48 + 0x48) != 0) {
      func_0x000140613c20(*(undefined8 *)(lStack_48 + 0x50),*(longlong *)(lStack_48 + 0x48) << 3,4);
    }
  }
  else if (*(int *)(lStack_48 + 0x40) == 1) {
    FUN_140be24b0(lStack_48 + 0x48);
  }
  lVar1 = lStack_48;
  *(undefined4 *)(lStack_48 + 0x40) = 7;
  *(undefined4 *)(lStack_48 + 0x44) = uStack_50;
  *(longlong *)(lStack_48 + 0x48) = lStack_58;
  *(undefined8 *)(lStack_48 + 0x50) = uStack_60;
  *(undefined8 *)(lStack_48 + 0x58) = uStack_68;
  *(undefined4 *)(lStack_48 + 0x60) = 1;
  *(undefined4 *)(lStack_48 + 100) = uStack_4c;
  func_0x000142923c00(alStack_160,lStack_48,0xf8);
  lStack_218 = alStack_160[0];
  alStack_160[0] = 2;
  if (lStack_218 != 2) {
    func_0x000142923c00(auStack_210,lVar1 + 8,0xb0);
    lVar1 = plStack_80[2];
    if (lVar1 == *plStack_80) {
      FUN_140563300(plStack_80,&UNK_142aaaa00);
    }
    func_0x000142923c10(lVar1 * 0xb8 + plStack_80[1],&lStack_218,0xb8);
    plStack_80[2] = lVar1 + 1;
    uStack_39 = 0;
    FUN_1408935c0(alStack_160);
    return;
  }
  FUN_1428d9310(&UNK_142c81680);
                    /* WARNING: Does not return */
  pcVar2 = (code *)invalidInstructionException();
  (*pcVar2)();
}

