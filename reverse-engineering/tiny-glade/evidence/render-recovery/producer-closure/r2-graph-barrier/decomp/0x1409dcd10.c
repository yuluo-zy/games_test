
void FUN_1409dcd10(longlong *param_1,undefined8 param_2)

{
  longlong *plVar1;
  undefined8 *puVar2;
  longlong lVar3;
  longlong lVar4;
  code *pcVar5;
  longlong lVar6;
  longlong lVar7;
  longlong lStack_358;
  undefined4 uStack_350;
  undefined4 uStack_34c;
  undefined4 uStack_348;
  undefined4 uStack_344;
  undefined4 uStack_340;
  undefined4 uStack_33c;
  undefined4 uStack_338;
  undefined4 uStack_334;
  longlong lStack_330;
  longlong lStack_328;
  longlong lStack_320;
  longlong lStack_58;
  longlong lStack_50;
  longlong *plStack_48;
  longlong lStack_40;
  undefined8 uStack_38;
  undefined8 uStack_30;
  
  uStack_30 = 0xfffffffffffffffe;
  lStack_358 = *param_1;
  plStack_48 = param_1;
  lStack_40 = lStack_358;
  uStack_38 = param_2;
  if (SBORROW8(0,lStack_358)) {
    FUN_1428d9310(&UNK_142ac3a78);
  }
  else {
    func_0x000142923c00(&lStack_358,param_2,0x300);
    lVar6 = func_0x000140613c10(0x300,8);
    if (lVar6 == 0) {
      FUN_1428d9000(8,0x300);
    }
    else {
      func_0x000142923c00(lVar6,&lStack_358,0x300);
      lStack_58 = param_1[5];
      lStack_50 = param_1[6];
      param_1[5] = lVar6;
      param_1[6] = (longlong)&UNK_142ac3a90;
      if (lStack_58 == 0) {
        lVar6 = param_1[8];
        *param_1 = -0x8000000000000000;
        lStack_320 = param_1[7];
        uStack_350 = (undefined4)param_1[1];
        uStack_34c = *(undefined4 *)((longlong)param_1 + 0xc);
        uStack_348 = (undefined4)param_1[2];
        uStack_344 = *(undefined4 *)((longlong)param_1 + 0x14);
        uStack_340 = (undefined4)param_1[3];
        uStack_33c = *(undefined4 *)((longlong)param_1 + 0x1c);
        uStack_338 = (undefined4)param_1[4];
        uStack_334 = *(undefined4 *)((longlong)param_1 + 0x24);
        lStack_330 = param_1[5];
        lStack_328 = param_1[6];
        lVar3 = *(longlong *)(lVar6 + 0xa0);
        if (lVar3 == *(longlong *)(lVar6 + 0x90)) {
          FUN_1405627b0(lVar6 + 0x90,&UNK_142abc8c0);
        }
        lVar4 = *(longlong *)(lVar6 + 0x98);
        lVar7 = lVar3 * 0x40;
        plVar1 = (longlong *)(lVar4 + 0x30 + lVar7);
        *plVar1 = lStack_328;
        plVar1[1] = lStack_320;
        puVar2 = (undefined8 *)(lVar4 + 0x20 + lVar7);
        *puVar2 = CONCAT44(uStack_334,uStack_338);
        puVar2[1] = lStack_330;
        puVar2 = (undefined8 *)(lVar4 + 0x10 + lVar7);
        *puVar2 = CONCAT44(uStack_344,uStack_348);
        puVar2[1] = CONCAT44(uStack_33c,uStack_340);
        *(longlong *)(lVar4 + lVar7) = lStack_358;
        ((longlong *)(lVar4 + lVar7))[1] = CONCAT44(uStack_34c,uStack_350);
        *(longlong *)(lVar6 + 0xa0) = lVar3 + 1;
        return;
      }
      FUN_1428d9430(&UNK_142ac3ab0,0x20,&UNK_142ac3ad0);
    }
  }
                    /* WARNING: Does not return */
  pcVar5 = (code *)invalidInstructionException();
  (*pcVar5)();
}

