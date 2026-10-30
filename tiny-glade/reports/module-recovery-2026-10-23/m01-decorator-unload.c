
void __cdecl tiny_glade::decorator_plugin::decorators_on_glade_unload(void)

{
  undefined4 uVar1;
  undefined8 *puVar2;
  undefined4 *puVar3;
  undefined8 *puVar4;
  undefined8 *puVar5;
  undefined1 auVar6 [16];
  undefined1 auVar7 [16];
  undefined1 auVar8 [16];
  undefined1 auVar9 [16];
  longlong lVar10;
  void *pvVar11;
  undefined4 extraout_EAX;
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
  longlong *extraout_RAX_09;
  byte *extraout_RAX_10;
  longlong extraout_RAX_11;
  byte *extraout_RAX_12;
  longlong extraout_RAX_13;
  byte *extraout_RAX_14;
  longlong extraout_RAX_15;
  byte *extraout_RAX_16;
  longlong extraout_RAX_17;
  byte *extraout_RAX_18;
  longlong extraout_RAX_19;
  byte *extraout_RAX_20;
  undefined8 *extraout_RAX_21;
  byte *extraout_RAX_22;
  longlong extraout_RAX_23;
  longlong *extraout_RAX_24;
  longlong *extraout_RAX_25;
  byte *extraout_RAX_26;
  longlong extraout_RAX_27;
  longlong *extraout_RAX_28;
  byte *extraout_RAX_29;
  longlong extraout_RAX_30;
  longlong *extraout_RAX_31;
  longlong *extraout_RAX_32;
  longlong *extraout_RAX_33;
  longlong *extraout_RAX_34;
  longlong *extraout_RAX_35;
  byte *extraout_RAX_36;
  longlong extraout_RAX_37;
  undefined8 *in_RCX;
  uint *in_RDX;
  uint *puVar12;
  longlong lVar13;
  ulonglong uVar14;
  longlong lVar15;
  longlong *plVar16;
  longlong lVar17;
  uint *puVar18;
  uint *puVar19;
  undefined *local_5d8;
  undefined8 uStack_5d0;
  undefined8 local_5c8;
  undefined8 uStack_5c0;
  longlong local_5b8;
  uint *puStack_5b0;
  undefined *local_548;
  undefined8 uStack_540;
  undefined8 local_538;
  undefined8 uStack_530;
  longlong local_528;
  uint *puStack_520;
  uint local_4b8;
  undefined4 uStack_4b4;
  undefined4 uStack_4b0;
  uint uStack_4ac;
  undefined8 local_4a8;
  undefined8 uStack_4a0;
  longlong local_498;
  uint *puStack_490;
  undefined *local_488;
  undefined8 uStack_480;
  undefined8 local_478;
  undefined8 uStack_470;
  longlong local_468;
  uint *puStack_460;
  undefined *local_458;
  undefined8 uStack_450;
  undefined8 local_448;
  undefined8 uStack_440;
  longlong local_438;
  uint *puStack_430;
  undefined *local_428;
  undefined8 uStack_420;
  undefined8 local_418;
  undefined8 uStack_410;
  undefined4 *local_2d0;
  undefined4 *local_2c8;
  undefined4 *local_2c0;
  undefined4 *local_2b8;
  undefined4 *local_2b0;
  undefined4 *local_2a8;
  uint *local_2a0;
  undefined4 *local_298;
  undefined4 *local_290;
  undefined4 *local_288;
  uint *local_280;
  undefined4 *local_278;
  undefined4 *local_270;
  uint *local_268;
  uint *local_260;
  longlong *local_258;
  undefined8 local_250;
  undefined4 *local_248;
  longlong *local_240;
  undefined8 local_238;
  uint *local_230;
  undefined *local_228;
  undefined8 uStack_220;
  undefined8 local_218;
  undefined8 uStack_210;
  longlong local_208;
  uint *puStack_200;
  undefined4 *local_190;
  longlong local_188;
  undefined8 *local_180;
  undefined4 *local_178;
  undefined4 *local_170;
  uint *local_168;
  undefined4 local_15c;
  undefined4 local_158;
  undefined4 local_154;
  undefined4 local_150;
  undefined4 local_14c;
  undefined4 local_148;
  undefined4 local_144;
  undefined4 local_140;
  uint local_13c;
  undefined4 local_138;
  undefined4 local_134;
  uint local_130;
  uint local_12c;
  undefined4 local_128;
  uint local_124;
  void *local_120;
  undefined8 *local_118;
  undefined8 *local_110;
  undefined8 *local_108;
  undefined8 *local_100;
  void *local_f8;
  undefined8 *local_f0;
  void *local_e8;
  undefined8 *local_e0;
  undefined8 *local_d8;
  undefined4 local_cc;
  undefined4 local_c8;
  uint local_c4;
  uint *local_c0;
  longlong local_b8;
  undefined8 *local_b0;
  uint *local_a8;
  undefined8 local_a0;
  
  local_a0 = 0xfffffffffffffffe;
  puVar2 = (undefined8 *)*in_RCX;
  puVar3 = (undefined4 *)in_RCX[2];
  uVar1 = *(undefined4 *)((longlong)in_RCX + 0x1c);
  local_a8 = (uint *)in_RCX[4];
  local_190 = (undefined4 *)in_RCX[6];
  local_b8 = CONCAT44(local_b8._4_4_,*(undefined4 *)((longlong)in_RCX + 0x3c));
  local_b0 = (undefined8 *)in_RCX[8];
  local_178 = (undefined4 *)in_RCX[10];
  local_cc = *(undefined4 *)((longlong)in_RCX + 0x5c);
  local_120 = (void *)in_RCX[0xc];
  local_2d0 = (undefined4 *)in_RCX[0xe];
  local_15c = *(undefined4 *)((longlong)in_RCX + 0x7c);
  plVar16 = (longlong *)in_RCX[0x10];
  local_2c8 = (undefined4 *)in_RCX[0x12];
  local_158 = *(undefined4 *)((longlong)in_RCX + 0x9c);
  local_f0 = (undefined8 *)in_RCX[0x14];
  local_2c0 = (undefined4 *)in_RCX[0x16];
  local_154 = *(undefined4 *)((longlong)in_RCX + 0xbc);
  local_2b8 = (undefined4 *)in_RCX[0x18];
  local_2a8 = (undefined4 *)in_RCX[0x1a];
  local_14c = *(undefined4 *)((longlong)in_RCX + 0xdc);
  local_2a0 = (uint *)in_RCX[0x1c];
  local_2b0 = (undefined4 *)in_RCX[0x1e];
  local_150 = *(undefined4 *)((longlong)in_RCX + 0xfc);
  local_e8 = (void *)in_RCX[0x20];
  local_298 = (undefined4 *)in_RCX[0x22];
  local_148 = *(undefined4 *)((longlong)in_RCX + 0x11c);
  puVar4 = (undefined8 *)in_RCX[0x24];
  local_290 = (undefined4 *)in_RCX[0x26];
  local_144 = *(undefined4 *)((longlong)in_RCX + 0x13c);
  local_250 = in_RCX[0x28];
  local_248 = (undefined4 *)in_RCX[0x2a];
  local_128 = *(undefined4 *)((longlong)in_RCX + 0x15c);
  puVar5 = (undefined8 *)in_RCX[0x2c];
  local_288 = (undefined4 *)in_RCX[0x2e];
  local_140 = *(undefined4 *)((longlong)in_RCX + 0x17c);
  local_118 = (undefined8 *)in_RCX[0x30];
  local_280 = (uint *)in_RCX[0x32];
  local_13c = *(uint *)((longlong)in_RCX + 0x19c);
  local_e0 = (undefined8 *)in_RCX[0x34];
  local_170 = (undefined4 *)in_RCX[0x36];
  local_c8 = *(undefined4 *)((longlong)in_RCX + 0x1bc);
  local_110 = (undefined8 *)in_RCX[0x38];
  local_278 = (undefined4 *)in_RCX[0x3a];
  local_138 = *(undefined4 *)((longlong)in_RCX + 0x1dc);
  local_108 = (undefined8 *)in_RCX[0x3c];
  local_270 = (undefined4 *)in_RCX[0x3e];
  local_134 = *(undefined4 *)((longlong)in_RCX + 0x1fc);
  local_f8 = *(void **)in_RDX;
  local_268 = *(uint **)(in_RDX + 4);
  local_130 = in_RDX[7];
  local_100 = *(undefined8 **)(in_RDX + 8);
  local_260 = *(uint **)(in_RDX + 0xc);
  local_12c = in_RDX[0xf];
  local_d8 = *(undefined8 **)(in_RDX + 0x10);
  local_168 = *(uint **)(in_RDX + 0x14);
  local_c4 = in_RDX[0x17];
  local_238 = *(undefined8 *)(in_RDX + 0x18);
  local_230 = *(uint **)(in_RDX + 0x1c);
  local_124 = in_RDX[0x1f];
  local_240 = *(longlong **)(in_RDX + 0x22);
  local_258 = *(longlong **)(in_RDX + 0x24);
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
    *(uint **)(extraout_RAX + 0x10) = in_RDX;
    local_528 = extraout_RAX_00;
  }
  else {
    local_528 = *(longlong *)(extraout_RAX + 8);
    in_RDX = *(uint **)(extraout_RAX + 0x10);
  }
  *(longlong *)(extraout_RAX + 8) = local_528 + 1;
  local_538 = 0;
  uStack_530 = 0;
  local_548 = &DAT_1429c2e40;
  uStack_540 = 0;
  puStack_520 = in_RDX;
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
    *(uint **)(extraout_RAX_01 + 0x10) = in_RDX;
    local_208 = extraout_RAX_02;
  }
  else {
    local_208 = *(longlong *)(extraout_RAX_01 + 8);
    in_RDX = *(uint **)(extraout_RAX_01 + 0x10);
  }
  *(longlong *)(extraout_RAX_01 + 8) = local_208 + 1;
  local_218 = 0;
  uStack_210 = 0;
  local_228 = &DAT_1429c2e40;
  uStack_220 = 0;
  puStack_200 = in_RDX;
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
    *(uint **)(extraout_RAX_03 + 0x10) = in_RDX;
    local_188 = extraout_RAX_04;
    local_c0 = in_RDX;
  }
  else {
    local_188 = *(longlong *)(extraout_RAX_03 + 8);
    local_c0 = *(uint **)(extraout_RAX_03 + 0x10);
  }
  *(longlong *)(extraout_RAX_03 + 8) = local_188 + 1;
  local_498 = local_528;
  puStack_490 = puStack_520;
  local_4a8 = local_538;
  uStack_4a0 = uStack_530;
  local_4b8 = (uint)local_548;
  uStack_4b4 = (undefined4)((ulonglong)local_548 >> 0x20);
  uStack_4b0 = (undefined4)uStack_540;
  uStack_4ac = (uint)((ulonglong)uStack_540 >> 0x20);
  local_488 = local_228;
  uStack_480 = uStack_220;
  local_478 = local_218;
  uStack_470 = uStack_210;
  local_468 = local_208;
  puStack_460 = puStack_200;
  local_458 = &DAT_1429c2e40;
  uStack_450 = 0;
  local_448 = 0;
  uStack_440 = 0;
  *puVar3 = uVar1;
  local_180 = puVar2;
  core::ptr::drop_in_place<country_core::resources::walls::decorator_storage::DecoratorStorage>();
  local_180[0xe] = local_448;
  local_180[0xf] = uStack_440;
  local_180[0xc] = local_458;
  local_180[0xd] = uStack_450;
  local_180[10] = local_468;
  local_180[0xb] = puStack_460;
  local_180[8] = local_478;
  local_180[9] = uStack_470;
  local_180[6] = local_488;
  local_180[7] = uStack_480;
  local_180[4] = local_498;
  local_180[5] = puStack_490;
  local_180[2] = local_4a8;
  local_180[3] = uStack_4a0;
  *local_180 = CONCAT44(uStack_4b4,local_4b8);
  local_180[1] = CONCAT44(uStack_4ac,uStack_4b0);
  local_180[0x10] = local_188;
  local_180[0x11] = local_c0;
  local_180[0x12] = 0;
  local_180[0x24] = 0;
  *(undefined1 *)(local_180 + 0x36) = 0;
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
    *(uint **)(extraout_RAX_05 + 0x10) = in_RDX;
    lVar13 = extraout_RAX_06;
    local_c0 = in_RDX;
  }
  else {
    lVar13 = *(longlong *)(extraout_RAX_05 + 8);
    local_c0 = *(uint **)(extraout_RAX_05 + 0x10);
  }
  *(longlong *)(extraout_RAX_05 + 8) = lVar13 + 1;
  *local_190 = (undefined4)local_b8;
  puVar12 = (uint *)((longlong)local_a8 + 0x20);
  local_b8 = lVar13;
  hashbrown::raw::RawTableInner::
  drop_inner_table<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::sync::Arc<system_decorator::blueprint::DecoratorBlueprint,alloc::alloc::Global>_>,alloc::alloc::Global>
            ();
  *(undefined8 *)((longlong)local_a8 + 0x10) = 0;
  *(undefined8 *)((longlong)local_a8 + 0x18) = 0;
  *(undefined **)local_a8 = &DAT_1429c2e40;
  *(undefined8 *)((longlong)local_a8 + 8) = 0;
  *(longlong *)((longlong)local_a8 + 0x20) = local_b8;
  *(uint **)((longlong)local_a8 + 0x28) = local_c0;
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
    *(uint **)(extraout_RAX_07 + 0x10) = puVar12;
    local_b8 = extraout_RAX_08;
    local_a8 = puVar12;
  }
  else {
    local_b8 = *(longlong *)(extraout_RAX_07 + 8);
    local_a8 = *(uint **)(extraout_RAX_07 + 0x10);
  }
  *(longlong *)(extraout_RAX_07 + 8) = local_b8 + 1;
  *local_178 = local_cc;
  hashbrown::raw::RawTableInner::
  drop_inner_table<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::sync::Arc<system_decorator::blueprint::DecoratorBlueprint,alloc::alloc::Global>_>,alloc::alloc::Global>
            ();
  local_b0[2] = 0;
  local_b0[3] = 0;
  *local_b0 = &DAT_1429c2e40;
  local_b0[1] = 0;
  local_b0[4] = local_b8;
  local_b0[5] = local_a8;
  country_core::resources::walls::decorator_archivist::impl$1::default();
  *local_2d0 = local_15c;
  core::ptr::drop_in_place<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
            ();
  memcpy(local_120,&local_4b8,0x1e8);
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX_09 != (longlong *)0x0) {
    lVar15 = *extraout_RAX_09;
    lVar10 = extraout_RAX_09[1];
    *extraout_RAX_09 = *extraout_RAX_09 + 1;
    *local_2c8 = local_158;
    lVar13 = 0;
    if (*plVar16 != 0) {
      lVar13 = *plVar16 << 5;
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    lVar17 = plVar16[4];
    if (lVar17 != 0) {
      auVar6._8_8_ = 0;
      auVar6._0_8_ = lVar17 + 1;
      lVar17 = lVar17 + (SUB168(auVar6 * ZEXT816(0x18),0) + 0xfU & 0xfffffffffffffff0) + 0x11;
      lVar13 = SUB168(auVar6 * ZEXT816(0x18),8);
      if (lVar17 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
        lVar13 = lVar17;
      }
    }
    *plVar16 = 0;
    plVar16[1] = 8;
    plVar16[2] = 0;
    plVar16[5] = 0;
    plVar16[6] = 0;
    plVar16[3] = (longlong)&anon_a85850e2b41faa58b5d70f5d3927ab67_1_llvm_4557221853777664021;
    plVar16[4] = 0;
    plVar16[7] = lVar15;
    plVar16[8] = lVar10;
    *(undefined1 *)(plVar16 + 9) = 0;
    std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
    if ((*extraout_RAX_10 & 1) == 0) {
      std::sys::random::hashmap_random_keys();
      extraout_RAX_10[0] = 1;
      extraout_RAX_10[1] = 0;
      extraout_RAX_10[2] = 0;
      extraout_RAX_10[3] = 0;
      extraout_RAX_10[4] = 0;
      extraout_RAX_10[5] = 0;
      extraout_RAX_10[6] = 0;
      extraout_RAX_10[7] = 0;
      *(longlong *)(extraout_RAX_10 + 8) = extraout_RAX_11;
      *(longlong *)(extraout_RAX_10 + 0x10) = lVar13;
      local_498 = extraout_RAX_11;
    }
    else {
      local_498 = *(longlong *)(extraout_RAX_10 + 8);
      lVar13 = *(longlong *)(extraout_RAX_10 + 0x10);
    }
    *(longlong *)(extraout_RAX_10 + 8) = local_498 + 1;
    local_4a8 = 0;
    uStack_4a0 = 0;
    local_4b8 = 0x429c2e40;
    uStack_4b4 = 1;
    uStack_4b0 = 0;
    uStack_4ac = 0;
    puStack_490 = (uint *)lVar13;
    std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
    if ((*extraout_RAX_12 & 1) == 0) {
      std::sys::random::hashmap_random_keys();
      extraout_RAX_12[0] = 1;
      extraout_RAX_12[1] = 0;
      extraout_RAX_12[2] = 0;
      extraout_RAX_12[3] = 0;
      extraout_RAX_12[4] = 0;
      extraout_RAX_12[5] = 0;
      extraout_RAX_12[6] = 0;
      extraout_RAX_12[7] = 0;
      *(longlong *)(extraout_RAX_12 + 8) = extraout_RAX_13;
      *(longlong *)(extraout_RAX_12 + 0x10) = lVar13;
      local_b0 = (undefined8 *)extraout_RAX_13;
      local_a8 = (uint *)lVar13;
    }
    else {
      local_b0 = *(undefined8 **)(extraout_RAX_12 + 8);
      local_a8 = (uint *)*(longlong *)(extraout_RAX_12 + 0x10);
    }
    *(longlong *)(extraout_RAX_12 + 8) = (longlong)local_b0 + 1;
    local_228 = (undefined *)CONCAT44(uStack_4b4,local_4b8);
    uStack_220 = CONCAT44(uStack_4ac,uStack_4b0);
    local_208 = local_498;
    puStack_200 = puStack_490;
    local_218 = local_4a8;
    uStack_210 = uStack_4a0;
    *local_2c0 = local_154;
    core::ptr::drop_in_place<system_decorator::decorator_visual_state::DecoratorVisualState>();
    *local_f0 = 0;
    local_f0[1] = 0x10;
    local_f0[2] = 0;
    local_f0[3] = local_228;
    local_f0[4] = uStack_220;
    local_f0[5] = local_218;
    local_f0[6] = uStack_210;
    local_f0[7] = local_208;
    local_f0[8] = puStack_200;
    local_f0[9] = &DAT_1429c2e40;
    local_f0[10] = 0;
    local_f0[0xb] = 0;
    local_f0[0xc] = 0;
    local_f0[0xd] = local_b0;
    local_f0[0xe] = local_a8;
    _ZN100__LT_country_core__components__drawable__WireframePass_u20_as_u20_bevy_ecs__component__Component_GT_14clone_behavior17h98754d446e2f5bc0E
              ();
    *local_2a8 = local_14c;
    *local_2b8 = extraout_EAX;
    uStack_4ac = uStack_4ac & 0xff000000;
    uStack_4b4 = 0;
    uStack_4b0 = 0;
    *local_2b0 = local_150;
    *local_2a0 = 0;
    *(ulonglong *)(local_2a0 + 1) = (ulonglong)local_4b8;
    *(undefined8 *)((longlong)local_2a0 + 0xb) = 0;
    puVar12 = local_2a0;
    std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
    if ((*extraout_RAX_14 & 1) == 0) {
      std::sys::random::hashmap_random_keys();
      extraout_RAX_14[0] = 1;
      extraout_RAX_14[1] = 0;
      extraout_RAX_14[2] = 0;
      extraout_RAX_14[3] = 0;
      extraout_RAX_14[4] = 0;
      extraout_RAX_14[5] = 0;
      extraout_RAX_14[6] = 0;
      extraout_RAX_14[7] = 0;
      *(longlong *)(extraout_RAX_14 + 8) = extraout_RAX_15;
      *(uint **)(extraout_RAX_14 + 0x10) = puVar12;
      local_5b8 = extraout_RAX_15;
    }
    else {
      local_5b8 = *(longlong *)(extraout_RAX_14 + 8);
      puVar12 = *(uint **)(extraout_RAX_14 + 0x10);
    }
    *(longlong *)(extraout_RAX_14 + 8) = local_5b8 + 1;
    local_5c8 = 0;
    uStack_5c0 = 0;
    local_5d8 = &DAT_1429c2e40;
    uStack_5d0 = 0;
    puStack_5b0 = puVar12;
    std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
    if ((*extraout_RAX_16 & 1) == 0) {
      std::sys::random::hashmap_random_keys();
      extraout_RAX_16[0] = 1;
      extraout_RAX_16[1] = 0;
      extraout_RAX_16[2] = 0;
      extraout_RAX_16[3] = 0;
      extraout_RAX_16[4] = 0;
      extraout_RAX_16[5] = 0;
      extraout_RAX_16[6] = 0;
      extraout_RAX_16[7] = 0;
      *(longlong *)(extraout_RAX_16 + 8) = extraout_RAX_17;
      *(uint **)(extraout_RAX_16 + 0x10) = puVar12;
      local_528 = extraout_RAX_17;
    }
    else {
      local_528 = *(longlong *)(extraout_RAX_16 + 8);
      puVar12 = *(uint **)(extraout_RAX_16 + 0x10);
    }
    *(longlong *)(extraout_RAX_16 + 8) = local_528 + 1;
    local_538 = 0;
    uStack_530 = 0;
    local_548 = &DAT_1429c2e40;
    uStack_540 = 0;
    puStack_520 = puVar12;
    std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
    if ((*extraout_RAX_18 & 1) == 0) {
      std::sys::random::hashmap_random_keys();
      extraout_RAX_18[0] = 1;
      extraout_RAX_18[1] = 0;
      extraout_RAX_18[2] = 0;
      extraout_RAX_18[3] = 0;
      extraout_RAX_18[4] = 0;
      extraout_RAX_18[5] = 0;
      extraout_RAX_18[6] = 0;
      extraout_RAX_18[7] = 0;
      *(longlong *)(extraout_RAX_18 + 8) = extraout_RAX_19;
      *(uint **)(extraout_RAX_18 + 0x10) = puVar12;
      local_208 = extraout_RAX_19;
    }
    else {
      local_208 = *(longlong *)(extraout_RAX_18 + 8);
      puVar12 = *(uint **)(extraout_RAX_18 + 0x10);
    }
    *(longlong *)(extraout_RAX_18 + 8) = local_208 + 1;
    local_218 = 0;
    uStack_210 = 0;
    local_228 = &DAT_1429c2e40;
    uStack_220 = 0;
    puStack_200 = puVar12;
    std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
    if ((*extraout_RAX_20 & 1) == 0) {
      std::sys::random::hashmap_random_keys();
      extraout_RAX_20[0] = 1;
      extraout_RAX_20[1] = 0;
      extraout_RAX_20[2] = 0;
      extraout_RAX_20[3] = 0;
      extraout_RAX_20[4] = 0;
      extraout_RAX_20[5] = 0;
      extraout_RAX_20[6] = 0;
      extraout_RAX_20[7] = 0;
      *(undefined8 **)(extraout_RAX_20 + 8) = extraout_RAX_21;
      *(uint **)(extraout_RAX_20 + 0x10) = puVar12;
      local_b0 = extraout_RAX_21;
      local_a8 = puVar12;
    }
    else {
      local_b0 = *(undefined8 **)(extraout_RAX_20 + 8);
      local_a8 = *(uint **)(extraout_RAX_20 + 0x10);
    }
    *(longlong *)(extraout_RAX_20 + 8) = (longlong)local_b0 + 1;
    local_498 = local_5b8;
    puStack_490 = puStack_5b0;
    local_4a8 = local_5c8;
    uStack_4a0 = uStack_5c0;
    local_4b8 = (uint)local_5d8;
    uStack_4b4 = (undefined4)((ulonglong)local_5d8 >> 0x20);
    uStack_4b0 = (undefined4)uStack_5d0;
    uStack_4ac = (uint)((ulonglong)uStack_5d0 >> 0x20);
    local_488 = local_548;
    uStack_480 = uStack_540;
    local_478 = local_538;
    uStack_470 = uStack_530;
    local_468 = local_528;
    puStack_460 = puStack_520;
    local_458 = local_228;
    uStack_450 = uStack_220;
    local_448 = local_218;
    uStack_440 = uStack_210;
    local_438 = local_208;
    puStack_430 = puStack_200;
    local_418 = 0;
    uStack_410 = 0;
    local_428 = &DAT_1429c2e40;
    uStack_420 = 0;
    *local_298 = local_148;
    core::ptr::drop_in_place<system_decorator::audio::op_confirm_audio::DecoratorAudioEvents>();
    pvVar11 = local_e8;
    puVar12 = &local_4b8;
    memcpy(local_e8,puVar12,0xb0);
    *(undefined8 **)((longlong)pvVar11 + 0xb0) = local_b0;
    *(uint **)((longlong)pvVar11 + 0xb8) = local_a8;
    std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
    if ((*extraout_RAX_22 & 1) == 0) {
      std::sys::random::hashmap_random_keys();
      extraout_RAX_22[0] = 1;
      extraout_RAX_22[1] = 0;
      extraout_RAX_22[2] = 0;
      extraout_RAX_22[3] = 0;
      extraout_RAX_22[4] = 0;
      extraout_RAX_22[5] = 0;
      extraout_RAX_22[6] = 0;
      extraout_RAX_22[7] = 0;
      *(longlong *)(extraout_RAX_22 + 8) = extraout_RAX_23;
      *(uint **)(extraout_RAX_22 + 0x10) = puVar12;
      lVar13 = extraout_RAX_23;
      puVar19 = puVar12;
    }
    else {
      lVar13 = *(longlong *)(extraout_RAX_22 + 8);
      puVar19 = *(uint **)(extraout_RAX_22 + 0x10);
    }
    *(longlong *)(extraout_RAX_22 + 8) = lVar13 + 1;
    *local_290 = local_144;
    lVar15 = puVar4[1];
    if (lVar15 != 0) {
      auVar7._8_8_ = 0;
      auVar7._0_8_ = lVar15 + 1;
      puVar18 = (uint *)(lVar15 + (SUB168(auVar7 * ZEXT816(0x18),0) + 0xfU & 0xfffffffffffffff0) +
                        0x11);
      puVar12 = SUB168(auVar7 * ZEXT816(0x18),8);
      if (puVar18 != (uint *)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
        puVar12 = puVar18;
      }
    }
    puVar4[2] = 0;
    puVar4[3] = 0;
    *puVar4 = &DAT_1429c2e40;
    puVar4[1] = 0;
    puVar4[4] = lVar13;
    puVar4[5] = puVar19;
    *(undefined1 *)(puVar4 + 6) = 0;
    core::ops::function::FnOnce::
    call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
              ();
    if (extraout_RAX_24 != (longlong *)0x0) {
      local_208 = *extraout_RAX_24;
      puStack_200 = (uint *)extraout_RAX_24[1];
      *extraout_RAX_24 = *extraout_RAX_24 + 1;
      local_218 = 0;
      uStack_210 = 0;
      local_228 = &anon_e34e5d0578d4d828d43b3955f8816025_65_llvm_2456333134789696354;
      uStack_220 = 0;
      core::ops::function::FnOnce::
      call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                ();
      if (extraout_RAX_25 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      lVar15 = *extraout_RAX_25;
      lVar10 = extraout_RAX_25[1];
      *extraout_RAX_25 = *extraout_RAX_25 + 1;
      local_498 = local_208;
      puStack_490 = puStack_200;
      local_4a8 = local_218;
      uStack_4a0 = uStack_210;
      local_4b8 = (uint)local_228;
      uStack_4b4 = (undefined4)((ulonglong)local_228 >> 0x20);
      uStack_4b0 = (undefined4)uStack_220;
      uStack_4ac = (uint)((ulonglong)uStack_220 >> 0x20);
      local_488 = &anon_e34e5d0578d4d828d43b3955f8816025_65_llvm_2456333134789696354;
      uStack_480 = 0;
      local_478 = 0;
      uStack_470 = 0;
      *local_288 = local_140;
      lVar13 = puVar5[1];
      if (lVar13 != 0) {
        auVar8._8_8_ = 0;
        auVar8._0_8_ = lVar13 + 1;
        puVar19 = (uint *)(lVar13 + (SUB168(auVar8 * ZEXT816(0x18),0) + 0xfU & 0xfffffffffffffff0) +
                          0x11);
        puVar12 = SUB168(auVar8 * ZEXT816(0x18),8);
        if (puVar19 != (uint *)0x0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
          puVar12 = puVar19;
        }
      }
      lVar13 = puVar5[7];
      if (lVar13 != 0) {
        auVar9._8_8_ = 0;
        auVar9._0_8_ = lVar13 + 1;
        puVar19 = (uint *)(lVar13 + (SUB168(auVar9 * ZEXT816(0x18),0) + 0xfU & 0xfffffffffffffff0) +
                          0x11);
        puVar12 = SUB168(auVar9 * ZEXT816(0x18),8);
        if (puVar19 != (uint *)0x0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
          puVar12 = puVar19;
        }
      }
      puVar5[8] = local_478;
      puVar5[9] = uStack_470;
      puVar5[6] = local_488;
      puVar5[7] = uStack_480;
      puVar5[4] = local_498;
      puVar5[5] = puStack_490;
      puVar5[2] = local_4a8;
      puVar5[3] = uStack_4a0;
      *puVar5 = CONCAT44(uStack_4b4,local_4b8);
      puVar5[1] = CONCAT44(uStack_4ac,uStack_4b0);
      puVar5[10] = lVar15;
      puVar5[0xb] = lVar10;
      std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
      if ((*extraout_RAX_26 & 1) == 0) {
        std::sys::random::hashmap_random_keys();
        extraout_RAX_26[0] = 1;
        extraout_RAX_26[1] = 0;
        extraout_RAX_26[2] = 0;
        extraout_RAX_26[3] = 0;
        extraout_RAX_26[4] = 0;
        extraout_RAX_26[5] = 0;
        extraout_RAX_26[6] = 0;
        extraout_RAX_26[7] = 0;
        *(longlong *)(extraout_RAX_26 + 8) = extraout_RAX_27;
        *(uint **)(extraout_RAX_26 + 0x10) = puVar12;
        local_498 = extraout_RAX_27;
      }
      else {
        local_498 = *(longlong *)(extraout_RAX_26 + 8);
        puVar12 = *(uint **)(extraout_RAX_26 + 0x10);
      }
      *(longlong *)(extraout_RAX_26 + 8) = local_498 + 1;
      local_4a8 = 0;
      uStack_4a0 = 0;
      local_4b8 = 0x429c2e40;
      uStack_4b4 = 1;
      uStack_4b0 = 0;
      uStack_4ac = 0;
      puStack_490 = puVar12;
      core::ops::function::FnOnce::
      call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                ();
      if (extraout_RAX_28 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      local_b0 = (undefined8 *)*extraout_RAX_28;
      local_a8 = (uint *)extraout_RAX_28[1];
      *extraout_RAX_28 = (longlong)local_b0 + 1;
      local_228 = (undefined *)CONCAT44(uStack_4b4,local_4b8);
      uStack_220 = CONCAT44(uStack_4ac,uStack_4b0);
      local_208 = local_498;
      puStack_200 = puStack_490;
      local_218 = local_4a8;
      uStack_210 = uStack_4a0;
      uVar14 = (ulonglong)local_13c;
      *local_280 = local_13c;
      core::ptr::
      drop_in_place<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>
                ();
      *local_118 = 0;
      local_118[1] = 8;
      local_118[2] = 0;
      local_118[3] = local_228;
      local_118[4] = uStack_220;
      local_118[5] = local_218;
      local_118[6] = uStack_210;
      local_118[7] = local_208;
      local_118[8] = puStack_200;
      local_118[9] = &anon_e34e5d0578d4d828d43b3955f8816025_65_llvm_2456333134789696354;
      local_118[10] = 0;
      local_118[0xb] = 0;
      local_118[0xc] = 0;
      local_118[0xd] = local_b0;
      local_118[0xe] = local_a8;
      std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
      if ((*extraout_RAX_29 & 1) == 0) {
        std::sys::random::hashmap_random_keys();
        extraout_RAX_29[0] = 1;
        extraout_RAX_29[1] = 0;
        extraout_RAX_29[2] = 0;
        extraout_RAX_29[3] = 0;
        extraout_RAX_29[4] = 0;
        extraout_RAX_29[5] = 0;
        extraout_RAX_29[6] = 0;
        extraout_RAX_29[7] = 0;
        *(longlong *)(extraout_RAX_29 + 8) = extraout_RAX_30;
        *(ulonglong *)(extraout_RAX_29 + 0x10) = uVar14;
        local_b0 = (undefined8 *)extraout_RAX_30;
        local_a8 = (uint *)uVar14;
      }
      else {
        local_b0 = *(undefined8 **)(extraout_RAX_29 + 8);
        local_a8 = (uint *)*(ulonglong *)(extraout_RAX_29 + 0x10);
      }
      *(longlong *)(extraout_RAX_29 + 8) = (longlong)local_b0 + 1;
      *local_170 = local_c8;
      hashbrown::raw::RawTableInner::
      drop_inner_table<tuple_<country_core::resources::walls::WallId,country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo>_>,alloc::alloc::Global>
                ();
      local_e0[2] = 0;
      local_e0[3] = 0;
      *local_e0 = &DAT_1429c2e40;
      local_e0[1] = 0;
      local_e0[4] = local_b0;
      local_e0[5] = local_a8;
      system_decorator::decorator_visual::deco_differential_state::impl$2::
      default<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyParams>();
      *local_278 = local_138;
      hashbrown::raw::RawTableInner::
      drop_inner_table<tuple_<country_core::resources::walls::WallId,system_decorator::decorator_visual::deco_differential_state::WallDecoDiff<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyParams>_>,alloc::alloc::Global>
                ();
      puVar2 = local_110;
      lVar13 = local_110[7];
      if ((lVar13 != 0) && (lVar13 + (lVar13 * 8 + 0x17U & 0xfffffffffffffff0) != -0x11)) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      puVar2[10] = local_468;
      puVar2[0xb] = puStack_460;
      puVar2[8] = local_478;
      puVar2[9] = uStack_470;
      puVar2[6] = local_488;
      puVar2[7] = uStack_480;
      puVar2[4] = local_498;
      puVar2[5] = puStack_490;
      puVar2[2] = local_4a8;
      puVar2[3] = uStack_4a0;
      *puVar2 = CONCAT44(uStack_4b4,local_4b8);
      puVar2[1] = CONCAT44(uStack_4ac,uStack_4b0);
      petgraph::graphmap::impl$19::
      default<country_core::resources::walls::decorator::DecoratorId,tuple_<>,enum2_<petgraph::Undirected>,std::hash::random::RandomState>
                ();
      petgraph::graphmap::impl$19::
      default<country_core::resources::walls::decorator::DecoratorId,tuple_<>,enum2_<petgraph::Undirected>,std::hash::random::RandomState>
                ();
      core::ops::function::FnOnce::
      call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                ();
      if (extraout_RAX_31 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      local_b0 = (undefined8 *)*extraout_RAX_31;
      local_a8 = (uint *)extraout_RAX_31[1];
      *extraout_RAX_31 = (longlong)local_b0 + 1;
      memcpy(&local_4a8,&local_548,0x90);
      memcpy(&local_5d8,&local_228,0x90);
      *local_270 = local_134;
      core::ptr::drop_in_place<country_core::resources::stairs::stairs_state::StairsState>();
      puVar2 = local_108;
      *local_108 = 0;
      local_108[2] = 3;
      memcpy(local_108 + 3,&local_4b8,0xa0);
      memcpy(puVar2 + 0x17,&local_5d8,0x90);
      puVar2[0x29] = &anon_a85850e2b41faa58b5d70f5d3927ab67_1_llvm_4557221853777664021;
      puVar2[0x2a] = 0;
      puVar2[0x2b] = 0;
      puVar2[0x2c] = 0;
      puVar2[0x2d] = local_b0;
      puVar2[0x2e] = local_a8;
      *(undefined1 *)(puVar2 + 0x2f) = 0;
      core::ops::function::FnOnce::
      call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                ();
      if (extraout_RAX_32 != (longlong *)0x0) {
        local_5b8 = *extraout_RAX_32;
        puStack_5b0 = (uint *)extraout_RAX_32[1];
        *extraout_RAX_32 = *extraout_RAX_32 + 1;
        local_5c8 = 0;
        uStack_5c0 = 0;
        local_5d8 = &anon_e34e5d0578d4d828d43b3955f8816025_65_llvm_2456333134789696354;
        uStack_5d0 = 0;
        core::ops::function::FnOnce::
        call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                  ();
        if (extraout_RAX_33 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
          std::thread::local::panic_access_error();
        }
        local_528 = *extraout_RAX_33;
        puStack_520 = (uint *)extraout_RAX_33[1];
        *extraout_RAX_33 = *extraout_RAX_33 + 1;
        local_538 = 0;
        uStack_530 = 0;
        local_548 = &anon_e34e5d0578d4d828d43b3955f8816025_65_llvm_2456333134789696354;
        uStack_540 = 0;
        core::ops::function::FnOnce::
        call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                  ();
        if (extraout_RAX_34 != (longlong *)0x0) {
          local_208 = *extraout_RAX_34;
          puStack_200 = (uint *)extraout_RAX_34[1];
          *extraout_RAX_34 = *extraout_RAX_34 + 1;
          local_218 = 0;
          uStack_210 = 0;
          local_228 = &anon_e34e5d0578d4d828d43b3955f8816025_65_llvm_2456333134789696354;
          uStack_220 = 0;
          core::ops::function::FnOnce::
          call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                    ();
          if (extraout_RAX_35 != (longlong *)0x0) {
            local_b0 = (undefined8 *)*extraout_RAX_35;
            local_a8 = (uint *)extraout_RAX_35[1];
            *extraout_RAX_35 = (longlong)local_b0 + 1;
            local_498 = local_5b8;
            puStack_490 = puStack_5b0;
            local_4a8 = local_5c8;
            uStack_4a0 = uStack_5c0;
            local_4b8 = (uint)local_5d8;
            uStack_4b4 = (undefined4)((ulonglong)local_5d8 >> 0x20);
            uStack_4b0 = (undefined4)uStack_5d0;
            uStack_4ac = (uint)((ulonglong)uStack_5d0 >> 0x20);
            local_488 = local_548;
            uStack_480 = uStack_540;
            local_478 = local_538;
            uStack_470 = uStack_530;
            local_468 = local_528;
            puStack_460 = puStack_520;
            local_458 = local_228;
            uStack_450 = uStack_220;
            local_448 = local_218;
            uStack_440 = uStack_210;
            local_438 = local_208;
            puStack_430 = puStack_200;
            local_418 = 0;
            uStack_410 = 0;
            local_428 = &anon_e34e5d0578d4d828d43b3955f8816025_65_llvm_2456333134789696354;
            uStack_420 = 0;
            *local_268 = local_130;
            core::ptr::
            drop_in_place<country_core::resources::stairs::stairs_collider::StairColliderRegistry>()
            ;
            pvVar11 = local_f8;
            memcpy(local_f8,&local_4b8,0xb0);
            *(undefined8 **)((longlong)pvVar11 + 0xb0) = local_b0;
            *(uint **)((longlong)pvVar11 + 0xb8) = local_a8;
            system_decorator::decorator_visual::deco_differential_state::impl$2::
            default<system_decorator::decorator_visual::door_stairs_assemble_bricks::DoorStairsAssemblyParams>
                      ();
            *local_260 = local_12c;
            hashbrown::raw::RawTableInner::
            drop_inner_table<tuple_<country_core::resources::walls::WallId,system_decorator::decorator_visual::deco_differential_state::WallDecoDiff<system_decorator::decorator_visual::door_stairs_assemble_bricks::DoorStairsAssemblyParams>_>,alloc::alloc::Global>
                      ();
            puVar2 = local_100;
            lVar13 = local_100[7];
            lVar15 = 0;
            if ((lVar13 != 0) &&
               (lVar15 = lVar13 + (lVar13 * 8 + 0x17U & 0xfffffffffffffff0) + 0x11, lVar15 != 0)) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
            puVar2[10] = local_468;
            puVar2[0xb] = puStack_460;
            puVar2[8] = local_478;
            puVar2[9] = uStack_470;
            puVar2[6] = local_488;
            puVar2[7] = uStack_480;
            puVar2[4] = local_498;
            puVar2[5] = puStack_490;
            puVar2[2] = local_4a8;
            puVar2[3] = uStack_4a0;
            *puVar2 = CONCAT44(uStack_4b4,local_4b8);
            puVar2[1] = CONCAT44(uStack_4ac,uStack_4b0);
            std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
            if ((*extraout_RAX_36 & 1) == 0) {
              std::sys::random::hashmap_random_keys();
              extraout_RAX_36[0] = 1;
              extraout_RAX_36[1] = 0;
              extraout_RAX_36[2] = 0;
              extraout_RAX_36[3] = 0;
              extraout_RAX_36[4] = 0;
              extraout_RAX_36[5] = 0;
              extraout_RAX_36[6] = 0;
              extraout_RAX_36[7] = 0;
              *(longlong *)(extraout_RAX_36 + 8) = extraout_RAX_37;
              *(longlong *)(extraout_RAX_36 + 0x10) = lVar15;
              local_b0 = (undefined8 *)extraout_RAX_37;
              local_a8 = (uint *)lVar15;
            }
            else {
              local_b0 = *(undefined8 **)(extraout_RAX_36 + 8);
              local_a8 = (uint *)*(longlong *)(extraout_RAX_36 + 0x10);
            }
            *(longlong *)(extraout_RAX_36 + 8) = (longlong)local_b0 + 1;
            *local_168 = local_c4;
            hashbrown::raw::RawTableInner::
            drop_inner_table<tuple_<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::resources::stairs::visual::StairsEdgeVisualState>,alloc::alloc::Global>
                      ();
            plVar16 = local_258;
            local_d8[2] = 0;
            local_d8[3] = 0;
            *local_d8 = &DAT_1429c2e40;
            local_d8[1] = 0;
            local_d8[4] = local_b0;
            local_d8[5] = local_a8;
            *(undefined1 *)(local_d8 + 6) = 0;
            *local_230 = local_124;
            country_core::resources::stairs::structural_graph::StairsStructuralGraph::clear();
            *local_248 = local_128;
            system_decorator::ui_systems::ui_decorator_affordance_dispatch::
            DecoratorAffordanceDispatcher::clear();
            if (local_240 != (longlong *)0x0) {
              plVar16 = local_240;
            }
            lVar13 = plVar16[2];
            if ((ulonglong)(*plVar16 - lVar13) < 8) {
              alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
            }
            *(code **)(plVar16[1] + lVar13) =
                 core::ops::function::FnOnce::
                 call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::system::commands::command::remove_resource::closure_env_0<country_core::resources::stairs::stairs_top_down::StairsTopDownMesh>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
            ;
            plVar16[2] = lVar13 + 8;
            return;
          }
                    // WARNING: Subroutine does not return
          std::thread::local::panic_access_error();
        }
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
    }
  }
                    // WARNING: Subroutine does not return
  std::thread::local::panic_access_error();
}

