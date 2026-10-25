// 0x140b46b00 refresh_session_list
// signature: void __cdecl refresh_session_list(void)
// callees: [{"name": "__rust_dealloc", "address": "140613c20"}, {"name": "driftsort_main<country_core::resources::session::SavedSession,alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::resources::session::SavedSession,std::time::SystemTime,country_core::resources::session::load_saved_session_list::closure_env$1>,alloc::vec::Vec<country_core::resources::session::SavedSession,alloc::alloc::Global>_>", "address": "140b4c1c0"}, {"name": "insertion_sort_shift_left<country_core::resources::session::SavedSession,alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::resources::session::SavedSession,std::time::SystemTime,country_core::resources::session::load_saved_session_list::closure_env$1>_>", "address": "141339540"}, {"name": "memcpy", "address": "142923c00"}, {"name": "readdir", "address": "140d68f80"}, {"name": "saves", "address": "140b9db60"}, {"name": "spec_extend<country_core::resources::session::SavedSession,core::iter::adapters::filter_map::FilterMap<std::fs::ReadDir,country_core::resources::session::load_saved_session_list::closure_env$0>,alloc::alloc::Global>", "address": "1412f9a80"}]


void __cdecl country_core::resources::session::refresh_session_list(void)

{
  undefined4 uVar1;
  longlong lVar2;
  ulonglong uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined4 *puVar7;
  longlong *in_RCX;
  ulonglong uVar8;
  undefined8 *in_RDX;
  undefined4 *puVar9;
  longlong local_538;
  longlong local_530;
  undefined1 local_528 [608];
  longlong *local_2c8;
  longlong local_2c0;
  undefined1 local_2b8 [608];
  undefined8 *local_58;
  undefined8 local_50;
  longlong local_48;
  undefined8 local_40;
  undefined8 *local_38;
  undefined8 local_30;
  
  local_30 = 0xfffffffffffffffe;
  *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
  lVar2 = *in_RCX;
  local_50 = *in_RDX;
  *(undefined8 *)(lVar2 + 0x10) = 0;
  paths::saves();
  std::sys::pal::windows::fs::readdir();
  if (local_538 == 2) {
    if (((uint)local_530 & 3) == 1) {
      local_48 = local_530 + -1;
      local_40 = *(undefined8 *)(local_530 + -1);
      local_38 = *(undefined8 **)(local_530 + 7);
      if ((code *)*local_38 != (code *)0x0) {
        (*(code *)*local_38)(local_40);
      }
      if (local_38[1] != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      std::alloc::__default_lib_allocator::__rust_dealloc();
      return;
    }
  }
  else {
    memcpy(local_2b8,local_528,0x260);
    local_2c8 = (longlong *)local_538;
    local_2c0 = local_530;
    local_58 = &local_50;
    alloc::vec::spec_extend::impl$0::
    spec_extend<country_core::resources::session::SavedSession,core::iter::adapters::filter_map::FilterMap<std::fs::ReadDir,country_core::resources::session::load_saved_session_list::closure_env_0>,alloc::alloc::Global>
              ();
    if (1 < *(ulonglong *)(lVar2 + 0x10)) {
      local_2c8 = &local_538;
      if (*(ulonglong *)(lVar2 + 0x10) < 0x15) {
        core::slice::sort::shared::smallsort::
        insertion_sort_shift_left<country_core::resources::session::SavedSession,alloc::slice::impl_0::sort_by_key::closure_env_0<country_core::resources::session::SavedSession,std::time::SystemTime,country_core::resources::session::load_saved_session_list::closure_env_1>_>
                  ();
        uVar3 = *(ulonglong *)(lVar2 + 0x10);
      }
      else {
        core::slice::sort::stable::
        driftsort_main<country_core::resources::session::SavedSession,alloc::slice::impl_0::sort_by_key::closure_env_0<country_core::resources::session::SavedSession,std::time::SystemTime,country_core::resources::session::load_saved_session_list::closure_env_1>,alloc::vec::Vec<country_core::resources::session::SavedSession,alloc::alloc::Global>_>
                  ();
        uVar3 = *(ulonglong *)(lVar2 + 0x10);
      }
      if (1 < uVar3) {
        uVar8 = uVar3 >> 1;
        puVar9 = (undefined4 *)(*(longlong *)(lVar2 + 8) + uVar3 * 0x1c + -4);
        puVar7 = (undefined4 *)(*(longlong *)(lVar2 + 8) + 0x18);
        do {
          uVar4 = *(undefined8 *)(puVar7 + -6);
          uVar5 = *(undefined8 *)(puVar7 + -4);
          uVar6 = *(undefined8 *)(puVar9 + -4);
          *(undefined8 *)(puVar7 + -6) = *(undefined8 *)(puVar9 + -6);
          *(undefined8 *)(puVar7 + -4) = uVar6;
          *(undefined8 *)(puVar9 + -6) = uVar4;
          *(undefined8 *)(puVar9 + -4) = uVar5;
          uVar4 = *(undefined8 *)(puVar7 + -2);
          *(undefined8 *)(puVar7 + -2) = *(undefined8 *)(puVar9 + -2);
          *(undefined8 *)(puVar9 + -2) = uVar4;
          uVar1 = *puVar7;
          *puVar7 = *puVar9;
          *puVar9 = uVar1;
          puVar7 = puVar7 + 7;
          puVar9 = puVar9 + -7;
          uVar8 = uVar8 - 1;
        } while (uVar8 != 0);
      }
    }
  }
  return;
}

