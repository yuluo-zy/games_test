
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl
system_decorator::decorator_visual::stairs_railings_assemble::railings_tips::tip_endpoint
          (ulonglong *env,uint first,uint second,ulonglong index,ulonglong end)

{
  float fVar1;
  undefined8 uVar2;
  float fVar3;
  float fVar4;
  uint uVar5;
  float fVar6;
  char *extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  float fVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  undefined8 local_130;
  float local_128;
  float fStack_124;
  float local_120;
  undefined8 local_100;
  float local_f8;
  undefined8 local_f4;
  undefined4 local_ec;
  undefined8 local_e8;
  float local_e0;
  undefined8 local_dc;
  uint local_d4;
  undefined8 local_d0;
  float local_c8;
  undefined1 local_c4 [156];
  
  if (*(char *)(*env + end) != '\0') {
    local_128 = (float)first;
    fStack_124 = (float)second;
    indexmap::map::IndexMap::
    get<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>,std::hash::random::RandomState,tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>_>
              ();
    country_core::resources::stairs::stairs_state::StairsState::graph();
    local_128 = (float)first;
    indexmap::map::IndexMap::
    get<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<tuple_<country_core::resources::walls::decorator::DecoratorId,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,std::hash::random::RandomState,country_core::resources::walls::decorator::DecoratorId>
              ();
    if ((((extraout_RAX_00 != 0) && (*(longlong *)(extraout_RAX_00 + 0x10) != 0)) &&
        (extraout_RAX != (char *)0x0)) &&
       ((*(longlong *)(extraout_RAX_00 + 0x10) * 8 - 8U < 8 && (*extraout_RAX == '\x01')))) {
      country_core::resources::walls::decorator_storage::DecoratorStorage::try_lookup_storage();
      fVar6 = __real_bf000000;
      if (extraout_RAX_01 == 0) {
        if (env[5] <= index) {
                    /* WARNING: Subroutine does not return */
          core::panicking::panic_bounds_check();
        }
        if (env[7] <= index) {
                    /* WARNING: Subroutine does not return */
          core::panicking::panic_bounds_check();
        }
        uVar2 = *(undefined8 *)(env[4] + index * 0xc);
        fVar8 = (float)uVar2;
        fVar9 = (float)((ulonglong)uVar2 >> 0x20);
        fVar1 = *(float *)(env[4] + 8 + index * 0xc);
        local_130 = *(undefined8 *)(env[6] + index * 8);
        if (env[9] <= index) {
                    /* WARNING: Subroutine does not return */
          core::panicking::panic_bounds_check();
        }
        fVar7 = *(float *)(env[8] + index * 4) + __real_bf000000;
        ::utils::swizzlers::impl$0::x0y(&local_128,(float *)&local_130);
        fVar4 = __xmm_00000000000000003f0000003f000000._4_4_;
        fVar3 = (float)__xmm_00000000000000003f0000003f000000;
        fVar10 = local_120 * __real_3f000000 * __real_3f000000;
        fVar11 = local_128 * (float)__xmm_00000000000000003f0000003f000000 *
                 (float)__xmm_00000000000000003f0000003f000000;
        fVar12 = fStack_124 * __xmm_00000000000000003f0000003f000000._4_4_ *
                 __xmm_00000000000000003f0000003f000000._4_4_;
        ::utils::swizzlers::impl$0::x0y(&local_128,(float *)&local_130);
        uVar5 = ___xmm_80000000800000008000000080000000;
        local_d4 = (uint)local_120 ^ ___xmm_80000000800000008000000080000000;
        local_dc = CONCAT44((uint)fStack_124 ^ _UNK_1429258f4,
                            (uint)local_128 ^ ___xmm_80000000800000008000000080000000);
        local_e8 = CONCAT44(fVar12 + fVar9,fVar11 + fVar8);
        fVar7 = fVar7 + fVar6;
        if (fVar7 <= __real_3dcccccd) {
          fVar7 = __real_3dcccccd;
        }
        local_100 = CONCAT44((float)local_130,
                             local_130._4_4_ ^ ___xmm_80000000800000008000000080000000);
        local_e0 = fVar10 + fVar1;
        ::utils::swizzlers::impl$0::x0y(&local_128,(float *)&local_100);
        fVar10 = fVar7 * local_128;
        fVar11 = fVar7 * fStack_124;
        local_128 = (float)(local_130._4_4_ ^ uVar5);
        fStack_124 = (float)local_130;
        ::utils::swizzlers::impl$0::x0y(local_c4,&local_128);
        local_d0 = CONCAT44(fVar9 - fVar11 * fVar4,fVar8 - fVar10 * fVar3);
        local_f8 = fVar1 + 0.0;
        local_100 = CONCAT44(fVar9 + __xmm_00000000000000003e80000000000000._4_4_,
                             fVar8 + (float)__xmm_00000000000000003e80000000000000);
        local_f4 = __xmm_0000000000000000bf80000080000000;
        local_ec = 0x80000000;
        local_c8 = local_120 * fVar7 * fVar6 + fVar1;
        country_core::systems::collision::world::RaycastWorld::raycast_simple_exclude_wall_id();
        if (local_128._0_1_ == '\x11') {
          country_core::systems::collision::world::RaycastWorld::raycast_simple_exclude_wall_id();
          if (local_128._0_1_ == '\x11') {
            country_core::systems::collision::world::RaycastWorld::raycast_simple_exclude_wall_id();
            if (local_128._0_1_ == '\x11') {
              *(undefined1 *)(env[0xb] + end) = 1;
            }
          }
        }
      }
    }
  }
  return;
}

