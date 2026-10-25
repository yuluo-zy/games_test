
void __cdecl
core::ops::function::FnOnce::
call_once<country_core::systems::collision::world::impl_14::raycast_simple::closure_env_0,tuple_<rapier3d::geometry::collider_components::ColliderHandle,ref_<rapier3d::geometry::collider::Collider>_>_>
          (void)

{
  longlong lVar1;
  ulonglong *extraout_RAX;
  longlong *in_RCX;
  ulonglong uVar2;
  
  lVar1 = *in_RCX;
  rapier3d::geometry::collider_set::ColliderSet::get_mut_internal();
  if (extraout_RAX == (ulonglong *)0x0) {
                    /* WARNING: Subroutine does not return */
    option::unwrap_failed();
  }
  uVar2 = *extraout_RAX & 0xffffffff;
  if (((uVar2 < *(ulonglong *)(lVar1 + 0x10)) && (lVar1 = *(longlong *)(lVar1 + 8), lVar1 != 0)) &&
     (*(uint *)(lVar1 + 0x20 + uVar2 * 0x28) == ((uint)(*extraout_RAX >> 0x20) | 1))) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  std::panicking::begin_panic<ref_<str_>_>();
}

