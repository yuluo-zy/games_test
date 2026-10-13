utils::geometry::rectangle::Rectangle2d::get_edge_id_at_u

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulonglong FUN_140c9acc0(longlong param_1,float param_2)

{
  float fVar1;
  code *pcVar2;
  ulonglong uVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  float fStack_6c;
  float *pfStack_68;
  code *pcStack_60;
  undefined *puStack_58;
  undefined8 uStack_50;
  undefined4 **ppuStack_48;
  undefined8 uStack_40;
  undefined8 uStack_38;
  
  fVar1 = *(float *)(param_1 + 0x14);
  fVar4 = fVar1 + *(float *)(param_1 + 0x10);
  fVar6 = fVar4 + fVar4;
  fVar5 = fVar1 / fVar6;
  fStack_6c = fVar5;
  if ((0.0 <= fVar5) && (fVar5 <= _DAT_142925904)) {
    if (NAN(fVar5)) goto LAB_140c9ae07;
    fVar4 = fVar4 / fVar6;
    fStack_6c = fVar4;
    if ((0.0 <= fVar4) && (fVar4 <= _DAT_142925904)) {
      if (NAN(fVar4)) goto LAB_140c9ae07;
      fStack_6c = (fVar1 + fVar1 + *(float *)(param_1 + 0x10)) / fVar6;
      if ((0.0 <= fStack_6c) && (fStack_6c <= _DAT_142925904)) {
        if (!NAN(fStack_6c)) {
          if ((param_2 < 0.0) || (fVar5 < param_2)) {
            if ((param_2 < fVar5) || (uVar3 = 1, fVar4 < param_2)) {
              uVar3 = (ulonglong)(-(uint)(param_2 <= fStack_6c && fVar4 <= param_2) & 1) ^ 3;
            }
          }
          else {
            uVar3 = 0;
          }
          return uVar3;
        }
        goto LAB_140c9ae07;
      }
    }
  }
  pfStack_68 = &fStack_6c;
  pcStack_60 = FUN_140d8c910;
  puStack_58 = &UNK_142e717b8;
  uStack_50 = 1;
  uStack_38 = 0;
  ppuStack_48 = &pfStack_68;
  uStack_40 = 1;
  FUN_1428d9390(&puStack_58,&UNK_142b2e808);
LAB_140c9ae07:
  FUN_1428d9430(&UNK_142e717c8,0x1d,&UNK_142b2e808);
  pcVar2 = (code *)swi(3);
  uVar3 = (*pcVar2)();
  return uVar3;
}

