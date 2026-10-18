// 0x1407b66e0 load_decorators
// sig: void __cdecl load_decorators(void)
// callees: [{"name": "_ZN100_$LT$country_core..components..drawable..WireframePass$u20$as$u20$bevy_ecs..component..Component$GT$14clone_behavior17h98754d446e2f5bc0E", "address": "140002370"}, {"name": "__rust_alloc", "address": "140613c10"}, {"name": "__rust_dealloc", "address": "140613c20"}, {"name": "add_color", "address": "140a07540"}, {"name": "as_mut_slice", "address": "140d76ed0"}, {"name": "capture", "address": "140d56700"}, {"name": "construct<anyhow::error::ContextError<ref$<str$>,std::io::error::Error>_>", "address": "1428f8140"}, {"name": "drop", "address": "140d685b0"}, {"name": "drop_slow<std::path::PathBuf,alloc::alloc::Global>", "address": "140d53bc0"}, {"name": "file_name", "address": "140d5a130"}, {"name": "format_inner", "address": "140d79f90"}, {"name": "handle_error", "address": "1428d8fe3"}, {"name": "load_json_as_mesh<std::path::PathBuf>", "address": "141286bf0"}, {"name": "load_mesh_into_library", "address": "140b1aca0"}, {"name": "memcpy", "address": "142923c00"}, {"name": "next", "address": "1408b99e0"}, {"name": "next", "address": "140d59e80"}, {"name": "path", "address": "140d59f10"}, {"name": "readdir", "address": "140d68f80"}, {"name": "slice_error_fail", "address": "1428d9940"}, {"name": "try_from", "address": "140d58e70"}, {"name": "unwrap_failed", "address": "1428d9760"}, {"name": "unwrap_failed", "address": "1428d9310"}]

// WARNING: Control flow encountered bad instruction data
// WARNING: Switch with 1 destination removed at 0x0001407b6bdb : 2 cases all go to same destination

void __cdecl system_decorator::load_decorators::load_decorators(void)

