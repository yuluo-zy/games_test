// addr=0x14129f400 name=load_mesh_atlases
// signature=void __cdecl load_mesh_atlases(void)


void __cdecl country_core::startup::load_mesh_atlases::load_mesh_atlases(void)

{
  float fVar1;
  longlong lVar2;
  undefined1 auVar3 [16];
  undefined4 uVar4;
  undefined4 *puVar5;
  longlong lVar6;
  longlong lVar7;
  longlong *plVar8;
  code *pcVar9;
  ushort uVar10;
  uint uVar11;
  int iVar12;
  uint uVar13;
  longlong *extraout_RAX;
  longlong *extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong *extraout_RAX_03;
  undefined8 *in_RCX;
  ulonglong uVar14;
  longlong in_RDX;
  longlong in_R8;
  longlong lVar15;
  longlong *in_R9;
  undefined1 (*pauVar16) [16];
  undefined1 (*pauVar17) [16];
  longlong lVar18;
  bool bVar19;
  uint uVar20;
  float fVar21;
  float fVar22;
  longlong lStack_220;
  longlong local_218;
  longlong local_210;
  ulonglong local_1e8;
  uint local_100;
  
  lVar15 = ((undefined8 *)*in_RCX)[3];
  uVar4 = *(undefined4 *)(in_RDX + 0x1c);
  puVar5 = *(undefined4 **)(in_RDX + 0x10);
  if (lVar15 != 0) {
    pauVar16 = *(undefined1 (**) [16])*in_RCX;
    pauVar17 = pauVar16 + 1;
    auVar3 = *pauVar16;
    uVar20 = ~(uint)(ushort)((ushort)(SUB161(auVar3 >> 7,0) & 1) |
                             (ushort)(SUB161(auVar3 >> 0xf,0) & 1) << 1 |
                             (ushort)(SUB161(auVar3 >> 0x17,0) & 1) << 2 |
                             (ushort)(SUB161(auVar3 >> 0x1f,0) & 1) << 3 |
                             (ushort)(SUB161(auVar3 >> 0x27,0) & 1) << 4 |
                             (ushort)(SUB161(auVar3 >> 0x2f,0) & 1) << 5 |
                             (ushort)(SUB161(auVar3 >> 0x37,0) & 1) << 6 |
                             (ushort)(SUB161(auVar3 >> 0x3f,0) & 1) << 7 |
                             (ushort)(SUB161(auVar3 >> 0x47,0) & 1) << 8 |
                             (ushort)(SUB161(auVar3 >> 0x4f,0) & 1) << 9 |
                             (ushort)(SUB161(auVar3 >> 0x57,0) & 1) << 10 |
                             (ushort)(SUB161(auVar3 >> 0x5f,0) & 1) << 0xb |
                             (ushort)(SUB161(auVar3 >> 0x67,0) & 1) << 0xc |
                             (ushort)(SUB161(auVar3 >> 0x6f,0) & 1) << 0xd |
                             (ushort)(SUB161(auVar3 >> 0x77,0) & 1) << 0xe |
                            (ushort)(byte)(auVar3[0xf] >> 7) << 0xf);
    do {
      if ((short)uVar20 == 0) {
        do {
          auVar3 = *pauVar17;
          uVar10 = (ushort)(SUB161(auVar3 >> 7,0) & 1) | (ushort)(SUB161(auVar3 >> 0xf,0) & 1) << 1
                   | (ushort)(SUB161(auVar3 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar3 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar3 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar3 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar3 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar3 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar3 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar3 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar3 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar3 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar3 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar3 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar3 >> 0x77,0) & 1) << 0xe |
                   (ushort)(byte)(auVar3[0xf] >> 7) << 0xf;
          pauVar16 = pauVar16 + -0x28;
          pauVar17 = pauVar17 + 1;
        } while (uVar10 == 0xffff);
        uVar20 = ~(uint)uVar10;
        uVar13 = uVar20 & -(uint)uVar10 - 2;
      }
      else {
        uVar13 = uVar20 - 1 & uVar20;
      }
      uVar11 = 0;
      for (; (uVar20 & 1) == 0; uVar20 = uVar20 >> 1 | 0x80000000) {
        uVar11 = uVar11 + 1;
      }
      uVar14 = (ulonglong)uVar11;
      lVar6 = *(longlong *)((longlong)pauVar16 + uVar14 * -0x28 + -8);
      if (lVar6 != 0) {
        lVar7 = *(longlong *)((longlong)pauVar16 + uVar14 * -0x28 + -0x10);
        lVar18 = 0;
        do {
          if (*(longlong *)(*(longlong *)(lVar7 + 0x88 + lVar18) + 0x20) != 0) {
            *puVar5 = uVar4;
            plVar8 = *(longlong **)((longlong)pauVar16 + uVar14 * -0x28 + -0x28);
            LOCK();
            lVar2 = *plVar8;
            *plVar8 = *plVar8 + 1;
            UNLOCK();
            if (*plVar8 == 0 || SCARRY8(lVar2,1) != *plVar8 < 0) {
                    // WARNING: Does not return
              pcVar9 = (code *)invalidInstructionException();
              (*pcVar9)();
            }
            resources::render::mesh_atlas_library::MeshAtlasLibrary::add<asset::prefab::PrefabName>
                      ();
          }
          lVar18 = lVar18 + 0x98;
        } while (lVar6 * 0x98 - lVar18 != 0);
      }
      lVar15 = lVar15 + -1;
      uVar20 = uVar13;
    } while (lVar15 != 0);
  }
  *puVar5 = uVar4;
  if (*in_R9 != 0) {
    *(undefined4 *)in_R9[2] = *(undefined4 *)((longlong)in_R9 + 0x1c);
  }
  load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env_0>();
  *puVar5 = uVar4;
  bVar19 = false;
  uVar20 = 0;
  while ((!bVar19 && (uVar20 < 6))) {
    bVar19 = 4 < uVar20;
    uVar13 = uVar20 + 1;
    if (bVar19) {
      uVar13 = 5;
    }
    core::ops::function::FnOnce::
    call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
              ();
    if (extraout_RAX == (longlong *)0x0) goto LAB_1412a00fd;
    *extraout_RAX = *extraout_RAX + 1;
    startup_terrain::grid_gen_cw();
    alloc::vec::spec_from_iter::impl$0::
    from_iter<array_<f32,3>,core::iter::adapters::flatten::FlatMap<core::ops::range::Range<u32>,array_<array_<f32,3>,2>,country_core::startup::load_mesh_atlases::generate_roof_floor_meshes::closure_env_0>_>
              ();
    if ((uVar20 != 0) && (local_218 != 0)) {
      lVar15 = 0;
      do {
        fVar1 = *(float *)(lStack_220 + lVar15);
        fVar21 = sinf(*(float *)(lStack_220 + 8 + lVar15) * 3.1415927);
        fVar21 = fVar21 * 0.5;
        *(float *)(lStack_220 + 8 + lVar15) = fVar21;
        uVar20 = -(uint)NAN(fVar1);
        fVar22 = 1.0 - (fVar21 + fVar21) * (fVar21 + fVar21);
        fVar21 = 0.0;
        if (0.0 <= fVar22) {
          fVar21 = fVar22;
        }
        fVar22 = 1.0;
        if (fVar21 <= 1.0) {
          fVar22 = fVar21;
        }
        *(float *)(lStack_220 + lVar15) =
             SQRT(fVar22) *
             (float)(uVar20 & 0x7fc00000 | ~uVar20 & ((uint)fVar1 & 0x80000000 | 0x3f800000)) * 0.5;
        lVar15 = lVar15 + 0xc;
      } while (local_218 * 0xc != lVar15);
    }
    country_core::render::mesh::Mesh::
    set_attribute<alloc::vec::Vec<array_<f32,3>,alloc::alloc::Global>_>();
    alloc::fmt::format::format_inner();
    resources::render::mesh_atlas_library::MeshAtlasLibrary::add<ref_<str_>_>();
    if (local_210 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    core::ptr::drop_in_place<country_core::render::mesh::Mesh>();
    uVar20 = uVar13;
  }
  *puVar5 = uVar4;
  uVar20 = 0;
  bVar19 = false;
  while ((!bVar19 && (uVar20 < 6))) {
    bVar19 = 4 < uVar20;
    uVar20 = uVar20 + 1;
    if (bVar19) {
      uVar20 = 5;
    }
    core::ops::function::FnOnce::
    call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
              ();
    if (extraout_RAX_00 == (longlong *)0x0) goto LAB_1412a00fd;
    *extraout_RAX_00 = *extraout_RAX_00 + 1;
    startup_terrain::grid_gen_ccw();
    alloc::vec::spec_from_iter::impl$0::
    from_iter<array_<f32,3>,core::iter::adapters::flatten::FlatMap<core::ops::range::Range<u32>,array_<array_<f32,3>,7>,country_core::startup::load_mesh_atlases::generate_curved_halftimber_meshes::closure_env_0>_>
              ();
    country_core::render::mesh::Mesh::
    set_attribute<alloc::vec::Vec<array_<f32,3>,alloc::alloc::Global>_>();
    alloc::fmt::format::format_inner();
    resources::render::mesh_atlas_library::MeshAtlasLibrary::add<ref_<str_>_>();
    if (local_210 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    core::ptr::drop_in_place<country_core::render::mesh::Mesh>();
  }
  *puVar5 = uVar4;
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX_01 != (longlong *)0x0) {
    *extraout_RAX_01 = *extraout_RAX_01 + 1;
    local_1e8 = 0;
    uVar20 = 0;
    while (uVar20 < 0x20) {
      if (local_1e8 == 0) {
        alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>();
      }
      *(uint *)(local_1e8 * 4 + 4) = uVar20 | 0x10000;
      if ((local_1e8 | 1) == 0) {
        alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>();
      }
      *(uint *)((local_1e8 | 1) * 4 + 4) = uVar20;
      if (local_1e8 == 0xfffffffffffffffe) {
        alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>();
      }
      uVar13 = uVar20 + 1;
      *(uint *)(local_1e8 * 4 + 0xc) = uVar13;
      if (local_1e8 == 0xfffffffffffffffd) {
        alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>();
      }
      *(uint *)(local_1e8 * 4 + 0x10) = uVar13;
      if (local_1e8 == 0xfffffffffffffffc) {
        alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>();
      }
      *(uint *)(local_1e8 * 4 + 0x14) = uVar20 + 0x10001;
      if (local_1e8 == 0xfffffffffffffffb) {
        alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>();
      }
      *(uint *)(local_1e8 * 4 + 0x18) = uVar20 | 0x10000;
      local_1e8 = local_1e8 + 6;
      uVar20 = uVar13;
    }
    resources::render::mesh_atlas_library::MeshAtlasLibrary::add<ref_<str_>_>();
    core::ptr::drop_in_place<country_core::render::mesh::Mesh>();
    *puVar5 = uVar4;
    local_100 = 4;
    while (local_100 < 0x41) {
      core::ops::function::FnOnce::
      call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                ();
      if (extraout_RAX_02 == (longlong *)0x0) goto LAB_1412a00fd;
      *extraout_RAX_02 = *extraout_RAX_02 + 1;
      local_1e8 = 0;
      for (uVar20 = 0; uVar20 < local_100; uVar20 = uVar20 + 1) {
        if (local_1e8 == 0) {
          alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>();
        }
        iVar12 = uVar20 * 3;
        *(int *)(local_1e8 * 4 + 4) = iVar12;
        if (local_1e8 == -1) {
          alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>();
        }
        *(int *)(local_1e8 * 4 + 8) = iVar12 + 1;
        if (local_1e8 == -2) {
          alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>();
        }
        *(int *)(local_1e8 * 4 + 0xc) = iVar12 + 2;
        local_1e8 = local_1e8 + 3;
      }
      alloc::fmt::format::format_inner();
      resources::render::mesh_atlas_library::MeshAtlasLibrary::add<ref_<str_>_>();
      if (local_210 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      local_100 = local_100 << 1;
      core::ptr::drop_in_place<country_core::render::mesh::Mesh>();
    }
    *puVar5 = uVar4;
    core::ops::function::FnOnce::
    call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
              ();
    if (extraout_RAX_03 != (longlong *)0x0) {
      *extraout_RAX_03 = *extraout_RAX_03 + 1;
      startup_terrain::grid_gen_cw();
      resources::render::mesh_atlas_library::MeshAtlasLibrary::add<ref_<str_>_>();
      core::ptr::drop_in_place<country_core::render::mesh::Mesh>();
      *puVar5 = uVar4;
      **(undefined4 **)(in_R8 + 0x10) = *(undefined4 *)(in_R8 + 0x1c);
      resources::render::mesh_atlas_library::MeshAtlasLibrary::register_meshes();
      return;
    }
  }
LAB_1412a00fd:
                    // WARNING: Subroutine does not return
  std::thread::local::panic_access_error();
}


