// insert 0x140a38560 size=53
// signature: void __cdecl insert(void)


void __cdecl country_core::resources::tree_storage::TreeStorage::insert(void)

{
  longlong in_RCX;
  longlong in_RDX;
  ulonglong uVar1;
  
  hashbrown::map::HashMap::
  insert<u64,ref_<system_clutter::autoclutter::AutoClutterObject>,std::hash::random::RandomState,alloc::alloc::Global>
            ();
  uVar1 = in_RDX + 1U;
  if (in_RDX + 1U <= *(ulonglong *)(in_RCX + 0x30)) {
    uVar1 = *(ulonglong *)(in_RCX + 0x30);
  }
  *(ulonglong *)(in_RCX + 0x30) = uVar1;
  return;
}

