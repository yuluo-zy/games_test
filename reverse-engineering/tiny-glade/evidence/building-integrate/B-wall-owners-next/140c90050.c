
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl utils::random_splits_into(void)

{
  float fVar1;
  float fVar2;
  ulonglong in_RCX;
  longlong lVar3;
  longlong *in_R9;
  ulonglong uVar4;
  longlong lVar5;
  float extraout_XMM0_Da;
  float in_XMM1_Da;
  float fVar6;
  float fVar7;
  float fVar8;
  ulonglong local_98;
  
  fVar1 = __real_3f800000;
  if (in_RCX < 3) {
    lVar3 = in_R9[2];
    if ((ulonglong)(*in_R9 - lVar3) < 2) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      lVar3 = in_R9[2];
    }
    *(undefined8 *)(in_R9[1] + lVar3 * 4) = 0x3f80000000000000;
  }
  else {
    fVar6 = (__real_3f800000 / (float)(in_RCX - 1)) * ___real_3efd70a4;
    fVar7 = fVar6;
    if (in_XMM1_Da <= fVar6) {
      fVar7 = in_XMM1_Da;
    }
    lVar5 = in_R9[2];
    if (lVar5 == *in_R9) {
      alloc::raw_vec::RawVec::
      grow_one<country_core::render::sparse_instance_buffer::ChunkId,alloc::alloc::Global>();
    }
    *(undefined4 *)(in_R9[1] + lVar5 * 4) = 0;
    lVar5 = lVar5 + 1;
    in_R9[2] = lVar5;
    fVar8 = (float)in_RCX + __real_bf800000;
    if ((ulonglong)(*in_R9 - lVar5) < in_RCX - 2) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      lVar5 = in_R9[2];
    }
    fVar2 = __real_bf000000;
    lVar3 = in_R9[1];
    local_98 = 0;
    do {
      fastrand::Rng::f32();
      uVar4 = local_98 + 1;
      *(float *)(lVar3 + lVar5 * 4 + local_98 * 4) =
           (extraout_XMM0_Da + fVar2) *
           (float)(~-(uint)NAN(in_XMM1_Da) & (uint)fVar7 | (uint)fVar6 & -(uint)NAN(in_XMM1_Da)) +
           (float)uVar4 * (fVar1 / fVar8);
      local_98 = uVar4;
    } while (in_RCX - 2 != uVar4);
    in_R9[2] = lVar5 + uVar4;
    if (lVar5 + uVar4 == *in_R9) {
      alloc::raw_vec::RawVec::
      grow_one<country_core::render::sparse_instance_buffer::ChunkId,alloc::alloc::Global>();
    }
    lVar3 = lVar5 + uVar4 + -1;
    *(undefined4 *)(lVar5 * 4 + in_R9[1] + uVar4 * 4) = 0x3f800000;
  }
  in_R9[2] = lVar3 + 2;
  return;
}

