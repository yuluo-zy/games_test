
void __cdecl utils::geometry::rectangle::Rectangle2d::circular_slice(void)

{
  float fVar1;
  float *extraout_RAX;
  longlong *in_RCX;
  longlong in_RDX;
  longlong lVar2;
  float fVar3;
  float in_XMM2_Da;
  float in_XMM3_Da;
  float fVar4;
  float fVar5;
  float fVar6;
  float *local_c0;
  code *local_b8;
  undefined4 **local_b0;
  float *pfStack_a8;
  longlong local_a0;
  float local_94 [31];
  
  local_94[1] = -NAN;
  local_94[2] = -NAN;
  std::alloc::__default_lib_allocator::__rust_alloc();
  fVar1 = __real_3f800000;
  if (extraout_RAX == (float *)0x0) {
                    /* WARNING: Subroutine does not return */
    alloc::alloc::handle_alloc_error();
  }
  local_c0 = (float *)CONCAT44(local_c0._4_4_,in_XMM2_Da);
  if ((in_XMM2_Da < 0.0) || (__real_3f800000 < in_XMM2_Da)) {
    local_b0 = &local_c0;
    pfStack_a8 = (float *)core::fmt::float::impl$5::fmt;
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_fmt();
  }
  if (NAN(in_XMM2_Da)) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  *extraout_RAX = in_XMM2_Da;
  local_b0 = (undefined4 **)0x1;
  local_a0 = 1;
  fVar5 = *(float *)(in_RDX + 0x14);
  fVar4 = fVar5 + *(float *)(in_RDX + 0x10);
  fVar3 = fVar4 + fVar4;
  fVar6 = fVar5 / fVar3;
  local_94[0] = fVar6;
  if ((fVar6 < 0.0) || (fVar1 < fVar6)) {
    local_c0 = local_94;
    local_b8 = core::fmt::float::impl$5::fmt;
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_fmt();
  }
  if (NAN(fVar6)) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  fVar4 = fVar4 / fVar3;
  local_94[0] = fVar4;
  if ((fVar4 < 0.0) || (fVar1 < fVar4)) {
    local_c0 = local_94;
    local_b8 = core::fmt::float::impl$5::fmt;
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_fmt();
  }
  if (NAN(fVar4)) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  fVar3 = (fVar5 + fVar5 + *(float *)(in_RDX + 0x10)) / fVar3;
  local_94[0] = fVar3;
  if ((0.0 <= fVar3) && (fVar3 <= fVar1)) {
    if (NAN(fVar3)) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic();
    }
    fVar5 = (float)(~-(uint)(in_XMM3_Da < in_XMM2_Da) & (uint)in_XMM3_Da |
                   (uint)(__real_3f800000 + in_XMM3_Da) & -(uint)(in_XMM3_Da < in_XMM2_Da));
    lVar2 = 1;
    pfStack_a8 = extraout_RAX;
    if ((in_XMM2_Da < 0.0) && (0.0 < fVar5)) {
      alloc::raw_vec::RawVec::
      grow_one<country_core::render::sparse_instance_buffer::ChunkId,alloc::alloc::Global>();
      pfStack_a8[1] = 0.0;
      local_a0 = 2;
      lVar2 = 2;
    }
    if ((in_XMM2_Da < fVar6) && (fVar6 < fVar5)) {
      if ((undefined4 **)lVar2 == local_b0) {
        alloc::raw_vec::RawVec::
        grow_one<country_core::render::sparse_instance_buffer::ChunkId,alloc::alloc::Global>();
      }
      pfStack_a8[lVar2] = fVar6;
      lVar2 = lVar2 + 1;
      local_a0 = lVar2;
    }
    if ((in_XMM2_Da < fVar4) && (fVar4 < fVar5)) {
      if ((undefined4 **)lVar2 == local_b0) {
        alloc::raw_vec::RawVec::
        grow_one<country_core::render::sparse_instance_buffer::ChunkId,alloc::alloc::Global>();
      }
      pfStack_a8[lVar2] = fVar4;
      lVar2 = lVar2 + 1;
      local_a0 = lVar2;
    }
    if ((in_XMM2_Da < fVar3) && (fVar3 < fVar5)) {
      if ((undefined4 **)lVar2 == local_b0) {
        alloc::raw_vec::RawVec::
        grow_one<country_core::render::sparse_instance_buffer::ChunkId,alloc::alloc::Global>();
      }
      pfStack_a8[lVar2] = fVar3;
      lVar2 = lVar2 + 1;
      local_a0 = lVar2;
    }
    if ((in_XMM2_Da < fVar1) && (__real_3f800000 < fVar5)) {
      if ((undefined4 **)lVar2 == local_b0) {
        alloc::raw_vec::RawVec::
        grow_one<country_core::render::sparse_instance_buffer::ChunkId,alloc::alloc::Global>();
      }
      pfStack_a8[lVar2] = 0.0;
      lVar2 = lVar2 + 1;
      local_a0 = lVar2;
    }
    fVar6 = fVar6 + fVar1;
    if ((in_XMM2_Da < fVar6) && (fVar6 < fVar5)) {
      fVar6 = fVar6 + __real_bf800000;
      local_94[0] = fVar6;
      if ((fVar6 < 0.0) || (fVar1 < fVar6)) {
        local_c0 = local_94;
        local_b8 = core::fmt::float::impl$5::fmt;
                    /* WARNING: Subroutine does not return */
        core::panicking::panic_fmt();
      }
      if ((undefined4 **)lVar2 == local_b0) {
        alloc::raw_vec::RawVec::
        grow_one<country_core::render::sparse_instance_buffer::ChunkId,alloc::alloc::Global>();
      }
      pfStack_a8[lVar2] = fVar6;
      lVar2 = lVar2 + 1;
      local_a0 = lVar2;
    }
    fVar4 = fVar4 + fVar1;
    if ((in_XMM2_Da < fVar4) && (fVar4 < fVar5)) {
      fVar4 = fVar4 + __real_bf800000;
      local_94[0] = fVar4;
      if ((fVar4 < 0.0) || (fVar1 < fVar4)) {
        local_c0 = local_94;
        local_b8 = core::fmt::float::impl$5::fmt;
                    /* WARNING: Subroutine does not return */
        core::panicking::panic_fmt();
      }
      if ((undefined4 **)lVar2 == local_b0) {
        alloc::raw_vec::RawVec::
        grow_one<country_core::render::sparse_instance_buffer::ChunkId,alloc::alloc::Global>();
      }
      pfStack_a8[lVar2] = fVar4;
      lVar2 = lVar2 + 1;
      local_a0 = lVar2;
    }
    fVar3 = fVar3 + fVar1;
    if ((in_XMM2_Da < fVar3) && (fVar3 < fVar5)) {
      fVar3 = fVar3 + __real_bf800000;
      local_94[0] = fVar3;
      if ((fVar3 < 0.0) || (fVar1 < fVar3)) {
        local_c0 = local_94;
        local_b8 = core::fmt::float::impl$5::fmt;
                    /* WARNING: Subroutine does not return */
        core::panicking::panic_fmt();
      }
      if ((undefined4 **)lVar2 == local_b0) {
        alloc::raw_vec::RawVec::
        grow_one<country_core::render::sparse_instance_buffer::ChunkId,alloc::alloc::Global>();
      }
      pfStack_a8[lVar2] = fVar3;
      lVar2 = lVar2 + 1;
      local_a0 = lVar2;
    }
    if ((0.0 <= in_XMM3_Da) && (in_XMM3_Da <= fVar1)) {
      if (!NAN(in_XMM3_Da)) {
        if ((undefined4 **)lVar2 == local_b0) {
          alloc::raw_vec::RawVec::
          grow_one<country_core::render::sparse_instance_buffer::ChunkId,alloc::alloc::Global>();
        }
        pfStack_a8[lVar2] = in_XMM3_Da;
        in_RCX[2] = lVar2 + 1;
        *in_RCX = (longlong)local_b0;
        in_RCX[1] = (longlong)pfStack_a8;
        return;
      }
                    /* WARNING: Subroutine does not return */
      core::panicking::panic();
    }
    local_c0 = local_94;
    local_b8 = core::fmt::float::impl$5::fmt;
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_fmt();
  }
  local_c0 = local_94;
  local_b8 = core::fmt::float::impl$5::fmt;
                    /* WARNING: Subroutine does not return */
  core::panicking::panic_fmt();
}

