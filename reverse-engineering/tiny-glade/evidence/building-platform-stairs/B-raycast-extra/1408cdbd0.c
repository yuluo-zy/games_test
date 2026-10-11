
void __cdecl country_core::systems::collision::world::RaycastWorld::get_collider_mesh(void)

{
  longlong in_RCX;
  int in_EDX;
  ulonglong in_R8;
  
  if ((((in_R8 & 0xffffffff) < *(ulonglong *)(in_RCX + 0x10)) && (*(longlong *)(in_RCX + 8) != 0))
     && (*(int *)(*(longlong *)(in_RCX + 8) + 0x20 + (in_R8 & 0xffffffff) * 0x28) == in_EDX)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  std::panicking::begin_panic<ref_<str_>_>();
}

