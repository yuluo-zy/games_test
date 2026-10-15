// 0x1404fe340 slot<ref$<str$>_>
// signature: void __cdecl slot<ref$<str$>_>(void)

void __cdecl toolbar::mei::config::TemplateView::slot<ref_<str_>_>(void)

{
  undefined8 uVar1;
  char extraout_AL;
  undefined8 extraout_RAX;
  char *extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  longlong in_RDX;
  undefined8 in_R8;
  undefined8 in_R9;
  undefined8 local_f0;
  undefined8 uStack_e8;
  undefined8 local_e0;
  code *pcStack_d8;
  undefined8 *local_d0;
  code *local_c8;
  undefined8 local_c0;
  char *local_b8;
  undefined8 local_b0;
  undefined8 local_a8;
  undefined8 local_a0;
  undefined8 uStack_98;
  undefined8 local_90;
  char *local_88;
  undefined8 local_80;
  undefined4 local_78;
  undefined4 local_74;
  undefined **local_70;
  undefined8 local_68;
  undefined8 *local_60;
  undefined8 local_58;
  undefined8 local_50;
  
  local_f0 = in_R8;
  uStack_e8 = in_R9;
  core::NodeContext::id();
  core::impl$0::as_id();
  minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
  local_b8 = (char *)0x41223169ff28813b;
  local_b0 = 0xa79b7268a2a968d9;
  ::utils::calculate_hash<tuple_<u64,core::any::TypeId>_>();
  local_c0 = extraout_RAX;
  local_b8 = extraout_RAX_00;
  ::utils::calculate_hash<tuple_<u64,u64>_>();
  core::NodeContext::has_global_id();
  if (extraout_AL == '\0') {
    if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
      pcStack_d8 = ::core::fmt::impl$74::fmt<str_>;
      local_c8 = ::core::fmt::impl$74::fmt<str_>;
      local_d0 = &local_f0;
      widgets::tweaker::TweakerView::as_inner_mut();
      local_a0 = *extraout_RAX_01;
      uStack_98 = extraout_RAX_01[1];
      local_74 = *(undefined4 *)(extraout_RAX_01 + 2);
      local_90 = 1;
      local_88 = "toolbar::mei::config";
      local_80 = 0x14;
      local_70 = &anon_5b758b46761ef39294929c3d11ba302d_4_llvm_14359997359250620829;
      local_68 = 2;
      local_58 = 2;
      local_50 = 0;
      local_c0 = 0;
      local_b8 = "toolbar::mei::config";
      local_b0 = 0x14;
      local_a8 = 0;
      local_78 = 1;
      local_60 = &local_e0;
      ::log::__private_api::impl$2::log();
    }
    uVar1 = *(undefined8 *)(in_RDX + 0x18);
    local_e0 = local_f0;
    pcStack_d8 = (code *)uStack_e8;
    core::impl$0::as_id();
    local_b8 = (char *)0x41223169ff28813b;
    local_b0 = 0xa79b7268a2a968d9;
    ::utils::calculate_hash<tuple_<u64,core::any::TypeId>_>();
    local_c0 = uVar1;
    ::utils::calculate_hash<tuple_<u64,u64>_>();
    core::NodeContext::child_with_global_id();
  }
  else {
    core::NodeContext::expect_child_with_global_id();
  }
  return;
}

