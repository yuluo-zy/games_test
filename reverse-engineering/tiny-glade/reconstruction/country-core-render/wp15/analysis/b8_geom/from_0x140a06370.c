// addr=0x140a06370 name=from
// signature=void __cdecl from(void)


void __cdecl country_core::geometry::plane::impl$1::from(void)

{
  undefined8 uVar1;
  longlong lVar2;
  longlong lVar3;
  undefined8 *puVar4;
  undefined8 *extraout_RAX;
  byte *extraout_RAX_00;
  longlong extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  undefined8 *in_RCX;
  undefined **ppuVar5;
  longlong lVar6;
  float in_XMM1_Da;
  undefined8 local_1a0;
  undefined8 uStack_198;
  undefined8 uStack_190;
  float local_148 [3];
  undefined8 local_13c;
  undefined4 local_134;
  undefined8 local_130;
  float local_128;
  undefined4 local_124;
  float local_120;
  undefined8 local_11c;
  undefined4 local_114;
  undefined8 local_110;
  float local_108;
  undefined4 local_104;
  float local_100;
  undefined8 local_fc;
  undefined4 local_f4;
  undefined8 local_f0;
  float local_e8;
  undefined4 local_e4;
  float local_e0;
  undefined8 local_dc;
  undefined4 local_d4;
  undefined8 local_d0;
  longlong local_c8;
  longlong lStack_c0;
  longlong local_b8;
  longlong local_b0;
  longlong lStack_a8;
  longlong local_a0;
  longlong local_98;
  longlong lStack_90;
  longlong local_88;
  undefined8 *local_80;
  longlong local_78;
  longlong lStack_70;
  longlong local_68;
  undefined8 *local_58;
  undefined1 local_4d;
  undefined1 local_4c;
  undefined1 local_4b;
  undefined1 local_4a;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  local_148[0] = in_XMM1_Da * 0.5;
  local_148[2] = -local_148[0];
  local_148[1] = 0.0;
  local_13c = 0x3f80000000000000;
  local_134 = 0;
  local_130 = 0x3f8000003f800000;
  local_124 = 0;
  local_11c = 0x3f80000000000000;
  local_114 = 0;
  local_110 = 0x3f800000;
  local_104 = 0;
  local_fc = 0x3f80000000000000;
  local_f4 = 0;
  local_f0 = 0;
  local_e4 = 0;
  local_dc = 0x3f80000000000000;
  local_d4 = 0;
  local_d0 = 0x3f80000000000000;
  ppuVar5 = (undefined **)0x4;
  local_128 = local_148[0];
  local_120 = local_148[0];
  local_108 = local_148[2];
  local_100 = local_148[0];
  local_e8 = local_148[2];
  local_e0 = local_148[2];
  local_58 = in_RCX;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  *extraout_RAX = 0x200000000;
  extraout_RAX[1] = 1;
  local_80 = extraout_RAX;
  extraout_RAX[2] = 0x200000003;
  local_98 = 0;
  lStack_90 = 4;
  local_88 = 0;
  local_b0 = 0;
  lStack_a8 = 4;
  local_a0 = 0;
  local_c8 = 0;
  lStack_c0 = 4;
  local_b8 = 0;
  for (lVar6 = 0; lVar2 = local_88, lVar6 != 0x80; lVar6 = lVar6 + 0x20) {
    if (local_88 == local_98) {
      local_49 = 1;
      local_4a = 1;
      local_4b = 1;
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    lVar3 = local_a0;
    *(undefined4 *)(lStack_90 + 8 + lVar2 * 0xc) = *(undefined4 *)((longlong)local_148 + lVar6 + 8);
    *(undefined8 *)(lStack_90 + lVar2 * 0xc) = *(undefined8 *)((longlong)local_148 + lVar6);
    local_88 = lVar2 + 1;
    if (local_a0 == local_b0) {
      local_49 = 1;
      local_4a = 1;
      local_4b = 1;
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    lVar2 = local_b8;
    ppuVar5 = (undefined **)(lVar3 * 3);
    *(undefined4 *)(lStack_a8 + 8 + lVar3 * 0xc) = *(undefined4 *)((longlong)&local_134 + lVar6);
    *(undefined8 *)(lStack_a8 + lVar3 * 0xc) = *(undefined8 *)((longlong)&local_13c + lVar6);
    local_a0 = lVar3 + 1;
    uVar1 = *(undefined8 *)((longlong)&local_130 + lVar6);
    if (local_b8 == local_c8) {
      local_49 = 1;
      local_4a = 1;
      local_4b = 1;
      ppuVar5 = &PTR_s_crates_country_core_src_geometry_142ac6660;
      alloc::raw_vec::RawVec::grow_one<array_<f32,2>,alloc::alloc::Global>();
    }
    *(undefined8 *)(lStack_c0 + lVar2 * 8) = uVar1;
    local_b8 = lVar2 + 1;
  }
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX_00 & 1) == 0) {
    local_49 = 1;
    local_4a = 1;
    local_4b = 1;
    std::sys::random::hashmap_random_keys();
    extraout_RAX_00[0] = 1;
    extraout_RAX_00[1] = 0;
    extraout_RAX_00[2] = 0;
    extraout_RAX_00[3] = 0;
    extraout_RAX_00[4] = 0;
    extraout_RAX_00[5] = 0;
    extraout_RAX_00[6] = 0;
    extraout_RAX_00[7] = 0;
    *(longlong *)(extraout_RAX_00 + 8) = extraout_RAX_01;
    *(undefined ***)(extraout_RAX_00 + 0x10) = ppuVar5;
    lVar6 = extraout_RAX_01;
  }
  else {
    lVar6 = *(longlong *)(extraout_RAX_00 + 8);
    ppuVar5 = *(undefined ***)(extraout_RAX_00 + 0x10);
  }
  puVar4 = local_80;
  *(longlong *)(extraout_RAX_00 + 8) = lVar6 + 1;
  local_68 = local_88;
  local_78 = local_98;
  lStack_70 = lStack_90;
  local_4c = 1;
  local_4d = 1;
  country_core::render::mesh::Mesh::
  set_attribute<alloc::vec::Vec<array_<f32,3>,alloc::alloc::Global>_>();
  local_68 = local_a0;
  local_78 = local_b0;
  lStack_70 = lStack_a8;
  local_4c = 1;
  local_4d = 0;
  country_core::render::mesh::Mesh::
  set_attribute<alloc::vec::Vec<array_<f32,3>,alloc::alloc::Global>_>();
  local_68 = local_b8;
  local_78 = local_c8;
  lStack_70 = lStack_c0;
  local_4c = 0;
  local_4d = 0;
  country_core::render::mesh::Mesh::
  set_attribute<alloc::vec::Vec<array_<f32,2>,alloc::alloc::Global>_>();
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_02 != (undefined8 *)0x0) {
    *extraout_RAX_02 = 0x3f8000003f800000;
    extraout_RAX_02[1] = 0x3f8000003f800000;
    extraout_RAX_02[2] = 0x3f8000003f800000;
    extraout_RAX_02[3] = 0x3f8000003f800000;
    extraout_RAX_02[4] = 0x3f8000003f800000;
    extraout_RAX_02[5] = 0x3f8000003f800000;
    local_78 = 4;
    local_68 = 4;
    local_4c = 0;
    local_4d = 0;
    country_core::render::mesh::Mesh::
    set_attribute<alloc::vec::Vec<array_<f32,3>,alloc::alloc::Global>_>();
    local_58[0xe] = 5;
    local_58[0xc] = ppuVar5;
    local_58[0xd] = &DAT_142ac65e8;
    local_58[10] = 0;
    local_58[0xb] = lVar6;
    local_58[8] = 0;
    local_58[9] = 0;
    local_58[6] = uStack_190;
    local_58[7] = &anon_cc634b371bd8d6b6c1e8ab9dcbca5d5b_40_llvm_9402988993337875171;
    local_58[4] = local_1a0;
    local_58[5] = uStack_198;
    local_58[2] = 6;
    local_58[3] = 0x8000000000000000;
    *local_58 = 6;
    local_58[1] = puVar4;
    return;
  }
  local_4c = 0;
  local_4d = 0;
                    // WARNING: Subroutine does not return
  alloc::raw_vec::handle_error();
}


