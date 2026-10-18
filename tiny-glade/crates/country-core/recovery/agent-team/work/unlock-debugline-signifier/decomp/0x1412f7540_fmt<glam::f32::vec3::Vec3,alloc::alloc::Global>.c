
void __cdecl alloc::vec::impl$27::fmt<glam::f32::vec3::Vec3,alloc::alloc::Global>(void)

{
  longlong in_RCX;
  longlong lVar1;
  
  lVar1 = *(longlong *)(in_RCX + 0x10);
  core::fmt::Formatter::debug_list();
  if (lVar1 != 0) {
    lVar1 = lVar1 * 0xc;
    do {
      core::fmt::builders::DebugSet::entry();
      lVar1 = lVar1 + -0xc;
    } while (lVar1 != 0);
  }
  core::fmt::builders::DebugList::finish();
  return;
}

