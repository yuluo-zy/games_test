
/* WARNING: Removing unreachable block (ram,0x0001412076f3) */
/* WARNING: Heritage AFTER dead removal. Example location: s0xfffffffffffffe78 : 0x0001412077ef */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
/* WARNING: Restarted to delay deadcode elimination for space: stack */

void FUN_1412075e0(longlong *param_1,float param_2,undefined8 param_3,undefined8 param_4)

{
  float fVar1;
  undefined8 *puVar2;
  code *pcVar3;
  float fVar4;
  float fVar5;
  undefined4 uVar6;
  undefined8 uVar7;
  float *pfVar8;
  longlong in_RDX;
  float *pfVar9;
  float *pfVar10;
  float fVar11;
  ulonglong uStack_1b8;
  undefined8 uStack_1b0;
  float fStack_1a8;
  float fStack_1a4;
  float fStack_1a0;
  undefined8 uStack_198;
  undefined8 uStack_190;
  undefined8 uStack_188;
  longlong *plStack_180;
  undefined8 uStack_178;
  undefined8 uStack_170;
  undefined4 uStack_168;
  undefined4 uStack_164;
  undefined4 uStack_160;
  undefined4 uStack_15c;
  ulonglong uStack_158;
  ulonglong uStack_150;
  undefined8 uStack_148;
  undefined8 uStack_140;
  undefined8 uStack_138;
  undefined8 *puStack_130;
  undefined1 uStack_128;
  undefined8 uStack_110;
  float fStack_108;
  float fStack_104;
  undefined8 uStack_100;
  float afStack_f8 [3];
  undefined4 uStack_ec;
  float fStack_e8;
  float fStack_e4;
  undefined8 uStack_e0;
  longlong **pplStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  ulonglong uStack_c0;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  
  uStack_d0 = param_3;
  uStack_c8 = param_4;
  if (param_2 < 0.0) {
    FUN_1428d9430(&UNK_142c1a090,0x21,&UNK_142c1a0e8);
    pcVar3 = (code *)swi(3);
    (*pcVar3)();
    return;
  }
  fVar1 = *(float *)((longlong)param_1 + 0x1c);
  uStack_158 = (ulonglong)*(uint *)(param_1 + 3);
  puVar2 = (undefined8 *)*param_1;
  uStack_178 = *puVar2;
  uStack_170 = puVar2[1];
  uStack_168 = *(undefined4 *)(puVar2 + 2);
  uStack_164 = *(undefined4 *)((longlong)puVar2 + 0x14);
  uStack_160 = *(undefined4 *)(puVar2 + 3);
  uStack_15c = *(undefined4 *)((longlong)puVar2 + 0x1c);
  uStack_148 = 0;
  uStack_140 = 0;
  uStack_138 = 0;
  plStack_180 = param_1 + 1;
  puStack_130 = &uStack_d0;
  uStack_128 = 0;
  uStack_150 = uStack_158;
  uStack_110 = FUN_141fd2ff0(&uStack_178);
  if (in_RDX != 0) {
    (*(code *)*plStack_180)(&fStack_108,uStack_110,in_RDX);
    uVar7 = uStack_110;
    uStack_e0 = uStack_100;
    uStack_110._4_4_ = (undefined4)((ulonglong)uStack_110 >> 0x20);
    afStack_f8[2] = (float)(undefined4)uStack_110;
    uStack_ec = uStack_110._4_4_;
    fStack_e8 = fStack_108;
    fStack_e4 = fStack_104;
    afStack_f8[0] = 1.4013e-45;
    afStack_f8[1] = 0.0;
    pfVar9 = afStack_f8;
    uStack_110 = uVar7;
    pplStack_d8 = &plStack_180;
    FUN_14123a910(&uStack_c0);
    fVar4 = _DAT_142925904;
    uStack_188 = uStack_90;
    uStack_190 = uStack_98;
    fStack_1a8 = (float)uStack_b0;
    fStack_1a4 = (float)((ulonglong)uStack_b0 >> 0x20);
    fStack_1a0 = (float)uStack_a8;
    uStack_1b8._0_4_ = (int)uStack_c0;
    if ((int)uStack_1b8 == 1) {
      uStack_190._0_4_ = (float)uStack_98;
      fVar11 = (float)uStack_190;
      uStack_190._4_4_ = (float)((ulonglong)uStack_98 >> 0x20);
      fVar5 = uStack_190._4_4_;
      uVar7 = uStack_188;
      uStack_198 = uStack_a0;
      uStack_1b8 = uStack_c0;
      uStack_1b0 = uStack_b8;
      uVar6 = (undefined4)uStack_b8;
      uStack_188._0_4_ = (float)uStack_90;
      fVar11 = SQRT(((float)uStack_188 - fStack_1a0) * ((float)uStack_188 - fStack_1a0) +
                    (fVar11 - fStack_1a8) * (fVar11 - fStack_1a8) +
                    (fVar5 - fStack_1a4) * (fVar5 - fStack_1a4));
      param_2 = (_DAT_142925904 - fVar1) * fVar11 - param_2;
      if (param_2 < 0.0) {
        uStack_188 = uVar7;
        do {
          *(int *)(param_1 + 3) = (int)uStack_1b0;
          *(undefined4 *)((longlong)param_1 + 0x1c) = 0x3f800000;
          uVar7 = FUN_141fd2ff0(&uStack_178);
          if (pfVar9 == (float *)0x0) {
            return;
          }
          (*(code *)*plStack_180)(&fStack_108,uVar7,pfVar9);
          uStack_110 = uVar7;
          uVar7 = uStack_110;
          if ((int)uStack_1b8 == 1) {
            fStack_1a0 = (float)uStack_188;
            uStack_1b0 = uStack_198;
            fStack_1a8 = (float)uStack_190;
            fStack_1a4 = (float)((ulonglong)uStack_190 >> 0x20);
            uStack_190 = CONCAT44(fStack_104,fStack_108);
            uStack_188 = uStack_100;
            pfVar8 = &fStack_104;
            pfVar10 = (float *)&uStack_100;
            pfVar9 = &fStack_108;
            uStack_198 = uVar7;
          }
          else {
            uStack_e0 = uStack_100;
            uStack_110._0_4_ = (undefined4)uVar7;
            uStack_110._4_4_ = (undefined4)((ulonglong)uVar7 >> 0x20);
            afStack_f8[2] = (float)(undefined4)uStack_110;
            uStack_ec = uStack_110._4_4_;
            fStack_e8 = fStack_108;
            fStack_e4 = fStack_104;
            afStack_f8[0] = 1.4013e-45;
            afStack_f8[1] = 0.0;
            pplStack_d8 = &plStack_180;
            uStack_110 = uVar7;
            FUN_14123a910(&uStack_c0,afStack_f8);
            uStack_188 = uStack_90;
            uStack_1b8 = uStack_c0;
            uStack_1b0 = uStack_b8;
            uStack_190 = uStack_98;
            fStack_1a8 = (float)uStack_b0;
            fStack_1a4 = (float)((ulonglong)uStack_b0 >> 0x20);
            fStack_1a0 = (float)uStack_a8;
            pfVar8 = (float *)((longlong)&uStack_190 + 4);
            pfVar10 = (float *)&uStack_188;
            pfVar9 = (float *)&uStack_190;
            uStack_198 = uStack_a0;
            if ((uStack_c0 & 1) == 0) {
              return;
            }
          }
          uVar6 = (undefined4)uStack_1b0;
          fVar11 = SQRT((*pfVar10 - fStack_1a0) * (*pfVar10 - fStack_1a0) +
                        (*pfVar9 - fStack_1a8) * (*pfVar9 - fStack_1a8) +
                        (*pfVar8 - fStack_1a4) * (*pfVar8 - fStack_1a4));
          param_2 = param_2 + fVar11;
        } while (param_2 < 0.0);
      }
      *(undefined4 *)(param_1 + 3) = uVar6;
      *(float *)((longlong)param_1 + 0x1c) = fVar4 - param_2 / fVar11;
    }
  }
  return;
}

