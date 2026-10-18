// sim_wren_boids 0x142326a80 sig=void __cdecl sim_wren_boids(void)

// WARNING: Removing unreachable block (ram,0x000142329d23)
// WARNING: Removing unreachable block (ram,0x000142329d40)
// WARNING: Removing unreachable block (ram,0x000142329d88)
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl system_critters::wren_boids::sim_wren_boids(void)

{
  undefined4 *puVar1;
  float *pfVar2;
  float *pfVar3;
  byte *pbVar4;
  int *piVar5;
  byte bVar6;
  byte bVar7;
  byte bVar8;
  undefined4 uVar9;
  undefined *puVar10;
  longlong lVar11;
  longlong lVar12;
  undefined8 uVar13;
  undefined1 auVar14 [16];
  undefined1 auVar15 [16];
  undefined1 auVar16 [16];
  undefined1 auVar17 [16];
  undefined1 auVar18 [16];
  undefined1 auVar19 [16];
  undefined1 auVar20 [16];
  undefined1 auVar21 [16];
  undefined1 auVar22 [16];
  undefined1 auVar23 [16];
  undefined1 auVar24 [16];
  undefined1 auVar25 [16];
  undefined1 auVar26 [16];
  undefined1 auVar27 [16];
  undefined1 auVar28 [16];
  undefined1 auVar29 [16];
  undefined1 auVar30 [16];
  bool bVar31;
  undefined **ppuVar32;
  undefined **ppuVar33;
  bool bVar34;
  char cVar35;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  uint uVar36;
  int iVar37;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  uint *extraout_RAX_02;
  longlong *extraout_RAX_03;
  uint *extraout_RAX_04;
  uint *extraout_RAX_05;
  byte *extraout_RAX_06;
  longlong extraout_RAX_07;
  longlong lVar38;
  longlong extraout_RAX_08;
  float *pfVar39;
  longlong extraout_RAX_09;
  ulonglong *puVar40;
  undefined1 (*extraout_RAX_10) [16];
  longlong *extraout_RAX_11;
  longlong *extraout_RAX_12;
  longlong *extraout_RAX_13;
  longlong *extraout_RAX_14;
  uint uVar41;
  ulonglong in_RCX;
  ulonglong uVar42;
  longlong lVar43;
  longlong lVar44;
  ulonglong uVar45;
  longlong lVar46;
  longlong *plVar47;
  longlong *in_RDX;
  ulonglong uVar48;
  undefined8 *puVar49;
  ulonglong uVar50;
  ulonglong uVar51;
  longlong *plVar52;
  uint *puVar53;
  float *pfVar54;
  longlong *in_R8;
  undefined *puVar55;
  ulonglong uVar56;
  ulonglong uVar57;
  undefined8 *in_R9;
  longlong lVar58;
  longlong lVar59;
  uint *puVar60;
  float *pfVar61;
  undefined **ppuVar62;
  undefined **ppuVar63;
  undefined1 uVar64;
  uint uVar65;
  float *pfVar66;
  uint *puVar67;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  float extraout_XMM0_Da_04;
  float extraout_XMM0_Da_05;
  float fVar68;
  float extraout_XMM0_Da_06;
  float extraout_XMM0_Da_07;
  float extraout_XMM0_Da_08;
  float extraout_XMM0_Da_09;
  float extraout_XMM0_Da_10;
  float extraout_XMM0_Da_11;
  float extraout_XMM0_Da_12;
  float extraout_XMM0_Da_13;
  float extraout_XMM0_Da_14;
  double dVar69;
  undefined1 auVar70 [16];
  float fVar71;
  undefined1 auVar72 [16];
  undefined1 auVar73 [16];
  undefined1 auVar74 [16];
  float fVar75;
  float fVar76;
  float fVar77;
  float fVar79;
  float fVar80;
  float fVar81;
  undefined1 auVar78 [16];
  float fVar82;
  undefined8 uVar83;
  undefined1 auVar84 [16];
  undefined1 auVar85 [16];
  undefined8 in_stack_00000028;
  longlong *in_stack_00000030;
  longlong *in_stack_00000038;
  ulonglong *in_stack_00000040;
  longlong *in_stack_00000048;
  longlong *in_stack_00000050;
  longlong *in_stack_00000058;
  longlong *in_stack_00000060;
  undefined8 local_418;
  float *pfStack_410;
  undefined8 local_408;
  longlong lStack_400;
  undefined8 local_3f8;
  ulonglong uStack_3f0;
  float local_3e8;
  float fStack_3e4;
  float fStack_3e0;
  longlong local_3d0;
  float *local_3c8;
  ulonglong local_3c0;
  longlong local_3a8;
  longlong local_3a0;
  ulonglong local_398;
  uint local_374;
  longlong local_350;
  int local_304;
  undefined8 local_2f8;
  undefined8 uStack_2f0;
  undefined8 uStack_2e8;
  longlong lStack_2e0;
  undefined1 local_2d8 [16];
  longlong local_2c8;
  undefined *puStack_2c0;
  longlong local_2b8;
  undefined8 uStack_2b0;
  undefined **local_2a8;
  undefined8 uStack_2a0;
  undefined8 local_298;
  float fStack_290;
  undefined4 uStack_28c;
  undefined4 uStack_288;
  undefined4 uStack_284;
  undefined8 local_280;
  ulonglong uStack_278;
  undefined8 local_270;
  undefined1 local_268;
  undefined **local_258;
  undefined8 uStack_250;
  undefined8 local_248;
  float fStack_240;
  undefined8 local_23c;
  undefined4 local_234;
  longlong local_220;
  ulonglong local_218;
  undefined **local_210;
  float *local_208;
  undefined **local_200;
  longlong lStack_1f8;
  undefined **local_1f0;
  undefined **local_1e8;
  float *pfStack_1e0;
  uint *local_1d8;
  longlong lStack_1d0;
  longlong local_1c8;
  ulonglong uStack_1c0;
  undefined8 local_1b8;
  undefined *puStack_1b0;
  undefined4 local_1a8;
  undefined4 uStack_1a4;
  undefined4 uStack_1a0;
  int iStack_19c;
  longlong *local_198;
  float *local_190;
  ulonglong local_188;
  ulonglong local_180;
  ulonglong local_178;
  ulonglong local_170;
  longlong local_168;
  uint *local_160;
  undefined **local_158;
  longlong local_150;
  undefined **local_148;
  longlong local_140;
  undefined1 local_131;
  float *local_130;
  ulonglong local_128;
  uint *local_120;
  longlong *local_118;
  uint local_10c;
  uint *local_108;
  longlong *local_100;
  undefined1 local_f3;
  undefined1 local_f2;
  undefined1 local_f1;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  plVar47 = in_RDX;
  local_218 = in_RCX;
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    bVar34 = false;
  }
  else {
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (plVar47 != (longlong *)0x0) {
        if (&DAT_0000002a < plVar47) {
          if (plVar47 != (longlong *)0x2b) goto LAB_142326b4c;
        }
        else if ((char)plVar47[0x2860981f] < -0x40) {
LAB_142326b4c:
                    // WARNING: Subroutine does not return
          core::str::slice_error_fail();
        }
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (plVar47 != (longlong *)0xfffffffffffffffe)) {
        if ((undefined1 *)((longlong)plVar47 + 2) < (undefined1 *)0x2b) {
          if (*(char *)((longlong)plVar47 + 0x14304c0fa) < -0x40) {
LAB_142326ba0:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if ((undefined1 *)((longlong)plVar47 + 2) != (undefined1 *)0x2b) goto LAB_142326ba0;
      }
    }
    puVar55 = &DAT_14304c2e1;
    do {
      bVar6 = puVar55[-1];
      uVar36 = (uint)(char)bVar6;
      if ((char)bVar6 < '\0') {
        bVar7 = puVar55[-2];
        if ((char)bVar7 < -0x40) {
          bVar8 = puVar55[-3];
          if ((char)bVar8 < -0x40) {
            pbVar4 = puVar55 + -4;
            puVar55 = puVar55 + -4;
            uVar36 = bVar8 & 0x3f | (*pbVar4 & 7) << 6;
          }
          else {
            puVar55 = puVar55 + -3;
            uVar36 = bVar8 & 0xf;
          }
          uVar36 = bVar7 & 0x3f | uVar36 << 6;
        }
        else {
          puVar55 = puVar55 + -2;
          uVar36 = bVar7 & 0x1f;
        }
        uVar36 = (uint)(bVar6 & 0x3f) | uVar36 << 6;
        if (uVar36 != 0x5c) goto LAB_142326c64;
LAB_142326c7b:
        if (puVar55 == &UNK_14304c2b7) break;
        if (puVar55 + -0x14304c2b7 < (undefined *)0x29) {
          if (-0x41 < (char)puVar55[1]) break;
        }
        else if (puVar55 + -0x14304c2b7 == (undefined *)0x29) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar55 = puVar55 + -1;
      if (uVar36 == 0x5c) goto LAB_142326c7b;
LAB_142326c64:
      if (uVar36 == 0x2f) goto LAB_142326c7b;
    } while (puVar55 != &DAT_14304c2b8);
    local_118 = in_R8;
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar47 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar47 = extraout_RAX_12, extraout_RAX_12 == (longlong *)0x0))
    goto LAB_142329ea1;
    if (*plVar47 != 0) goto LAB_142329ead;
    *plVar47 = -1;
    local_108 = (uint *)plVar47;
    puffin::ThreadProfiler::begin_scope();
    *(longlong *)local_108 = *(longlong *)local_108 + 1;
    bVar34 = true;
    in_R8 = local_118;
    local_108 = extraout_RAX_02;
  }
  *(undefined4 *)in_stack_00000030[2] = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
  plVar47 = (longlong *)*in_stack_00000030;
  if (*(char *)(*in_R8 + 5) == '\0') {
    plVar47[2] = 0;
    local_1d8 = (uint *)(plVar47 + 3);
    plVar52 = (longlong *)plVar47[4];
    lStack_1d0 = plVar47[5];
    plVar47[5] = 0;
    pfVar66 = (float *)(plVar52 + lStack_1d0 * 7);
    local_1c8 = 0;
    pfStack_1e0 = pfVar66;
    if (lStack_1d0 != 0) {
      do {
        if (((int)*plVar52 == 2) || (uVar51 = plVar52[2], uVar51 >> 0x20 == 0)) break;
        ppuVar62 = (undefined **)*in_stack_00000060;
        local_118 = plVar52 + 7;
        bevy_ecs::entity::Entities::contains();
        if (extraout_AL_00 != '\0') {
          lStack_2e0 = in_stack_00000060[3];
          uStack_2f0._0_4_ = (float)in_stack_00000060[1];
          uStack_2f0._4_4_ = (int)((ulonglong)in_stack_00000060[1] >> 0x20);
          uStack_2e8._0_4_ = (undefined4)in_stack_00000060[2];
          uStack_2e8._4_4_ = (undefined4)((ulonglong)in_stack_00000060[2] >> 0x20);
          local_2d8._0_8_ = uVar51;
          local_2f8 = ppuVar62;
          bevy_ecs::system::commands::EntityCommands::despawn();
        }
        pfVar54 = (float *)(plVar52 + 7);
        plVar52 = local_118;
      } while (pfVar54 != pfVar66);
    }
  }
  else {
    lVar38 = *in_RDX;
    if ((*(float *)(lVar38 + 0x28) != 0.0) || (NAN(*(float *)(lVar38 + 0x28)))) {
      local_350 = 0;
      local_198 = plVar47 + 6;
      local_118 = (longlong *)*in_R9;
      uVar51 = CONCAT71((int7)((ulonglong)in_R8 >> 8),1);
      local_220 = lVar38;
      if (SBORROW8(0,plVar47[6])) {
        local_108 = (uint *)(plVar47 + 3);
        uVar51 = 0;
        while( true ) {
          if (0x4f < uVar51) break;
          local_f1 = 1;
          fastrand::Rng::f32();
          fVar75 = extraout_XMM0_Da * 130.0 + -65.0;
          local_f1 = 1;
          fastrand::Rng::f32();
          local_f1 = 1;
          fastrand::Rng::f32();
          fVar76 = extraout_XMM0_Da_00 * 9.0 + 1.0;
          fVar79 = extraout_XMM0_Da_01 * 130.0 + -65.0;
          local_f1 = 1;
          ::utils::components::transform::impl$1::default();
          ppuVar62 = local_2f8;
          uVar13 = CONCAT44(uStack_2f0._4_4_,(float)uStack_2f0);
          local_f1 = 1;
          country_core::resources::audio_resources::AudioCore::instantiate_audio();
          if ((int)local_1e8 == 2) {
                    // WARNING: Subroutine does not return
            core::result::unwrap_failed();
          }
          local_3f8 = local_1c8;
          uStack_3f0 = uStack_1c0;
          local_408 = local_1d8;
          lStack_400 = lStack_1d0;
          local_418 = local_1e8;
          pfStack_410 = pfStack_1e0;
          country_core::resources::audio_resources::AudioInstance::stop_on_drop();
          uVar56 = *in_stack_00000040;
          uVar42 = uVar56 + 0xa0761d6478bd642f;
          *in_stack_00000040 = uVar42;
          auVar73._8_8_ = 0;
          auVar73._0_8_ = uVar42 ^ 0xe7037ed1a0b428db;
          auVar85._8_8_ = 0;
          auVar85._0_8_ = uVar42;
          uVar48 = SUB168(auVar73 * auVar85,8) ^ SUB168(auVar73 * auVar85,0);
          if ((uint)((int)uVar48 * 100) < 0x60) {
            uVar56 = uVar56 + 0x40ec3ac8f17ac85e;
            do {
              uVar42 = uVar42 + 0xa0761d6478bd642f;
              auVar14._8_8_ = 0;
              auVar14._0_8_ = uVar56 ^ 0xe7037ed1a0b428db;
              auVar21._8_8_ = 0;
              auVar21._0_8_ = uVar56;
              uVar48 = SUB168(auVar14 * auVar21,8) ^ SUB168(auVar14 * auVar21,0);
              uVar56 = uVar56 + 0xa0761d6478bd642f;
            } while ((uint)((int)uVar48 * 100) < 0x60);
            *in_stack_00000040 = uVar42;
          }
          iStack_19c = (int)((uVar48 & 0xffffffff) * 100 >> 0x20) + (int)uVar51;
          uStack_1a4 = 0;
          uStack_1a0 = 0;
          local_1b8 = 2;
          bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
          uStack_278 = uStack_278 & 0xffffffff00000000;
          local_270 = 0x6df6147cdf0aeade;
          local_298 = CONCAT44(fVar76,fVar75);
          local_248 = CONCAT44(fVar76,fVar75);
          local_2f8 = local_1e8;
          uStack_2f0._0_4_ = SUB84(pfStack_1e0,0);
          uStack_2f0._4_4_ = (int)((ulonglong)pfStack_1e0 >> 0x20);
          uStack_2e8._0_4_ = SUB84(local_1d8,0);
          uStack_2e8._4_4_ = (undefined4)((ulonglong)local_1d8 >> 0x20);
          local_2c8 = local_1b8;
          puStack_2c0 = puStack_1b0;
          local_2b8 = CONCAT44(uStack_1a4,local_1a8);
          uStack_2b0 = CONCAT44(iStack_19c,uStack_1a0);
          local_2a8 = ppuVar62;
          uStack_28c = 0x3f800000;
          uStack_288 = 0x3f800000;
          uStack_284 = 0x3f800000;
          local_268 = 0x11;
          local_258 = ppuVar62;
          local_23c = 0x3f8000003f800000;
          local_234 = 0x3f800000;
          local_f1 = 1;
          uStack_2a0 = uVar13;
          fStack_290 = fVar79;
          uStack_250 = uVar13;
          fStack_240 = fVar79;
          bevy_ecs::system::commands::Commands::
          spawn<tuple_<country_core::resources::critters::Wren,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::WrenInstanceData>,country_core::components::drawable::PrevTransform>_>
                    ();
          lVar46 = local_1c8;
          local_f1 = 1;
          fastrand::Rng::f32();
          local_f1 = 1;
          fastrand::Rng::f32();
          local_f1 = 1;
          fastrand::Rng::f32();
          lVar38 = plVar47[2];
          if (lVar38 == *plVar47) {
            local_f1 = 1;
            alloc::raw_vec::RawVec::grow_one<system_critters::boids::Boid,alloc::alloc::Global>();
          }
          lVar44 = plVar47[1];
          *(float *)(lVar44 + lVar38 * 0x18) = fVar75;
          pfVar66 = (float *)(lVar44 + 4 + lVar38 * 0x18);
          *pfVar66 = fVar76;
          pfVar66[1] = fVar79;
          *(float *)(lVar44 + 0xc + lVar38 * 0x18) = extraout_XMM0_Da_02 + -0.5;
          *(ulonglong *)(lVar44 + 0x10 + lVar38 * 0x18) =
               CONCAT44(extraout_XMM0_Da_04 + -0.5,extraout_XMM0_Da_03 + -0.5);
          plVar47[2] = lVar38 + 1;
          lVar38 = plVar47[5];
          if (lVar38 == plVar47[3]) {
            local_f1 = 1;
            alloc::raw_vec::RawVec::grow_one<enum2_<iced_core::event::Event>,alloc::alloc::Global>()
            ;
          }
          lVar44 = plVar47[4];
          lVar43 = lVar38 * 0x38;
          *(undefined4 *)(lVar44 + lVar43) = 0;
          *(longlong *)(lVar44 + 0x10 + lVar43) = lVar46;
          *(undefined8 *)(lVar44 + 0x18 + lVar43) = 0;
          *(undefined4 *)(lVar44 + 0x20 + lVar43) = 0x3f800000;
          *(undefined8 *)(lVar44 + 0x24 + lVar43) = 0;
          *(undefined4 *)(lVar44 + 0x2c + lVar43) = 0x3f800000;
          *(undefined4 *)(lVar44 + 0x30 + lVar43) = 0;
          plVar47[5] = lVar38 + 1;
          if (local_350 == 0) {
            local_f1 = 1;
            alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
          }
          uVar51 = uVar51 + 1;
          *(longlong *)(local_350 * 8 + 8) = lVar46;
          local_350 = local_350 + 1;
        }
        uStack_2e8._0_4_ = (undefined4)local_350;
        uStack_2e8._4_4_ = (undefined4)((ulonglong)local_350 >> 0x20);
        local_2f8 = (undefined **)0x0;
        uStack_2f0._0_4_ = 1.12104e-44;
        uStack_2f0._4_4_ = 0;
        if ((!SBORROW8(0,*local_198)) && (*local_198 != 0)) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        local_198[2] = CONCAT44(uStack_2e8._4_4_,(undefined4)uStack_2e8);
        *(undefined4 *)local_198 = (undefined4)local_2f8;
        *(int *)((longlong)local_198 + 4) = local_2f8._4_4_;
        *(float *)(local_198 + 1) = (float)uStack_2f0;
        *(int *)((longlong)local_198 + 0xc) = uStack_2f0._4_4_;
        uVar51 = 0;
      }
      local_210 = (undefined **)*in_stack_00000058;
      iVar37 = *(int *)((longlong)in_stack_00000058 + 0x14);
      puVar53 = *(uint **)((longlong)local_210 + 0xf8);
      uVar41 = iVar37 - (int)in_stack_00000058[2];
      uVar36 = 0xc233b000;
      if (uVar41 < 0xc233b000) {
        uVar36 = uVar41;
      }
      local_148 = (undefined **)0x8;
      local_140 = 0;
      local_128 = 0;
      local_170 = 0;
      local_10c = (uint)uVar51;
      local_158 = (undefined **)CONCAT44(local_158._4_4_,iVar37);
      local_190 = (float *)CONCAT71(local_190._1_7_,*(char *)((longlong)local_210 + 0x108));
      local_168 = CONCAT44(local_168._4_4_,uVar36);
      lVar38 = in_stack_00000058[1];
      puVar60 = puVar53 + *(longlong *)((longlong)local_210 + 0x100);
      uVar56 = 0;
      puVar67 = (uint *)0x0;
      ppuVar62 = local_148;
      uVar42 = local_170;
      cVar35 = *(char *)((longlong)local_210 + 0x108);
joined_r0x00014232748c:
      uVar64 = (undefined1)uVar51;
      local_170 = uVar42;
      local_150 = lVar38;
      local_148 = ppuVar62;
      local_120 = puVar60;
      if (cVar35 == '\0') {
        if (uVar56 != uVar42) goto LAB_14232762c;
        while (puVar53 != puVar60) {
          uVar36 = *puVar53;
          puVar53 = puVar53 + 1;
          if (*(longlong *)(*(longlong *)(lVar38 + 0x100) + 0x58 + (ulonglong)uVar36 * 0xa0) == 0) {
            do {
              if (puVar53 == puVar60) goto LAB_142327db0;
              uVar36 = *puVar53;
              puVar53 = puVar53 + 1;
            } while (*(longlong *)(*(longlong *)(lVar38 + 0x100) + 0x58 + (ulonglong)uVar36 * 0xa0)
                     == 0);
            lVar46 = *(longlong *)(lVar38 + 0x100) + (ulonglong)uVar36 * 0xa0;
          }
          else {
            lVar46 = *(longlong *)(lVar38 + 0x100) + (ulonglong)uVar36 * 0xa0;
          }
          lVar44 = *(longlong *)(lVar38 + 0x1a8) + (ulonglong)*(uint *)(lVar46 + 0x94) * 0x48;
          uVar51 = *(ulonglong *)
                    (*(longlong *)(lVar38 + 0x1a8) + 0x40 +
                    (ulonglong)*(uint *)(lVar46 + 0x94) * 0x48);
          if ((*(ulonglong *)((longlong)local_210 + 0x110) < uVar51) &&
             (uVar42 = *(ulonglong *)
                        (*(longlong *)(lVar44 + 0x38) +
                        *(ulonglong *)((longlong)local_210 + 0x110) * 8), uVar42 != 0)) {
            local_140 = *(undefined8 *)(*(longlong *)(lVar44 + 0x18) + 0x10 + ~uVar42 * 0x30);
            uVar42 = *(ulonglong *)((longlong)local_210 + 0x118);
            if (uVar51 <= uVar42) goto LAB_1423275e0;
LAB_1423275a7:
            uVar51 = *(ulonglong *)(*(longlong *)(lVar44 + 0x38) + uVar42 * 8);
            if (uVar51 == 0) goto LAB_1423275e0;
            local_128 = *(longlong *)(*(longlong *)(lVar44 + 0x18) + 0x10 + ~uVar51 * 0x30);
          }
          else {
            local_140 = 0;
            uVar42 = *(ulonglong *)((longlong)local_210 + 0x118);
            if (uVar42 < uVar51) goto LAB_1423275a7;
LAB_1423275e0:
            local_128 = 0;
          }
          local_f1 = uVar64;
          bevy_ecs::storage::table::Table::get_added_ticks_slice_for();
          ppuVar62 = *(undefined ***)(lVar46 + 0x50);
          local_170 = *(ulonglong *)(lVar46 + 0x58);
          uVar56 = 0;
          lVar38 = local_150;
          puVar60 = local_120;
          puVar67 = extraout_RAX_04;
          iVar37 = (int)local_158;
          uVar36 = (uint)local_168;
LAB_14232762c:
          do {
            uVar51 = (ulonglong)*(uint *)((longlong)ppuVar62 + 8 + uVar56 * 0x10);
            uVar42 = local_170;
            if (iVar37 - puVar67[uVar51] < uVar36) goto LAB_142327800;
            uVar56 = uVar56 + 1;
          } while (uVar56 != local_170);
        }
      }
      else {
        if (uVar56 != uVar42) goto LAB_1423277d1;
        while (puVar53 != puVar60) {
          uVar51 = (ulonglong)*puVar53;
          puVar53 = puVar53 + 1;
          lVar46 = *(longlong *)(lVar38 + 0x1a8);
          if (*(longlong *)(lVar46 + 0x10 + uVar51 * 0x48) == 0) {
            do {
              if (puVar53 == puVar60) goto LAB_142327db0;
              uVar36 = *puVar53;
              puVar53 = puVar53 + 1;
            } while (*(longlong *)(*(longlong *)(lVar38 + 0x1a8) + 0x10 + (ulonglong)uVar36 * 0x48)
                     == 0);
            lVar38 = *(longlong *)(lVar38 + 0x1a8) + (ulonglong)uVar36 * 0x48;
            lVar46 = lVar38;
          }
          else {
            lVar38 = lVar46 + uVar51 * 0x48;
            lVar46 = lVar46 + uVar51 * 0x48;
          }
          uVar51 = *(ulonglong *)(lVar38 + 0x40);
          if ((*(ulonglong *)((longlong)local_210 + 0x110) < uVar51) &&
             (uVar42 = *(ulonglong *)
                        (*(longlong *)(lVar38 + 0x38) +
                        *(ulonglong *)((longlong)local_210 + 0x110) * 8), uVar42 != 0)) {
            local_140 = *(undefined8 *)(*(longlong *)(lVar38 + 0x18) + 0x10 + ~uVar42 * 0x30);
            uVar42 = *(ulonglong *)((longlong)local_210 + 0x118);
            if (uVar51 <= uVar42) goto LAB_142327790;
LAB_142327757:
            uVar51 = *(ulonglong *)(*(longlong *)(lVar38 + 0x38) + uVar42 * 8);
            if (uVar51 == 0) goto LAB_142327790;
            local_128 = *(longlong *)(*(longlong *)(lVar38 + 0x18) + 0x10 + ~uVar51 * 0x30);
          }
          else {
            local_140 = 0;
            uVar42 = *(ulonglong *)((longlong)local_210 + 0x118);
            if (uVar42 < uVar51) goto LAB_142327757;
LAB_142327790:
            local_128 = 0;
          }
          local_f1 = uVar64;
          bevy_ecs::storage::table::Table::get_added_ticks_slice_for();
          uVar42 = *(ulonglong *)(lVar46 + 0x10);
          uVar56 = 0;
          lVar38 = local_150;
          puVar60 = local_120;
          puVar67 = extraout_RAX_05;
          iVar37 = (int)local_158;
          uVar36 = (uint)local_168;
LAB_1423277d1:
          do {
            uVar51 = uVar56 & 0xffffffff;
            ppuVar62 = local_148;
            if (iVar37 - puVar67[uVar51] < uVar36) goto LAB_142327800;
            uVar56 = uVar56 + 1;
          } while (uVar56 != uVar42);
        }
      }
LAB_142327db0:
      puStack_2c0 = (undefined *)*in_stack_00000048;
      local_2d8._0_8_ = in_stack_00000048[1];
      local_2b8 = *(longlong *)((longlong)puStack_2c0 + 0xf8);
      uStack_2b0 = local_2b8 + *(longlong *)((longlong)puStack_2c0 + 0x100) * 4;
      local_280 = 0;
      uStack_278 = 0;
      fStack_290 = 0.0;
      uStack_28c = 0;
      uStack_288 = 0;
      uStack_284 = 0;
      local_2c8 = local_2d8._0_8_ + 0xf8;
      local_2f8 = (undefined **)0x0;
      uStack_2e8._0_4_ = 0;
      uStack_2e8._4_4_ = 0;
      local_2d8._8_8_ = local_2d8._0_8_ + 0x1a0;
      local_2a8 = (undefined **)0x8;
      uStack_2a0 = 0;
      local_298 = 8;
      local_270 = CONCAT71(local_270._1_7_,*(undefined1 *)((longlong)puStack_2c0 + 0x108));
      puVar49 = &local_2f8;
      local_f1 = uVar64;
      alloc::vec::spec_from_iter::impl$0::
      from_iter<system_critters::wren_boids::sim_wren_boids::FreeSpot,core::iter::adapters::flatten::FlatMap<bevy_ecs::query::iter::QueryIter<ref_<country_core::resources::critters::BirdSpots>,tuple_<>_>,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::resources::critters::BirdSpot>,system_critters::wren_boids::sim_wren_boids::closure_1::closure_env_0>,system_critters::wren_boids::sim_wren_boids::closure_env_1>_>
                ();
      std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
      if ((*extraout_RAX_06 & 1) == 0) {
        std::sys::random::hashmap_random_keys();
        extraout_RAX_06[0] = 1;
        extraout_RAX_06[1] = 0;
        extraout_RAX_06[2] = 0;
        extraout_RAX_06[3] = 0;
        extraout_RAX_06[4] = 0;
        extraout_RAX_06[5] = 0;
        extraout_RAX_06[6] = 0;
        extraout_RAX_06[7] = 0;
        *(longlong *)(extraout_RAX_06 + 8) = extraout_RAX_07;
        *(undefined8 **)(extraout_RAX_06 + 0x10) = puVar49;
        lVar38 = extraout_RAX_07;
      }
      else {
        lVar38 = *(longlong *)(extraout_RAX_06 + 8);
      }
      *(longlong *)(extraout_RAX_06 + 8) = lVar38 + 1;
      lStack_2e0 = *in_stack_00000050;
      local_2f8 = (undefined **)in_stack_00000050[1];
      local_108 = (uint *)CONCAT44(local_108._4_4_,
                                   *(undefined4 *)((longlong)in_stack_00000050 + 0x14));
      local_210 = local_2f8 + 0x34;
      local_148 = local_2f8 + 0x1f;
      uStack_2a0 = 0;
      local_298 = 0;
      uStack_2b0 = 0;
      local_2a8 = (undefined **)0x0;
      fStack_290 = 0.0;
      uStack_28c = 0;
      local_2d8._8_8_ = *(longlong *)(lStack_2e0 + 0xf8) + *(longlong *)(lStack_2e0 + 0x100) * 4;
      local_2d8._0_8_ = *(longlong *)(lStack_2e0 + 0xf8);
      local_2c8 = 8;
      puStack_2c0 = (undefined *)0x0;
      local_2b8 = 8;
      uStack_288 = CONCAT31(uStack_288._1_3_,*(undefined1 *)(lStack_2e0 + 0x108));
      local_158 = local_2f8;
      local_150 = lStack_2e0;
      uStack_2f0 = local_210;
      uStack_2e8 = local_148;
      itertools::Itertools::
      multiunzip<core::iter::adapters::filter_map::FilterMap<bevy_ecs::query::iter::QueryIter<tuple_<bevy_ecs::entity::Entity,ref_<country_core::resources::tree_storage::BirdHaven>,ref_<country_core::systems::prefab::PrefabInstanceData>_>,bevy_ecs::query::filter::Without<country_core::components::TreeMarkedForDeletion>_>,system_critters::wren_boids::sim_wren_boids::closure_env_2>,tuple_<alloc::vec::Vec<utils::geometry::intersect::Sphere,alloc::alloc::Global>,alloc::vec::Vec<bevy_ecs::entity::Entity,alloc::alloc::Global>_>_>
                ();
      lVar46 = local_1c8;
      lVar38 = lStack_1d0;
      pfVar66 = pfStack_1e0;
      ppuVar62 = local_1e8;
      local_120 = local_1d8;
      local_160 = (uint *)uStack_1c0;
      local_200 = (undefined **)0x0;
      lStack_1f8 = 8;
      local_1f0 = (undefined **)0x0;
      uVar51 = plVar47[2];
      local_131 = uVar51 < 0x50;
      plVar52 = (longlong *)plVar47[1];
      local_170 = uVar51 * 0x18;
      if (uVar51 == 0) {
        local_140 = 4;
        local_130 = (float *)0x0;
        goto LAB_14232810b;
      }
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_08 == 0) {
        local_f3 = 1;
                    // WARNING: Subroutine does not return
        alloc::raw_vec::handle_error();
      }
      lVar44 = 0;
      uVar42 = uVar51;
      goto LAB_1423280a0;
    }
  }
  if (extraout_AL == '\0') {
    return;
  }
  puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
  if (*extraout_RAX_03 == 1) {
    plVar47 = extraout_RAX_03 + 1;
  }
  else if ((*extraout_RAX_03 != 0) ||
          (std::sys::thread_local::native::lazy::Storage::
           initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                     (), plVar47 = extraout_RAX_13, extraout_RAX_13 == (longlong *)0x0)) {
LAB_142329ea1:
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
  if (*plVar47 != 0) {
LAB_142329ead:
                    // WARNING: Subroutine does not return
    core::cell::panic_already_borrowed();
  }
  *plVar47 = -1;
  local_118 = plVar47;
  puffin::ThreadProfiler::end_scope();
  goto LAB_142329de9;
LAB_142327800:
  local_148 = ppuVar62;
  local_160 = puVar67;
  local_170 = uVar42;
  local_100 = (longlong *)(local_128 + uVar51 * 0x28);
  local_f1 = uVar64;
  country_core::resources::placement_data::PlacementDataSsbo::to_affine3a();
  local_178 = uVar56 + 1;
  local_130 = (float *)CONCAT44(local_130._4_4_,*(undefined4 *)((longlong)local_100 + 0x10));
  uVar36 = 0;
  while( true ) {
    uVar51 = (ulonglong)local_10c;
    if ((uint)local_130 <= uVar36) break;
    local_108 = (uint *)CONCAT44(local_108._4_4_,uVar36);
    fVar75 = *(float *)((longlong)local_100 + 0x14);
    fVar76 = *(float *)((longlong)local_100 + 0x18);
    fVar79 = *(float *)((longlong)local_100 + 0x1c);
    fVar81 = *(float *)((longlong)local_100 + 0x20);
    uVar64 = (undefined1)local_10c;
    local_f1 = uVar64;
    fastrand::Rng::f32();
    local_f1 = uVar64;
    fastrand::Rng::f32();
    local_f1 = uVar64;
    ::utils::uniform_sample_sphere();
    fVar71 = (float)uStack_2f0;
    fVar82 = SUB84(local_2f8,0);
    uVar51 = (ulonglong)local_2f8 >> 0x20;
    local_f1 = uVar64;
    fastrand::Rng::f32();
    local_f1 = uVar64;
    ::utils::components::transform::impl$1::default();
    ppuVar62 = local_2f8;
    uVar13 = CONCAT44(uStack_2f0._4_4_,(float)uStack_2f0);
    local_f1 = uVar64;
    country_core::resources::audio_resources::AudioCore::instantiate_audio();
    if ((int)local_1e8 == 2) {
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    country_core::resources::audio_resources::AudioInstance::start_if_not_playing();
    country_core::resources::audio_resources::AudioInstance::stop_on_drop();
    uVar56 = *in_stack_00000040;
    uVar42 = uVar56 + 0xa0761d6478bd642f;
    *in_stack_00000040 = uVar42;
    auVar15._8_8_ = 0;
    auVar15._0_8_ = uVar42 ^ 0xe7037ed1a0b428db;
    auVar22._8_8_ = 0;
    auVar22._0_8_ = uVar42;
    uVar48 = SUB168(auVar15 * auVar22,8) ^ SUB168(auVar15 * auVar22,0);
    if ((uint)((int)uVar48 * 100) < 0x60) {
      uVar56 = uVar56 + 0x40ec3ac8f17ac85e;
      do {
        uVar42 = uVar42 + 0xa0761d6478bd642f;
        auVar16._8_8_ = 0;
        auVar16._0_8_ = uVar56 ^ 0xe7037ed1a0b428db;
        auVar23._8_8_ = 0;
        auVar23._0_8_ = uVar56;
        uVar48 = SUB168(auVar16 * auVar23,8) ^ SUB168(auVar16 * auVar23,0);
        uVar56 = uVar56 + 0xa0761d6478bd642f;
      } while ((uint)((int)uVar48 * 100) < 0x60);
      *in_stack_00000040 = uVar42;
    }
    iStack_19c = (int)((uVar48 & 0xffffffff) * 100 >> 0x20) + (int)local_108._0_4_;
    uStack_1a4 = 0;
    uStack_1a0 = 0;
    local_1b8 = 2;
    bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
    fVar68 = fVar81 * 0.0;
    fVar77 = (float)local_3f8 * fVar68 + (float)local_408 * fVar68 + (float)local_418 * fVar81;
    fVar80 = local_3f8._4_4_ * fVar68 + local_408._4_4_ * fVar68 + local_418._4_4_ * fVar81;
    fVar81 = (float)uStack_3f0 * fVar68 + (float)lStack_400 * fVar68 + pfStack_410._0_4_ * fVar81;
    fVar81 = SQRT(fVar81 * fVar81 + fVar80 * fVar80 + fVar77 * fVar77);
    fVar68 = SQRT(extraout_XMM0_Da_05);
    fVar82 = fVar68 * fVar81 * fVar82 +
             fVar79 * (float)local_3f8 + fVar76 * (float)local_408 + fVar75 * (float)local_418 +
             local_3e8;
    fVar77 = fVar68 * fVar81 * (float)uVar51 +
             fVar79 * local_3f8._4_4_ + fVar76 * local_408._4_4_ + fVar75 * local_418._4_4_ +
             fStack_3e4;
    fVar75 = fVar79 * (float)uStack_3f0 + fVar76 * (float)lStack_400 + fVar75 * pfStack_410._0_4_ +
             fStack_3e0 + fVar71 * fVar81 * fVar68;
    uStack_278 = uStack_278 & 0xffffffff00000000;
    local_270 = 0x6df6147cdf0aeade;
    local_298 = CONCAT44(fVar77,fVar82);
    local_248 = CONCAT44(fVar77,fVar82);
    local_2f8 = local_1e8;
    uStack_2f0._0_4_ = SUB84(pfStack_1e0,0);
    uStack_2f0._4_4_ = (int)((ulonglong)pfStack_1e0 >> 0x20);
    uStack_2e8._0_4_ = SUB84(local_1d8,0);
    uStack_2e8._4_4_ = (undefined4)((ulonglong)local_1d8 >> 0x20);
    local_2c8 = local_1b8;
    puStack_2c0 = puStack_1b0;
    local_2b8 = CONCAT44(uStack_1a4,local_1a8);
    uStack_2b0 = CONCAT44(iStack_19c,uStack_1a0);
    local_2a8 = ppuVar62;
    uStack_28c = 0x3f800000;
    uStack_288 = 0x3f800000;
    uStack_284 = 0x3f800000;
    local_268 = 0x11;
    local_258 = ppuVar62;
    local_23c = 0x3f8000003f800000;
    local_234 = 0x3f800000;
    uVar64 = (undefined1)local_10c;
    uStack_2a0 = uVar13;
    fStack_290 = fVar75;
    uStack_250 = uVar13;
    fStack_240 = fVar75;
    local_f1 = uVar64;
    bevy_ecs::system::commands::Commands::
    spawn<tuple_<country_core::resources::critters::Wren,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::WrenInstanceData>,country_core::components::drawable::PrevTransform>_>
              ();
    lVar46 = local_1c8;
    local_f1 = uVar64;
    fastrand::Rng::f32();
    local_f1 = uVar64;
    fastrand::Rng::f32();
    local_f1 = uVar64;
    fastrand::Rng::f32();
    lVar38 = plVar47[2];
    if (lVar38 == *plVar47) {
      local_f1 = uVar64;
      alloc::raw_vec::RawVec::grow_one<system_critters::boids::Boid,alloc::alloc::Global>();
    }
    lVar44 = plVar47[1];
    *(ulonglong *)(lVar44 + lVar38 * 0x18) = CONCAT44(fVar77,fVar82);
    *(float *)(lVar44 + 8 + lVar38 * 0x18) = fVar75;
    *(float *)(lVar44 + 0xc + lVar38 * 0x18) = extraout_XMM0_Da_06 + -0.5;
    pfVar66 = (float *)(lVar44 + 0x10 + lVar38 * 0x18);
    *pfVar66 = extraout_XMM0_Da_07 * 0.5 + 0.2;
    pfVar66[1] = extraout_XMM0_Da_08 + -0.5;
    plVar47[2] = lVar38 + 1;
    lVar38 = plVar47[5];
    if (lVar38 == plVar47[3]) {
      local_f1 = uVar64;
      alloc::raw_vec::RawVec::grow_one<enum2_<iced_core::event::Event>,alloc::alloc::Global>();
    }
    lVar44 = plVar47[4];
    lVar43 = lVar38 * 0x38;
    *(undefined4 *)(lVar44 + lVar43) = 0;
    *(longlong *)(lVar44 + 0x10 + lVar43) = lVar46;
    *(undefined8 *)(lVar44 + 0x18 + lVar43) = 0;
    *(undefined4 *)(lVar44 + 0x20 + lVar43) = 0x3f800000;
    *(undefined8 *)(lVar44 + 0x24 + lVar43) = 0;
    *(undefined4 *)(lVar44 + 0x2c + lVar43) = 0x3f800000;
    *(undefined4 *)(lVar44 + 0x30 + lVar43) = 0;
    plVar47[5] = lVar38 + 1;
    local_f1 = (undefined1)local_10c;
    if (SBORROW8(0,plVar47[6])) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    lVar38 = plVar47[8];
    if (lVar38 == plVar47[6]) {
      alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
    }
    uVar36 = (int)local_108._0_4_ + 1;
    *(longlong *)(plVar47[7] + lVar38 * 8) = lVar46;
    plVar47[8] = lVar38 + 1;
  }
  lVar38 = local_150;
  iVar37 = (int)local_158;
  puVar60 = local_120;
  uVar56 = local_178;
  puVar67 = local_160;
  uVar36 = (uint)local_168;
  ppuVar62 = local_148;
  uVar42 = local_170;
  cVar35 = (char)local_190;
  goto joined_r0x00014232748c;
  while( true ) {
    puVar1 = (undefined4 *)((longlong)plVar52 + 0x14);
    uStack_2e8._0_4_ = (undefined4)((ulonglong)*(undefined8 *)((longlong)plVar52 + 0xc) >> 0x20);
    uStack_2f0 = (undefined **)plVar52[1];
    lVar43 = plVar52[1];
    local_2f8 = (undefined **)*plVar52;
    plVar52 = plVar52 + 3;
    uStack_2e8 = (undefined **)CONCAT44(*puVar1,(undefined4)uStack_2e8);
    *(undefined ***)(extraout_RAX_08 + 0x10 + lVar44) = uStack_2e8;
    *(longlong *)(extraout_RAX_08 + lVar44) = (longlong)local_2f8;
    ((longlong *)(extraout_RAX_08 + lVar44))[1] = lVar43;
    lVar44 = lVar44 + 0x18;
    uVar42 = uVar42 - 1;
    if (uVar42 == 0) break;
LAB_1423280a0:
    if (local_170 == lVar44) break;
  }
  plVar52 = (longlong *)plVar47[1];
  local_130 = (float *)plVar47[2];
  local_140 = extraout_RAX_08;
LAB_14232810b:
  local_128 = plVar47[4];
  if ((float *)plVar47[5] < local_130) {
    local_130 = (float *)plVar47[5];
  }
  local_168 = *in_stack_00000038;
  local_178 = in_stack_00000038[1];
  local_208 = local_3c8;
  local_190 = pfVar66 + (longlong)local_120 * 4;
  pfVar54 = (float *)0x0;
  pfVar61 = local_130;
  uVar42 = 0;
  uVar56 = local_180;
  local_100 = plVar52;
  ppuVar32 = uStack_2f0;
  ppuVar33 = uStack_2e8;
LAB_1423281a8:
  local_180 = uVar56;
  uVar48 = uVar42;
  ppuVar63 = local_1f0;
  if (pfVar54 < pfVar61) {
    pfVar2 = (float *)((longlong)pfVar54 + 1);
    uVar42 = uVar48 + 1;
    if ((SQRT(*(float *)(plVar52 + (longlong)pfVar54 * 3 + 1) *
              *(float *)(plVar52 + (longlong)pfVar54 * 3 + 1) +
              *(float *)(plVar52 + (longlong)pfVar54 * 3) *
              *(float *)(plVar52 + (longlong)pfVar54 * 3)) <= 91.92388) &&
       (pfVar3 = (float *)(plVar52 + (longlong)pfVar54 * 3), pfVar3[1] <= 30.0)) goto LAB_142328250;
    uStack_2f0 = ppuVar32;
    uStack_2e8 = ppuVar33;
    if (local_1f0 == local_200) {
      local_f2 = 1;
      alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
      plVar52 = local_100;
      pfVar61 = local_130;
    }
    goto LAB_142328190;
  }
  uStack_2a0 = 0;
  local_298 = 0;
  uStack_2b0 = 0;
  local_2a8 = (undefined **)0x0;
  fStack_290 = 0.0;
  uStack_28c = 0;
  local_2f8 = local_158;
  uStack_2f0._0_4_ = SUB84(local_210,0);
  uStack_2f0._4_4_ = (int)((ulonglong)local_210 >> 0x20);
  uStack_2e8._0_4_ = SUB84(local_148,0);
  uStack_2e8._4_4_ = (undefined4)((ulonglong)local_148 >> 0x20);
  lStack_2e0 = local_150;
  local_2d8._8_8_ = *(longlong *)(local_150 + 0xf8) + *(longlong *)(local_150 + 0x100) * 4;
  local_2d8._0_8_ = *(longlong *)(local_150 + 0xf8);
  local_2c8 = 8;
  puStack_2c0 = (undefined *)0x0;
  local_2b8 = 8;
  uStack_288 = CONCAT31(uStack_288._1_3_,*(undefined1 *)(local_150 + 0x108));
  local_f2 = 1;
  core::iter::adapters::map::impl$2::
  fold<usize,bevy_ecs::query::iter::QueryIter<tuple_<bevy_ecs::entity::Entity,ref_<country_core::resources::tree_storage::BirdHaven>,ref_<country_core::systems::prefab::PrefabInstanceData>_>,bevy_ecs::query::filter::Without<country_core::components::TreeMarkedForDeletion>_>,system_critters::wren_boids::sim_wren_boids::closure_env_4,usize,core::iter::traits::accum::impl_48::sum::closure_env_0<core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple_<bevy_ecs::entity::Entity,ref_<country_core::resources::tree_storage::BirdHaven>,ref_<country_core::systems::prefab::PrefabInstanceData>_>,bevy_ecs::query::filter::Without<country_core::components::TreeMarkedForDeletion>_>,system_critters::wren_boids::sim_wren_boids::closure_env_4>_>_>
            ();
  local_180 = extraout_RAX_09 + plVar47[2];
  puVar53 = *(uint **)(local_150 + 0xf8);
  local_160 = puVar53 + *(longlong *)(local_150 + 0x100);
  local_210 = (undefined **)CONCAT71(local_210._1_7_,*(char *)(local_150 + 0x108));
  local_3a8 = 8;
  local_190 = (float *)0x0;
  local_3a0 = 8;
  local_148 = (undefined **)0x0;
  uVar42 = 0;
  uVar56 = 0;
  local_398 = 0;
  local_304 = 0;
  _DEBUG_HAVEN_BIRD_COUNT = extraout_RAX_09;
  if (*(char *)(local_150 + 0x108) != '\0') goto LAB_142328a80;
LAB_142328967:
  if (uVar56 == uVar42) {
    if (puVar53 == local_160) {
LAB_14232950f:
      uStack_2f0._4_4_ = (int)local_398;
      if ((int)local_398 != 0) {
        puVar55 = (undefined *)(ulonglong)local_374;
        local_2f8 = (undefined **)0x1;
        if ((local_158[2] <= puVar55) ||
           (puVar10 = local_158[1], *(int *)(puVar10 + (longlong)puVar55 * 0x14) != (int)local_398))
        {
LAB_142329fc1:
          uStack_2f0._0_4_ = (float)local_374;
          local_f2 = 1;
                    // WARNING: Subroutine does not return
          core::result::unwrap_failed();
        }
        uVar42 = (ulonglong)*(uint *)(puVar10 + (longlong)puVar55 * 0x14 + 4);
        if (uVar42 == 0xffffffff) goto LAB_142329fc1;
        if ((*(ulonglong *)(local_150 + 0x38) <= uVar42) ||
           ((*(ulonglong *)
              (*(longlong *)(local_150 + 0x28) +
              (ulonglong)(*(uint *)(puVar10 + (longlong)puVar55 * 0x14 + 4) >> 6) * 8) >>
             (uVar42 & 0x3f) & 1) == 0)) {
          local_2f8 = (undefined **)(uVar42 << 0x20);
          goto LAB_142329fc1;
        }
        if (local_180 != 1000) {
          uVar36 = *(uint *)(puVar10 + (longlong)puVar55 * 0x14 + 0x10);
          lVar46 = *(longlong *)
                    (local_158[0x35] +
                    (ulonglong)*(uint *)(puVar10 + (longlong)puVar55 * 0x14 + 0xc) * 0x48 + 0x18);
          lVar43 = ~*(ulonglong *)
                     (*(longlong *)
                       (local_158[0x35] +
                       (ulonglong)*(uint *)(puVar10 + (longlong)puVar55 * 0x14 + 0xc) * 0x48 + 0x38)
                     + *(longlong *)(local_150 + 0x110) * 8) * 0x30;
          lVar44 = *(longlong *)(lVar46 + 0x10 + lVar43);
          *(float *)(*(longlong *)(lVar46 + 0x28 + lVar43) + (ulonglong)uVar36 * 4) =
               local_108._0_4_;
          piVar5 = (int *)(lVar44 + 0x10 + (ulonglong)uVar36 * 0x28);
          *piVar5 = *piVar5 + ((local_180 < 1000) - 1 | 1);
        }
      }
      uStack_2e8._0_4_ = SUB84(local_1f0,0);
      uStack_2e8._4_4_ = (undefined4)((ulonglong)local_1f0 >> 0x20);
      local_2f8 = local_200;
      uStack_2f0._0_4_ = (float)lStack_1f8;
      uStack_2f0._4_4_ = (int)((ulonglong)lStack_1f8 >> 0x20);
      local_f2 = 0;
      boids::BoidFlock::remove<system_critters::wren_boids::sim_wren_boids::closure_env_6>();
      local_130 = (float *)plVar47[1];
      uVar42 = plVar47[2];
      if ((ulonglong)plVar47[5] < (ulonglong)plVar47[2]) {
        uVar42 = plVar47[5];
      }
      lVar46 = plVar47[4];
      local_100 = (longlong *)
                  CONCAT44(local_100._4_4_,*(undefined4 *)((longlong)in_stack_00000038 + 0x14));
      uVar56 = local_178;
      uVar48 = 0;
      lVar44 = local_168;
      goto LAB_1423296d0;
    }
    puVar55 = local_158[0x20];
    while( true ) {
      lVar46 = (ulonglong)*puVar53 * 0xa0;
      uVar42 = *(ulonglong *)(puVar55 + lVar46 + 0x58);
      if (uVar42 != 0) break;
      puVar53 = puVar53 + 1;
      if (puVar53 == local_160) goto LAB_14232950f;
    }
    uVar56 = (ulonglong)*(uint *)(puVar55 + lVar46 + 0x94);
    puVar10 = local_158[0x35];
    lVar44 = *(longlong *)(puVar10 + uVar56 * 0x48 + 0x18);
    if ((*(ulonglong *)(local_150 + 0x118) < *(ulonglong *)(puVar10 + uVar56 * 0x48 + 0x40)) &&
       (uVar48 = *(ulonglong *)
                  (*(longlong *)(puVar10 + uVar56 * 0x48 + 0x38) +
                  *(ulonglong *)(local_150 + 0x118) * 8), uVar48 != 0)) {
      local_148 = *(undefined ***)(lVar44 + 0x10 + ~uVar48 * 0x30);
    }
    else {
      local_148 = (undefined **)0x0;
    }
    lVar43 = ~*(ulonglong *)
               (*(longlong *)(puVar10 + uVar56 * 0x48 + 0x38) + *(longlong *)(local_150 + 0x110) * 8
               ) * 0x30;
    local_190 = *(float **)(lVar44 + 0x10 + lVar43);
    local_208 = *(float **)(lVar44 + 0x28 + lVar43);
    local_3a8 = *(longlong *)(puVar55 + lVar46 + 0x50);
    uVar56 = 0;
    puVar53 = puVar53 + 1;
  }
  local_120 = puVar53;
  puVar40 = (ulonglong *)(local_3a8 + uVar56 * 0x10);
  uVar48 = (ulonglong)*(uint *)(local_3a8 + 8 + uVar56 * 0x10);
  do {
    pbVar4 = (byte *)((longlong)local_190 + uVar48 * 0x28);
    local_130 = local_208 + uVar48;
    local_188 = *puVar40;
    fVar75 = *(float *)(local_220 + 0x28);
    *local_130 = local_108._0_4_;
    *(float *)(pbVar4 + 0x24) = *(float *)(pbVar4 + 0x24) - fVar75;
    uVar36 = 0;
    local_218 = uVar42;
    local_128 = uVar56;
    if ((*(int *)(pbVar4 + 0x10) != 0) && (uVar36 = 0, (ulonglong)plVar47[2] < 0x50)) {
      fVar75 = *(float *)(local_220 + 0x28);
      local_f2 = 1;
      fastrand::Rng::f32();
      fVar75 = fVar75 / 5.0;
      fVar76 = (float)truncf();
      fVar79 = floorf(fVar75);
      iVar37 = (int)(longlong)fVar79;
      if (fVar79 < 0.0) {
        iVar37 = 0;
      }
      if (4.294967e+09 < fVar79) {
        iVar37 = -1;
      }
      uVar36 = (uint)(extraout_XMM0_Da_09 < fVar75 - fVar76) + iVar37;
    }
    local_f2 = 1;
    country_core::resources::placement_data::PlacementDataSsbo::to_affine3a();
    fVar75 = 0.0;
    if ((*pbVar4 & 1) != 0) {
      fVar75 = (float)*(undefined8 *)(pbVar4 + 8) - fStack_3e4;
      fVar76 = (float)((ulonglong)*(undefined8 *)(pbVar4 + 8) >> 0x20) - fStack_3e0;
      fVar75 = SQRT(fVar76 * fVar76 +
                    (*(float *)(pbVar4 + 4) - local_3e8) * (*(float *)(pbVar4 + 4) - local_3e8) +
                    fVar75 * fVar75);
    }
    fVar76 = *(float *)(pbVar4 + 0x14);
    fVar79 = *(float *)(pbVar4 + 0x18);
    fVar81 = *(float *)(pbVar4 + 0x1c);
    fVar71 = *(float *)(pbVar4 + 0x20);
    *local_130 = local_108._0_4_;
    pbVar4[0] = 1;
    pbVar4[1] = 0;
    pbVar4[2] = 0;
    pbVar4[3] = 0;
    *(float *)(pbVar4 + 4) = local_3e8;
    *(float *)(pbVar4 + 8) = fStack_3e4;
    *(float *)(pbVar4 + 0xc) = fStack_3e0;
    fVar82 = *(float *)(local_220 + 0x28);
    if (fVar82 <= 1e-10) {
      fVar82 = 1e-10;
    }
    uVar65 = 0;
    uVar41 = 0;
    if ((*(float *)(pbVar4 + 0x24) <= 0.0) && (uVar41 = 0, 10.0 < fVar75 / fVar82)) {
      if (4 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
        local_f2 = 1;
        toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
        local_2d8 = *extraout_RAX_10;
        local_2c8 = 5;
        puStack_2c0 = &DAT_14304c368;
        local_2b8 = 0x1b;
        local_2a8 = &PTR_s_Tree_has_been_shaken__Spawning_b_14304c340;
        uStack_2a0 = 1;
        local_298 = 8;
        local_2f8 = (undefined **)0x0;
        fStack_290 = 0.0;
        uStack_28c = 0;
        uStack_288 = 0;
        uStack_284 = 0;
        uStack_2f0._0_4_ = 132.7633;
        uStack_2f0._4_4_ = 1;
        uStack_2e8._0_4_ = 0x1b;
        uStack_2e8._4_4_ = 0;
        lStack_2e0 = 0;
        uStack_2b0 = CONCAT44(*(undefined4 *)extraout_RAX_10[1],1);
        local_f2 = 1;
        ::log::__private_api::impl$2::log();
      }
      local_f2 = 1;
      fastrand::Rng::f32();
      *local_130 = local_108._0_4_;
      *(float *)(pbVar4 + 0x24) = extraout_XMM0_Da_10 * 6.0 + (1.0 - extraout_XMM0_Da_10) * 0.8;
      uVar56 = *in_stack_00000040;
      uVar42 = uVar56 + 0xa0761d6478bd642f;
      *in_stack_00000040 = uVar42;
      auVar17._8_8_ = 0;
      auVar17._0_8_ = uVar42 ^ 0xe7037ed1a0b428db;
      auVar24._8_8_ = 0;
      auVar24._0_8_ = uVar42;
      uVar48 = SUB168(auVar17 * auVar24,8) ^ SUB168(auVar17 * auVar24,0);
      if ((int)uVar48 == 0) {
        uVar56 = uVar56 + 0x40ec3ac8f17ac85e;
        do {
          uVar42 = uVar42 + 0xa0761d6478bd642f;
          auVar18._8_8_ = 0;
          auVar18._0_8_ = uVar56 ^ 0xe7037ed1a0b428db;
          auVar25._8_8_ = 0;
          auVar25._0_8_ = uVar56;
          uVar48 = SUB168(auVar18 * auVar25,8) ^ SUB168(auVar18 * auVar25,0);
          uVar56 = uVar56 + 0xa0761d6478bd642f;
        } while ((int)uVar48 == 0);
        *in_stack_00000040 = uVar42;
      }
      uVar41 = (int)((uVar48 & 0xffffffff) * 5 >> 0x20) + 1;
    }
    local_128 = local_128 + 1;
    fVar75 = fVar71 * 0.0;
    fVar82 = fVar75 * (float)local_3f8 + (float)local_408 * fVar75 + fVar71 * (float)local_418;
    fVar68 = fVar75 * local_3f8._4_4_ + local_408._4_4_ * fVar75 + fVar71 * local_418._4_4_;
    fVar75 = fVar75 * (float)uStack_3f0 + (float)lStack_400 * fVar75 + fVar71 * pfStack_410._0_4_;
    if (uVar36 < uVar41) {
      uVar36 = uVar41;
    }
    local_100 = (longlong *)CONCAT44(local_100._4_4_,uVar36);
    fVar75 = SQRT(fVar75 * fVar75 + fVar68 * fVar68 + fVar82 * fVar82);
    for (; uVar65 < (uint)local_100; uVar65 = uVar65 + 1) {
      iVar37 = *(int *)(pbVar4 + 0x10) + -1;
      if (*(int *)(pbVar4 + 0x10) == 0) {
        iVar37 = 0;
      }
      *local_130 = local_108._0_4_;
      *(int *)(pbVar4 + 0x10) = iVar37;
      local_f2 = 1;
      fastrand::Rng::f32();
      fVar68 = extraout_XMM0_Da_11 + -0.5;
      local_f2 = 1;
      fastrand::Rng::f32();
      local_f2 = 1;
      fastrand::Rng::f32();
      fVar77 = extraout_XMM0_Da_12 * 0.5 + 0.2;
      fVar82 = extraout_XMM0_Da_13 + -0.5;
      fVar71 = 1.0 / SQRT(fVar82 * fVar82 + fVar77 * fVar77 + fVar68 * fVar68);
      bVar31 = (int)fVar71 - 1U < 0x7fffff ||
               (int)ABS(fVar71) - 0x800000U < 0x7f000000 && -1 < (int)fVar71;
      if (bVar31) {
        fVar82 = fVar82 * fVar71;
      }
      else {
        fVar82 = 0.0;
      }
      uVar36 = (uint)bVar31;
      uVar41 = (int)(uVar36 << 0x1f) >> 0x1f;
      fVar68 = (float)((uint)(fVar71 * fVar68) & (int)(uVar36 << 0x1f) >> 0x1f);
      fVar71 = (float)(~uVar41 & 0x3f800000 | (uint)(fVar71 * fVar77) & uVar41);
      local_f2 = 1;
      ::utils::components::transform::impl$1::default();
      ppuVar32 = local_2f8;
      uVar13 = CONCAT44(uStack_2f0._4_4_,(float)uStack_2f0);
      local_f2 = 1;
      country_core::resources::audio_resources::AudioCore::instantiate_audio();
      if ((int)local_1e8 == 2) {
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
      country_core::resources::audio_resources::AudioInstance::start_if_not_playing();
      country_core::resources::audio_resources::AudioInstance::stop_on_drop();
      uVar56 = *in_stack_00000040;
      uVar42 = uVar56 + 0xa0761d6478bd642f;
      *in_stack_00000040 = uVar42;
      auVar19._8_8_ = 0;
      auVar19._0_8_ = uVar42 ^ 0xe7037ed1a0b428db;
      auVar26._8_8_ = 0;
      auVar26._0_8_ = uVar42;
      uVar48 = SUB168(auVar19 * auVar26,8) ^ SUB168(auVar19 * auVar26,0);
      if ((uint)((int)uVar48 * 100) < 0x60) {
        uVar56 = uVar56 + 0x40ec3ac8f17ac85e;
        do {
          uVar42 = uVar42 + 0xa0761d6478bd642f;
          auVar20._8_8_ = 0;
          auVar20._0_8_ = uVar56 ^ 0xe7037ed1a0b428db;
          auVar27._8_8_ = 0;
          auVar27._0_8_ = uVar56;
          uVar48 = SUB168(auVar20 * auVar27,8) ^ SUB168(auVar20 * auVar27,0);
          uVar56 = uVar56 + 0xa0761d6478bd642f;
        } while ((uint)((int)uVar48 * 100) < 0x60);
        *in_stack_00000040 = uVar42;
      }
      iStack_19c = (int)((uVar48 & 0xffffffff) * 100 >> 0x20) + uVar65;
      uStack_1a4 = 0;
      uStack_1a0 = 0;
      local_1b8 = 2;
      bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
      uVar83 = CONCAT44(fVar75 * fVar71 * 1.01 +
                        fStack_3e4 +
                        fVar81 * local_3f8._4_4_ +
                        fVar79 * local_408._4_4_ + fVar76 * local_418._4_4_,
                        fVar75 * fVar68 * 1.01 +
                        local_3e8 +
                        fVar81 * (float)local_3f8 +
                        fVar79 * (float)local_408 + fVar76 * (float)local_418);
      fVar77 = fVar75 * fVar82 * 1.01 +
               fStack_3e0 +
               fVar81 * (float)uStack_3f0 + fVar79 * (float)lStack_400 + fVar76 * pfStack_410._0_4_;
      uStack_278 = uStack_278 & 0xffffffff00000000;
      local_270 = 0x6df6147cdf0aeade;
      local_2d8._8_8_ = uStack_1c0;
      local_2d8._0_8_ = local_1c8;
      local_2f8 = local_1e8;
      uStack_2f0._0_4_ = SUB84(pfStack_1e0,0);
      uStack_2f0._4_4_ = (int)((ulonglong)pfStack_1e0 >> 0x20);
      uStack_2e8._0_4_ = SUB84(local_1d8,0);
      uStack_2e8._4_4_ = (undefined4)((ulonglong)local_1d8 >> 0x20);
      lStack_2e0 = lStack_1d0;
      local_2c8 = local_1b8;
      puStack_2c0 = puStack_1b0;
      local_2b8 = CONCAT44(uStack_1a4,local_1a8);
      uStack_2b0 = CONCAT44(iStack_19c,uStack_1a0);
      local_2a8 = ppuVar32;
      uStack_28c = 0x3f800000;
      uStack_288 = 0x3f800000;
      uStack_284 = 0x3f800000;
      local_268 = 0x11;
      local_258 = ppuVar32;
      local_23c = 0x3f8000003f800000;
      local_234 = 0x3f800000;
      local_f2 = 1;
      uStack_2a0 = uVar13;
      local_298 = uVar83;
      fStack_290 = fVar77;
      uStack_250 = uVar13;
      local_248 = uVar83;
      fStack_240 = fVar77;
      bevy_ecs::system::commands::Commands::
      spawn<tuple_<country_core::resources::critters::Wren,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::WrenInstanceData>,country_core::components::drawable::PrevTransform>_>
                ();
      lVar44 = local_1c8;
      lVar46 = plVar47[2];
      if (lVar46 == *plVar47) {
        local_f2 = 1;
        alloc::raw_vec::RawVec::grow_one<system_critters::boids::Boid,alloc::alloc::Global>();
      }
      lVar43 = plVar47[1];
      *(undefined8 *)(lVar43 + lVar46 * 0x18) = uVar83;
      *(float *)(lVar43 + 8 + lVar46 * 0x18) = fVar77;
      pfVar66 = (float *)(lVar43 + 0xc + lVar46 * 0x18);
      *pfVar66 = fVar68;
      pfVar66[1] = fVar71;
      *(float *)(lVar43 + 0x14 + lVar46 * 0x18) = fVar82;
      plVar47[2] = lVar46 + 1;
      lVar46 = plVar47[5];
      if (lVar46 == plVar47[3]) {
        local_f2 = 1;
        alloc::raw_vec::RawVec::grow_one<enum2_<iced_core::event::Event>,alloc::alloc::Global>();
      }
      lVar43 = plVar47[4];
      lVar58 = lVar46 * 0x38;
      *(undefined4 *)(lVar43 + lVar58) = 0;
      *(longlong *)(lVar43 + 0x10 + lVar58) = lVar44;
      *(undefined8 *)(lVar43 + 0x18 + lVar58) = 0;
      *(undefined4 *)(lVar43 + 0x20 + lVar58) = 0x3f800000;
      *(undefined8 *)(lVar43 + 0x24 + lVar58) = 0;
      *(undefined4 *)(lVar43 + 0x2c + lVar58) = 0x3f800000;
      *(undefined4 *)(lVar43 + 0x30 + lVar58) = 0;
      plVar47[5] = lVar46 + 1;
      if (SBORROW8(0,plVar47[6])) {
        local_f2 = 1;
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      lVar46 = plVar47[8];
      if (lVar46 == plVar47[6]) {
        local_f2 = 1;
        alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
      }
      *(longlong *)(plVar47[7] + lVar46 * 8) = lVar44;
      plVar47[8] = lVar46 + 1;
    }
    if ((uint)local_100 == 0) {
      if (local_180 < 1000) {
        if (*(uint *)(pbVar4 + 0x10) < 5) {
LAB_142329492:
          local_f2 = 1;
          fastrand::Rng::f32();
          local_304 = local_304 + 1;
          if (extraout_XMM0_Da_14 < 1.0 / (float)local_304) {
            local_398 = local_188 >> 0x20;
            local_374 = (uint)local_188;
          }
          goto LAB_142329429;
        }
      }
      else if ((1000 < local_180) && (*(int *)(pbVar4 + 0x10) != 0)) goto LAB_142329492;
      uVar42 = local_218;
      uVar56 = local_128;
      puVar53 = local_120;
    }
    else {
LAB_142329429:
      uVar42 = local_218;
      uVar56 = local_128;
      puVar53 = local_120;
    }
    local_218 = uVar42;
    local_128 = uVar56;
    local_120 = puVar53;
    if ((char)local_210 == '\0') goto LAB_142328967;
LAB_142328a80:
    if (uVar56 == uVar42) {
      if (puVar53 == local_160) goto LAB_14232950f;
      puVar55 = local_158[0x35];
      while( true ) {
        uVar56 = (ulonglong)*puVar53;
        uVar42 = *(ulonglong *)(puVar55 + uVar56 * 0x48 + 0x10);
        if (uVar42 != 0) break;
        puVar53 = puVar53 + 1;
        if (puVar53 == local_160) goto LAB_14232950f;
      }
      lVar46 = *(longlong *)(puVar55 + uVar56 * 0x48 + 0x18);
      if ((*(ulonglong *)(local_150 + 0x118) < *(ulonglong *)(puVar55 + uVar56 * 0x48 + 0x40)) &&
         (uVar48 = *(ulonglong *)
                    (*(longlong *)(puVar55 + uVar56 * 0x48 + 0x38) +
                    *(ulonglong *)(local_150 + 0x118) * 8), uVar48 != 0)) {
        local_148 = *(undefined ***)(lVar46 + 0x10 + ~uVar48 * 0x30);
      }
      else {
        local_148 = (undefined **)0x0;
      }
      lVar44 = ~*(ulonglong *)
                 (*(longlong *)(puVar55 + uVar56 * 0x48 + 0x38) +
                 *(longlong *)(local_150 + 0x110) * 8) * 0x30;
      local_190 = *(float **)(lVar46 + 0x10 + lVar44);
      local_208 = *(float **)(lVar46 + 0x28 + lVar44);
      local_3a0 = *(longlong *)(puVar55 + uVar56 * 0x48 + 8);
      uVar56 = 0;
      puVar53 = puVar53 + 1;
    }
    local_120 = puVar53;
    puVar40 = (ulonglong *)(local_3a0 + uVar56 * 8);
    uVar48 = uVar56 & 0xffffffff;
  } while( true );
LAB_142328250:
  uVar45 = *(ulonglong *)((longlong)pfVar54 * 0x38 + local_128 + 0x10);
  uVar50 = uVar45 & 0xffffffff;
  pfVar54 = pfVar2;
  uVar56 = local_180;
  if ((uVar50 < *(ulonglong *)(local_178 + 0x10)) &&
     (*(int *)(*(longlong *)(local_178 + 8) + uVar50 * 0x14) == (int)(uVar45 >> 0x20))) {
    lVar44 = *(longlong *)(local_178 + 8) + uVar50 * 0x14;
    uVar45 = (ulonglong)*(uint *)(lVar44 + 4);
    if ((((uVar45 != 0xffffffff) && (uVar45 < *(ulonglong *)(local_168 + 0x38))) &&
        ((*(ulonglong *)
           (*(longlong *)(local_168 + 0x28) + (ulonglong)(*(uint *)(lVar44 + 4) >> 6) * 8) >>
          (uVar45 & 0x3f) & 1) != 0)) &&
       (plVar52 = local_100,
       *(int *)(*(longlong *)
                 (*(longlong *)
                   (*(longlong *)(local_178 + 0x1a8) + 0x18 +
                   (ulonglong)*(uint *)(lVar44 + 0xc) * 0x48) + 0x10 +
                 ~*(ulonglong *)
                   (*(longlong *)
                     (*(longlong *)(local_178 + 0x1a8) + 0x38 +
                     (ulonglong)*(uint *)(lVar44 + 0xc) * 0x48) +
                   *(longlong *)(local_168 + 0x118) * 8) * 0x30) + 0x30 +
               (ulonglong)*(uint *)(lVar44 + 0x10) * 0x50) != 0)) {
      local_2f8 = (undefined **)&local_131;
      uStack_2f0._0_4_ = (float)in_stack_00000028;
      uStack_2f0._4_4_ = (int)((ulonglong)in_stack_00000028 >> 0x20);
      uStack_2e8._0_4_ = (undefined4)local_218;
      uStack_2e8._4_4_ = (undefined4)(local_218 >> 0x20);
      local_f2 = 1;
      local_188 = uVar42;
      boids::Boid::frame_update<system_critters::wren_boids::sim_wren_boids::closure_env_3>();
      ppuVar63 = local_1f0;
      plVar52 = local_100;
      pfVar61 = local_130;
      uVar42 = local_188;
      if (extraout_AL_01 == '\0') {
        if (local_3c0 != 0) {
          uVar56 = 0;
          pfVar54 = local_208;
          do {
            fVar75 = pfVar54[1] - pfVar3[1];
            if (-4.0 <= fVar75) {
              auVar72._0_4_ = pfVar54[2] - pfVar3[2];
              auVar72._4_4_ = *pfVar54 - *pfVar3;
              auVar72._8_8_ = 0;
              fVar76 = SQRT(auVar72._4_4_ * auVar72._4_4_ + fVar75 * fVar75 +
                            auVar72._0_4_ * auVar72._0_4_);
              if (fVar76 <= 8.0) {
                auVar29._4_4_ = fVar76;
                auVar29._0_4_ = fVar76;
                auVar29._8_8_ = 0;
                auVar73 = divps(auVar72,auVar29);
                fVar79 = pfVar3[3];
                fVar81 = (float)*(longlong *)(pfVar3 + 4);
                fVar71 = (float)((ulonglong)*(longlong *)(pfVar3 + 4) >> 0x20);
                auVar84._4_12_ = __xmm_00000000000000003f80000000000000._4_12_;
                auVar84._0_4_ = fVar75;
                auVar30._4_4_ = SQRT(fVar71 * fVar71 + fVar79 * fVar79 + fVar81 * fVar81);
                auVar30._0_4_ = fVar76;
                auVar30._8_8_ = 0;
                auVar85 = divps(auVar84,auVar30);
                fVar75 = auVar85._4_4_;
                if (0.7 <= fVar79 * fVar75 * auVar73._4_4_ + fVar81 * fVar75 * auVar85._0_4_ +
                           fVar71 * fVar75 * auVar73._0_4_) {
                  if (local_3c0 <= uVar56) {
                    local_f2 = 1;
                    // WARNING: Subroutine does not return
                    alloc::vec::impl$1::swap_remove::assert_failed();
                  }
                  local_3c0 = local_3c0 - 1;
                  uVar13 = *(undefined8 *)(local_208 + local_3c0 * 4 + 2);
                  *(undefined8 *)pfVar54 = *(undefined8 *)(local_208 + local_3c0 * 4);
                  *(undefined8 *)(pfVar54 + 2) = uVar13;
                  local_f2 = 1;
                  hashbrown::map::HashMap::
                  insert<bevy_ecs::entity::Entity,system_critters::wren_boids::sim_wren_boids::FreeSpot,std::hash::random::RandomState,alloc::alloc::Global>
                            ();
                  pfVar54 = pfVar2;
                  plVar52 = local_100;
                  pfVar61 = local_130;
                  uVar42 = local_188;
                  uVar56 = local_180;
                  ppuVar32 = (undefined **)CONCAT44(uStack_2f0._4_4_,(float)uStack_2f0);
                  ppuVar33 = (undefined **)CONCAT44(uStack_2e8._4_4_,(undefined4)uStack_2e8);
                  goto LAB_1423281a8;
                }
              }
            }
            pfVar54 = pfVar54 + 4;
            uVar56 = uVar56 + 1;
          } while (pfVar54 != local_208 + local_3c0 * 4);
        }
        pfVar54 = pfVar2;
        uVar56 = local_180;
        ppuVar32 = (undefined **)CONCAT44(uStack_2f0._4_4_,(float)uStack_2f0);
        ppuVar33 = (undefined **)CONCAT44(uStack_2e8._4_4_,(undefined4)uStack_2e8);
        if ((0x50 < uVar51) &&
           (ppuVar32 = (undefined **)CONCAT44(uStack_2f0._4_4_,(float)uStack_2f0),
           ppuVar33 = (undefined **)CONCAT44(uStack_2e8._4_4_,(undefined4)uStack_2e8),
           local_120 != (uint *)0x0)) {
          uVar45 = 0;
          pfVar39 = pfVar66;
          do {
            if ((pfVar3[2] - pfVar39[2]) * (pfVar3[2] - pfVar39[2]) +
                (pfVar3[1] - pfVar39[1]) * (pfVar3[1] - pfVar39[1]) +
                (*pfVar3 - *pfVar39) * (*pfVar3 - *pfVar39) < pfVar39[3] * pfVar39[3]) {
              if (local_160 <= uVar45) {
                local_f2 = 1;
                    // WARNING: Subroutine does not return
                core::panicking::panic_bounds_check();
              }
              uVar50 = *(ulonglong *)(lVar46 + uVar45 * 8);
              puVar55 = (undefined *)(uVar50 & 0xffffffff);
              iVar37 = (int)(uVar50 >> 0x20);
              local_180 = uVar56;
              if ((puVar55 < local_158[2]) &&
                 (puVar10 = local_158[1], *(int *)(puVar10 + (longlong)puVar55 * 0x14) == iVar37)) {
                uVar57 = (ulonglong)*(uint *)(puVar10 + (longlong)puVar55 * 0x14 + 4);
                if (uVar57 != 0xffffffff) {
                  if ((*(ulonglong *)(local_150 + 0x38) <= uVar57) ||
                     ((*(ulonglong *)
                        (*(longlong *)(local_150 + 0x28) +
                        (ulonglong)(*(uint *)(puVar10 + (longlong)puVar55 * 0x14 + 4) >> 6) * 8) >>
                       (uVar57 & 0x3f) & 1) == 0)) {
                    local_2f8 = (undefined **)(uVar57 << 0x20);
                    goto LAB_14232a07b;
                  }
                  uVar50 = (ulonglong)*(uint *)(puVar10 + (longlong)puVar55 * 0x14 + 0x10);
                  lVar44 = *(longlong *)
                            (local_158[0x35] +
                            (ulonglong)*(uint *)(puVar10 + (longlong)puVar55 * 0x14 + 0xc) * 0x48 +
                            0x18);
                  lVar58 = ~*(ulonglong *)
                             (*(longlong *)
                               (local_158[0x35] +
                               (ulonglong)*(uint *)(puVar10 + (longlong)puVar55 * 0x14 + 0xc) * 0x48
                               + 0x38) + *(longlong *)(local_150 + 0x110) * 8) * 0x30;
                  lVar43 = *(longlong *)(lVar44 + 0x10 + lVar58);
                  uVar56 = uVar50 * 4 + *(longlong *)(lVar44 + 0x20 + lVar58);
                  *(float *)(*(longlong *)(lVar44 + 0x28 + lVar58) + uVar50 * 4) = local_108._0_4_;
                  uVar36 = *(uint *)(lVar43 + 0x10 + uVar50 * 0x28);
                  if (4 < uVar36) goto LAB_142328621;
                  *(uint *)(lVar43 + uVar50 * 0x28 + 0x10) = uVar36 + 1;
                  local_180 = uVar56;
                  if (local_1f0 == local_200) {
                    local_f2 = 1;
                    alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
                    plVar52 = local_100;
                    pfVar61 = local_130;
                    uVar42 = local_188;
                  }
                  goto LAB_142328190;
                }
              }
              local_2f8 = (undefined **)((uVar56 & 0xffffffff00000000) + 1);
LAB_14232a07b:
              uStack_2f0._0_4_ = (float)uVar50;
              local_f2 = 1;
              uStack_2f0._4_4_ = iVar37;
                    // WARNING: Subroutine does not return
              core::result::unwrap_failed();
            }
LAB_142328621:
            pfVar39 = pfVar39 + 4;
            uVar45 = uVar45 + 1;
            ppuVar32 = uStack_2f0;
            ppuVar33 = uStack_2e8;
          } while (pfVar39 != local_190);
        }
      }
      else {
        if (local_1f0 == local_200) {
          local_f2 = 1;
          alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
          plVar52 = local_100;
          pfVar61 = local_130;
          uVar42 = local_188;
        }
LAB_142328190:
        *(ulonglong *)(lStack_1f8 + (longlong)ppuVar63 * 8) = uVar48;
        local_1f0 = (undefined **)((longlong)ppuVar63 + 1);
        pfVar54 = pfVar2;
        uVar56 = local_180;
        ppuVar32 = uStack_2f0;
        ppuVar33 = uStack_2e8;
      }
    }
  }
  goto LAB_1423281a8;
LAB_1423296d0:
  uVar45 = uVar48;
  if (uVar45 < uVar42) {
    uVar48 = uVar45 + 1;
    lVar43 = uVar45 * 0x38;
    uVar50 = *(ulonglong *)(lVar46 + 0x10 + lVar43);
    uVar57 = uVar50 & 0xffffffff;
    if ((uVar57 < *(ulonglong *)(uVar56 + 0x10)) &&
       (*(int *)(*(longlong *)(uVar56 + 8) + uVar57 * 0x14) == (int)(uVar50 >> 0x20))) {
      lVar58 = *(longlong *)(uVar56 + 8) + uVar57 * 0x14;
      uVar50 = (ulonglong)*(uint *)(lVar58 + 4);
      if ((uVar50 != 0xffffffff) &&
         ((uVar50 < *(ulonglong *)(lVar44 + 0x38) &&
          ((*(ulonglong *)
             (*(longlong *)(lVar44 + 0x28) + (ulonglong)(*(uint *)(lVar58 + 4) >> 6) * 8) >>
            (uVar50 & 0x3f) & 1) != 0)))) {
        puVar1 = (undefined4 *)(lVar46 + lVar43);
        uVar50 = (ulonglong)*(uint *)(lVar58 + 0x10);
        lVar11 = *(longlong *)
                  (*(longlong *)(uVar56 + 0x1a8) + 0x18 + (ulonglong)*(uint *)(lVar58 + 0xc) * 0x48)
        ;
        lVar58 = *(longlong *)
                  (*(longlong *)(uVar56 + 0x1a8) + 0x38 + (ulonglong)*(uint *)(lVar58 + 0xc) * 0x48)
        ;
        lVar59 = ~*(ulonglong *)(lVar58 + *(longlong *)(lVar44 + 0x110) * 8) * 0x30;
        local_118 = *(longlong **)(lVar11 + 0x28 + lVar59);
        lVar58 = ~*(ulonglong *)(lVar58 + *(longlong *)(lVar44 + 0x118) * 8) * 0x30;
        lVar12 = *(longlong *)(lVar11 + 0x10 + lVar58);
        plVar47 = *(longlong **)(lVar11 + 0x28 + lVar58);
        pfVar54 = (float *)(uVar50 * 0x30 + *(longlong *)(lVar11 + 0x10 + lVar59));
        lVar58 = lVar12 + uVar50 * 0x50;
        *puVar1 = 0;
        iVar37 = *(int *)(lVar12 + 0x30 + uVar50 * 0x50);
        pfVar66 = (float *)((longlong)local_130 + uVar45 * 0x18);
        if (iVar37 != 2) {
          if (iVar37 != 4) {
            lVar43 = *(longlong *)(pfVar54 + 4);
            pfVar66[2] = pfVar54[6];
            *(longlong *)pfVar66 = lVar43;
            fVar75 = *pfVar54;
            fVar76 = pfVar54[1];
            fVar79 = pfVar54[2];
            fVar81 = pfVar54[3];
            fVar82 = fVar81 * fVar81;
            fVar68 = fVar79 * fVar79 + fVar76 * fVar76 + fVar75 * fVar75;
            fVar71 = fVar79 * -1.0 + fVar76 * -0.0 + fVar75 * -0.0;
            fVar71 = fVar71 + fVar71;
            fVar81 = fVar81 + fVar81;
            pfVar66[3] = (fVar81 * (fVar76 * -1.0 - fVar79 * -0.0) +
                         fVar71 * fVar75 + (fVar82 - fVar68) * -0.0) * 10.0;
            pfVar66[4] = (fVar81 * (fVar79 * -0.0 - fVar75 * -1.0) +
                         fVar71 * fVar76 + (fVar82 - fVar68) * -0.0) * 10.0;
            pfVar66[5] = (fVar81 * (fVar75 * -0.0 - fVar76 * -0.0) +
                         fVar71 * fVar79 + (fVar82 - fVar68) * -1.0) * 10.0;
            uVar56 = local_178;
            goto LAB_1423296d0;
          }
          fVar75 = *(float *)(lVar58 + 0x38) - pfVar66[1];
          if (-4.0 <= fVar75) {
            auVar70._0_4_ = *(float *)(lVar58 + 0x3c) - pfVar66[2];
            auVar70._4_4_ = *(float *)(lVar58 + 0x34) - *pfVar66;
            auVar70._8_8_ = 0;
            fVar76 = SQRT(auVar70._4_4_ * auVar70._4_4_ + fVar75 * fVar75 +
                          auVar70._0_4_ * auVar70._0_4_);
            if (fVar76 <= 8.0) {
              auVar74._4_4_ = fVar76;
              auVar74._0_4_ = fVar76;
              auVar74._8_8_ = 0;
              auVar73 = divps(auVar70,auVar74);
              fVar79 = pfVar66[3];
              fVar81 = (float)*(undefined8 *)(pfVar66 + 4);
              fVar71 = (float)((ulonglong)*(undefined8 *)(pfVar66 + 4) >> 0x20);
              auVar78._4_12_ = __xmm_00000000000000003f80000000000000._4_12_;
              auVar78._0_4_ = fVar75;
              auVar28._4_4_ = SQRT(fVar71 * fVar71 + fVar79 * fVar79 + fVar81 * fVar81);
              auVar28._0_4_ = fVar76;
              auVar28._8_8_ = 0;
              auVar85 = divps(auVar78,auVar28);
              fVar75 = auVar85._4_4_;
              if (0.7 <= fVar79 * fVar75 * auVar73._4_4_ + fVar81 * fVar75 * auVar85._0_4_ +
                         fVar71 * fVar75 * auVar73._0_4_) {
                uVar9 = *(undefined4 *)(lVar58 + 0x3c);
                uVar13 = *(undefined8 *)(lVar58 + 0x34);
                *puVar1 = 1;
                puVar1[3] = uVar9;
                *(undefined8 *)(puVar1 + 1) = uVar13;
                goto LAB_142329a00;
              }
            }
          }
          *(uint *)((longlong)plVar47 + uVar50 * 4) = (uint)local_100;
          *(undefined8 *)(lVar58 + 0x30) = 2;
        }
LAB_142329a00:
        local_108 = (uint *)(lVar43 + lVar46 + 0x10);
        uStack_2f0._0_4_ = pfVar66[2];
        local_2f8 = *(undefined ***)pfVar66;
        *(uint *)((longlong)local_118 + uVar50 * 4) = (uint)local_100;
        pfVar54[6] = (float)uStack_2f0;
        *(undefined ***)(pfVar54 + 4) = local_2f8;
        fVar75 = (float)*(undefined8 *)(pfVar66 + 3);
        fVar76 = (float)((ulonglong)*(undefined8 *)(pfVar66 + 3) >> 0x20);
        fVar75 = 1.0 / SQRT(pfVar66[5] * pfVar66[5] + fVar76 * fVar76 + fVar75 * fVar75);
        local_198 = plVar47;
        if ((int)fVar75 - 1U < 0x7fffff ||
            (int)ABS(fVar75) - 0x800000U < 0x7f000000 && -1 < (int)fVar75) {
          fVar76 = fVar76 * fVar75;
          if (-0.3 < fVar76) goto LAB_142329ba7;
          *(uint *)((longlong)plVar47 + uVar50 * 4) = (uint)local_100;
          fVar75 = fmodf(*(float *)(lVar58 + 0x44),1.0);
          *(float *)(lVar58 + 0x44) = fVar75;
          dVar69 = fmod((double)*local_108 * 1.6180340051651,0.3);
          sin((dVar69 + 1.0) * *(double *)(local_220 + 0x10) * 4.0);
          iVar37 = (uint)local_100;
          *(uint *)((longlong)local_198 + uVar50 * 4) = (uint)local_100;
          local_f2 = 0;
          ::utils::time_continuous_lerp_mut();
        }
        else {
          fVar76 = 0.0;
LAB_142329ba7:
          fVar75 = *(float *)(local_220 + 0x28);
          *(uint *)((longlong)plVar47 + uVar50 * 4) = (uint)local_100;
          *(float *)(lVar58 + 0x44) =
               (fVar76 * 0.5 + 0.5) * fVar75 * 7.0 + *(float *)(lVar58 + 0x44);
          iVar37 = (uint)local_100;
        }
        local_f2 = 0;
        dolly::util::
        look_at<dolly::handedness::RightHanded,glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat>();
        *(int *)((longlong)local_118 + uVar50 * 4) = iVar37;
        *(undefined ***)pfVar54 = local_2f8;
        *(longlong *)(pfVar54 + 2) = CONCAT44(uStack_2f0._4_4_,(float)uStack_2f0);
        local_f2 = 0;
        core::hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
        local_f2 = 0;
        hashbrown::raw::RawTable::
        remove_entry<tuple_<bevy_ecs::entity::Entity,system_critters::wren_boids::sim_wren_boids::FreeSpot>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<bevy_ecs::entity::Entity,bevy_ecs::entity::Entity,system_critters::wren_boids::sim_wren_boids::FreeSpot>_>
                  ();
        uVar56 = local_178;
        lVar44 = local_168;
        if (local_2f8._4_4_ != 0) {
          pfStack_1e0 = (float *)CONCAT44(pfStack_1e0._4_4_,(undefined4)uStack_2e8);
          local_1e8 = (undefined **)CONCAT44(uStack_2f0._4_4_,(float)uStack_2f0);
          *(int *)((longlong)local_198 + uVar50 * 4) = iVar37;
          *(undefined4 *)(lVar58 + 0x30) = 4;
          *(undefined ***)(lVar58 + 0x34) = local_1e8;
          *(undefined4 *)(lVar58 + 0x3c) = (undefined4)uStack_2e8;
          *(undefined4 *)(lVar58 + 0x40) = uStack_2e8._4_4_;
        }
      }
    }
    goto LAB_1423296d0;
  }
  if (uVar51 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (lVar38 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (ppuVar62 != (undefined **)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
                    // WARNING: Read-only address (ram,0x0001429345b0) is written
  if (local_3d0 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (!bVar34) {
    return;
  }
  puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
  if (*extraout_RAX_11 == 1) {
    plVar47 = extraout_RAX_11 + 1;
  }
  else if ((*extraout_RAX_11 != 0) ||
          (std::sys::thread_local::native::lazy::Storage::
           initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                     (), plVar47 = extraout_RAX_14, extraout_RAX_14 == (longlong *)0x0))
  goto LAB_142329ea1;
  if (*plVar47 != 0) goto LAB_142329ead;
  *plVar47 = -1;
  local_118 = plVar47;
  puffin::ThreadProfiler::end_scope();
LAB_142329de9:
  *local_118 = *local_118 + 1;
                    // WARNING: Read-only address (ram,0x0001429345b0) is written
  return;
}

