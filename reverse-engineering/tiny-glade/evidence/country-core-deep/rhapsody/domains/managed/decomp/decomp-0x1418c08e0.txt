
undefined8 * FUN_1418c08e0(undefined8 *param_1,undefined2 param_2)

{
  code *pcVar1;
  undefined1 *puVar2;
  undefined1 auStack_a70 [376];
  undefined1 auStack_8f8 [376];
  undefined1 auStack_780 [376];
  undefined1 auStack_608 [376];
  undefined1 auStack_490 [376];
  undefined1 auStack_318 [376];
  undefined1 auStack_1a0 [376];
  undefined8 uStack_28;
  
  uStack_28 = 0xfffffffffffffffe;
  func_0x000142923c20(auStack_1a0,0,0x178);
  func_0x000142923c20(auStack_318,0,0x178);
  func_0x000142923c20(auStack_490,0,0x178);
  func_0x000142923c20(auStack_608,0,0x178);
  func_0x000142923c20(auStack_780,0,0x178);
  func_0x000142923c20(auStack_8f8,0,0x178);
  func_0x000142923c20(auStack_a70,0,0x178);
  puVar2 = (undefined1 *)func_0x000140613c10(0x20,8);
  if (puVar2 != (undefined1 *)0x0) {
    *puVar2 = 0;
    *(undefined8 *)(puVar2 + 8) = 0;
    *(undefined8 *)(puVar2 + 0x10) = 8;
    *(undefined8 *)(puVar2 + 0x18) = 0;
    *(undefined2 *)(param_1 + 0x14a) = param_2;
    func_0x000142923c20(param_1 + 1,0,0xa48);
    *param_1 = puVar2;
    return param_1;
  }
  FUN_1428d9000(8,0x20);
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

