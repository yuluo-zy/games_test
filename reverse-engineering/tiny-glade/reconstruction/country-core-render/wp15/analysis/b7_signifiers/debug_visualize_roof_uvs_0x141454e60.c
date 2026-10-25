// addr=0x141454e60 name=debug_visualize_roof_uvs
// signature=void __cdecl debug_visualize_roof_uvs(void)


// WARNING: Removing unreachable block (ram,0x0001414554dc)

void __cdecl country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs(void)

{
  uint *puVar1;
  longlong lVar2;
  int *piVar3;
  undefined8 *puVar4;
  char cVar5;
  uint uVar6;
  ulonglong *puVar7;
  longlong lVar8;
  longlong lVar9;
  code *pcVar10;
  longlong lVar11;
  longlong *extraout_RAX;
  longlong *extraout_RAX_00;
  longlong *plVar12;
  undefined8 extraout_RAX_01;
  ulonglong extraout_RAX_02;
  longlong *in_RCX;
  ulonglong uVar13;
  longlong *in_RDX;
  ulonglong uVar14;
  undefined *puVar15;
  longlong lVar16;
  uint *puVar17;
  ulonglong uVar18;
  longlong *in_R9;
  ulonglong uVar19;
  longlong *plVar20;
  longlong *plVar21;
  int iVar22;
  int iVar23;
  int iVar24;
  int iVar25;
  longlong *in_stack_00000028;
  longlong local_d8;
  longlong local_b8;
  
  lVar16 = in_R9[2];
  in_R9[2] = 0;
  if (lVar16 != 0) {
    lVar16 = lVar16 << 3;
    puVar7 = (ulonglong *)in_R9[1];
    do {
      if (*puVar7 >> 0x20 == 0) break;
      bevy_ecs::system::commands::EntityCommands::despawn();
      lVar16 = lVar16 + -8;
      puVar7 = puVar7 + 1;
    } while (lVar16 != 0);
  }
  if ((*(char *)(*in_RCX + 0xf2) != '\x01') || (*(char *)(*in_RCX + 0x102) != '\x02')) {
    return;
  }
  lVar16 = *in_RDX;
  lVar8 = in_RDX[1];
  puVar17 = *(uint **)(lVar16 + 0xf8);
  puVar1 = puVar17 + *(longlong *)(lVar16 + 0x100);
  cVar5 = *(char *)(lVar16 + 0x108);
  lVar9 = *in_stack_00000028;
  local_d8 = 8;
  local_b8 = 0;
  uVar19 = 0;
  uVar18 = 0;
  do {
    if (cVar5 == '\0') {
      if (uVar18 == uVar19) {
        do {
          if (puVar17 == puVar1) {
            return;
          }
          uVar6 = *puVar17;
          puVar17 = puVar17 + 1;
          uVar19 = *(ulonglong *)(*(longlong *)(lVar8 + 0x100) + 0x58 + (ulonglong)uVar6 * 0xa0);
        } while (uVar19 == 0);
        lVar11 = *(longlong *)(lVar8 + 0x100) + (ulonglong)uVar6 * 0xa0;
        uVar18 = (ulonglong)*(uint *)(lVar11 + 0x94);
        lVar2 = *(longlong *)(lVar8 + 0x1a8) + uVar18 * 0x48;
        if ((*(ulonglong *)(lVar16 + 0x110) <
             *(ulonglong *)(*(longlong *)(lVar8 + 0x1a8) + 0x40 + uVar18 * 0x48)) &&
           (uVar18 = *(ulonglong *)
                      (*(longlong *)(lVar2 + 0x38) + *(ulonglong *)(lVar16 + 0x110) * 8),
           uVar18 != 0)) {
          local_b8 = *(longlong *)(*(longlong *)(lVar2 + 0x18) + 0x10 + ~uVar18 * 0x30);
        }
        else {
          local_b8 = 0;
        }
        local_d8 = *(longlong *)(lVar11 + 0x50);
        uVar18 = 0;
      }
      uVar13 = (ulonglong)*(uint *)(local_d8 + 8 + uVar18 * 0x10);
    }
    else {
      if (uVar18 == uVar19) {
        do {
          if (puVar17 == puVar1) {
            return;
          }
          uVar6 = *puVar17;
          puVar17 = puVar17 + 1;
          uVar19 = *(ulonglong *)(*(longlong *)(lVar8 + 0x1a8) + 0x10 + (ulonglong)uVar6 * 0x48);
        } while (uVar19 == 0);
        lVar2 = *(longlong *)(lVar8 + 0x1a8) + (ulonglong)uVar6 * 0x48;
        if ((*(ulonglong *)(lVar16 + 0x110) < *(ulonglong *)(lVar2 + 0x40)) &&
           (uVar18 = *(ulonglong *)
                      (*(longlong *)(lVar2 + 0x38) + *(ulonglong *)(lVar16 + 0x110) * 8),
           uVar18 != 0)) {
          local_b8 = *(longlong *)(*(longlong *)(lVar2 + 0x18) + 0x10 + ~uVar18 * 0x30);
        }
        else {
          local_b8 = 0;
        }
        uVar18 = 0;
      }
      uVar13 = uVar18 & 0xffffffff;
    }
    lVar2 = local_b8 + uVar13 * 0x48;
    if (*(longlong *)(lVar2 + 0x10) == 0) {
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
    uVar13 = *(ulonglong *)(lVar2 + 0x40);
    core::ops::function::FnOnce::
    call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
              ();
    if (extraout_RAX == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    *extraout_RAX = *extraout_RAX + 1;
    alloc::vec::spec_from_iter::impl$0::
    from_iter<array_<f32,3>,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl_1::triangles::closure_env_0>,array_<glam::f32::vec3::Vec3,3>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env_0>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env_2>_>
              ();
    country_core::render::mesh::Mesh::
    set_attribute<alloc::vec::Vec<array_<f32,3>,alloc::alloc::Global>_>();
    plVar20 = (longlong *)(uVar13 * 3);
    alloc::vec::spec_from_iter::impl$0::
    from_iter<array_<f32,3>,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<array_<glam::f32::vec2::Vec2,3>_>,array_<glam::f32::vec2::Vec2,3>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env_1>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs::closure_env_3>_>
              ();
    country_core::render::mesh::Mesh::
    set_attribute<alloc::vec::Vec<array_<f32,3>,alloc::alloc::Global>_>();
    if (uVar13 == 0) {
      plVar21 = (longlong *)&DAT_00000004;
      plVar20 = (longlong *)0x0;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_00 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
        alloc::raw_vec::handle_error();
      }
      plVar21 = extraout_RAX_00;
      if (uVar13 < 3) {
        plVar12 = (longlong *)0x0;
      }
      else {
        plVar12 = (longlong *)((ulonglong)plVar20 & 0x1ffffffffffffff8);
        uVar14 = 0;
        iVar22 = 0;
        iVar23 = 1;
        iVar24 = 2;
        iVar25 = 3;
        do {
          piVar3 = (int *)((longlong)extraout_RAX_00 + uVar14);
          *piVar3 = iVar22;
          piVar3[1] = iVar23;
          piVar3[2] = iVar24;
          piVar3[3] = iVar25;
          piVar3 = (int *)((longlong)extraout_RAX_00 + uVar14 + 0x10);
          *piVar3 = iVar22 + 4;
          piVar3[1] = iVar23 + 4;
          piVar3[2] = iVar24 + 4;
          piVar3[3] = iVar25 + 4;
          iVar22 = iVar22 + 8;
          iVar23 = iVar23 + 8;
          iVar24 = iVar24 + 8;
          iVar25 = iVar25 + 8;
          uVar14 = uVar14 + 0x20;
        } while ((uVar13 * 0xc & 0xffffffffffffffe0) != uVar14);
        if (plVar20 == plVar12) goto LAB_1414554d0;
      }
      do {
        *(int *)((longlong)extraout_RAX_00 + (longlong)plVar12 * 4) = (int)plVar12;
        plVar12 = (longlong *)((longlong)plVar12 + 1);
      } while (plVar20 != plVar12);
    }
LAB_1414554d0:
    asset_libraries::mesh_library::AssetMeshLibrary::add();
    puVar15 = &DAT_142c617d4;
    asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name();
    if ((extraout_RAX_02 & 1) == 0) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    bevy_ecs::system::commands::Commands::spawn_empty();
    if (plVar21 == (longlong *)0x0) {
      plVar21 = plVar20;
    }
    lVar2 = plVar21[2];
    if ((ulonglong)(*plVar21 - lVar2) < 0x78) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar11 = plVar21[1];
    *(code **)(lVar11 + lVar2) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::DrawableMeshBundle>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    puVar4 = (undefined8 *)(lVar11 + 8 + lVar2);
    *puVar4 = 0;
    puVar4[1] = 0x3f80000000000000;
    *(undefined8 *)(lVar11 + 0x18 + lVar2) = 0;
    *(undefined4 *)(lVar11 + 0x20 + lVar2) = 0;
    *(undefined8 *)(lVar11 + 0x24 + lVar2) = 0x3f8000003f800000;
    *(undefined4 *)(lVar11 + 0x2c + lVar2) = 0x3f800000;
    *(undefined8 *)(lVar11 + 0x38 + lVar2) = extraout_RAX_01;
    *(undefined **)(lVar11 + 0x40 + lVar2) = puVar15;
    *(undefined1 *)(lVar11 + 0x48 + lVar2) = 0;
    *(undefined8 *)(lVar11 + 0x58 + lVar2) = 0;
    *(code **)(lVar11 + 0x68 + lVar2) = bevy_ecs::error::handler::panic;
    plVar21[2] = lVar2 + 0x78;
    plVar20 = *(longlong **)(lVar9 + 0x18);
    LOCK();
    lVar2 = *plVar20;
    *plVar20 = *plVar20 + 1;
    UNLOCK();
    if (*plVar20 == 0 || SCARRY8(lVar2,1) != *plVar20 < 0) {
                    // WARNING: Does not return
      pcVar10 = (code *)invalidInstructionException();
      (*pcVar10)();
    }
    bevy_ecs::system::commands::EntityCommands::
    insert<country_core::components::transient_mesh::TransientMesh>();
    lVar2 = in_R9[2];
    if (lVar2 == *in_R9) {
      alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
    }
    uVar18 = uVar18 + 1;
    *(undefined8 *)(in_R9[1] + lVar2 * 8) = 0;
    in_R9[2] = lVar2 + 1;
  } while( true );
}


