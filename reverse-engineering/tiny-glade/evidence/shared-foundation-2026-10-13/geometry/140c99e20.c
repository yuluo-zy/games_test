utils::geometry::rectangle::Rectangle2d::get_normal_at_u_non_normalized_corners

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 tg_math_140c99e20(undefined8 *param_1,float param_2)

{
  code *pcVar1;
  char cVar2;
  float fVar3;
  undefined8 uVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fStack_7c;
  float *pfStack_78;
  code *pcStack_70;
  undefined *puStack_68;
  undefined8 uStack_60;
  float **ppfStack_58;
  undefined8 uStack_50;
  undefined8 uStack_48;
  
  fVar3 = *(float *)((longlong)param_1 + 0x14);
  fVar5 = fVar3 + *(float *)(param_1 + 2);
  fVar7 = fVar5 + fVar5;
  fVar6 = fVar3 / fVar7;
  fStack_7c = fVar6;
  if ((0.0 <= fVar6) && (fVar6 <= _DAT_142925904)) {
    if (NAN(fVar6)) goto LAB_140c9a11e;
    fVar5 = fVar5 / fVar7;
    fStack_7c = fVar5;
    if ((0.0 <= fVar5) && (fVar5 <= _DAT_142925904)) {
      if (NAN(fVar5)) goto LAB_140c9a11e;
      fStack_7c = (fVar3 + fVar3 + *(float *)(param_1 + 2)) / fVar7;
      if ((0.0 <= fStack_7c) && (fStack_7c <= _DAT_142925904)) {
        if (NAN(fStack_7c)) goto LAB_140c9a11e;
        if ((param_2 != 0.0) || (NAN(param_2))) {
          if ((param_2 != _DAT_142925904) || (NAN(param_2) || NAN(_DAT_142925904))) {
            if ((fVar6 != param_2) || (NAN(fVar6) || NAN(param_2))) {
              if ((fVar5 != param_2) || (NAN(fVar5) || NAN(param_2))) {
                if ((fStack_7c != param_2) || (NAN(fStack_7c) || NAN(param_2))) {
                  if ((param_2 < 0.0) || (fVar6 <= param_2)) {
                    cVar2 = -(fVar6 < param_2);
                    if (param_2 < fVar6) {
                      cVar2 = (fVar6 < param_2) + '\x01';
                    }
                    if ((fVar5 <= param_2) || (0xfd < (byte)(cVar2 - 3U))) {
                      cVar2 = -(fVar5 < param_2);
                      if (param_2 < fVar5) {
                        cVar2 = (fVar5 < param_2) + '\x01';
                      }
                      fVar5 = (float)((ulonglong)*param_1 >> 0x20);
                      fVar3 = (float)*param_1;
                      if ((fStack_7c <= param_2) || (0xfd < (byte)(cVar2 - 3U))) {
                        fVar6 = fVar3 * _DAT_142b2e560;
                        fVar7 = fVar5 * _UNK_142b2e564;
                        fVar5 = (float)((uint)fVar5 ^ _UNK_1429258f8) * _UNK_142b2e568;
                        fVar3 = fVar3 * _UNK_142b2e56c;
                      }
                      else {
                        fVar6 = fVar3 * _DAT_14292e6b0;
                        fVar7 = fVar5 * _UNK_14292e6b4;
                        fVar5 = (float)((uint)fVar5 ^ _UNK_1429258f8) * _UNK_14292e6b8;
                        fVar3 = fVar3 * _UNK_14292e6bc;
                      }
                    }
                    else {
                      fVar5 = (float)((ulonglong)*param_1 >> 0x20);
                      fVar3 = (float)*param_1;
                      fVar6 = fVar3 * _DAT_142a0b160;
                      fVar7 = fVar5 * _UNK_142a0b164;
                      fVar5 = (float)((uint)fVar5 ^ _DAT_1429258f0) * _UNK_142a0b168;
                      fVar3 = fVar3 * _UNK_142a0b16c;
                    }
                  }
                  else {
                    fVar5 = (float)((ulonglong)*param_1 >> 0x20);
                    fVar3 = (float)*param_1;
                    fVar6 = fVar3 * _DAT_142b2e570;
                    fVar7 = fVar5 * _UNK_142b2e574;
                    fVar5 = (float)((uint)fVar5 ^ _DAT_1429258f0) * _UNK_142b2e578;
                    fVar3 = fVar3 * _UNK_142b2e57c;
                  }
                  return CONCAT44(fVar3 + fVar7,fVar5 + fVar6);
                }
                fVar6 = (float)((ulonglong)*param_1 >> 0x20);
                fVar5 = (float)*param_1;
                fVar3 = fVar5 * _DAT_1429cd180;
                fVar7 = fVar6 * _UNK_1429cd184;
                fVar6 = (float)((uint)fVar6 ^ _DAT_1429258f0) * _UNK_1429cd188;
                fVar5 = fVar5 * _UNK_1429cd18c;
              }
              else {
                fVar7 = (float)((ulonglong)*param_1 >> 0x20);
                fVar6 = (float)((uint)fVar7 ^ _DAT_1429258f0);
                fVar5 = (float)*param_1;
                fVar3 = fVar5;
              }
            }
            else {
              fVar5 = (float)((ulonglong)*param_1 >> 0x20);
              fVar3 = (float)*param_1;
              fVar7 = fVar5 * _UNK_142b2e584;
              fVar6 = (float)((uint)fVar5 ^ _DAT_1429258f0) * _UNK_142b2e588;
              fVar5 = fVar3 * _UNK_142b2e58c;
              fVar3 = fVar3 * _DAT_142b2e580;
            }
            fVar6 = fVar6 + fVar3;
            fVar5 = fVar5 + fVar7;
            goto LAB_140c99fab;
          }
        }
        fVar5 = (float)((ulonglong)*param_1 >> 0x20);
        fVar3 = (float)*param_1;
        fVar6 = fVar5 - fVar3;
        fVar5 = (float)((uint)fVar3 ^ _UNK_1429258f4) - fVar5;
LAB_140c99fab:
        fVar3 = _DAT_142925904 / SQRT(fVar5 * fVar5 + fVar6 * fVar6);
        return CONCAT44(_DAT_14292e6f4 * fVar3 * fVar5,_DAT_14292e6f4 * fVar3 * fVar6);
      }
    }
  }
  pfStack_78 = &fStack_7c;
  pcStack_70 = FUN_140d8c910;
  puStack_68 = &UNK_142e717b8;
  uStack_60 = 1;
  uStack_48 = 0;
  ppfStack_58 = &pfStack_78;
  uStack_50 = 1;
  FUN_1428d9390(&puStack_68,&UNK_142b2e808);
LAB_140c9a11e:
  FUN_1428d9430(&UNK_142e717c8,0x1d,&UNK_142b2e808);
  pcVar1 = (code *)swi(3);
  uVar4 = (*pcVar1)();
  return uVar4;
}

