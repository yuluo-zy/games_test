
void FUN_140be6a90(void)

{
  code *pcVar1;
  undefined4 uStack_38;
  undefined4 uStack_34;
  undefined4 uStack_30;
  undefined4 uStack_2c;
  undefined8 uStack_28;
  undefined8 uStack_20;
  longlong lStack_18;
  undefined8 uStack_10;
  
  uStack_10 = 0xfffffffffffffffe;
  FUN_1418c1310(&uStack_38,0x10000);
  lStack_18 = CONCAT44(uRam0000000143634e44,uRam0000000143634e40);
  uStack_20 = CONCAT44(uRam0000000143634e4c,uRam0000000143634e48);
  uRam0000000143634e50 = uStack_28;
  uRam0000000143634e40 = uStack_38;
  uRam0000000143634e44 = uStack_34;
  uRam0000000143634e48 = uStack_30;
  uRam0000000143634e4c = uStack_2c;
  if (lStack_18 == 0) {
    return;
  }
  FUN_1428d9430(&UNK_142b12f50,0x20,&UNK_142b12f70);
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

