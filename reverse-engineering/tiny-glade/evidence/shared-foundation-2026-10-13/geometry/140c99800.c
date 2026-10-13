utils::geometry::rectangle::Rectangle2d::get_pos_at_u

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulonglong FUN_140c99800(undefined8 *param_1,float param_2)

{
  code *pcVar1;
  char cVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  float fVar5;
  ulonglong uVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  float fStack_70;
  float fStack_6c;
  float *pfStack_68;
  code *pcStack_60;
  undefined *puStack_58;
  undefined8 uStack_50;
  float **ppfStack_48;
  undefined8 uStack_40;
  undefined8 uStack_38;
  
  fVar7 = (float)((ulonglong)param_1[2] >> 0x20);
  fVar5 = (float)param_1[2];
  fVar8 = fVar7 + fVar5;
  fVar10 = fVar8 + fVar8;
  fVar9 = fVar7 / fVar10;
  fStack_70 = fVar9;
  fStack_6c = param_2;
  if ((fVar9 < 0.0) || (_DAT_142925904 < fVar9)) {
LAB_140c99a93:
    pfStack_68 = &fStack_70;
    pcStack_60 = FUN_140d8c910;
    puStack_58 = &UNK_142e717b8;
    uStack_50 = 1;
    uStack_38 = 0;
    ppfStack_48 = &pfStack_68;
    uStack_40 = 1;
    FUN_1428d9390(&puStack_58,&UNK_142b2e808);
  }
  else if (!NAN(fVar9)) {
    fVar8 = fVar8 / fVar10;
    fStack_70 = fVar8;
    if ((fVar8 < 0.0) || (_DAT_142925904 < fVar8)) goto LAB_140c99a93;
    if (!NAN(fVar8)) {
      fStack_70 = (fVar7 + fVar7 + fVar5) / fVar10;
      if ((fStack_70 < 0.0) || (_DAT_142925904 < fStack_70)) goto LAB_140c99a93;
      if (!NAN(fStack_70)) {
        if ((param_2 != 0.0) || (NAN(param_2))) {
          if ((param_2 != _DAT_142925904) || (NAN(param_2) || NAN(_DAT_142925904))) {
            if ((fVar9 != param_2) || (NAN(fVar9) || NAN(param_2))) {
              if ((fVar8 != param_2) || (NAN(fVar8) || NAN(param_2))) {
                if ((fStack_70 != param_2) || (NAN(fStack_70) || NAN(param_2))) {
                  if ((param_2 < 0.0) || (fVar9 <= param_2)) {
                    cVar2 = -(fVar9 < param_2);
                    if (param_2 < fVar9) {
                      cVar2 = (fVar9 < param_2) + '\x01';
                    }
                    if ((fVar8 <= param_2) || (0xfd < (byte)(cVar2 - 3U))) {
                      cVar2 = -(fVar8 < param_2);
                      if (param_2 < fVar8) {
                        cVar2 = (fVar8 < param_2) + '\x01';
                      }
                      if ((fStack_70 <= param_2) || (0xfd < (byte)(cVar2 - 3U))) {
                        uVar3 = 3;
                        uVar4 = 4;
                      }
                      else {
                        uVar3 = 2;
                        uVar4 = 3;
                      }
                    }
                    else {
                      uVar3 = 1;
                      uVar4 = 2;
                    }
                  }
                  else {
                    uVar4 = 1;
                    uVar3 = 0;
                  }
                  uVar6 = FUN_140c99b20(0,&fStack_6c,uVar3,uVar4);
                  return uVar6;
                }
                if (0.0 < fVar7 * fVar5) {
                  fVar9 = (float)((uint)((ulonglong)*param_1 >> 0x20) ^ _DAT_1429258f0);
                  fVar8 = (float)*param_1;
                  fVar5 = fVar5 * _DAT_142b2e540;
                  fVar7 = fVar7 * _UNK_142b2e548;
                  goto LAB_140c99942;
                }
                goto LAB_140c99b03;
              }
              if (fVar7 * fVar5 <= 0.0) goto LAB_140c99b03;
              fVar9 = (float)((uint)((ulonglong)*param_1 >> 0x20) ^ _DAT_1429258f0);
              fVar8 = (float)*param_1;
              fVar5 = fVar5 * _DAT_14293ac60;
              fVar7 = fVar7 * _UNK_14293ac68;
              goto LAB_140c99942;
            }
            if (0.0 < fVar7 * fVar5) {
              fVar9 = (float)((uint)((ulonglong)*param_1 >> 0x20) ^ _DAT_1429258f0);
              fVar8 = (float)*param_1;
              fVar5 = fVar5 * _DAT_142aa5d60;
              fVar7 = fVar7 * _UNK_142aa5d68;
              goto LAB_140c99942;
            }
            goto LAB_140c99b03;
          }
        }
        if (0.0 < fVar7 * fVar5) {
          fVar9 = (float)((uint)((ulonglong)*param_1 >> 0x20) ^ _DAT_1429258f0);
          fVar8 = (float)*param_1;
          fVar5 = fVar5 * _DAT_142b2e550;
          fVar7 = fVar7 * _UNK_142b2e558;
LAB_140c99942:
          return (ulonglong)(uint)(*(float *)(param_1 + 1) + fVar7 * fVar9 + fVar5 * fVar8);
        }
        goto LAB_140c99b03;
      }
    }
  }
  FUN_1428d9430(&UNK_142e717c8,0x1d,&UNK_142b2e808);
LAB_140c99b03:
  FUN_1428d9430(&UNK_142b2e66c,0x23,&UNK_142b2e6b8);
  pcVar1 = (code *)swi(3);
  uVar6 = (*pcVar1)();
  return uVar6;
}

