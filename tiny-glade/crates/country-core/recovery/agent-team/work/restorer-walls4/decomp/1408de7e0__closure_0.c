// void __cdecl closure$0(void)

void __cdecl
country_core::systems::wall::shapes::rectangle::signifier_edit_dims_line::signifier_edit_dims::
closure_0(void)

{
  size_t sVar1;
  undefined4 uVar2;
  longlong lVar3;
  longlong lVar4;
  void *pvVar5;
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined1 *extraout_RAX_01;
  undefined1 *extraout_RAX_02;
  undefined8 *in_RCX;
  longlong in_RDX;
  longlong *plVar6;
  undefined1 *puVar7;
  undefined4 uStack_84;
  longlong *local_80;
  undefined8 local_70;
  
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  *(undefined4 *)((longlong)extraout_RAX_00 + 0xf) = 0x72656461;
  *extraout_RAX_00 = 0x635f786574726576;
  extraout_RAX_00[1] = 0x6168735f726f6c6f;
  uVar2 = *(undefined4 *)(in_RDX + 0x40);
  systems::ui::signifiers::SignifierEntityBuilder::build();
  plVar6 = (longlong *)CONCAT44(uStack_84,uVar2);
  if (plVar6 == (longlong *)0x0) {
    plVar6 = local_80;
  }
  lVar3 = plVar6[2];
  if ((ulonglong)(*plVar6 - lVar3) < 0x28) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  lVar4 = plVar6[1];
  *(code **)(lVar4 + lVar3) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<enum2_<country_core::components::drawable::DrawTopology>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  *(undefined8 *)(lVar4 + 8 + lVar3) = local_70;
  *(undefined8 *)(lVar4 + 0x10 + lVar3) = 0x4040000000000001;
  *(undefined1 *)(lVar4 + 0x18 + lVar3) = 0;
  *(code **)(lVar4 + 0x20 + lVar3) = bevy_ecs::error::handler::panic;
  plVar6[2] = lVar3 + 0x28;
  sVar1 = *(ulonglong *)(in_RDX + 0x10) * 8;
  if ((*(ulonglong *)(in_RDX + 0x10) >> 0x3d != 0) || (0x7ffffffffffffffc < sVar1)) {
LAB_1408deb2c:
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  pvVar5 = *(void **)(in_RDX + 8);
  if (sVar1 == 0) {
    puVar7 = &DAT_00000004;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    puVar7 = extraout_RAX_01;
    if (extraout_RAX_01 == (undefined1 *)0x0) goto LAB_1408deb2c;
  }
  memcpy(puVar7,pvVar5,sVar1);
  sVar1 = *(ulonglong *)(in_RDX + 0x28) * 4;
  if ((*(ulonglong *)(in_RDX + 0x28) >> 0x3e == 0) && (sVar1 < 0x7ffffffffffffffd)) {
    pvVar5 = *(void **)(in_RDX + 0x20);
    if (sVar1 == 0) {
      puVar7 = &DAT_00000004;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      puVar7 = extraout_RAX_02;
      if (extraout_RAX_02 == (undefined1 *)0x0) goto LAB_1408deb40;
    }
    memcpy(puVar7,pvVar5,sVar1);
    bevy_ecs::system::commands::EntityCommands::
    insert<country_core::systems::wall::shapes::rectangle::signifier_edit_dims_line::EditRectangleLinePreview>
              ();
    *extraout_RAX = local_70;
    *in_RCX = 1;
    in_RCX[1] = extraout_RAX;
    in_RCX[2] = 1;
    return;
  }
LAB_1408deb40:
                    // WARNING: Subroutine does not return
  alloc::raw_vec::handle_error();
}

