
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 * FUN_1408e2750(undefined4 *param_1,float *param_2,longlong param_3)

{
  code *pcVar1;
  undefined4 *puVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  float fVar13;
  float fVar14;
  float fVar15;
  undefined8 uStack_120;
  undefined1 auStack_118 [8];
  float fStack_110;
  float fStack_10c;
  float fStack_108;
  float fStack_104;
  float fStack_100;
  float fStack_fc;
  undefined *puStack_f8;
  undefined8 uStack_f0;
  undefined8 uStack_e8;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  
  fVar3 = *param_2;
  fVar13 = param_2[1];
  fVar10 = fVar13;
  if (fVar13 <= fVar3) {
    fVar10 = fVar3;
  }
  if ((float)(~-(uint)NAN(fVar3) & (uint)fVar10 | -(uint)NAN(fVar3) & (uint)fVar13) <=
      _DAT_142925984) {
    if ((*(byte *)(param_3 + 8) & 1) == 0) {
      fVar3 = (*(float *)(param_3 + 0x14) + *(float *)(param_3 + 0x14)) *
              (float)*(undefined8 *)(param_3 + 0x24) * _DAT_1429258e0;
    }
    else {
      uStack_e8 = *(undefined8 *)(param_3 + 0x1c);
      puStack_f8 = *(undefined **)(param_3 + 0xc);
      uStack_f0 = *(undefined8 *)(param_3 + 0x14);
      func_0x000140c99540(&fStack_108,&puStack_f8);
      fVar3 = (float)((ulonglong)*(undefined8 *)(param_3 + 0x24) >> 0x20) *
              (float)((ulonglong)*(undefined8 *)(param_3 + 0x1c) >> 0x20) * _UNK_14293ac68 *
              fStack_100 +
              (float)*(undefined8 *)(param_3 + 0x24) * (float)*(undefined8 *)(param_3 + 0x1c) *
              _DAT_14293ac60 * fStack_108;
    }
    FUN_1408e2eb0(param_1 + 1,param_3,fVar3);
    *param_1 = 0;
    return param_1;
  }
  if ((*(byte *)(param_3 + 8) & 1) == 0) goto LAB_1408e2e73;
  uStack_e8 = *(undefined8 *)(param_3 + 0x1c);
  puStack_f8 = *(undefined **)(param_3 + 0xc);
  uStack_f0 = *(undefined8 *)(param_3 + 0x14);
  fVar12 = (float)*(undefined8 *)(param_3 + 0x1c);
  fVar8 = (float)((ulonglong)*(undefined8 *)(param_3 + 0x1c) >> 0x20);
  fVar9 = (float)*(undefined8 *)(param_3 + 0x24) * fVar12 * 0.0;
  fVar10 = (float)((ulonglong)*(undefined8 *)(param_3 + 0x24) >> 0x20) * fVar8 * 0.0;
  if (*(char *)(param_3 + 0x3c) != '\x01') {
    func_0x000140c99540(&fStack_108,&puStack_f8);
    fVar3 = fVar3 * _DAT_142925870;
    fVar13 = fVar9 - fVar3;
    fVar10 = fVar10 + 0.0;
    uStack_120 = CONCAT44(fStack_fc * fVar10 + fStack_104 * fVar13,
                          fStack_100 * fVar10 + fStack_108 * fVar13);
    uStack_c8 = *(undefined8 *)(param_3 + 0x14);
    uStack_c0 = *(undefined8 *)(param_3 + 0x1c);
    fStack_10c = *(float *)(param_3 + 0x18);
    fVar11 = *(float *)(param_3 + 0x4c);
    func_0x000140c9cdc0(auStack_118,&uStack_120);
    fVar7 = fStack_110;
    fVar14 = auStack_118._0_4_;
    fVar15 = auStack_118._4_4_;
    fVar13 = *(float *)(param_3 + 0x30);
    fVar4 = 0.0;
    if (_DAT_142925984 < fVar13) {
      fVar4 = fVar8;
      if (fVar8 <= fVar12) {
        fVar4 = fVar12;
      }
      fVar5 = (((float)(~-(uint)NAN(fVar12) & (uint)fVar4 | -(uint)NAN(fVar12) & (uint)fVar8) +
               _DAT_142aa5084) / _DAT_142aa5088) / _DAT_142925970;
      fVar4 = 0.0;
      if (0.0 <= fVar5) {
        fVar4 = fVar5;
      }
      fVar5 = _DAT_142925904;
      if (fVar4 <= _DAT_142925904) {
        fVar5 = fVar4;
      }
      fVar4 = (_DAT_1429cd078 - (fVar5 + fVar5)) * fVar5 * fVar5;
      fVar4 = fVar4 * _DAT_1429cd2d4 + (_DAT_142925904 - fVar4) * _DAT_142a7ba84;
      if (fVar4 < 0.0) goto LAB_1408e2e52;
      fVar6 = _DAT_1429cd2d4 * fVar13 + (_DAT_142925904 - fVar13) * _DAT_142925970;
      fVar5 = 0.0;
      if (0.0 <= fVar6) {
        fVar5 = fVar6;
      }
      if (fVar5 <= fVar4) {
        fVar4 = fVar5;
      }
    }
    func_0x000140c99540(&fStack_108,&puStack_f8);
    fVar3 = fVar3 + fVar9;
    uStack_120 = CONCAT44(fVar10 * fStack_fc + fVar3 * fStack_104,
                          fVar10 * fStack_100 + fVar3 * fStack_108);
    func_0x000140c9cdc0(auStack_118,&uStack_120);
    fVar10 = auStack_118._0_4_;
    fVar9 = auStack_118._4_4_;
    fVar3 = 0.0;
    if (fVar13 <= _DAT_142925984) goto LAB_1408e2d8a;
    fVar3 = fVar8;
    if (fVar8 <= fVar12) {
      fVar3 = fVar12;
    }
    fVar12 = (((float)(~-(uint)NAN(fVar12) & (uint)fVar3 | -(uint)NAN(fVar12) & (uint)fVar8) +
              _DAT_142aa5084) / _DAT_142aa5088) / _DAT_142925970;
    fVar3 = 0.0;
    if (0.0 <= fVar12) {
      fVar3 = fVar12;
    }
    fVar12 = _DAT_142925904;
    if (fVar3 <= _DAT_142925904) {
      fVar12 = fVar3;
    }
    fVar3 = (_DAT_1429cd078 - (fVar12 + fVar12)) * fVar12 * fVar12;
    fVar3 = fVar3 * _DAT_1429cd2d4 + (_DAT_142925904 - fVar3) * _DAT_142a7ba84;
    if (fVar3 < 0.0) goto LAB_1408e2e64;
    fVar12 = fVar13 * _DAT_1429cd2d4 + (_DAT_142925904 - fVar13) * _DAT_142925970;
    fVar13 = 0.0;
    if (0.0 <= fVar12) {
      fVar13 = fVar12;
    }
LAB_1408e2d86:
    if (fVar13 <= fVar3) {
      fVar3 = fVar13;
    }
LAB_1408e2d8a:
    *(ulonglong *)(param_1 + 1) =
         CONCAT44(fVar4 + fVar15 + fVar11,fVar4 * 0.0 + fVar14 + (float)uStack_c8);
    param_1[3] = fVar7 + fStack_10c + fVar4 * 0.0;
    *(ulonglong *)(param_1 + 4) =
         CONCAT44(fVar3 + fVar11 + fVar9,fVar3 * 0.0 + (float)uStack_c8 + fVar10);
    param_1[6] = fStack_110 + fStack_10c + fVar3 * 0.0;
    *param_1 = 1;
    return param_1;
  }
  func_0x000140c99540(&fStack_108,&puStack_f8);
  fVar13 = fVar13 * _DAT_142925870;
  fVar9 = fVar9 + 0.0;
  fVar3 = fVar10 - fVar13;
  uStack_120 = CONCAT44(fVar3 * fStack_fc + fVar9 * fStack_104,
                        fVar3 * fStack_100 + fVar9 * fStack_108);
  uStack_c8 = *(undefined8 *)(param_3 + 0x14);
  uStack_c0 = *(undefined8 *)(param_3 + 0x1c);
  fStack_10c = *(float *)(param_3 + 0x18);
  fVar11 = *(float *)(param_3 + 0x4c);
  func_0x000140c9cdc0(auStack_118,&uStack_120);
  fVar7 = fStack_110;
  fVar14 = auStack_118._0_4_;
  fVar15 = auStack_118._4_4_;
  fVar5 = *(float *)(param_3 + 0x30);
  fVar4 = 0.0;
  if (_DAT_142925984 < fVar5) {
    fVar3 = fVar8;
    if (fVar8 <= fVar12) {
      fVar3 = fVar12;
    }
    fVar4 = (((float)(~-(uint)NAN(fVar12) & (uint)fVar3 | -(uint)NAN(fVar12) & (uint)fVar8) +
             _DAT_142aa5084) / _DAT_142aa5088) / _DAT_142925970;
    fVar3 = 0.0;
    if (0.0 <= fVar4) {
      fVar3 = fVar4;
    }
    fVar4 = _DAT_142925904;
    if (fVar3 <= _DAT_142925904) {
      fVar4 = fVar3;
    }
    fVar3 = (_DAT_1429cd078 - (fVar4 + fVar4)) * fVar4 * fVar4;
    fVar4 = fVar3 * _DAT_1429cd2d4 + (_DAT_142925904 - fVar3) * _DAT_142a7ba84;
    if (0.0 <= fVar4) {
      fVar6 = _DAT_1429cd2d4 * fVar5 + (_DAT_142925904 - fVar5) * _DAT_142925970;
      fVar3 = 0.0;
      if (0.0 <= fVar6) {
        fVar3 = fVar6;
      }
      if (fVar3 <= fVar4) {
        fVar4 = fVar3;
      }
      goto LAB_1408e2985;
    }
LAB_1408e2e52:
    FUN_140d8b480();
  }
  else {
LAB_1408e2985:
    func_0x000140c99540(&fStack_108,&puStack_f8);
    fVar13 = fVar13 + fVar10;
    uStack_120 = CONCAT44(fVar13 * fStack_fc + fVar9 * fStack_104,
                          fVar13 * fStack_100 + fVar9 * fStack_108);
    func_0x000140c9cdc0(auStack_118,&uStack_120);
    fVar10 = auStack_118._0_4_;
    fVar9 = auStack_118._4_4_;
    fVar3 = 0.0;
    if (fVar5 <= _DAT_142925984) goto LAB_1408e2d8a;
    fVar3 = fVar8;
    if (fVar8 <= fVar12) {
      fVar3 = fVar12;
    }
    fVar13 = (((float)(~-(uint)NAN(fVar12) & (uint)fVar3 | -(uint)NAN(fVar12) & (uint)fVar8) +
              _DAT_142aa5084) / _DAT_142aa5088) / _DAT_142925970;
    fVar3 = 0.0;
    if (0.0 <= fVar13) {
      fVar3 = fVar13;
    }
    fVar13 = _DAT_142925904;
    if (fVar3 <= _DAT_142925904) {
      fVar13 = fVar3;
    }
    fVar3 = (_DAT_1429cd078 - (fVar13 + fVar13)) * fVar13 * fVar13;
    fVar3 = fVar3 * _DAT_1429cd2d4 + (_DAT_142925904 - fVar3) * _DAT_142a7ba84;
    if (0.0 <= fVar3) {
      fVar12 = fVar5 * _DAT_1429cd2d4 + (_DAT_142925904 - fVar5) * _DAT_142925970;
      fVar13 = 0.0;
      if (0.0 <= fVar12) {
        fVar13 = fVar12;
      }
      goto LAB_1408e2d86;
    }
  }
LAB_1408e2e64:
  FUN_140d8b480();
LAB_1408e2e73:
  puStack_f8 = &UNK_142aa57d0;
  uStack_f0 = 1;
  uStack_e8 = 8;
  uStack_e0 = 0;
  uStack_d8 = 0;
  FUN_1428d9390(&puStack_f8,&UNK_142aa5818);
  pcVar1 = (code *)swi(3);
  puVar2 = (undefined4 *)(*pcVar1)();
  return puVar2;
}

