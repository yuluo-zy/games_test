
void __cdecl country_core::resources::stairs::stairs_state::StairsState::compute_checksum(void)

{
  undefined8 extraout_RAX;
  byte *in_RCX;
  ulonglong local_68;
  undefined8 local_60;
  longlong local_58;
  
  if ((*in_RCX & 1) == 0) {
    iter_edges_and_stubs<country_core::resources::stairs::stairs_state::impl_5::take_snapshot_impl::closure_env_0>
              ();
    StairsSnapshot::new();
    if ((local_68 & 1) == 0) {
      ::utils::calculate_hash<country_core::resources::stairs::stairs_state::StairsSnapshot>();
      local_60 = extraout_RAX;
    }
    if (local_58 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    in_RCX[0] = 1;
    in_RCX[1] = 0;
    in_RCX[2] = 0;
    in_RCX[3] = 0;
    in_RCX[4] = 0;
    in_RCX[5] = 0;
    in_RCX[6] = 0;
    in_RCX[7] = 0;
    *(undefined8 *)(in_RCX + 8) = local_60;
  }
  return;
}

