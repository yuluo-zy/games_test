
void __cdecl tiny_glade::hang_watchdog::hang_watchdog(void)

{
  int iVar1;
  longlong lVar2;
  longlong lVar3;
  undefined *puVar4;
  undefined *puVar5;
  longlong lVar6;
  undefined4 *extraout_RAX;
  undefined4 *extraout_RAX_00;
  ulonglong extraout_RAX_01;
  ulonglong extraout_RAX_02;
  ulonglong extraout_RAX_03;
  ulonglong extraout_RAX_04;
  undefined8 *extraout_RAX_05;
  undefined8 *extraout_RAX_06;
  undefined8 *extraout_RAX_07;
  undefined8 *extraout_RAX_08;
  undefined8 *extraout_RAX_09;
  undefined8 extraout_RAX_10;
  uint uVar7;
  undefined ***pppuVar8;
  ulonglong uVar9;
  int iVar10;
  code *pcVar11;
  longlong lVar12;
  bool bVar13;
  bool bVar14;
  undefined4 uVar15;
  undefined4 uVar16;
  undefined4 uVar17;
  undefined4 uVar18;
  char ***local_1b8;
  code *pcStack_1b0;
  longlong *local_1a8;
  undefined8 local_1a0;
  undefined8 local_198;
  undefined8 local_190;
  undefined1 local_188;
  undefined8 local_180;
  undefined8 local_170;
  undefined8 local_160;
  undefined8 local_158;
  undefined1 local_150;
  longlong local_140;
  char **local_138;
  char **local_130;
  code *local_128;
  code *local_120;
  char ***local_118;
  code *pcStack_110;
  longlong *local_108;
  code *local_100;
  longlong local_f8;
  char **local_f0;
  longlong ***local_e8;
  code *local_e0;
  longlong local_d8;
  undefined **local_d0;
  char *local_c8;
  undefined8 local_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined8 uStack_a8;
  undefined8 local_a0;
  char *local_98;
  undefined8 local_90;
  undefined4 local_88;
  undefined4 local_84;
  undefined **local_80;
  undefined8 local_78;
  char ****local_70;
  undefined8 local_68;
  char ****ppppcStack_60;
  undefined8 local_58;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  pppuVar8 = (undefined ***)0x0;
  thread_priority::ThreadPriority::set_for_current();
  if ((int)local_118 == 4) {
    if (*(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E < 3)
    goto LAB_140365fdb;
    toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
    uVar15 = *extraout_RAX;
    uVar16 = extraout_RAX[1];
    uVar17 = extraout_RAX[2];
    uVar18 = extraout_RAX[3];
    local_84 = extraout_RAX[4];
    local_a0 = 3;
    local_80 = &PTR_s_hang_watchdog_thread_priority_su_1429c5768;
    local_70 = (char ****)&DAT_00000008;
    local_68 = 0;
  }
  else {
    local_1a8 = local_108;
    local_1b8 = local_118;
    pcStack_1b0 = pcStack_110;
    if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E == 0)
    goto LAB_140365fdb;
    local_e8 = (longlong ***)&local_1b8;
    local_e0 = thread_priority::impl$9::fmt;
    toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
    uVar15 = *extraout_RAX_00;
    uVar16 = extraout_RAX_00[1];
    uVar17 = extraout_RAX_00[2];
    uVar18 = extraout_RAX_00[3];
    local_84 = extraout_RAX_00[4];
    local_a0 = 1;
    local_80 = (undefined **)&DAT_1429c5808;
    local_70 = (char ****)&local_e8;
    local_68 = 1;
  }
  ppppcStack_60 = (char ****)0x0;
  local_78 = 1;
  local_90 = 0x19;
  local_98 = "tiny_glade::hang_watchdogError setting priority of hang_watchdog thread: ";
  local_d0 = (undefined **)0x0;
  local_c8 = "tiny_glade::hang_watchdogError setting priority of hang_watchdog thread: ";
  local_c0 = 0x19;
  local_b8 = 0;
  uStack_b0 = CONCAT44(uVar16,uVar15);
  uStack_a8 = CONCAT44(uVar18,uVar17);
  local_88 = 1;
  pppuVar8 = &local_d0;
  ::log::__private_api::impl$2::log();
LAB_140365fdb:
  puVar4 = __imp__ZN12country_core7systems8TICK_IDX17h88d558048ca3865aE;
  iVar10 = *(int *)__imp__ZN12country_core7systems8TICK_IDX17h88d558048ca3865aE;
  std::time::Instant::now();
  std::time::impl$1::add();
  uVar9 = extraout_RAX_01;
  while( true ) {
    uVar7 = 0;
    std::thread::sleep();
    std::time::Instant::now();
    bVar13 = extraout_RAX_02 < uVar9;
    if (extraout_RAX_02 == uVar9) {
      bVar13 = uVar7 < (uint)pppuVar8;
    }
    if (bVar13 == true) {
      do {
        uVar7 = 0;
        std::thread::sleep();
        std::time::Instant::now();
        bVar14 = extraout_RAX_03 < uVar9;
        if (extraout_RAX_03 == uVar9) {
          bVar14 = uVar7 < (uint)pppuVar8;
        }
        bVar13 = false;
      } while (bVar14);
    }
    pppuVar8 = (undefined ***)(ulonglong)bVar13;
    parking_lot_core::parking_lot::deadlock_impl::check_deadlock();
    puVar5 = __imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E;
    local_f8 = local_d8;
    if (local_d8 != 0) break;
    iVar1 = *(int *)puVar4;
    if (iVar1 == iVar10) {
      if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
        local_1b8 = (char ***)&DAT_1429c5958;
        pcStack_1b0 = core::time::impl$12::fmt;
        toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
        uStack_b0 = *extraout_RAX_07;
        uStack_a8 = extraout_RAX_07[1];
        local_84 = *(undefined4 *)(extraout_RAX_07 + 2);
        local_a0 = 1;
        local_98 = "tiny_glade::hang_watchdogError setting priority of hang_watchdog thread: ";
        local_90 = 0x19;
        local_80 = &PTR_s_The_frame_index_has_not_changed_i_1429c5990;
        local_78 = 1;
        local_70 = &local_1b8;
        local_68 = 1;
        ppppcStack_60 = (char ****)0x0;
        local_d0 = (undefined **)0x0;
        local_c8 = "tiny_glade::hang_watchdogError setting priority of hang_watchdog thread: ";
        local_c0 = 0x19;
        local_b8 = 0;
        local_88 = 1;
        ::log::__private_api::impl$2::log();
      }
      if (2 < *(ulonglong *)puVar5) {
        toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
        uStack_b0 = *extraout_RAX_08;
        uStack_a8 = extraout_RAX_08[1];
        local_84 = *(undefined4 *)(extraout_RAX_08 + 2);
        local_a0 = 3;
        local_98 = "tiny_glade::hang_watchdogError setting priority of hang_watchdog thread: ";
        local_90 = 0x19;
        local_80 = &PTR_DAT_1429c5898;
        local_78 = 1;
        local_70 = (char ****)&DAT_00000008;
        local_d0 = (undefined **)0x0;
        local_68 = 0;
        ppppcStack_60 = (char ****)0x0;
        local_c8 = "tiny_glade::hang_watchdogError setting priority of hang_watchdog thread: ";
        local_c0 = 0x19;
        local_b8 = 0;
        local_88 = 1;
        ::log::__private_api::impl$2::log();
      }
      DAT_100000041 = 1;
      pcVar11 = (code *)swi(0x29);
      (*pcVar11)(7);
                    // WARNING: Does not return
      pcVar11 = (code *)invalidInstructionException();
      (*pcVar11)();
    }
    std::time::Instant::now();
    std::time::impl$1::add();
    uVar9 = extraout_RAX_04;
    iVar10 = iVar1;
    if ((char ****)local_e8 != (char ****)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
    toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
    uStack_b0 = *extraout_RAX_09;
    uStack_a8 = extraout_RAX_09[1];
    local_84 = *(undefined4 *)(extraout_RAX_09 + 2);
    local_a0 = 1;
    local_98 = "tiny_glade::hang_watchdogError setting priority of hang_watchdog thread: ";
    local_90 = 0x19;
    local_80 = &PTR_s_Deadlock_detected_1429c5848;
    local_78 = 1;
    local_70 = (char ****)&DAT_00000008;
    local_d0 = (undefined **)0x0;
    local_68 = 0;
    ppppcStack_60 = (char ****)0x0;
    local_c8 = "tiny_glade::hang_watchdogError setting priority of hang_watchdog thread: ";
    local_c0 = 0x19;
    local_b8 = 0;
    local_88 = 1;
    ::log::__private_api::impl$2::log();
  }
  local_120 = local_e0 + local_f8 * 0x18;
  local_f0 = (char **)0x0;
  pcVar11 = local_e0;
  while (pcVar11 != local_120) {
    local_130 = local_f0;
    if (*(longlong *)puVar5 != 0) {
      local_1b8 = &local_130;
      pcStack_1b0 = core::fmt::num::imp::impl$9::fmt;
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      uStack_b0 = *extraout_RAX_05;
      uStack_a8 = extraout_RAX_05[1];
      local_84 = *(undefined4 *)(extraout_RAX_05 + 2);
      local_a0 = 1;
      local_98 = "tiny_glade::hang_watchdogError setting priority of hang_watchdog thread: ";
      local_90 = 0x19;
      local_80 = &PTR_DAT_1429c58d0;
      local_78 = 2;
      local_68 = 1;
      ppppcStack_60 = (char ****)0x0;
      local_d0 = (undefined **)0x0;
      local_c8 = "tiny_glade::hang_watchdogError setting priority of hang_watchdog thread: ";
      local_c0 = 0x19;
      local_b8 = 0;
      local_88 = 1;
      local_70 = &local_1b8;
      ::log::__private_api::impl$2::log();
    }
    local_128 = pcVar11 + 0x18;
    local_f0 = (char **)((longlong)local_f0 + 1);
    lVar2 = *(longlong *)(pcVar11 + 8);
    lVar3 = *(longlong *)(pcVar11 + 0x10);
    lVar6 = lVar2;
    while (lVar12 = lVar6, pcVar11 = local_128, lVar12 != lVar3 * 0x20 + lVar2) {
      lVar6 = lVar12 + 0x20;
      if (*(longlong *)puVar5 != 0) {
        local_138 = *(char ***)(lVar12 + 0x18);
        local_118 = &local_138;
        pcStack_110 = core::fmt::num::imp::impl$9::fmt;
        local_108 = &local_140;
        local_100 = core::fmt::impl$74::fmt<backtrace::capture::Backtrace>;
        local_1b8 = (char ***)0x2;
        local_1a8 = (longlong *)0x2;
        local_198 = 0;
        local_190 = 0x20;
        local_188 = 3;
        local_180 = 2;
        local_170 = 2;
        local_160 = 1;
        local_158 = 0x400000020;
        local_150 = 3;
        local_140 = lVar12;
        toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
        uStack_b0 = *extraout_RAX_06;
        uStack_a8 = extraout_RAX_06[1];
        local_84 = *(undefined4 *)(extraout_RAX_06 + 2);
        local_a0 = 1;
        local_98 = "tiny_glade::hang_watchdogError setting priority of hang_watchdog thread: ";
        local_90 = 0x19;
        local_80 = &PTR_s_Thread_backtrace__1429c5920;
        local_78 = 2;
        local_70 = &local_118;
        local_68 = 2;
        local_58 = 2;
        local_d0 = (undefined **)0x0;
        local_c8 = "tiny_glade::hang_watchdogError setting priority of hang_watchdog thread: ";
        local_c0 = 0x19;
        local_b8 = 0;
        local_88 = 1;
        ppppcStack_60 = &local_1b8;
        ::log::__private_api::impl$2::log();
      }
    }
  }
  if (2 < *(ulonglong *)puVar5) {
    local_d0 = &PTR_DAT_1429c5898;
    local_c8 = (char *)0x1;
    local_c0 = 8;
    local_b8 = 0;
    uStack_b0 = 0;
    toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
    local_1b8 = (char ***)0x1429c57b8;
    pcStack_1b0 = (code *)0x19;
    local_1a8 = (longlong *)0x1429c57b8;
    local_1a0 = 0x19;
    local_198 = extraout_RAX_10;
    ::log::__private_api::log<tuple_<>,log::__private_api::GlobalLogger>();
  }
  DAT_100000041 = 1;
  pcVar11 = (code *)swi(0x29);
  (*pcVar11)(7);
                    // WARNING: Does not return
  pcVar11 = (code *)invalidInstructionException();
  (*pcVar11)();
}

