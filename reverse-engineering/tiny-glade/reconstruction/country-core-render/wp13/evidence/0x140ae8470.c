// name: new_from_session size: 2682
// void __cdecl new_from_session(void)

void __cdecl country_core::systems::save::save_backup_list::SaveBackupList::new_from_session(void)

{
  longlong *plVar1;
  undefined ****ppppuVar2;
  undefined ***pppuVar3;
  undefined ***pppuVar4;
  longlong lVar5;
  char extraout_AL;
  int iVar6;
  undefined ***extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined ***extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong *in_RCX;
  longlong lVar7;
  uint uVar8;
  uint uVar9;
  ulonglong uVar10;
  undefined8 *puVar11;
  ulonglong uVar12;
  char *pcVar13;
  ulonglong uVar14;
  bool bVar15;
  undefined **local_f78;
  code *local_f70 [74];
  undefined ****local_d20;
  undefined ***pppuStack_d18;
  undefined ***local_d10;
  undefined8 uStack_d08;
  undefined8 local_d00;
  undefined8 uStack_cf8;
  undefined8 local_cf0;
  char *local_ce8;
  undefined8 local_ce0;
  undefined4 local_cd8;
  undefined4 local_cd4;
  undefined **local_cd0;
  undefined8 local_cc8;
  undefined *****local_cc0;
  undefined8 local_cb8;
  undefined8 local_cb0;
  undefined8 local_ac0;
  undefined ****local_860;
  undefined ***local_858;
  longlong *local_850 [76];
  undefined ***local_5f0;
  undefined8 *local_5e8;
  undefined ****ppppuStack_5e0;
  undefined ***local_5d8;
  undefined8 uStack_5d0;
  undefined8 local_5c8;
  undefined ****local_398;
  undefined ***pppuStack_390;
  undefined ***local_388;
  undefined8 uStack_380;
  undefined8 local_378;
  undefined8 uStack_370;
  undefined ***local_128;
  undefined8 *local_120;
  undefined ****ppppuStack_118;
  undefined ***local_110;
  undefined8 uStack_108;
  undefined8 local_100;
  undefined8 *local_f8;
  undefined ****ppppuStack_f0;
  undefined ***local_e8;
  undefined8 uStack_e0;
  undefined8 local_d8;
  undefined8 *local_d0;
  undefined1 *local_c8;
  undefined ***local_c0;
  undefined ****local_b8;
  longlong local_b0;
  longlong local_90;
  longlong lStack_88;
  longlong local_80;
  uint local_74;
  undefined1 *local_70;
  undefined8 *local_68;
  undefined1 local_59;
  undefined ***local_58;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  local_90 = 0;
  lStack_88 = 8;
  local_80 = 0;
  resources::session::SessionId::backup_folder_path();
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  std::path::Path::is_dir();
  if (extraout_AL == '\0') {
    local_d20 = (undefined ****)CONCAT71(local_d20._1_7_,1);
    std::sys::os_str::wtf8::Buf::as_mut_slice();
    std::fs::DirBuilder::_create();
    uVar9 = 0;
    if (extraout_RAX_01 != (undefined ***)0x0) {
      local_5f0 = extraout_RAX_01;
      if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
        local_398 = &local_5f0;
        pppuStack_390 = (undefined ***)std::io::error::impl$0::fmt;
        toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
        local_d00 = *extraout_RAX_02;
        uStack_cf8 = extraout_RAX_02[1];
        local_cd4 = *(undefined4 *)(extraout_RAX_02 + 2);
        local_cf0 = 1;
        local_ce8 = "country_core::systems::save::save_backup_list";
        local_ce0 = 0x2d;
        local_cd0 = (undefined **)&DAT_142ae3700;
        local_cc8 = 1;
        local_cc0 = &local_398;
        local_cb8 = 1;
        local_cb0 = 0;
        local_d20 = (undefined ****)0x0;
        pppuStack_d18 = (undefined ***)0x142ae3728;
        local_d10 = (undefined ***)&DAT_0000002d;
        uStack_d08 = 0;
        local_cd8 = 1;
        ::log::__private_api::impl$2::log();
      }
      if (((uint)local_5f0 & 3) == 1) {
        local_70 = (undefined1 *)((longlong)local_5f0 + -1);
        local_58 = *(undefined ****)((longlong)local_5f0 + -1);
        local_68 = *(undefined8 **)((longlong)local_5f0 + 7);
        if ((code *)*local_68 != (code *)0x0) {
          (*(code *)*local_68)(local_58);
        }
        if (local_68[1] != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
  }
  else {
    std::sys::os_str::wtf8::Buf::as_mut_slice();
    std::sys::pal::windows::fs::readdir();
    pppuVar3 = pppuStack_390;
    ppppuVar2 = local_398;
    if (local_398 == (undefined ****)0x2) {
      local_5f0 = pppuStack_390;
      local_59 = 1;
      std::backtrace::Backtrace::capture();
      local_378 = local_d00;
      uStack_370 = uStack_cf8;
      local_388 = local_d10;
      uStack_380 = uStack_d08;
      local_398 = local_d20;
      pppuStack_390 = pppuStack_d18;
      local_d20 = (undefined ****)0x142ae367c;
      pppuStack_d18 = (undefined ***)&DAT_00000023;
      local_d10 = pppuVar3;
      local_59 = 0;
      anyhow::Error::construct<anyhow::error::ContextError<ref_<str_>,std::io::error::Error>_>();
      local_5f0 = extraout_RAX;
      if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
        local_398 = &local_5f0;
        pppuStack_390 = (undefined ***)save_backup_list::impl$2::fmt;
        toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
        local_d00 = *extraout_RAX_00;
        uStack_cf8 = extraout_RAX_00[1];
        local_cd4 = *(undefined4 *)(extraout_RAX_00 + 2);
        local_cf0 = 1;
        local_ce8 = "country_core::systems::save::save_backup_list";
        local_ce0 = 0x2d;
        local_cd0 = &PTR_s_Error_reading_save_backup_direct_142ae37f8;
        local_cc8 = 1;
        local_cb8 = 1;
        local_cb0 = 0;
        local_d20 = (undefined ****)0x0;
        pppuStack_d18 = (undefined ***)0x142ae3728;
        local_d10 = (undefined ***)&DAT_0000002d;
        uStack_d08 = 0;
        local_cd8 = 1;
        local_cc0 = &local_398;
        ::log::__private_api::impl$2::log();
      }
      anyhow::error::impl$6::drop();
      uVar9 = 0;
    }
    else {
      memcpy(local_850,&local_388,0x260);
      local_d20 = (undefined ****)0x0;
      pppuStack_d18 = pppuVar3;
      local_ac0 = 0;
      local_860 = ppppuVar2;
      local_858 = pppuVar3;
      local_74 = 0;
      if (ppppuVar2 != (undefined ****)0x2) {
LAB_140ae88c3:
        local_b8 = &local_5f0;
LAB_140ae88f0:
        std::fs::impl$30::next();
        pppuVar3 = pppuStack_390;
        if (local_398 != (undefined ****)0x1) {
          if (local_398 != (undefined ****)0x0) {
            if (pppuStack_390 == (undefined ***)0x0) {
              if (((uint)local_388 & 3) == 1) {
                local_70 = (undefined1 *)((longlong)local_388 + -1);
                local_58 = *(undefined ****)((longlong)local_388 + -1);
                local_68 = *(undefined8 **)((longlong)local_388 + 7);
                if ((code *)*local_68 != (code *)0x0) {
                  (*(code *)*local_68)(local_58);
                }
                if (local_68[1] != 0) {
                  std::alloc::__default_lib_allocator::__rust_dealloc();
                }
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
            }
            else {
              LOCK();
              *pppuStack_390 = (undefined **)((longlong)*pppuStack_390 + -1);
              UNLOCK();
              if (*pppuStack_390 == (undefined **)0x0) {
                alloc::sync::Arc::drop_slow<std::path::PathBuf,alloc::alloc::Global>();
              }
            }
          }
          if (local_860 == (undefined ****)0x0) {
LAB_140ae8e58:
            LOCK();
            *local_850[0] = *local_850[0] + -1;
            UNLOCK();
            if (*local_850[0] == 0) {
              alloc::sync::Arc::drop_slow<std::path::PathBuf,alloc::alloc::Global>();
            }
          }
          else if ((int)local_860 != 2) {
            std::sys::pal::windows::fs::impl$7::drop();
            goto LAB_140ae8e58;
          }
          local_860 = (undefined ****)0x2;
          goto LAB_140ae8e7c;
        }
        if (pppuStack_390 == (undefined ***)0x0) {
          if (((uint)local_388 & 3) == 1) {
            local_70 = (undefined1 *)((longlong)local_388 + -1);
            local_58 = *(undefined ****)((longlong)local_388 + -1);
            local_68 = *(undefined8 **)((longlong)local_388 + 7);
            if ((code *)*local_68 != (code *)0x0) {
              (*(code *)*local_68)(local_58);
            }
            if (local_68[1] != 0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          goto LAB_140ae88f0;
        }
        memcpy(local_f70,&local_388,0x250);
        local_f78 = (undefined **)0x0;
        memcpy(&local_5e8,&local_388,0x250);
        local_5f0 = pppuVar3;
        core::ops::function::impls::impl$3::
        call_mut<tuple_<std::fs::DirEntry>,country_core::systems::save::save_backup_list::impl_1::iter_backup_files::closure_env_0>
                  ();
        pppuVar3 = local_128;
        if (SBORROW8(0,(longlong)local_128)) {
          if (local_f78 != (undefined **)0x0) {
            LOCK();
            *local_f78 = *local_f78 + -1;
            UNLOCK();
            if (*local_f78 == (undefined *)0x0) {
              alloc::sync::Arc::drop_slow<std::path::PathBuf,alloc::alloc::Global>();
            }
          }
          goto LAB_140ae88f0;
        }
        local_d8 = local_100;
        local_e8 = local_110;
        uStack_e0 = uStack_108;
        local_f8 = local_120;
        ppppuStack_f0 = ppppuStack_118;
        if (local_f78 != (undefined **)0x0) {
          LOCK();
          *local_f78 = *local_f78 + -1;
          UNLOCK();
          if (*local_f78 == (undefined *)0x0) {
            alloc::sync::Arc::drop_slow<std::path::PathBuf,alloc::alloc::Global>();
          }
        }
        local_5f0 = pppuVar3;
        local_5c8 = local_d8;
        local_5d8 = local_e8;
        uStack_5d0 = uStack_e0;
        local_5e8 = local_f8;
        ppppuStack_5e0 = ppppuStack_f0;
        local_49 = 1;
        std::sys::os_str::wtf8::Buf::as_mut_slice();
        local_49 = 1;
        std::path::Path::file_stem();
        uVar9 = local_74;
        if (extraout_RAX_03 != 0) {
          local_49 = 1;
          std::ffi::os_str::impl$41::try_from();
          pppuVar3 = local_388;
          if ((int)local_398 == 1) {
            local_49 = 1;
                    // WARNING: Subroutine does not return
            core::option::unwrap_failed();
          }
          local_58 = pppuStack_390;
          local_f78 = &PTR_s_history_auto_142ae3780;
          local_f70[0] = anyhow::context::impl$3::
                         fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
          local_398 = (undefined ****)&DAT_142ae3798;
          pppuStack_390 = (undefined ***)0x2;
          local_388 = &local_f78;
          uStack_380 = 1;
          local_378 = 0;
          local_49 = 1;
          alloc::fmt::format::format_inner();
          pppuVar4 = local_c0;
          uVar14 = (longlong)pppuVar3 - (longlong)local_c0;
          puVar11 = local_d0;
          if (pppuVar3 < local_c0) {
LAB_140ae8b8c:
            uVar9 = local_74;
            if (puVar11 != (undefined8 *)0x0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
              uVar9 = local_74;
            }
            goto LAB_140ae8ba6;
          }
          local_68 = local_d0;
          local_70 = local_c8;
          iVar6 = memcmp(local_c8,local_58,(size_t)local_c0);
          puVar11 = local_68;
          if (iVar6 != 0) goto LAB_140ae8b8c;
          if (uVar14 == 0) {
            bVar15 = true;
            uVar10 = 0;
          }
          else {
            pcVar13 = (char *)((longlong)local_58 + (longlong)pppuVar4);
            if (uVar14 == 1) {
              bVar15 = true;
              uVar10 = 0;
              if ((*pcVar13 != '+') && (local_58 = (undefined ***)0x1, *pcVar13 != '-'))
              goto LAB_140ae8d1e;
            }
            else {
              if (*pcVar13 == '+') {
                pcVar13 = pcVar13 + 1;
                local_58 = (undefined ***)(uVar14 - 1);
                if (9 < uVar14) {
LAB_140ae8cd3:
                  uVar12 = 0;
                  uVar14 = 0;
                  do {
                    bVar15 = local_58 != (undefined ***)uVar12;
                    uVar10 = uVar14;
                    if (local_58 == (undefined ***)uVar12) break;
                    uVar9 = (uint)(uVar14 * 10);
                    uVar8 = (byte)pcVar13[uVar12] - 0x30;
                    if ((9 < uVar8) || ((int)(uVar14 * 10 >> 0x20) != 0)) goto LAB_140ae8d4a;
                    uVar12 = uVar12 + 1;
                    uVar14 = (ulonglong)(uVar9 + uVar8);
                    uVar10 = 0;
                  } while (!CARRY4(uVar9,uVar8));
                  goto joined_r0x000140ae8d90;
                }
              }
              else {
                local_58 = (undefined ***)uVar14;
                if (8 < uVar14) goto LAB_140ae8cd3;
              }
LAB_140ae8d1e:
              uVar14 = 0;
              uVar10 = 0;
              do {
                bVar15 = 9 < (byte)pcVar13[uVar14] - 0x30;
                if (bVar15) goto LAB_140ae8d4a;
                uVar10 = (ulonglong)(((byte)pcVar13[uVar14] - 0x30) + (int)uVar10 * 10);
                uVar14 = uVar14 + 1;
              } while (local_58 != (undefined ***)uVar14);
            }
          }
          goto joined_r0x000140ae8d90;
        }
        goto LAB_140ae8ba6;
      }
LAB_140ae8e7c:
      core::ptr::
      drop_in_place<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl_1::iter_backup_files::closure_env_0>_>
                ();
      uVar9 = local_74;
    }
  }
  in_RCX[2] = local_80;
  *in_RCX = local_90;
  in_RCX[1] = lStack_88;
  *(uint *)(in_RCX + 3) = uVar9;
  if (local_b0 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
LAB_140ae8d4a:
  uVar10 = 0;
joined_r0x000140ae8d90:
  local_58 = (undefined ***)uVar10;
  if (local_68 != (undefined8 *)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  uVar9 = (int)local_58 + 1;
  if (uVar9 <= local_74) {
    uVar9 = local_74;
  }
  if (bVar15) {
    uVar9 = local_74;
  }
LAB_140ae8ba6:
  local_74 = uVar9;
  lVar5 = local_80;
  local_58 = local_5f0;
  local_68 = local_5e8;
  local_398 = ppppuStack_5e0;
  pppuStack_390 = local_5d8;
  if (local_80 == local_90) {
    alloc::raw_vec::RawVec::
    grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
              ();
  }
  lVar7 = lVar5 * 0x20;
  *(undefined ****)(lStack_88 + lVar7) = local_58;
  *(undefined8 **)(lStack_88 + 8 + lVar7) = local_68;
  plVar1 = (longlong *)(lStack_88 + 0x10 + lVar7);
  *plVar1 = (longlong)local_398;
  plVar1[1] = (longlong)pppuStack_390;
  local_80 = lVar5 + 1;
  if (local_860 == (undefined ****)0x2) goto LAB_140ae8e7c;
  goto LAB_140ae88c3;
}


