// 0x140683a20 load_font_table
// signature: void __cdecl load_font_table(void)

void __cdecl toolbar::mei::core::MeiContext::load_font_table(void)

{
  longlong *plVar1;
  undefined8 *extraout_RAX;
  longlong in_RCX;
  char local_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  
  config::load_font_table();
  if (local_e0 != '2') {
                    // WARNING: Subroutine does not return
    ::core::result::unwrap_failed();
  }
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX != (undefined8 *)0x0) {
    extraout_RAX[6] = local_b8;
    extraout_RAX[7] = uStack_b0;
    extraout_RAX[4] = uStack_c8;
    extraout_RAX[5] = uStack_c0;
    extraout_RAX[2] = uStack_d8;
    extraout_RAX[3] = uStack_d0;
    *extraout_RAX = 1;
    extraout_RAX[1] = 1;
    plVar1 = *(longlong **)(in_RCX + 0x260);
    LOCK();
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if (*plVar1 == 0) {
      alloc::sync::Arc::drop_slow<tg_text::font_table::FontTable,alloc::alloc::Global>();
    }
    *(undefined8 **)(in_RCX + 0x260) = extraout_RAX;
    return;
  }
                    // WARNING: Subroutine does not return
  alloc::alloc::handle_alloc_error();
}

