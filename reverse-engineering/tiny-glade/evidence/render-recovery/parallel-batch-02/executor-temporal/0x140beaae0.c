
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 FUN_140beaae0(longlong param_1,ulonglong param_2)

{
  code *pcVar1;
  longlong *plVar2;
  undefined8 uVar3;
  longlong lVar4;
  ulonglong uVar5;
  undefined1 auVar6 [16];
  
  uVar5 = param_2;
  plVar2 = (longlong *)FUN_140bd4cd0(param_1 + 0x20);
  if (plVar2 != (longlong *)0x0) {
    if (uVar5 < 0x10) {
      FUN_1428d9430(&UNK_142b14710,0x3e,&UNK_142b14750);
      pcVar1 = (code *)swi(3);
      uVar3 = (*pcVar1)();
      return uVar3;
    }
    lVar4 = plVar2[1] - *plVar2;
    auVar6._8_4_ = (int)((ulonglong)lVar4 >> 0x20);
    auVar6._0_8_ = lVar4;
    auVar6._12_4_ = _UNK_14292e934;
    if ((double)*(float *)(*(longlong *)(param_2 + 0x658) + 0x3a8) *
        ((auVar6._8_8_ - _UNK_14292e948) +
        ((double)CONCAT44(_UNK_14292e930,(int)lVar4) - _UNK_14292e940)) < _UNK_1429a25a8) {
      return 1;
    }
  }
  return 0;
}

