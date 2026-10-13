// addr=0x14089d300 name=get_keycode
// signature=void __cdecl get_keycode(void)


void __cdecl country_core::game_key_bindings::KeyBinding::get_keycode(void)

{
  byte bVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  char cVar4;
  int iVar5;
  undefined4 uVar6;
  undefined8 *puVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  longlong lVar10;
  code *pcVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  byte bVar14;
  undefined8 extraout_RAX;
  undefined8 uVar15;
  undefined8 extraout_RAX_00;
  undefined8 extraout_RAX_01;
  ulonglong extraout_RAX_02;
  longlong lVar16;
  byte bVar17;
  byte *in_RCX;
  byte *in_RDX;
  undefined8 unaff_RBX;
  longlong unaff_RBP;
  longlong lVar18;
  byte *unaff_RDI;
  byte *in_R8;
  longlong lVar19;
  longlong *plVar20;
  undefined4 in_XMM0_Da;
  undefined4 in_XMM0_Db;
  undefined4 in_XMM0_Dc;
  undefined4 in_XMM0_Dd;
  undefined4 uVar21;
  
  iVar5 = *(int *)(&DAT_142a9ab2c + (ulonglong)*in_RDX * 4);
  bVar14 = (byte)in_RCX;
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
  switch(*in_RDX) {
  case 0:
    in_R8 = in_R8 + 0x30;
    break;
  case 1:
    in_R8 = in_R8 + 0x3c;
    break;
  case 2:
    in_R8 = in_R8 + 0x18;
    break;
  case 3:
    in_R8 = in_R8 + 0x24;
    break;
  case 5:
    in_R8 = in_R8 + 0xc;
    break;
  case 6:
    in_R8 = in_R8 + 0x48;
    break;
  case 7:
    in_R8 = in_R8 + 0x54;
    break;
  case 8:
    in_R8 = in_R8 + 0x60;
    break;
  case 9:
    in_R8 = in_R8 + 0x6c;
    break;
  case 10:
    in_R8 = in_R8 + 0x78;
    break;
  case 0xb:
    in_R8 = in_R8 + 0x84;
    break;
  case 0xc:
    in_R8 = in_R8 + 0x90;
    break;
  case 0xd:
    in_R8 = in_R8 + 0x9c;
    break;
  case 0xe:
    in_R8 = in_R8 + 0xa8;
    break;
  case 0xf:
    in_R8 = in_R8 + 0xb4;
    break;
  case 0x10:
    in_R8 = in_R8 + 0xc0;
    break;
  case 0x11:
    in_R8 = in_R8 + 0xcc;
    break;
  case 0x12:
    in_R8 = in_R8 + 0xd8;
    break;
  case 0x13:
    in_R8 = in_R8 + 0xe4;
    break;
  case 0x14:
    in_R8 = in_R8 + 0xf0;
    break;
  case 0x15:
    in_R8 = in_R8 + 0xfc;
    break;
  case 0x16:
    in_R8 = in_R8 + 0x108;
    break;
  case 0x17:
    in_R8 = in_R8 + 0x114;
    break;
  case 0x18:
    in_R8 = in_R8 + 0x120;
    break;
  case 0x19:
    in_R8 = in_R8 + 300;
    break;
  case 0x1a:
    in_R8 = in_R8 + 0x138;
    break;
  case 0x1b:
    in_R8 = in_R8 + 0x144;
    break;
  case 0x1c:
    in_R8 = in_R8 + 0x150;
    break;
  case 0x1d:
    in_R8 = in_R8 + 0x15c;
    break;
  case 0x1e:
    in_R8 = in_R8 + 0x168;
    break;
  case 0x1f:
    in_R8 = in_R8 + 0x174;
    break;
  case 0x20:
    in_R8 = in_R8 + 0x180;
    break;
  case 0x21:
    in_R8 = in_R8 + 0x18c;
    break;
  case 0x22:
    in_R8 = in_R8 + 0x198;
    break;
  case 0x23:
    in_R8 = in_R8 + 0x1a4;
    break;
  case 0x24:
    in_R8 = in_R8 + 0x1b0;
    break;
  case 0x25:
    pcVar11 = (code *)swi(3);
    (*pcVar11)();
    return;
  case 0x26:
    *in_RCX = *in_RCX + bVar14;
    *(undefined8 *)(unaff_RBP + 0x100) = *(undefined8 *)(unaff_RBP + 0x70);
    *(undefined8 *)(unaff_RBP + 0x108) = *(undefined8 *)(unaff_RBP + 0x78);
    *(undefined8 *)(unaff_RBP + 0xf0) = *(undefined8 *)(unaff_RBP + 0x60);
    *(undefined8 *)(unaff_RBP + 0xf8) = *(undefined8 *)(unaff_RBP + 0x68);
    *(undefined8 *)(unaff_RBP + 0xe0) = *(undefined8 *)(unaff_RBP + 0x50);
    *(undefined8 *)(unaff_RBP + 0xe8) = *(undefined8 *)(unaff_RBP + 0x58);
    *(undefined8 *)(unaff_RBP + 0xd0) = *(undefined8 *)(unaff_RBP + 0x40);
    *(undefined8 *)(unaff_RBP + 0xd8) = *(undefined8 *)(unaff_RBP + 0x48);
    *(undefined8 *)(unaff_RBP + 0xc0) = *(undefined8 *)(unaff_RBP + 0x30);
    *(undefined8 *)(unaff_RBP + 200) = *(undefined8 *)(unaff_RBP + 0x38);
    *(byte **)(unaff_RBP + 0x138) = unaff_RDI;
    *(undefined1 *)(unaff_RBP + 0x186) = 0;
    asset_libraries::mesh_library::AssetMeshLibrary::add();
    *(undefined8 *)(unaff_RBP + 0x160) = extraout_RAX_01;
    uVar15 = extraout_RAX_01;
    if (*(longlong *)(unaff_RBP + 0x168) != 0) {
      *(int *)(unaff_RBP + 0x178) = (int)CONCAT71((int7)((ulonglong)extraout_RAX_01 >> 8),1);
      std::alloc::__default_lib_allocator::__rust_dealloc();
      goto LAB_14089e296;
    }
    goto LAB_14089e28e;
  case 0x27:
    *(int *)in_RCX = *(int *)in_RCX + (int)in_RCX;
    bVar1 = *unaff_RDI;
    bVar17 = (byte)(&DAT_142a9ab2c + iVar5);
    *unaff_RDI = *unaff_RDI + bVar17;
    *(char *)(unaff_RBP + 0x70) = *(char *)(unaff_RBP + 0x70) + bVar14 + CARRY1(bVar1,bVar17);
    *(undefined4 *)(unaff_RBP + 0x100) = in_XMM0_Da;
    *(undefined4 *)(unaff_RBP + 0x104) = in_XMM0_Db;
    *(undefined4 *)(unaff_RBP + 0x108) = in_XMM0_Dc;
    *(undefined4 *)(unaff_RBP + 0x10c) = in_XMM0_Dd;
    *(undefined8 *)(unaff_RBP + 0xf0) = *(undefined8 *)(unaff_RBP + 0x60);
    *(undefined8 *)(unaff_RBP + 0xf8) = *(undefined8 *)(unaff_RBP + 0x68);
    *(undefined8 *)(unaff_RBP + 0xe0) = *(undefined8 *)(unaff_RBP + 0x50);
    *(undefined8 *)(unaff_RBP + 0xe8) = *(undefined8 *)(unaff_RBP + 0x58);
    *(undefined8 *)(unaff_RBP + 0xd0) = *(undefined8 *)(unaff_RBP + 0x40);
    *(undefined8 *)(unaff_RBP + 0xd8) = *(undefined8 *)(unaff_RBP + 0x48);
    *(undefined8 *)(unaff_RBP + 0xc0) = *(undefined8 *)(unaff_RBP + 0x30);
    *(undefined8 *)(unaff_RBP + 200) = *(undefined8 *)(unaff_RBP + 0x38);
    *(byte **)(unaff_RBP + 0x138) = unaff_RDI;
    asset_libraries::mesh_library::AssetMeshLibrary::add();
    *(undefined8 *)(unaff_RBP + 0x160) = extraout_RAX;
    uVar15 = 0;
    if ((code *)**(undefined8 **)(unaff_RBP + 0x170) != (code *)0x0) {
      uVar15 = (*(code *)**(undefined8 **)(unaff_RBP + 0x170))(*(undefined8 *)(unaff_RBP + 0x168));
    }
    if (*(longlong *)(*(longlong *)(unaff_RBP + 0x170) + 8) != 0) {
      *(int *)(unaff_RBP + 0x178) = (int)CONCAT71((int7)((ulonglong)uVar15 >> 8),1);
code_r0x00014089e05f:
      std::alloc::__default_lib_allocator::__rust_dealloc();
      goto LAB_14089e296;
    }
    goto LAB_14089e28e;
  case 0x28:
    *(undefined4 *)(unaff_RBP + 0x110) = in_XMM0_Da;
    *(undefined4 *)(unaff_RBP + 0x114) = in_XMM0_Db;
    *(undefined4 *)(unaff_RBP + 0x118) = in_XMM0_Dc;
    *(undefined4 *)(unaff_RBP + 0x11c) = in_XMM0_Dd;
    *(undefined8 *)(unaff_RBP + 0x100) = *(undefined8 *)(unaff_RBP + -0x18);
    *(undefined8 *)(unaff_RBP + 0x108) = *(undefined8 *)(unaff_RBP + -0x10);
    *(undefined8 *)(unaff_RBP + 0xf0) = *(undefined8 *)(unaff_RBP + -0x28);
    *(undefined8 *)(unaff_RBP + 0xf8) = *(undefined8 *)(unaff_RBP + -0x20);
    *(undefined8 *)(unaff_RBP + 0xe0) = *(undefined8 *)(unaff_RBP + -0x38);
    *(undefined8 *)(unaff_RBP + 0xe8) = *(undefined8 *)(unaff_RBP + -0x30);
    *(undefined8 *)(unaff_RBP + 0xd0) = *(undefined8 *)(unaff_RBP + -0x48);
    *(undefined8 *)(unaff_RBP + 0xd8) = *(undefined8 *)(unaff_RBP + -0x40);
    *(undefined8 *)(unaff_RBP + 0xc0) = *(undefined8 *)(unaff_RBP + -0x58);
    *(undefined8 *)(unaff_RBP + 200) = *(undefined8 *)(unaff_RBP + -0x50);
    *(byte **)(unaff_RBP + 0x138) = unaff_RDI;
    asset_libraries::mesh_library::AssetMeshLibrary::add();
    *(undefined8 *)(unaff_RBP + 0x160) = extraout_RAX_00;
    uVar15 = extraout_RAX_00;
    if (*(longlong *)(unaff_RBP + 0x170) != 0) {
      *(int *)(unaff_RBP + 0x178) = (int)CONCAT71((int7)((ulonglong)extraout_RAX_00 >> 8),1);
      goto code_r0x00014089e05f;
    }
LAB_14089e28e:
    *(int *)(unaff_RBP + 0x178) = (int)CONCAT71((int7)((ulonglong)uVar15 >> 8),1);
LAB_14089e296:
    bevy_ecs::system::commands::Commands::spawn_empty();
    lVar19 = *(longlong *)(unaff_RBP + 0x158);
    if (*(char *)(lVar19 + 0x50) == '\0') {
      uVar21 = 0x3f800000;
      cVar4 = *(char *)(lVar19 + 0x40);
    }
    else {
      uVar21 = *(undefined4 *)(lVar19 + 0x54);
      cVar4 = *(char *)(lVar19 + 0x40);
    }
    if (cVar4 == '\0') {
      *(undefined1 *)(lVar19 + 0x40) = 0;
      *(undefined4 *)(lVar19 + 0x44) = uVar21;
      uVar21 = 0;
    }
    uVar15 = *(undefined8 *)(lVar19 + 0x68);
    asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name();
    if ((extraout_RAX_02 & 1) != 0) {
      plVar20 = *(longlong **)(unaff_RBP + 200);
      if (plVar20 == (longlong *)0x0) {
        plVar20 = *(longlong **)(unaff_RBP + 0xd0);
      }
      puVar7 = *(undefined8 **)(unaff_RBP + 0x158);
      uVar12 = *puVar7;
      uVar13 = puVar7[1];
      uVar2 = *(undefined4 *)(puVar7 + 0xf);
      uVar6 = *(undefined4 *)(puVar7 + 9);
      uVar3 = *(undefined4 *)((longlong)puVar7 + 0x4c);
      uVar8 = puVar7[8];
      uVar9 = *(undefined8 *)(unaff_RBP + 0xe0);
      lVar19 = plVar20[2];
      if ((ulonglong)(*plVar20 - lVar19) < 0x98) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar18 = *(longlong *)(unaff_RBP + 0x158);
      lVar16 = plVar20[1];
      *(code **)(lVar16 + lVar19) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::DrawableMeshBundle,country_core::systems::ui::signifiers::SignifierAnimation>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      puVar7 = (undefined8 *)(lVar16 + 8 + lVar19);
      *puVar7 = uVar12;
      puVar7[1] = uVar13;
      *(undefined8 *)(lVar16 + 0x18 + lVar19) = *(undefined8 *)(lVar18 + 0x7c);
      *(undefined4 *)(lVar16 + 0x20 + lVar19) = *(undefined4 *)(lVar18 + 0x84);
      *(undefined4 *)(lVar16 + 0x24 + lVar19) = uVar21;
      *(undefined4 *)(lVar16 + 0x28 + lVar19) = uVar21;
      *(undefined4 *)(lVar16 + 0x2c + lVar19) = uVar21;
      *(undefined8 *)(lVar16 + 0x38 + lVar19) = *(undefined8 *)(unaff_RBP + 0x160);
      *(undefined8 *)(lVar16 + 0x40 + lVar19) = uVar15;
      *(undefined4 *)(lVar16 + 0x48 + lVar19) = uVar6;
      *(undefined4 *)(lVar16 + 0x4c + lVar19) = uVar3;
      *(undefined8 *)(lVar16 + 0x50 + lVar19) = uVar8;
      *(undefined4 *)(lVar16 + 0x58 + lVar19) = uVar2;
      *(undefined8 *)(lVar16 + 0x5c + lVar19) = 0;
      *(undefined1 *)(lVar16 + 0x68 + lVar19) = 0;
      *(undefined8 *)(lVar16 + 0x78 + lVar19) = uVar9;
      *(code **)(lVar16 + 0x88 + lVar19) = bevy_ecs::error::handler::panic;
      lVar16 = lVar19 + 0x98;
      plVar20[2] = lVar16;
      if (*(char *)(lVar18 + 0x89) == '\0') {
        if (*(char *)(lVar18 + 0x8a) == '\0') {
          if ((ulonglong)(*plVar20 - lVar16) < 0x20) {
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
          }
          lVar16 = plVar20[1];
          *(code **)(lVar16 + 0x98 + lVar19) =
               core::ops::function::FnOnce::
               call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::UiPass>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
          ;
          *(code **)(lVar16 + 0xa0 + lVar19) = bevy_ecs::error::handler::panic;
          *(undefined8 *)(lVar16 + 0xa8 + lVar19) = uVar9;
          *(undefined1 *)(lVar16 + 0xb0 + lVar19) = 0;
          lVar16 = lVar19 + 0xb8;
          plVar20[2] = lVar16;
          lVar18 = 0xd8;
          if ((ulonglong)(*plVar20 - lVar16) < 0x20) {
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
          }
        }
        else {
          lVar18 = 0xb8;
          if ((ulonglong)(*plVar20 - lVar16) < 0x20) {
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
          }
        }
      }
      else {
        if ((ulonglong)(*plVar20 - lVar16) < 0x20) {
          alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        }
        lVar16 = plVar20[1];
        *(code **)(lVar16 + 0x98 + lVar19) =
             core::ops::function::FnOnce::
             call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::WireframePass>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
        ;
        *(code **)(lVar16 + 0xa0 + lVar19) = bevy_ecs::error::handler::panic;
        *(undefined8 *)(lVar16 + 0xa8 + lVar19) = uVar9;
        *(undefined1 *)(lVar16 + 0xb0 + lVar19) = 0;
        lVar16 = lVar19 + 0xb8;
        plVar20[2] = lVar16;
        lVar18 = 0xd8;
        if ((ulonglong)(*plVar20 - lVar16) < 0x20) {
          alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        }
      }
      lVar10 = plVar20[1];
      *(code **)(lVar10 + lVar16) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::DontCastShadow>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      *(code **)(lVar10 + 8 + lVar16) = bevy_ecs::error::handler::panic;
      *(undefined8 *)(lVar10 + 0x10 + lVar16) = uVar9;
      *(undefined1 *)(lVar10 + 0x18 + lVar16) = 0;
      lVar19 = lVar19 + lVar18;
      plVar20[2] = lVar19;
      lVar16 = *(longlong *)(unaff_RBP + 0x158);
      if ((*(byte *)(lVar16 + 0x58) & 1) != 0) {
        uVar21 = *(undefined4 *)(lVar16 + 0x5c);
        if ((ulonglong)(*plVar20 - lVar19) < 0x28) {
          alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        }
        lVar16 = plVar20[1];
        *(code **)(lVar16 + lVar19) =
             core::ops::function::FnOnce::
             call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::systems::ui::signifiers::SignifierMinimumLifespan>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
        ;
        *(code **)(lVar16 + 8 + lVar19) = bevy_ecs::error::handler::panic;
        *(undefined8 *)(lVar16 + 0x10 + lVar19) = uVar9;
        *(undefined4 *)(lVar16 + 0x18 + lVar19) = 0;
        *(undefined4 *)(lVar16 + 0x1c + lVar19) = uVar21;
        *(undefined1 *)(lVar16 + 0x20 + lVar19) = 0;
        lVar19 = lVar19 + 0x28;
        plVar20[2] = lVar19;
        lVar16 = *(longlong *)(unaff_RBP + 0x158);
      }
      if (*(char *)(lVar16 + 0x88) != '\0') {
        if ((ulonglong)(*plVar20 - lVar19) < 0x20) {
          alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        }
        lVar16 = plVar20[1];
        *(code **)(lVar16 + lVar19) =
             core::ops::function::FnOnce::
             call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::systems::ui::signifiers::SignifierParentToVolumeOrigin>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
        ;
        *(code **)(lVar16 + 8 + lVar19) = bevy_ecs::error::handler::panic;
        *(undefined8 *)(lVar16 + 0x10 + lVar19) = uVar9;
        *(undefined1 *)(lVar16 + 0x18 + lVar19) = 0;
        plVar20[2] = lVar19 + 0x20;
      }
      if (*(char *)(unaff_RBP + 0x178) != '\0') {
        plVar20 = *(longlong **)(*(longlong *)(unaff_RBP + 0xb0) + 0x18);
        LOCK();
        lVar19 = *plVar20;
        *plVar20 = *plVar20 + 1;
        UNLOCK();
        if (*plVar20 == 0 || SCARRY8(lVar19,1) != *plVar20 < 0) {
                    // WARNING: Does not return
          pcVar11 = (code *)invalidInstructionException();
          (*pcVar11)();
        }
        bevy_ecs::system::commands::EntityCommands::
        insert<country_core::components::transient_mesh::TransientMesh>();
      }
      puVar7 = *(undefined8 **)(unaff_RBP + 0xa8);
      puVar7[4] = *(undefined8 *)(unaff_RBP + 0xe0);
      uVar15 = *(undefined8 *)(unaff_RBP + 0xc0);
      uVar8 = *(undefined8 *)(unaff_RBP + 200);
      uVar9 = *(undefined8 *)(unaff_RBP + 0xd8);
      puVar7[2] = *(undefined8 *)(unaff_RBP + 0xd0);
      puVar7[3] = uVar9;
      *puVar7 = uVar15;
      puVar7[1] = uVar8;
      if (*(longlong *)(*(longlong *)(unaff_RBP + 0x158) + 0x60) != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      return;
    }
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  case 0x29:
    bVar1 = *in_RCX;
    *in_RCX = *in_RCX + bVar14;
    if (CARRY1(bVar1,bVar14)) {
      in_R8 = in_RCX;
    }
    *(undefined **)(unaff_RBP + 0xa8) = &DAT_142a9ab2c + iVar5;
    iVar5 = *(int *)(&DAT_142a9abc0 + (longlong)in_R8 * 4);
    *(undefined **)(unaff_RBP + 0x158) = &DAT_142a9ab2c;
    *(undefined8 *)(unaff_RBP + 0xb0) = unaff_RBX;
                    // WARNING: Could not recover jumptable at 0x00014089dddf. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(&DAT_142a9abc0 + iVar5))(&DAT_142a9abc0 + iVar5);
    return;
  }
  *(undefined4 *)(in_RCX + 8) = *(undefined4 *)(in_R8 + 8);
  *(undefined8 *)in_RCX = *(undefined8 *)in_R8;
  return;
}


