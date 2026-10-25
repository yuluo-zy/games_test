// walls_on_glade_unload
// sig: void __cdecl walls_on_glade_unload(void)
// size: 3765

// WARNING: Removing unreachable block (ram,0x0001402b2680)
// WARNING: Removing unreachable block (ram,0x0001402b26b6)
// WARNING: Removing unreachable block (ram,0x0001402b26c9)
// WARNING: Removing unreachable block (ram,0x0001402b26f3)
// WARNING: Removing unreachable block (ram,0x0001402b2714)
// WARNING: Removing unreachable block (ram,0x0001402b2732)
// WARNING: Removing unreachable block (ram,0x0001402b2745)
// WARNING: Removing unreachable block (ram,0x0001402b2ab3)
// WARNING: Removing unreachable block (ram,0x0001402b2763)
// WARNING: Removing unreachable block (ram,0x0001402b2782)
// WARNING: Removing unreachable block (ram,0x0001402b27a0)
// WARNING: Removing unreachable block (ram,0x0001402b27d0)
// WARNING: Removing unreachable block (ram,0x0001402b2ad5)
// WARNING: Removing unreachable block (ram,0x0001402b2815)
// WARNING: Removing unreachable block (ram,0x0001402b2adc)
// WARNING: Removing unreachable block (ram,0x0001402b2b12)
// WARNING: Removing unreachable block (ram,0x0001402b2abf)
// WARNING: Removing unreachable block (ram,0x0001402b28c8)
// WARNING: Removing unreachable block (ram,0x0001402b290a)
// WARNING: Removing unreachable block (ram,0x0001402b2491)
// WARNING: Removing unreachable block (ram,0x0001402b24c9)
// WARNING: Removing unreachable block (ram,0x0001402b24dc)
// WARNING: Removing unreachable block (ram,0x0001402b2506)
// WARNING: Removing unreachable block (ram,0x0001402b2527)
// WARNING: Removing unreachable block (ram,0x0001402b2545)
// WARNING: Removing unreachable block (ram,0x0001402b28fe)
// WARNING: Removing unreachable block (ram,0x0001402b2558)
// WARNING: Removing unreachable block (ram,0x0001402b29ed)
// WARNING: Removing unreachable block (ram,0x0001402b257a)
// WARNING: Removing unreachable block (ram,0x0001402b29f9)
// WARNING: Removing unreachable block (ram,0x0001402b2599)
// WARNING: Removing unreachable block (ram,0x0001402b25b0)
// WARNING: Removing unreachable block (ram,0x0001402b25e0)
// WARNING: Removing unreachable block (ram,0x0001402b2a0f)
// WARNING: Removing unreachable block (ram,0x0001402b262d)
// WARNING: Removing unreachable block (ram,0x0001402b2a16)
// WARNING: Removing unreachable block (ram,0x0001402b2a4c)
// WARNING: Removing unreachable block (ram,0x0001402b2865)

void __cdecl tiny_glade::app_systems::walls_on_glade_unload(void)

