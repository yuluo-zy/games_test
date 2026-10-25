
void FUN_140beec30(longlong param_1,longlong param_2,undefined8 param_3,undefined8 param_4)

{
  char cVar1;
  code *pcVar2;
  char *pcVar3;
  undefined8 unaff_RDI;
  char *unaff_R12;
  longlong unaff_R14;
  longlong unaff_R15;
  bool bVar4;
  int iStack_a0;
  undefined4 uStack_9c;
  undefined4 uStack_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined4 uStack_68;
  undefined4 uStack_64;
  undefined8 uStack_60;
  char *pcStack_48;
  undefined8 uStack_40;
  
  uStack_40 = 0xfffffffffffffffe;
  if (*(longlong *)(param_1 + 0x870) == 0) {
    return;
  }
  if (*(short *)(param_2 + 0x14) == *(short *)(*(longlong *)(param_1 + 0x648) + 0xa50)) {
    if (*(char *)(param_2 + 0x16) != '\x01') goto code_r0x000140beede6;
    unaff_R15 = *(longlong *)(param_2 + 8);
    if (*(int *)(unaff_R15 + 0xc) == *(int *)(param_2 + 0x10)) {
      unaff_R14 = *(longlong *)(param_1 + 0x678);
      unaff_R12 = (char *)(unaff_R14 + 0x10);
      LOCK();
      bVar4 = *(char *)(unaff_R14 + 0x10) == '\0';
      if (bVar4) {
        *(char *)(unaff_R14 + 0x10) = '\x01';
      }
      UNLOCK();
      unaff_RDI = param_4;
      if (!bVar4) goto code_r0x000140beee10;
      goto code_r0x000140beecb7;
    }
    FUN_1428d9310(&UNK_142b15980);
  }
  else {
    uStack_78 = 0;
    param_2 = param_2 + 0x14;
    FUN_1428ca816(0,param_2,*(longlong *)(param_1 + 0x648) + 0xa50,&uStack_78,&UNK_142d022a0);
code_r0x000140beede6:
    uStack_78 = 0;
    FUN_1428ca902(0,param_2 + 0x16,&UNK_142d022d9,&uStack_78,&UNK_142d022c0);
code_r0x000140beee10:
    FUN_1428cd210(unaff_R12);
code_r0x000140beecb7:
    pcStack_48 = unaff_R12;
    FUN_140c81c70(unaff_R12);
    if (*(char *)(unaff_R15 + 0x7c) != '\x02') {
      FUN_140c14e00(&uStack_78,unaff_R14 + 0x18,unaff_R15 + 0x38,param_3,unaff_RDI);
      if (((uStack_78 < 6) && ((0x32UL >> (uStack_78 & 0x3f) & 1) != 0)) && (uStack_70 != 0)) {
        func_0x000140613c20(CONCAT44(uStack_64,uStack_68),uStack_70,1);
      }
      pcVar3 = pcStack_48;
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
      FUN_140d797d0(&iStack_a0,param_3,unaff_RDI);
      if (SBORROW8(0,CONCAT44(uStack_9c,iStack_a0))) {
        pcVar3 = (char *)CONCAT44(uStack_8c,uStack_90);
        uStack_60 = *(undefined8 *)(unaff_R15 + 0x30);
        uStack_78 = CONCAT44(uStack_78._4_4_,0x3b9cbe00);
        uStack_70 = 0;
        uStack_68 = 9;
        iStack_a0 = (**(code **)(param_1 + 0x870))(*(undefined8 *)(param_1 + 0x8b0),&uStack_78);
        if (iStack_a0 == 0) {
          *(undefined1 *)CONCAT44(uStack_94,uStack_98) = 0;
          if (pcVar3 != (char *)0x0) {
            func_0x000140613c20((undefined1 *)CONCAT44(uStack_94,uStack_98),pcVar3,1);
          }
          return;
        }
        pcStack_48 = pcVar3;
        FUN_1428d9760(&UNK_142b15850,0x1b,&iStack_a0,&UNK_142b15128,&UNK_142b159b0);
      }
      else {
        uStack_68 = uStack_90;
        uStack_64 = uStack_8c;
        uStack_60 = CONCAT44(uStack_84,uStack_88);
        uStack_78 = CONCAT44(uStack_9c,iStack_a0);
        uStack_70 = CONCAT44(uStack_94,uStack_98);
        FUN_1428d9760(&UNK_142b15188,0x2b,&uStack_78,&UNK_142b151d8,&UNK_142b15998);
      }
      goto code_r0x000140beeeb9;
    }
  }
  FUN_1428d9310(&UNK_142d02388);
code_r0x000140beeeb9:
                    /* WARNING: Does not return */
  pcVar2 = (code *)invalidInstructionException();
  (*pcVar2)();
}

