// log_time_to_first_frame
// sig: void __cdecl log_time_to_first_frame(void)
// size: 312

void __cdecl tiny_glade::log_time_to_first_frame(void)

{
  int iVar1;
  undefined8 *extraout_RAX;
  undefined1 *local_a0;
  code *local_98;
  undefined1 local_90 [16];
  undefined8 local_80;
  char *local_78;
  undefined8 local_70;
  undefined8 local_68;
  undefined8 local_60;
  undefined8 uStack_58;
  undefined8 local_50;
  char *local_48;
  undefined8 local_40;
  undefined4 local_38;
  undefined4 local_34;
  undefined **local_30;
  undefined8 local_28;
  undefined1 **local_20;
  undefined8 local_18;
  undefined8 local_10;
  
  iVar1 = STARTUP_INSTANT;
  STARTUP_INSTANT = 1000000000;
  if (iVar1 != 1000000000) {
    if (2 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
      std::time::Instant::elapsed();
      local_a0 = local_90;
      local_98 = core::time::impl$12::fmt;
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      local_60 = *extraout_RAX;
      uStack_58 = extraout_RAX[1];
      local_34 = *(undefined4 *)(extraout_RAX + 2);
      local_50 = 3;
      local_48 = "tiny_glade";
      local_40 = 10;
      local_30 = &anon_255560f7730ce50369b85a6e02f1f18a_43_llvm_5072382169254303063;
      local_28 = 1;
      local_20 = &local_a0;
      local_18 = 1;
      local_10 = 0;
      local_80 = 0;
      local_78 = "tiny_glade";
      local_70 = 10;
      local_68 = 0;
      local_38 = 1;
      ::log::__private_api::impl$2::log();
    }
  }
  return;
}

