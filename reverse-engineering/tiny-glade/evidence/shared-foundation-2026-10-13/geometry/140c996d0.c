utils::geometry::rectangle::Rectangle2d::get_point_u

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140c996d0(longlong param_1,undefined8 param_2)

{
  code *pcVar1;
  float fVar2;
  float fVar3;
  float fStack_44;
  float *pfStack_40;
  code *pcStack_38;
  undefined *puStack_30;
  undefined8 uStack_28;
  float **ppfStack_20;
  undefined8 uStack_18;
  undefined8 uStack_10;
  
  fVar2 = 0.0;
  switch(param_2) {
  case 0:
    break;
  case 1:
    fVar2 = *(float *)(param_1 + 0x10) + *(float *)(param_1 + 0x14);
    fVar2 = *(float *)(param_1 + 0x14) / (fVar2 + fVar2);
    break;
  case 2:
    fVar2 = *(float *)(param_1 + 0x14) + *(float *)(param_1 + 0x10);
    fVar2 = fVar2 / (fVar2 + fVar2);
    break;
  case 3:
    fVar2 = *(float *)(param_1 + 0x14);
    fVar3 = fVar2 + *(float *)(param_1 + 0x10);
    fVar2 = (fVar2 + fVar2 + *(float *)(param_1 + 0x10)) / (fVar3 + fVar3);
    break;
  case 4:
    fVar2 = _DAT_142925904;
    break;
  default:
    FUN_1428d9430(&UNK_142b2e7c0,0x18,&UNK_142b2e7f0);
    goto code_r0x000140c99788;
  }
  fStack_44 = fVar2;
  if ((fVar2 < 0.0) || (_DAT_142925904 < fVar2)) {
code_r0x000140c99788:
    pfStack_40 = &fStack_44;
    pcStack_38 = FUN_140d8c910;
    puStack_30 = &UNK_142e717b8;
    uStack_28 = 1;
    uStack_10 = 0;
    ppfStack_20 = &pfStack_40;
    uStack_18 = 1;
    FUN_1428d9390(&puStack_30,&UNK_142b2e808);
  }
  else if (!NAN(fVar2)) {
    return;
  }
  FUN_1428d9430(&UNK_142e717c8,0x1d,&UNK_142b2e808);
  pcVar1 = (code *)swi(3);
  (*pcVar1)();
  return;
}

