// 0x14032e0b0 image<ref$<str$>_>
// signature: void __cdecl image<ref$<str$>_>(void)

void __cdecl toolbar::mei::widgets::impl$26::image<ref_<str_>_>(void)

{
  undefined8 uVar1;
  undefined8 extraout_RAX;
  size_t *extraout_RAX_00;
  void *extraout_RAX_01;
  undefined8 *in_RCX;
  longlong in_RDX;
  void *_Dst;
  void *in_stack_00000028;
  size_t in_stack_00000030;
  undefined4 in_stack_00000038;
  
  core::impl$0::as_id();
  minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
  ::utils::calculate_hash<tuple_<u64,core::any::TypeId>_>();
  uVar1 = *(undefined8 *)(in_RDX + 8);
  ::utils::calculate_hash<tuple_<u64,u64>_>();
  core::NodeContext::child_with_global_id();
  core::NodeContext::get_content_mut_impl<toolbar::mei::widgets::ImageContent>();
  if ((longlong)in_stack_00000030 < 0) {
LAB_14032e1f8:
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  if (in_stack_00000030 == 0) {
    _Dst = (void *)0x1;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    _Dst = extraout_RAX_01;
    if (extraout_RAX_01 == (void *)0x0) goto LAB_14032e1f8;
  }
  memcpy(_Dst,in_stack_00000028,in_stack_00000030);
  if (*extraout_RAX_00 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  *extraout_RAX_00 = in_stack_00000030;
  extraout_RAX_00[1] = (size_t)_Dst;
  extraout_RAX_00[2] = in_stack_00000030;
  *(undefined4 *)(extraout_RAX_00 + 3) = in_stack_00000038;
  *(undefined1 *)((longlong)extraout_RAX_00 + 0x1c) = 0;
  in_RCX[2] = 0x1f6d60c8858b8ff;
  *in_RCX = uVar1;
  in_RCX[1] = extraout_RAX;
  return;
}

