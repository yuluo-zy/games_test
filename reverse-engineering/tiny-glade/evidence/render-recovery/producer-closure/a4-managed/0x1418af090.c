
void FUN_1418af090(longlong param_1,longlong *param_2,undefined8 param_3)

{
  code *pcVar1;
  longlong lVar2;
  longlong lVar3;
  longlong unaff_RDI;
  bool bVar4;
  undefined8 auStack_f8 [24];
  int aiStack_34 [3];
  
  aiStack_34[1] = -2;
  aiStack_34[2] = -1;
  if (*(short *)((longlong)param_2 + 0xc) == *(short *)(param_1 + 0xa50)) {
    if (*(char *)((longlong)param_2 + 0xe) != '\0') goto code_r0x0001418af16a;
    unaff_RDI = *param_2;
    if (*(int *)(unaff_RDI + 0xc) == (int)param_2[1]) {
      func_0x000142923c00(auStack_f8,unaff_RDI + 0x10,0xc0);
      FUN_140bdbe40(auStack_f8,param_3);
      FUN_140be0990(auStack_f8);
      aiStack_34[0] = *(int *)(unaff_RDI + 8);
      if (aiStack_34[0] == 0) {
        *(int *)(unaff_RDI + 0xc) = *(int *)(unaff_RDI + 0xc) + 1;
        lVar2 = *(longlong *)(param_1 + 0x178);
        do {
          *(longlong *)(unaff_RDI + 0x10) = lVar2;
          LOCK();
          lVar3 = *(longlong *)(param_1 + 0x178);
          bVar4 = lVar2 == lVar3;
          if (bVar4) {
            *(longlong *)(param_1 + 0x178) = unaff_RDI;
            lVar3 = lVar2;
          }
          UNLOCK();
          lVar2 = lVar3;
        } while (!bVar4);
        return;
      }
      goto code_r0x0001418af1c1;
    }
  }
  else {
    auStack_f8[0] = 0;
    param_2 = (longlong *)((longlong)param_2 + 0xc);
    FUN_1428ca816(0,param_2,param_1 + 0xa50,auStack_f8,&UNK_142d02310);
code_r0x0001418af16a:
    auStack_f8[0] = 0;
    param_2 = (longlong *)((longlong)param_2 + 0xe);
    FUN_1428ca902(0,param_2,&UNK_142d022b8,auStack_f8,&UNK_142d02328);
  }
  auStack_f8[0] = 0;
  FUN_1428ca7a0(0,unaff_RDI + 0xc,param_2 + 1,auStack_f8,&UNK_142d02340);
code_r0x0001418af1c1:
  auStack_f8[0] = 0;
  FUN_1428d7d73(0,aiStack_34,&UNK_142d02280,auStack_f8,&UNK_142d02288);
  pcVar1 = (code *)swi(3);
  (*pcVar1)();
  return;
}

