// next_index 0x140a38600 size=13
// signature: void __cdecl next_index(void)


void __cdecl country_core::resources::tree_storage::TreeStorage::next_index(void)

{
  longlong in_RCX;
  
  *(longlong *)(in_RCX + 0x30) = *(longlong *)(in_RCX + 0x30) + 1;
  return;
}

