
void FUN_14123ae50(ulonglong *param_1,ulonglong *param_2)

{
  float fVar1;
  float fVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  float fVar6;
  float fVar7;
  uint uVar8;
  uint uVar9;
  float fVar10;
  uint uVar11;
  undefined1 auVar12 [16];
  undefined1 auVar13 [16];
  undefined1 auVar14 [16];
  undefined1 auVar15 [16];
  undefined1 auVar16 [16];
  undefined1 auVar17 [16];
  undefined1 auVar18 [16];
  undefined1 auVar19 [16];
  uint uVar20;
  uint uVar21;
  undefined1 auVar22 [16];
  
  fVar1 = *(float *)(param_2 + 1);
  fVar10 = *(float *)((longlong)param_2 + 0x14);
  fVar2 = fVar10;
  if (fVar1 <= fVar10) {
    fVar2 = fVar1;
  }
  fVar7 = (float)(~-(uint)NAN(fVar1) & (uint)fVar2 | -(uint)NAN(fVar1) & (uint)fVar10);
  fVar2 = *(float *)(param_2 + 4);
  fVar6 = fVar2;
  if (fVar7 <= fVar2) {
    fVar6 = fVar7;
  }
  if (fVar10 <= fVar1) {
    fVar10 = fVar1;
  }
  fVar1 = fVar2;
  if (fVar2 <= fVar10) {
    fVar1 = fVar10;
  }
  *(uint *)(param_1 + 1) = -(uint)NAN(fVar7) & (uint)fVar2 | ~-(uint)NAN(fVar7) & (uint)fVar6;
  uVar3 = *param_2;
  auVar14._8_8_ = 0;
  auVar14._0_8_ = uVar3;
  uVar4 = *(ulonglong *)((longlong)param_2 + 0xc);
  auVar19._8_8_ = 0;
  auVar19._0_8_ = uVar4;
  uVar5 = param_2[3];
  auVar16._8_8_ = 0;
  auVar16._0_8_ = uVar4;
  auVar17 = maxps(auVar16,auVar14);
  auVar12 = minps(auVar19,auVar14);
  uVar8 = -(uint)NAN((float)uVar3);
  uVar9 = -(uint)NAN((float)(uVar3 >> 0x20));
  auVar22._0_4_ = ~uVar8 & auVar17._0_4_;
  auVar22._4_4_ = ~uVar9 & auVar17._4_4_;
  auVar22._8_4_ = auVar17._8_4_;
  auVar22._12_4_ = auVar17._12_4_;
  uVar20 = (uint)uVar4 & uVar8;
  uVar21 = (uint)(uVar4 >> 0x20) & uVar9;
  auVar17._4_4_ = uVar21;
  auVar17._0_4_ = uVar20;
  auVar17._8_8_ = 0;
  auVar22 = auVar22 | auVar17;
  uVar11 = (uint)(uVar5 >> 0x20);
  auVar18._8_8_ = 0;
  auVar18._0_8_ = uVar5;
  auVar19 = maxps(auVar18,auVar22);
  auVar15._0_4_ = ~uVar8 & auVar12._0_4_;
  auVar15._4_4_ = ~uVar9 & auVar12._4_4_;
  auVar15._8_4_ = auVar12._8_4_;
  auVar15._12_4_ = auVar12._12_4_;
  auVar12._4_4_ = uVar21;
  auVar12._0_4_ = uVar20;
  auVar12._8_8_ = 0;
  auVar15 = auVar15 | auVar12;
  auVar13._8_8_ = 0;
  auVar13._0_8_ = uVar5;
  auVar17 = minps(auVar13,auVar15);
  *param_1 = CONCAT44(~-(uint)NAN(auVar15._4_4_) & auVar17._4_4_,
                      ~-(uint)NAN(auVar15._0_4_) & auVar17._0_4_) |
             CONCAT44(uVar11 & -(uint)NAN(auVar15._4_4_),(uint)uVar5 & -(uint)NAN(auVar15._0_4_));
  *(ulonglong *)((longlong)param_1 + 0xc) =
       CONCAT44(~-(uint)NAN(auVar22._4_4_) & auVar19._4_4_,
                ~-(uint)NAN(auVar22._0_4_) & auVar19._0_4_) |
       CONCAT44(uVar11 & -(uint)NAN(auVar22._4_4_),(uint)uVar5 & -(uint)NAN(auVar22._0_4_));
  *(uint *)((longlong)param_1 + 0x14) =
       -(uint)NAN(fVar10) & (uint)fVar2 | ~-(uint)NAN(fVar10) & (uint)fVar1;
  return;
}

