
void __cdecl country_core::systems::collision::world::Rapier::raycast_simple(void)

{
  undefined8 *puVar1;
  float fVar2;
  float fVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  uint uVar6;
  undefined8 uVar7;
  uint uVar8;
  ulonglong *extraout_RAX;
  undefined8 *in_RCX;
  ulonglong uVar9;
  longlong in_RDX;
  longlong lVar10;
  undefined8 *in_R8;
  longlong lVar11;
  longlong lVar12;
  uint uVar13;
  float in_stack_00000030;
  uint in_stack_00000038;
  undefined1 local_120 [16];
  int local_110;
  float local_104;
  undefined1 local_100 [8];
  longlong local_f8;
  undefined1 *local_f0;
  undefined8 local_e8;
  float local_e0;
  undefined8 local_dc;
  float local_d4;
  undefined1 *local_d0;
  undefined *local_c8;
  undefined8 local_c0;
  undefined4 local_b0;
  undefined4 local_a4;
  
  fVar2 = *(float *)(in_R8 + 1);
  uVar4 = *in_R8;
  uVar5 = *(undefined8 *)((longlong)in_R8 + 0xc);
  fVar3 = *(float *)((longlong)in_R8 + 0x14);
  lVar10 = 0x20;
  uVar13 = 0;
  lVar11 = 0;
  do {
    uVar8 = uVar13;
    do {
      if (lVar10 == 0x3cb0) {
        if (lVar11 == 0) {
          *(undefined1 *)in_RCX = 0x11;
        }
        else {
          rapier3d::geometry::collider_set::ColliderSet::get_mut_internal();
          if (extraout_RAX == (ulonglong *)0x0) {
                    /* WARNING: Subroutine does not return */
            core::option::unwrap_failed();
          }
          uVar9 = *extraout_RAX & 0xffffffff;
          if (((*(ulonglong *)(in_RDX + 0x10) <= uVar9) ||
              (lVar11 = *(longlong *)(in_RDX + 8), lVar11 == 0)) ||
             (*(uint *)(lVar11 + 0x20 + uVar9 * 0x28) != ((uint)(*extraout_RAX >> 0x20) | 1))) {
                    /* WARNING: Subroutine does not return */
            std::panicking::begin_panic<ref_<str_>_>();
          }
          puVar1 = (undefined8 *)(lVar11 + uVar9 * 0x28);
          in_RCX[2] = puVar1[2];
          uVar7 = puVar1[1];
          *in_RCX = *puVar1;
          in_RCX[1] = uVar7;
          in_RCX[3] = CONCAT44((float)((ulonglong)uVar4 >> 0x20) +
                               in_stack_00000030 * (float)((ulonglong)uVar5 >> 0x20),
                               (float)uVar4 + in_stack_00000030 * (float)uVar5);
          *(float *)(in_RCX + 4) = fVar2 + fVar3 * in_stack_00000030;
          *(float *)((longlong)in_RCX + 0x24) = in_stack_00000030;
        }
        return;
      }
      lVar10 = lVar10 + 0x390;
      uVar13 = uVar8 + 1;
      uVar6 = uVar8 & 0x1f;
      uVar8 = uVar13;
    } while ((in_stack_00000038 >> uVar6 & 1) == 0);
    lVar12 = in_RDX + lVar10 + -0x390;
    local_c8 = &DAT_142aa37f8;
    local_c0 = 0;
    local_b0 = 0;
    local_a4 = 0;
    local_f8 = lVar12;
    local_f0 = local_120;
    local_e8 = uVar4;
    local_e0 = fVar2;
    local_dc = uVar5;
    local_d4 = fVar3;
    local_d0 = local_100;
    rapier3d::pipeline::query_pipeline::QueryPipeline::cast_ray();
    if (local_110 == 1) {
      lVar11 = lVar12;
      in_stack_00000030 = local_104;
    }
  } while( true );
}