{
  undefined8 *puVar1;
  longlong lVar2;
  longlong lVar3;
  longlong *plVar4;
  int iVar5;
  byte *extraout_RAX;
  longlong extraout_RAX_00;
  byte *extraout_RAX_01;
  longlong extraout_RAX_02;
  byte *extraout_RAX_03;
  longlong extraout_RAX_04;
  byte *extraout_RAX_05;
  longlong extraout_RAX_06;
  byte *extraout_RAX_07;
  longlong extraout_RAX_08;
  byte *extraout_RAX_09;
  longlong extraout_RAX_10;
  byte *extraout_RAX_11;
  longlong extraout_RAX_12;
  longlong lVar6;
  longlong *extraout_RAX_13;
  undefined8 *extraout_RAX_14;
  longlong lVar7;
  undefined8 extraout_RAX_15;
  longlong extraout_RAX_16;
  longlong lVar8;
  undefined8 in_RDX;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined4 *puVar11;
  undefined4 *puVar12;
  undefined8 *in_R8;
  undefined8 *in_R9;
  undefined4 uVar13;
  undefined8 *in_stack_00000028;
  longlong in_stack_00000030;
  undefined8 *in_stack_00000038;
  undefined8 *in_stack_00000040;
  longlong *in_stack_00000058;
  undefined8 *in_stack_00000060;
  longlong in_stack_00000068;
  longlong local_78;
  
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX & 1) == 0) {
    std::sys::random::hashmap_random_keys();
    extraout_RAX[0] = 1;
    extraout_RAX[1] = 0;
    extraout_RAX[2] = 0;
    extraout_RAX[3] = 0;
    extraout_RAX[4] = 0;
    extraout_RAX[5] = 0;
    extraout_RAX[6] = 0;
    extraout_RAX[7] = 0;
    *(longlong *)(extraout_RAX + 8) = extraout_RAX_00;
    *(undefined8 *)(extraout_RAX + 0x10) = in_RDX;
    lVar7 = extraout_RAX_00;
  }
  else {
    lVar7 = *(longlong *)(extraout_RAX + 8);
    in_RDX = *(undefined8 *)(extraout_RAX + 0x10);
  }
  *(longlong *)(extraout_RAX + 8) = lVar7 + 1;
  uVar9 = in_RDX;
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX_01 & 1) == 0) {
    std::sys::random::hashmap_random_keys();
    extraout_RAX_01[0] = 1;
    extraout_RAX_01[1] = 0;
    extraout_RAX_01[2] = 0;
    extraout_RAX_01[3] = 0;
    extraout_RAX_01[4] = 0;
    extraout_RAX_01[5] = 0;
    extraout_RAX_01[6] = 0;
    extraout_RAX_01[7] = 0;
    *(longlong *)(extraout_RAX_01 + 8) = extraout_RAX_02;
    *(undefined8 *)(extraout_RAX_01 + 0x10) = uVar9;
    lVar8 = extraout_RAX_02;
  }
  else {
    lVar8 = *(longlong *)(extraout_RAX_01 + 8);
    uVar9 = *(undefined8 *)(extraout_RAX_01 + 0x10);
  }
  *(longlong *)(extraout_RAX_01 + 8) = lVar8 + 1;
  uVar10 = uVar9;
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX_03 & 1) == 0) {
    std::sys::random::hashmap_random_keys();
    extraout_RAX_03[0] = 1;
    extraout_RAX_03[1] = 0;
    extraout_RAX_03[2] = 0;
    extraout_RAX_03[3] = 0;
    extraout_RAX_03[4] = 0;
    extraout_RAX_03[5] = 0;
    extraout_RAX_03[6] = 0;
    extraout_RAX_03[7] = 0;
    *(longlong *)(extraout_RAX_03 + 8) = extraout_RAX_04;
    *(undefined8 *)(extraout_RAX_03 + 0x10) = uVar10;
    lVar6 = extraout_RAX_04;
    local_78 = uVar10;
  }
  else {
    lVar6 = *(longlong *)(extraout_RAX_03 + 8);
    local_78 = *(undefined8 *)(extraout_RAX_03 + 0x10);
  }
  *(longlong *)(extraout_RAX_03 + 8) = lVar6 + 1;
  *(undefined4 *)in_R8[2] = *(undefined4 *)((longlong)in_R8 + 0x1c);
  puVar1 = (undefined8 *)*in_R8;
  core::ptr::drop_in_place<country_core::resources::walls::public_walls::PublicWalls>();
  puVar1[0xe] = 0;
  puVar1[0xf] = 0;
  puVar1[0xc] = &DAT_1429ada80;
  puVar1[0xd] = 0;
  puVar1[10] = lVar8;
  puVar1[0xb] = uVar9;
  puVar1[8] = 0;
  puVar1[9] = 0;
  puVar1[6] = &DAT_1429ada80;
  puVar1[7] = 0;
  puVar1[4] = lVar7;
  puVar1[5] = in_RDX;
  puVar1[2] = 0;
  puVar1[3] = 0;
  *puVar1 = &DAT_1429ada80;
  puVar1[1] = 0;
  puVar1[0x10] = lVar6;
  puVar1[0x11] = local_78;
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX_05 & 1) == 0) {
    std::sys::random::hashmap_random_keys();
    extraout_RAX_05[0] = 1;
    extraout_RAX_05[1] = 0;
    extraout_RAX_05[2] = 0;
    extraout_RAX_05[3] = 0;
    extraout_RAX_05[4] = 0;
    extraout_RAX_05[5] = 0;
    extraout_RAX_05[6] = 0;
    extraout_RAX_05[7] = 0;
    *(longlong *)(extraout_RAX_05 + 8) = extraout_RAX_06;
    *(undefined8 *)(extraout_RAX_05 + 0x10) = uVar10;
    lVar7 = extraout_RAX_06;
  }
  else {
    lVar7 = *(longlong *)(extraout_RAX_05 + 8);
    uVar10 = *(undefined8 *)(extraout_RAX_05 + 0x10);
  }
  *(longlong *)(extraout_RAX_05 + 8) = lVar7 + 1;
  *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
  puVar1 = (undefined8 *)*in_stack_00000028;
  local_78 = 0;
  if ((puVar1[1] != 0) && (local_78 = puVar1[1] * 0x11 + 0x21, local_78 != 0)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  puVar1[2] = 0;
  puVar1[3] = 0;
  *puVar1 = &DAT_1429ada80;
  puVar1[1] = 0;
  puVar1[4] = lVar7;
  puVar1[5] = uVar10;
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX_07 & 1) == 0) {
    std::sys::random::hashmap_random_keys();
    extraout_RAX_07[0] = 1;
    extraout_RAX_07[1] = 0;
    extraout_RAX_07[2] = 0;
    extraout_RAX_07[3] = 0;
    extraout_RAX_07[4] = 0;
    extraout_RAX_07[5] = 0;
    extraout_RAX_07[6] = 0;
    extraout_RAX_07[7] = 0;
    *(longlong *)(extraout_RAX_07 + 8) = extraout_RAX_08;
    *(longlong *)(extraout_RAX_07 + 0x10) = local_78;
    lVar7 = extraout_RAX_08;
  }
  else {
    lVar7 = *(longlong *)(extraout_RAX_07 + 8);
    local_78 = *(longlong *)(extraout_RAX_07 + 0x10);
  }
  *(longlong *)(extraout_RAX_07 + 8) = lVar7 + 1;
  *(undefined4 *)in_R9[2] = *(undefined4 *)((longlong)in_R9 + 0x1c);
  puVar1 = (undefined8 *)*in_R9;
  hashbrown::raw::RawTableInner::
  drop_inner_table<tuple_<country_core::resources::walls::WallId,alloc::vec::Vec<bevy_ecs::entity::Entity,alloc::alloc::Global>_>,alloc::alloc::Global>
            ();
  puVar1[2] = 0;
  puVar1[3] = 0;
  *puVar1 = &DAT_1429ada80;
  puVar1[1] = 0;
  puVar1[4] = lVar7;
  puVar1[5] = local_78;
  puVar11 = (undefined4 *)in_stack_00000058[2];
  *puVar11 = *(undefined4 *)((longlong)in_stack_00000058 + 0x1c);
  *(undefined1 *)(*in_stack_00000058 + 0x10) = 0;
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX_09 & 1) == 0) {
    std::sys::random::hashmap_random_keys();
    extraout_RAX_09[0] = 1;
    extraout_RAX_09[1] = 0;
    extraout_RAX_09[2] = 0;
    extraout_RAX_09[3] = 0;
    extraout_RAX_09[4] = 0;
    extraout_RAX_09[5] = 0;
    extraout_RAX_09[6] = 0;
    extraout_RAX_09[7] = 0;
    *(longlong *)(extraout_RAX_09 + 8) = extraout_RAX_10;
    *(undefined4 **)(extraout_RAX_09 + 0x10) = puVar11;
    lVar7 = extraout_RAX_10;
  }
  else {
    lVar7 = *(longlong *)(extraout_RAX_09 + 8);
    puVar11 = *(undefined4 **)(extraout_RAX_09 + 0x10);
  }
  *(longlong *)(extraout_RAX_09 + 8) = lVar7 + 1;
  puVar12 = puVar11;
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX_11 & 1) == 0) {
    std::sys::random::hashmap_random_keys();
    extraout_RAX_11[0] = 1;
    extraout_RAX_11[1] = 0;
    extraout_RAX_11[2] = 0;
    extraout_RAX_11[3] = 0;
    extraout_RAX_11[4] = 0;
    extraout_RAX_11[5] = 0;
    extraout_RAX_11[6] = 0;
    extraout_RAX_11[7] = 0;
    *(longlong *)(extraout_RAX_11 + 8) = extraout_RAX_12;
    *(undefined4 **)(extraout_RAX_11 + 0x10) = puVar12;
    lVar6 = extraout_RAX_12;
  }
  else {
    lVar6 = *(longlong *)(extraout_RAX_11 + 8);
    puVar12 = *(undefined4 **)(extraout_RAX_11 + 0x10);
  }
  *(longlong *)(extraout_RAX_11 + 8) = lVar6 + 1;
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX_13 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
  lVar2 = *extraout_RAX_13;
  lVar3 = extraout_RAX_13[1];
  *extraout_RAX_13 = lVar2 + 1;
  *(undefined4 *)in_stack_00000060[2] = *(undefined4 *)((longlong)in_stack_00000060 + 0x1c);
  puVar1 = (undefined8 *)*in_stack_00000060;
  core::ptr::drop_in_place<system_wall_constructor::PlatformStairsStorage>();
  *puVar1 = 0;
  puVar1[1] = 8;
  puVar1[2] = 0;
  puVar1[3] = &DAT_1429ada80;
  puVar1[4] = 0;
  puVar1[5] = 0;
  puVar1[6] = 0;
  puVar1[7] = lVar7;
  puVar1[8] = puVar11;
  puVar1[9] = &DAT_1429ada80;
  puVar1[10] = 0;
  puVar1[0xb] = 0;
  puVar1[0xc] = 0;
  puVar1[0xd] = lVar6;
  puVar1[0xe] = puVar12;
  puVar1[0xf] = &anon_e34e5d0578d4d828d43b3955f8816025_65_llvm_2456333134789696354;
  puVar1[0x10] = 0;
  puVar1[0x11] = 0;
  puVar1[0x12] = 0;
  puVar1[0x13] = lVar2;
  puVar1[0x14] = lVar3;
  uVar13 = *(undefined4 *)(in_stack_00000030 + 0x1c);
  puVar11 = *(undefined4 **)(in_stack_00000030 + 0x10);
  *puVar11 = uVar13;
  country_core::resources::walls::inner_walls::InnerWalls::remove_all();
  *puVar11 = uVar13;
  country_core::resources::walls::inner_walls::InnerWalls::reset_max_id();
  **(undefined4 **)(in_stack_00000068 + 0x10) = *(undefined4 *)(in_stack_00000068 + 0x1c);
  country_core::resources::walls::wooden_gates::Gates::clear();
  country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::new();
  *(undefined4 *)in_stack_00000038[2] = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
  puVar1 = (undefined8 *)*in_stack_00000038;
  core::ptr::
  drop_in_place<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>();
  puVar1[10] = lVar8;
  puVar1[0xb] = uVar9;
  puVar1[8] = 0;
  puVar1[9] = 0;
  puVar1[6] = &DAT_1429ada80;
  puVar1[7] = 0;
  puVar1[4] = lVar6;
  puVar1[5] = puVar12;
  puVar1[2] = 0;
  puVar1[3] = 0;
  *puVar1 = &DAT_1429ada80;
  puVar1[1] = 0;
  country_core::resources::color_id::WallColorIds::new();
  *(undefined4 *)in_stack_00000040[2] = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
  plVar4 = (longlong *)*in_stack_00000040;
  if (*plVar4 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  plVar4[2] = 0;
  *plVar4 = (longlong)&DAT_1429ada80;
  plVar4[1] = 0;
  frozen::FrozenMap::
  get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
            ();
  if (extraout_RAX_14 != (undefined8 *)0x0) {
    if ((extraout_RAX_14[3] != 0x32) ||
       (iVar5 = memcmp(&anon_5c92bc5c8d3c75d08c04de538de06c31_17_llvm_4411040535591931162,
                       (void *)extraout_RAX_14[2],0x32), iVar5 != 0)) {
                    // WARNING: Subroutine does not return
      core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
    }
    lVar7 = (**(code **)(extraout_RAX_14[1] + 0x18))(*extraout_RAX_14);
    if (lVar7 != 0) {
      country_core::render::impl$0::deref();
      country_core::resources::color_id_roof::impl$0::default_value();
      if ((int)extraout_RAX_15 == 0) {
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_16 == 0) {
                    // WARNING: Subroutine does not return
        alloc::raw_vec::handle_error();
      }
      uVar13 = (undefined4)((ulonglong)extraout_RAX_15 >> 0x20);
      lVar8 = 0x14;
      do {
        puVar11 = (undefined4 *)(extraout_RAX_16 + -0x50 + lVar8 * 4);
        *puVar11 = uVar13;
        puVar11[1] = uVar13;
        puVar11[2] = uVar13;
        puVar11[3] = uVar13;
        puVar11 = (undefined4 *)(extraout_RAX_16 + -0x40 + lVar8 * 4);
        *puVar11 = uVar13;
        puVar11[1] = uVar13;
        puVar11[2] = uVar13;
        puVar11[3] = uVar13;
        puVar11 = (undefined4 *)(extraout_RAX_16 + -0x30 + lVar8 * 4);
        *puVar11 = uVar13;
        puVar11[1] = uVar13;
        puVar11[2] = uVar13;
        puVar11[3] = uVar13;
        puVar11 = (undefined4 *)(extraout_RAX_16 + -0x20 + lVar8 * 4);
        *puVar11 = uVar13;
        puVar11[1] = uVar13;
        puVar11[2] = uVar13;
        puVar11[3] = uVar13;
        puVar11 = (undefined4 *)(extraout_RAX_16 + -0x10 + lVar8 * 4);
        *puVar11 = uVar13;
        puVar11[1] = uVar13;
        puVar11[2] = uVar13;
        puVar11[3] = uVar13;
        puVar11 = (undefined4 *)(extraout_RAX_16 + lVar8 * 4);
        *puVar11 = uVar13;
        puVar11[1] = uVar13;
        puVar11[2] = uVar13;
        puVar11[3] = uVar13;
        lVar8 = lVar8 + 0x18;
      } while (lVar8 != 0x500c);
      *(undefined4 *)(extraout_RAX_16 + 0x13fe0) = uVar13;
      *(undefined4 *)(extraout_RAX_16 + 0x13fe4) = uVar13;
      *(undefined4 *)(extraout_RAX_16 + 0x13fe8) = uVar13;
      *(undefined4 *)(extraout_RAX_16 + 0x13fec) = uVar13;
      *(undefined4 *)(extraout_RAX_16 + 0x13ff0) = uVar13;
      *(undefined4 *)(extraout_RAX_16 + 0x13ff4) = uVar13;
      *(undefined4 *)(extraout_RAX_16 + 0x13ff8) = uVar13;
      *(undefined4 *)(extraout_RAX_16 + 0x13ffc) = uVar13;
      if (0x13fff < *(ulonglong *)(lVar7 + 0x18)) {
        rhapsody::raw::device::GraphicsDevice::update_buffer_bytes();
      }
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

