utils::geometry::aabb::intersection_aabb_segment2

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_141240b30(undefined4 *param_1,float param_2,float param_3,float param_4,float param_5,
                  float param_6,float param_7,float param_8,float param_9)

{
  undefined4 uVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  
  param_8 = param_8 - param_6;
  param_9 = param_9 - param_7;
  fVar8 = (param_4 - param_6) / param_8;
  fVar6 = (param_5 - param_7) / param_9;
  fVar3 = (param_2 - param_6) / param_8;
  fVar5 = (param_3 - param_7) / param_9;
  fVar2 = fVar3;
  if (fVar8 <= fVar3) {
    fVar2 = fVar8;
  }
  fVar4 = fVar5;
  if (fVar6 <= fVar5) {
    fVar4 = fVar6;
  }
  fVar7 = (float)(~-(uint)NAN(fVar6) & (uint)fVar4 | -(uint)NAN(fVar6) & (uint)fVar5);
  fVar4 = fVar3;
  if (fVar3 <= fVar8) {
    fVar4 = fVar8;
  }
  fVar2 = (float)(~-(uint)NAN(fVar8) & (uint)fVar2 | -(uint)NAN(fVar8) & (uint)fVar3);
  if (fVar5 <= fVar6) {
    fVar5 = fVar6;
  }
  fVar3 = fVar7;
  if (fVar7 <= fVar2) {
    fVar3 = fVar2;
  }
  fVar3 = (float)(~-(uint)NAN(fVar2) & (uint)fVar3 | -(uint)NAN(fVar2) & (uint)fVar7);
  fVar2 = fVar5;
  if (fVar4 <= fVar5) {
    fVar2 = fVar4;
  }
  uVar1 = 0;
  if ((fVar3 <= (float)(~-(uint)NAN(fVar4) & (uint)fVar2 | -(uint)NAN(fVar4) & (uint)fVar5)) &&
     (fVar3 <= _DAT_142925904)) {
    if (0.0 <= fVar3) {
      param_6 = param_6 + param_8 * fVar3;
      param_7 = param_7 + param_9 * fVar3;
    }
    else if ((((param_4 <= param_6) || (param_5 <= param_7)) || (param_6 <= param_2)) ||
            (param_7 <= param_3)) goto LAB_141240cd8;
    *(ulonglong *)(param_1 + 1) = CONCAT44(param_7,param_6);
    uVar1 = 1;
  }
LAB_141240cd8:
  *param_1 = uVar1;
  return;
}

