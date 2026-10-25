
/* WARNING: Removing unreachable block (ram,0x00014074e574) */
/* WARNING: Removing unreachable block (ram,0x00014074d131) */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl
system_wall_constructor::segmentation::sample_wall_path_intersections::
sample_wall_path_intersections(void)

{
  byte *pbVar1;
  float *pfVar2;
  char *pcVar3;
  undefined8 *puVar4;
  undefined8 *puVar5;
  byte bVar6;
  byte bVar7;
  float fVar8;
  ulonglong *puVar9;
  longlong lVar10;
  longlong lVar11;
  float *pfVar12;
  float *pfVar13;
  uint uVar14;
  uint uVar15;
  undefined8 uVar16;
  undefined8 uVar17;
  undefined8 uVar18;
  bool bVar19;
  float fVar20;
  float fVar21;
  float fVar22;
  char extraout_AL;
  uint uVar23;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong lVar24;
  ulonglong extraout_RAX_02;
  longlong extraout_RAX_03;
  float *extraout_RAX_04;
  float *pfVar25;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  longlong *extraout_RAX_07;
  undefined4 *_Dst;
  float *pfVar26;
  longlong *plVar27;
  longlong *in_RDX;
  ulonglong uVar28;
  ulonglong uVar29;
  ulonglong uVar30;
  float fVar31;
  longlong lVar32;
  undefined8 *in_R8;
  undefined *puVar33;
  longlong lVar34;
  ulonglong uVar35;
  ushort uVar36;
  longlong lVar37;
  ulonglong uVar38;
  longlong lVar39;
  longlong lVar40;
  ulonglong uVar41;
  float *pfVar42;
  int iVar43;
  int iVar44;
  byte bVar45;
  char cVar46;
  char cVar55;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float fVar47;
  float extraout_XMM0_Da_03;
  float extraout_XMM0_Da_04;
  float extraout_XMM0_Da_05;
  float extraout_XMM0_Da_06;
  float extraout_XMM0_Da_07;
  float extraout_XMM0_Da_08;
  float extraout_XMM0_Da_09;
  float extraout_XMM0_Da_10;
  float fVar48;
  float fVar49;
  float extraout_XMM0_Da_11;
  float extraout_XMM0_Da_12;
  float extraout_XMM0_Da_13;
  float extraout_XMM0_Da_14;
  float extraout_XMM0_Da_15;
  float extraout_XMM0_Da_16;
  float extraout_XMM0_Da_17;
  float extraout_XMM0_Da_18;
  float fVar50;
  char cVar53;
  char cVar54;
  undefined1 auVar51 [16];
  undefined1 auVar52 [16];
  undefined1 auVar56 [16];
  undefined1 auVar57 [16];
  float fVar58;
  float fVar59;
  float fVar60;
  float fVar61;
  float fVar62;
  float fVar63;
  float unaff_XMM10_Da;
  float fVar64;
  float fVar65;
  float fVar66;
  float fVar67;
  undefined8 local_248;
  undefined8 uStack_240;
  undefined8 local_238;
  undefined8 uStack_230;
  undefined8 local_228;
  undefined8 uStack_220;
  undefined8 local_218;
  longlong local_1c0;
  ulonglong local_138;
  ulonglong local_130;
  longlong lStack_128;
  ulonglong local_120;
  
  plVar27 = in_RDX;
  puffin::are_scopes_on();
  if (extraout_AL != '\0') {
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (plVar27 != (longlong *)0x0) {
        if (&DAT_00000064 < plVar27) {
          if (plVar27 != (longlong *)&DAT_00000065) goto LAB_14074c7ed;
        }
        else if ((char)plVar27[0x2854f6e2] < -0x40) {
LAB_14074c7ed:
                    /* WARNING: Subroutine does not return */
          core::str::slice_error_fail();
        }
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (plVar27 != (longlong *)0xfffffffffffffffe)) {
        if ((undefined1 *)((longlong)plVar27 + 2) < (undefined1 *)0x65) {
          if (*(char *)((longlong)plVar27 + 0x142a7b712) < -0x40) {
LAB_14074c83a:
                    /* WARNING: Subroutine does not return */
            core::str::slice_error_fail();
          }
        }
        else if ((undefined1 *)((longlong)plVar27 + 2) != (undefined1 *)0x65) goto LAB_14074c83a;
      }
    }
    puVar33 = &DAT_142a7b88a;
    do {
      bVar45 = puVar33[-1];
      uVar23 = (uint)(char)bVar45;
      if ((char)bVar45 < '\0') {
        bVar6 = puVar33[-2];
        if ((char)bVar6 < -0x40) {
          bVar7 = puVar33[-3];
          if ((char)bVar7 < -0x40) {
            pbVar1 = puVar33 + -4;
            puVar33 = puVar33 + -4;
            uVar23 = bVar7 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar33 = puVar33 + -3;
            uVar23 = bVar7 & 0xf;
          }
          uVar23 = bVar6 & 0x3f | uVar23 << 6;
        }
        else {
          puVar33 = puVar33 + -2;
          uVar23 = bVar6 & 0x1f;
        }
        uVar23 = (uint)(bVar45 & 0x3f) | uVar23 << 6;
        if (uVar23 != 0x5c) goto LAB_14074c8f4;
LAB_14074c906:
        if (puVar33 == &UNK_142a7b837) break;
        if (puVar33 + -0x142a7b837 < (undefined *)0x52) {
          if (-0x41 < (char)puVar33[1]) break;
        }
        else if (puVar33 + -0x142a7b837 == (undefined *)0x52) break;
                    /* WARNING: Subroutine does not return */
        core::str::slice_error_fail();
      }
      puVar33 = puVar33 + -1;
      if (uVar23 == 0x5c) goto LAB_14074c906;
LAB_14074c8f4:
      if (uVar23 == 0x2f) goto LAB_14074c906;
    } while (puVar33 != &DAT_142a7b838);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar27 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar27 = extraout_RAX_06, extraout_RAX_06 == (longlong *)0x0)) {
LAB_14074e675:
                    /* WARNING: Subroutine does not return */
      std::thread::local::panic_access_error();
    }
    if (*plVar27 != 0) {
LAB_14074e681:
                    /* WARNING: Subroutine does not return */
      core::cell::panic_already_borrowed();
    }
    *plVar27 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar27 = *plVar27 + 1;
  }
  *(undefined4 *)in_RDX[2] = *(undefined4 *)((longlong)in_RDX + 0x1c);
  plVar27 = (longlong *)*in_RDX;
  country_core::systems::paths::draw_path::PathRastersResources::as_float_grid();
  puVar9 = (ulonglong *)*in_R8;
  lVar32 = in_R8[1];
  uVar28 = *puVar9;
  uVar41 = 0;
  uVar38 = uVar28 - *(ulonglong *)(lVar32 + 0x18);
  if (uVar28 < *(ulonglong *)(lVar32 + 0x18)) {
    uVar38 = uVar41;
  }
  uVar29 = uVar28 - *(ulonglong *)(lVar32 + 0x38);
  if (uVar28 < *(ulonglong *)(lVar32 + 0x38)) {
    uVar29 = uVar41;
  }
  lVar40 = uVar38 * 0x10 + *(longlong *)(lVar32 + 8);
  uVar28 = *(ulonglong *)(lVar32 + 0x10) - uVar38;
  if (*(ulonglong *)(lVar32 + 0x10) < uVar38) {
    lVar40 = 8;
    uVar28 = uVar41;
  }
  lVar24 = uVar29 * 0x10 + *(longlong *)(lVar32 + 0x28);
  uVar38 = *(ulonglong *)(lVar32 + 0x30) - uVar29;
  if (*(ulonglong *)(lVar32 + 0x30) < uVar29) {
    lVar24 = 8;
    uVar38 = uVar41;
  }
  *puVar9 = (*(longlong *)(lVar32 + 0x40) - uVar38) - uVar28;
  lVar37 = uVar28 * 0x10 + lVar40;
  lVar32 = uVar38 * 0x10 + lVar24;
  if (lVar40 == 0) goto LAB_14074cae0;
