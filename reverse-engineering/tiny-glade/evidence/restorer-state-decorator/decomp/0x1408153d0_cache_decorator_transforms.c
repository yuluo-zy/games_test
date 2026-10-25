
void __cdecl system_decorator::cache_decorator_transforms::cache_decorator_transforms(void)

{
  byte *pbVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  byte bVar4;
  byte bVar5;
  ulonglong uVar6;
  ushort uVar7;
  uint uVar8;
  undefined1 *puVar9;
  code *pcVar10;
  float *pfVar11;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  byte bVar12;
  char extraout_AL_02;
  char extraout_AL_03;
  uint uVar13;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong lVar14;
  ulonglong extraout_RAX_02;
  longlong extraout_RAX_03;
  ulonglong extraout_RAX_04;
  float *extraout_RAX_05;
  longlong extraout_RAX_06;
  undefined8 *extraout_RAX_07;
  longlong extraout_RAX_08;
  float *extraout_RAX_09;
  longlong extraout_RAX_10;
  undefined8 *extraout_RAX_11;
  float *extraout_RAX_12;
  longlong extraout_RAX_13;
  float *pfVar15;
  longlong extraout_RAX_14;
  undefined8 *extraout_RAX_15;
  longlong extraout_RAX_16;
  undefined8 *extraout_RAX_17;
  ulonglong extraout_RAX_18;
  ulonglong uVar16;
  undefined8 *extraout_RAX_19;
  longlong *extraout_RAX_20;
  longlong *extraout_RAX_21;
  longlong *extraout_RAX_22;
  undefined8 *in_RCX;
  longlong lVar17;
  float *pfVar18;
  ulonglong uVar19;
  longlong *plVar20;
  undefined8 *in_RDX;
  undefined8 *puVar21;
  undefined1 (*pauVar22) [16];
  ulonglong uVar23;
  ulonglong uVar24;
  undefined *puVar25;
  longlong *in_R9;
  longlong lVar26;
  longlong lVar27;
  undefined1 (*pauVar28) [16];
  longlong lVar29;
  ulonglong uVar30;
  undefined1 (*pauVar31) [16];
  longlong *plVar32;
  float extraout_XMM0_Da;
  float fVar33;
  float fVar34;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  float extraout_XMM0_Da_04;
  undefined4 extraout_XMM0_Da_05;
  uint uVar35;
  float fVar36;
  float fVar37;
  int iVar39;
  undefined1 auVar38 [16];
  float fVar40;
  float fVar41;
  float fVar42;
  float fVar43;
  float fVar44;
  float unaff_XMM11_Da;
  float fVar45;
  float unaff_XMM11_Db;
  float unaff_XMM11_Dc;
  float unaff_XMM11_Dd;
  float fVar46;
  longlong *in_stack_00000028;
  longlong *in_stack_00000030;
  longlong *in_stack_00000038;
  undefined8 *in_stack_00000040;
  undefined8 *in_stack_00000048;
  ulonglong uStack_2d0;
  undefined8 local_280;
  undefined1 (*local_278) [16];
  undefined1 *puStack_270;
  ushort local_268;
  undefined6 uStack_266;
  undefined8 local_260;
  longlong local_258;
  longlong local_250;
  undefined1 (*local_248) [16];
  longlong local_240;
  longlong local_238;
  longlong local_230;
  undefined8 *local_228;
  undefined8 local_220;
  undefined1 (*local_218) [16];
  undefined1 *puStack_210;
  undefined8 local_208;
  undefined8 local_1f8;
  longlong local_1f0;
  longlong local_1e8;
  longlong local_1d8;
  uint local_1cc;
  undefined4 *local_1c8;
  undefined8 local_1c0;
  undefined8 local_1b8;
  undefined8 uStack_1b0;
  undefined1 *local_1a8;
  undefined4 uStack_1a0;
  float fStack_19c;
  undefined4 uStack_198;
  undefined4 uStack_194;
  ulonglong uStack_190;
  undefined4 *local_180;
  undefined8 local_178;
  undefined1 *local_170;
  undefined1 (*local_168) [16];
  undefined1 *puStack_160;
  undefined8 local_158;
  undefined8 local_150;
  float local_148;
  undefined8 local_140;
  code *local_138;
  undefined4 local_130;
  char local_129;
  float local_128;
  float fStack_124;
  undefined8 uStack_120;
  undefined8 local_118;
  ushort uStack_110;
  undefined2 uStack_10e;
  float fStack_10c;
  undefined4 uStack_108;
  undefined4 uStack_104;
  ulonglong uStack_100;
  undefined8 local_f8;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  puVar21 = in_RDX;
  local_228 = in_RCX;
  puffin::are_scopes_on();
  local_129 = extraout_AL;
  if (extraout_AL != '\0') {
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (puVar21 != (undefined8 *)0x0) {
        if (&DAT_00000047 < puVar21) {
          if (puVar21 != (undefined8 *)&DAT_00000048) goto LAB_1408154a5;
        }
        else if (*(char *)((longlong)puVar21 + 0x142a8faeb) < -0x40) {
LAB_1408154a5:
                    // WARNING: Subroutine does not return
          core::str::slice_error_fail();
        }
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (puVar21 != (undefined8 *)0xfffffffffffffffe)) {
        if ((undefined1 *)((longlong)puVar21 + 2) < (undefined1 *)0x48) {
          if (*(char *)((longlong)puVar21 + 0x142a8faed) < -0x40) {
LAB_1408154f9:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if ((undefined1 *)((longlong)puVar21 + 2) != (undefined1 *)0x48) goto LAB_1408154f9;
      }
    }
    puVar25 = &DAT_142a8fd02;
    do {
      bVar12 = puVar25[-1];
      uVar13 = (uint)(char)bVar12;
      if ((char)bVar12 < '\0') {
        bVar4 = puVar25[-2];
        if ((char)bVar4 < -0x40) {
          bVar5 = puVar25[-3];
          if ((char)bVar5 < -0x40) {
            pbVar1 = puVar25 + -4;
            puVar25 = puVar25 + -4;
            uVar13 = bVar5 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar25 = puVar25 + -3;
            uVar13 = bVar5 & 0xf;
          }
          uVar13 = bVar4 & 0x3f | uVar13 << 6;
        }
        else {
          puVar25 = puVar25 + -2;
          uVar13 = bVar4 & 0x1f;
        }
        uVar13 = (uint)(bVar12 & 0x3f) | uVar13 << 6;
        if (uVar13 != 0x5c) goto LAB_1408155b4;
LAB_1408155c6:
        if (puVar25 == &UNK_142a8fcc7) break;
        if (puVar25 + -0x142a8fcc7 < (undefined *)0x3a) {
          if (-0x41 < (char)puVar25[1]) break;
        }
        else if (puVar25 + -0x142a8fcc7 == (undefined *)0x3a) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar25 = puVar25 + -1;
      if (uVar13 == 0x5c) goto LAB_1408155c6;
LAB_1408155b4:
      if (uVar13 == 0x2f) goto LAB_1408155c6;
    } while (puVar25 != &DAT_142a8fcc8);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar20 = extraout_RAX_01 + 1;
    }
    else {
      if (*extraout_RAX_01 != 0) goto LAB_140817284;
      std::sys::thread_local::native::lazy::Storage::
      initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                ();
      plVar20 = extraout_RAX_21;
      if (extraout_RAX_21 == (longlong *)0x0) goto LAB_140817284;
    }
    if (*plVar20 != 0) goto LAB_140817417;
    *plVar20 = -1;
    local_f8 = plVar20;
    puffin::ThreadProfiler::begin_scope();
    *local_f8 = *local_f8 + 1;
  }
  local_130 = *(undefined4 *)((longlong)in_RDX + 0x1c);
  local_1c8 = (undefined4 *)in_RDX[2];
  *local_1c8 = local_130;
  local_1c0 = *in_RDX;
  country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::
  swap_buffers_and_clear();
  local_220 = *local_228;
  country_core::resources::walls::decorator_storage::Decorators::iter_removed_ids();
  lVar27 = CONCAT44(fStack_124,local_128);
  lVar29 = CONCAT44(uStack_104,uStack_108);
  local_1f0 = *in_R9;
  local_238 = *in_stack_00000038;
  local_240 = local_238 + 0xa8;
  local_1e8 = *in_stack_00000030;
  local_250 = in_stack_00000030[1];
  local_230 = local_238 + 0xf0;
  uVar13 = (uint)uStack_110;
  pauVar28 = (undefined1 (*) [16])CONCAT44(uStack_120._4_4_,(float)uStack_120);
  while (lVar29 != 0) {
    lVar29 = lVar29 + -1;
    if ((short)uVar13 == 0) {
      do {
        auVar38 = *pauVar28;
        uVar7 = (ushort)(SUB161(auVar38 >> 7,0) & 1) | (ushort)(SUB161(auVar38 >> 0xf,0) & 1) << 1 |
                (ushort)(SUB161(auVar38 >> 0x17,0) & 1) << 2 |
                (ushort)(SUB161(auVar38 >> 0x1f,0) & 1) << 3 |
                (ushort)(SUB161(auVar38 >> 0x27,0) & 1) << 4 |
                (ushort)(SUB161(auVar38 >> 0x2f,0) & 1) << 5 |
                (ushort)(SUB161(auVar38 >> 0x37,0) & 1) << 6 |
                (ushort)(SUB161(auVar38 >> 0x3f,0) & 1) << 7 |
                (ushort)(SUB161(auVar38 >> 0x47,0) & 1) << 8 |
                (ushort)(SUB161(auVar38 >> 0x4f,0) & 1) << 9 |
                (ushort)(SUB161(auVar38 >> 0x57,0) & 1) << 10 |
                (ushort)(SUB161(auVar38 >> 0x5f,0) & 1) << 0xb |
                (ushort)(SUB161(auVar38 >> 0x67,0) & 1) << 0xc |
                (ushort)(SUB161(auVar38 >> 0x6f,0) & 1) << 0xd |
                (ushort)(SUB161(auVar38 >> 0x77,0) & 1) << 0xe |
                (ushort)(byte)(auVar38[0xf] >> 7) << 0xf;
        lVar27 = lVar27 + -0x980;
        pauVar28 = pauVar28 + 1;
      } while (uVar7 == 0xffff);
      uVar13 = ~(uint)uVar7;
      local_1cc = uVar13 & -(uint)uVar7 - 2;
      local_248 = pauVar28;
    }
    else {
      if (lVar27 == 0) break;
      local_1cc = uVar13 - 1 & uVar13;
      local_248 = pauVar28;
    }
    uVar35 = 0;
    for (; (uVar13 & 1) == 0; uVar13 = uVar13 >> 1 | 0x80000000) {
      uVar35 = uVar35 + 1;
    }
    lVar17 = (ulonglong)uVar35 * -0x98;
    local_180 = *(undefined4 **)(lVar27 + -0x98 + lVar17);
    pauVar22 = *(undefined1 (**) [16])(lVar27 + -0x60 + lVar17);
    auVar38 = *pauVar22;
    pauVar31 = pauVar22 + 1;
    lVar17 = *(longlong *)(lVar27 + -0x48 + lVar17);
    uVar35 = ~(uint)(ushort)((ushort)(SUB161(auVar38 >> 7,0) & 1) |
                             (ushort)(SUB161(auVar38 >> 0xf,0) & 1) << 1 |
                             (ushort)(SUB161(auVar38 >> 0x17,0) & 1) << 2 |
                             (ushort)(SUB161(auVar38 >> 0x1f,0) & 1) << 3 |
                             (ushort)(SUB161(auVar38 >> 0x27,0) & 1) << 4 |
                             (ushort)(SUB161(auVar38 >> 0x2f,0) & 1) << 5 |
                             (ushort)(SUB161(auVar38 >> 0x37,0) & 1) << 6 |
                             (ushort)(SUB161(auVar38 >> 0x3f,0) & 1) << 7 |
                             (ushort)(SUB161(auVar38 >> 0x47,0) & 1) << 8 |
                             (ushort)(SUB161(auVar38 >> 0x4f,0) & 1) << 9 |
                             (ushort)(SUB161(auVar38 >> 0x57,0) & 1) << 10 |
                             (ushort)(SUB161(auVar38 >> 0x5f,0) & 1) << 0xb |
                             (ushort)(SUB161(auVar38 >> 0x67,0) & 1) << 0xc |
                             (ushort)(SUB161(auVar38 >> 0x6f,0) & 1) << 0xd |
                             (ushort)(SUB161(auVar38 >> 0x77,0) & 1) << 0xe |
                            (ushort)(byte)(auVar38[0xf] >> 7) << 0xf);
    while (uVar13 = local_1cc, pauVar28 = local_248, lVar17 != 0) {
      lVar17 = lVar17 + -1;
      if ((short)uVar35 == 0) {
        do {
          auVar38 = *pauVar31;
          uVar7 = (ushort)(SUB161(auVar38 >> 7,0) & 1) | (ushort)(SUB161(auVar38 >> 0xf,0) & 1) << 1
                  | (ushort)(SUB161(auVar38 >> 0x17,0) & 1) << 2 |
                  (ushort)(SUB161(auVar38 >> 0x1f,0) & 1) << 3 |
                  (ushort)(SUB161(auVar38 >> 0x27,0) & 1) << 4 |
                  (ushort)(SUB161(auVar38 >> 0x2f,0) & 1) << 5 |
                  (ushort)(SUB161(auVar38 >> 0x37,0) & 1) << 6 |
                  (ushort)(SUB161(auVar38 >> 0x3f,0) & 1) << 7 |
                  (ushort)(SUB161(auVar38 >> 0x47,0) & 1) << 8 |
                  (ushort)(SUB161(auVar38 >> 0x4f,0) & 1) << 9 |
                  (ushort)(SUB161(auVar38 >> 0x57,0) & 1) << 10 |
                  (ushort)(SUB161(auVar38 >> 0x5f,0) & 1) << 0xb |
                  (ushort)(SUB161(auVar38 >> 0x67,0) & 1) << 0xc |
                  (ushort)(SUB161(auVar38 >> 0x6f,0) & 1) << 0xd |
                  (ushort)(SUB161(auVar38 >> 0x77,0) & 1) << 0xe |
                  (ushort)(byte)(auVar38[0xf] >> 7) << 0xf;
          pauVar22 = pauVar22 + -0x50;
          pauVar31 = pauVar31 + 1;
        } while (uVar7 == 0xffff);
        uVar35 = ~(uint)uVar7;
        uVar13 = uVar35 & -(uint)uVar7 - 2;
      }
      else {
        uVar13 = uVar35 - 1 & uVar35;
      }
      uVar8 = 0;
      for (; (uVar35 & 1) == 0; uVar35 = uVar35 >> 1 | 0x80000000) {
        uVar8 = uVar8 + 1;
      }
      uVar16 = (ulonglong)uVar8;
      local_f8._4_4_ = (undefined4)((ulonglong)local_f8 >> 0x20);
      local_f8 = (longlong *)CONCAT44(local_f8._4_4_,*(undefined4 *)pauVar22[uVar16 * -5 + -5]);
      lVar14 = (ulonglong)(byte)pauVar22[uVar16 * -5 + -3][10] - 1;
      if (2 < (byte)(pauVar22[uVar16 * -5 + -3][10] - 2)) {
        lVar14 = 0;
      }
      if (lVar14 == 0) {
        local_140 = *(undefined8 **)(pauVar22[uVar16 * -5 + -5] + 8);
        country_core::resources::walls::public_walls::PublicWalls::get();
        if (extraout_RAX_03 == 0) {
                    // WARNING: Subroutine does not return
          core::option::unwrap_failed();
        }
        country_core::resources::walls::wall_attachment::WallCornerAttachment::
        to_rectangle_corner_id();
        if (((extraout_RAX_04 & 1) == 0) ||
           (country_core::resources::walls::public_wall_state::PublicWallState::
            corner_pos_and_normal_xz(), ((uint)local_128 & 1) == 0)) {
          ::utils::wall_space::WallSpaceCoord::get_tangent();
          ::utils::wall_space::WallSpaceCoord::to_world_space();
          ::utils::xz_rotation_bad();
          fVar44 = (float)local_1b8;
          fVar33 = (float)uStack_1b0;
          fVar43 = local_1b8._4_4_;
        }
        else {
          local_218 = (undefined1 (*) [16])CONCAT44((float)uStack_120,fStack_124);
          ::utils::swizzlers::impl$1::xvy();
          ::utils::xz_rotation_bad();
          fVar44 = (float)local_280;
          fVar33 = local_278._0_4_;
          local_128 = (float)local_1b8;
          fStack_124 = local_1b8._4_4_;
          uStack_120._0_4_ = (float)uStack_1b0;
          uStack_120._4_4_ = uStack_1b0._4_4_;
          fVar43 = local_280._4_4_;
        }
LAB_140815f30:
        local_170 = (undefined1 *)0x3f8000003f800000;
        local_168 = (undefined1 (*) [16])CONCAT44(local_168._4_4_,0x3f800000);
      }
      else {
        if (lVar14 != 1) {
                    // WARNING: Subroutine does not return
          core::panicking::panic();
        }
        fVar34 = *(float *)pauVar22[uVar16 * -5 + -4];
        country_core::resources::walls::public_walls::PublicWalls::roof_entity();
        if (extraout_RAX_02 >> 0x20 != 0) {
          uVar19 = extraout_RAX_02 & 0xffffffff;
          iVar39 = (int)(extraout_RAX_02 >> 0x20);
          if ((uVar19 < *(ulonglong *)(local_250 + 0x10)) &&
             (*(int *)(*(longlong *)(local_250 + 8) + uVar19 * 0x14) == iVar39)) {
            lVar14 = *(longlong *)(local_250 + 8) + uVar19 * 0x14;
            uVar19 = (ulonglong)*(uint *)(lVar14 + 4);
            if (uVar19 != 0xffffffff) {
              if ((*(ulonglong *)(local_1e8 + 0x38) <= uVar19) ||
                 ((*(ulonglong *)
                    (*(longlong *)(local_1e8 + 0x28) + (ulonglong)(*(uint *)(lVar14 + 4) >> 6) * 8)
                   >> (uVar19 & 0x3f) & 1) == 0)) {
                lVar27 = uVar19 << 0x20;
                goto LAB_140817439;
              }
              uVar19 = (ulonglong)*(uint *)(lVar14 + 0xc);
              lVar26 = *(longlong *)(local_250 + 0x1a8);
              if (*(ulonglong *)(local_1e8 + 0x110) < *(ulonglong *)(lVar26 + 0x40 + uVar19 * 0x48))
              {
                uVar24 = *(ulonglong *)
                          (*(longlong *)(lVar26 + 0x38 + uVar19 * 0x48) +
                          *(ulonglong *)(local_1e8 + 0x110) * 8);
                if (uVar24 == 0) goto LAB_140815aeb;
                lVar26 = *(longlong *)
                          (*(longlong *)(lVar26 + uVar19 * 0x48 + 0x18) + 0x10 + ~uVar24 * 0x30);
              }
              else {
LAB_140815aeb:
                lVar26 = 0;
              }
              uVar35 = *(uint *)(lVar14 + 0x10);
              country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume::
              get_position_from_mesh_uv();
              fVar33 = uStack_120._4_4_;
              fVar43 = (float)uStack_120;
              fVar44 = fStack_124;
              if (local_128 == 1.4013e-45) {
                local_1d8 = lVar26 + (ulonglong)uVar35 * 0x58;
                fVar42 = 0.0;
                fVar40 = 0.0;
                fVar41 = 0.0;
                fVar45 = 1.0;
                if ((~pauVar22[uVar16 * -5 + -1][0xe] & 0x3e) == 0 ||
                    (~pauVar22[uVar16 * -5 + -1][0xe] & 0x38) != 0) {
                  country_core::resources::roofs::roof_shape::Roof::rotation_yaw_radians();
                  local_1b8 = (undefined1 (*) [16])
                              CONCAT44(local_1b8._4_4_,fVar34 + extraout_XMM0_Da);
                  country_core::resources::walls::decorator::DecoratorRotationXZ::to_quat();
                  fVar42 = local_128;
                  fVar40 = fStack_124;
                  fVar41 = (float)uStack_120;
                  fVar45 = uStack_120._4_4_;
                }
                country_core::resources::roofs::roof_shape::Roof::is_flat();
                fVar34 = fVar42;
                fVar36 = fVar40;
                fVar37 = fVar41;
                if ((extraout_AL_00 == '\0') ||
                   (bVar12 = pauVar22[uVar16 * -5 + -1][0xe] - 0x38, bVar12 < 8 && bVar12 != 6)) {
                  if (pauVar22[uVar16 * -5 + -1][0xe] == '?') {
                    local_128 = (float)local_f8;
                    indexmap::map::IndexMap::
                    get<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<tuple_<country_core::resources::walls::decorator::DecoratorId,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,std::hash::random::RandomState,country_core::resources::walls::decorator::DecoratorId>
                              ();
                    if (extraout_RAX_06 == 0) {
                      pfVar15 = (float *)&DAT_00000004;
                      pfVar11 = (float *)&DAT_00000004;
                    }
                    else {
                      pfVar15 = *(float **)(extraout_RAX_06 + 8) +
                                *(longlong *)(extraout_RAX_06 + 0x10) * 2;
                      pfVar11 = *(float **)(extraout_RAX_06 + 8);
                    }
                    do {
                      pfVar18 = pfVar11;
                      if (pfVar18 == pfVar15) goto LAB_140815eed;
                      pfVar11 = pfVar18 + 2;
                    } while (*(char *)(pfVar18 + 1) != '\0');
                    fStack_124 = *pfVar18;
                    local_128 = (float)local_f8;
                    indexmap::map::IndexMap::
                    get<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,country_core::resources::stairs::stairs_assembly_params::AdjustedStairEdge,std::hash::random::RandomState,tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>_>
                              ();
                    if (extraout_RAX_08 == 0) {
                    // WARNING: Subroutine does not return
                      core::panicking::panic();
                    }
                    ::utils::xz_rotation_bad();
                    fVar34 = local_128;
                    fVar36 = fStack_124;
                    fVar37 = (float)uStack_120;
                    fVar45 = uStack_120._4_4_;
                  }
                  else {
                    if ((*(byte *)(local_1d8 + 8) & 1) != 0) {
                      if (2 < (byte)(pauVar22[uVar16 * -5 + -1][0xc] - 1)) {
                    // WARNING: Subroutine does not return
                        core::panicking::panic();
                      }
                      local_1b8 = (undefined1 (*) [16])CONCAT44(fVar43 + 1.0,fVar44 + 0.0);
                      uStack_1b0 = (undefined1 (*) [16])CONCAT44(uStack_1b0._4_4_,fVar33 + 0.0);
                      country_core::resources::roofs::roof_shape::Roof::
                      get_roof_shape_from_pos_ws_clamped();
                      country_core::systems::wall::valid_enclosure::ShapeParameters::as_rectangle();
                      local_1a8 = (undefined1 *)extraout_RAX_07[2];
                      local_1b8 = (undefined1 (*) [16])*extraout_RAX_07;
                      uStack_1b0 = (undefined1 (*) [16])extraout_RAX_07[1];
                      ::utils::geometry::rectangle::Rectangle2d::closest_edge_normal();
                    }
                    ::utils::xz_rotation_bad();
                    fVar34 = fStack_124 * fVar41 * -1.0 + (float)uStack_120 * fVar40 * 1.0 +
                             fVar42 * uStack_120._4_4_ * 1.0 + fVar45 * local_128;
                    fVar36 = local_128 * fVar41 * 1.0 + uStack_120._4_4_ * fVar40 * 1.0 +
                             fVar42 * (float)uStack_120 * -1.0 + fVar45 * fStack_124;
                    fVar37 = uStack_120._4_4_ * fVar41 * 1.0 + local_128 * fVar40 * -1.0 +
                             fVar42 * fStack_124 * 1.0 + fVar45 * (float)uStack_120;
                    fVar45 = (float)uStack_120 * fVar41 * -1.0 + fStack_124 * fVar40 * -1.0 +
                             fVar42 * local_128 * -1.0 + fVar45 * uStack_120._4_4_;
                  }
                }
LAB_140815eed:
                uStack_120._4_4_ = fVar45;
                uStack_120._0_4_ = fVar37;
                fStack_124 = fVar36;
                local_128 = fVar34;
                country_core::resources::roofs::roof_shape::Roof::is_flat();
                if ((extraout_AL_01 != '\0') && (pauVar22[uVar16 * -5 + -1][0xe] == '?')) {
                  country_core::resources::walls::decorator::DecoratorExtraInfo::as_stair();
                  fVar43 = fVar43 + *extraout_RAX_09;
                }
                goto LAB_140815f30;
              }
              goto LAB_140815c00;
            }
          }
          lVar27 = 1;
LAB_140817439:
          local_128 = (float)lVar27;
          fStack_124 = (float)((ulonglong)lVar27 >> 0x20);
          uStack_120._0_4_ = (float)extraout_RAX_02;
          uStack_120._4_4_ = (float)iVar39;
                    // WARNING: Subroutine does not return
          core::result::unwrap_failed();
        }
LAB_140815c00:
        country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransformStorage
        ::get();
        if (extraout_RAX_05 == (float *)0x0) {
          fVar44 = -100.0;
          fVar43 = -100.0;
          fVar33 = -100.0;
        }
        else {
          unaff_XMM11_Da = *extraout_RAX_05;
          unaff_XMM11_Db = extraout_RAX_05[1];
          unaff_XMM11_Dc = extraout_RAX_05[2];
          unaff_XMM11_Dd = extraout_RAX_05[3];
          fVar44 = extraout_RAX_05[4];
          fVar43 = extraout_RAX_05[5];
          fVar33 = extraout_RAX_05[6];
          local_278 = (undefined1 (*) [16])CONCAT44(local_278._4_4_,extraout_RAX_05[9]);
          local_280 = *(undefined1 (**) [16])(extraout_RAX_05 + 7);
          local_178 = *(ulonglong *)(extraout_RAX_05 + 10);
        }
        ::utils::components::transform::impl$1::default();
        local_1b8 = (undefined1 (*) [16])CONCAT44(uStack_108,fStack_10c);
        uStack_1b0 = (undefined1 (*) [16])CONCAT44(uStack_1b0._4_4_,uStack_104);
        puVar21 = &local_1b8;
        if (extraout_RAX_05 != (float *)0x0) {
          local_128 = unaff_XMM11_Da;
          fStack_124 = unaff_XMM11_Db;
          uStack_120._0_4_ = unaff_XMM11_Dc;
          uStack_120._4_4_ = unaff_XMM11_Dd;
          puVar21 = &local_280;
        }
        local_168 = (undefined1 (*) [16])CONCAT44(local_168._4_4_,*(undefined4 *)(puVar21 + 1));
        local_170 = (undefined1 *)*puVar21;
      }
      *local_1c8 = local_130;
      local_118 = (undefined8 *)CONCAT44(fVar43,fVar44);
      uStack_110 = SUB42(fVar33,0);
      uStack_10e = (undefined2)((uint)fVar33 >> 0x10);
      uStack_104 = local_168._0_4_;
      fStack_10c = SUB84(local_170,0);
      uStack_108 = (undefined4)((ulonglong)local_170 >> 0x20);
      uStack_100 = local_178;
      country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::insert
                ();
      uVar35 = uVar13;
    }
  }
  local_180 = (undefined4 *)local_228[2];
  uVar16 = local_178 >> 0x20;
  local_178 = CONCAT44((int)uVar16,*(undefined4 *)((longlong)local_228 + 0x1c));
  *local_180 = *(undefined4 *)((longlong)local_228 + 0x1c);
  country_core::resources::walls::decorator_storage::DecoratorStorage::get_mut();
  if (extraout_RAX_10 != 0) {
    local_1b8 = *(undefined1 (**) [16])(extraout_RAX_10 + 0x30);
    local_1a8 = *local_1b8 + *(longlong *)(extraout_RAX_10 + 0x38) + 1;
    auVar38 = *local_1b8;
    uStack_1a0._0_2_ =
         ~((ushort)(SUB161(auVar38 >> 7,0) & 1) | (ushort)(SUB161(auVar38 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar38 >> 0x17,0) & 1) << 2 |
           (ushort)(SUB161(auVar38 >> 0x1f,0) & 1) << 3 |
           (ushort)(SUB161(auVar38 >> 0x27,0) & 1) << 4 |
           (ushort)(SUB161(auVar38 >> 0x2f,0) & 1) << 5 |
           (ushort)(SUB161(auVar38 >> 0x37,0) & 1) << 6 |
           (ushort)(SUB161(auVar38 >> 0x3f,0) & 1) << 7 |
           (ushort)(SUB161(auVar38 >> 0x47,0) & 1) << 8 |
           (ushort)(SUB161(auVar38 >> 0x4f,0) & 1) << 9 |
           (ushort)(SUB161(auVar38 >> 0x57,0) & 1) << 10 |
           (ushort)(SUB161(auVar38 >> 0x5f,0) & 1) << 0xb |
           (ushort)(SUB161(auVar38 >> 0x67,0) & 1) << 0xc |
           (ushort)(SUB161(auVar38 >> 0x6f,0) & 1) << 0xd |
           (ushort)(SUB161(auVar38 >> 0x77,0) & 1) << 0xe | (ushort)(byte)(auVar38[0xf] >> 7) << 0xf
          );
    uStack_190 = extraout_RAX_10 + 0x60;
    uStack_1b0 = local_1b8 + 1;
    uStack_198 = (undefined4)*(undefined8 *)(extraout_RAX_10 + 0x48);
    uStack_194 = (undefined4)((ulonglong)*(undefined8 *)(extraout_RAX_10 + 0x48) >> 0x20);
    lVar27 = *in_stack_00000028;
    local_f8 = (longlong *)*in_stack_00000040;
    while( true ) {
      country_core::resources::walls::decorator_storage::impl$3::next();
      if (local_278 == (undefined1 (*) [16])0x0) break;
      fVar43 = (float)local_280;
      local_208 = CONCAT62(uStack_266,local_268);
      local_218 = local_278;
      puStack_210 = puStack_270;
      minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
      if (*(char *)((longlong)extraout_RAX_11 + 0x22) != '\x03') {
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      local_148 = *(float *)(extraout_RAX_11 + 1);
      local_150 = *extraout_RAX_11;
      uStack_120 = (undefined1 (*) [16])(*(longlong *)(lVar27 + 0x10) * 2);
      local_128 = (float)*(undefined8 *)(lVar27 + 8);
      fStack_124 = (float)((ulonglong)*(undefined8 *)(lVar27 + 8) >> 0x20);
      local_118 = *(undefined8 **)(lVar27 + 0x50);
      ::utils::float_grid::impl$0::
      bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid>()
      ;
      ::utils::swizzlers::impl$1::xvy();
      fVar44 = local_148 * 0.5;
      fVar33 = sinf(fVar44);
      fVar34 = cosf(fVar44);
      fVar44 = 0.0;
      pauVar28 = (undefined1 (*) [16])((ulonglong)(uint)fVar34 << 0x20);
      bVar12 = *(char *)((longlong)extraout_RAX_11 + 0x46) - 0x38;
      if (7 < bVar12) {
        bVar12 = 6;
      }
      if (bVar12 == 6) {
        country_core::resources::clutter_meshes::clutter_mesh_ty_data();
        puVar21 = (undefined8 *)CONCAT44(fStack_124,local_128);
        local_118 = (undefined8 *)puVar21[2];
        uVar2 = puVar21[3];
        uStack_110 = (ushort)uVar2;
        uStack_10e = (undefined2)((ulonglong)uVar2 >> 0x10);
        fStack_10c = (float)((ulonglong)uVar2 >> 0x20);
        local_128 = (float)*puVar21;
        fStack_124 = (float)((ulonglong)*puVar21 >> 0x20);
        uStack_120._0_4_ = (float)puVar21[1];
        uStack_120._4_4_ = (float)((ulonglong)puVar21[1] >> 0x20);
        ::utils::geometry::rectangle::Rectangle3d::set_orientation_xz();
        local_168 = (undefined1 (*) [16])CONCAT44(local_168._4_4_,local_138._0_4_);
        local_170 = (undefined1 *)local_140;
        ::utils::geometry::rectangle::Rectangle3d::translate();
        local_170 = *(undefined1 **)(lVar27 + 8);
        local_168 = (undefined1 (*) [16])(*(longlong *)(lVar27 + 0x10) * 2);
        puStack_160 = *(undefined1 **)(lVar27 + 0x50);
        ::utils::float_grid::impl$0::
        bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid>
                  ();
        local_170 = *(undefined1 **)(lVar27 + 8);
        local_168 = (undefined1 (*) [16])(*(longlong *)(lVar27 + 0x10) * 2);
        puStack_160 = *(undefined1 **)(lVar27 + 0x50);
        ::utils::float_grid::impl$0::
        bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid>
                  ();
        local_170 = *(undefined1 **)(lVar27 + 8);
        local_168 = (undefined1 (*) [16])(*(longlong *)(lVar27 + 0x10) * 2);
        puStack_160 = *(undefined1 **)(lVar27 + 0x50);
        ::utils::float_grid::impl$0::
        bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid>
                  ();
        local_170 = *(undefined1 **)(lVar27 + 8);
        local_168 = (undefined1 (*) [16])(*(longlong *)(lVar27 + 0x10) * 2);
        puStack_160 = *(undefined1 **)(lVar27 + 0x50);
        ::utils::float_grid::impl$0::
        bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid>
                  ();
        fVar43 = extraout_XMM0_Da_01;
        if (extraout_XMM0_Da_01 <= extraout_XMM0_Da_00) {
          fVar43 = extraout_XMM0_Da_00;
        }
        fVar34 = extraout_XMM0_Da_01;
        if (extraout_XMM0_Da_00 <= extraout_XMM0_Da_01) {
          fVar34 = extraout_XMM0_Da_00;
        }
        fVar42 = (float)(~-(uint)NAN(extraout_XMM0_Da_00) & (uint)fVar34 |
                        -(uint)NAN(extraout_XMM0_Da_00) & (uint)extraout_XMM0_Da_01);
        fVar34 = extraout_XMM0_Da_02;
        if (fVar42 <= extraout_XMM0_Da_02) {
          fVar34 = fVar42;
        }
        fVar42 = (float)(-(uint)NAN(fVar42) & (uint)extraout_XMM0_Da_02 |
                        ~-(uint)NAN(fVar42) & (uint)fVar34);
        fVar34 = extraout_XMM0_Da_03;
        if (fVar42 <= extraout_XMM0_Da_03) {
          fVar34 = fVar42;
        }
        fVar42 = (float)(-(uint)NAN(fVar42) & (uint)extraout_XMM0_Da_03 |
                        ~-(uint)NAN(fVar42) & (uint)fVar34);
        fVar34 = 0.0;
        if (0.0 <= fVar42 + fVar42) {
          fVar34 = fVar42 + fVar42;
        }
        fVar40 = 1.0;
        if (fVar34 <= 1.0) {
          fVar40 = fVar34;
        }
        fVar34 = 1.0 - (3.0 - (fVar40 + fVar40)) * fVar40 * fVar40;
        fVar42 = (extraout_XMM0_Da_00 + extraout_XMM0_Da_01 + extraout_XMM0_Da_02 +
                 extraout_XMM0_Da_03) * 0.25 * fVar34 + (1.0 - fVar34) * fVar42;
        fVar34 = extraout_XMM0_Da_02;
        if (extraout_XMM0_Da_02 <= fVar43) {
          fVar34 = fVar43;
        }
        fVar34 = (float)(-(uint)NAN(fVar43) & (uint)extraout_XMM0_Da_02 |
                        ~-(uint)NAN(fVar43) & (uint)fVar34);
        fVar43 = extraout_XMM0_Da_03;
        if (extraout_XMM0_Da_03 <= fVar34) {
          fVar43 = fVar34;
        }
        fVar34 = (fStack_10c - (float)CONCAT22(uStack_10e,uStack_110)) * -0.5 +
                 (float)(-(uint)NAN(fVar34) & (uint)extraout_XMM0_Da_03 |
                        ~-(uint)NAN(fVar34) & (uint)fVar43);
        fVar43 = fVar42;
        if (fVar42 <= fVar34) {
          fVar43 = fVar34;
        }
        local_140 = (undefined8 *)
                    CONCAT44(~-(uint)NAN(fVar34) & (uint)fVar43 | -(uint)NAN(fVar34) & (uint)fVar42,
                             (undefined4)local_140);
      }
      else {
        pauVar28 = (undefined1 (*) [16])((ulonglong)(uint)fVar34 << 0x20);
        if (bVar12 == 7) {
          country_core::resources::walls::decorator::DecoratorExtraInfo::as_stair();
          local_140 = (undefined8 *)
                      CONCAT44(*extraout_RAX_12 + local_140._4_4_,(undefined4)local_140);
          local_128 = fVar43;
          indexmap::map::IndexMap::
          get<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<tuple_<country_core::resources::walls::decorator::DecoratorId,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,std::hash::random::RandomState,country_core::resources::walls::decorator::DecoratorId>
                    ();
          if (extraout_RAX_13 == 0) {
            pfVar15 = (float *)&DAT_00000004;
            pfVar11 = (float *)&DAT_00000004;
          }
          else {
            pfVar15 = *(float **)(extraout_RAX_13 + 8) + *(longlong *)(extraout_RAX_13 + 0x10) * 2;
            pfVar11 = *(float **)(extraout_RAX_13 + 8);
          }
          do {
            pfVar18 = pfVar11;
            pauVar28 = (undefined1 (*) [16])((ulonglong)(uint)fVar34 << 0x20);
            if (pfVar18 == pfVar15) goto LAB_1408166c0;
            pfVar11 = pfVar18 + 2;
          } while (*(char *)(pfVar18 + 1) != '\0');
          fStack_124 = *pfVar18;
          local_128 = fVar43;
          indexmap::map::IndexMap::
          get<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,country_core::resources::stairs::stairs_assembly_params::AdjustedStairEdge,std::hash::random::RandomState,tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>_>
                    ();
          if (extraout_RAX_14 == 0) {
                    // WARNING: Subroutine does not return
            core::panicking::panic();
          }
          ::utils::xz_rotation_from_forward();
          fVar44 = local_128;
          fVar33 = fStack_124;
          pauVar28 = uStack_120;
        }
      }
LAB_1408166c0:
      pcVar10 = local_138;
      puVar9 = (undefined1 *)local_140;
      country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransformStorage::
      get();
      if (extraout_RAX_15 == (undefined8 *)0x0) {
LAB_140816721:
        *local_1c8 = local_130;
        uStack_108 = 0x3f800000;
        uStack_104 = 0x3f800000;
        uStack_100 = uStack_2d0;
        local_118 = (undefined8 *)puVar9;
        uStack_110 = (ushort)pcVar10;
        uStack_10e = (undefined2)((ulonglong)pcVar10 >> 0x10);
        fStack_10c = 1.0;
        local_128 = fVar44;
        fStack_124 = fVar33;
        uStack_120 = pauVar28;
        country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::
        insert();
        country_core::resources::walls::decorator_storage::DecoratorMaybeMut::get_mut();
      }
      else {
        ::utils::components::transform::Transform::abs_diff_eq();
        if (extraout_AL_02 == '\0') goto LAB_140816721;
        *local_1c8 = local_130;
        local_118 = (undefined8 *)extraout_RAX_15[2];
        uVar2 = extraout_RAX_15[3];
        uStack_100 = extraout_RAX_15[5];
        uStack_108 = (undefined4)extraout_RAX_15[4];
        uStack_104 = (undefined4)((ulonglong)extraout_RAX_15[4] >> 0x20);
        uStack_110 = (ushort)uVar2;
        uStack_10e = (undefined2)((ulonglong)uVar2 >> 0x10);
        fStack_10c = (float)((ulonglong)uVar2 >> 0x20);
        local_128 = (float)*extraout_RAX_15;
        fStack_124 = (float)((ulonglong)*extraout_RAX_15 >> 0x20);
        uStack_120._0_4_ = (float)extraout_RAX_15[1];
        uStack_120._4_4_ = (float)((ulonglong)extraout_RAX_15[1] >> 0x20);
        country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::
        insert();
      }
    }
  }
  *local_180 = (undefined4)local_178;
  country_core::resources::walls::decorator_storage::DecoratorStorage::get_mut();
  if (extraout_RAX_16 != 0) {
    local_280 = *(undefined1 (**) [16])(extraout_RAX_16 + 0x30);
    puStack_270 = *local_280 + *(longlong *)(extraout_RAX_16 + 0x38) + 1;
    auVar38 = *local_280;
    local_268 = ~((ushort)(SUB161(auVar38 >> 7,0) & 1) | (ushort)(SUB161(auVar38 >> 0xf,0) & 1) << 1
                  | (ushort)(SUB161(auVar38 >> 0x17,0) & 1) << 2 |
                  (ushort)(SUB161(auVar38 >> 0x1f,0) & 1) << 3 |
                  (ushort)(SUB161(auVar38 >> 0x27,0) & 1) << 4 |
                  (ushort)(SUB161(auVar38 >> 0x2f,0) & 1) << 5 |
                  (ushort)(SUB161(auVar38 >> 0x37,0) & 1) << 6 |
                  (ushort)(SUB161(auVar38 >> 0x3f,0) & 1) << 7 |
                  (ushort)(SUB161(auVar38 >> 0x47,0) & 1) << 8 |
                  (ushort)(SUB161(auVar38 >> 0x4f,0) & 1) << 9 |
                  (ushort)(SUB161(auVar38 >> 0x57,0) & 1) << 10 |
                  (ushort)(SUB161(auVar38 >> 0x5f,0) & 1) << 0xb |
                  (ushort)(SUB161(auVar38 >> 0x67,0) & 1) << 0xc |
                  (ushort)(SUB161(auVar38 >> 0x6f,0) & 1) << 0xd |
                  (ushort)(SUB161(auVar38 >> 0x77,0) & 1) << 0xe |
                 (ushort)(byte)(auVar38[0xf] >> 7) << 0xf);
    local_260 = *(undefined8 *)(extraout_RAX_16 + 0x48);
    local_258 = extraout_RAX_16 + 0x60;
    local_278 = local_280 + 1;
    local_180 = (undefined4 *)*in_stack_00000048;
    local_1f0 = local_238 + 0x198;
    while( true ) {
      puVar21 = &local_280;
      country_core::resources::walls::decorator_storage::impl$3::next();
      if (local_168 == (undefined1 (*) [16])0x0) break;
      local_f8 = (longlong *)CONCAT44(local_f8._4_4_,local_170._0_4_);
      local_208 = local_158;
      local_218 = local_168;
      puStack_210 = puStack_160;
      minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
      if (*(char *)((longlong)extraout_RAX_17 + 0x22) != '\x04') {
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      fVar43 = *(float *)(extraout_RAX_17 + 1);
      fVar44 = *(float *)((longlong)extraout_RAX_17 + 0xc);
      lVar27 = *(longlong *)(local_238 + 8);
      lVar29 = 0;
      plVar20 = (longlong *)(lVar27 + -0xf8);
      do {
        plVar32 = plVar20;
        if (*(longlong *)(local_238 + 0x10) * -0x150 - lVar29 == 0) {
          local_1a8 = (undefined1 *)0xc2c8000080000000;
          uStack_1a0 = -0.0;
          local_1b8 = (undefined1 (*) [16])0x0;
          uStack_1b0 = (undefined1 (*) [16])0x3f80000000000000;
          uStack_194 = 0;
          fStack_19c = 0.0;
          uStack_198 = 0;
          goto LAB_1408170ff;
        }
        auVar38._0_4_ = -(uint)((int)plVar32[0x44] == (int)*extraout_RAX_17);
        iVar39 = -(uint)((int)((ulonglong)plVar32[0x44] >> 0x20) ==
                        (int)((ulonglong)*extraout_RAX_17 >> 0x20));
        auVar38._4_4_ = auVar38._0_4_;
        auVar38._8_4_ = iVar39;
        auVar38._12_4_ = iVar39;
        uVar13 = movmskpd((int)puVar21,auVar38);
        puVar21 = (undefined8 *)(ulonglong)uVar13;
        lVar29 = lVar29 + -0x150;
        plVar20 = plVar32 + 0x2a;
      } while (uVar13 != 3);
      local_150 = CONCAT44(local_150._4_4_,fVar44);
      if ((fVar44 < 0.0) || (1.0 < fVar44)) {
        local_140 = &local_150;
        local_138 = core::fmt::float::impl$5::fmt;
        local_128 = 97.107056;
        fStack_124 = 1.4013e-45;
        uStack_120._0_4_ = 1.4013e-45;
        uStack_120._4_4_ = 0.0;
        uStack_108 = 0;
        uStack_104 = 0;
        local_118 = &local_140;
        uStack_110 = 1;
        uStack_10e = 0;
        fStack_10c = 0.0;
                    // WARNING: Subroutine does not return
        core::panicking::panic_fmt();
      }
      if (NAN(fVar44)) {
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      ::utils::curve::Curve::
      get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
                ();
      uVar16 = plVar32[0x47];
      if (uVar16 < 2) {
        lVar27 = (lVar27 - lVar29) + -0x20;
        if (uVar16 == 0) goto LAB_140816a20;
LAB_140816998:
        lVar29 = uVar16 * 4;
        uVar19 = 0;
        uVar24 = 0;
        do {
          uVar23 = *(ushort *)(lVar27 + 2 + uVar19 * 4) + uVar24;
          if ((extraout_RAX_18 & 0xffffffff) < uVar23) break;
          uVar19 = uVar19 + 1;
          lVar29 = lVar29 + -4;
          uVar24 = uVar23;
        } while (lVar29 != 0);
      }
      else {
        lVar27 = plVar32[0x45];
        uVar16 = plVar32[0x46];
        if (uVar16 != 0) goto LAB_140816998;
LAB_140816a20:
        uVar19 = 0;
        uVar24 = 0;
      }
      uVar23 = extraout_RAX_18 & 0xffffffff;
      if (uVar16 - 1 < uVar19) {
        uVar19 = uVar16 - 1;
      }
      if (uVar16 <= uVar19) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      if ((ulonglong)plVar32[0x2e] <= uVar23) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      uVar16 = uVar23 + 1;
      if ((ulonglong)plVar32[0x2e] <= uVar16) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      uVar2 = *(undefined8 *)(plVar32[0x2d] + uVar23 * 8);
      uVar3 = *(undefined8 *)(plVar32[0x2d] + 8 + uVar23 * 8);
      fVar44 = (float)uVar2;
      fVar33 = (float)((ulonglong)uVar2 >> 0x20);
      fVar44 = ((float)uVar3 - fVar44) * extraout_XMM0_Da_04 + fVar44;
      fVar33 = ((float)((ulonglong)uVar3 >> 0x20) - fVar33) * extraout_XMM0_Da_04 + fVar33;
      fVar34 = 1.0 / SQRT(fVar33 * fVar33 + fVar44 * fVar44);
      uVar13 = (uint)((int)fVar34 - 1U < 0x7fffff ||
                     (int)ABS(fVar34) - 0x800000U < 0x7f000000 && -1 < (int)fVar34);
      uVar35 = uVar13 << 0x1f;
      local_1f8 = CONCAT44((int)(uVar13 << 0x1f) >> 0x1f & (uint)(fVar34 * fVar44),
                           ~uVar35 & 0x80000000 | (int)uVar35 >> 0x1f & (uint)(-fVar34 * fVar33));
      if (SBORROW8(0,plVar32[0x32])) {
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      uVar6 = plVar32[0x34];
      if (uVar6 <= uVar23) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      if (uVar6 <= uVar16) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      uVar30 = *(ushort *)(lVar27 + 2 + uVar19 * 4) + uVar24;
      lVar29 = plVar32[0x33];
      fVar44 = 1.0 - extraout_XMM0_Da_04;
      uVar2 = *(undefined8 *)(lVar29 + uVar23 * 0xc);
      uVar3 = *(undefined8 *)(lVar29 + uVar16 * 0xc);
      fVar34 = extraout_XMM0_Da_04 * (float)uVar3 + (float)uVar2 * fVar44;
      fVar33 = extraout_XMM0_Da_04 * (float)((ulonglong)uVar3 >> 0x20) +
               (float)((ulonglong)uVar2 >> 0x20) * fVar44;
      fVar44 = *(float *)(lVar29 + 8 + uVar16 * 0xc) * extraout_XMM0_Da_04 +
               fVar44 * *(float *)(lVar29 + 8 + uVar23 * 0xc);
      switch(*(undefined1 *)(lVar27 + uVar19 * 4)) {
      case 2:
        uVar19 = uVar30 + 1;
        if (uVar19 < uVar24) {
                    // WARNING: Subroutine does not return
          core::slice::index::slice_index_order_fail();
        }
        if ((ulonglong)plVar32[0x37] <= uVar30) {
                    // WARNING: Subroutine does not return
          core::slice::index::slice_end_index_len_fail();
        }
        lVar27 = plVar32[0x36];
        if (lVar27 + uVar19 * 4 == 4) {
          local_178 = uVar16;
                    // WARNING: Subroutine does not return
          core::option::unwrap_failed();
        }
        fVar42 = (*(float *)(lVar27 + uVar30 * 4) - *(float *)(lVar27 + uVar24 * 4)) *
                 *(float *)(plVar32 + 0x38);
        if (fVar42 <= 1e-08) {
          fVar42 = 1e-08;
        }
        fVar42 = 0.5 / fVar42;
        if (1.0 <= fVar42) {
          fVar42 = 1.0;
        }
        local_178 = uVar16;
        fVar42 = (float)roundf(1.0 / fVar42);
        uVar16 = 0;
        if (0.0 <= fVar42) {
          uVar16 = (longlong)(fVar42 - 9.223372e+18) & (longlong)fVar42 >> 0x3f | (longlong)fVar42;
        }
        if (1.8446743e+19 < fVar42) {
          uVar16 = 0xffffffffffffffff;
        }
        if (uVar6 <= uVar30) {
                    // WARNING: Subroutine does not return
          core::slice::index::slice_end_index_len_fail();
        }
        lVar27 = lVar29 + uVar19 * 0xc;
        if (lVar27 == 0xc) {
                    // WARNING: Subroutine does not return
          core::option::unwrap_failed();
        }
        fVar42 = *(float *)(lVar29 + 4 + uVar24 * 0xc);
        fVar40 = *(float *)(lVar27 + -8);
        fVar41 = fVar40;
        if (fVar40 <= fVar42) {
          fVar41 = fVar42;
        }
        fVar45 = fVar40;
        if (fVar42 <= fVar40) {
          fVar45 = fVar42;
        }
        fVar42 = (float)(-(uint)NAN(fVar42) & (uint)fVar40 | ~-(uint)NAN(fVar42) & (uint)fVar45);
        fVar41 = (fVar41 - fVar42) / (float)(uVar16 + (uVar16 == 0));
        fVar40 = fVar41;
        if (fVar41 <= 1e-05) {
          fVar40 = 1e-05;
        }
        fVar33 = ceilf((fVar33 - fVar42) / fVar40);
        fVar33 = fVar33 * fVar41 + fVar42;
        uVar16 = local_178;
        break;
      case 3:
        if (uVar30 + 1 < uVar24) {
                    // WARNING: Subroutine does not return
          core::slice::index::slice_index_order_fail();
        }
        if (uVar6 <= uVar30) {
                    // WARNING: Subroutine does not return
          core::slice::index::slice_end_index_len_fail();
        }
        if ((uVar30 + 1) - uVar24 != 2) {
          local_128 = 0.0;
          fStack_124 = 0.0;
                    // WARNING: Subroutine does not return
          core::panicking::assert_failed<usize,usize>();
        }
        local_148 = *(float *)(lVar29 + 8 + uVar24 * 0xc);
        local_150 = *(undefined8 *)(lVar29 + uVar24 * 0xc);
        local_138 = (code *)CONCAT44(local_138._4_4_,*(undefined4 *)(lVar29 + 0x14 + uVar24 * 0xc));
        local_140 = *(undefined8 **)(lVar29 + 0xc + uVar24 * 0xc);
        local_178 = uVar16;
        decorator_visual::stairs_assemble::calculate_adjusted_ladder_positions();
        fVar40 = fStack_124;
        fVar41 = (float)local_118;
        fVar45 = (float)uStack_120 - local_118._4_4_;
        fVar42 = fVar45;
        if ((float)local_118 <= fStack_124) {
          fVar42 = local_118._4_4_ - (float)uStack_120;
        }
        fVar46 = local_128 - uStack_120._4_4_;
        fVar36 = (float)(~-(uint)(fStack_124 < (float)local_118) &
                         (uint)(uStack_120._4_4_ - local_128) |
                        -(uint)(fStack_124 < (float)local_118) & (uint)fVar46);
        fVar37 = 1.0 / SQRT(fVar42 * fVar42 + fVar36 * fVar36);
        if ((int)fVar37 - 1U < 0x7fffff ||
            (int)ABS(fVar37) - 0x800000U < 0x7f000000 && -1 < (int)fVar37) {
          fVar36 = fVar36 * fVar37;
          fVar42 = fVar42 * fVar37;
        }
        else {
          fVar42 = 0.0;
          fVar36 = 0.0;
        }
        local_150 = CONCAT44(fVar42,fVar36);
        ::utils::swizzlers::impl$0::x0y();
        fVar42 = fVar40 - fVar41;
        fVar42 = ceilf(SQRT(fVar45 * fVar45 + fVar42 * fVar42 + fVar46 * fVar46) / 0.65);
        uVar16 = 0;
        if (0.0 <= fVar42) {
          uVar16 = (longlong)(fVar42 - 9.223372e+18) & (longlong)fVar42 >> 0x3f | (longlong)fVar42;
        }
        if (1.8446743e+19 < fVar42) {
          uVar16 = 0xffffffffffffffff;
        }
        if (uVar16 < 4) {
          uVar16 = 3;
        }
        fVar41 = (fVar41 - fVar40) / (float)(uVar16 - 1);
        fVar42 = fVar41;
        if (fVar41 <= 1e-05) {
          fVar42 = 1e-05;
        }
        fVar33 = ceilf((fVar33 - fVar40) / fVar42);
        fVar34 = fVar34 + SUB84(local_140,0) * 0.2;
        fVar33 = fVar33 * fVar41 + fVar40 + (float)((ulonglong)local_140 >> 0x20) * 0.2;
        fVar44 = fVar44 + local_138._0_4_ * 0.2;
        uVar16 = local_178;
        break;
      default:
      }
      if ((ulonglong)plVar32[0x2b] <= uVar23) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      if ((ulonglong)plVar32[0x2b] <= uVar16) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      lVar27 = plVar32[0x2a];
      fVar42 = *(float *)(lVar27 + uVar23 * 4);
      fVar40 = *(float *)(lVar27 + 4 + uVar23 * 4);
      ::utils::swizzlers::impl$0::x0y();
      fVar45 = fStack_124;
      fVar41 = local_128;
      fVar43 = fVar43 * (extraout_XMM0_Da_04 * (fVar40 - fVar42) + fVar42) * 0.5;
      fVar42 = (float)uStack_120 * fVar43;
      country_core::resources::walls::decorator::DecoratorRotationXZ::from_xz_forward_vector();
      country_core::resources::walls::decorator::DecoratorRotationXZ::offset();
      local_140 = (undefined8 *)CONCAT44(local_140._4_4_,extraout_XMM0_Da_05);
      country_core::resources::walls::decorator::DecoratorRotationXZ::to_quat();
      uStack_1a0 = fVar44 + fVar42;
      local_1b8 = (undefined1 (*) [16])CONCAT44(fStack_124,local_128);
      uStack_1b0 = uStack_120;
      local_1a8 = (undefined1 *)CONCAT44(fVar33 + fVar43 * fVar45,fVar34 + fVar43 * fVar41);
      fStack_19c = 1.0;
      uStack_198 = 0x3f800000;
      uStack_194 = 0x3f800000;
LAB_1408170ff:
      country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransformStorage::
      get();
      if (extraout_RAX_19 == (undefined8 *)0x0) {
LAB_140817138:
        *local_1c8 = local_130;
        uStack_108 = uStack_198;
        uStack_104 = uStack_194;
        uStack_100 = uStack_190;
        local_118 = (undefined8 *)local_1a8;
        uStack_110 = SUB42(uStack_1a0,0);
        uStack_10e = (undefined2)((uint)uStack_1a0 >> 0x10);
        fStack_10c = fStack_19c;
        local_128 = SUB84(local_1b8,0);
        fStack_124 = (float)((ulonglong)local_1b8 >> 0x20);
        uStack_120._0_4_ = SUB84(uStack_1b0,0);
        uStack_120._4_4_ = (float)((ulonglong)uStack_1b0 >> 0x20);
        country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::
        insert();
        country_core::resources::walls::decorator_storage::DecoratorMaybeMut::get_mut();
      }
      else {
        ::utils::components::transform::Transform::abs_diff_eq();
        if (extraout_AL_03 == '\0') goto LAB_140817138;
        *local_1c8 = local_130;
        local_118 = (undefined8 *)extraout_RAX_19[2];
        uVar2 = extraout_RAX_19[3];
        uStack_100 = extraout_RAX_19[5];
        uStack_108 = (undefined4)extraout_RAX_19[4];
        uStack_104 = (undefined4)((ulonglong)extraout_RAX_19[4] >> 0x20);
        uStack_110 = (ushort)uVar2;
        uStack_10e = (undefined2)((ulonglong)uVar2 >> 0x10);
        fStack_10c = (float)((ulonglong)uVar2 >> 0x20);
        local_128 = (float)*extraout_RAX_19;
        fStack_124 = (float)((ulonglong)*extraout_RAX_19 >> 0x20);
        uStack_120._0_4_ = (float)extraout_RAX_19[1];
        uStack_120._4_4_ = (float)((ulonglong)extraout_RAX_19[1] >> 0x20);
        country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::
        insert();
      }
    }
  }
  if (local_129 != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_20 == 1) {
      plVar20 = extraout_RAX_20 + 1;
    }
    else {
      if (*extraout_RAX_20 != 0) {
LAB_140817284:
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      std::sys::thread_local::native::lazy::Storage::
      initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                ();
      plVar20 = extraout_RAX_22;
      if (extraout_RAX_22 == (longlong *)0x0) goto LAB_140817284;
    }
    if (*plVar20 != 0) {
LAB_140817417:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar20 = -1;
    local_f8 = plVar20;
    puffin::ThreadProfiler::end_scope();
    *local_f8 = *local_f8 + 1;
  }
  return;
}

