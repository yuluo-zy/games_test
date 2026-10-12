
void FUN_140533d90(longlong *param_1)

{
  longlong *plVar1;
  longlong lVar2;
  longlong lVar3;
  longlong lVar4;
  longlong lVar5;
  longlong lVar6;
  longlong lVar7;
  longlong lVar8;
  code *pcVar9;
  longlong lVar10;
  longlong lVar11;
  longlong lVar12;
  longlong lVar13;
  longlong lVar14;
  
  lVar2 = *param_1;
  if (SBORROW8(0,lVar2)) {
    FUN_1428d9310(&UNK_1429f4ec8,-lVar2);
  }
  else {
    lVar3 = param_1[5];
    param_1[5] = 1;
    param_1[6] = (longlong)&UNK_1429f4ee0;
    if (lVar3 == 0) {
      lVar3 = param_1[8];
      *param_1 = -0x8000000000000000;
      lVar4 = param_1[7];
      lVar10 = param_1[1];
      lVar11 = param_1[2];
      lVar12 = param_1[3];
      lVar13 = param_1[4];
      lVar5 = param_1[5];
      lVar6 = param_1[6];
      lVar7 = *(longlong *)(lVar3 + 0xa0);
      if (lVar7 == *(longlong *)(lVar3 + 0x90)) {
        FUN_1405627b0(lVar3 + 0x90,&UNK_14295b538);
      }
      lVar8 = *(longlong *)(lVar3 + 0x98);
      lVar14 = lVar7 * 0x40;
      plVar1 = (longlong *)(lVar8 + 0x30 + lVar14);
      *plVar1 = lVar6;
      plVar1[1] = lVar4;
      plVar1 = (longlong *)(lVar8 + 0x20 + lVar14);
      *plVar1 = lVar13;
      plVar1[1] = lVar5;
      plVar1 = (longlong *)(lVar8 + 0x10 + lVar14);
      *plVar1 = lVar11;
      plVar1[1] = lVar12;
      *(longlong *)(lVar8 + lVar14) = lVar2;
      ((longlong *)(lVar8 + lVar14))[1] = lVar10;
      *(longlong *)(lVar3 + 0xa0) = lVar7 + 1;
      return;
    }
    FUN_1428d9430(&UNK_1429f4f00,0x20,&UNK_1429f4f20);
  }
                    /* WARNING: Does not return */
  pcVar9 = (code *)invalidInstructionException();
  (*pcVar9)();
}