LAB_14074caa7:
  if (lVar40 == lVar37) goto LAB_14074cae0;
  *puVar9 = *puVar9 + 1;
  lVar10 = plVar27[3];
  local_1c0 = lVar40 + 0x10;
  lVar34 = lVar24;
  lVar11 = lVar40;
  do {
    lVar24 = lVar34;
    lVar40 = local_1c0;
    if (lVar10 == 0) {
LAB_14074e557:
                    /* WARNING: Subroutine does not return */
      core::option::unwrap_failed();
    }
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    lVar10 = *plVar27;
    bVar45 = (byte)(extraout_RAX_02 >> 0x39);
    auVar51 = ZEXT216(CONCAT11(bVar45,bVar45));
    auVar51 = pshuflw(auVar51,auVar51,0);
    lVar34 = 0;
    uVar28 = extraout_RAX_02;
    while( true ) {
      uVar28 = plVar27[1] & uVar28;
      pcVar3 = (char *)(lVar10 + uVar28);
      cVar46 = auVar51[0];
      auVar57[0] = -(*pcVar3 == cVar46);
      cVar53 = auVar51[1];
      auVar57[1] = -(pcVar3[1] == cVar53);
      cVar54 = auVar51[2];
      auVar57[2] = -(pcVar3[2] == cVar54);
      cVar55 = auVar51[3];
      auVar57[3] = -(pcVar3[3] == cVar55);
      auVar57[4] = -(pcVar3[4] == cVar46);
      auVar57[5] = -(pcVar3[5] == cVar53);
      auVar57[6] = -(pcVar3[6] == cVar54);
      auVar57[7] = -(pcVar3[7] == cVar55);
      auVar57[8] = -(pcVar3[8] == cVar46);
      auVar57[9] = -(pcVar3[9] == cVar53);
      auVar57[10] = -(pcVar3[10] == cVar54);
      auVar57[0xb] = -(pcVar3[0xb] == cVar55);
      auVar57[0xc] = -(pcVar3[0xc] == cVar46);
      auVar57[0xd] = -(pcVar3[0xd] == cVar53);
      auVar57[0xe] = -(pcVar3[0xe] == cVar54);
      auVar57[0xf] = -(pcVar3[0xf] == cVar55);
      uVar36 = (ushort)(SUB161(auVar57 >> 7,0) & 1) | (ushort)(SUB161(auVar57 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar57 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar57 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar57 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar57 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar57 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar57 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar57 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar57 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar57 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar57 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar57 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar57 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar57 >> 0x77,0) & 1) << 0xe | (ushort)(auVar57[0xf] >> 7) << 0xf;
      uVar23 = (uint)uVar36;
      if (uVar36 != 0) break;
LAB_14074cbb0:
      auVar56[0] = -(*pcVar3 == __xmm_ffffffffffffffffffffffffffffffff);
      auVar56[1] = -(pcVar3[1] == UNK_142925991);
      auVar56[2] = -(pcVar3[2] == UNK_142925992);
      auVar56[3] = -(pcVar3[3] == UNK_142925993);
      auVar56[4] = -(pcVar3[4] == UNK_142925994);
      auVar56[5] = -(pcVar3[5] == UNK_142925995);
      auVar56[6] = -(pcVar3[6] == UNK_142925996);
      auVar56[7] = -(pcVar3[7] == UNK_142925997);
      auVar56[8] = -(pcVar3[8] == UNK_142925998);
      auVar56[9] = -(pcVar3[9] == UNK_142925999);
      auVar56[10] = -(pcVar3[10] == UNK_14292599a);
      auVar56[0xb] = -(pcVar3[0xb] == UNK_14292599b);
      auVar56[0xc] = -(pcVar3[0xc] == UNK_14292599c);
      auVar56[0xd] = -(pcVar3[0xd] == UNK_14292599d);
      auVar56[0xe] = -(pcVar3[0xe] == UNK_14292599e);
      auVar56[0xf] = -(pcVar3[0xf] == UNK_14292599f);
      if ((((((((((((((((SUB161(auVar56 >> 7,0) & 1) != 0 || (SUB161(auVar56 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar56 >> 0x17,0) & 1) != 0) || (SUB161(auVar56 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar56 >> 0x27,0) & 1) != 0) || (SUB161(auVar56 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar56 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar56 >> 0x3f,0) & 1) != 0) || (SUB161(auVar56 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar56 >> 0x4f,0) & 1) != 0) || (SUB161(auVar56 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar56 >> 0x5f,0) & 1) != 0) || (SUB161(auVar56 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar56 >> 0x6f,0) & 1) != 0) || (SUB161(auVar56 >> 0x77,0) & 1) != 0) ||
          auVar56[0xf] < '\0') goto LAB_14074e557;
      uVar28 = uVar28 + lVar34 + 0x10;
      lVar34 = lVar34 + 0x10;
    }
    while( true ) {
      uVar14 = 0;
      for (uVar15 = uVar23; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
        uVar14 = uVar14 + 1;
      }
      uVar38 = uVar14 + uVar28 & plVar27[1];
      lVar39 = uVar38 * -0x2a8;
      if (*(longlong *)(lVar11 + 8) == *(longlong *)(lVar10 + -0x2a8 + lVar39)) break;
      uVar36 = (ushort)(uVar23 - 1) & (ushort)uVar23;
      uVar23 = CONCAT22((short)(uVar23 - 1 >> 0x10),uVar36);
      if (uVar36 == 0) goto LAB_14074cbb0;
    }
    uVar28 = *(ulonglong *)(lVar10 + -600 + lVar39);
    toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
    lVar39 = lVar39 + lVar10;
    uVar41 = *(ulonglong *)(lVar39 + -0x1d0);
    uVar29 = uVar28;
    fVar58 = __real_3f800000;
    if (uVar41 < uVar28) {
      uVar35 = uVar28 - uVar41;
      uVar29 = uVar41;
      if (*(longlong *)(lVar39 + -0x1e0) - uVar41 < uVar35) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        uVar29 = *(ulonglong *)(lVar39 + -0x1d0);
      }
      fVar58 = __real_3f800000;
      lVar11 = *(longlong *)(lVar39 + -0x1d8);
      _Dst = (undefined4 *)(lVar11 + uVar29 * 4);
      if (1 < uVar35) {
        memset(_Dst,0,(~uVar41 + uVar28) * 4);
        _Dst = (undefined4 *)(lVar11 + ((uVar29 + uVar28) - uVar41) * 4 + -4);
        uVar29 = (uVar35 + uVar29) - 1;
      }
      *_Dst = 0;
      uVar29 = uVar29 + 1;
    }
    *(ulonglong *)(lVar39 + -0x1d0) = uVar29;
    if (uVar28 < 2) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_fmt();
    }
    lVar11 = *(longlong *)(lVar39 + -0x1d8);
    if (uVar28 < uVar29) {
      uVar29 = uVar28;
    }
    uVar28 = 0;
    if (uVar29 != 0) {
      do {
        ::utils::float_grid::impl$0::
        bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
        ::utils::float_grid::impl$0::
        bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
        ::utils::float_grid::impl$0::
        bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
        ::utils::float_grid::impl$0::
        bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
        uVar41 = uVar28 + 1;
        fVar62 = extraout_XMM0_Da + extraout_XMM0_Da_00 + extraout_XMM0_Da_01 + extraout_XMM0_Da_02;
        fVar58 = 0.0;
        if (0.0 <= fVar62) {
          fVar58 = fVar62;
        }
        fVar62 = __real_3f800000;
        if (fVar58 <= __real_3f800000) {
          fVar62 = fVar58;
        }
        uVar35 = 0;
        fVar48 = __real_3f800000;
        do {
          fVar58 = ((float)(longlong)uVar35 + __real_3f800000) * __real_3e800000;
          fVar58 = (float)((uint)fVar58 ^ ___xmm_80000000800000008000000080000000) * fVar58;
          fVar47 = expf(fVar58 + fVar58);
          ::utils::float_grid::impl$0::
          bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>
                    ();
          ::utils::float_grid::impl$0::
          bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>
                    ();
          ::utils::float_grid::impl$0::
          bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>
                    ();
          ::utils::float_grid::impl$0::
          bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>
                    ();
          ::utils::float_grid::impl$0::
          bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>
                    ();
          ::utils::float_grid::impl$0::
          bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>
                    ();
          ::utils::float_grid::impl$0::
          bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>
                    ();
          ::utils::float_grid::impl$0::
          bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>
                    ();
          fVar58 = __real_3f800000;
          uVar35 = uVar35 + 1;
          unaff_XMM10_Da =
               extraout_XMM0_Da_03 + extraout_XMM0_Da_04 + extraout_XMM0_Da_05 + extraout_XMM0_Da_06
          ;
          fVar50 = 0.0;
          if (0.0 <= unaff_XMM10_Da) {
            fVar50 = unaff_XMM10_Da;
          }
          fVar60 = __real_3f800000;
          if (fVar50 <= __real_3f800000) {
            fVar60 = fVar50;
          }
          fVar66 = extraout_XMM0_Da_07 + extraout_XMM0_Da_08 + extraout_XMM0_Da_09 +
                   extraout_XMM0_Da_10;
          fVar50 = 0.0;
          if (0.0 <= fVar66) {
            fVar50 = fVar66;
          }
          fVar66 = __real_3f800000;
          if (fVar50 <= __real_3f800000) {
            fVar66 = fVar50;
          }
          fVar50 = fVar66;
          if (fVar60 <= fVar66) {
            fVar50 = fVar60;
          }
          fVar62 = fVar62 + (float)(~-(uint)NAN(fVar60) & (uint)fVar50 |
                                   -(uint)NAN(fVar60) & (uint)fVar66) * (fVar47 + fVar47);
          fVar48 = fVar48 + fVar47 + fVar47;
        } while (uVar35 < 4);
        *(float *)(lVar11 + uVar28 * 4) = fVar62 / fVar48;
        uVar28 = uVar41;
      } while (uVar41 < uVar29);
    }
    lVar11 = *(longlong *)(lVar39 + -0x1d8);
    uVar28 = *(ulonglong *)(lVar39 + -0x1d0);
    if (uVar28 == 0) {
      uVar29 = 0;
    }
    else {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      fVar58 = __real_3f800000;
      if ((uVar28 < 8) || (4U - lVar11 < 0x20)) {
        uVar41 = 0;
      }
      else {
        uVar29 = uVar28 & 0xfffffffffffffff8;
        uVar41 = 0;
        do {
          puVar4 = (undefined8 *)(lVar11 + uVar41 * 4);
          uVar16 = puVar4[1];
          puVar5 = (undefined8 *)(lVar11 + 0x10 + uVar41 * 4);
          uVar17 = *puVar5;
          uVar18 = puVar5[1];
          *(undefined8 *)(uVar41 * 4 + 4) = *puVar4;
          *(undefined8 *)(uVar41 * 4 + 0xc) = uVar16;
          *(undefined8 *)(uVar41 * 4 + 0x14) = uVar17;
          *(undefined8 *)(uVar41 * 4 + 0x1c) = uVar18;
          uVar41 = uVar41 + 8;
        } while (uVar29 != uVar41);
        uVar41 = uVar29;
        if (uVar28 == uVar29) goto LAB_14074d220;
      }
      uVar29 = uVar41;
      uVar30 = uVar41;
      for (uVar35 = uVar28 & 3; uVar35 != 0; uVar35 = uVar35 - 1) {
        *(undefined4 *)(uVar29 * 4 + 4) = *(undefined4 *)(lVar11 + uVar30 * 4);
        uVar29 = uVar29 + 1;
        uVar30 = uVar30 + 1;
      }
      if (uVar41 - uVar28 < 0xfffffffffffffffd) {
        do {
          *(undefined4 *)(uVar29 * 4 + 4) = *(undefined4 *)(lVar11 + uVar30 * 4);
          *(undefined4 *)(uVar29 * 4 + 8) = *(undefined4 *)(lVar11 + 4 + uVar30 * 4);
          *(undefined4 *)(uVar29 * 4 + 0xc) = *(undefined4 *)(lVar11 + 8 + uVar30 * 4);
          *(undefined4 *)(uVar29 * 4 + 0x10) = *(undefined4 *)(lVar11 + 0xc + uVar30 * 4);
          uVar29 = uVar29 + 4;
          uVar30 = uVar30 + 4;
        } while (uVar28 != uVar30);
      }
    }
LAB_14074d220:
    if (*(longlong *)(lVar39 + -0x1d0) != 0) {
      lVar11 = *(longlong *)(lVar39 + -0x1d8);
      lVar34 = *(longlong *)(lVar39 + -0x1d0) << 2;
      uVar28 = 0;
      do {
        if (uVar29 == uVar28) {
                    /* WARNING: Subroutine does not return */
          core::panicking::panic_bounds_check();
        }
        fVar62 = *(float *)(uVar28 * 4 + 4);
        fVar48 = fVar58;
        if (uVar28 - 1 < uVar29) {
          fVar62 = fVar62 + *(float *)(uVar28 * 4) * __real_3f1b4598;
          fVar48 = __real_3fcda2cc;
        }
        if (uVar28 + 1 < uVar29) {
          fVar62 = fVar62 + *(float *)(uVar28 * 4 + 8) * __real_3f1b4598;
          fVar48 = fVar48 + __real_3f1b4598;
        }
        if (uVar28 - 2 < uVar29) {
          fVar62 = fVar62 + *(float *)(uVar28 * 4 + -4) * __real_3e0a9555;
          fVar48 = fVar48 + __real_3e0a9555;
        }
        if (uVar28 + 2 < uVar29) {
          fVar62 = fVar62 + *(float *)(uVar28 * 4 + 0xc) * __real_3e0a9555;
          fVar48 = fVar48 + __real_3e0a9555;
        }
        *(float *)(lVar11 + uVar28 * 4) = fVar62 / fVar48;
        lVar34 = lVar34 + -4;
        uVar28 = uVar28 + 1;
      } while (lVar34 != 0);
    }
    uVar28 = *(ulonglong *)(lVar39 + -600);
    if (uVar28 == 0) {
      local_130 = 0;
      lStack_128 = 4;
      pfVar25 = (float *)&DAT_00000004;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_03 == 0) {
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error();
      }
      std::alloc::__default_lib_allocator::__rust_alloc();
      pfVar25 = extraout_RAX_04;
      local_130 = uVar28;
      lStack_128 = extraout_RAX_03;
      if (extraout_RAX_04 == (float *)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error();
      }
    }
    local_120 = 0;
    local_138 = 0;
    if (uVar28 == 0) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_bounds_check();
    }
    uVar41 = *(ulonglong *)(lVar39 + -0x220);
    if (uVar41 == 0) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_bounds_check();
    }
    pfVar12 = *(float **)(lVar39 + -0x260);
    pfVar42 = *(float **)(lVar39 + -0x228);
    pfVar13 = *(float **)(lVar39 + -0x1d8);
    uVar29 = *(ulonglong *)(lVar39 + -0x1d0);
    fVar50 = *pfVar12;
    fVar62 = pfVar12[1];
    fVar48 = 0.0;
    if (*pfVar42 <= __real_3f000000) {
      if (uVar29 == 0) {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic_bounds_check();
      }
      fVar48 = *pfVar13;
    }
    uVar35 = uVar28;
    if (uVar29 < uVar28) {
      uVar35 = uVar29;
    }
    if (uVar41 < uVar35) {
      uVar35 = uVar41;
    }
    uVar41 = 0;
    bVar19 = false;
    fVar47 = unaff_XMM10_Da;
    fVar60 = 0.0;
LAB_14074d511:
    fVar22 = _UNK_142a7b64c;
    fVar21 = _UNK_142a7b648;
    fVar20 = _UNK_142a7b644;
    fVar61 = ___xmm_c0a00000c0a00000c0a00000c0a00000;
    fVar65 = _UNK_14292593c;
    fVar64 = _UNK_142925938;
    fVar49 = _UNK_142925934;
    fVar63 = ___xmm_3f8000003f8000003f8000003f800000;
    unaff_XMM10_Da = __real_3f800000;
    pfVar26 = pfVar12 + uVar41 * 3 + 2;
    fVar67 = fVar48;
    fVar31 = fVar50;
    fVar66 = fVar62;
    while (uVar29 = uVar41, uVar29 < uVar35) {
      pfVar2 = pfVar26 + -2;
      fVar50 = *pfVar2;
      fVar62 = pfVar26[-1];
      uVar16 = *(undefined8 *)pfVar2;
      fVar8 = *pfVar26;
      fVar48 = (float)(~-(uint)(__real_3f000000 < pfVar42[uVar29]) & (uint)pfVar13[uVar29]);
      fVar59 = fVar48;
      if (__real_3d4ccccd < fVar67) {
        if (fVar48 <= __real_3d4ccccd) {
          fVar63 = (__real_3d4ccccd - fVar67) / (fVar48 - fVar67);
          if (__real_3f7fbe77 <= fVar63) {
            if (bVar19) goto LAB_14074e12b;
          }
          else {
            fVar49 = __real_3f800000 - fVar63;
            fVar65 = fVar63 * fVar8 + fVar49 * 0.0;
            fVar64 = fVar50 * fVar63 + fVar49 * fVar31;
            fVar63 = (float)((ulonglong)*(undefined8 *)pfVar2 >> 0x20) * fVar63 + fVar49 * fVar66;
            if (SQRT((fVar65 - 0.0) * (fVar65 - 0.0) +
                     (fVar63 - fVar66) * (fVar63 - fVar66) + (fVar64 - fVar31) * (fVar64 - fVar31))
                <= __real_3dcccccd) {
              if (bVar19) goto LAB_14074e01f;
            }
            else {
              fVar49 = fVar31 * (float)__xmm_00000000000000003f0000003f000000 +
                       fVar64 * (float)__xmm_00000000000000003f0000003f000000;
              fVar61 = fVar66 * __xmm_00000000000000003f0000003f000000._4_4_ +
                       fVar63 * __xmm_00000000000000003f0000003f000000._4_4_;
              fVar66 = __real_3f000000 * 0.0 + fVar65 * __real_3f000000;
              ::utils::float_grid::impl$0::
              bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>
                        ();
              ::utils::float_grid::impl$0::
              bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>
                        ();
              ::utils::float_grid::impl$0::
              bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>
                        ();
              ::utils::float_grid::impl$0::
              bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>
                        ();
              if (bVar19) {
                if (((fVar58 == fVar49) && (!NAN(fVar58) && !NAN(fVar49))) && (fVar60 == fVar61)) {
                  if ((fVar47 == fVar66) && (!NAN(fVar47) && !NAN(fVar66))) goto LAB_14074e01f;
                }
              }
              if (local_120 == local_130) {
                alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
              }
              *(ulonglong *)(lStack_128 + local_120 * 0xc) = CONCAT44(fVar61,fVar49);
              *(float *)(lStack_128 + 8 + local_120 * 0xc) = fVar66;
              local_120 = local_120 + 1;
              if (local_138 == uVar28) {
                alloc::raw_vec::RawVec::grow_one<f32,alloc::alloc::Global>();
              }
              fVar47 = extraout_XMM0_Da_15 + extraout_XMM0_Da_16 + extraout_XMM0_Da_17 +
                       extraout_XMM0_Da_18;
              fVar58 = 0.0;
              if (0.0 <= fVar47) {
                fVar58 = fVar47;
              }
              fVar47 = __real_3f800000;
              if (fVar58 <= __real_3f800000) {
                fVar47 = fVar58;
              }
              pfVar25[local_138] = fVar47;
              local_138 = local_138 + 1;
              bVar19 = true;
              fVar47 = fVar66;
              fVar58 = fVar49;
              fVar60 = fVar61;
LAB_14074e01f:
              if (((fVar58 == fVar64) && (!NAN(fVar58) && !NAN(fVar64))) && (fVar60 == fVar63)) {
                if ((fVar47 == fVar65) && (!NAN(fVar47) && !NAN(fVar65))) {
                  if (bVar19) goto LAB_14074e12b;
                  goto LAB_14074e14f;
                }
              }
            }
            if (local_120 == local_130) {
              alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
            }
            pfVar26 = (float *)(lStack_128 + local_120 * 0xc);
            *pfVar26 = fVar64;
            pfVar26[1] = fVar63;
            *(float *)(lStack_128 + 8 + local_120 * 0xc) = fVar65;
            local_120 = local_120 + 1;
            if (local_138 == uVar28) {
              alloc::raw_vec::RawVec::grow_one<f32,alloc::alloc::Global>();
            }
            pfVar25[local_138] = 0.0;
            local_138 = local_138 + 1;
            bVar19 = true;
            fVar47 = fVar65;
            fVar58 = fVar64;
            fVar60 = fVar63;
LAB_14074e12b:
            if ((fVar58 == fVar50) && (!NAN(fVar58) && !NAN(fVar50))) {
              if ((fVar60 == fVar62) && (!NAN(fVar60) && !NAN(fVar62))) {
                if ((fVar47 == fVar8) && (!NAN(fVar47) && !NAN(fVar8))) goto LAB_14074e147;
              }
            }
          }
LAB_14074e14f:
          if (local_120 == local_130) {
            alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
          }
          *(float *)(lStack_128 + local_120 * 0xc) = fVar50;
          *(float *)(lStack_128 + 4 + local_120 * 0xc) = fVar62;
          *(float *)(lStack_128 + 8 + local_120 * 0xc) = fVar8;
          local_120 = local_120 + 1;
          fVar59 = 0.0;
          if (local_138 == uVar28) {
            alloc::raw_vec::RawVec::grow_one<f32,alloc::alloc::Global>();
          }
          goto LAB_14074d4d0;
        }
        if ((float)((uint)(fVar66 - fVar62) & ___xmm_7fffffff7fffffff7fffffff7fffffff) <=
            __real_3d4ccccd) {
          if (bVar19) goto LAB_14074dda5;
        }
        else {
          fVar63 = fVar50 * (float)__xmm_00000000000000003f0000003f000000 +
                   fVar31 * (float)__xmm_00000000000000003f0000003f000000;
          fVar49 = (float)((ulonglong)*(undefined8 *)pfVar2 >> 0x20) *
                   __xmm_00000000000000003f0000003f000000._4_4_ +
                   fVar66 * __xmm_00000000000000003f0000003f000000._4_4_;
          fVar66 = fVar8 * __real_3f000000 + __real_3f000000 * 0.0;
          if (bVar19) {
            iVar43 = -(uint)(fVar58 != fVar63);
            iVar44 = -(uint)(fVar60 != fVar49);
            auVar52._4_4_ = iVar44;
            auVar52._0_4_ = iVar43;
            auVar52._8_4_ = iVar44;
            auVar52._12_4_ = iVar44;
            auVar51._8_8_ = auVar52._8_8_;
            auVar51._4_4_ = iVar43;
            auVar51._0_4_ = iVar43;
            uVar23 = movmskpd(fVar31,auVar51);
            if (((uVar23 & 1) == 0) && ((byte)((byte)uVar23 >> 1) == 0)) {
              if ((fVar47 == fVar66) && (!NAN(fVar47) && !NAN(fVar66))) goto LAB_14074dda5;
            }
          }
          if (local_120 == local_130) {
            alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
          }
          pfVar26 = (float *)(lStack_128 + local_120 * 0xc);
          *pfVar26 = fVar63;
          pfVar26[1] = fVar49;
          *(float *)(lStack_128 + 8 + local_120 * 0xc) = fVar66;
          local_120 = local_120 + 1;
          if (local_138 == uVar28) {
            alloc::raw_vec::RawVec::grow_one<f32,alloc::alloc::Global>();
          }
          pfVar25[local_138] = fVar48;
          local_138 = local_138 + 1;
          bVar19 = true;
          fVar47 = fVar66;
          fVar58 = fVar63;
          fVar60 = fVar49;
LAB_14074dda5:
          if ((fVar58 == fVar50) && (!NAN(fVar58) && !NAN(fVar50))) {
            if ((fVar60 == fVar62) && (!NAN(fVar60) && !NAN(fVar62))) {
              if ((fVar47 == fVar8) && (!NAN(fVar47) && !NAN(fVar8))) goto LAB_14074e147;
            }
          }
        }
        if (local_120 == local_130) {
          alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
        }
        *(float *)(lStack_128 + local_120 * 0xc) = fVar50;
        *(float *)(lStack_128 + 4 + local_120 * 0xc) = fVar62;
        *(float *)(lStack_128 + 8 + local_120 * 0xc) = fVar8;
        local_120 = local_120 + 1;
        if (local_138 == uVar28) {
          alloc::raw_vec::RawVec::grow_one<f32,alloc::alloc::Global>();
        }
        goto LAB_14074d4d0;
      }
      if (__real_3d4ccccd < fVar48) {
        fVar63 = (__real_3d4ccccd - fVar67) / (fVar48 - fVar67);
        if (__real_3f7fbe77 <= fVar63) {
          if (bVar19) {
LAB_14074dd85:
            if ((fVar58 != fVar50) || (NAN(fVar58) || NAN(fVar50))) goto LAB_14074df80;
LAB_14074df68:
            if ((fVar60 != fVar62) || (NAN(fVar60) || NAN(fVar62))) goto LAB_14074df80;
            if ((fVar47 != fVar8) || (NAN(fVar47) || NAN(fVar8))) goto LAB_14074df80;
LAB_14074e147:
            uVar41 = uVar29 + 1;
            goto LAB_14074d511;
          }
        }
        else {
          fVar49 = __real_3f800000 - fVar63;
          fVar64 = fVar63 * fVar50 + fVar49 * fVar31;
          fVar65 = fVar63 * fVar62 + fVar49 * fVar66;
          fVar66 = fVar63 * fVar8 + fVar49 * 0.0;
          if ((((!bVar19) || (fVar58 != fVar64)) || (NAN(fVar58) || NAN(fVar64))) ||
             (fVar60 != fVar65)) {
LAB_14074d881:
            if (local_120 == local_130) {
              alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
            }
            *(ulonglong *)(lStack_128 + local_120 * 0xc) = CONCAT44(fVar65,fVar64);
            *(float *)(lStack_128 + 8 + local_120 * 0xc) = fVar66;
            local_120 = local_120 + 1;
            if (local_138 == uVar28) {
              alloc::raw_vec::RawVec::grow_one<f32,alloc::alloc::Global>();
            }
            pfVar25[local_138] = 0.0;
            local_138 = local_138 + 1;
            bVar19 = true;
            fVar63 = fVar66;
            fVar49 = fVar64;
            fVar61 = fVar65;
          }
          else if ((fVar47 != fVar66) ||
                  (fVar63 = fVar47, fVar49 = fVar58, fVar61 = fVar60, NAN(fVar47) || NAN(fVar66)))
          goto LAB_14074d881;
          if (__real_3dcccccd <
              SQRT((fVar66 - fVar8) * (fVar66 - fVar8) +
                   (fVar65 - fVar62) * (fVar65 - fVar62) + (fVar64 - fVar50) * (fVar64 - fVar50))) {
            fVar58 = fVar50 * (float)__xmm_00000000000000003f0000003f000000 +
                     fVar64 * (float)__xmm_00000000000000003f0000003f000000;
            fVar60 = (float)((ulonglong)uVar16 >> 0x20) *
                     __xmm_00000000000000003f0000003f000000._4_4_ +
                     fVar65 * __xmm_00000000000000003f0000003f000000._4_4_;
            fVar47 = fVar8 * __real_3f000000 + fVar66 * __real_3f000000;
            ::utils::float_grid::impl$0::
            bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>
                      ();
            ::utils::float_grid::impl$0::
            bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>
                      ();
            ::utils::float_grid::impl$0::
            bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>
                      ();
            ::utils::float_grid::impl$0::
            bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>
                      ();
            if (bVar19) {
              if (((fVar49 == fVar58) && (!NAN(fVar49) && !NAN(fVar58))) && (fVar61 == fVar60)) {
                if ((fVar63 == fVar47) && (!NAN(fVar63) && !NAN(fVar47))) goto LAB_14074dab5;
              }
            }
            if (local_120 == local_130) {
              alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
            }
            *(ulonglong *)(lStack_128 + local_120 * 0xc) = CONCAT44(fVar60,fVar58);
            *(float *)(lStack_128 + 8 + local_120 * 0xc) = fVar47;
            local_120 = local_120 + 1;
            if (local_138 == uVar28) {
              alloc::raw_vec::RawVec::grow_one<f32,alloc::alloc::Global>();
            }
            fVar63 = extraout_XMM0_Da_11 + extraout_XMM0_Da_12 + extraout_XMM0_Da_13 +
                     extraout_XMM0_Da_14;
            fVar66 = 0.0;
            if (0.0 <= fVar63) {
              fVar66 = fVar63;
            }
            fVar63 = __real_3f800000;
            if (fVar66 <= __real_3f800000) {
              fVar63 = fVar66;
            }
            pfVar25[local_138] = fVar63;
            local_138 = local_138 + 1;
            bVar19 = true;
            if ((fVar58 == fVar50) && (!NAN(fVar58) && !NAN(fVar50))) goto LAB_14074df68;
          }
          else {
LAB_14074dab5:
            fVar47 = fVar63;
            fVar58 = fVar49;
            fVar60 = fVar61;
            if (bVar19) goto LAB_14074dd85;
          }
        }
LAB_14074df80:
        if (local_120 == local_130) {
          alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
        }
        *(float *)(lStack_128 + local_120 * 0xc) = fVar50;
        *(float *)(lStack_128 + 4 + local_120 * 0xc) = fVar62;
        *(float *)(lStack_128 + 8 + local_120 * 0xc) = fVar8;
        local_120 = local_120 + 1;
        if (local_138 == uVar28) {
          alloc::raw_vec::RawVec::grow_one<f32,alloc::alloc::Global>();
        }
        goto LAB_14074d4d0;
      }
      if ((((!bVar19) || (fVar58 != fVar50)) || (NAN(fVar58) || NAN(fVar50))) ||
         ((fVar60 != fVar62 || (NAN(fVar60) || NAN(fVar62))))) goto LAB_14074d5ce;
      pfVar26 = pfVar26 + 3;
      if ((fVar47 != fVar8) ||
         (uVar41 = uVar29 + 1, fVar67 = fVar48, fVar31 = fVar50, fVar66 = fVar62,
         NAN(fVar47) || NAN(fVar8))) goto LAB_14074d5ce;
    }
    if (local_138 == 0) {
      local_138 = 0;
      unaff_XMM10_Da = fVar47;
    }
    else {
      pfVar12 = (float *)(lVar39 + -0x18);
      uVar41 = local_138 - 1 & 0x3fffffffffffffff;
      pfVar42 = pfVar25;
      fVar58 = __real_c0a00000;
      if ((2 < uVar41) &&
         ((float *)(uVar38 * -0x2a8 + lVar10 + -0x14) <= pfVar25 || pfVar25 + local_138 <= pfVar12))
      {
        uVar41 = uVar41 + 1;
        uVar29 = uVar41 & 0xfffffffffffffffc;
        fVar58 = *pfVar12;
        uVar38 = 0;
        do {
          pfVar42 = pfVar25 + uVar38;
          fVar62 = *pfVar42;
          fVar48 = pfVar42[1];
          fVar50 = pfVar42[2];
          fVar47 = expf(pfVar42[3] * fVar22);
          fVar50 = expf(fVar50 * fVar21);
          fVar62 = expf(fVar62 * fVar61);
          fVar48 = expf(fVar48 * fVar20);
          pfVar42 = pfVar25 + uVar38;
          *pfVar42 = (fVar63 - fVar62) * fVar58;
          pfVar42[1] = (fVar49 - fVar48) * fVar58;
          pfVar42[2] = (fVar64 - fVar50) * fVar58;
          pfVar42[3] = (fVar65 - fVar47) * fVar58;
          uVar38 = uVar38 + 4;
        } while (uVar29 != uVar38);
        pfVar42 = pfVar25 + uVar29;
        fVar58 = __real_c0a00000;
        if (uVar41 == uVar29) goto LAB_14074e2d9;
      }
      do {
        pfVar13 = pfVar42 + 1;
        fVar62 = expf(*pfVar42 * fVar58);
        *pfVar42 = (unaff_XMM10_Da - fVar62) * *pfVar12;
        pfVar42 = pfVar13;
      } while (pfVar13 != pfVar25 + local_138);
    }
LAB_14074e2d9:
    ::utils::curve::Curve::
    crashy_new_from_points<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemanticThresholded3D>();
    if (*(longlong *)(lVar39 + -0x218) != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if (*(longlong *)(lVar39 + -0x200) != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    *(undefined8 *)(lVar39 + -0x1e8) = local_218;
    *(undefined8 *)(lVar39 + -0x1f8) = local_228;
    *(undefined8 *)(lVar39 + -0x1f0) = uStack_220;
    *(undefined8 *)(lVar39 + -0x208) = local_238;
    *(undefined8 *)(lVar39 + -0x200) = uStack_230;
    *(undefined8 *)(lVar39 + -0x218) = local_248;
    *(undefined8 *)(lVar39 + -0x210) = uStack_240;
    if (*(longlong *)(lVar39 + -0x1e0) != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    *(ulonglong *)(lVar39 + -0x1e0) = uVar28;
    *(float **)(lVar39 + -0x1d8) = pfVar25;
    *(ulonglong *)(lVar39 + -0x1d0) = local_138;
    if (lVar40 != 0) goto LAB_14074caa7;
LAB_14074cae0:
    if ((lVar24 == 0) || (lVar24 == lVar32)) {
      if (extraout_AL == '\0') {
        return;
      }
      puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
      if (*extraout_RAX_05 == 1) {
        plVar27 = extraout_RAX_05 + 1;
      }
      else if ((*extraout_RAX_05 != 0) ||
              (std::sys::thread_local::native::lazy::Storage::
               initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                         (), plVar27 = extraout_RAX_07, extraout_RAX_07 == (longlong *)0x0))
      goto LAB_14074e675;
      if (*plVar27 == 0) {
        *plVar27 = -1;
        puffin::ThreadProfiler::end_scope();
        *plVar27 = *plVar27 + 1;
        return;
      }
      goto LAB_14074e681;
    }
    local_1c0 = 0;
    *puVar9 = *puVar9 + 1;
    lVar10 = plVar27[3];
    lVar34 = lVar24 + 0x10;
    lVar11 = lVar24;
  } while( true );
LAB_14074d5ce:
  if (local_120 == local_130) {
    alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
  }
  *(float *)(lStack_128 + local_120 * 0xc) = fVar50;
  *(float *)(lStack_128 + 4 + local_120 * 0xc) = fVar62;
  *(float *)(lStack_128 + 8 + local_120 * 0xc) = fVar8;
  local_120 = local_120 + 1;
  fVar59 = 0.0;
  if (local_138 == uVar28) {
    alloc::raw_vec::RawVec::grow_one<f32,alloc::alloc::Global>();
  }
LAB_14074d4d0:
  pfVar25[local_138] = fVar59;
  local_138 = local_138 + 1;
  bVar19 = true;
  uVar41 = uVar29 + 1;
  fVar47 = fVar8;
  fVar58 = fVar50;
  fVar60 = fVar62;
  goto LAB_14074d511;
}

