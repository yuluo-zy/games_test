
// WARNING: Removing unreachable block (ram,0x0001412b4cdd)
// WARNING: Removing unreachable block (ram,0x0001412b4cf2)

void __cdecl
country_core::resources::walls::decorator_archivist::DecoratorArchivist::
archive_current_decorators_in_storage<ref_<str_>_>(void)

{
  code *pcVar1;
  longlong lVar2;
  longlong extraout_RAX;
  void *extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong *extraout_RAX_03;
  void *_Dst;
  void *in_stack_00000030;
  size_t in_stack_00000038;
  size_t *local_f8;
  code *pcStack_f0;
  undefined8 local_e8;
  undefined8 uStack_e0;
  longlong local_d8;
  longlong lStack_d0;
  undefined *local_c8;
  longlong lStack_c0;
  undefined8 **local_b8;
  undefined8 uStack_b0;
  longlong local_a8;
  longlong lStack_a0;
  size_t local_90;
  void *local_88;
  undefined8 local_70;
  
  local_70 = 0xfffffffffffffffe;
  decorator_storage::DecoratorStorage::get();
  if (extraout_RAX == 0) {
    if ((longlong)in_stack_00000038 < 0) {
LAB_1412b4d47:
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    if (in_stack_00000038 == 0) {
      _Dst = (void *)0x1;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      _Dst = extraout_RAX_00;
      if (extraout_RAX_00 == (void *)0x0) goto LAB_1412b4d47;
    }
    memcpy(_Dst,in_stack_00000030,in_stack_00000038);
    local_90 = in_stack_00000038;
    local_88 = _Dst;
    core::ops::function::FnOnce::
    call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
              ();
    if (extraout_RAX_01 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    local_d8 = *extraout_RAX_01;
    lStack_d0 = extraout_RAX_01[1];
    *extraout_RAX_01 = *extraout_RAX_01 + 1;
    local_e8 = 0;
    uStack_e0 = 0;
    local_f8 = (size_t *)&anon_bf4ed97acacab9c6e71765005891de1b_1_llvm_1767052282543269069;
    pcStack_f0 = (code *)0x0;
    core::ops::function::FnOnce::
    call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
              ();
    if (extraout_RAX_02 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    local_a8 = *extraout_RAX_02;
    lStack_a0 = extraout_RAX_02[1];
    *extraout_RAX_02 = *extraout_RAX_02 + 1;
    local_c8 = &anon_335873a3c4aec110b80141a54edc665a_1_llvm_11115642958688891036;
    lStack_c0 = 0;
    local_b8 = (undefined8 **)0x0;
    uStack_b0 = 0;
    core::ops::function::FnOnce::
    call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
              ();
    lVar2 = lStack_c0;
    pcVar1 = pcStack_f0;
    if (extraout_RAX_03 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    *extraout_RAX_03 = *extraout_RAX_03 + 1;
    local_f8 = &local_90;
    pcStack_f0 = alloc::string::impl$23::fmt;
    local_c8 = &anon_41f28c8b0f8809a1d72a2d55317e3f61_6_llvm_146935197314726508;
    lStack_c0 = 2;
    local_b8 = &local_f8;
    uStack_b0 = 1;
    local_a8 = 0;
    alloc::fmt::format::format_inner();
    archive_storage_decorators<alloc::string::String>();
    if ((pcVar1 != (code *)0x0) && ((longlong)pcVar1 * 0x51 != -0x61)) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if ((lVar2 != 0) && (lVar2 + (lVar2 * 4 + 0x13U & 0xfffffffffffffff0) != -0x11)) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if (local_90 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  else {
    archive_storage_decorators<ref_<str_>_>();
  }
  return;
}

