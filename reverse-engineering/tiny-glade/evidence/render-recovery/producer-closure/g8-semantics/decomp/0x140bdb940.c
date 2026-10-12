
undefined8 FUN_140bdb940(undefined8 param_1,longlong param_2,longlong *param_3,undefined8 param_4)

{
  longlong **pplVar1;
  longlong *plVar2;
  code *pcVar3;
  ulonglong *puVar4;
  longlong lVar5;
  undefined4 *puVar6;
  longlong ***ppplVar7;
  longlong ***ppplStack_120;
  undefined *puStack_118;
  longlong *plStack_110;
  code *pcStack_108;
  longlong *plStack_100;
  code *pcStack_f8;
  undefined1 uStack_f0;
  undefined8 uStack_e8;
  undefined *puStack_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined4 uStack_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined8 uStack_b8;
  undefined *puStack_b0;
  undefined8 uStack_a8;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined *puStack_98;
  undefined8 uStack_90;
  longlong ***ppplStack_88;
  undefined8 uStack_80;
  longlong ***ppplStack_78;
  undefined8 uStack_70;
  longlong **pplStack_68;
  code *pcStack_60;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  puVar4 = puRam000000014362c3a8;
  uStack_48 = 0xfffffffffffffffe;
  ppplVar7 = (longlong ***)(*(longlong *)(param_2 + 8) + -8);
  lVar5 = *(longlong *)(param_2 + 0x10) << 3;
  do {
    if (lVar5 == 0) {
      FUN_1428d9310(&UNK_142b119b0);
      goto code_r0x000140bdbbd7;
    }
    pplVar1 = ppplVar7[1];
    ppplVar7 = ppplVar7 + 1;
    lVar5 = lVar5 + -8;
  } while (pplVar1[0xd2] != (longlong *)*param_3 || pplVar1[0xd3] != (longlong *)param_3[1]);
  if (2 < *puRam000000014362c3a8) {
    FUN_140c00400(&pplStack_68,pplVar1 + 2);
    plStack_110 = (longlong *)(*ppplVar7 + 9);
    puStack_118 = &UNK_1400dd3f0;
    pcStack_108 = FUN_140bffe80;
    pcStack_f8 = FUN_140c10290;
    ppplStack_120 = &pplStack_68;
    plStack_100 = param_3;
    puVar6 = (undefined4 *)func_0x00014067dd80(&UNK_142b11a38);
    uStack_c8 = *puVar6;
    uStack_c4 = puVar6[1];
    uStack_c0 = puVar6[2];
    uStack_bc = puVar6[3];
    uStack_9c = puVar6[4];
    uStack_b8 = 3;
    puStack_b0 = &UNK_142b110c0;
    uStack_a8 = 0x13;
    puStack_98 = &UNK_142b119f8;
    uStack_90 = 4;
    ppplStack_88 = (longlong ***)&ppplStack_120;
    uStack_80 = 3;
    ppplStack_78 = (longlong ***)0x0;
    uStack_e8 = 0;
    puStack_e0 = &UNK_142b110c0;
    uStack_d8 = 0x13;
    uStack_d0 = 0;
    uStack_a0 = 1;
    func_0x000140d4a330(&uStack_49,&uStack_e8);
    if ((longlong ***)pplStack_68 != (longlong ***)0x0) {
      func_0x000140613c20(pcStack_60,pplStack_68,1);
    }
  }
  if (3 < *puVar4) {
    pcStack_60 = FUN_140bdd270;
    ppplStack_120 = (longlong ***)0x2;
    plStack_110 = (longlong *)0x2;
    plStack_100 = (longlong *)0x0;
    pcStack_f8 = (code *)0x400000020;
    uStack_f0 = 3;
    pplStack_68 = (longlong **)ppplVar7;
    puVar6 = (undefined4 *)func_0x00014067dd80(&UNK_142b11a80);
    uStack_c8 = *puVar6;
    uStack_c4 = puVar6[1];
    uStack_c0 = puVar6[2];
    uStack_bc = puVar6[3];
    uStack_9c = puVar6[4];
    uStack_b8 = 4;
    puStack_b0 = &UNK_142b110c0;
    uStack_a8 = 0x13;
    puStack_98 = &UNK_142b11a70;
    uStack_90 = 1;
    ppplStack_88 = &pplStack_68;
    uStack_80 = 1;
    ppplStack_78 = (longlong ***)&ppplStack_120;
    uStack_70 = 1;
    uStack_e8 = 0;
    puStack_e0 = &UNK_142b110c0;
    uStack_d8 = 0x13;
    uStack_d0 = 0;
    uStack_a0 = 1;
    func_0x000140d4a330(&uStack_49,&uStack_e8);
  }
  plVar2 = *(longlong **)(param_2 + 0x18);
  LOCK();
  lVar5 = *plVar2;
  *plVar2 = *plVar2 + 1;
  UNLOCK();
  if (*plVar2 != 0 && SCARRY8(lVar5,1) == *plVar2 < 0) {
    FUN_140bf2560(param_1,ppplVar7,plVar2,param_4);
    return param_1;
  }
code_r0x000140bdbbd7:
                    /* WARNING: Does not return */
  pcVar3 = (code *)invalidInstructionException();
  (*pcVar3)();
}

