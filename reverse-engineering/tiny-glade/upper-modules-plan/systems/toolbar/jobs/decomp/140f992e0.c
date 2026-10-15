// 0x140f992e0 new_at_pos<ref$<str$>_>
// signature: void __cdecl new_at_pos<ref$<str$>_>(void)

void __cdecl toolbar::mei::core::MeiContext::new_at_pos<ref_<str_>_>(void)

{
  longlong lVar1;
  longlong lVar2;
  ulonglong extraout_RAX;
  longlong extraout_RAX_00;
  undefined8 *in_RCX;
  longlong lVar3;
  longlong *in_RDX;
  ulonglong uVar4;
  ulonglong uVar5;
  undefined4 in_stack_00000028;
  undefined4 in_stack_00000030;
  undefined8 *in_stack_00000038;
  undefined4 in_stack_00000040;
  undefined4 in_stack_00000048;
  char in_stack_00000050;
  
  ::utils::calculate_hash<tuple_<ref_<str_>,ref_<tuple_<>_>_>_>();
  uVar5 = extraout_RAX ^ 0xb1695320004fb443;
  uVar4 = uVar5;
  reuse_or_create_node();
  *(undefined1 *)(extraout_RAX_00 + 0x5d) = 0;
  if (in_stack_00000050 != '\0') {
    *(undefined1 *)(extraout_RAX_00 + 0xb7) = 1;
  }
  if ((uVar4 & 1) == 0) {
    lVar1 = in_RDX[2];
    if (lVar1 == *in_RDX) {
      alloc::raw_vec::RawVec::grow_one<enum2_<iced_core::event::Event>,alloc::alloc::Global>();
    }
    lVar2 = in_RDX[1];
    lVar3 = lVar1 * 0x38;
    *(undefined4 *)(lVar2 + lVar3) = 4;
    *(undefined4 *)(lVar2 + 4 + lVar3) = in_stack_00000028;
    *(undefined4 *)(lVar2 + 8 + lVar3) = in_stack_00000030;
    *(undefined4 *)(lVar2 + 0xc + lVar3) = in_stack_00000040;
    *(undefined4 *)(lVar2 + 0x10 + lVar3) = in_stack_00000048;
    *(undefined8 *)(lVar2 + 0x14 + lVar3) = *in_stack_00000038;
    *(undefined4 *)(lVar2 + 0x1c + lVar3) = *(undefined4 *)(in_stack_00000038 + 1);
    *(ulonglong *)(lVar2 + 0x30 + lVar3) = uVar5;
    in_RDX[2] = lVar1 + 1;
  }
  *in_RCX = in_RDX;
  in_RCX[1] = uVar5;
  in_RCX[2] = 0;
  return;
}

