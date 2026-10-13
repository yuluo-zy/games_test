
undefined8 *
FUN_141466a40(undefined8 *param_1,longlong param_2,longlong param_3,undefined4 param_4,
             undefined8 param_5)

{
  longlong lVar1;
  code *pcVar2;
  undefined8 *puVar3;
  undefined4 uStack_94;
  undefined *puStack_90;
  longlong *plStack_88;
  longlong **pplStack_80;
  longlong lStack_78;
  undefined4 *puStack_70;
  undefined8 uStack_68;
  undefined8 uStack_60;
  longlong *plStack_58;
  longlong **pplStack_50;
  longlong *plStack_48;
  code *pcStack_40;
  longlong lStack_30;
  longlong lStack_28;
  
  lVar1 = *(longlong *)(param_2 + 0x10);
  uStack_94 = param_4;
  if (lVar1 == *(longlong *)(param_3 + 0x10)) {
    puStack_90 = *(undefined **)(param_2 + 8);
    plStack_88 = (longlong *)(puStack_90 + lVar1 * 8);
    pplStack_80 = (longlong **)0x0;
    puStack_70 = &uStack_94;
    lStack_78 = param_3;
    FUN_1412fb280(&plStack_58,&puStack_90,&UNK_142c63308);
    FUN_141469430(&puStack_90,&plStack_58,0);
    if (!SBORROW8(0,(longlong)puStack_90)) {
      param_1[6] = uStack_60;
      param_1[4] = puStack_70;
      param_1[5] = uStack_68;
      param_1[2] = pplStack_80;
      param_1[3] = lStack_78;
      *param_1 = puStack_90;
      param_1[1] = plStack_88;
      return param_1;
    }
  }
  else {
    plStack_58 = &lStack_30;
    pplStack_50 = (longlong **)FUN_140d8d6c0;
    plStack_48 = &lStack_28;
    pcStack_40 = FUN_140d8d6c0;
    puStack_90 = &UNK_142c634a0;
    plStack_88 = (longlong *)0x2;
    puStack_70 = (undefined4 *)0x0;
    pplStack_80 = &plStack_58;
    lStack_78 = 2;
    lStack_30 = lVar1;
    lStack_28 = *(longlong *)(param_3 + 0x10);
    FUN_1428d9390(&puStack_90,param_5);
  }
  plStack_58 = plStack_88;
  pplStack_50 = pplStack_80;
  FUN_1428d9760(&UNK_142c63340,0x2b,&plStack_58,&UNK_142c63320,param_5);
  pcVar2 = (code *)swi(3);
  puVar3 = (undefined8 *)(*pcVar2)();
  return puVar3;
}

