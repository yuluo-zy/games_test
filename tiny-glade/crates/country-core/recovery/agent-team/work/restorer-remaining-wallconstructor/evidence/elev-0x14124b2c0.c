
void __cdecl
system_wall_constructor::construct_elevation_supports::circle_bottom_layer::
construct_circle_bottom_layer(void)

{
  longlong lVar1;
  char extraout_AL;
  int iVar2;
  ulonglong uVar3;
  longlong extraout_RAX;
  undefined8 *extraout_RAX_00;
  longlong lVar4;
  int *piVar5;
  float *pfVar6;
  float *pfVar7;
  longlong in_RDX;
  ulonglong uVar8;
  undefined8 *in_R8;
  float *pfVar9;
  float *pfVar10;
  float *pfVar11;
  bool bVar12;
  float extraout_XMM0_Da;
  float fVar13;
  undefined8 local_1e0;
  longlong local_1d8;
  undefined8 local_1d0;
  undefined8 uStack_1c8;
  longlong local_1c0;
  undefined8 local_1b8;
  undefined8 uStack_1b0;
  longlong local_1a8;
  undefined8 local_1a0;
  undefined8 uStack_198;
  int local_190;
  undefined8 *local_188;
  undefined1 *local_180;
  undefined **local_178;
  undefined8 local_170;
  float **local_168;
  undefined8 local_160;
  undefined8 local_158;
  undefined8 local_148;
  float local_140;
  undefined4 local_13c;
  float *local_138;
  code *local_130;
  longlong local_128;
  longlong local_120;
  longlong local_118;
  float *local_110;
  float *local_108;
  float local_f8;
  undefined1 local_f1;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  local_148 = *in_R8;
  local_140 = *(float *)(in_R8 + 1) + 0.28;
  local_13c = *(undefined4 *)((longlong)in_R8 + 0xc);
  ::utils::geometry::circle::Circle2d::perimeter();
  fVar13 = extraout_XMM0_Da * 1000.0;
  uVar3 = 0;
  if (0.0 <= fVar13) {
    uVar3 = (longlong)(fVar13 - 9.223372e+18) & (longlong)fVar13 >> 0x3f | (longlong)fVar13;
  }
  uVar8 = 0xffffffffffffffff;
  if (fVar13 <= 1.8446743e+19) {
    uVar8 = uVar3;
  }
  minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
  local_128 = extraout_RAX + uVar8;
  local_120 = in_RDX;
  country_core::resources::walls::inner_wall_state::WallStyle::is_halftimbered();
  if (extraout_AL != '\0') {
    return;
  }
  frozen::FrozenMap::
  get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
            ();
  if (extraout_RAX_00 != (undefined8 *)0x0) {
    local_178 = (undefined **)&anon_11a7c4262c3a602e5b20874ec0ef09d1_1_llvm_11959870551130474958;
    local_170 = 0x3a;
    if ((extraout_RAX_00[3] != 0x3a) ||
       (iVar2 = memcmp(&anon_11a7c4262c3a602e5b20874ec0ef09d1_1_llvm_11959870551130474958,
                       (void *)extraout_RAX_00[2],0x3a), iVar2 != 0)) {
      local_1e0 = 0;
                    // WARNING: Subroutine does not return
      core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
    }
    lVar4 = (**(code **)(extraout_RAX_00[1] + 0x18))(*extraout_RAX_00);
    if (lVar4 != 0) {
      local_f1 = 0;
      lVar1 = *(longlong *)(local_120 + 0x150);
      if (lVar1 == 0) {
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      LOCK();
      bVar12 = *(char *)(lVar1 + 0x18) == '\0';
      if (bVar12) {
        *(char *)(lVar1 + 0x18) = '\x01';
      }
      UNLOCK();
      if (!bVar12) {
        parking_lot::raw_mutex::RawMutex::lock_slow();
      }
      parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
      piVar5 = (int *)(*(longlong *)(lVar1 + 0x10) * 4 + *(longlong *)(lVar1 + 8) + -4);
      if (piVar5 == (int *)0x0 || *(longlong *)(lVar1 + 0x10) == 0) {
        local_1d8 = *(longlong *)(lVar4 + 0x28);
        if (local_1d8 == 0) {
                    // WARNING: Subroutine does not return
          core::option::unwrap_failed();
        }
        local_190 = (int)local_1d8 + -1;
      }
      else {
        local_190 = *piVar5;
        local_1d8 = *(longlong *)(lVar4 + 0x28);
      }
      local_1e0 = *(undefined8 *)(lVar4 + 0x20);
      local_1d0 = *(undefined8 *)(lVar4 + 0x30);
      uStack_1c8 = *(undefined8 *)(lVar4 + 0x38);
      local_1b8 = *(undefined8 *)(lVar4 + 0x40);
      uStack_1b0 = *(undefined8 *)(lVar4 + 0x48);
      local_1a0 = *(undefined8 *)(lVar4 + 0x50);
      uStack_198 = *(undefined8 *)(lVar4 + 0x58);
      local_188 = &local_1e0;
      local_180 = &local_f1;
      local_1c0 = lVar1;
      local_1a8 = lVar4;
      ceilf(extraout_XMM0_Da / 0.69);
      local_118 = 0;
      local_110 = (float *)&DAT_00000004;
      local_108 = (float *)0x0;
      ::utils::random_splits_into();
      pfVar9 = local_110 + (longlong)local_108;
      bVar12 = true;
      pfVar6 = local_108;
      pfVar10 = local_110;
      while (pfVar10 != pfVar9) {
        pfVar11 = pfVar10 + 1;
        pfVar7 = pfVar6;
        pfVar6 = pfVar10;
        if (bVar12) {
          if (pfVar11 == pfVar9) break;
          pfVar7 = pfVar10;
          pfVar6 = pfVar11;
          pfVar11 = pfVar10 + 2;
        }
        local_f8 = *pfVar7;
        fVar13 = *pfVar6;
        if ((local_f8 < 0.0) || (1.0 < local_f8)) {
          local_138 = &local_f8;
          local_130 = core::fmt::float::impl$5::fmt;
          local_178 = &anon_95101f4d2e0328d894274322bc39dc37_3_llvm_14090887955549727255;
          local_170 = 1;
          local_158 = 0;
          local_168 = &local_138;
          local_160 = 1;
                    // WARNING: Subroutine does not return
          core::panicking::panic_fmt();
        }
        if (NAN(local_f8)) {
                    // WARNING: Subroutine does not return
          core::panicking::panic();
        }
        ::utils::geometry::circle::Circle2d::get_pos_at_u();
        local_f8 = fVar13;
        if ((fVar13 < 0.0) || (1.0 < fVar13)) {
          local_138 = &local_f8;
          local_130 = core::fmt::float::impl$5::fmt;
          local_178 = &anon_95101f4d2e0328d894274322bc39dc37_3_llvm_14090887955549727255;
          local_170 = 1;
          local_158 = 0;
          local_168 = &local_138;
          local_160 = 1;
                    // WARNING: Subroutine does not return
          core::panicking::panic_fmt();
        }
        if (NAN(fVar13)) {
                    // WARNING: Subroutine does not return
          core::panicking::panic();
        }
        ::utils::geometry::circle::Circle2d::get_pos_at_u();
        minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
        bVar12 = false;
        add_bottom_brick<ref_mut_<system_wall_constructor::construct_elevation_supports::circle_bottom_layer::construct_circle_bottom_layer::closure_env_0>_>
                  ();
        pfVar10 = pfVar11;
      }
      if (local_118 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      lVar4 = local_1c0;
      parking_lot_core::parking_lot::deadlock_impl::release_resource();
      LOCK();
      bVar12 = *(char *)(lVar4 + 0x18) == '\x01';
      if (bVar12) {
        *(char *)(lVar4 + 0x18) = '\0';
      }
      UNLOCK();
      if (bVar12) {
        return;
      }
      parking_lot::raw_mutex::RawMutex::unlock_slow();
      return;
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

