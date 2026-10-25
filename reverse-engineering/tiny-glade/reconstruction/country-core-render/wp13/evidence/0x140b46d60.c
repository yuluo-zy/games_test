// name: delete_session size: 1604
// void __cdecl delete_session(void)

void __cdecl country_core::resources::session::delete_session(void)

{
  longlong lVar1;
  size_t _Size;
  char cVar2;
  char extraout_AL;
  char extraout_AL_00;
  void *extraout_RAX;
  longlong extraout_RAX_00;
  undefined8 *in_RCX;
  ulonglong uVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  void *_Dst;
  ulonglong uVar6;
  ulonglong uVar7;
  bool bVar8;
  bool bVar9;
  longlong *local_178;
  code *pcStack_170;
  undefined4 local_148;
  undefined4 uStack_144;
  undefined4 uStack_140;
  undefined4 uStack_13c;
  undefined4 local_138;
  int iStack_134;
  longlong local_a8 [2];
  char local_98;
  char local_97;
  longlong local_90;
  longlong *local_88;
  code *pcStack_80;
  undefined8 local_78;
  undefined8 *local_68;
  undefined8 *local_60;
  undefined8 *local_58;
  undefined1 local_4a;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  lVar1 = in_RCX[1];
  uVar4 = *(ulonglong *)*in_RCX;
  uVar5 = 0;
  uVar6 = uVar4 - *(ulonglong *)(lVar1 + 0x18);
  if (uVar4 < *(ulonglong *)(lVar1 + 0x18)) {
    uVar6 = uVar5;
  }
  uVar3 = uVar4 - *(ulonglong *)(lVar1 + 0x38);
  if (uVar4 < *(ulonglong *)(lVar1 + 0x38)) {
    uVar3 = uVar5;
  }
  bVar8 = *(ulonglong *)(lVar1 + 0x30) < uVar3;
  uVar3 = *(ulonglong *)(lVar1 + 0x30) - uVar3;
  uVar4 = uVar3;
  if (bVar8) {
    uVar4 = uVar5;
  }
  bVar9 = *(ulonglong *)(lVar1 + 0x10) < uVar6;
  uVar6 = *(ulonglong *)(lVar1 + 0x10) - uVar6;
  uVar7 = uVar6;
  if (bVar9) {
    uVar7 = uVar5;
  }
  uVar4 = uVar4 + uVar7;
  if (!bVar9 && uVar6 != 0 || !bVar8 && uVar3 != 0) {
    uVar4 = uVar5;
  }
  *(ulonglong *)*in_RCX = *(longlong *)(lVar1 + 0x40) - uVar4;
  if ((bVar9 || uVar6 == 0) && (bVar8 || uVar3 == 0)) {
    return;
  }
  SessionId::save_folder_path();
  local_4a = 1;
  paths::deleted_saves();
  local_88 = (undefined8 *)0x0;
  pcStack_80 = (code *)0x1;
  local_78 = 0;
  uuid::fmt::impl$1::fmt();
  if (extraout_AL != '\0') {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  local_60 = local_88;
  local_58 = (undefined8 *)pcStack_80;
  std::path::Path::_join();
  if (local_60 != (undefined8 *)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  _Size = CONCAT44(iStack_134,local_138);
  if (iStack_134 < 0) {
LAB_140b47362:
    local_49 = 1;
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  if (_Size == 0) {
    _Dst = (void *)0x1;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    _Dst = extraout_RAX;
    if (extraout_RAX == (void *)0x0) goto LAB_140b47362;
  }
  memcpy(_Dst,(void *)CONCAT44(uStack_13c,uStack_140),_Size);
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  std::sys::pal::windows::fs::exists();
  if (_Size != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (local_98 == '\0') {
    if (local_97 == '\0') goto LAB_140b47193;
    local_60 = (undefined8 *)((ulonglong)local_60 & 0xffffffffffffff00);
    paths::deleted_saves();
    uuid::Uuid::new_v4();
    local_88 = (undefined8 *)0x0;
    pcStack_80 = (code *)0x1;
    local_78 = 0;
    uuid::fmt::impl$1::fmt();
    if (extraout_AL_00 != '\0') {
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    local_58 = local_88;
    local_68 = (undefined8 *)pcStack_80;
    std::path::Path::_join();
    if (local_58 != (undefined8 *)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    cVar2 = (char)local_60;
    if (CONCAT44(uStack_144,local_148) != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if ((cVar2 == '\0') || (((uint)local_90 & 3) != 1)) goto LAB_140b47193;
    local_68 = (undefined8 *)(local_90 + -1);
    local_60 = *(undefined8 **)(local_90 + -1);
    local_58 = *(undefined8 **)(local_90 + 7);
    if ((code *)*local_58 != (code *)0x0) {
      (*(code *)*local_58)(local_60);
    }
  }
  else {
    if (((uint)local_90 & 3) != 1) goto LAB_140b47193;
    local_68 = (undefined8 *)(local_90 + -1);
    local_60 = *(undefined8 **)(local_90 + -1);
    local_58 = *(undefined8 **)(local_90 + 7);
    if ((code *)*local_58 != (code *)0x0) {
      (*(code *)*local_58)(local_60);
    }
  }
  if (local_58[1] != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  std::alloc::__default_lib_allocator::__rust_dealloc();
LAB_140b47193:
  local_88 = local_178;
  pcStack_80 = pcStack_170;
  local_49 = 0;
  std::fs::rename<std::path::PathBuf,std::path::PathBuf>();
  if (extraout_RAX_00 != 0) {
    local_a8[0] = extraout_RAX_00;
    if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
      local_88 = local_a8;
      pcStack_80 = std::io::error::impl$0::fmt;
      ::toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      ::log::__private_api::impl$2::log();
    }
    if (((uint)local_a8[0] & 3) == 1) {
      local_68 = (undefined8 *)(local_a8[0] + -1);
      local_60 = *(undefined8 **)(local_a8[0] + -1);
      local_58 = *(undefined8 **)(local_a8[0] + 7);
      if ((code *)*local_58 != (code *)0x0) {
        (*(code *)*local_58)(local_60);
      }
      if (local_58[1] != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  return;
}


