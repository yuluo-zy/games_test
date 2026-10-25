
void __cdecl country_core::utils::debug_viz::impl$5::show::closure_1(void)

{
  size_t _Size;
  float fVar1;
  undefined4 uVar2;
  byte bVar3;
  longlong *plVar4;
  void *pvVar5;
  longlong lVar6;
  longlong lVar7;
  undefined1 auVar8 [16];
  undefined1 auVar9 [16];
  longlong *plVar10;
  ulonglong uVar11;
  undefined1 *extraout_RAX;
  ulonglong _Size_00;
  undefined1 *extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  void *_Dst;
  ulonglong *extraout_RAX_02;
  undefined8 *in_RCX;
  longlong in_RDX;
  char *_Src;
  longlong *plVar12;
  longlong *plVar13;
  longlong *local_1a8;
  undefined1 *puStack_1a0;
  longlong *local_198;
  ulonglong uStack_190;
  undefined1 *local_188;
  ulonglong uStack_180;
  undefined8 local_118;
  longlong *plStack_110;
  longlong *local_108;
  undefined1 *puStack_100;
  longlong *local_f8;
  ulonglong uStack_f0;
  undefined1 *local_e8;
  ulonglong uStack_e0;
  undefined2 local_d8;
  undefined8 local_d4;
  undefined4 local_c8;
  undefined4 local_c0;
  size_t local_b8;
  size_t local_a8;
  undefined4 local_a0;
  undefined5 local_9c;
  undefined3 uStack_97;
  undefined5 uStack_94;
  undefined1 local_8f;
  byte local_8e;
  undefined8 *local_78;
  undefined1 *local_70;
  longlong *local_68;
  undefined8 local_60;
  
  local_60 = 0xfffffffffffffffe;
  plVar4 = *(longlong **)(in_RDX + 0x10);
  auVar8._8_8_ = 0;
  auVar8._0_8_ = plVar4;
  uVar11 = SUB168(auVar8 * ZEXT816(0xc),0);
  if ((SUB168(auVar8 * ZEXT816(0xc),8) != 0) || (0x7ffffffffffffffc < uVar11)) {
LAB_140b05a09:
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  pvVar5 = *(void **)(in_RDX + 8);
  local_78 = in_RCX;
  if (uVar11 == 0) {
    local_70 = &DAT_00000004;
    local_68 = (longlong *)0x0;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    local_70 = extraout_RAX;
    local_68 = plVar4;
    if (extraout_RAX == (undefined1 *)0x0) goto LAB_140b05a09;
  }
  memcpy(local_70,pvVar5,uVar11);
  uVar11 = *(ulonglong *)(in_RDX + 0x28);
  auVar9._8_8_ = 0;
  auVar9._0_8_ = uVar11;
  _Size_00 = SUB168(auVar9 * ZEXT816(0xc),0);
  if ((SUB168(auVar9 * ZEXT816(0xc),8) == 0) && (_Size_00 < 0x7ffffffffffffffd)) {
    pvVar5 = *(void **)(in_RDX + 0x20);
    if (_Size_00 == 0) {
      local_188 = &DAT_00000004;
      uStack_190 = 0;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      local_188 = extraout_RAX_00;
      uStack_190 = uVar11;
      if (extraout_RAX_00 == (undefined1 *)0x0) goto LAB_140b05a1b;
    }
    memcpy(local_188,pvVar5,_Size_00);
    local_1a8 = local_68;
    puStack_1a0 = local_70;
    local_198 = plVar4;
    uStack_180 = uVar11;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_01 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    *(undefined4 *)((longlong)extraout_RAX_01 + 0xf) = 0x72656461;
    *extraout_RAX_01 = 0x635f786574726576;
    extraout_RAX_01[1] = 0x6168735f726f6c6f;
    local_108 = local_1a8;
    puStack_100 = puStack_1a0;
    local_f8 = local_198;
    uStack_f0 = uStack_190;
    local_e8 = local_188;
    uStack_e0 = uStack_180;
    local_9c = 0;
    uStack_97 = 0;
    uStack_94 = 0;
    local_8e = *(byte *)(in_RDX + 0x38) ^ 1;
    local_118 = 0;
    plStack_110 = (longlong *)0x3f80000000000000;
    local_d8 = 0x100;
    local_d4 = 0x13f800000;
    local_c8 = 0;
    local_c0 = 0;
    local_b8 = 0x13;
    local_a8 = 0x13;
    local_a0 = 0;
    local_8f = 0;
    bVar3 = *(byte *)(in_RDX + 0x39);
    _Size = (ulonglong)(bVar3 ^ 1) * 8 + 0xb;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (_Dst == (void *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    _Src = "vertex_color_shaderdashed_line";
    if (bVar3 != 0) {
      _Src = "dashed_line";
    }
    memcpy(_Dst,_Src,_Size);
    std::alloc::__default_lib_allocator::__rust_dealloc();
    local_b8 = _Size;
    local_a8 = _Size;
    memcpy(&local_1a8,&local_118,0x90);
    systems::ui::signifiers::SignifierEntityBuilder::build();
    plVar10 = local_f8;
    plVar12 = local_108;
    plVar4 = plStack_110;
    if (bVar3 != 0) {
      fVar1 = *(float *)(in_RDX + 0x30);
      plVar13 = plStack_110;
      if (plStack_110 == (longlong *)0x0) {
        plVar13 = local_108;
      }
      lVar6 = plVar13[2];
      if ((ulonglong)(*plVar13 - lVar6) < 0x20) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar7 = plVar13[1];
      *(code **)(lVar7 + lVar6) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::DashedLineLength>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      *(code **)(lVar7 + 8 + lVar6) = bevy_ecs::error::handler::panic;
      *(longlong **)(lVar7 + 0x10 + lVar6) = plVar10;
      *(float *)(lVar7 + 0x18 + lVar6) = fVar1 * 4.0;
      *(undefined1 *)(lVar7 + 0x1c + lVar6) = 0;
      plVar13[2] = lVar6 + 0x20;
    }
    uVar2 = *(undefined4 *)(in_RDX + 0x34);
    if (plVar4 != (longlong *)0x0) {
      plVar12 = plVar4;
    }
    lVar6 = plVar12[2];
    if ((ulonglong)(*plVar12 - lVar6) < 0x28) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar7 = plVar12[1];
    *(code **)(lVar7 + lVar6) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<enum2_<country_core::components::drawable::DrawTopology>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(longlong **)(lVar7 + 8 + lVar6) = plVar10;
    *(undefined4 *)(lVar7 + 0x10 + lVar6) = 1;
    *(undefined4 *)(lVar7 + 0x14 + lVar6) = uVar2;
    *(undefined1 *)(lVar7 + 0x18 + lVar6) = 0;
    *(code **)(lVar7 + 0x20 + lVar6) = bevy_ecs::error::handler::panic;
    plVar12[2] = lVar6 + 0x28;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_02 == (ulonglong *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    *extraout_RAX_02 = (ulonglong)plVar10;
    *local_78 = 1;
    local_78[1] = extraout_RAX_02;
    local_78[2] = 1;
    return;
  }
LAB_140b05a1b:
                    // WARNING: Subroutine does not return
  alloc::raw_vec::handle_error();
}

