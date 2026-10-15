// 0x1404fe550 into_slot<ref$<str$>_>
// signature: void __cdecl into_slot<ref$<str$>_>(void)

void __cdecl toolbar::mei::config::TemplateView::into_slot<ref_<str_>_>(void)

{
  undefined8 uVar1;
  char extraout_AL;
  undefined8 extraout_RAX;
  char *extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  longlong in_RDX;
  undefined8 in_R8;
  undefined8 in_R9;
  undefined8 local_108;
  code *pcStack_100;
  undefined8 *local_f8;
  code *local_f0;
  undefined8 local_e8;
  char *pcStack_e0;
  undefined8 local_d8;
  undefined8 local_d0;
  undefined8 local_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
  char *local_b0;
  undefined8 local_a8;
  undefined4 local_a0;
  undefined4 local_9c;
  undefined **local_98;
  undefined8 local_90;
  undefined8 *local_88;
  undefined8 local_80;
  undefined8 local_78;
  undefined8 local_60;
  undefined8 uStack_58;
  undefined8 local_50;
  undefined8 uStack_48;
  
  local_60 = in_R8;
  uStack_58 = in_R9;
  core::NodeContext::id();
  core::impl$0::as_id();
  minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
  pcStack_e0 = (char *)0x41223169ff28813b;
  local_d8 = 0xa79b7268a2a968d9;
  ::utils::calculate_hash<tuple_<u64,core::any::TypeId>_>();
  local_e8 = extraout_RAX;
  pcStack_e0 = extraout_RAX_00;
  ::utils::calculate_hash<tuple_<u64,u64>_>();
  core::NodeContext::has_global_id();
  if (extraout_AL == '\0') {
    if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
      pcStack_100 = ::core::fmt::impl$74::fmt<str_>;
      local_f0 = ::core::fmt::impl$74::fmt<str_>;
      local_f8 = &local_60;
      widgets::tweaker::TweakerView::as_inner_mut();
      local_c8 = *extraout_RAX_01;
      uStack_c0 = extraout_RAX_01[1];
      local_9c = *(undefined4 *)(extraout_RAX_01 + 2);
      local_b8 = 1;
      local_b0 = "toolbar::mei::config";
      local_a8 = 0x14;
      local_98 = &anon_5b758b46761ef39294929c3d11ba302d_4_llvm_14359997359250620829;
      local_90 = 2;
      local_80 = 2;
      local_78 = 0;
      local_e8 = 0;
      pcStack_e0 = "toolbar::mei::config";
      local_d8 = 0x14;
      local_d0 = 0;
      local_a0 = 1;
      local_88 = &local_108;
      ::log::__private_api::impl$2::log();
    }
    local_108 = *(undefined8 *)(in_RDX + 0x10);
    uVar1 = *(undefined8 *)(in_RDX + 0x18);
    local_f8 = *(undefined8 **)(in_RDX + 0x20);
    local_50 = local_60;
    uStack_48 = uStack_58;
    pcStack_100 = (code *)uVar1;
    core::impl$0::as_id();
    pcStack_e0 = (char *)0x41223169ff28813b;
    local_d8 = 0xa79b7268a2a968d9;
    ::utils::calculate_hash<tuple_<u64,core::any::TypeId>_>();
    local_e8 = uVar1;
    ::utils::calculate_hash<tuple_<u64,u64>_>();
    core::NodeContext::into_child_with_global_id();
  }
  else {
    local_d8 = *(undefined8 *)(in_RDX + 0x20);
    local_e8 = *(undefined8 *)(in_RDX + 0x10);
    pcStack_e0 = *(char **)(in_RDX + 0x18);
    core::NodeContext::expect_child_with_global_id();
  }
  return;
}

