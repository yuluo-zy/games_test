
void __cdecl
alloc::vec::spec_from_iter::impl$0::
from_iter<f32,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3>_>,system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure_env_1>_>
          (void)

{
  longlong extraout_RAX;
  longlong lVar1;
  longlong *in_RCX;
  longlong *in_RDX;
  longlong lVar2;
  
  if (in_RDX[1] - *in_RDX == 0) {
    lVar2 = 0;
    lVar1 = 4;
  }
  else {
    lVar2 = ((ulonglong)(in_RDX[1] - *in_RDX) >> 2) * -0x5555555555555555;
    std::alloc::__default_lib_allocator::__rust_alloc();
    lVar1 = extraout_RAX;
    if (extraout_RAX == 0) {
                    /* WARNING: Subroutine does not return */
      raw_vec::handle_error();
    }
  }
  core::iter::adapters::map::impl$2::
  fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3>_>,system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure_env_1,tuple_<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env_0<f32,alloc::vec::impl_20::extend_trusted::closure_env_0<f32,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3>_>,system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure_env_1>_>_>_>
            ();
  in_RCX[2] = 0;
  *in_RCX = lVar2;
  in_RCX[1] = lVar1;
  return;
}

