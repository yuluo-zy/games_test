// addr=0x140a398b0 name=reload_nani
// signature=void __cdecl reload_nani(void)


void __cdecl country_core::startup::startup_nani::reload_nani(void)

{
  undefined1 auVar1 [16];
  byte bVar2;
  ulonglong *puVar3;
  ulonglong uVar4;
  undefined4 *puVar5;
  ushort uVar6;
  uint uVar7;
  longlong *plVar8;
  uint uVar9;
  uint uVar10;
  longlong *extraout_RAX;
  ulonglong uVar11;
  longlong *plVar12;
  longlong extraout_RAX_00;
  undefined8 *in_RCX;
  longlong lVar13;
  ulonglong uVar14;
  longlong *in_R8;
  ulonglong uVar15;
  undefined8 *in_R9;
  ulonglong uVar16;
  undefined1 (*pauVar17) [16];
  longlong *plVar18;
  longlong lVar19;
  longlong *plVar20;
  longlong *plVar21;
  undefined1 (*pauVar22) [16];
  bool bVar23;
  longlong local_138;
  undefined1 (*local_e8) [16];
  ulonglong uStack_e0;
  ulonglong local_d8;
  longlong lStack_d0;
  longlong local_c8;
  longlong lStack_c0;
  undefined1 (**local_b8) [16];
  longlong local_b0;
  ulonglong local_a8;
  longlong local_a0;
  undefined *local_98;
  longlong *local_90;
  undefined8 local_88;
  longlong *local_80;
  undefined8 *local_78;
  longlong local_70;
  undefined1 local_61;
  undefined8 local_60;
  
  local_60 = 0xfffffffffffffffe;
  local_80 = in_R8;
  local_78 = in_RCX;
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
  local_c8 = *extraout_RAX;
  lStack_c0 = extraout_RAX[1];
  *extraout_RAX = *extraout_RAX + 1;
  local_e8 = (undefined1 (*) [16])&anon_b07311dcbeea092d4c88647b99c4ec26_8_llvm_12982147591757793021
  ;
  uStack_e0 = 0;
  local_d8 = 0;
  lStack_d0 = 0;
  puVar3 = (ulonglong *)*in_R9;
  lVar19 = in_R9[1];
  uVar4 = *puVar3;
  uVar14 = 0;
  uVar11 = uVar4 - *(ulonglong *)(lVar19 + 0x18);
  if (uVar4 < *(ulonglong *)(lVar19 + 0x18)) {
    uVar11 = uVar14;
  }
  uVar15 = uVar4 - *(ulonglong *)(lVar19 + 0x38);
  if (uVar4 < *(ulonglong *)(lVar19 + 0x38)) {
    uVar15 = uVar14;
  }
  plVar20 = (longlong *)(uVar11 * 0x38 + *(longlong *)(lVar19 + 8));
  uVar4 = *(ulonglong *)(lVar19 + 0x10);
  uVar16 = uVar4 - uVar11;
  if (uVar4 < uVar11) {
    uVar16 = uVar14;
  }
  if (uVar4 < uVar11) {
    plVar20 = (longlong *)&DAT_00000008;
  }
  plVar12 = (longlong *)(uVar15 * 0x38 + *(longlong *)(lVar19 + 0x28));
  uVar4 = *(ulonglong *)(lVar19 + 0x30) - uVar15;
  if (*(ulonglong *)(lVar19 + 0x30) < uVar15) {
    plVar12 = (longlong *)&DAT_00000008;
    uVar4 = uVar14;
  }
  *puVar3 = (*(longlong *)(lVar19 + 0x40) - uVar4) - uVar16;
  local_90 = plVar12 + uVar4 * 7;
  plVar21 = plVar20;
  do {
    while ((plVar21 != (longlong *)0x0 && (plVar21 != plVar20 + uVar16 * 7))) {
      plVar18 = plVar21 + 7;
      *puVar3 = *puVar3 + 1;
      plVar8 = plVar12;
      if (lStack_d0 == 0) goto LAB_140a39a76;
LAB_140a399b0:
      plVar12 = plVar8;
      local_138 = *plVar21;
      if (local_d8 < plVar21[3] + 1U >> 1) {
        local_61 = 1;
        hashbrown::raw::RawTable::
        reserve_rehash<tuple_<country_core::render::nani::gpu_world::GpuWorldSubsetKind,tuple_<>_>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::render::nani::gpu_world::GpuWorldSubsetKind,tuple_<>,std::hash::random::RandomState>_>
                  ();
      }
      local_b8 = &local_e8;
      local_61 = 1;
      hashbrown::raw::RawIterRange::
      fold_impl<tuple_<country_core::render::nani::gpu_world::GpuWorldSubsetKind,tuple_<>_>,hashbrown::map::impl_44::fold::closure_env_0<country_core::render::nani::gpu_world::GpuWorldSubsetKind,tuple_<>,tuple_<>,hashbrown::map::impl_58::fold::closure_env_0<country_core::render::nani::gpu_world::GpuWorldSubsetKind,tuple_<>,tuple_<>,core::iter::adapters::copied::copy_fold::closure_env_0<country_core::render::nani::gpu_world::GpuWorldSubsetKind,tuple_<>,core::iter::adapters::map::map_fold::closure_env_0<country_core::render::nani::gpu_world::GpuWorldSubsetKind,tuple_<country_core::render::nani::gpu_world::GpuWorldSubsetKind,tuple_<>_>,tuple_<>,hashbrown::set::impl_10::extend::closure_env_0<country_core::render::nani::gpu_world::GpuWorldSubsetKind,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::copied::Copied<std::collections::hash::set::Iter<country_core::render::nani::gpu_world::GpuWorldSubsetKind>_>_>,core::iter::traits::iterator::Iterator::for_each::call::closure_env_0<tuple_<country_core::render::nani::gpu_world::GpuWorldSubsetKind,tuple_<>_>,hashbrown::map::impl_82::extend::closure_env_0<country_core::render::nani::gpu_world::GpuWorldSubsetKind,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<std::collections::hash::set::Iter<country_core::render::nani::gpu_world::GpuWorldSubsetKind>_>,hashbrown::set::impl_10::extend::closure_env_0<country_core::render::nani::gpu_world::GpuWorldSubsetKind,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::copied::Copied<std::collections::hash::set::Iter<country_core::render::nani::gpu_world::GpuWorldSubsetKind>_>_>_>_>_>_>_>_>_>,tuple_<>_>
                ();
      plVar21 = plVar18;
    }
    if ((plVar12 == (longlong *)0x0) || (plVar12 == local_90)) break;
    plVar18 = (longlong *)0x0;
    *puVar3 = *puVar3 + 1;
    plVar8 = plVar12 + 7;
    plVar21 = plVar12;
    if (lStack_d0 != 0) goto LAB_140a399b0;
LAB_140a39a76:
    plVar12 = plVar8;
    local_61 = 1;
    hashbrown::raw::impl$13::
    clone_from<tuple_<country_core::render::nani::gpu_world::GpuWorldSubsetKind,tuple_<>_>,alloc::alloc::Global>
              ();
    local_c8 = plVar21[4];
    lStack_c0 = plVar21[5];
    plVar21 = plVar18;
  } while( true );
  auVar1 = *local_e8;
  local_a8 = uStack_e0;
  if (uStack_e0 == 0) {
    local_88 = 0;
  }
  else {
    local_a0 = uStack_e0 + (uStack_e0 & 0xfffffffffffffff0) + 0x21;
    local_98 = (undefined *)((longlong)local_e8 + (-0x10 - (uStack_e0 & 0xfffffffffffffff0)));
    local_88 = 0x10;
  }
  pauVar22 = local_e8 + 1;
  uVar9 = ~(uint)(ushort)((ushort)(SUB161(auVar1 >> 7,0) & 1) |
                          (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1 |
                          (ushort)(SUB161(auVar1 >> 0x17,0) & 1) << 2 |
                          (ushort)(SUB161(auVar1 >> 0x1f,0) & 1) << 3 |
                          (ushort)(SUB161(auVar1 >> 0x27,0) & 1) << 4 |
                          (ushort)(SUB161(auVar1 >> 0x2f,0) & 1) << 5 |
                          (ushort)(SUB161(auVar1 >> 0x37,0) & 1) << 6 |
                          (ushort)(SUB161(auVar1 >> 0x3f,0) & 1) << 7 |
                          (ushort)(SUB161(auVar1 >> 0x47,0) & 1) << 8 |
                          (ushort)(SUB161(auVar1 >> 0x4f,0) & 1) << 9 |
                          (ushort)(SUB161(auVar1 >> 0x57,0) & 1) << 10 |
                          (ushort)(SUB161(auVar1 >> 0x5f,0) & 1) << 0xb |
                          (ushort)(SUB161(auVar1 >> 0x67,0) & 1) << 0xc |
                          (ushort)(SUB161(auVar1 >> 0x6f,0) & 1) << 0xd |
                          (ushort)(SUB161(auVar1 >> 0x77,0) & 1) << 0xe |
                         (ushort)(byte)(auVar1[0xf] >> 7) << 0xf);
  local_90 = (longlong *)CONCAT44(local_90._4_4_,*(undefined4 *)((longlong)local_80 + 0x1c));
  puVar5 = (undefined4 *)local_80[2];
  local_78 = (undefined8 *)*local_78;
  local_70 = *local_80 + 0x1458;
  pauVar17 = local_e8;
  lVar19 = lStack_d0;
  local_80 = (longlong *)*local_80;
  do {
    do {
      bVar23 = lVar19 == 0;
      lVar19 = lVar19 + -1;
      if (bVar23) {
        if (local_a0 != 0 && local_a8 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        return;
      }
      if ((short)uVar9 == 0) {
        do {
          auVar1 = *pauVar22;
          uVar6 = (ushort)(SUB161(auVar1 >> 7,0) & 1) | (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1 |
                  (ushort)(SUB161(auVar1 >> 0x17,0) & 1) << 2 |
                  (ushort)(SUB161(auVar1 >> 0x1f,0) & 1) << 3 |
                  (ushort)(SUB161(auVar1 >> 0x27,0) & 1) << 4 |
                  (ushort)(SUB161(auVar1 >> 0x2f,0) & 1) << 5 |
                  (ushort)(SUB161(auVar1 >> 0x37,0) & 1) << 6 |
                  (ushort)(SUB161(auVar1 >> 0x3f,0) & 1) << 7 |
                  (ushort)(SUB161(auVar1 >> 0x47,0) & 1) << 8 |
                  (ushort)(SUB161(auVar1 >> 0x4f,0) & 1) << 9 |
                  (ushort)(SUB161(auVar1 >> 0x57,0) & 1) << 10 |
                  (ushort)(SUB161(auVar1 >> 0x5f,0) & 1) << 0xb |
                  (ushort)(SUB161(auVar1 >> 0x67,0) & 1) << 0xc |
                  (ushort)(SUB161(auVar1 >> 0x6f,0) & 1) << 0xd |
                  (ushort)(SUB161(auVar1 >> 0x77,0) & 1) << 0xe |
                  (ushort)(byte)(auVar1[0xf] >> 7) << 0xf;
          pauVar17 = pauVar17 + -1;
          pauVar22 = pauVar22 + 1;
        } while (uVar6 == 0xffff);
        uVar10 = ~(uint)uVar6;
        uVar9 = uVar10 & -(uint)uVar6 - 2;
      }
      else {
        uVar10 = uVar9;
        uVar9 = uVar9 - 1 & uVar9;
      }
      uVar7 = 0;
      for (; (uVar10 & 1) == 0; uVar10 = uVar10 >> 1 | 0x80000000) {
        uVar7 = uVar7 + 1;
      }
      bVar2 = (*pauVar17)[~(ulonglong)uVar7];
      resources::render::mesh_atlas_library::MeshAtlasLibrary::get_by_world_subset();
      *puVar5 = local_90._0_4_;
    } while (*(longlong *)(extraout_RAX_00 + 0x98) == 0);
    lVar13 = *(longlong *)(extraout_RAX_00 + 0x98) * 0x10;
    if (*(ulonglong *)((longlong)local_80 + 0x1470) < lVar13 + (ulonglong)bVar2 * 0x400) break;
    local_b8 = (undefined1 (**) [16])0x0;
    local_b0 = lVar13;
    rhapsody::raw::device::GraphicsDevice::update_buffer_bytes();
  } while (local_138 == 0xb);
                    // WARNING: Subroutine does not return
  core::result::unwrap_failed();
}


