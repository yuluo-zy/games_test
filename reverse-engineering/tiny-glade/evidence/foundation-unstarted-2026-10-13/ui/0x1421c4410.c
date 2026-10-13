
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

longlong * FUN_1421c4410(longlong *param_1,longlong *param_2)

{
  code *pcVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined8 uVar5;
  undefined4 *puVar6;
  longlong *plVar7;
  longlong lVar8;
  longlong lVar9;
  longlong lVar10;
  longlong lVar11;
  undefined8 uVar12;
  undefined *puStack_d8;
  undefined8 uStack_d0;
  longlong ***ppplStack_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  longlong lStack_a8;
  longlong *plStack_a0;
  longlong lStack_98;
  longlong **pplStack_90;
  code *pcStack_88;
  longlong *plStack_80;
  longlong lStack_78;
  undefined4 *puStack_70;
  longlong *plStack_68;
  longlong lStack_60;
  longlong lStack_58;
  undefined4 *puStack_50;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  lVar8 = 0x25;
  plStack_80 = param_2;
  uVar5 = FUN_140b9d480(&UNK_142ffa7a8);
  lVar11 = -0x8000000000000000;
  lVar10 = lVar8;
  switch(*param_2) {
  case 0x10:
  case 0x11:
code_r0x0001421c4482:
    lVar9 = 0x1a;
    param_2 = (longlong *)FUN_140b9d480(&UNK_142ffa81e);
    if (lVar9 < 0) break;
joined_r0x0001421c4755:
    if (lVar9 != 0) goto code_r0x0001421c44a8;
code_r0x0001421c4624:
    puVar6 = (undefined4 *)0x1;
code_r0x0001421c4629:
    func_0x000142923c00(puVar6,param_2,lVar9);
    goto code_r0x0001421c463a;
  case 0x12:
    lVar9 = 0x2c;
    param_2 = (longlong *)FUN_140b9d480(&UNK_142ffa7cd);
    if (lVar9 < 0) break;
joined_r0x0001421c457a:
    if (lVar9 != 0) {
      uVar12 = 1;
      puVar6 = (undefined4 *)func_0x000140613c10(lVar9,1);
      if (puVar6 != (undefined4 *)0x0) goto code_r0x0001421c4581;
      goto code_r0x0001421c4763;
    }
    puVar6 = (undefined4 *)0x1;
code_r0x0001421c4581:
    plVar7 = param_2;
    lStack_58 = lVar9;
    puStack_50 = puVar6;
    func_0x000142923c00(puVar6,param_2,lVar9);
    lStack_60 = -0x8000000000000000;
    plStack_68 = plVar7;
    goto joined_r0x0001421c459d;
  default:
    if (1 < *param_2 - 0xbU) goto code_r0x0001421c4482;
    lVar9 = 0x25;
    param_2 = (longlong *)FUN_140b9d480(&UNK_142ffa7f9);
    if (-1 < lVar9) goto joined_r0x0001421c457a;
    break;
  case 0x14:
    lVar9 = 0x20;
    param_2 = (longlong *)FUN_140b9d480(&UNK_142ffa838);
    lVar10 = 0x23;
    uVar5 = FUN_140b9d480(&UNK_142ffa858);
    if (lVar9 < 0) break;
code_r0x0001421c4618:
    if (lVar9 == 0) goto code_r0x0001421c4624;
code_r0x0001421c44a8:
    uVar12 = 1;
    puVar6 = (undefined4 *)func_0x000140613c10(lVar9,1);
    if (puVar6 != (undefined4 *)0x0) goto code_r0x0001421c4629;
    goto code_r0x0001421c4763;
  case 0x15:
    lVar9 = 0x22;
    param_2 = (longlong *)FUN_140b9d480(&UNK_142ffa87b);
    lVar10 = 0x30;
    uVar5 = FUN_140b9d480(&UNK_142ffa89d);
    if (-1 < lVar9) goto code_r0x0001421c4618;
    break;
  case 0x16:
    lVar9 = 0x1f;
    param_2 = (longlong *)FUN_140b9d480(&UNK_142ffa8cd);
    if (-1 < lVar9) goto joined_r0x0001421c4755;
    break;
  case 0x17:
    lVar9 = 0x10;
    uVar5 = 1;
    puVar6 = (undefined4 *)func_0x000140613c10(0x10,1);
    uVar4 = _UNK_142ffa8f8;
    uVar3 = _UNK_142ffa8f4;
    uVar2 = _UNK_142ffa8f0;
    if (puVar6 == (undefined4 *)0x0) {
      FUN_1428d8fe3(1,0x10,&UNK_142ffa030);
      pcVar1 = (code *)swi(3);
      plVar7 = (longlong *)(*pcVar1)();
      return plVar7;
    }
    *puVar6 = _UNK_142ffa8ec;
    puVar6[1] = uVar2;
    puVar6[2] = uVar3;
    puVar6[3] = uVar4;
    lVar10 = 0;
code_r0x0001421c463a:
    pplStack_90 = &plStack_80;
    pcStack_88 = FUN_140deee70;
    puStack_d8 = &UNK_142ffa900;
    uStack_d0 = 1;
    ppplStack_c8 = &pplStack_90;
    uStack_c0 = 1;
    uStack_b8 = 0;
    lStack_78 = lVar9;
    puStack_70 = puVar6;
    lStack_58 = lVar9;
    puStack_50 = puVar6;
    FUN_140d79f90(&lStack_a8,&puStack_d8);
    lStack_60 = lStack_a8;
    plStack_68 = plStack_a0;
    lVar9 = lStack_98;
joined_r0x0001421c459d:
    if (lVar10 == 0) {
code_r0x0001421c46fb:
      *param_1 = lStack_58;
      param_1[1] = (longlong)puStack_50;
      param_1[2] = lStack_58;
      param_1[3] = lStack_60;
      param_1[4] = (longlong)plStack_68;
      param_1[5] = lVar9;
      param_1[6] = lVar11;
      param_1[7] = (longlong)param_2;
      param_1[8] = lVar10;
      return param_1;
    }
    if (-1 < lVar10) {
      uVar12 = 1;
      param_2 = (longlong *)func_0x000140613c10(lVar10,1);
      if (param_2 != (longlong *)0x0) {
        func_0x000142923c00(param_2,uVar5,lVar10);
        lVar11 = lVar10;
        goto code_r0x0001421c46fb;
      }
      goto code_r0x0001421c4778;
    }
    goto code_r0x0001421c4775;
  }
  uVar12 = 0;
  lVar10 = lVar8;
code_r0x0001421c4763:
  FUN_1428d8fe3(uVar12,lVar9,&UNK_142ffa030);
code_r0x0001421c4775:
  uVar12 = 0;
code_r0x0001421c4778:
  FUN_1428d8fe3(uVar12,lVar10,&UNK_142ffa030);
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

