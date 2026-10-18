// closure$0  size=1034
// void __cdecl closure$0(void)

void __cdecl
country_core::systems::wall::shapes::circle::ui_create_circle::circle_signifier::closure_0(void)

{
  size_t _Size;
  byte bVar1;
  longlong lVar2;
  longlong lVar3;
  undefined8 uVar4;
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  void *_Dst;
  undefined8 *in_RCX;
  undefined8 *in_RDX;
  undefined *puVar5;
  longlong *plVar6;
  undefined4 extraout_XMM0_Da;
  longlong *local_198;
  undefined8 *puStack_190;
  undefined *local_188;
  undefined8 uStack_180;
  undefined8 local_178;
  undefined8 uStack_170;
  undefined8 local_108;
  longlong *plStack_100;
  longlong *local_f8;
  undefined8 *puStack_f0;
  undefined *local_e8;
  undefined8 uStack_e0;
  undefined8 local_d8;
  undefined8 uStack_d0;
  undefined2 local_c8;
  undefined8 local_c4;
  undefined4 local_b8;
  undefined4 local_b0;
  size_t local_a8;
  size_t local_98;
  undefined4 local_90;
  undefined5 local_8c;
  undefined3 uStack_87;
  undefined5 uStack_84;
  undefined2 local_7f;
  undefined8 *local_68;
  undefined8 local_60;
  
  local_60 = 0xfffffffffffffffe;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  uVar4 = in_RDX[1];
  *extraout_RAX_00 = *in_RDX;
  extraout_RAX_00[1] = uVar4;
  local_188 = &DAT_142c82270;
  local_198 = (longlong *)0x8000000000000004;
  puStack_190 = extraout_RAX_00;
  local_68 = extraout_RAX;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_01 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  *(undefined4 *)((longlong)extraout_RAX_01 + 0xf) = 0x72656461;
  *extraout_RAX_01 = 0x635f786574726576;
  extraout_RAX_01[1] = 0x6168735f726f6c6f;
  local_f8 = local_198;
  puStack_f0 = puStack_190;
  local_e8 = local_188;
  uStack_e0 = uStack_180;
  local_d8 = local_178;
  uStack_d0 = uStack_170;
  local_8c = 0;
  uStack_87 = 0;
  uStack_84 = 0;
  local_108 = 0;
  plStack_100 = (longlong *)0x3f80000000000000;
  local_c8 = 0x100;
  local_c4 = 0x13f800000;
  local_b8 = 0;
  local_b0 = 0;
  local_a8 = 0x13;
  local_98 = 0x13;
  local_90 = 0;
  local_7f = 0;
  bVar1 = *(byte *)(in_RDX + 2);
  _Size = (ulonglong)(bVar1 ^ 1) * 8 + 0xb;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (_Dst != (void *)0x0) {
    puVar5 = &DAT_142c822a0;
    if (bVar1 != 0) {
      puVar5 = &DAT_142c822b3;
    }
    memcpy(_Dst,puVar5,_Size);
    std::alloc::__default_lib_allocator::__rust_dealloc();
    local_a8 = _Size;
    local_98 = _Size;
    memcpy(&local_198,&local_108,0x90);
    systems::ui::signifiers::SignifierEntityBuilder::build();
    puVar5 = local_e8;
    plVar6 = plStack_100;
    if (plStack_100 == (longlong *)0x0) {
      plVar6 = local_f8;
    }
    lVar2 = plVar6[2];
    if ((ulonglong)(*plVar6 - lVar2) < 0x28) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar3 = plVar6[1];
    *(code **)(lVar3 + lVar2) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<enum2_<country_core::components::drawable::DrawTopology>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(undefined **)(lVar3 + 8 + lVar2) = puVar5;
    *(undefined8 *)(lVar3 + 0x10 + lVar2) = 0x4040000000000001;
    *(undefined1 *)(lVar3 + 0x18 + lVar2) = 0;
    *(code **)(lVar3 + 0x20 + lVar2) = bevy_ecs::error::handler::panic;
    plVar6[2] = lVar2 + 0x28;
    ::utils::geometry::circle::Circle2d::perimeter();
    lVar2 = plVar6[2];
    if ((ulonglong)(*plVar6 - lVar2) < 0x20) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar3 = plVar6[1];
    *(code **)(lVar3 + lVar2) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::DashedLineLength>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar3 + 8 + lVar2) = bevy_ecs::error::handler::panic;
    *(undefined **)(lVar3 + 0x10 + lVar2) = puVar5;
    *(undefined4 *)(lVar3 + 0x18 + lVar2) = extraout_XMM0_Da;
    *(undefined1 *)(lVar3 + 0x1c + lVar2) = 0;
    plVar6[2] = lVar2 + 0x20;
    if ((ulonglong)(*plVar6 - (lVar2 + 0x20)) < 0x20) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar3 = plVar6[1];
    *(code **)(lVar3 + 0x20 + lVar2) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::MeshFollowTerrain>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar3 + 0x28 + lVar2) = bevy_ecs::error::handler::panic;
    *(undefined **)(lVar3 + 0x30 + lVar2) = puVar5;
    *(undefined1 *)(lVar3 + 0x38 + lVar2) = 0;
    plVar6[2] = lVar2 + 0x40;
    *local_68 = puVar5;
    *in_RCX = 1;
    in_RCX[1] = local_68;
    in_RCX[2] = 1;
    return;
  }
                    // WARNING: Subroutine does not return
  alloc::raw_vec::handle_error();
}



// CALLEES: [{'name': '__rust_alloc', 'address': '140613c10'}, {'name': '__rust_dealloc', 'address': '140613c20'}, {'name': 'build', 'address': '14089dd50'}, {'name': 'do_reserve_and_handle<alloc::alloc::Global>', 'address': '1428a9a60'}, {'name': 'handle_alloc_error', 'address': '1428d9000'}, {'name': 'handle_error', 'address': '1428d8fe3'}, {'name': 'memcpy', 'address': '142923c00'}, {'name': 'perimeter', 'address': '140c97970'}]