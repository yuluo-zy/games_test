
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl country_core::systems::collision::world::RaycastMeshData::new_indexed(void)

{
  undefined1 *puVar1;
  ulonglong uVar2;
  undefined1 auVar3 [16];
  undefined1 *_Src;
  undefined1 *extraout_RAX;
  undefined1 *extraout_RAX_00;
  undefined1 *extraout_RAX_01;
  undefined1 *extraout_RAX_02;
  ulonglong extraout_RAX_03;
  undefined1 *extraout_RAX_04;
  ulonglong extraout_RAX_05;
  ulonglong extraout_RAX_06;
  ulonglong extraout_RAX_07;
  ulonglong *in_RCX;
  ulonglong *in_RDX;
  undefined1 **ppuVar4;
  undefined1 *puVar5;
  undefined1 *puVar6;
  ulonglong *in_R8;
  ulonglong *in_R9;
  ulonglong uVar7;
  undefined1 *_Dst;
  longlong lVar8;
  ulonglong uVar9;
  ulonglong *in_stack_00000028;
  ulonglong local_d8;
  undefined1 *local_d0;
  undefined1 *local_c8;
  undefined1 *puStack_c0;
  ulonglong local_b8;
  undefined1 *local_b0;
  ulonglong local_a8;
  ulonglong local_a0;
  ulonglong *local_98;
  undefined1 *local_90;
  ulonglong local_88;
  undefined1 **local_80;
  ulonglong local_78;
  undefined1 *local_70;
  ulonglong local_68;
  ulonglong local_60;
  undefined1 local_4d;
  undefined1 local_4c;
  undefined1 local_4b;
  undefined1 local_4a;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  local_d0 = (undefined1 *)in_RDX[1];
  puVar1 = (undefined1 *)in_RDX[2];
  puStack_c0 = (undefined1 *)in_R8[1];
  uVar7 = in_R8[2];
  local_49 = 1;
  local_4a = 1;
  local_4b = 1;
  local_4c = 1;
  local_c8 = puVar1;
  local_b8 = uVar7;
  local_b0 = local_d0;
  local_98 = in_R9;
  local_78 = (ulonglong)puStack_c0;
  ::utils::
  calculate_hash<tuple_<ref_<slice2_<array_<u32,3>_>_>,ref_<slice2_<glam::u32::uvec3::UVec3>_>_>_>()
  ;
  uVar9 = *local_98;
  puVar6 = extraout_RAX;
  if (!SBORROW8(0,uVar9)) {
    local_c8 = (undefined1 *)local_98[1];
    puStack_c0 = (undefined1 *)local_98[2];
    local_49 = 1;
    local_4a = 1;
    local_4b = 1;
    local_4c = 1;
    local_d0 = extraout_RAX;
    ::utils::calculate_hash<tuple_<u64,ref_<slice2_<glam::u32::uvec3::UVec3>_>_>_>();
    puVar6 = extraout_RAX_00;
  }
  uVar2 = *in_stack_00000028;
  if (!SBORROW8(0,uVar2)) {
    local_c8 = (undefined1 *)in_stack_00000028[1];
    puStack_c0 = (undefined1 *)in_stack_00000028[2];
    local_49 = 1;
    local_4a = 1;
    local_4b = 1;
    local_4c = 1;
    local_d0 = puVar6;
    ::utils::calculate_hash<tuple_<u64,ref_<slice2_<glam::u32::uvec2::UVec2>_>_>_>();
    puVar6 = extraout_RAX_01;
  }
  local_49 = 1;
  local_4a = 1;
  local_4b = 1;
  local_4c = 1;
  alloc::vec::spec_from_iter::impl$0::
  from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array_<f32,3>_>_>,country_core::render::mesh::impl_0::normals_without_indices::closure_env_0>_>
            ();
  _Src = local_b0;
  auVar3._8_8_ = 0;
  auVar3._0_8_ = puVar1;
  local_a0 = SUB168(auVar3 * ZEXT816(0xc),0);
  local_88 = uVar2;
  local_60 = uVar7;
  if ((SUB168(auVar3 * ZEXT816(0xc),8) == 0) && (local_a0 < 0x7ffffffffffffffd)) {
    if (local_a0 == 0) {
      _Dst = &DAT_00000004;
      puVar5 = (undefined1 *)0x0;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      puVar5 = puVar1;
      _Dst = extraout_RAX_02;
      if (extraout_RAX_02 == (undefined1 *)0x0) goto LAB_1408cf302;
    }
    memcpy(_Dst,_Src,local_a0);
    local_4d = 0;
    ppuVar4 = &local_d0;
    local_d0 = puVar5;
    local_c8 = _Dst;
    puStack_c0 = puVar1;
    parry3d::shape::shared_shape::SharedShape::trimesh();
    uVar7 = local_60;
    puVar5 = (undefined1 *)*in_RDX;
    local_90 = _Src;
    local_80 = ppuVar4;
    local_70 = (undefined1 *)extraout_RAX_03;
    if (puVar1 < puVar5) {
      if (puVar1 == (undefined1 *)0x0) {
        local_90 = &DAT_00000004;
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      else {
        std::alloc::__default_lib_allocator::__rust_realloc();
        local_90 = extraout_RAX_04;
        if (extraout_RAX_04 == (undefined1 *)0x0) {
          local_60 = (longlong)puVar5 * 0xc;
                    /* WARNING: Subroutine does not return */
          alloc::raw_vec::handle_error();
        }
      }
    }
    uVar2 = *in_R8;
    local_d8 = local_78;
    if (uVar7 < uVar2) {
      if (uVar7 == 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
        local_d8 = 4;
      }
      else {
        std::alloc::__default_lib_allocator::__rust_realloc();
        local_d8 = extraout_RAX_05;
        if (extraout_RAX_05 == 0) {
          local_70 = puVar1;
          local_60 = uVar2 * 0xc;
                    /* WARNING: Subroutine does not return */
          alloc::raw_vec::handle_error();
        }
      }
    }
    if (SBORROW8(0,uVar9)) {
      local_68 = 0;
    }
    else {
      uVar7 = local_98[1];
      local_a8 = local_98[2];
      local_68 = uVar7;
      if (local_a8 < uVar9) {
        if (local_a8 == 0) {
          local_68 = 4;
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        else {
          std::alloc::__default_lib_allocator::__rust_realloc();
          local_68 = extraout_RAX_06;
          if (extraout_RAX_06 == 0) {
            local_88 = uVar7;
            local_80 = (undefined1 **)(uVar9 * 0xc);
            local_70 = puVar1;
                    /* WARNING: Subroutine does not return */
            alloc::raw_vec::handle_error();
          }
        }
      }
    }
    if (SBORROW8(0,local_88)) {
      uVar7 = local_d8;
      uVar9 = 0;
    }
    else {
      uVar2 = in_stack_00000028[1];
      uVar7 = in_stack_00000028[2];
      uVar9 = uVar2;
      if (uVar7 < local_88) {
        lVar8 = local_88 << 3;
        if (uVar7 == 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
          uVar9 = 4;
        }
        else {
          std::alloc::__default_lib_allocator::__rust_realloc();
          uVar9 = extraout_RAX_07;
          if (extraout_RAX_07 == 0) {
            local_88 = lVar8;
            local_80 = (undefined1 **)uVar2;
            local_70 = puVar1;
                    /* WARNING: Subroutine does not return */
            alloc::raw_vec::handle_error();
          }
        }
      }
    }
    *in_RCX = local_d8;
    in_RCX[1] = local_60;
    in_RCX[2] = (ulonglong)local_90;
    in_RCX[3] = (ulonglong)puVar1;
    in_RCX[6] = local_68;
    in_RCX[7] = local_a8;
    in_RCX[8] = uVar9;
    in_RCX[9] = uVar7;
    in_RCX[4] = (ulonglong)local_70;
    in_RCX[5] = (ulonglong)local_80;
    uVar7 = _UNK_142aa3ec8;
    in_RCX[10] = _DAT_142aa3ec0;
    in_RCX[0xb] = uVar7;
    uVar7 = _UNK_142aa3ed8;
    in_RCX[0xc] = _DAT_142aa3ed0;
    in_RCX[0xd] = uVar7;
    in_RCX[0xe] = (ulonglong)puVar6;
    return;
  }
LAB_1408cf302:
  local_4d = 1;
                    /* WARNING: Subroutine does not return */
  alloc::raw_vec::handle_error();
}

