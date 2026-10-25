// address 0x142075070
// name nani_update_solid_vertex_color_window_glass
// signature void __cdecl nani_update_solid_vertex_color_window_glass(void)

void __cdecl country_core::render::nani::systems::nani_update_solid_vertex_color_window_glass(void)

{
  byte *pbVar1;
  char *pcVar2;
  byte bVar3;
  byte bVar4;
  longlong lVar5;
  longlong lVar6;
  longlong lVar7;
  ulonglong uVar8;
  uint uVar9;
  uint uVar10;
  int iVar11;
  int iVar12;
  int iVar13;
  int iVar14;
  char cVar15;
  char cVar16;
  char cVar17;
  char cVar18;
  char cVar19;
  char cVar20;
  char cVar21;
  char cVar22;
  char cVar23;
  char cVar24;
  char cVar25;
  char cVar26;
  char cVar27;
  char cVar28;
  char cVar29;
  char cVar30;
  bool bVar31;
  char extraout_AL;
  char extraout_AL_00;
  uint uVar32;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong lVar33;
  ulonglong extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  longlong *in_RCX;
  longlong *plVar34;
  longlong *in_RDX;
  ulonglong uVar35;
  uint *puVar36;
  ushort uVar37;
  longlong *in_R8;
  undefined *puVar38;
  ulonglong *puVar39;
  ulonglong uVar40;
  longlong lVar41;
  byte bVar42;
  char cVar43;
  char cVar45;
  char cVar46;
  char cVar47;
  undefined1 auVar44 [16];
  undefined1 auVar48 [16];
  undefined1 auVar49 [16];
  undefined1 unaff_XMM6_Ba;
  undefined1 unaff_XMM6_Bb;
  char cVar50;
  undefined1 unaff_XMM6_Bc;
  char cVar51;
  undefined1 unaff_XMM6_Bd;
  char cVar52;
  undefined1 unaff_XMM6_Be;
  undefined1 unaff_XMM6_Bf;
  char cVar53;
  undefined1 unaff_XMM6_Bg;
  char cVar54;
  undefined1 unaff_XMM6_Bh;
  char cVar55;
  undefined1 unaff_XMM6_Bi;
  undefined1 unaff_XMM6_Bj;
  char cVar56;
  undefined1 unaff_XMM6_Bk;
  char cVar57;
  undefined1 unaff_XMM6_Bl;
  char cVar58;
  undefined1 unaff_XMM6_Bm;
  undefined1 unaff_XMM6_Bn;
  char cVar59;
  undefined1 unaff_XMM6_Bo;
  char cVar60;
  undefined1 unaff_XMM6_Bp;
  char cVar61;
  undefined1 auVar62 [16];
  undefined4 local_258;
  undefined4 uStack_254;
  undefined4 uStack_250;
  undefined4 uStack_24c;
  longlong local_238;
  ulonglong local_228;
  ulonglong local_220;
  longlong local_1f8;
  longlong local_1f0;
  longlong local_1e8;
  byte local_1e0;
  byte local_1d8;
  ulonglong local_130;
  longlong local_128;
  longlong local_108;
  longlong local_f8;
  longlong local_f0;
  
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    bVar31 = false;
  }
  else {
    uVar35 = 0x50;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar35 != 0) {
        if (uVar35 < 0x50) {
          if ((char)(&DAT_142fd2e35)[uVar35] < -0x40) {
LAB_14207512b:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar35 != 0x50) goto LAB_14207512b;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar35 != 0xfffffffffffffffe)) {
        if (uVar35 + 2 < 0x50) {
          if ((char)(&UNK_142fd2e37)[uVar35] < -0x40) {
LAB_142075189:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar35 + 2 != 0x50) goto LAB_142075189;
      }
    }
    puVar38 = &DAT_142fd345e;
    do {
      bVar42 = puVar38[-1];
      uVar32 = (uint)(char)bVar42;
      if ((char)bVar42 < '\0') {
        bVar3 = puVar38[-2];
        if ((char)bVar3 < -0x40) {
          bVar4 = puVar38[-3];
          if ((char)bVar4 < -0x40) {
            pbVar1 = puVar38 + -4;
            puVar38 = puVar38 + -4;
            uVar32 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar38 = puVar38 + -3;
            uVar32 = bVar4 & 0xf;
          }
          uVar32 = bVar3 & 0x3f | uVar32 << 6;
        }
        else {
          puVar38 = puVar38 + -2;
          uVar32 = bVar3 & 0x1f;
        }
        uVar32 = (uint)(bVar42 & 0x3f) | uVar32 << 6;
        if (uVar32 != 0x5c) goto LAB_142075254;
LAB_142075266:
        if (puVar38 == &UNK_142fd342f) break;
        if (puVar38 + -0x142fd342f < (undefined *)0x2e) {
          if (-0x41 < (char)puVar38[1]) break;
        }
        else if (puVar38 + -0x142fd342f == (undefined *)0x2e) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar38 = puVar38 + -1;
      if (uVar32 == 0x5c) goto LAB_142075266;
LAB_142075254:
      if (uVar32 == 0x2f) goto LAB_142075266;
    } while (puVar38 != &DAT_142fd3430);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar34 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar34 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0))
    goto LAB_142075ed5;
    if (*plVar34 != 0) goto LAB_142075ee1;
    *plVar34 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar34 = *plVar34 + 1;
    bVar31 = true;
  }
  gpu_world::GpuWorld::owned_subset_mut();
  lVar5 = *in_RCX;
  lVar6 = in_RCX[1];
  bevy_ecs::query::iter::QueryIterationCursor::
  init<tuple_<ref_<country_core::components::drawable::NaniActiveInstance>,ref_<utils::components::transform::Transform>,ref_<country_core::resources::window_lights::WindowLight>,ref_<country_core::resources::color_id::ColorId>,enum2_<core::option::Option<ref_<country_core::components::SemiTransparent>_>_>,enum2_<core::option::Option<ref_<country_core::components::drawable::AlbedoMap>_>_>_>,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData>_>_>
            ();
  puVar36 = (uint *)CONCAT44(uStack_254,local_258);
  local_128 = local_238;
  local_f8 = local_1f8;
  local_f0 = local_1f0;
  plVar34 = (longlong *)*in_RDX;
  lVar7 = *in_R8;
  iVar11 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                            CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                  CONCAT13(unaff_XMM6_Bd,
                           CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
  cVar50 = (char)((uint)iVar11 >> 8);
  cVar51 = (char)((uint)iVar11 >> 0x10);
  cVar52 = (char)((uint)iVar11 >> 0x18);
  iVar12 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                            CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                  CONCAT13(unaff_XMM6_Bh,
                           CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
  cVar53 = (char)((uint)iVar12 >> 8);
  cVar54 = (char)((uint)iVar12 >> 0x10);
  cVar55 = (char)((uint)iVar12 >> 0x18);
  iVar13 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                            CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                  CONCAT13(unaff_XMM6_Bl,
                           CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
  cVar56 = (char)((uint)iVar13 >> 8);
  cVar57 = (char)((uint)iVar13 >> 0x10);
  cVar58 = (char)((uint)iVar13 >> 0x18);
  iVar14 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                            CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                  CONCAT13(unaff_XMM6_Bp,
                           CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
  cVar59 = (char)((uint)iVar14 >> 8);
  cVar60 = (char)((uint)iVar14 >> 0x10);
  cVar61 = (char)((uint)iVar14 >> 0x18);
  local_108 = local_1e8;
  if ((local_1d8 & 1) != 0) goto LAB_142075700;
LAB_142075427:
  if (local_220 != local_228) goto LAB_1420756e0;
  do {
    if (puVar36 == (uint *)CONCAT44(uStack_24c,uStack_250)) goto LAB_142075dbe;
    uVar32 = *puVar36;
    puVar36 = puVar36 + 1;
    local_228 = *(ulonglong *)(*(longlong *)(lVar6 + 0x100) + 0x58 + (ulonglong)uVar32 * 0xa0);
  } while (local_228 == 0);
  lVar33 = *(longlong *)(lVar6 + 0x100) + (ulonglong)uVar32 * 0xa0;
  lVar41 = *(longlong *)(lVar6 + 0x1a8) + (ulonglong)*(uint *)(lVar33 + 0x94) * 0x48;
  uVar35 = *(ulonglong *)
            (*(longlong *)(lVar6 + 0x1a8) + 0x40 + (ulonglong)*(uint *)(lVar33 + 0x94) * 0x48);
  if ((*(ulonglong *)(lVar5 + 0x110) < uVar35) &&
     (*(longlong *)(*(longlong *)(lVar41 + 0x38) + *(ulonglong *)(lVar5 + 0x110) * 8) != 0)) {
    uVar40 = *(ulonglong *)(lVar5 + 0x118);
    if (uVar35 <= uVar40) goto LAB_14207551a;
LAB_1420754e4:
    if (*(longlong *)(*(longlong *)(lVar41 + 0x38) + uVar40 * 8) == 0) goto LAB_14207551a;
    uVar40 = *(ulonglong *)(lVar5 + 0x120);
    if (uVar40 < uVar35) goto LAB_142075530;
LAB_14207556d:
    local_f8 = 0;
    uVar40 = *(ulonglong *)(lVar5 + 0x128);
    if (uVar35 <= uVar40) goto LAB_1420755b5;
LAB_14207558b:
    uVar40 = *(ulonglong *)(*(longlong *)(lVar41 + 0x38) + uVar40 * 8);
    if (uVar40 == 0) goto LAB_1420755b5;
    local_f0 = *(longlong *)(*(longlong *)(lVar41 + 0x18) + 0x10 + ~uVar40 * 0x30);
  }
  else {
    uVar40 = *(ulonglong *)(lVar5 + 0x118);
    if (uVar40 < uVar35) goto LAB_1420754e4;
LAB_14207551a:
    uVar40 = *(ulonglong *)(lVar5 + 0x120);
    if (uVar35 <= uVar40) goto LAB_14207556d;
LAB_142075530:
    uVar40 = *(ulonglong *)(*(longlong *)(lVar41 + 0x38) + uVar40 * 8);
    if (uVar40 == 0) goto LAB_14207556d;
    local_f8 = *(longlong *)(*(longlong *)(lVar41 + 0x18) + 0x10 + ~uVar40 * 0x30);
    uVar40 = *(ulonglong *)(lVar5 + 0x128);
    if (uVar40 < uVar35) goto LAB_14207558b;
LAB_1420755b5:
    local_f0 = 0;
  }
  uVar40 = *(ulonglong *)(lVar5 + 0x130);
  uVar8 = *(ulonglong *)(lVar33 + 0x88);
  if ((uVar40 < uVar8) && (*(longlong *)(*(longlong *)(lVar33 + 0x80) + uVar40 * 8) != 0)) {
    if ((uVar40 < uVar35) && (*(longlong *)(*(longlong *)(lVar41 + 0x38) + uVar40 * 8) != 0)) {
      uVar40 = *(ulonglong *)(lVar5 + 0x138);
      goto joined_r0x000142075652;
    }
    uVar40 = *(ulonglong *)(lVar5 + 0x138);
    if (uVar40 < uVar8) goto LAB_142075674;
LAB_1420756bb:
    local_1e0 = 0;
  }
  else {
    uVar40 = *(ulonglong *)(lVar5 + 0x138);
joined_r0x000142075652:
    if (uVar8 <= uVar40) goto LAB_1420756bb;
LAB_142075674:
    if (*(longlong *)(*(longlong *)(lVar33 + 0x80) + uVar40 * 8) == 0) goto LAB_1420756bb;
    local_1e0 = 1;
    if ((uVar40 < uVar35) &&
       (uVar35 = *(ulonglong *)(*(longlong *)(lVar41 + 0x38) + uVar40 * 8), uVar35 != 0)) {
      local_108 = *(longlong *)(*(longlong *)(lVar41 + 0x18) + 0x10 + ~uVar35 * 0x30);
    }
    else {
      local_108 = 0;
    }
  }
  local_128 = *(longlong *)(lVar33 + 0x50);
  local_220 = 0;
LAB_1420756e0:
  uVar35 = (ulonglong)*(uint *)(local_128 + 8 + local_220 * 0x10);
  local_130 = local_220;
  do {
    puVar39 = (ulonglong *)(local_108 + uVar35 * 8);
    if ((local_1e0 & 1) == 0) {
      puVar39 = (ulonglong *)0x0;
    }
    if (plVar34[3] != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      bVar42 = (byte)(extraout_RAX_03 >> 0x39);
      auVar44 = ZEXT216(CONCAT11(bVar42,bVar42));
      auVar44 = pshuflw(auVar44,auVar44,0);
      lVar41 = 0;
      uVar40 = extraout_RAX_03;
      while( true ) {
        uVar40 = plVar34[1] & uVar40;
        auVar62 = *(undefined1 (*) [16])(*plVar34 + uVar40);
        cVar43 = auVar44[0];
        auVar49[0] = -(auVar62[0] == cVar43);
        cVar45 = auVar44[1];
        auVar49[1] = -(auVar62[1] == cVar45);
        cVar46 = auVar44[2];
        auVar49[2] = -(auVar62[2] == cVar46);
        cVar47 = auVar44[3];
        auVar49[3] = -(auVar62[3] == cVar47);
        auVar49[4] = -(auVar62[4] == cVar43);
        auVar49[5] = -(auVar62[5] == cVar45);
        auVar49[6] = -(auVar62[6] == cVar46);
        auVar49[7] = -(auVar62[7] == cVar47);
        auVar49[8] = -(auVar62[8] == cVar43);
        auVar49[9] = -(auVar62[9] == cVar45);
        auVar49[10] = -(auVar62[10] == cVar46);
        auVar49[0xb] = -(auVar62[0xb] == cVar47);
        auVar49[0xc] = -(auVar62[0xc] == cVar43);
        auVar49[0xd] = -(auVar62[0xd] == cVar45);
        auVar49[0xe] = -(auVar62[0xe] == cVar46);
        auVar49[0xf] = -(auVar62[0xf] == cVar47);
        uVar37 = (ushort)(SUB161(auVar49 >> 7,0) & 1) | (ushort)(SUB161(auVar49 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar49 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar49 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar49 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar49 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar49 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar49 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar49 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar49 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar49 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar49 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar49 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar49 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar49 >> 0x77,0) & 1) << 0xe | (ushort)(auVar49[0xf] >> 7) << 0xf
        ;
        uVar32 = (uint)uVar37;
        if (uVar37 != 0) {
          do {
            uVar9 = 0;
            for (uVar10 = uVar32; (uVar10 & 1) == 0; uVar10 = uVar10 >> 1 | 0x80000000) {
              uVar9 = uVar9 + 1;
            }
            if (*(longlong *)(uVar35 * 0x10 + local_f8) ==
                *(longlong *)(*plVar34 + -0x18 + (uVar9 + uVar40 & plVar34[1]) * -0x18))
            goto LAB_142075a91;
            uVar37 = (ushort)(uVar32 - 1) & (ushort)uVar32;
            uVar32 = CONCAT22((short)(uVar32 - 1 >> 0x10),uVar37);
          } while (uVar37 != 0);
        }
        auVar48[0] = -(auVar62[0] == (char)iVar11);
        auVar48[1] = -(auVar62[1] == cVar50);
        auVar48[2] = -(auVar62[2] == cVar51);
        auVar48[3] = -(auVar62[3] == cVar52);
        auVar48[4] = -(auVar62[4] == (char)iVar12);
        auVar48[5] = -(auVar62[5] == cVar53);
        auVar48[6] = -(auVar62[6] == cVar54);
        auVar48[7] = -(auVar62[7] == cVar55);
        auVar48[8] = -(auVar62[8] == (char)iVar13);
        auVar48[9] = -(auVar62[9] == cVar56);
        auVar48[10] = -(auVar62[10] == cVar57);
        auVar48[0xb] = -(auVar62[0xb] == cVar58);
        auVar48[0xc] = -(auVar62[0xc] == (char)iVar14);
        auVar48[0xd] = -(auVar62[0xd] == cVar59);
        auVar48[0xe] = -(auVar62[0xe] == cVar60);
        auVar48[0xf] = -(auVar62[0xf] == cVar61);
        if ((((((((((((((((SUB161(auVar48 >> 7,0) & 1) != 0 || (SUB161(auVar48 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar48 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar48 >> 0x1f,0) & 1) != 0) || (SUB161(auVar48 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar48 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar48 >> 0x37,0) & 1) != 0) || (SUB161(auVar48 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar48 >> 0x47,0) & 1) != 0) || (SUB161(auVar48 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar48 >> 0x57,0) & 1) != 0) || (SUB161(auVar48 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar48 >> 0x67,0) & 1) != 0) || (SUB161(auVar48 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar48 >> 0x77,0) & 1) != 0) || auVar48[0xf] < '\0') break;
        uVar40 = uVar40 + lVar41 + 0x10;
        lVar41 = lVar41 + 0x10;
      }
    }
LAB_142075a91:
    if (puVar39 != (ulonglong *)0x0) {
      uVar40 = *puVar39;
      if (*(longlong *)(lVar7 + 0x48) != 0) {
        lVar41 = *(longlong *)(lVar7 + 0x30);
        uVar8 = *(ulonglong *)(lVar7 + 0x38);
        bVar42 = (byte)(uVar40 >> 0x39);
        auVar44 = ZEXT216(CONCAT11(bVar42,bVar42));
        auVar44 = pshuflw(auVar44,auVar44,0);
        cVar43 = auVar44[0];
        cVar45 = auVar44[1];
        cVar46 = auVar44[2];
        cVar47 = auVar44[3];
        lVar33 = 0;
        while( true ) {
          uVar40 = uVar8 & uVar40;
          pcVar2 = (char *)(lVar41 + uVar40);
          cVar15 = *pcVar2;
          cVar16 = pcVar2[1];
          cVar17 = pcVar2[2];
          cVar18 = pcVar2[3];
          cVar19 = pcVar2[4];
          cVar20 = pcVar2[5];
          cVar21 = pcVar2[6];
          cVar22 = pcVar2[7];
          cVar23 = pcVar2[8];
          cVar24 = pcVar2[9];
          cVar25 = pcVar2[10];
          cVar26 = pcVar2[0xb];
          cVar27 = pcVar2[0xc];
          cVar28 = pcVar2[0xd];
          cVar29 = pcVar2[0xe];
          cVar30 = pcVar2[0xf];
          auVar44[0] = -(cVar15 == cVar43);
          auVar44[1] = -(cVar16 == cVar45);
          auVar44[2] = -(cVar17 == cVar46);
          auVar44[3] = -(cVar18 == cVar47);
          auVar44[4] = -(cVar19 == cVar43);
          auVar44[5] = -(cVar20 == cVar45);
          auVar44[6] = -(cVar21 == cVar46);
          auVar44[7] = -(cVar22 == cVar47);
          auVar44[8] = -(cVar23 == cVar43);
          auVar44[9] = -(cVar24 == cVar45);
          auVar44[10] = -(cVar25 == cVar46);
          auVar44[0xb] = -(cVar26 == cVar47);
          auVar44[0xc] = -(cVar27 == cVar43);
          auVar44[0xd] = -(cVar28 == cVar45);
          auVar44[0xe] = -(cVar29 == cVar46);
          auVar44[0xf] = -(cVar30 == cVar47);
          uVar37 = (ushort)(SUB161(auVar44 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar44 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar44 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar44 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar44 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar44 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar44 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar44 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar44 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar44 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar44 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar44 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar44 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar44 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar44 >> 0x77,0) & 1) << 0xe |
                   (ushort)(auVar44[0xf] >> 7) << 0xf;
          uVar32 = (uint)uVar37;
          while (uVar9 = uVar32, uVar37 != 0) {
            for (; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
            }
            ::utils::name_hash::impl$2::eq();
            if (extraout_AL_00 != '\0') {
              asset_libraries::texture_library::AssetTextureLibrary::handle_to_bindless_index();
              goto LAB_142075bb2;
            }
            uVar37 = (ushort)(uVar32 - 1) & (ushort)uVar32;
            uVar32 = CONCAT22((short)(uVar32 - 1 >> 0x10),uVar37);
          }
          auVar62[0] = -(cVar15 == (char)iVar11);
          auVar62[1] = -(cVar16 == cVar50);
          auVar62[2] = -(cVar17 == cVar51);
          auVar62[3] = -(cVar18 == cVar52);
          auVar62[4] = -(cVar19 == (char)iVar12);
          auVar62[5] = -(cVar20 == cVar53);
          auVar62[6] = -(cVar21 == cVar54);
          auVar62[7] = -(cVar22 == cVar55);
          auVar62[8] = -(cVar23 == (char)iVar13);
          auVar62[9] = -(cVar24 == cVar56);
          auVar62[10] = -(cVar25 == cVar57);
          auVar62[0xb] = -(cVar26 == cVar58);
          auVar62[0xc] = -(cVar27 == (char)iVar14);
          auVar62[0xd] = -(cVar28 == cVar59);
          auVar62[0xe] = -(cVar29 == cVar60);
          auVar62[0xf] = -(cVar30 == cVar61);
          if ((((((((((((((((SUB161(auVar62 >> 7,0) & 1) != 0 || (SUB161(auVar62 >> 0xf,0) & 1) != 0
                           ) || (SUB161(auVar62 >> 0x17,0) & 1) != 0) ||
                         (SUB161(auVar62 >> 0x1f,0) & 1) != 0) ||
                        (SUB161(auVar62 >> 0x27,0) & 1) != 0) ||
                       (SUB161(auVar62 >> 0x2f,0) & 1) != 0) || (SUB161(auVar62 >> 0x37,0) & 1) != 0
                      ) || (SUB161(auVar62 >> 0x3f,0) & 1) != 0) ||
                    (SUB161(auVar62 >> 0x47,0) & 1) != 0) || (SUB161(auVar62 >> 0x4f,0) & 1) != 0)
                  || (SUB161(auVar62 >> 0x57,0) & 1) != 0) || (SUB161(auVar62 >> 0x5f,0) & 1) != 0)
                || (SUB161(auVar62 >> 0x67,0) & 1) != 0) || (SUB161(auVar62 >> 0x6f,0) & 1) != 0) ||
              (SUB161(auVar62 >> 0x77,0) & 1) != 0) || auVar62[0xf] < '\0') break;
          uVar40 = uVar40 + lVar33 + 0x10;
          lVar33 = lVar33 + 0x10;
        }
      }
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
LAB_142075bb2:
    if (*(int *)(local_f0 + uVar35 * 4) < 0) {
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    gpu_world::OwnedGpuWorldSubset::
    update_instance_data<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData>
              ();
    local_220 = local_130 + 1;
    if ((local_1d8 & 1) == 0) goto LAB_142075427;
LAB_142075700:
    if (local_220 == local_228) {
      do {
        if (puVar36 == (uint *)CONCAT44(uStack_24c,uStack_250)) goto LAB_142075dbe;
        uVar32 = *puVar36;
        puVar36 = puVar36 + 1;
        local_228 = *(ulonglong *)(*(longlong *)(lVar6 + 0x1a8) + 0x10 + (ulonglong)uVar32 * 0x48);
      } while (local_228 == 0);
      lVar41 = *(longlong *)(lVar6 + 0x1a8) + (ulonglong)uVar32 * 0x48;
      uVar35 = *(ulonglong *)(lVar41 + 0x40);
      if ((*(ulonglong *)(lVar5 + 0x110) < uVar35) &&
         (*(longlong *)(*(longlong *)(lVar41 + 0x38) + *(ulonglong *)(lVar5 + 0x110) * 8) != 0)) {
        uVar40 = *(ulonglong *)(lVar5 + 0x118);
        if (uVar35 <= uVar40) goto LAB_1420757d1;
LAB_14207579b:
        if (*(longlong *)(*(longlong *)(lVar41 + 0x38) + uVar40 * 8) == 0) goto LAB_1420757d1;
        uVar40 = *(ulonglong *)(lVar5 + 0x120);
        if (uVar40 < uVar35) goto LAB_1420757e7;
LAB_142075824:
        local_f8 = 0;
        uVar40 = *(ulonglong *)(lVar5 + 0x128);
        if (uVar35 <= uVar40) goto LAB_14207587f;
LAB_142075842:
        uVar40 = *(ulonglong *)(*(longlong *)(lVar41 + 0x38) + uVar40 * 8);
        if (uVar40 == 0) goto LAB_14207587f;
        local_f0 = *(longlong *)(*(longlong *)(lVar41 + 0x18) + 0x10 + ~uVar40 * 0x30);
        uVar40 = *(ulonglong *)(lVar5 + 0x130);
        if (uVar40 < uVar35) goto LAB_14207589d;
LAB_1420758e2:
        uVar40 = *(ulonglong *)(lVar5 + 0x138);
      }
      else {
        uVar40 = *(ulonglong *)(lVar5 + 0x118);
        if (uVar40 < uVar35) goto LAB_14207579b;
LAB_1420757d1:
        uVar40 = *(ulonglong *)(lVar5 + 0x120);
        if (uVar35 <= uVar40) goto LAB_142075824;
LAB_1420757e7:
        uVar40 = *(ulonglong *)(*(longlong *)(lVar41 + 0x38) + uVar40 * 8);
        if (uVar40 == 0) goto LAB_142075824;
        local_f8 = *(longlong *)(*(longlong *)(lVar41 + 0x18) + 0x10 + ~uVar40 * 0x30);
        uVar40 = *(ulonglong *)(lVar5 + 0x128);
        if (uVar40 < uVar35) goto LAB_142075842;
LAB_14207587f:
        local_f0 = 0;
        uVar40 = *(ulonglong *)(lVar5 + 0x130);
        if (uVar35 <= uVar40) goto LAB_1420758e2;
LAB_14207589d:
        if (*(longlong *)(*(longlong *)(lVar41 + 0x38) + uVar40 * 8) == 0) goto LAB_1420758e2;
        uVar40 = *(ulonglong *)(lVar5 + 0x138);
      }
      if ((uVar40 < uVar35) &&
         (uVar35 = *(ulonglong *)(*(longlong *)(lVar41 + 0x38) + uVar40 * 8), uVar35 != 0)) {
        local_108 = *(longlong *)(*(longlong *)(lVar41 + 0x18) + 0x10 + ~uVar35 * 0x30);
        local_1e0 = 1;
        local_220 = 0;
      }
      else {
        local_220 = 0;
        local_1e0 = 0;
      }
    }
    uVar35 = local_220 & 0xffffffff;
    local_130 = local_220;
  } while( true );
LAB_142075dbe:
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  LOCK();
  lVar5 = *extraout_RAX_02;
  if (lVar5 == 8) {
    *extraout_RAX_02 = 0;
  }
  UNLOCK();
  if (lVar5 != 8) {
    parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow();
  }
  if (bVar31) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_04 == 1) {
      plVar34 = extraout_RAX_04 + 1;
    }
    else if ((*extraout_RAX_04 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar34 = extraout_RAX_06, extraout_RAX_06 == (longlong *)0x0)) {
LAB_142075ed5:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar34 != 0) {
LAB_142075ee1:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar34 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar34 = *plVar34 + 1;
  }
  return;
}

