
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::new(void)

{
  undefined8 uVar1;
  longlong extraout_RAX;
  byte *extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong lVar2;
  undefined8 *in_RCX;
  undefined8 uVar3;
  
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX != 0) {
    uVar3 = 0x1000;
    alloc::vec::Vec::
    extend_with<country_core::systems::wall::wall_state::wall_spatial_hash::GridCell,alloc::alloc::Global>
              ();
    std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
    if ((*extraout_RAX_00 & 1) == 0) {
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
      *(undefined8 *)(extraout_RAX_00 + 0x10) = uVar3;
      lVar2 = extraout_RAX_01;
    }
    else {
      lVar2 = *(longlong *)(extraout_RAX_00 + 8);
      uVar3 = *(undefined8 *)(extraout_RAX_00 + 0x10);
    }
    *(longlong *)(extraout_RAX_00 + 8) = lVar2 + 1;
    in_RCX[2] = 0;
    *in_RCX = 0x1000;
    in_RCX[1] = extraout_RAX;
    uVar1 = _UNK_142adc4c8;
    in_RCX[3] = anon_4935de7db48d1856e7679eb80d7ea883_2_llvm_17144954438807827936;
    in_RCX[4] = uVar1;
    uVar1 = _UNK_142adc4d8;
    in_RCX[5] = _DAT_142adc4d0;
    in_RCX[6] = uVar1;
    in_RCX[7] = lVar2;
    in_RCX[8] = uVar3;
    in_RCX[9] = 0x4000000040;
    in_RCX[10] = __xmm_00000000000000004302000043020000;
    *(undefined4 *)(in_RCX + 0xb) = 0;
    return;
  }
                    /* WARNING: Subroutine does not return */
  alloc::raw_vec::handle_error();
}

