utils::geometry::circle::Circle2d::get_closest_u_from_pos

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140c976f0(undefined8 *param_1,float param_2,float param_3)

{
  code *pcVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  float fStack_8c;
  float *pfStack_88;
  code *pcStack_80;
  undefined *puStack_78;
  undefined8 uStack_70;
  undefined4 **ppuStack_68;
  undefined8 uStack_60;
  undefined8 uStack_58;
  
  fVar2 = _DAT_142925904;
  param_2 = param_2 - (float)*param_1;
  param_3 = param_3 - (float)((ulonglong)*param_1 >> 0x20);
  fVar5 = _DAT_142925904 / SQRT(param_3 * param_3 + param_2 * param_2);
  if (0x7ffffe < (int)fVar5 - 1U && (0x7effffff < (int)ABS(fVar5) - 0x800000U || (int)fVar5 < 0)) {
    return;
  }
  func_0x000142923cf0((int)fVar5 < 0,param_3 * fVar5);
  fVar3 = (float)func_0x000142923e20();
  fVar5 = _DAT_142934674;
  fVar4 = (float)func_0x000142923f10();
  fVar5 = fVar2 - (fVar3 - (float)(~-(uint)(fVar4 < 0.0) & (uint)fVar4 |
                                  (uint)(fVar4 + fVar5) & -(uint)(fVar4 < 0.0))) / fVar5;
  if (NAN(fVar5)) {
    FUN_1428d9430(&UNK_142e71d78,0x1d,&UNK_142e71dc0);
  }
  else {
    fVar5 = (float)func_0x000142923f10(fVar5,fVar2);
    fStack_8c = (float)(~-(uint)(fVar5 < 0.0) & (uint)fVar5 |
                       (uint)(fVar5 + fVar2) & -(uint)(fVar5 < 0.0));
    if ((0.0 <= fStack_8c) && (fStack_8c <= fVar2)) {
      if (!NAN(fStack_8c)) {
        return;
      }
      goto LAB_140c978c6;
    }
  }
  pfStack_88 = &fStack_8c;
  pcStack_80 = FUN_140d8c910;
  puStack_78 = &UNK_142e717b8;
  uStack_70 = 1;
  uStack_58 = 0;
  ppuStack_68 = &pfStack_88;
  uStack_60 = 1;
  FUN_1428d9390(&puStack_78,&UNK_142b2e4b8);
LAB_140c978c6:
  FUN_1428d9430(&UNK_142e717c8,0x1d,&UNK_142b2e4b8);
  pcVar1 = (code *)swi(3);
  (*pcVar1)();
  return;
}

