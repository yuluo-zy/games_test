// addr=0x140a06b00 name=try_from
// signature=void __cdecl try_from(void)


void __cdecl country_core::geometry::sphere::impl$2::try_from(void)

{
  size_t _Size;
  float fVar1;
  undefined8 *extraout_RAX;
  undefined1 *extraout_RAX_00;
  longlong lVar2;
  ulonglong extraout_RAX_01;
  byte *extraout_RAX_02;
  ulonglong extraout_RAX_03;
  ulonglong *in_RCX;
  ulonglong in_RDX;
  float *pfVar3;
  ulonglong *puVar4;
  ulonglong uVar5;
  undefined8 *puVar6;
  undefined1 *_Dst;
  float in_XMM2_Da;
  undefined1 *local_1b8;
  code *local_1b0;
  ulonglong *local_1a8;
  code *local_1a0;
  ulonglong *local_198;
  ulonglong local_190;
  ulonglong local_180;
  longlong local_168;
  ulonglong local_160;
  undefined8 uStack_158;
  undefined8 local_150;
  undefined **local_148;
  ulonglong uStack_140;
  undefined1 **local_138;
  ulonglong uStack_130;
  ulonglong local_128;
  ulonglong uStack_120;
  ulonglong uStack_118;
  undefined *local_110;
  ulonglong uStack_108;
  ulonglong local_100;
  ulonglong uStack_f8;
  ulonglong local_f0;
  ulonglong *local_e8;
  undefined *puStack_e0;
  ulonglong local_d8;
  longlong local_d0;
  undefined1 local_c8 [8];
  float fStack_c0;
  ulonglong local_b0;
  ulonglong uStack_a8;
  ulonglong local_a0;
  undefined1 *local_98;
  ulonglong local_90;
  ulonglong local_88;
  undefined8 *puStack_80;
  ulonglong local_78;
  ulonglong local_68;
  undefined8 *local_60;
  undefined1 local_53;
  undefined1 local_52;
  undefined1 local_51;
  undefined1 local_50;
  undefined1 local_4f;
  undefined1 local_4e;
  undefined1 local_4d;
  undefined1 local_4c;
  undefined1 local_4b;
  undefined1 local_4a;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  if (in_RDX < 0x50) {
    fStack_c0 = in_XMM2_Da;
    hexasphere::Subdivided::
    new<array_<f32,2>,hexasphere::shapes::IcoSphereBase,country_core::geometry::sphere::impl_2::try_from::closure_env_0>
              ();
    local_d0 = (longlong)local_1a8 * 0xc;
    if (local_1a8 == (ulonglong *)0x0) {
      local_60 = (undefined8 *)&DAT_00000004;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
        alloc::raw_vec::handle_error();
      }
      if (local_1a8 == (ulonglong *)0x1) {
        uVar5 = 0;
      }
      else {
        pfVar3 = (float *)((longlong)local_1b0 + 0x10);
        uVar5 = 0;
        puVar6 = extraout_RAX;
        do {
          fVar1 = pfVar3[-2];
          *puVar6 = CONCAT44(pfVar3[-3] * fStack_c0,pfVar3[-4] * fStack_c0);
          *(float *)(puVar6 + 1) = fVar1 * fStack_c0;
          fVar1 = pfVar3[2];
          *(ulonglong *)((longlong)puVar6 + 0xc) =
               CONCAT44(pfVar3[1] * fStack_c0,*pfVar3 * fStack_c0);
          *(float *)((longlong)puVar6 + 0x14) = fVar1 * fStack_c0;
          uVar5 = uVar5 + 2;
          pfVar3 = pfVar3 + 8;
          puVar6 = puVar6 + 3;
        } while (((ulonglong)local_1a8 & 0xfffffffffffffffe) != uVar5);
      }
      local_60 = extraout_RAX;
      if (((ulonglong)local_1a8 & 1) != 0) {
        pfVar3 = (float *)((longlong)local_1b0 + uVar5 * 0x10);
        fVar1 = pfVar3[2];
        *(ulonglong *)((longlong)extraout_RAX + uVar5 * 0xc) =
             CONCAT44(fStack_c0 * pfVar3[1],fStack_c0 * *pfVar3);
        *(float *)((longlong)extraout_RAX + uVar5 * 0xc + 8) = fStack_c0 * fVar1;
      }
    }
    local_90 = (ulonglong)local_1a8;
    local_4c = 1;
    alloc::vec::spec_from_iter::impl$0::
    from_iter<array_<f32,3>,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::sse2::vec3a::Vec3A>_>,array_<f32,3>_(*)(glam::f32::sse2::vec3a::Vec3A)>_>
              ();
    _Size = local_190 * 8;
    if ((local_190 >> 0x3d != 0) || (0x7ffffffffffffffc < _Size)) {
LAB_140a07067:
      local_4d = 1;
      local_4e = 1;
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    if (_Size == 0) {
      _Dst = &DAT_00000004;
      local_68 = 0;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_00 == (undefined1 *)0x0) goto LAB_140a07067;
      local_68 = local_190;
      _Dst = extraout_RAX_00;
    }
    memcpy(_Dst,local_198,_Size);
    lVar2 = (local_168 + 1) * (local_168 + 1);
    local_b0 = lVar2 * 0x14;
    uVar5 = lVar2 * 0x50;
    local_98 = _Dst;
    if ((local_b0 >> 0x3e != 0) || (0x7ffffffffffffffc < uVar5)) {
LAB_140a06db5:
      local_4f = 1;
      local_50 = 1;
      local_51 = 1;
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    if (uVar5 == 0) {
      uStack_a8 = 4;
      local_b0 = 0;
      puVar4 = local_198;
    }
    else {
      puVar4 = (ulonglong *)&DAT_00000004;
      std::alloc::__default_lib_allocator::__rust_alloc();
      uStack_a8 = extraout_RAX_01;
      if (extraout_RAX_01 == 0) goto LAB_140a06db5;
    }
    local_a0 = 0;
    for (uVar5 = 0; uVar5 < 0x14; uVar5 = uVar5 + 1) {
      if (local_180 <= uVar5) {
        local_49 = 1;
        local_4a = 1;
        local_4b = 1;
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      local_49 = 1;
      local_4a = 1;
      local_4b = 1;
      puVar4 = &local_b0;
      hexasphere::Triangle::add_indices();
    }
    std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
    if ((*extraout_RAX_02 & 1) == 0) {
      local_49 = 1;
      local_4a = 1;
      local_4b = 1;
      std::sys::random::hashmap_random_keys();
      extraout_RAX_02[0] = 1;
      extraout_RAX_02[1] = 0;
      extraout_RAX_02[2] = 0;
      extraout_RAX_02[3] = 0;
      extraout_RAX_02[4] = 0;
      extraout_RAX_02[5] = 0;
      extraout_RAX_02[6] = 0;
      extraout_RAX_02[7] = 0;
      *(ulonglong *)(extraout_RAX_02 + 8) = extraout_RAX_03;
      *(ulonglong **)(extraout_RAX_02 + 0x10) = puVar4;
      local_f0 = extraout_RAX_03;
    }
    else {
      local_f0 = *(ulonglong *)(extraout_RAX_02 + 8);
      puVar4 = *(ulonglong **)(extraout_RAX_02 + 0x10);
    }
    *(ulonglong *)(extraout_RAX_02 + 8) = local_f0 + 1;
    local_100 = 0;
    uStack_f8 = 0;
    local_110 = &anon_cc634b371bd8d6b6c1e8ab9dcbca5d5b_40_llvm_9402988993337875171;
    uStack_108 = 0;
    uStack_130 = 0x8000000000000000;
    puStack_e0 = &DAT_142ac66c0;
    local_d8 = 6;
    local_148 = (undefined **)local_b0;
    uStack_140 = uStack_a8;
    local_138 = (undefined1 **)local_a0;
    local_88 = local_90;
    puStack_80 = local_60;
    local_78 = local_90;
    local_52 = 1;
    local_53 = 1;
    local_e8 = puVar4;
    country_core::render::mesh::Mesh::
    set_attribute<alloc::vec::Vec<array_<f32,3>,alloc::alloc::Global>_>();
    local_78 = local_150;
    local_88 = local_160;
    puStack_80 = (undefined8 *)uStack_158;
    local_52 = 1;
    local_53 = 0;
    country_core::render::mesh::Mesh::
    set_attribute<alloc::vec::Vec<array_<f32,3>,alloc::alloc::Global>_>();
    local_88 = local_68;
    puStack_80 = (undefined8 *)local_98;
    local_78 = local_190;
    local_52 = 0;
    local_53 = 0;
    country_core::render::mesh::Mesh::
    set_attribute<alloc::vec::Vec<array_<f32,2>,alloc::alloc::Global>_>();
    in_RCX[0xe] = local_d8;
    in_RCX[0xc] = (ulonglong)local_e8;
    in_RCX[0xd] = (ulonglong)puStack_e0;
    in_RCX[10] = uStack_f8;
    in_RCX[0xb] = local_f0;
    in_RCX[8] = uStack_108;
    in_RCX[9] = local_100;
    in_RCX[6] = uStack_118;
    in_RCX[7] = (ulonglong)local_110;
    in_RCX[4] = local_128;
    in_RCX[5] = uStack_120;
    in_RCX[2] = (ulonglong)local_138;
    in_RCX[3] = uStack_130;
    *in_RCX = (ulonglong)local_148;
    in_RCX[1] = uStack_140;
    core::ptr::
    drop_in_place<hexasphere::Subdivided<array_<f32,2>,hexasphere::shapes::IcoSphereBase>_>();
  }
  else {
    local_88 = (in_RDX + 1) * (in_RDX + 1) * 10 + 2;
    local_1b8 = local_c8;
    local_1b0 = core::fmt::num::imp::impl$9::fmt;
    local_1a8 = &local_88;
    local_1a0 = core::fmt::num::imp::impl$9::fmt;
    local_148 = &PTR_DAT_142ac6758;
    uStack_140 = 3;
    local_138 = &local_1b8;
    uStack_130 = 2;
    local_128 = 0;
    alloc::fmt::format::format_inner();
    *in_RCX = 0x8000000000000000;
  }
  return;
}


