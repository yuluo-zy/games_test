
void FUN_1418af800(longlong param_1,longlong *param_2,undefined8 param_3)

{
  longlong *plVar1;
  code *pcVar2;
  longlong lVar3;
  longlong lVar4;
  longlong unaff_RDI;
  bool bVar5;
  undefined8 **ppuStack_e8;
  undefined8 **ppuStack_e0;
  undefined8 uStack_d8;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  undefined4 uStack_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined4 uStack_b8;
  undefined4 uStack_b4;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined8 uStack_80;
  longlong lStack_78;
  undefined8 uStack_70;
  longlong *plStack_60;
  longlong lStack_58;
  longlong *plStack_50;
  undefined8 *puStack_48;
  longlong *plStack_40;
  undefined8 **ppuStack_38;
  undefined8 **ppuStack_30;
  undefined8 uStack_28;
  
  uStack_28 = 0xfffffffffffffffe;
  if (*(short *)((longlong)param_2 + 0xc) == *(short *)(param_1 + 0xa50)) {
    if (*(char *)((longlong)param_2 + 0xe) != '\x01') goto code_r0x0001418af968;
    unaff_RDI = *param_2;
    if (*(int *)(unaff_RDI + 0xc) == (int)param_2[1]) {
      lStack_78 = *(longlong *)(unaff_RDI + 0x70);
      uStack_70 = *(undefined8 *)(unaff_RDI + 0x78);
      uStack_88 = *(undefined8 *)(unaff_RDI + 0x60);
      uStack_80 = *(undefined8 *)(unaff_RDI + 0x68);
      uStack_98 = *(undefined8 *)(unaff_RDI + 0x50);
      uStack_90 = *(undefined8 *)(unaff_RDI + 0x58);
      uStack_d8 = *(undefined8 *)(unaff_RDI + 0x10);
      uStack_d0 = *(undefined4 *)(unaff_RDI + 0x18);
      uStack_cc = *(undefined4 *)(unaff_RDI + 0x1c);
      uStack_c8 = *(undefined4 *)(unaff_RDI + 0x20);
      uStack_c4 = *(undefined4 *)(unaff_RDI + 0x24);
      uStack_c0 = *(undefined4 *)(unaff_RDI + 0x28);
      uStack_bc = *(undefined4 *)(unaff_RDI + 0x2c);
      uStack_b8 = *(undefined4 *)(unaff_RDI + 0x30);
      uStack_b4 = *(undefined4 *)(unaff_RDI + 0x34);
      uStack_b0 = *(undefined4 *)(unaff_RDI + 0x38);
      uStack_ac = *(undefined4 *)(unaff_RDI + 0x3c);
      uStack_a8 = *(undefined4 *)(unaff_RDI + 0x40);
      uStack_a4 = *(undefined4 *)(unaff_RDI + 0x44);
      uStack_a0 = *(undefined4 *)(unaff_RDI + 0x48);
      uStack_9c = *(undefined4 *)(unaff_RDI + 0x4c);
      FUN_1418afb90(&uStack_d8,param_3);
      lVar3 = lStack_78;
      if (uStack_70._4_1_ != '\x02') {
        plStack_60 = &lStack_78;
        puStack_48 = &uStack_70;
        lStack_58 = lStack_78;
        ppuStack_e8 = &puStack_48;
        ppuStack_e0 = &plStack_50;
        plStack_40 = &lStack_58;
        ppuStack_38 = &plStack_60;
        ppuStack_30 = &ppuStack_e8;
        plStack_50 = plStack_60;
        FUN_1418bc7d0(&plStack_40);
        if (lVar3 != 0) {
          plStack_40 = (longlong *)(lVar3 + -0x10);
          LOCK();
          plVar1 = (longlong *)(lVar3 + -0x10);
          *plVar1 = *plVar1 + -1;
          UNLOCK();
          if (*plVar1 == 0) {
            FUN_140c15f40(&plStack_40);
          }
        }
      }
      plStack_40 = (longlong *)CONCAT44(plStack_40._4_4_,*(int *)(unaff_RDI + 8));
      if (*(int *)(unaff_RDI + 8) == 0) {
        *(int *)(unaff_RDI + 0xc) = *(int *)(unaff_RDI + 0xc) + 1;
        lVar3 = *(longlong *)(param_1 + 0x2f0);
        do {
          *(longlong *)(unaff_RDI + 0x10) = lVar3;
          LOCK();
          lVar4 = *(longlong *)(param_1 + 0x2f0);
          bVar5 = lVar3 == lVar4;
          if (bVar5) {
            *(longlong *)(param_1 + 0x2f0) = unaff_RDI;
            lVar4 = lVar3;
          }
          UNLOCK();
          lVar3 = lVar4;
        } while (!bVar5);
        return;
      }
      goto code_r0x0001418af9bf;
    }
  }
  else {
    uStack_d8 = 0;
    param_2 = (longlong *)((longlong)param_2 + 0xc);
    FUN_1428ca816(0,param_2,param_1 + 0xa50,&uStack_d8,&UNK_142d02310);
code_r0x0001418af968:
    uStack_d8 = 0;
    param_2 = (longlong *)((longlong)param_2 + 0xe);
    FUN_1428ca902(0,param_2,&UNK_142d022d9,&uStack_d8,&UNK_142d02328);
  }
  uStack_d8 = 0;
  FUN_1428ca7a0(0,unaff_RDI + 0xc,param_2 + 1,&uStack_d8,&UNK_142d02340);
code_r0x0001418af9bf:
  uStack_d8 = 0;
  FUN_1428d7d73(0,&plStack_40,&UNK_142d02280,&uStack_d8,&UNK_142d02288);
  pcVar2 = (code *)swi(3);
  (*pcVar2)();
  return;
}

