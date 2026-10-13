// name: add_history_edit_to_save_file size: 694
// void __cdecl add_history_edit_to_save_file(void)

// WARNING: Removing unreachable block (ram,0x000140a43af7)
// WARNING: Removing unreachable block (ram,0x000140a43b04)
// WARNING: Removing unreachable block (ram,0x000140a43b08)
// WARNING: Removing unreachable block (ram,0x000140a43b15)
// WARNING: Removing unreachable block (ram,0x000140a43b24)
// WARNING: Removing unreachable block (ram,0x000140a43bcc)
// WARNING: Removing unreachable block (ram,0x000140a43b30)
// WARNING: Removing unreachable block (ram,0x000140a43bff)
// WARNING: Removing unreachable block (ram,0x000140a43b44)
// WARNING: Removing unreachable block (ram,0x000140a43b5a)
// WARNING: Removing unreachable block (ram,0x000140a43b5e)

void __cdecl country_core::systems::save::save_game::add_history_edit_to_save_file(void)

{
  longlong lVar1;
  undefined8 uVar2;
  ulonglong uVar3;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *in_RCX;
  ulonglong uVar4;
  longlong *in_R8;
  ulonglong uVar5;
  size_t _Size;
  ulonglong uVar6;
  longlong lVar7;
  
  lVar1 = *in_RCX;
  uVar2 = *(undefined8 *)(lVar1 + 0x38);
  lVar7 = in_R8[2];
  if (lVar7 == *in_R8) {
    alloc::raw_vec::RawVec::
    grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
              ();
  }
  *(undefined8 *)(in_R8[1] + lVar7 * 8) = uVar2;
  in_R8[2] = lVar7 + 1;
  if (lVar7 == 0) goto LAB_140a43a7b;
  uVar3 = *(ulonglong *)(lVar1 + 0x38);
  uVar6 = uVar3 + 2;
  uVar5 = 0xffffffffffffffff;
  if (uVar3 < 0xfffffffffffffffe) {
    uVar5 = uVar6;
  }
  uVar4 = *(ulonglong *)(lVar1 + 0x30);
  if ((*(ulonglong *)(lVar1 + 0x20) < uVar5) &&
     (*(ulonglong *)(lVar1 + 0x20) - uVar4 < uVar5 - uVar4)) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    uVar4 = *(ulonglong *)(lVar1 + 0x30);
    lVar7 = *(longlong *)(lVar1 + 0x28);
    _Size = uVar3 - uVar4;
    if (uVar4 <= uVar3 && _Size != 0) {
LAB_140a43a45:
      memset((void *)(uVar4 + lVar7),0,_Size);
      *(ulonglong *)(lVar1 + 0x30) = uVar3;
      uVar4 = uVar3;
    }
  }
  else {
    lVar7 = *(longlong *)(lVar1 + 0x28);
    _Size = uVar3 - uVar4;
    if (uVar4 <= uVar3 && _Size != 0) goto LAB_140a43a45;
  }
  *(undefined2 *)(lVar7 + uVar3) = 0xa2c;
  if (uVar4 < uVar6) {
    *(ulonglong *)(lVar1 + 0x30) = uVar6;
  }
  *(ulonglong *)(lVar1 + 0x38) = uVar6;
  shadow_file::ShadowFile::mark_dirty();
LAB_140a43a7b:
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == 0) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  resources::history::_::impl$0::
  serialize<ref_mut_<serde_json::ser::Serializer<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>_>
            ();
  if (extraout_RAX_00 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  shadow_file::ShadowFile::mark_dirty();
  std::alloc::__default_lib_allocator::__rust_dealloc();
  return;
}


