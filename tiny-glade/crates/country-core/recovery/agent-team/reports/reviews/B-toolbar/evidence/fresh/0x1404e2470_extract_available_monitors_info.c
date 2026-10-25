
void __cdecl tiny_glade::game_loop::GameLoop::extract_available_monitors_info(void)

{
  undefined8 *puVar1;
  ushort uVar2;
  code *pcVar3;
  DWORD DVar4;
  DWORD DVar5;
  char extraout_AL;
  BOOL BVar6;
  longlong extraout_RAX;
  longlong *extraout_RAX_00;
  undefined8 *****_Dst;
  longlong *extraout_RAX_01;
  undefined8 ****ppppuVar7;
  longlong in_RCX;
  longlong *plVar8;
  ulonglong uVar9;
  longlong lVar10;
  char *pcVar11;
  longlong lVar12;
  longlong lVar13;
  char *pcVar14;
  DWORD iModeNum;
  undefined8 uVar15;
  longlong unaff_R15;
  bool bVar16;
  longlong local_438;
  ulonglong local_428;
  undefined8 uStack_410;
  undefined8 local_408;
  undefined8 uStack_400;
  undefined8 local_3f8;
  undefined8 uStack_3f0;
  undefined8 local_3e8;
  undefined8 uStack_3e0;
  undefined8 local_3d8;
  DEVMODEW local_3c8;
  longlong *local_2e0;
  longlong **local_2d8;
  longlong *local_2d0;
  undefined8 uStack_2c8;
  char *local_2c0;
  ulonglong local_270;
  longlong local_268;
  longlong local_260;
  longlong local_258;
  undefined8 ****local_238;
  code *local_230;
  longlong *local_228;
  longlong *plStack_220;
  ulonglong local_218;
  longlong local_210;
  longlong local_208;
  longlong local_200;
  longlong *local_1f8;
  undefined8 uStack_1f0;
  char *local_1e8;
  undefined8 ****local_1d8;
  code *pcStack_1d0;
  longlong *local_1c8;
  longlong *plStack_1c0;
  undefined8 ****local_1b8;
  longlong *plStack_1b0;
  longlong *local_1a8;
  char *pcStack_1a0;
  undefined8 local_198;
  undefined4 local_190;
  undefined4 local_18c;
  undefined **local_188;
  undefined8 local_180;
  undefined8 ****local_178;
  undefined8 local_170;
  undefined8 local_168;
  longlong *local_f0;
  longlong *local_e8;
  ulonglong local_e0;
  longlong local_d8;
  undefined8 ****local_d0;
  code *local_c8;
  longlong *local_c0;
  int iStack_b8;
  undefined2 uStack_b4;
  undefined2 uStack_b2;
  undefined4 local_ac;
  longlong local_a8;
  longlong *local_a0;
  longlong local_98;
  undefined8 local_90;
  longlong **local_88;
  longlong *local_80;
  undefined1 local_73;
  undefined1 local_72;
  undefined1 local_71;
  undefined8 local_70;
  
  local_70 = 0xfffffffffffffffe;
  uVar9 = 0xe5e047cb9d6d98d7;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX != 0) && (uVar9 < *(ulonglong *)(in_RCX + 0x248))) &&
     (uVar9 = *(ulonglong *)(*(longlong *)(in_RCX + 0x240) + uVar9 * 8), uVar9 != 0)) {
    local_200 = ~uVar9 * 0x60;
    if (*(longlong *)(*(longlong *)(in_RCX + 0x210) + 0x20 + local_200) != 0) {
      local_200 = *(longlong *)(in_RCX + 0x210) + local_200;
      local_e8 = *(longlong **)(local_200 + 0x10);
      local_ac = *(undefined4 *)(in_RCX + 0x5e0);
      *(undefined4 *)(local_200 + 0x5c) = local_ac;
      country_core::resources::window::AvailableMonitors::clear();
      winit::platform_impl::platform::monitor::available_monitors();
      local_a8 = local_268;
      local_e0 = local_270;
      lVar12 = local_260;
      do {
        lVar13 = local_258 + -1;
        if (local_258 == 0) {
          if (local_e0 != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          return;
        }
        local_218 = local_e0;
        if (lVar12 + 1U < local_e0) {
          local_218 = 0;
        }
        local_f0 = *(longlong **)(local_a8 + lVar12 * 8);
        setup::impl$0::from_winit();
        local_a0 = (longlong *)0x0;
        local_90 = 0;
        local_71 = 1;
        local_88 = (longlong **)local_f0;
        winit::platform_impl::platform::monitor::get_monitor_info();
        local_210 = lVar12;
        local_208 = lVar13;
        if ((local_3c8.dmDeviceName._0_4_ & 1) == 0) {
          local_3d8 = CONCAT44(local_3c8.dmFormName._2_4_,local_3c8._100_4_);
          local_3e8 = CONCAT44(local_3c8.field6_0x4c.field1.dmDisplayFixedOutput,
                               local_3c8.field6_0x4c.field1.dmDisplayOrientation);
          uStack_3e0 = CONCAT44(local_3c8._96_4_,local_3c8._92_4_);
          local_3f8 = CONCAT44(local_3c8.dmFields,CONCAT22(local_3c8.dmDriverExtra,local_3c8.dmSize)
                              );
          uStack_3f0 = CONCAT44(local_3c8.field6_0x4c.field1.dmPosition.y,
                                local_3c8.field6_0x4c.field1.dmPosition.x);
          local_438 = CONCAT44(local_3c8.dmDeviceName._8_4_,local_3c8.dmDeviceName._4_4_);
          local_428 = CONCAT44(local_3c8.dmDeviceName._24_4_,local_3c8.dmDeviceName._20_4_);
          uStack_410 = CONCAT44(local_3c8.dmDeviceName._48_4_,local_3c8.dmDeviceName._44_4_);
          local_408 = CONCAT44(local_3c8.dmDeviceName._56_4_,local_3c8.dmDeviceName._52_4_);
          uStack_400 = CONCAT44(local_3c8._64_4_,local_3c8.dmDeviceName._60_4_);
          iModeNum = 0;
LAB_1404e2858:
          local_3c8.dmSpecVersion = 0;
          local_3c8.dmDriverVersion = 0;
          local_3c8.dmDriverExtra = 0;
          local_3c8.dmFields = 0;
          local_3c8.field6_0x4c.field1.dmPosition.x = 0;
          local_3c8.dmReserved2 = 0;
          local_3c8.dmPanningWidth = 0;
          local_3c8.dmPanningHeight = 0;
          local_3c8.dmICMIntent = 0;
          local_3c8.dmMediaType = 0;
          local_3c8.dmDitherType = 0;
          local_3c8.dmReserved1 = 0;
          local_3c8.dmPelsHeight = 0;
          local_3c8.field17_0xb4.dmDisplayFlags = 0;
          local_3c8.dmDisplayFrequency = 0;
          local_3c8.dmICMMethod = 0;
          local_3c8.dmFormName[0x1d] = L'\0';
          local_3c8.dmFormName[0x1e] = L'\0';
          local_3c8.dmFormName[0x1f] = L'\0';
          local_3c8.dmLogPixels = 0;
          local_3c8.dmBitsPerPel = 0;
          local_3c8.dmPelsWidth = 0;
          local_3c8.dmFormName[0x15] = L'\0';
          local_3c8.dmFormName[0x16] = L'\0';
          local_3c8.dmFormName[0x17] = L'\0';
          local_3c8.dmFormName[0x18] = L'\0';
          local_3c8.dmFormName[0x19] = L'\0';
          local_3c8.dmFormName[0x1a] = L'\0';
          local_3c8.dmFormName[0x1b] = L'\0';
          local_3c8.dmFormName[0x1c] = L'\0';
          local_3c8.dmFormName[0xd] = L'\0';
          local_3c8.dmFormName[0xe] = L'\0';
          local_3c8.dmFormName[0xf] = L'\0';
          local_3c8.dmFormName[0x10] = L'\0';
          local_3c8.dmFormName[0x11] = L'\0';
          local_3c8.dmFormName[0x12] = L'\0';
          local_3c8.dmFormName[0x13] = L'\0';
          local_3c8.dmFormName[0x14] = L'\0';
          local_3c8.dmFormName[5] = L'\0';
          local_3c8.dmFormName[6] = L'\0';
          local_3c8.dmFormName[7] = L'\0';
          local_3c8.dmFormName[8] = L'\0';
          local_3c8.dmFormName[9] = L'\0';
          local_3c8.dmFormName[10] = L'\0';
          local_3c8.dmFormName[0xb] = L'\0';
          local_3c8.dmFormName[0xc] = L'\0';
          local_3c8.dmYResolution = 0;
          local_3c8.dmTTOption = 0;
          local_3c8.dmCollate = 0;
          local_3c8.dmFormName[0] = L'\0';
          local_3c8.dmFormName[1] = L'\0';
          local_3c8.dmFormName[2] = L'\0';
          local_3c8.dmFormName[3] = L'\0';
          local_3c8.dmFormName[4] = L'\0';
          local_3c8.field6_0x4c.field1.dmPosition.y = 0;
          local_3c8.field6_0x4c.field1.dmDisplayOrientation = 0;
          local_3c8.field6_0x4c.field1.dmDisplayFixedOutput = 0;
          local_3c8.dmColor = 0;
          local_3c8.dmDuplex = 0;
          local_3c8.dmDeviceName[0x18] = L'\0';
          local_3c8.dmDeviceName[0x19] = L'\0';
          local_3c8.dmDeviceName[0x1a] = L'\0';
          local_3c8.dmDeviceName[0x1b] = L'\0';
          local_3c8.dmDeviceName[0x1c] = L'\0';
          local_3c8.dmDeviceName[0x1d] = L'\0';
          local_3c8.dmDeviceName[0x1e] = L'\0';
          local_3c8.dmDeviceName[0x1f] = L'\0';
          local_3c8.dmDeviceName[0x10] = L'\0';
          local_3c8.dmDeviceName[0x11] = L'\0';
          local_3c8.dmDeviceName[0x12] = L'\0';
          local_3c8.dmDeviceName[0x13] = L'\0';
          local_3c8.dmDeviceName[0x14] = L'\0';
          local_3c8.dmDeviceName[0x15] = L'\0';
          local_3c8.dmDeviceName[0x16] = L'\0';
          local_3c8.dmDeviceName[0x17] = L'\0';
          local_3c8.dmDeviceName[8] = L'\0';
          local_3c8.dmDeviceName[9] = L'\0';
          local_3c8.dmDeviceName[10] = L'\0';
          local_3c8.dmDeviceName[0xb] = L'\0';
          local_3c8.dmDeviceName[0xc] = L'\0';
          local_3c8.dmDeviceName[0xd] = L'\0';
          local_3c8.dmDeviceName[0xe] = L'\0';
          local_3c8.dmDeviceName[0xf] = L'\0';
          local_3c8.dmDeviceName[0] = L'\0';
          local_3c8.dmDeviceName[1] = L'\0';
          local_3c8.dmDeviceName[2] = L'\0';
          local_3c8.dmDeviceName[3] = L'\0';
          local_3c8.dmDeviceName[4] = L'\0';
          local_3c8.dmDeviceName[5] = L'\0';
          local_3c8.dmDeviceName[6] = L'\0';
          local_3c8.dmDeviceName[7] = L'\0';
          local_3c8.dmSize = 0xdc;
          BVar6 = EnumDisplaySettingsExW((LPCWSTR)&uStack_410,iModeNum,&local_3c8,0);
          DVar4 = local_3c8.dmBitsPerPel;
          if (BVar6 != 0) {
            if ((~local_3c8.dmFields & 0x5c0000) != 0) {
              local_71 = 1;
                    // WARNING: Subroutine does not return
              core::panicking::panic();
            }
            pcVar3 = (code *)CONCAT44(local_3c8.dmPelsHeight,local_3c8.dmPelsWidth);
            pcVar14 = (char *)(ulonglong)local_3c8.dmBitsPerPel;
            DVar5 = local_3c8.dmDisplayFrequency;
            memcpy(&local_1d8,&local_3c8,0xdc);
            std::alloc::__default_lib_allocator::__rust_alloc();
            if (_Dst == (undefined8 *****)0x0) {
              local_71 = 1;
                    // WARNING: Subroutine does not return
              alloc::alloc::handle_alloc_error();
            }
            memcpy(_Dst,&local_1d8,0xdc);
            iStack_b8 = DVar5 * 1000;
            local_c0 = (longlong *)local_88;
            uStack_b4 = (undefined2)DVar4;
            local_d0 = _Dst;
            local_c8 = pcVar3;
            if (local_a0 != (longlong *)0x0) goto code_r0x0001404e2975;
            plVar8 = (longlong *)0x0;
            goto LAB_1404e2a02;
          }
          bVar16 = local_a0 != (longlong *)0x0;
          plVar8 = local_a0;
          uVar15 = 0;
          unaff_R15 = local_98;
          if (bVar16) {
            uVar15 = local_90;
          }
        }
        else {
          local_d0 = (undefined8 ****)
                     CONCAT44(local_3c8.dmDeviceName._12_4_,local_3c8.dmDeviceName._8_4_);
          plVar8 = (longlong *)0x0;
          uVar15 = 0;
          if (1 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
            local_230 = std::io::error::impl$9::fmt;
            local_238 = &local_d0;
            toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
            local_1b8 = (undefined8 ****)*extraout_RAX_00;
            plStack_1b0 = (longlong *)extraout_RAX_00[1];
            local_18c = (undefined4)extraout_RAX_00[2];
            local_1a8 = (longlong *)0x2;
            pcStack_1a0 = "winit::platform_impl::platform::monitor";
            local_198 = 0x27;
            local_188 = &PTR_s_Error_from_get_monitor_info__1429e9940;
            local_180 = 1;
            local_178 = &local_238;
            local_170 = 1;
            local_168 = 0;
            local_1d8 = (undefined8 *****)0x0;
            pcStack_1d0 = (code *)0x1429e9968;
            local_1c8 = (longlong *)&DAT_00000027;
            plStack_1c0 = (longlong *)0x0;
            local_190 = 1;
            ::log::__private_api::impl$2::log();
            plVar8 = local_a0;
            uVar15 = local_90;
            unaff_R15 = local_98;
          }
          if (((uint)local_d0 & 3) == 1) {
            local_d8 = (longlong)local_d0 + -1;
            local_80 = *(longlong **)((longlong)local_d0 + -1);
            local_88 = *(longlong ***)((longlong)local_d0 + 7);
            if ((code *)*local_88 != (code *)0x0) {
              (*(code *)*local_88)(local_80);
            }
            if (local_88[1] != (longlong *)0x0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          bVar16 = plVar8 != (longlong *)0x0;
          if (!bVar16) {
            uVar15 = 0;
          }
        }
        local_1d8 = (undefined8 ****)(ulonglong)bVar16;
        pcStack_1d0 = (code *)0x0;
        plStack_1b0 = (longlong *)0x0;
        local_72 = 1;
        local_1c8 = plVar8;
        plStack_1c0 = (longlong *)unaff_R15;
        local_1b8 = local_1d8;
        local_1a8 = plVar8;
        pcStack_1a0 = (char *)unaff_R15;
        local_198 = uVar15;
        alloc::vec::spec_from_iter::impl$0::
        from_iter<winit::monitor::VideoMode,core::iter::adapters::map::Map<core::iter::adapters::map::Map<alloc::collections::btree::set::IntoIter<winit::monitor::VideoMode,alloc::alloc::Global>,winit::platform_impl::platform::monitor::impl_7::video_modes::closure_env_0>,winit::monitor::impl_5::video_modes::closure_env_0>_>
                  ();
        if (1 < local_428) {
          if (local_428 < 0x15) {
            local_73 = 1;
            core::slice::sort::shared::smallsort::
            insertion_sort_shift_left<winit::monitor::VideoMode,bool_(*)(ref_<winit::monitor::VideoMode>,ref_<winit::monitor::VideoMode>)>
                      ();
          }
          else {
            local_73 = 1;
            core::slice::sort::stable::
            driftsort_main<winit::monitor::VideoMode,bool_(*)(ref_<winit::monitor::VideoMode>,ref_<winit::monitor::VideoMode>),alloc::vec::Vec<winit::monitor::VideoMode,alloc::alloc::Global>_>
                      ();
          }
        }
        local_80 = local_2e0;
        local_88 = local_2d8;
        alloc::vec::spec_from_iter::impl$0::
        from_iter<country_core::render::render_config::VideoMode,core::iter::adapters::map::Map<core::slice::iter::Iter<winit::monitor::VideoMode>,country_core::render::render_config::VideoMode_(*)(ref_<winit::monitor::VideoMode>)>_>
                  ();
        local_3c8.dmDeviceName._16_4_ = SUB84(local_1c8,0);
        local_3c8.dmDeviceName._20_4_ = SUB84((ulonglong)local_1c8 >> 0x20,0);
        local_3c8.dmDeviceName._0_4_ = SUB84(local_1d8,0);
        local_3c8.dmDeviceName._4_4_ = SUB84((ulonglong)local_1d8 >> 0x20,0);
        local_3c8.dmDeviceName._8_4_ = SUB84(pcStack_1d0,0);
        local_3c8.dmDeviceName._12_4_ = SUB84((ulonglong)pcStack_1d0 >> 0x20,0);
        *(undefined4 *)(local_200 + 0x5c) = local_ac;
        plStack_1c0 = local_80;
        local_1b8 = (undefined8 ****)local_88;
        pcStack_1a0 = local_2c0;
        plStack_1b0 = local_2d0;
        local_1a8 = (longlong *)uStack_2c8;
        lVar12 = local_e8[2];
        if (lVar12 == *local_e8) {
          alloc::raw_vec::RawVec::
          grow_one<country_core::resources::window::AvailableMonitor,alloc::alloc::Global>();
        }
        lVar13 = local_e8[1];
        lVar10 = lVar12 * 0x40;
        puVar1 = (undefined8 *)(lVar13 + 0x30 + lVar10);
        *puVar1 = local_1a8;
        puVar1[1] = pcStack_1a0;
        plVar8 = (longlong *)(lVar13 + 0x20 + lVar10);
        *plVar8 = (longlong)local_1b8;
        plVar8[1] = (longlong)plStack_1b0;
        puVar1 = (undefined8 *)(lVar13 + 0x10 + lVar10);
        *puVar1 = local_1c8;
        puVar1[1] = plStack_1c0;
        *(longlong *)(lVar13 + lVar10) = (longlong)local_1d8;
        ((longlong *)(lVar13 + lVar10))[1] = (longlong)pcStack_1d0;
        local_e8[2] = lVar12 + 1;
        for (uVar9 = local_428; uVar9 != 0; uVar9 = uVar9 - 1) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        unaff_R15 = 0;
        if (local_438 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        lVar12 = (local_210 - local_218) + 1;
        local_258 = local_208;
      } while( true );
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
code_r0x0001404e2975:
  local_80 = (longlong *)CONCAT44(local_80._4_4_,iModeNum);
  plVar8 = local_a0;
  lVar12 = local_98;
  while( true ) {
    uVar2 = *(ushort *)((longlong)plVar8 + 0x16a);
    pcVar11 = (char *)(ulonglong)uVar2;
    pcVar14 = (char *)0xffffffffffffffff;
    lVar13 = 0;
    do {
      if ((ulonglong)uVar2 << 5 == lVar13) {
        pcVar14 = pcVar11;
        if (lVar12 != 0) goto LAB_1404e29ed;
        goto LAB_1404e2b43;
      }
      winit::monitor::impl$1::partial_cmp();
      lVar13 = lVar13 + 0x20;
      pcVar14 = pcVar14 + 1;
    } while (extraout_AL == '\x01');
    if (extraout_AL == '\0') {
      std::alloc::__default_lib_allocator::__rust_dealloc();
      iModeNum = (DWORD)local_80 + 1;
      goto LAB_1404e2858;
    }
    pcVar11 = pcVar14;
    if (lVar12 == 0) break;
LAB_1404e29ed:
    lVar12 = lVar12 + -1;
    plVar8 = (longlong *)plVar8[(longlong)(pcVar11 + 0x2e)];
  }
LAB_1404e2b43:
  iModeNum = (DWORD)local_80;
  if ((undefined8 *****)local_d0 != (undefined8 *****)0x0) {
LAB_1404e2a02:
    ppppuVar7 = local_d0;
    pcStack_1d0 = local_c8;
    plStack_1c0 = (longlong *)CONCAT26(uStack_b2,CONCAT24(uStack_b4,iStack_b8));
    local_1c8 = local_c0;
    local_1b8 = (undefined8 ****)&local_a0;
    local_1a8 = (longlong *)0x0;
    local_1d8 = local_d0;
    plStack_1b0 = plVar8;
    pcStack_1a0 = pcVar14;
    local_80 = plVar8;
    if (plVar8 == (longlong *)0x0) {
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_01 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
        alloc::alloc::handle_alloc_error();
      }
      extraout_RAX_01[0x2c] = 0;
      local_98 = 0;
      extraout_RAX_01[3] = (longlong)plStack_1c0;
      extraout_RAX_01[1] = (longlong)pcStack_1d0;
      extraout_RAX_01[2] = (longlong)local_1c8;
      *(undefined2 *)((longlong)extraout_RAX_01 + 0x16a) = 1;
      *extraout_RAX_01 = (longlong)ppppuVar7;
      ppppuVar7 = (undefined8 ****)&local_a0;
      local_a0 = extraout_RAX_01;
    }
    else {
      uStack_1f0 = 0;
      local_228 = local_c0;
      local_230 = local_c8;
      local_238 = local_d0;
      plStack_220 = plStack_1c0;
      local_1f8 = plVar8;
      local_1e8 = pcVar14;
      alloc::collections::btree::node::Handle::
      insert_recursing<winit::monitor::VideoMode,alloc::collections::btree::set_val::SetValZST,alloc::alloc::Global,alloc::collections::btree::map::entry::impl_8::insert_entry::closure_env_0<winit::monitor::VideoMode,alloc::collections::btree::set_val::SetValZST,alloc::alloc::Global>_>
                ();
      ppppuVar7 = local_1b8;
    }
    ppppuVar7[2] = (undefined8 ***)((longlong)ppppuVar7[2] + 1);
  }
  iModeNum = iModeNum + 1;
  goto LAB_1404e2858;
}

