
undefined8 FUN_140c01e80(longlong param_1)

{
  int iVar1;
  ulonglong *puVar2;
  undefined8 *puVar3;
  undefined8 *puVar4;
  undefined8 *puVar5;
  byte abStack_114 [16];
  int iStack_104;
  undefined *puStack_100;
  undefined8 *puStack_f8;
  longlong lStack_f0;
  undefined8 uStack_e8;
  undefined8 uStack_e0;
  undefined4 uStack_d8;
  undefined4 uStack_d4;
  undefined8 uStack_d0;
  undefined *puStack_c8;
  undefined8 uStack_c0;
  undefined4 uStack_b8;
  undefined4 uStack_b4;
  undefined *puStack_b0;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined8 uStack_80;
  undefined *puStack_78;
  undefined8 *puStack_70;
  undefined1 uStack_61;
  undefined8 uStack_60;
  
  uStack_60 = 0xfffffffffffffffe;
  if (*(int *)(param_1 + 8) == 0x10de) {
    FUN_1418b1340(&puStack_100);
    puVar2 = puRam000000014362c3a8;
    if (SBORROW8(0,(longlong)puStack_100)) {
      FUN_1429096d0();
      return 1;
    }
    puStack_78 = puStack_100;
    puStack_70 = puStack_f8;
    puVar4 = puStack_f8 + lStack_f0;
    iVar1 = *(int *)(param_1 + 0xc);
    for (puVar5 = puStack_f8; puVar5 != puVar4; puVar5 = puVar5 + 1) {
      uStack_80 = *puVar5;
      FUN_1418b17b0(abStack_114,&uStack_80);
      if ((abStack_114[0] & 1) == 0) {
        if (iStack_104 == iVar1) {
          if (2 < *puVar2) {
            puVar4 = (undefined8 *)func_0x00014067dd80(&UNK_142b17bc0);
            uStack_e0 = *puVar4;
            uStack_d8 = *(undefined4 *)(puVar4 + 1);
            uStack_d4 = *(undefined4 *)((longlong)puVar4 + 0xc);
            uStack_b4 = *(undefined4 *)(puVar4 + 2);
            uStack_d0 = 3;
            puStack_c8 = &UNK_142b17bd8;
            uStack_c0 = 0x1e;
            puStack_b0 = &UNK_142b17bb0;
            uStack_a8 = 1;
            uStack_a0 = 8;
            puStack_100 = (undefined *)0x0;
            uStack_98 = 0;
            uStack_90 = 0;
            puStack_f8 = (undefined8 *)&UNK_142b17bd8;
            lStack_f0 = 0x1e;
            uStack_e8 = 0;
            uStack_b8 = 1;
            func_0x000140d4a330(&uStack_61,&puStack_100);
          }
          if (puStack_78 != (undefined *)0x0) {
            func_0x000140613c20(puStack_70,(longlong)puStack_78 << 3,8);
          }
          return 0;
        }
      }
      else if (1 < *puVar2) {
        puVar3 = (undefined8 *)func_0x00014067dd80(&UNK_142b17c20);
        uStack_e0 = *puVar3;
        uStack_d8 = *(undefined4 *)(puVar3 + 1);
        uStack_d4 = *(undefined4 *)((longlong)puVar3 + 0xc);
        uStack_b4 = *(undefined4 *)(puVar3 + 2);
        uStack_d0 = 2;
        puStack_c8 = &UNK_142b17bd8;
        uStack_c0 = 0x1e;
        puStack_b0 = &UNK_142b17c10;
        uStack_a8 = 1;
        uStack_a0 = 8;
        puStack_100 = (undefined *)0x0;
        uStack_98 = 0;
        uStack_90 = 0;
        puStack_f8 = (undefined8 *)&UNK_142b17bd8;
        lStack_f0 = 0x1e;
        uStack_e8 = 0;
        uStack_b8 = 1;
        func_0x000140d4a330(&uStack_61,&puStack_100);
      }
    }
    if (puStack_78 != (undefined *)0x0) {
      func_0x000140613c20(puStack_70,(longlong)puStack_78 << 3,8);
    }
  }
  puStack_100 = &UNK_142b17c50;
  puStack_f8 = (undefined8 *)0x1;
  lStack_f0 = 8;
  uStack_e8 = 0;
  uStack_e0 = 0;
  FUN_1428caaa0(&puStack_100);
  return 1;
}

