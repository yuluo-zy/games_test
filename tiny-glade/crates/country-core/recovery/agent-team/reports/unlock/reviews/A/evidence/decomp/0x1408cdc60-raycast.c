// addr 0x1408cdc60 sig void __cdecl raycast(void) size 784

void __cdecl country_core::systems::collision::world::Rapier::raycast(void)

{
  undefined8 uVar1;
  uint uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  uint uVar5;
  ulonglong *extraout_RAX;
  longlong in_RCX;
  uint uVar6;
  ulonglong in_RDX;
  ulonglong uVar7;
  longlong lVar8;
  uint uVar9;
  undefined8 *in_R8;
  longlong lVar10;
  int iVar11;
  uint uVar12;
  undefined4 in_stack_00000028;
  uint in_stack_00000038;
  undefined8 local_138;
  undefined4 local_130;
  undefined1 local_128 [16];
  undefined1 *local_118;
  undefined *local_110;
  undefined8 local_108;
  ulonglong local_fc;
  undefined4 local_f4;
  undefined4 local_ec;
  longlong local_e0;
  undefined1 local_d0 [8];
  longlong local_c8;
  undefined1 *local_c0;
  uint local_b8;
  uint local_b4;
  int local_b0;
  undefined4 uStack_ac;
  undefined8 local_a4;
  undefined4 local_9c;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  
  uVar3 = *in_R8;
  uVar4 = in_R8[1];
  uVar1 = in_R8[2];
  iVar11 = 4;
  lVar8 = 0x20;
  uVar12 = 0;
  uVar7 = in_RDX;
  do {
    local_118 = local_d0;
    uVar5 = uVar12;
    do {
      uVar9 = (uint)in_R8;
      uVar6 = (uint)uVar7;
      if (lVar8 == 0x3cb0) {
        local_118 = (undefined1 *)local_e0;
        local_110 = (undefined *)CONCAT44(uVar9,uVar6);
        local_108 = CONCAT44(in_stack_00000028,iVar11);
        local_fc = local_138;
        local_f4 = local_130;
        if (iVar11 == 4) {
          *(undefined4 *)(in_RCX + 0x34) = 2;
        }
        else {
          rapier3d::geometry::collider_set::ColliderSet::get_mut_internal();
          if (extraout_RAX == (ulonglong *)0x0) {
                    // WARNING: Subroutine does not return
            core::option::unwrap_failed();
          }
          uVar7 = *extraout_RAX & 0xffffffff;
          if (((*(ulonglong *)(in_RDX + 0x10) <= uVar7) || (*(longlong *)(in_RDX + 8) == 0)) ||
             (*(uint *)(*(longlong *)(in_RDX + 8) + 0x20 + uVar7 * 0x28) !=
              ((uint)(*extraout_RAX >> 0x20) | 1))) {
                    // WARNING: Subroutine does not return
            std::panicking::begin_panic<ref_<str_>_>();
          }
          rapier_hit_to_raycast_world_result();
        }
        return;
      }
      lVar8 = lVar8 + 0x390;
      uVar12 = uVar5 + 1;
      uVar2 = uVar5 & 0x1f;
      uVar5 = uVar12;
    } while ((in_stack_00000038 >> uVar2 & 1) == 0);
    lVar10 = in_RDX + lVar8 + -0x390;
    local_110 = &DAT_142aa3858;
    local_108 = 0;
    local_fc = local_fc & 0xffffffff;
    local_ec = 0;
    local_c8 = lVar10;
    local_c0 = local_128;
    local_98 = uVar3;
    uStack_90 = uVar4;
    local_88 = uVar1;
    rapier3d::pipeline::query_pipeline::QueryPipeline::cast_ray_and_get_normal();
    if (local_b0 != 4) {
      local_138 = local_a4;
      local_130 = local_9c;
      uVar6 = local_b8;
      uVar9 = local_b4;
      local_e0 = lVar10;
      in_stack_00000028 = uStack_ac;
      iVar11 = local_b0;
    }
    in_R8 = (undefined8 *)(ulonglong)uVar9;
    uVar7 = (ulonglong)uVar6;
  } while( true );
}


// CALLEES: [{"address": "1428d8725", "name": "begin_panic<ref$<str$>_>"}, {"address": "140b945d0", "name": "cast_ray_and_get_normal"}, {"address": "140b93940", "name": "get_mut_internal"}, {"address": "1408ce4a0", "name": "rapier_hit_to_raycast_world_result"}, {"address": "1428d9310", "name": "unwrap_failed"}]