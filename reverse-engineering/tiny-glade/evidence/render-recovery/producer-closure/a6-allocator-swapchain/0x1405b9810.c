
char FUN_1405b9810(longlong param_1,undefined8 *param_2)

{
  char *pcVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  code *pcVar5;
  char cVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  longlong lVar9;
  bool bVar10;
  undefined1 auStack_3c8 [392];
  undefined1 auStack_240 [296];
  undefined8 uStack_118;
  undefined1 auStack_b8 [16];
  longlong *aplStack_a8 [6];
  int iStack_78;
  undefined8 *puStack_50;
  char *pcStack_48;
  undefined8 uStack_40;
  
  uStack_40 = 0xfffffffffffffffe;
  uVar2 = *param_2;
  uVar8 = param_2[1];
  uVar3 = param_2[2];
  uVar4 = param_2[3];
  lVar9 = *(longlong *)(param_1 + 0x678);
  pcVar1 = (char *)(lVar9 + 0x10);
  LOCK();
  bVar10 = *(char *)(lVar9 + 0x10) == '\0';
  if (bVar10) {
    *(char *)(lVar9 + 0x10) = '\x01';
  }
  UNLOCK();
  if (!bVar10) {
    FUN_1428cd210(pcVar1,param_2,1000000000);
  }
  pcStack_48 = pcVar1;
  FUN_140c81c70(pcVar1);
  lVar9 = lVar9 + 0x18;
  uVar7 = func_0x00014067dd80(lVar9);
  FUN_140c15d20(uVar2,uVar4,uVar7,uVar8);
  FUN_140053bb0(auStack_240,&UNK_142a04a98,0x17);
  uStack_118 = uVar3;
  func_0x000142923c00(auStack_3c8,auStack_240,0x188);
  puStack_50 = (undefined8 *)func_0x000140613c10(0x10,8);
  if (puStack_50 != (undefined8 *)0x0) {
    *puStack_50 = uVar2;
    puStack_50[1] = lVar9;
    FUN_140053ee0(auStack_b8,auStack_3c8,uVar4,puStack_50,&UNK_142a02258);
    if (iStack_78 != 2) {
      LOCK();
      *aplStack_a8[0] = *aplStack_a8[0] + -1;
      UNLOCK();
      if (*aplStack_a8[0] == 0) {
        FUN_140c41170(aplStack_a8);
      }
    }
    uVar8 = func_0x00014067dd80(lVar9);
    FUN_140c15ba0(uVar2,uVar4,uVar8);
    pcVar1 = pcStack_48;
    FUN_140c81e10();
    LOCK();
    cVar6 = *pcVar1;
    bVar10 = cVar6 == '\x01';
    if (bVar10) {
      *pcVar1 = '\0';
      cVar6 = '\x01';
    }
    UNLOCK();
    if (!bVar10) {
      cVar6 = FUN_1428cdb90(pcVar1,0);
    }
    return cVar6;
  }
  FUN_1428d9000(8,0x10);
                    /* WARNING: Does not return */
  pcVar5 = (code *)invalidInstructionException();
  (*pcVar5)();
}

