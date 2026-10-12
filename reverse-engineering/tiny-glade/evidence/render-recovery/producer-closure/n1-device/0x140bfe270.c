
void FUN_140bfe270(undefined8 param_1,undefined8 param_2,byte *param_3,longlong *param_4)

{
  code *pcVar1;
  undefined1 auStack_e0 [48];
  undefined *puStack_b0;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined1 auStack_58 [16];
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  if ((*param_3 & 4) != 0) {
                    /* WARNING: Could not recover jumptable at 0x000140bfe2be. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*(code *)(&UNK_142b174fc + *(int *)(&UNK_142b174fc + *param_4 * 4)))();
    return;
  }
  FUN_1428d9430(&UNK_142b16ff8,0x43,&UNK_142b17060);
  puStack_b0 = &UNK_142b17098;
  uStack_a8 = 1;
  uStack_a0 = 8;
  uStack_98 = 0;
  uStack_90 = 0;
  FUN_1428d9596(0,&UNK_142b15760,auStack_58,&puStack_b0,&UNK_142b170a8);
  FUN_1428d9430(&UNK_142b0fae0,0x22,&UNK_142b0fcf0);
  puStack_b0 = (undefined *)0x0;
  FUN_1428d9596(0,auStack_e0,auStack_58,&puStack_b0,&UNK_142b170c0);
  FUN_1428d9430(&UNK_142b15fb0,0x28,&UNK_142b17078);
  FUN_1428d9760(&UNK_142b15188,0x2b,&puStack_b0,&UNK_142b151b8,&UNK_142b170d8);
  pcVar1 = (code *)swi(3);
  (*pcVar1)();
  return;
}

