// addr=0x1412a8e00 name=new
// signature=void __cdecl new(void)


void __cdecl country_core::startup::load_prefabs::PrefabWatcher::new(void)

{
  longlong lVar1;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *in_RCX;
  char in_DL;
  longlong local_88;
  longlong lStack_80;
  longlong local_78;
  longlong lStack_70;
  longlong local_68;
  longlong local_58;
  longlong lStack_50;
  longlong lStack_48;
  longlong lStack_40;
  longlong lStack_38;
  longlong local_30;
  
  lVar1 = 3;
  if (in_DL != '\0') {
    hotwatch::GenericHotwatch::new_with_custom_delay<notify::windows::ReadDirectoryChangesWatcher>()
    ;
    if (local_58 == 3) {
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    local_68 = local_30;
    local_78 = lStack_40;
    lStack_70 = lStack_38;
    local_88 = lStack_50;
    lStack_80 = lStack_48;
    lVar1 = local_58;
  }
  alloc::sync::impl$56::
  default<std::sync::poison::mutex::Mutex<std::collections::hash::set::HashSet<country_core::render::nani::gpu_world::GpuWorldSubsetKind,std::hash::random::RandomState>_>_>
            ();
  alloc::sync::impl$56::
  default<std::sync::poison::mutex::Mutex<std::collections::hash::set::HashSet<country_core::render::nani::gpu_world::GpuWorldSubsetKind,std::hash::random::RandomState>_>_>
            ();
  in_RCX[4] = lStack_70;
  in_RCX[5] = local_68;
  in_RCX[2] = lStack_80;
  in_RCX[3] = local_78;
  *in_RCX = lVar1;
  in_RCX[1] = local_88;
  in_RCX[6] = extraout_RAX;
  in_RCX[7] = extraout_RAX_00;
  return;
}


