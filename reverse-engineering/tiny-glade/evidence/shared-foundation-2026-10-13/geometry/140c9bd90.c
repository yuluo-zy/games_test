utils::geometry::rectangle::Rectangle2d::circular_slice

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 * FUN_140c9bd90(undefined8 *param_1,longlong param_2,float param_3,float param_4)

{
  code *pcVar1;
  float fVar2;
  float *pfVar3;
  undefined8 *puVar4;
  float **ppfVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  undefined *puStack_f0;
  undefined8 uStack_e8;
  undefined8 **ppuStack_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  float *pfStack_c0;
  code *pcStack_b8;
  undefined4 **ppuStack_b0;
  float *pfStack_a8;
  undefined4 **ppuStack_a0;
  float afStack_94 [31];
  
  afStack_94[1] = -NAN;
  afStack_94[2] = -NAN;
  pfVar3 = (float *)func_0x000140613c10(4,4);
  fVar2 = _DAT_142925904;
  if (pfVar3 == (float *)0x0) {
    FUN_1428d9000(4,4);
    pcVar1 = (code *)swi(3);
    puVar4 = (undefined8 *)(*pcVar1)();
    return puVar4;
  }
  pfStack_c0 = (float *)CONCAT44(pfStack_c0._4_4_,param_3);
  if ((param_3 < 0.0) || (_DAT_142925904 < param_3)) {
    ppuStack_b0 = &pfStack_c0;
    pfStack_a8 = (float *)FUN_140d8c910;
    puStack_f0 = &UNK_142e717b8;
    uStack_e8 = 1;
    uStack_d0 = 0;
    ppuStack_e0 = &ppuStack_b0;
    uStack_d8 = 1;
    FUN_1428d9390(&puStack_f0,&UNK_142e71898);
  }
  else if (NAN(param_3)) {
    FUN_1428d9430(&UNK_142e717c8,0x1d,&UNK_142e71898);
  }
  else {
    *pfVar3 = param_3;
    ppuStack_b0 = (float **)0x1;
    ppuStack_a0 = (float **)0x1;
    fVar8 = *(float *)(param_2 + 0x14);
    fVar7 = fVar8 + *(float *)(param_2 + 0x10);
    fVar6 = fVar7 + fVar7;
    fVar9 = fVar8 / fVar6;
    pfStack_a8 = pfVar3;
    afStack_94[0] = fVar9;
    if ((fVar9 < 0.0) || (fVar2 < fVar9)) {
      pfStack_c0 = afStack_94;
      pcStack_b8 = FUN_140d8c910;
      puStack_f0 = &UNK_142e717b8;
      uStack_e8 = 1;
      uStack_d0 = 0;
      ppuStack_e0 = (undefined8 **)&pfStack_c0;
      uStack_d8 = 1;
      FUN_1428d9390(&puStack_f0,&UNK_142b2e808);
    }
    else if (NAN(fVar9)) {
      FUN_1428d9430(&UNK_142e717c8,0x1d,&UNK_142b2e808);
    }
    else {
      fVar7 = fVar7 / fVar6;
      afStack_94[0] = fVar7;
      if ((fVar7 < 0.0) || (fVar2 < fVar7)) {
        pfStack_c0 = afStack_94;
        pcStack_b8 = FUN_140d8c910;
        puStack_f0 = &UNK_142e717b8;
        uStack_e8 = 1;
        uStack_d0 = 0;
        ppuStack_e0 = (undefined8 **)&pfStack_c0;
        uStack_d8 = 1;
        FUN_1428d9390(&puStack_f0,&UNK_142b2e808);
      }
      else if (NAN(fVar7)) {
        FUN_1428d9430(&UNK_142e717c8,0x1d,&UNK_142b2e808);
      }
      else {
        fVar6 = (fVar8 + fVar8 + *(float *)(param_2 + 0x10)) / fVar6;
        afStack_94[0] = fVar6;
        if ((fVar6 < 0.0) || (fVar2 < fVar6)) {
          pfStack_c0 = afStack_94;
          pcStack_b8 = FUN_140d8c910;
          puStack_f0 = &UNK_142e717b8;
          uStack_e8 = 1;
          uStack_d0 = 0;
          ppuStack_e0 = (undefined8 **)&pfStack_c0;
          uStack_d8 = 1;
          FUN_1428d9390(&puStack_f0,&UNK_142b2e808);
        }
        else if (NAN(fVar6)) {
          FUN_1428d9430(&UNK_142e717c8,0x1d,&UNK_142b2e808);
        }
        else {
          fVar8 = (float)(~-(uint)(param_4 < param_3) & (uint)param_4 |
                         (uint)(_DAT_142925904 + param_4) & -(uint)(param_4 < param_3));
          ppfVar5 = (float **)0x1;
          if ((param_3 < 0.0) && (0.0 < fVar8)) {
            FUN_140b5f870(&ppuStack_b0,&UNK_142b2e910);
            pfStack_a8[1] = 0.0;
            ppuStack_a0 = (float **)0x2;
            ppfVar5 = (float **)0x2;
          }
          if ((param_3 < fVar9) && (fVar9 < fVar8)) {
            if (ppfVar5 == (float **)ppuStack_b0) {
              FUN_140b5f870(&ppuStack_b0,&UNK_142b2e928);
            }
            pfStack_a8[(longlong)ppfVar5] = fVar9;
            ppfVar5 = (float **)((longlong)ppfVar5 + 1);
            ppuStack_a0 = ppfVar5;
          }
          if ((param_3 < fVar7) && (fVar7 < fVar8)) {
            if (ppfVar5 == (float **)ppuStack_b0) {
              FUN_140b5f870(&ppuStack_b0,&UNK_142b2e940);
            }
            pfStack_a8[(longlong)ppfVar5] = fVar7;
            ppfVar5 = (float **)((longlong)ppfVar5 + 1);
            ppuStack_a0 = ppfVar5;
          }
          if ((param_3 < fVar6) && (fVar6 < fVar8)) {
            if (ppfVar5 == (float **)ppuStack_b0) {
              FUN_140b5f870(&ppuStack_b0,&UNK_142b2e958);
            }
            pfStack_a8[(longlong)ppfVar5] = fVar6;
            ppfVar5 = (float **)((longlong)ppfVar5 + 1);
            ppuStack_a0 = ppfVar5;
          }
          if ((param_3 < fVar2) && (_DAT_142925904 < fVar8)) {
            if (ppfVar5 == (float **)ppuStack_b0) {
              FUN_140b5f870(&ppuStack_b0,&UNK_142b2e970);
            }
            pfStack_a8[(longlong)ppfVar5] = 0.0;
            ppfVar5 = (float **)((longlong)ppfVar5 + 1);
            ppuStack_a0 = ppfVar5;
          }
          fVar9 = fVar9 + fVar2;
          if ((param_3 < fVar9) && (fVar9 < fVar8)) {
            fVar9 = fVar9 + _DAT_14295a0e4;
            afStack_94[0] = fVar9;
            if ((fVar9 < 0.0) || (fVar2 < fVar9)) {
              pfStack_c0 = afStack_94;
              pcStack_b8 = FUN_140d8c910;
              puStack_f0 = &UNK_142e717b8;
              uStack_e8 = 1;
              uStack_d0 = 0;
              ppuStack_e0 = (undefined8 **)&pfStack_c0;
              uStack_d8 = 1;
              FUN_1428d9390(&puStack_f0,&UNK_142b2e988);
              goto LAB_140c9c4dc;
            }
            if (ppfVar5 == (float **)ppuStack_b0) {
              FUN_140b5f870(&ppuStack_b0,&UNK_142b2e9a0);
            }
            pfStack_a8[(longlong)ppfVar5] = fVar9;
            ppfVar5 = (float **)((longlong)ppfVar5 + 1);
            ppuStack_a0 = ppfVar5;
          }
          fVar7 = fVar7 + fVar2;
          if ((param_3 < fVar7) && (fVar7 < fVar8)) {
            fVar7 = fVar7 + _DAT_14295a0e4;
            afStack_94[0] = fVar7;
            if ((fVar7 < 0.0) || (fVar2 < fVar7)) {
              pfStack_c0 = afStack_94;
              pcStack_b8 = FUN_140d8c910;
              puStack_f0 = &UNK_142e717b8;
              uStack_e8 = 1;
              uStack_d0 = 0;
              ppuStack_e0 = (undefined8 **)&pfStack_c0;
              uStack_d8 = 1;
              FUN_1428d9390(&puStack_f0,&UNK_142b2e9b8);
              goto LAB_140c9c4dc;
            }
            if (ppfVar5 == (float **)ppuStack_b0) {
              FUN_140b5f870(&ppuStack_b0,&UNK_142b2e9d0);
            }
            pfStack_a8[(longlong)ppfVar5] = fVar7;
            ppfVar5 = (float **)((longlong)ppfVar5 + 1);
            ppuStack_a0 = ppfVar5;
          }
          fVar6 = fVar6 + fVar2;
          if ((param_3 < fVar6) && (fVar6 < fVar8)) {
            fVar6 = fVar6 + _DAT_14295a0e4;
            afStack_94[0] = fVar6;
            if ((fVar6 < 0.0) || (fVar2 < fVar6)) {
              pfStack_c0 = afStack_94;
              pcStack_b8 = FUN_140d8c910;
              puStack_f0 = &UNK_142e717b8;
              uStack_e8 = 1;
              uStack_d0 = 0;
              ppuStack_e0 = (undefined8 **)&pfStack_c0;
              uStack_d8 = 1;
              FUN_1428d9390(&puStack_f0,&UNK_142b2e9e8);
              goto LAB_140c9c4dc;
            }
            if (ppfVar5 == (float **)ppuStack_b0) {
              FUN_140b5f870(&ppuStack_b0,&UNK_142b2ea00);
            }
            pfStack_a8[(longlong)ppfVar5] = fVar6;
            ppfVar5 = (float **)((longlong)ppfVar5 + 1);
            ppuStack_a0 = ppfVar5;
          }
          afStack_94[0] = param_4;
          if ((param_4 < 0.0) || (fVar2 < param_4)) {
            pfStack_c0 = afStack_94;
            pcStack_b8 = FUN_140d8c910;
            puStack_f0 = &UNK_142e717b8;
            uStack_e8 = 1;
            uStack_d0 = 0;
            ppuStack_e0 = (undefined8 **)&pfStack_c0;
            uStack_d8 = 1;
            FUN_1428d9390(&puStack_f0,&UNK_142e71898);
          }
          else {
            if (!NAN(param_4)) {
              if (ppfVar5 == (float **)ppuStack_b0) {
                FUN_140b5f870(&ppuStack_b0,&UNK_142b2ea18);
              }
              pfStack_a8[(longlong)ppfVar5] = param_4;
              param_1[2] = (longlong)ppfVar5 + 1;
              *param_1 = ppuStack_b0;
              param_1[1] = pfStack_a8;
              return param_1;
            }
            FUN_1428d9430(&UNK_142e717c8,0x1d,&UNK_142e71898);
          }
        }
      }
    }
  }
LAB_140c9c4dc:
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

