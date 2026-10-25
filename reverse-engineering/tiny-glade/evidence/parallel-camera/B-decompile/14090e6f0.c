
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl country_core::systems::camera_rig::MainCamera::match_free_camera_to_normal(void)

{
  undefined4 uVar1;
  undefined8 uVar2;
  longlong lVar3;
  longlong lVar4;
  longlong lVar5;
  undefined8 *puVar6;
  longlong lVar7;
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  longlong in_RCX;
  longlong lVar8;
  undefined8 *puVar9;
  undefined4 uStack_b4;
  longlong local_a8;
  undefined8 *puStack_a0;
  longlong local_98;
  undefined4 uStack_90;
  undefined8 local_88;
  undefined8 uStack_80;
  undefined8 local_78;
  undefined4 local_70;
  longlong *local_68;
  undefined8 local_60;
  
  local_60 = 0xfffffffffffffffe;
  uVar2 = *(undefined8 *)(in_RCX + 0x138);
  uVar1 = *(undefined4 *)(in_RCX + 0x140);
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
    alloc::alloc::handle_alloc_error();
  }
  *extraout_RAX = uVar2;
  *(undefined4 *)(extraout_RAX + 1) = uVar1;
  local_68 = extraout_RAX;
  alloc::raw_vec::RawVec::
  grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
            ();
  _DAT_00000008 = local_68;
  _DAT_00000010 = &anon_f854cc98b5e1efe54f6ab98d53b9f85b_26_llvm_5637972437638668905;
  local_a8 = 0;
  puStack_a0 = (undefined8 *)&DAT_00000008;
  local_98 = 1;
  uVar2 = *(undefined8 *)(in_RCX + 0x150);
  std::alloc::__default_lib_allocator::__rust_alloc();
  lVar8 = local_98;
  if (extraout_RAX_00 != (undefined8 *)0x0) {
    *extraout_RAX_00 = CONCAT44((int)uVar2,(int)((ulonglong)uVar2 >> 0x20));
    local_68 = extraout_RAX_00;
    if (local_98 == local_a8) {
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                ();
    }
    puStack_a0[lVar8 * 2] = local_68;
    puStack_a0[lVar8 * 2 + 1] = &anon_f854cc98b5e1efe54f6ab98d53b9f85b_30_llvm_5637972437638668905;
    local_78 = lVar8 + 1;
    local_88 = local_a8;
    uStack_80 = puStack_a0;
    std::alloc::__default_lib_allocator::__rust_alloc();
    lVar8 = local_78;
    if (extraout_RAX_01 != (undefined8 *)0x0) {
      *extraout_RAX_01 = 0;
      extraout_RAX_01[1] = 0;
      *(undefined4 *)(extraout_RAX_01 + 4) = 0;
      extraout_RAX_01[6] = __xmm_00000000000000003f8000003f800000;
      *(undefined4 *)(extraout_RAX_01 + 7) = 0x3f800000;
      local_68 = extraout_RAX_01;
      if (local_78 == local_88) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      puVar6 = uStack_80;
      lVar4 = local_88;
      uStack_80[lVar8 * 2] = local_68;
      uStack_80[lVar8 * 2 + 1] = &anon_f854cc98b5e1efe54f6ab98d53b9f85b_28_llvm_5637972437638668905;
      lVar8 = lVar8 + 1;
      local_78 = CONCAT44(_UNK_142aece54,_UNK_142aece50);
      local_70 = _UNK_142aece58;
      local_88 = CONCAT44(_UNK_142aece44,
                          _anon_f854cc98b5e1efe54f6ab98d53b9f85b_17_llvm_5637972437638668905);
      uStack_80 = (undefined8 *)CONCAT44(_DAT_142aece4c,_UNK_142aece48);
      if (lVar8 != 0) {
        puVar9 = puVar6;
        do {
          (**(code **)(puVar9[1] + 0x30))(&local_a8,*puVar9,&local_88,0);
          puVar9 = puVar9 + 2;
          local_70 = uStack_90;
          local_78 = local_98;
          local_88 = local_a8;
          uStack_80 = puStack_a0;
        } while (puVar9 != puVar6 + lVar8 * 2);
      }
      lVar7 = local_78;
      puVar9 = uStack_80;
      lVar5 = local_88;
      lVar3 = CONCAT44(uStack_b4,local_70);
      local_68 = (longlong *)(in_RCX + 0x270);
      core::ptr::
      drop_in_place<alloc::vec::Vec<alloc::boxed::Box<dyn_<bevy_app::plugin::Plugin>,alloc::alloc::Global>,alloc::alloc::Global>_>
                ();
      local_68[6] = lVar3;
      local_68[4] = (longlong)puVar9;
      local_68[5] = lVar7;
      local_68[2] = lVar8;
      local_68[3] = lVar5;
      *local_68 = lVar4;
      local_68[1] = (longlong)puVar6;
      return;
    }
                    /* WARNING: Subroutine does not return */
    alloc::alloc::handle_alloc_error();
  }
                    /* WARNING: Subroutine does not return */
  alloc::alloc::handle_alloc_error();
}

