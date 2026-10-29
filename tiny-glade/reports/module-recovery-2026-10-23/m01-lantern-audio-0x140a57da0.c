
void __cdecl country_core::resources::audio_resources::impl$10::set_position(void)

{
  char cVar1;
  longlong lVar2;
  char *pcVar3;
  undefined ***pppuVar4;
  char extraout_AL;
  char extraout_AL_00;
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  int *in_RCX;
  undefined8 *in_RDX;
  longlong local_1b8;
  undefined **local_180;
  void *pvStack_178;
  ulonglong local_170;
  undefined1 local_168 [64];
  undefined **local_128;
  code *pcStack_120;
  undefined1 *local_118;
  code *local_110;
  int ***local_108;
  undefined4 uStack_100;
  undefined4 uStack_fc;
  undefined8 uStack_f8;
  undefined4 uStack_f0;
  undefined4 uStack_ec;
  undefined4 uStack_e8;
  undefined4 uStack_e4;
  undefined4 uStack_e0;
  undefined4 local_dc;
  undefined8 local_d8;
  undefined *local_d0;
  undefined8 local_c8;
  undefined8 local_c0;
  undefined *local_b8;
  undefined8 local_b0;
  int ***local_a8;
  undefined8 local_a0;
  undefined8 local_98;
  char *local_80;
  undefined ***local_78;
  code *pcStack_70;
  undefined ***local_68;
  undefined8 local_60;
  undefined ****local_58;
  undefined8 local_50;
  char local_39;
  undefined8 local_38;
  
  local_38 = 0xfffffffffffffffe;
  if (*in_RCX == 1) {
    uStack_100 = *(undefined4 *)(in_RDX + 1);
    local_108 = (int ***)*in_RDX;
    uStack_fc = 0;
    uStack_f8._0_4_ = 0;
    uStack_f8._4_4_ = 0;
    uStack_f0 = 0x80000000;
    uStack_ec = 0x80000000;
    uStack_e8 = 0xbf800000;
    uStack_e4 = 0;
    uStack_e0 = 0x3f800000;
    local_dc = 0;
    libfmod::EventInstance::set_3d_attributes();
    if (local_1b8 != -0x7ffffffffffffffc) {
      local_128 = (undefined **)(in_RCX + 4);
      pcStack_120 = alloc::string::impl$24::fmt;
      local_110 = libfmod::impl$341::fmt;
      local_108 = (int ***)0x2;
      uStack_100 = 2;
      uStack_fc = 0;
      uStack_f8._0_4_ = 2;
      uStack_f8._4_4_ = 0;
      uStack_e8 = 0;
      uStack_e4 = 0;
      uStack_e0 = 0x20;
      local_dc = 0;
      local_d8 = CONCAT71(local_d8._1_7_,3);
      local_d0 = (undefined *)0x2;
      local_c8 = 2;
      local_c0 = 2;
      local_b0 = 1;
      local_a8 = (int ***)0x400000020;
      local_a0 = CONCAT71(local_a0._1_7_,3);
      local_78 = (undefined ***)&PTR_s_Failed_to_set_3d_attributes_for_F_142ad2d40;
      pcStack_70 = (code *)0x2;
      local_60 = 2;
      local_50 = 2;
      local_118 = local_168;
      local_68 = &local_128;
      local_58 = (undefined ****)&local_108;
      alloc::fmt::format::format_inner();
      local_128 = local_180;
      pcStack_120 = pvStack_178;
      local_118 = (undefined1 *)local_170;
      if (DAT_143634d80 != 3) {
        local_78 = (undefined ***)CONCAT71(local_78._1_7_,1);
        local_108 = (int ***)&local_78;
        std::sys::sync::once::futex::Once::call();
      }
      std::sync::poison::mutex::Mutex::lock<winit::dpi::PhysicalSize<u32>_>();
      if ((int)local_108 == 1) {
        local_78 = (undefined ***)CONCAT44(uStack_fc,uStack_100);
        pcStack_70 = (code *)CONCAT71(pcStack_70._1_7_,(char)uStack_f8);
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
      local_80 = (char *)CONCAT44(uStack_fc,uStack_100);
      local_39 = (char)uStack_f8;
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
        alloc::raw_vec::handle_error();
      }
      *(undefined8 *)((longlong)extraout_RAX + 7) = 0x2928202168746170;
      *extraout_RAX = 0x705f656c75646f6d;
      local_108 = (int ***)&DAT_0000000f;
      uStack_100 = SUB84(extraout_RAX,0);
      uStack_fc = (undefined4)((ulonglong)extraout_RAX >> 0x20);
      uStack_f8._0_4_ = 0xf;
      uStack_f8._4_4_ = 0;
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      lVar2 = CONCAT44(uStack_f8._4_4_,(undefined4)uStack_f8);
      builtin_strncpy((char *)(CONCAT44(uStack_fc,uStack_100) + lVar2),"$crate :: Level ",0x10);
      *(undefined8 *)(CONCAT44(uStack_fc,uStack_100) + 0xf + lVar2) = 0x6e726157203a3a20;
      local_68 = (undefined ***)(lVar2 + 0x17);
      pcStack_70 = (code *)CONCAT44(uStack_fc,uStack_100);
      local_78 = (undefined ***)local_108;
      if ((ulonglong)((longlong)local_108 - (longlong)local_68) < local_118) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      pppuVar4 = local_68;
      memcpy(pcStack_70 + (longlong)local_68,pcStack_120,(size_t)local_118);
      local_68 = (undefined ***)((longlong)pppuVar4 + (longlong)local_118);
      local_108 = (int ***)local_78;
      uStack_100 = SUB84(pcStack_70,0);
      uStack_fc = (undefined4)((ulonglong)pcStack_70 >> 0x20);
      uStack_f8 = (longlong)local_68;
      alloc::collections::btree::map::BTreeMap::
      insert<alloc::string::String,alloc::collections::btree::set_val::SetValZST,alloc::alloc::Global>
                ();
      if ((extraout_AL == '\0') &&
         (1 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E)) {
        pcStack_70 = alloc::string::impl$23::fmt;
        local_78 = &local_128;
        ::toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
        local_d8 = 2;
        local_d0 = &DAT_142ad2550;
        local_c8 = 0x28;
        local_b8 = &DAT_142ad2a78;
        local_b0 = 1;
        local_a0 = 1;
        local_98 = 0;
        local_108 = (int ***)0x0;
        uStack_100 = 0x42ad2550;
        uStack_fc = 1;
        uStack_f8._0_4_ = 0x28;
        uStack_f8._4_4_ = 0;
        uStack_f0 = 0;
        uStack_ec = 0;
        uStack_e8 = (undefined4)*extraout_RAX_00;
        uStack_e4 = (undefined4)((ulonglong)*extraout_RAX_00 >> 0x20);
        uStack_e0 = (undefined4)extraout_RAX_00[1];
        local_dc = (undefined4)((ulonglong)extraout_RAX_00[1] >> 0x20);
        local_c0 = CONCAT44(*(undefined4 *)(extraout_RAX_00 + 2),1);
        local_a8 = (int ***)&local_78;
        ::log::__private_api::impl$2::log();
      }
      pcVar3 = local_80;
      if ((local_39 == '\0') &&
         ((*(ulonglong *)
            __imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E &
          0x7fffffffffffffff) != 0)) {
        std::panicking::panic_count::is_zero_slow_path();
        if (extraout_AL_00 == '\0') {
          pcVar3[1] = '\x01';
        }
      }
      LOCK();
      cVar1 = *pcVar3;
      *pcVar3 = '\0';
      UNLOCK();
      if (cVar1 == '\x02') {
        std::sys::sync::mutex::futex::Mutex::wake();
      }
      if ((int **)local_128 != (int **)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      core::ptr::drop_in_place<enum2_<libfmod::Error>_>();
    }
  }
  return;
}


