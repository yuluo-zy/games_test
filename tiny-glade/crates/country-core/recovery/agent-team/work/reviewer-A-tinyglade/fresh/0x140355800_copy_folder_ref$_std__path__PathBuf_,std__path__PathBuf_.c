// copy_folder<ref$<std::path::PathBuf>,std::path::PathBuf> @ 0x140355800 size=1239
// sig: void __cdecl copy_folder<ref$<std::path::PathBuf>,std::path::PathBuf>(void)


void __cdecl tiny_glade::fs_util::copy_folder<ref_<std::path::PathBuf>,std::path::PathBuf>(void)

{
  longlong *plVar1;
  longlong *plVar2;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong *in_RDX;
  ulonglong local_838;
  longlong *local_830;
  longlong *local_828;
  undefined1 local_820 [600];
  longlong *local_5c8;
  longlong *local_5c0;
  undefined1 local_5b8 [608];
  longlong *local_358;
  longlong *local_350;
  longlong *local_348 [76];
  longlong local_e8;
  longlong local_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  uint local_98;
  uint uStack_94;
  uint local_90;
  longlong local_78;
  undefined8 uStack_70;
  undefined8 local_68;
  undefined8 uStack_60;
  longlong *local_58;
  undefined1 local_4a;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  local_58 = in_RDX;
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  local_358 = (longlong *)CONCAT71(local_358._1_7_,1);
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  std::fs::DirBuilder::_create();
  if (extraout_RAX == 0) {
    std::sys::pal::windows::fs::readdir();
    if ((int)local_358 == 2) {
      local_830 = local_350;
      local_838 = 2;
    }
    else {
      memcpy(&local_838,&local_358,0x270);
    }
    anyhow::context::impl$0::
    with_context<std::fs::ReadDir,std::io::error::Error,alloc::string::String,tiny_glade::fs_util::copy_folder::closure_env_0<std::path::PathBuf,std::path::PathBuf>_>
              ();
    plVar2 = local_5c0;
    plVar1 = local_5c8;
    if (local_5c8 != (longlong *)0x2) {
      memcpy(local_348,local_5b8,0x260);
      local_358 = plVar1;
      local_350 = plVar2;
      while (std::fs::impl$30::next(), plVar2 = local_828, plVar1 = local_830, (local_838 & 1) != 0)
      {
        if (local_830 == (longlong *)0x0) {
          anyhow::error::impl$1::from<std::io::error::Error>();
LAB_140355c77:
          if (local_358 != (longlong *)0x0) {
            std::sys::pal::windows::fs::impl$7::drop();
          }
          LOCK();
          *local_348[0] = *local_348[0] + -1;
          UNLOCK();
          if (*local_348[0] == 0) {
            alloc::sync::Arc::drop_slow<std::path::PathBuf,alloc::alloc::Global>();
          }
          goto LAB_140355ca6;
        }
        memcpy(local_5b8,local_820,0x248);
        local_5c8 = plVar1;
        local_5c0 = plVar2;
        std::fs::DirEntry::path();
        local_49 = 1;
        std::fs::DirEntry::file_type();
        if ((local_98 & 1) != 0) {
          local_49 = 1;
          anyhow::error::impl$1::from<std::io::error::Error>();
joined_r0x000140355c14:
          if (local_c8 != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
LAB_140355c5e:
          LOCK();
          *local_5c8 = *local_5c8 + -1;
          UNLOCK();
          if (*local_5c8 == 0) {
            alloc::sync::Arc::drop_slow<std::path::PathBuf,alloc::alloc::Global>();
          }
          goto LAB_140355c77;
        }
        if (((uStack_94 & 0x10) == 0) || ((uStack_94 & 0x400) != 0 && (local_90 & 0x20000000) != 0))
        {
          local_68 = local_b8;
          uStack_60 = uStack_b0;
          local_78 = local_c8;
          uStack_70 = uStack_c0;
          std::sys::os_str::wtf8::Buf::as_mut_slice();
          std::fs::DirEntry::file_name();
          std::sys::os_str::wtf8::Buf::as_mut_slice();
          std::path::Path::_join();
          if (CONCAT44(uStack_94,local_98) != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          std::sys::os_str::wtf8::Buf::as_mut_slice();
          std::sys::os_str::wtf8::Buf::as_mut_slice();
          std::sys::pal::windows::fs::copy();
          anyhow::context::impl$0::
          with_context<u64,std::io::error::Error,alloc::string::String,tiny_glade::fs_util::copy_folder::closure_env_1<std::path::PathBuf,std::path::PathBuf>_>
                    ();
          if (extraout_RAX_01 != 0) {
            local_c8 = local_78;
            if (local_e8 != 0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
              local_c8 = local_78;
            }
            goto joined_r0x000140355c14;
          }
          if (local_e8 != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          if (local_78 != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
        }
        else {
          local_68 = local_b8;
          uStack_60 = uStack_b0;
          local_78 = local_c8;
          uStack_70 = uStack_c0;
          local_4a = 1;
          std::sys::os_str::wtf8::Buf::as_mut_slice();
          local_4a = 1;
          std::fs::DirEntry::file_name();
          std::sys::os_str::wtf8::Buf::as_mut_slice();
          std::path::Path::_join();
          if (CONCAT44(uStack_94,local_98) != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          local_4a = 0;
          copy_folder<std::path::PathBuf,std::path::PathBuf>();
          if (extraout_RAX_00 != 0) goto LAB_140355c5e;
        }
        LOCK();
        *local_5c8 = *local_5c8 + -1;
        UNLOCK();
        if (*local_5c8 == 0) {
          alloc::sync::Arc::drop_slow<std::path::PathBuf,alloc::alloc::Global>();
        }
      }
      if (local_358 != (longlong *)0x0) {
        std::sys::pal::windows::fs::impl$7::drop();
      }
      LOCK();
      *local_348[0] = *local_348[0] + -1;
      UNLOCK();
      if (*local_348[0] == 0) {
        alloc::sync::Arc::drop_slow<std::path::PathBuf,alloc::alloc::Global>();
      }
    }
  }
  else {
    anyhow::error::impl$1::from<std::io::error::Error>();
  }
LAB_140355ca6:
  if (*local_58 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}


