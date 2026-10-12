
void FUN_1409dcfd0(longlong *param_1,undefined8 param_2)

{
  longlong *plVar1;
  longlong lVar2;
  longlong lVar3;
  longlong lVar4;
  longlong lVar5;
  longlong lVar6;
  longlong lVar7;
  code *pcVar8;
  longlong lVar9;
  longlong lVar10;
  longlong lVar11;
  longlong lVar12;
  longlong lVar13;
  longlong lVar14;
  
  lVar2 = *param_1;
  if (SBORROW8(0,lVar2)) {
    FUN_1428d9310(&UNK_142ac3a78);
  }
  else {
    lVar13 = func_0x000140613c10(0xc0,8);
    if (lVar13 == 0) {
      FUN_1428d9000(8,0xc0);
    }
    else {
      func_0x000142923c00(lVar13,param_2,0xc0);
      lVar3 = param_1[5];
      param_1[5] = lVar13;
      param_1[6] = (longlong)&UNK_142ac3ae8;
      if (lVar3 == 0) {
        lVar13 = param_1[8];
        *param_1 = -0x8000000000000000;
        lVar3 = param_1[7];
        lVar9 = param_1[1];
        lVar10 = param_1[2];
        lVar11 = param_1[3];
        lVar12 = param_1[4];
        lVar4 = param_1[5];
        lVar5 = param_1[6];
        lVar6 = *(longlong *)(lVar13 + 0xa0);
        if (lVar6 == *(longlong *)(lVar13 + 0x90)) {
          FUN_1405627b0(lVar13 + 0x90,&UNK_142abc8c0);
        }
        lVar7 = *(longlong *)(lVar13 + 0x98);
        lVar14 = lVar6 * 0x40;
        plVar1 = (longlong *)(lVar7 + 0x30 + lVar14);
        *plVar1 = lVar5;
        plVar1[1] = lVar3;
        plVar1 = (longlong *)(lVar7 + 0x20 + lVar14);
        *plVar1 = lVar12;
        plVar1[1] = lVar4;
        plVar1 = (longlong *)(lVar7 + 0x10 + lVar14);
        *plVar1 = lVar10;
        plVar1[1] = lVar11;
        *(longlong *)(lVar7 + lVar14) = lVar2;
        ((longlong *)(lVar7 + lVar14))[1] = lVar9;
        *(longlong *)(lVar13 + 0xa0) = lVar6 + 1;
        return;
      }
      FUN_1428d9430(&UNK_142ac3ab0,0x20,&UNK_142ac3ad0);
    }
  }
                    /* WARNING: Does not return */
  pcVar8 = (code *)invalidInstructionException();
  (*pcVar8)();
}

