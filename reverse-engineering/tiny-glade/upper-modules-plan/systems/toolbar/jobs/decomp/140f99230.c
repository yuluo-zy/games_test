// 0x140f99230 safe_frame<ref$<str$>_>
// signature: void __cdecl safe_frame<ref$<str$>_>(void)

void __cdecl toolbar::mei::core::MeiContext::safe_frame<ref_<str_>_>(void)

{
  longlong lVar1;
  longlong lVar2;
  ulonglong extraout_RAX;
  longlong extraout_RAX_00;
  undefined8 *in_RCX;
  longlong *in_RDX;
  ulonglong uVar3;
  ulonglong uVar4;
  char in_stack_00000028;
  
  ::utils::calculate_hash<tuple_<ref_<str_>,ref_<tuple_<>_>_>_>();
  uVar4 = extraout_RAX ^ 0xb1695320004fb443;
  uVar3 = uVar4;
  reuse_or_create_node();
  *(undefined1 *)(extraout_RAX_00 + 0x5d) = 0;
  if (in_stack_00000028 != '\0') {
    *(undefined1 *)(extraout_RAX_00 + 0xb7) = 1;
  }
  if ((uVar3 & 1) == 0) {
    lVar1 = in_RDX[2];
    if (lVar1 == *in_RDX) {
      alloc::raw_vec::RawVec::grow_one<enum2_<iced_core::event::Event>,alloc::alloc::Global>();
    }
    lVar2 = in_RDX[1];
    *(undefined4 *)(lVar2 + lVar1 * 0x38) = 1;
    *(ulonglong *)(lVar2 + 0x30 + lVar1 * 0x38) = uVar4;
    in_RDX[2] = lVar1 + 1;
  }
  *in_RCX = in_RDX;
  in_RCX[1] = uVar4;
  in_RCX[2] = 0;
  return;
}

