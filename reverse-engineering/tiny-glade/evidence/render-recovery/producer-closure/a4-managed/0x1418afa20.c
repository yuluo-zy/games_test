
void FUN_1418afa20(longlong param_1,longlong *param_2,longlong param_3)

{
  undefined8 uVar1;
  code *pcVar2;
  longlong lVar3;
  longlong lVar4;
  longlong unaff_RDI;
  bool bVar5;
  int iStack_5c;
  undefined8 auStack_58 [6];
  
  if (*(short *)((longlong)param_2 + 0xc) == *(short *)(param_1 + 0xa50)) {
    if (*(char *)((longlong)param_2 + 0xe) != '\a') goto code_r0x0001418afafa;
    unaff_RDI = *param_2;
    if (*(int *)(unaff_RDI + 0xc) == (int)param_2[1]) {
      uVar1 = *(undefined8 *)(unaff_RDI + 0x10);
      lVar3 = *(longlong *)(param_3 + 0x88);
      if (lVar3 == *(longlong *)(param_3 + 0x78)) {
        FUN_140562870(param_3 + 0x78,&UNK_142d05398);
      }
      *(undefined8 *)(*(longlong *)(param_3 + 0x80) + lVar3 * 8) = uVar1;
      *(longlong *)(param_3 + 0x88) = lVar3 + 1;
      iStack_5c = *(int *)(unaff_RDI + 8);
      if (iStack_5c == 0) {
        *(int *)(unaff_RDI + 0xc) = *(int *)(unaff_RDI + 0xc) + 1;
        lVar3 = *(longlong *)(param_1 + 0xa48);
        do {
          *(longlong *)(unaff_RDI + 0x10) = lVar3;
          LOCK();
          lVar4 = *(longlong *)(param_1 + 0xa48);
          bVar5 = lVar3 == lVar4;
          if (bVar5) {
            *(longlong *)(param_1 + 0xa48) = unaff_RDI;
            lVar4 = lVar3;
          }
          UNLOCK();
          lVar3 = lVar4;
        } while (!bVar5);
        return;
      }
      goto code_r0x0001418afb55;
    }
  }
  else {
    auStack_58[0] = 0;
    param_2 = (longlong *)((longlong)param_2 + 0xc);
    FUN_1428ca816(0,param_2,param_1 + 0xa50,auStack_58,&UNK_142d02310);
code_r0x0001418afafa:
    auStack_58[0] = 0;
    param_2 = (longlong *)((longlong)param_2 + 0xe);
    FUN_1428ca902(0,param_2,&UNK_142d02358,auStack_58,&UNK_142d02328);
  }
  auStack_58[0] = 0;
  FUN_1428ca7a0(0,unaff_RDI + 0xc,param_2 + 1,auStack_58,&UNK_142d02340);
code_r0x0001418afb55:
  auStack_58[0] = 0;
  FUN_1428d7d73(0,&iStack_5c,&UNK_142d02280,auStack_58,&UNK_142d02288);
  pcVar2 = (code *)swi(3);
  (*pcVar2)();
  return;
}

