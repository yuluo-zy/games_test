utils::geometry::aabb::Aabb3::expand_to_contain_point

void tg_math_141240330(ulonglong *param_1,ulonglong *param_2)

{
  float fVar1;
  float fVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  float fVar5;
  uint uVar6;
  uint uVar8;
  undefined1 auVar7 [16];
  undefined1 auVar9 [16];
  undefined1 auVar10 [16];
  undefined1 auVar11 [16];
  
  fVar1 = *(float *)(param_1 + 1);
  fVar2 = *(float *)(param_2 + 1);
  fVar5 = fVar2;
  if (fVar1 <= fVar2) {
    fVar5 = fVar1;
  }
  *(uint *)(param_1 + 1) = ~-(uint)NAN(fVar1) & (uint)fVar5 | -(uint)NAN(fVar1) & (uint)fVar2;
  fVar1 = *(float *)((longlong)param_1 + 0x14);
  fVar5 = fVar2;
  if (fVar2 <= fVar1) {
    fVar5 = fVar1;
  }
  uVar3 = *param_2;
  auVar7._8_8_ = 0;
  auVar7._0_8_ = uVar3;
  uVar4 = *param_1;
  auVar11._8_8_ = 0;
  auVar11._0_8_ = uVar4;
  uVar8 = (uint)(uVar3 >> 0x20);
  auVar10._8_8_ = 0;
  auVar10._0_8_ = uVar3;
  auVar11 = minps(auVar10,auVar11);
  uVar6 = -(uint)NAN((float)(uVar4 >> 0x20));
  *param_1 = CONCAT44(~uVar6 & auVar11._4_4_,~-(uint)NAN((float)uVar4) & auVar11._0_4_) |
             CONCAT44(uVar8 & uVar6,(uint)uVar3 & -(uint)NAN((float)uVar4));
  uVar4 = *(ulonglong *)((longlong)param_1 + 0xc);
  auVar9._8_8_ = 0;
  auVar9._0_8_ = uVar4;
  auVar7 = maxps(auVar7,auVar9);
  uVar6 = -(uint)NAN((float)(uVar4 >> 0x20));
  *(ulonglong *)((longlong)param_1 + 0xc) =
       CONCAT44(~uVar6 & auVar7._4_4_,~-(uint)NAN((float)uVar4) & auVar7._0_4_) |
       CONCAT44(uVar8 & uVar6,(uint)uVar3 & -(uint)NAN((float)uVar4));
  *(uint *)((longlong)param_1 + 0x14) =
       ~-(uint)NAN(fVar1) & (uint)fVar5 | -(uint)NAN(fVar1) & (uint)fVar2;
  return;
}

