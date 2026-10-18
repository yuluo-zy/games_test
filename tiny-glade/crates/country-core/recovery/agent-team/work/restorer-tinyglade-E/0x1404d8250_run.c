
void __cdecl tiny_glade::game_loop::GameLoop::run(void)

{
  longlong *****ppppplVar1;
  longlong ******pppppplVar2;
  longlong *plVar3;
  code *pcVar4;
  uint6 uVar5;
  longlong ******pppppplVar6;
  undefined1 *_Src;
  longlong *******ppppppplVar7;
  longlong ******pppppplVar8;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  uint extraout_EAX;
  longlong *******extraout_RAX;
  longlong ******_Dst;
  longlong ******extraout_RAX_00;
  longlong *****_Dst_00;
  longlong extraout_RAX_01;
  longlong ******extraout_RAX_02;
  longlong ******extraout_RAX_03;
  longlong extraout_RAX_04;
  longlong extraout_RAX_05;
  undefined8 *extraout_RAX_06;
  longlong extraout_RAX_07;
  ulonglong *extraout_RAX_08;
  longlong in_RCX;
  longlong lVar9;
  ulonglong uVar10;
  longlong ****pppplVar11;
  longlong ******unaff_RBX;
  longlong *****unaff_RDI;
  char cVar12;
  uint uVar13;
  longlong ******in_R9;
  longlong ******unaff_R12;
  longlong *****ppppplVar14;
  longlong *****ppppplVar15;
  bool bVar16;
  longlong ******local_178;
  longlong *****ppppplStack_170;
  longlong ****local_168;
  longlong *****ppppplStack_160;
  longlong ****local_158;
  longlong ***ppplStack_150;
  undefined1 *local_148;
  char *local_140;
  longlong *****ppppplStack_138;
  undefined8 local_130;
  longlong *****local_128;
  longlong ****local_120;
  longlong ******local_118;
  undefined8 local_110;
  undefined8 local_108;
  longlong ******local_f0;
  longlong *****ppppplStack_e8;
  longlong ****local_e0;
  longlong *****ppppplStack_d8;
  longlong ****local_d0;
  longlong *local_c8;
  longlong ***local_c0;
  undefined1 *puStack_b8;
  byte local_b0;
  undefined7 uStack_af;
  longlong *****ppppplStack_a8;
  uint local_a0;
  ushort uStack_9c;
  undefined2 uStack_9a;
  longlong *local_98;
  longlong *****local_90;
  longlong *****local_80;
  longlong ****local_78;
  undefined8 local_70;
  longlong *****local_68;
  undefined1 local_5b;
  undefined1 local_5a;
  undefined1 local_59;
  longlong local_58;
  undefined1 local_4c;
  undefined1 local_4b;
  undefined1 local_4a;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  local_4c = 1;
  local_58 = in_RCX;
  main_loop();
  local_178 = (longlong ******)extraout_RAX;
  if (extraout_RAX == (longlong *******)0x2) {
    local_4c = 1;
    anyhow::error::impl$6::drop();
  }
  local_c8 = *(longlong **)(local_58 + 0x10e8);
  local_c0 = (longlong ***)0x0;
  ppppplVar14 = (longlong *****)local_c8[3];
  pppppplVar2 = (longlong ******)local_c8[2];
  LOCK();
  ppppplVar1 = *pppppplVar2;
  *pppppplVar2 = (longlong *****)((longlong)*pppppplVar2 + 1);
  UNLOCK();
  if (*pppppplVar2 == (longlong *****)0x0 ||
      SCARRY8((longlong)ppppplVar1,1) != (longlong)*pppppplVar2 < 0) {
                    // WARNING: Does not return
    pcVar4 = (code *)invalidInstructionException();
    (*pcVar4)();
  }
  local_68 = (longlong *****)(pppppplVar2 + 2);
  LOCK();
  bVar16 = *(char *)(pppppplVar2 + 2) == '\0';
  if (bVar16) {
    *(char *)(pppppplVar2 + 2) = '\x01';
  }
  UNLOCK();
  local_80 = (longlong *****)pppppplVar2;
  if (!bVar16) {
    local_49 = 1;
    std::sys::sync::mutex::futex::Mutex::lock_contended();
  }
  if ((*(ulonglong *)__imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E &
      0x7fffffffffffffff) == 0) {
    uVar13 = 0;
    cVar12 = *(char *)((longlong)pppppplVar2 + 0x11);
    pppppplVar8 = in_R9;
    ppppppplVar7 = (longlong *******)local_178;
    pppppplVar6 = (longlong ******)ppppplStack_d8;
    local_178 = (longlong ******)local_68;
  }
  else {
    local_49 = 1;
    std::panicking::panic_count::is_zero_slow_path();
    uVar13 = extraout_EAX ^ 1;
    cVar12 = *(char *)((longlong)pppppplVar2 + 0x11);
    pppppplVar8 = in_R9;
    ppppppplVar7 = (longlong *******)local_178;
    pppppplVar6 = (longlong ******)ppppplStack_d8;
    local_178 = (longlong ******)local_68;
  }
  ppppplStack_d8 = (longlong *****)pppppplVar8;
  local_68 = (longlong *****)local_178;
  if (cVar12 != '\0') {
    uVar10 = (ulonglong)ppppplStack_170 >> 8;
    ppppplStack_170 = (longlong *****)CONCAT71((int7)uVar10,(char)uVar13);
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  local_70 = (longlong *******)CONCAT44(local_70._4_4_,uVar13);
  if (*(int *)(pppppplVar2 + 7) == 1) {
    ppppplVar1 = pppppplVar2[8];
    ppppplVar15 = pppppplVar2[9];
    local_78 = (longlong ****)ppppplVar14;
    if (ppppplVar1 == (longlong *****)0x0) {
      if (((ulonglong)ppppplVar15 & 1) == 0) {
        ppppplVar15 = (longlong *****)0x0;
        unaff_RBX = (longlong ******)0x0;
      }
      else {
        ppppplStack_d8 = pppppplVar2[10];
        ppppplVar15 = (longlong *****)0x1;
        unaff_RBX = (longlong ******)0x0;
      }
    }
    else {
      uVar5 = *(uint6 *)(pppppplVar2 + 0xb);
      unaff_R12 = (longlong ******)(ulonglong)*(uint *)(pppppplVar2 + 0xb);
      local_90 = pppppplVar2[10];
      std::alloc::__default_lib_allocator::__rust_alloc();
      local_178 = (longlong ******)ppppppplVar7;
      if (_Dst == (longlong ******)0x0) {
        local_59 = 1;
        local_5a = 1;
                    // WARNING: Subroutine does not return
        alloc::alloc::handle_alloc_error();
      }
      memcpy(_Dst,ppppplVar1,0xdc);
      unaff_RDI = (longlong *****)(ulonglong)uVar5;
      unaff_RBX = _Dst;
      ppppplStack_d8 = local_90;
      ppppppplVar7 = (longlong *******)local_178;
      uVar13 = (uint)local_70;
    }
    local_178 = (longlong ******)ppppppplVar7;
    local_f0 = (longlong ******)0x1;
    ppppplStack_e8 = (longlong *****)unaff_RBX;
    local_e0 = (longlong ****)ppppplVar15;
    local_d0 = (longlong ****)unaff_RDI;
    if (((longlong ****)local_c0 == (longlong ****)0x0) ||
       ((puStack_b8 == (undefined1 *)0x0) != (unaff_RBX == (longlong ******)0x0))) {
LAB_1404d8481:
      ppppplVar14 = (longlong *****)local_78;
      if ((((((ulonglong)local_c0 & 1) != 0) &&
           (puStack_b8 == (undefined1 *)0x0 &&
            (longlong ******)ppppplStack_e8 == (longlong ******)0x0)) && ((int)local_e0 == 1)) &&
         ((local_b0 & 1) == 0)) {
        local_4a = 1;
        local_4b = 1;
        winit::platform_impl::platform::monitor::current_monitor();
        if ((longlong ******)ppppplStack_d8 == extraout_RAX_00) goto LAB_1404d8500;
        uVar13 = (uint)local_70;
      }
      goto LAB_1404d8587;
    }
    if (unaff_RBX == (longlong ******)0x0) {
      ppppppplVar7 = (longlong *******)local_178;
      if (((ulonglong)ppppplVar15 & 1) == 0) {
        if ((local_b0 & 1) != 0) goto LAB_1404d8481;
      }
      else if ((local_b0 & ppppplStack_d8 == ppppplStack_a8) == 0) goto LAB_1404d8481;
    }
    else {
      local_4a = 1;
      local_4b = 1;
      winit::platform_impl::platform::monitor::impl$0::eq();
      if (extraout_AL == '\0') {
        ppppplVar14 = (longlong *****)local_78;
        uVar13 = (uint)local_70;
        if (((ulonglong)local_f0 & 1) == 0) goto LAB_1404d8587;
        goto LAB_1404d8481;
      }
LAB_1404d8500:
      uVar13 = (uint)local_70;
      ppppppplVar7 = (longlong *******)local_178;
      if ((longlong ******)ppppplStack_e8 != (longlong ******)0x0 &&
          (longlong *******)local_f0 != (longlong *******)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
        ppppppplVar7 = (longlong *******)local_178;
      }
    }
LAB_1404d83b3:
    local_178 = (longlong ******)ppppppplVar7;
    if (((char)uVar13 == '\0') &&
       ((*(ulonglong *)__imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E
        & 0x7fffffffffffffff) != 0)) {
      local_49 = 1;
      std::panicking::panic_count::is_zero_slow_path();
      if (extraout_AL_01 == '\0') {
        *(undefined1 *)((longlong)pppppplVar2 + 0x11) = 1;
      }
    }
    LOCK();
    cVar12 = *(char *)local_68;
    *(char *)local_68 = '\0';
    UNLOCK();
    if (cVar12 == '\x02') {
      local_49 = 1;
      std::sys::sync::mutex::futex::Mutex::wake();
    }
    LOCK();
    *local_80 = (longlong ****)((longlong)*local_80 + -1);
    UNLOCK();
    if ((longlong *****)*local_80 == (longlong *****)0x0) {
      local_5b = 1;
      alloc::sync::Arc::
      drop_slow<std::sync::poison::mutex::Mutex<winit::platform_impl::platform::window_state::WindowState>,alloc::alloc::Global>
                ();
    }
    if (((longlong ****)local_c0 != (longlong ****)0x0) && (puStack_b8 != (undefined1 *)0x0)) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  else {
    local_f0 = (longlong ******)0x0;
    ppppplStack_d8 = (longlong *****)pppppplVar6;
    if ((longlong ****)local_c0 == (longlong ****)0x0) goto LAB_1404d83b3;
LAB_1404d8587:
    pppppplVar6 = (longlong ******)ppppplStack_a8;
    _Src = puStack_b8;
    cVar12 = (char)uVar13;
    if (((ulonglong)local_c0 & 1) == 0) {
      ppppplVar15 = (longlong *****)0x0;
      ppppplVar1 = pppppplVar2[7];
      pppppplVar6 = unaff_RBX;
    }
    else {
      unaff_RDI = (longlong *****)CONCAT71(uStack_af,local_b0);
      if (puStack_b8 == (undefined1 *)0x0) {
        ppppplVar15 = (longlong *****)0x1;
        local_78 = (longlong ****)0x0;
        ppppplVar1 = pppppplVar2[7];
      }
      else {
        uVar10 = (ulonglong)uStack_9c;
        local_90 = (longlong *****)(ulonglong)local_a0;
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (_Dst_00 == (longlong *****)0x0) {
          local_4a = 1;
          local_4b = 1;
                    // WARNING: Subroutine does not return
          alloc::alloc::handle_alloc_error();
        }
        local_78 = (longlong ****)_Dst_00;
        memcpy(_Dst_00,_Src,0xdc);
        unaff_R12 = (longlong ******)((uVar10 << 0x20) + (longlong)local_90);
        ppppplVar15 = (longlong *****)0x1;
        cVar12 = (char)local_70;
        ppppplVar1 = pppppplVar2[7];
      }
    }
    if ((ppppplVar1 != (longlong *****)0x0) && (pppppplVar2[8] != (longlong *****)0x0)) {
      local_90 = (longlong *****)unaff_R12;
      std::alloc::__default_lib_allocator::__rust_dealloc();
      unaff_R12 = (longlong ******)local_90;
    }
    pppppplVar2[7] = ppppplVar15;
    pppppplVar2[8] = (longlong *****)local_78;
    pppppplVar2[9] = unaff_RDI;
    pppppplVar2[10] = (longlong *****)pppppplVar6;
    pppppplVar2[0xb] = (longlong *****)unaff_R12;
    if ((cVar12 == '\0') &&
       ((*(ulonglong *)__imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E
        & 0x7fffffffffffffff) != 0)) {
      local_4a = 0;
      local_4b = 1;
      std::panicking::panic_count::is_zero_slow_path();
      if (extraout_AL_00 == '\0') {
        *(undefined1 *)((longlong)pppppplVar2 + 0x11) = 1;
      }
    }
    LOCK();
    cVar12 = *(char *)local_68;
    *(char *)local_68 = '\0';
    UNLOCK();
    if (cVar12 == '\x02') {
      local_4a = 0;
      local_4b = 1;
      std::sys::sync::mutex::futex::Mutex::wake();
    }
    local_178 = local_f0;
    ppppplStack_170 = ppppplStack_e8;
    local_168 = local_e0;
    ppppplStack_160 = ppppplStack_d8;
    local_158 = local_d0;
    local_140 = (char *)CONCAT71(uStack_af,local_b0);
    ppplStack_150 = local_c0;
    local_148 = puStack_b8;
    ppppplStack_138 = ppppplStack_a8;
    local_130 = CONCAT26(uStack_9a,CONCAT24(uStack_9c,local_a0));
    local_128 = local_80;
    local_4a = 0;
    local_4b = 0;
    local_120 = (longlong ****)ppppplVar14;
    winit::platform_impl::platform::event_loop::EventLoopThreadExecutor::
    execute_in_thread<winit::platform_impl::platform::window::impl_0::set_fullscreen::closure_env_0>
              ();
  }
  uVar10 = 0x7c31cf58ebf489e1;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_01 != 0) && (uVar10 < *(ulonglong *)(local_58 + 0x248))) &&
     (uVar10 = *(ulonglong *)(*(longlong *)(local_58 + 0x240) + uVar10 * 8), uVar10 != 0)) {
    lVar9 = ~uVar10 * 0x60;
    if (*(longlong *)(*(longlong *)(local_58 + 0x210) + 0x20 + lVar9) != 0) {
      lVar9 = *(longlong *)(*(longlong *)(*(longlong *)(local_58 + 0x210) + lVar9 + 0x10) + 0x40);
      if (lVar9 < -0x7ffffffffffffffb && lVar9 != -0x7fffffffffffffff) {
LAB_1404d8900:
        uVar10 = 0x2ed189581b92decf;
        bevy_ecs::component::Components::get_resource_id();
        if (((extraout_RAX_05 != 0) && (uVar10 < *(ulonglong *)(local_58 + 0x248))) &&
           ((uVar10 = *(ulonglong *)(*(longlong *)(local_58 + 0x240) + uVar10 * 8), uVar10 != 0 &&
            (*(longlong *)(*(longlong *)(local_58 + 0x210) + 0x20 + ~uVar10 * 0x60) != 0)))) {
          pppplVar11 = (longlong ****)0x0;
          bevy_ecs::storage::blob_vec::BlobVec::swap_remove_and_forget_unchecked();
          local_98 = (longlong *)*extraout_RAX_06;
          country_core::render::impl$0::deref();
          rhapsody::raw::device::GraphicsDevice::
          with_global_allocator<tiny_glade::game_loop::shut_down_everything::closure_env_0,tuple_<u64,u64>_>
                    ();
          local_c0 = (longlong ***)pppplVar11;
          if ((extraout_RAX_07 != 0 || pppplVar11 != (longlong ****)0x0) &&
             (3 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E)) {
            local_f0 = &local_80;
            ppppplStack_e8 = (longlong *****)core::fmt::num::imp::impl$9::fmt;
            local_e0 = &local_c0;
            ppppplStack_d8 = (longlong *****)core::fmt::num::imp::impl$9::fmt;
            toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
            local_158 = (longlong ****)*extraout_RAX_08;
            ppplStack_150 = (longlong ***)extraout_RAX_08[1];
            local_148 = &DAT_00000004;
            local_140 = "tiny_glade::game_loop";
            ppppplStack_138 = (longlong *****)&DAT_00000015;
            local_128 = (longlong *****)&PTR_s_Peak_GPU_memory_use__device_loca_1429ea3e8;
            local_120 = (longlong ****)0x2;
            local_118 = (longlong ******)&local_f0;
            local_110 = 2;
            local_108 = 0;
            local_178 = (longlong ******)0x0;
            ppppplStack_170 = (longlong *****)0x1429e9ce2;
            local_168 = (longlong ****)&DAT_00000015;
            ppppplStack_160 = (longlong *****)0x0;
            local_130 = CONCAT44((int)extraout_RAX_08[2],1);
            ::log::__private_api::impl$2::log();
          }
          LOCK();
          *local_98 = *local_98 + -1;
          UNLOCK();
          if (*local_98 == 0) {
            alloc::sync::Arc::drop_slow<rhapsody::raw::device::GraphicsDevice,alloc::alloc::Global>
                      ();
          }
          LOCK();
          *local_c8 = *local_c8 + -1;
          UNLOCK();
          if (*local_c8 == 0) {
            local_4c = 0;
            alloc::sync::Arc::drop_slow<winit::window::Window,alloc::alloc::Global>();
          }
          core::ptr::drop_in_place<bevy_app::app::App>();
          core::ptr::drop_in_place<std::sync::mpsc::Receiver<bevy_app::sub_app::SubApp>_>();
          if (!SBORROW8(0,*(longlong *)(local_58 + 0xa88))) {
            core::ptr::drop_in_place<bevy_app::sub_app::SubApp>();
          }
          core::ptr::
          drop_in_place<std::sync::mpsc::Receiver<enum2_<tiny_glade::event_loop::OsWindowEvent>_>_>
                    ();
          core::ptr::
          drop_in_place<std::sync::mpsc::Sender<enum2_<tiny_glade::event_loop::GameEvent>_>_>();
          core::ptr::drop_in_place<tiny_glade::run_game::RenderResourceCache>();
          core::ptr::drop_in_place<tiny_glade::run_game::RenderGraphs>();
          rhapsody::raw::managed::handle::impl$6::drop();
          plVar3 = *(longlong **)(local_58 + 0x10f0);
          if (plVar3 != (longlong *)0x0) {
            LOCK();
            *plVar3 = *plVar3 + -1;
            UNLOCK();
            if (*plVar3 == 0) {
              alloc::sync::Arc::
              drop_slow<steamworks::Inner<steamworks::ClientManager>,alloc::alloc::Global>();
            }
          }
          core::ptr::drop_in_place<tiny_glade::egui_backend::EguiBackend>();
          return;
        }
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_02 == (longlong ******)0x0) {
                    // WARNING: Subroutine does not return
        alloc::raw_vec::handle_error();
      }
      local_178 = (longlong ******)&DAT_00000080;
      local_168 = (longlong ****)0x0;
      local_f0 = (longlong ******)&local_178;
      ppppplStack_170 = (longlong *****)extraout_RAX_02;
      country_core::input::input_stream::_::impl$0::
      serialize<ref_mut_<serde_json::ser::Serializer<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>_>
                ();
      if (extraout_RAX_03 == (longlong ******)0x0) {
        local_68 = ppppplStack_170;
        local_70 = (longlong *******)local_178;
        if (!SBORROW8(0,(longlong)local_178)) {
          std::sys::os_str::wtf8::Buf::as_mut_slice();
          std::fs::write::inner();
          if (local_70 != (longlong *******)0x0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          if (((uint)extraout_RAX_04 & 3) == 1) {
            local_78 = (longlong ****)(extraout_RAX_04 + -1);
            local_68 = *(longlong ******)(extraout_RAX_04 + -1);
            local_70 = *(longlong ********)(extraout_RAX_04 + 7);
            if (*local_70 != (longlong ******)0x0) {
              (*(code *)*local_70)(local_68);
            }
            if (local_70[1] != (longlong ******)0x0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          goto LAB_1404d8900;
        }
      }
      else {
        local_68 = (longlong *****)extraout_RAX_03;
        if ((longlong *******)local_178 != (longlong *******)0x0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
      }
      local_178 = (longlong ******)local_68;
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

