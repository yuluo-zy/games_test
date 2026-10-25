
void FUN_140bfe650(longlong param_1,longlong param_2,undefined8 param_3,undefined8 param_4)

{
  char cVar1;
  code *pcVar2;
  char *pcVar3;
  undefined8 unaff_RDI;
  longlong unaff_R12;
  char *unaff_R13;
  longlong unaff_R14;
  longlong unaff_R15;
  bool bVar4;
  int iStack_b0;
  undefined4 uStack_ac;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined4 uStack_98;
  undefined4 uStack_94;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined4 uStack_78;
  undefined4 uStack_74;
  undefined8 uStack_70;
  char *pcStack_50;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  if (*(longlong *)(param_1 + 0x870) == 0) {
    return;
  }
  if (*(short *)(param_2 + 0x14) == *(short *)(*(longlong *)(param_1 + 0x648) + 0xa50)) {
    if (*(char *)(param_2 + 0x16) != '\0') goto code_r0x000140bfe817;
    unaff_R12 = *(longlong *)(param_2 + 8);
    if (*(int *)(unaff_R12 + 0xc) == *(int *)(param_2 + 0x10)) {
      if (*(char *)(unaff_R12 + 0xcc) != '\x02') {
        unaff_R14 = unaff_R12 + 0x88;
        unaff_R15 = *(longlong *)(param_1 + 0x678);
        unaff_R13 = (char *)(unaff_R15 + 0x10);
        LOCK();
        bVar4 = *(char *)(unaff_R15 + 0x10) == '\0';
        if (bVar4) {
          *(char *)(unaff_R15 + 0x10) = '\x01';
        }
        UNLOCK();
        unaff_RDI = param_4;
        if (!bVar4) goto code_r0x000140bfe841;
        goto code_r0x000140bfe6f1;
      }
      goto code_r0x000140bfe760;
    }
    FUN_1428d9310(&UNK_142b17108);
  }
  else {
    uStack_88 = 0;
    param_2 = param_2 + 0x14;
    FUN_1428ca816(0,param_2,*(longlong *)(param_1 + 0x648) + 0xa50,&uStack_88,&UNK_142d022a0);
code_r0x000140bfe817:
    uStack_88 = 0;
    FUN_1428ca902(0,param_2 + 0x16,&UNK_142d022b8,&uStack_88,&UNK_142d022c0);
code_r0x000140bfe841:
    FUN_1428cd210(unaff_R13);
    param_4 = unaff_RDI;
code_r0x000140bfe6f1:
    pcStack_50 = unaff_R13;
    FUN_140c81c70(unaff_R13);
    FUN_140c14e00(&uStack_88,unaff_R15 + 0x18,unaff_R14,param_3,param_4);
    if (((uStack_88 < 6) && ((0x32UL >> (uStack_88 & 0x3f) & 1) != 0)) && (uStack_80 != 0)) {
      func_0x000140613c20(CONCAT44(uStack_74,uStack_78),uStack_80,1);
    }
    pcVar3 = pcStack_50;
    FUN_140c81e10();
    LOCK();
    cVar1 = *pcVar3;
    if (cVar1 == '\x01') {
      *pcVar3 = '\0';
    }
    UNLOCK();
    if (cVar1 != '\x01') {
      FUN_1428cdb90(pcVar3,0);
    }
code_r0x000140bfe760:
    FUN_140d797d0(&iStack_b0,param_3,param_4);
    if (SBORROW8(0,CONCAT44(uStack_ac,iStack_b0))) {
      pcVar3 = (char *)CONCAT44(uStack_9c,uStack_a0);
      uStack_70 = *(undefined8 *)(unaff_R12 + 0x40);
      uStack_88 = CONCAT44(uStack_88._4_4_,0x3b9cbe00);
      uStack_80 = 0;
      uStack_78 = 10;
      iStack_b0 = (**(code **)(param_1 + 0x870))(*(undefined8 *)(param_1 + 0x8b0),&uStack_88);
      if (iStack_b0 == 0) {
        *(undefined1 *)CONCAT44(uStack_a4,uStack_a8) = 0;
        if (pcVar3 != (char *)0x0) {
          func_0x000140613c20((undefined1 *)CONCAT44(uStack_a4,uStack_a8),pcVar3,1);
        }
        return;
      }
      pcStack_50 = pcVar3;
      FUN_1428d9760(&UNK_142b15850,0x1b,&iStack_b0,&UNK_142b15128,&UNK_142b17138);
      goto code_r0x000140bfe8dc;
    }
  }
  uStack_78 = uStack_a0;
  uStack_74 = uStack_9c;
  uStack_70 = CONCAT44(uStack_94,uStack_98);
  uStack_88 = CONCAT44(uStack_ac,iStack_b0);
  uStack_80 = CONCAT44(uStack_a4,uStack_a8);
  FUN_1428d9760(&UNK_142b15188,0x2b,&uStack_88,&UNK_142b151d8,&UNK_142b17120);
code_r0x000140bfe8dc:
                    /* WARNING: Does not return */
  pcVar2 = (code *)invalidInstructionException();
  (*pcVar2)();
}

