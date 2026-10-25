
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl country_core::systems::camera_rig::walk::WalkCamera::new(void)

{
  longlong lVar1;
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  longlong *in_RCX;
  undefined8 *puVar2;
  undefined8 *puVar3;
  undefined4 local_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined4 uStack_b8;
  undefined4 uStack_b4;
  undefined4 uStack_b0;
  longlong local_a8;
  undefined8 *puStack_a0;
  longlong local_98;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined4 uStack_78;
  undefined4 uStack_74;
  undefined8 local_68;
  undefined8 uStack_60;
  undefined8 local_58;
  undefined4 uStack_50;
  undefined8 *local_40;
  undefined8 local_38;
  
  local_38 = 0xfffffffffffffffe;
  local_a8 = 0;
  puStack_a0 = (undefined8 *)&DAT_00000008;
  local_98 = 0;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
    alloc::alloc::handle_alloc_error();
  }
  *extraout_RAX = __xmm_00000000000000003f80000000000000;
  *(undefined4 *)(extraout_RAX + 1) = 0;
  local_40 = extraout_RAX;
  alloc::raw_vec::RawVec::
  grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
            ();
  *puStack_a0 = local_40;
  puStack_a0[1] = &anon_f854cc98b5e1efe54f6ab98d53b9f85b_26_llvm_5637972437638668905;
  local_68 = local_a8;
  uStack_60 = puStack_a0;
  local_58 = 1;
  std::alloc::__default_lib_allocator::__rust_alloc();
  lVar1 = local_58;
  if (extraout_RAX_00 != (undefined8 *)0x0) {
    *extraout_RAX_00 = 0;
    local_40 = extraout_RAX_00;
    if (local_58 == local_68) {
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                ();
    }
    uStack_60[lVar1 * 2] = local_40;
    uStack_60[lVar1 * 2 + 1] = &anon_f854cc98b5e1efe54f6ab98d53b9f85b_30_llvm_5637972437638668905;
    uStack_b8 = _UNK_142aece50;
    uStack_b4 = _UNK_142aece54;
    uStack_b0 = _UNK_142aece58;
    local_c8 = _anon_f854cc98b5e1efe54f6ab98d53b9f85b_17_llvm_5637972437638668905;
    uStack_c4 = _UNK_142aece44;
    uStack_c0 = _UNK_142aece48;
    uStack_bc = _DAT_142aece4c;
    local_98 = lVar1 + 1;
    local_a8 = local_68;
    puStack_a0 = uStack_60;
    _uStack_90 = CONCAT44(_UNK_142aece44,
                          _anon_f854cc98b5e1efe54f6ab98d53b9f85b_17_llvm_5637972437638668905);
    uStack_88 = _UNK_142aece48;
    uStack_84 = _DAT_142aece4c;
    uStack_80 = _UNK_142aece50;
    uStack_7c = _UNK_142aece54;
    uStack_78 = _UNK_142aece58;
    if (local_98 != 0) {
      puVar2 = uStack_60 + local_98 * 2;
      puVar3 = uStack_60;
      local_58 = local_98;
      do {
        (**(code **)(puVar3[1] + 0x30))(&local_68,*puVar3,&local_c8,0);
        puVar3 = puVar3 + 2;
        local_c8 = (undefined4)local_68;
        uStack_c4 = local_68._4_4_;
        uStack_c0 = (undefined4)uStack_60;
        uStack_bc = uStack_60._4_4_;
        uStack_b8 = (undefined4)local_58;
        uStack_b4 = local_58._4_4_;
        uStack_b0 = uStack_50;
      } while (puVar3 != puVar2);
    }
    in_RCX[6] = CONCAT44(uStack_74,uStack_b0);
    in_RCX[4] = CONCAT44(uStack_bc,uStack_c0);
    in_RCX[5] = CONCAT44(uStack_b4,uStack_b8);
    in_RCX[2] = local_98;
    in_RCX[3] = CONCAT44(uStack_c4,local_c8);
    *in_RCX = local_a8;
    in_RCX[1] = (longlong)puStack_a0;
    *(undefined4 *)(in_RCX + 10) = 0;
    *(undefined1 *)((longlong)in_RCX + 0x54) = 0;
    *(undefined1 *)(in_RCX + 7) = 0x11;
    return;
  }
                    /* WARNING: Subroutine does not return */
  alloc::alloc::handle_alloc_error();
}