{
  int iVar1;
  int iVar2;
  int *piVar3;
  uint extraout_EAX;
  int *extraout_RAX;
  undefined8 *in_RCX;
  uint uVar4;
  undefined8 *in_R8;
  ulonglong local_950;
  longlong *plStack_948;
  longlong *local_940;
  undefined8 uStack_938;
  undefined8 local_930;
  undefined8 uStack_928;
  longlong local_6e0;
  undefined8 local_6d8;
  int **local_6c8;
  code *local_6c0;
  ulonglong local_6b8;
  longlong *local_6a8;
  undefined8 uStack_6a0;
  undefined8 local_698;
  undefined8 uStack_690;
  longlong *local_448;
  longlong *local_440;
  undefined1 local_438 [584];
  longlong local_1f0;
  int *local_1d0;
  ulonglong local_1c8;
  int *local_1b8;
  int *piStack_1b0;
  ulonglong local_1a8;
  undefined8 uStack_1a0;
  undefined8 local_198;
  undefined8 uStack_190;
  undefined8 local_188;
  undefined8 uStack_180;
  undefined8 local_178;
  undefined8 uStack_170;
  undefined8 local_168;
  undefined8 uStack_160;
  undefined8 local_158;
  undefined8 uStack_150;
  undefined8 local_148;
  longlong *local_138;
  int *piStack_130;
  ulonglong local_128;
  undefined8 uStack_120;
  undefined8 local_118;
  undefined8 uStack_110;
  undefined8 local_108;
  undefined8 uStack_100;
  undefined8 local_f8;
  undefined8 uStack_f0;
  undefined8 local_e8;
  undefined8 uStack_e0;
  undefined8 local_d8;
  undefined8 uStack_d0;
  undefined8 local_c8;
  undefined8 *local_c0;
  undefined8 local_b8;
  undefined1 local_99;
  undefined8 local_98;
  longlong local_90;
  undefined1 local_6d;
  undefined8 local_60;
  
  local_60 = 0xfffffffffffffffe;
  local_c0 = in_RCX;
  std::sys::pal::windows::fs::readdir();
  if (local_950 == 2) {
    local_99 = 1;
    std::backtrace::Backtrace::capture();
    local_698 = local_930;
    uStack_690 = uStack_928;
    local_6a8 = local_940;
    uStack_6a0 = uStack_938;
    local_99 = 0;
    anyhow::Error::construct<anyhow::error::ContextError<ref_<str_>,std::io::error::Error>_>();
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  memcpy(&local_6a8,&local_940,0x260);
  local_6b8 = local_950;
  local_b8 = *in_R8;
  do {
    std::fs::impl$30::next();
    if ((local_950 & 1) == 0) {
      if (local_6b8 != 0) {
        std::sys::pal::windows::fs::impl$7::drop();
      }
      LOCK();
      *local_6a8 = *local_6a8 + -1;
      UNLOCK();
      if (*local_6a8 == 0) {
        alloc::sync::Arc::drop_slow<std::path::PathBuf,alloc::alloc::Global>();
      }
      _ZN100__LT_country_core__components__drawable__WireframePass_u20_as_u20_bevy_ecs__component__Component_GT_14clone_behavior17h98754d446e2f5bc0E
                ();
      local_90 = CONCAT44(local_90._4_4_,*(undefined4 *)((longlong)local_c0 + 0x1c));
      local_b8 = *local_c0;
      local_98 = local_c0[2];
      if ((char)extraout_EAX == '\x1a') {
        return;
      }
      country_core::resources::clutter_meshes::_::impl$0::next();
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
      if ((POPCOUNT((ulonglong)
                    (&DAT_142a8560c +
                    *(int *)(&DAT_142a8560c + ((ulonglong)extraout_EAX & 0xff) * 4)) & 0xff) & 1U)
          != 0) {
                    // WARNING: Bad instruction - Truncating control flow here
        halt_baddata();
      }
                    // WARNING: Bad instruction - Truncating control flow here
      halt_baddata();
    }
    if (plStack_948 == (longlong *)0x0) {
      local_138 = local_940;
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    local_448 = plStack_948;
    local_440 = local_940;
    memcpy(local_438,&uStack_938,0x248);
    std::fs::DirEntry::file_name();
    std::sys::os_str::wtf8::Buf::as_mut_slice();
    std::ffi::os_str::impl$41::try_from();
    if ((int)local_138 == 1) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    if (4 < local_128) {
      uVar4 = *(uint *)((longlong)piStack_130 + (local_128 - 5));
      uVar4 = uVar4 >> 0x18 | (uVar4 & 0xff0000) >> 8 | (uVar4 & 0xff00) << 8 | uVar4 << 0x18;
      if (uVar4 == 0x2e6a736f) {
        if (*(char *)((longlong)piStack_130 + (local_128 - 1)) == 'n') {
LAB_1407b6889:
          local_1d0 = piStack_130;
          local_1c8 = local_128 - 5;
          std::fs::DirEntry::path();
          country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf>();
          piVar3 = local_1d0;
          if (SBORROW8(0,(longlong)local_138)) {
            local_1b8 = piStack_130;
                    // WARNING: Subroutine does not return
            core::result::unwrap_failed();
          }
          local_148 = local_c8;
          local_158 = local_d8;
          uStack_150 = uStack_d0;
          local_168 = local_e8;
          uStack_160 = uStack_e0;
          local_178 = local_f8;
          uStack_170 = uStack_f0;
          local_188 = local_108;
          uStack_180 = uStack_100;
          local_198 = local_118;
          uStack_190 = uStack_110;
          local_1a8 = local_128;
          uStack_1a0 = uStack_120;
          local_1b8 = (int *)local_138;
          piStack_1b0 = piStack_130;
          if (6 < local_1c8) {
            if ((local_1c8 != 7) && (*(char *)((longlong)local_1d0 + 7) < -0x40)) {
              local_6d = 1;
                    // WARNING: Subroutine does not return
              core::str::slice_error_fail();
            }
            std::alloc::__default_lib_allocator::__rust_alloc();
            if (extraout_RAX == (int *)0x0) {
              local_6d = 1;
                    // WARNING: Subroutine does not return
              alloc::raw_vec::handle_error();
            }
            iVar1 = *piVar3;
            *(undefined4 *)((longlong)extraout_RAX + 3) = *(undefined4 *)((longlong)piVar3 + 3);
            *extraout_RAX = iVar1;
            iVar1 = *extraout_RAX;
            iVar2 = *(int *)((longlong)extraout_RAX + 3);
            std::alloc::__default_lib_allocator::__rust_dealloc();
            if (iVar2 == 0x656e696c && iVar1 == 0x6c74756f) {
              local_138 = (longlong *)0x3f8000003f800000;
              piStack_130 = (int *)CONCAT44(piStack_130._4_4_,0x3f800000);
              local_6d = 1;
              country_core::render::mesh::Mesh::add_color();
            }
          }
          local_c8 = local_148;
          local_d8 = local_158;
          uStack_d0 = uStack_150;
          local_e8 = local_168;
          uStack_e0 = uStack_160;
          local_f8 = local_178;
          uStack_f0 = uStack_170;
          local_108 = local_188;
          uStack_100 = uStack_180;
          local_118 = local_198;
          uStack_110 = uStack_190;
          local_128 = local_1a8;
          uStack_120 = uStack_1a0;
          local_138 = (longlong *)local_1b8;
          piStack_130 = piStack_1b0;
          local_6c8 = &local_1d0;
          local_6c0 = anyhow::context::impl$3::
                      fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
          alloc::fmt::format::format_inner();
          local_90 = local_6e0;
          local_98 = local_6d8;
          country_core::startup::load_mesh_into_library();
          if (local_90 != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
        }
      }
      else if (((uVar4 < 0x2e6a7370) - 1 | 1) == 0) goto LAB_1407b6889;
    }
    if (local_1f0 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    LOCK();
    *local_448 = *local_448 + -1;
    UNLOCK();
    if (*local_448 == 0) {
      alloc::sync::Arc::drop_slow<std::path::PathBuf,alloc::alloc::Global>();
    }
  } while( true );
}

