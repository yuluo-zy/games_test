// 0x140ad6e00 do_ui_edit_rectangle_dims size=1649 sig=void __cdecl do_ui_edit_rectangle_dims(void)
void __cdecl
country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims
          (void)

{
  byte *pbVar1;
  undefined8 *puVar2;
  char *pcVar3;
  undefined4 uVar4;
  byte bVar5;
  byte bVar6;
  undefined4 uVar7;
  undefined4 *puVar8;
  longlong lVar9;
  ulonglong uVar10;
  uint uVar11;
  uint uVar12;
  undefined1 *puVar13;
  int iVar14;
  char extraout_AL;
  char extraout_AL_00;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  undefined *puVar15;
  longlong *extraout_RAX_01;
  ulonglong uVar16;
  int *piVar17;
  longlong *extraout_RAX_02;
  ulonglong extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  longlong *extraout_RAX_07;
  uint uVar18;
  longlong *in_RCX;
  ulonglong uVar19;
  longlong lVar20;
  longlong *plVar21;
  longlong *in_RDX;
  ulonglong uVar22;
  ulonglong uVar23;
  ulonglong uVar24;
  ulonglong uVar25;
  ulonglong uVar26;
  int *piVar27;
  longlong lVar28;
  undefined8 *in_R8;
  longlong lVar29;
  longlong *in_R9;
  ushort uVar30;
  longlong lVar31;
  longlong lVar32;
  ulonglong uVar33;
  ulonglong uVar34;
  undefined8 uVar35;
  bool bVar36;
  bool bVar37;
  byte bVar38;
  char cVar39;
  char cVar44;
  undefined4 extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  float fVar40;
  char cVar42;
  char cVar43;
  float fVar45;
  undefined1 auVar41 [16];
  float fVar46;
  float fVar47;
  float fVar48;
  float fVar49;
  float fVar50;
  float fVar52;
  undefined1 auVar51 [16];
  undefined1 auVar53 [16];
  float fVar54;
  longlong *in_stack_00000028;
  undefined8 *in_stack_00000038;
  longlong *in_stack_00000040;
  longlong *in_stack_00000048;
  undefined8 *in_stack_00000050;
  int iStack_27c;
  float fStack_26c;
  undefined8 local_244;
  undefined8 uStack_23c;
  int local_234;
  undefined1 local_22c [12];
  undefined8 local_220;
  uint uStack_20c;
  undefined4 uStack_208;
  undefined4 uStack_204;
  undefined8 uStack_200;
  undefined8 uStack_1f8;
  longlong *local_1e0;
  longlong *local_1d8;
  undefined4 *local_1d0;
  undefined4 *local_1c8;
  longlong *local_1c0;
  undefined8 uStack_1b8;
  undefined4 uStack_1b0;
  longlong local_1a8;
  undefined8 *local_1a0;
  undefined8 local_198;
  undefined4 local_190;
  undefined4 local_18c;
  longlong local_188;
  longlong local_180;
  undefined4 uStack_178;
  float fStack_174;
  float fStack_170;
  float fStack_16c;
  undefined4 uStack_168;
  undefined4 uStack_164;
  undefined4 uStack_160;
  undefined8 uStack_158;
  undefined8 uStack_150;
  undefined8 uStack_148;
  longlong *local_140;
  undefined8 local_138;
  undefined8 uStack_130;
  undefined8 local_128;
  undefined8 local_118;
  undefined8 uStack_110;
  undefined8 uStack_108;
  undefined8 uStack_100;
  longlong lStack_f8;
  longlong *local_e8;
  undefined8 local_e0;
  
  local_e0 = 0xfffffffffffffffe;
  local_198 = *in_stack_00000038;
  local_180 = in_stack_00000038[4];
  local_1c8 = (undefined4 *)in_stack_00000038[6];
  local_18c = *(undefined4 *)((longlong)in_stack_00000038 + 0x3c);
  local_188 = in_stack_00000038[8];
  puVar8 = (undefined4 *)in_stack_00000038[10];
  uVar7 = *(undefined4 *)((longlong)in_stack_00000038 + 0x5c);
  local_1c0 = in_R9;
  local_1a0 = in_R8;
  puffin::are_scopes_on();
  local_e8 = (longlong *)CONCAT71(local_e8._1_7_,extraout_AL);
  local_1e0 = in_RCX;
  local_1d8 = in_RDX;
  local_1d0 = puVar8;
  local_190 = uVar7;
  if (extraout_AL == '\0') {
    local_220 = 0;
  }
  else {
    uVar22 = 0x61;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar22 != 0) {
        if (uVar22 < 0x61) {
          if ((char)(&DAT_142ae11bf)[uVar22] < -0x40) {
LAB_140ad6f1c:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar22 != 0x61) goto LAB_140ad6f1c;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar22 != 0xfffffffffffffffe)) {
        if (uVar22 + 2 < 0x61) {
          if ((char)(&UNK_142ae11c1)[uVar22] < -0x40) {
LAB_140ad6f7a:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar22 + 2 != 0x61) goto LAB_140ad6f7a;
      }
    }
    puVar15 = &DAT_142ae17ff;
    do {
      bVar38 = puVar15[-1];
      uVar18 = (uint)(char)bVar38;
      if ((char)bVar38 < '\0') {
        bVar5 = puVar15[-2];
        if ((char)bVar5 < -0x40) {
          bVar6 = puVar15[-3];
          if ((char)bVar6 < -0x40) {
            pbVar1 = puVar15 + -4;
            puVar15 = puVar15 + -4;
            uVar18 = bVar6 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar15 = puVar15 + -3;
            uVar18 = bVar6 & 0xf;
          }
          uVar18 = bVar5 & 0x3f | uVar18 << 6;
        }
        else {
          puVar15 = puVar15 + -2;
          uVar18 = bVar5 & 0x1f;
        }
        uVar18 = (uint)(bVar38 & 0x3f) | uVar18 << 6;
        if (uVar18 != 0x5c) goto LAB_140ad7039;
LAB_140ad704e:
        if (puVar15 == &UNK_142ae17af) break;
        if (puVar15 + -0x142ae17af < (undefined *)0x4f) {
          if (-0x41 < (char)puVar15[1]) break;
        }
        else if (puVar15 + -0x142ae17af == (undefined *)0x4f) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar15 = puVar15 + -1;
      if (uVar18 == 0x5c) goto LAB_140ad704e;
LAB_140ad7039:
      if (uVar18 == 0x2f) goto LAB_140ad704e;
    } while (puVar15 != &DAT_142ae17b0);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      local_140 = extraout_RAX_01 + 1;
    }
    else {
      if (*extraout_RAX_01 != 0) goto LAB_140ad7c1b;
      std::sys::thread_local::native::lazy::Storage::
      initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                ();
      local_140 = extraout_RAX_05;
      if (extraout_RAX_05 == (longlong *)0x0) goto LAB_140ad7c1b;
    }
    if (*local_140 != 0) goto LAB_140ad7c27;
    *local_140 = -1;
    puffin::ThreadProfiler::begin_scope();
    *local_140 = *local_140 + 1;
    local_220 = 1;
  }
  lVar9 = in_stack_00000050[1];
  uVar22 = *(ulonglong *)*in_stack_00000050;
  uVar33 = 0;
  uVar19 = uVar22 - *(ulonglong *)(lVar9 + 0x18);
  if (uVar22 < *(ulonglong *)(lVar9 + 0x18)) {
    uVar19 = uVar33;
  }
  uVar16 = uVar22 - *(ulonglong *)(lVar9 + 0x38);
  if (uVar22 < *(ulonglong *)(lVar9 + 0x38)) {
    uVar16 = uVar33;
  }
  uVar22 = *(ulonglong *)(lVar9 + 0x10);
  uVar10 = *(ulonglong *)(lVar9 + 0x30);
  bVar36 = uVar10 < uVar16;
  uVar23 = uVar10 - uVar16;
  uVar24 = uVar23;
  if (bVar36) {
    uVar24 = uVar33;
  }
  bVar37 = uVar22 < uVar19;
  uVar25 = uVar22 - uVar19;
  uVar26 = uVar25;
  if (bVar37) {
    uVar26 = uVar33;
  }
  lVar32 = *(longlong *)(lVar9 + 8);
  uVar34 = uVar24 + uVar26;
  if (!bVar37 && uVar25 != 0 || !bVar36 && uVar23 != 0) {
    uVar34 = uVar33;
  }
  lVar31 = *(longlong *)(lVar9 + 0x28);
  *(ulonglong *)*in_stack_00000050 = *(longlong *)(lVar9 + 0x40) - uVar34;
  if (!bVar37 && uVar25 != 0 || !bVar36 && uVar23 != 0) {
    lVar31 = lVar31 + uVar16 * 0x70;
    lVar32 = lVar32 + uVar19 * 0x70;
    if (uVar22 < uVar19) {
      lVar32 = 8;
    }
    piVar27 = (int *)0x0;
    if (uVar19 < uVar22) {
      piVar27 = (int *)(lVar32 + -0x70 + uVar26 * 0x70);
    }
    if (uVar10 < uVar16) {
      lVar31 = 8;
    }
    piVar17 = (int *)(lVar31 + -0x70 + uVar24 * 0x70);
    if (uVar10 <= uVar16) {
      piVar17 = piVar27;
    }
    if (*piVar17 != 2) {
      iVar14 = piVar17[0x18];
      lVar9 = *(longlong *)(piVar17 + 0xe);
      lVar32 = *(longlong *)(piVar17 + 0x16);
      local_244 = *(undefined8 *)(piVar17 + 9);
      uStack_23c = *(undefined8 *)(piVar17 + 0xb);
      local_234 = piVar17[0xd];
      local_138 = *(undefined8 *)(piVar17 + 0x10);
      uStack_130 = *(undefined8 *)(piVar17 + 0x12);
      local_128 = *(undefined8 *)(piVar17 + 0x14);
      plVar21 = (longlong *)*local_1e0;
      local_118._0_4_ = (undefined4)lVar9;
      local_118._4_4_ = (float)((ulonglong)lVar9 >> 0x20);
      local_1a8 = lVar32;
      if (plVar21[3] != 0) {
        core::hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
        lVar31 = *plVar21;
        bVar38 = (byte)(extraout_RAX_03 >> 0x39);
        auVar41 = ZEXT216(CONCAT11(bVar38,bVar38));
        auVar41 = pshuflw(auVar41,auVar41,0);
        lVar29 = 0;
        uVar22 = extraout_RAX_03;
        do {
          uVar22 = plVar21[1] & uVar22;
          pcVar3 = (char *)(lVar31 + uVar22);
          cVar39 = auVar41[0];
          auVar53[0] = -(*pcVar3 == cVar39);
          cVar42 = auVar41[1];
          auVar53[1] = -(pcVar3[1] == cVar42);
          cVar43 = auVar41[2];
          auVar53[2] = -(pcVar3[2] == cVar43);
          cVar44 = auVar41[3];
          auVar53[3] = -(pcVar3[3] == cVar44);
          auVar53[4] = -(pcVar3[4] == cVar39);
          auVar53[5] = -(pcVar3[5] == cVar42);
          auVar53[6] = -(pcVar3[6] == cVar43);
          auVar53[7] = -(pcVar3[7] == cVar44);
          auVar53[8] = -(pcVar3[8] == cVar39);
          auVar53[9] = -(pcVar3[9] == cVar42);
          auVar53[10] = -(pcVar3[10] == cVar43);
          auVar53[0xb] = -(pcVar3[0xb] == cVar44);
          auVar53[0xc] = -(pcVar3[0xc] == cVar39);
          auVar53[0xd] = -(pcVar3[0xd] == cVar42);
          auVar53[0xe] = -(pcVar3[0xe] == cVar43);
          auVar53[0xf] = -(pcVar3[0xf] == cVar44);
          uVar30 = (ushort)(SUB161(auVar53 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar53 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar53 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar53 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar53 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar53 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar53 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar53 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar53 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar53 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar53 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar53 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar53 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar53 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar53 >> 0x77,0) & 1) << 0xe |
                   (ushort)(auVar53[0xf] >> 7) << 0xf;
          uVar18 = (uint)uVar30;
          if (uVar30 != 0) {
LAB_140ad7349:
            uVar11 = 0;
            for (uVar12 = uVar18; (uVar12 & 1) == 0; uVar12 = uVar12 >> 1 | 0x80000000) {
              uVar11 = uVar11 + 1;
            }
            lVar28 = (uVar11 + uVar22 & plVar21[1]) * -0x2a8;
            if (*(longlong *)(lVar31 + -0x2a8 + lVar28) != lVar9) goto code_r0x000140ad7364;
            if ((*(ulonglong *)(lVar31 + -0x290 + lVar28) < 2) ||
               (fVar46 = 0.0, *(float *)(lVar31 + -0x270 + lVar28) <= 0.0)) {
                    // WARNING: Subroutine does not return
              core::panicking::panic();
            }
            ::utils::geometry::rectangle::Rectangle2d::get_edge_normal();
            local_118._0_4_ = extraout_XMM0_Da;
            local_118._4_4_ = fVar46;
            ::utils::swizzlers::impl$0::x0y();
            lVar31 = lVar31 + lVar28;
            switch((char)iVar14) {
            case '\x02':
              break;
            case '\x03':
              *(undefined4 *)local_1d8[2] = *(undefined4 *)((longlong)local_1d8 + 0x1c);
              *(undefined1 *)(*local_1d8 + 0x1e0) = 1;
              uVar7 = local_244._4_4_;
              ::utils::geometry::rectangle::Rectangle2d::get_edge_normal();
              fVar40 = (float)local_244;
              fVar45 = (float)uStack_23c;
              uStack_108._0_4_ = (undefined4)local_128;
              uStack_108._4_4_ = (undefined4)((ulonglong)local_128 >> 0x20);
              local_118._0_4_ = (undefined4)local_138;
              local_118._4_4_ = (float)((ulonglong)local_138 >> 0x20);
              uStack_110._0_4_ = (float)uStack_130;
              uStack_110._4_4_ = (float)((ulonglong)uStack_130 >> 0x20);
              fVar47 = fVar46;
              ::utils::geometry::rectangle::Rectangle2d::get_point();
              fVar48 = fVar47;
              ::utils::geometry::rectangle::Rectangle2d::get_point();
              fVar49 = fVar48;
              ::utils::geometry::rectangle::Rectangle2d::get_edge_normal();
              fVar50 = extraout_XMM0_Da_02 - extraout_XMM0_Da_01;
              fVar52 = fVar48 - fVar47;
              fVar54 = fVar52 * fVar52 + fVar50 * fVar50;
              if (fVar54 <= 1e-20) {
                fVar54 = 1e-20;
              }
              fVar54 = ((fVar45 - fVar47) * fVar52 + (fVar40 - extraout_XMM0_Da_01) * fVar50) /
                       fVar54;
              local_118._0_4_ = (undefined4)local_138;
              local_118._4_4_ = (float)((ulonglong)local_138 >> 0x20);
              uStack_110._0_4_ = (float)uStack_130;
              uStack_110._4_4_ = (float)((ulonglong)uStack_130 >> 0x20);
              uStack_108._0_4_ = (undefined4)local_128;
              uStack_108._4_4_ = (undefined4)((ulonglong)local_128 >> 0x20);
              *(undefined4 *)local_1a0[2] = *(undefined4 *)((longlong)local_1a0 + 0x1c);
              puVar8 = (undefined4 *)*local_1a0;
              *puVar8 = 2;
              puVar8[1] = uVar7;
              *(longlong *)(puVar8 + 2) = lVar9;
              *(longlong *)(puVar8 + 4) = lVar32;
              *(undefined8 *)(puVar8 + 6) = local_138;
              *(undefined8 *)(puVar8 + 8) = uStack_130;
              *(undefined8 *)(puVar8 + 10) = local_128;
              *(undefined8 *)(puVar8 + 0xc) = local_138;
              *(undefined8 *)(puVar8 + 0xe) = uStack_130;
              *(undefined8 *)(puVar8 + 0x10) = local_128;
              fVar40 = fVar49 * (fVar45 - ((1.0 - fVar54) * fVar47 + fVar54 * fVar48)) +
                       extraout_XMM0_Da_03 *
                       (fVar40 - ((1.0 - fVar54) * extraout_XMM0_Da_01 +
                                 fVar54 * extraout_XMM0_Da_02));
              *(ulonglong *)(puVar8 + 0x12) = CONCAT44(fVar40 * fVar46,fVar40 * extraout_XMM0_Da_00)
              ;
              uStack_178 = (undefined4)local_118;
              fStack_174 = local_118._4_4_;
              fStack_170 = (float)uStack_110;
              fStack_16c = uStack_110._4_4_;
              uStack_168 = (undefined4)uStack_108;
              uStack_164 = uStack_108._4_4_;
              break;
            case '\x04':
              if ((*(byte *)(lVar31 + -0x198) & 1) == 0) {
                local_118._0_4_ = 0x42aeaea0;
                local_118._4_4_ = 1.4013e-45;
                uStack_110._0_4_ = 1.4013e-45;
                uStack_110._4_4_ = 0.0;
                uStack_108._0_4_ = 8;
                uStack_108._4_4_ = 0;
                uStack_100._0_4_ = 0;
                uStack_100._4_4_ = 0;
                lStack_f8 = 0;
                    // WARNING: Subroutine does not return
                core::panicking::panic_fmt();
              }
              uVar7 = *(undefined4 *)(lVar31 + -0x20);
              uVar4 = *(undefined4 *)(lVar31 + -0x194);
              uStack_108._3_1_ = (undefined1)local_128;
              uStack_108._4_4_ = (undefined4)((ulonglong)local_128 >> 8);
              uStack_100._0_3_ = (undefined3)((ulonglong)local_128 >> 0x28);
              local_118._3_1_ = (undefined1)local_138;
              local_118._4_4_ = (float)((ulonglong)local_138 >> 8);
              uStack_110._0_3_ = (undefined3)((ulonglong)local_138 >> 0x28);
              uStack_110._3_1_ = (undefined1)uStack_130;
              uStack_110._4_4_ = (float)((ulonglong)uStack_130 >> 8);
              uStack_108._0_3_ = (undefined3)((ulonglong)uStack_130 >> 0x28);
              *(undefined4 *)in_stack_00000048[2] =
                   *(undefined4 *)((longlong)in_stack_00000048 + 0x1c);
              lVar32 = *in_stack_00000048;
              lVar29 = *(longlong *)(lVar32 + 0x30);
              uVar35 = *(undefined8 *)(lVar32 + 0x40);
              if (lVar29 == *(longlong *)(lVar32 + 0x20)) {
                alloc::raw_vec::RawVec::
                grow_one<country_core::resources::window::AvailableMonitor,alloc::alloc::Global>();
              }
              lVar28 = *(longlong *)(lVar32 + 0x28);
              lVar20 = lVar29 * 0x40;
              *(undefined1 *)(lVar28 + lVar20) = 1;
              puVar2 = (undefined8 *)(lVar28 + 1 + lVar20);
              *puVar2 = CONCAT44(local_118._4_4_,(undefined4)local_118);
              puVar2[1] = CONCAT44(uStack_110._4_4_,(float)uStack_110);
              puVar2 = (undefined8 *)(lVar28 + 0xc + lVar20);
              *puVar2 = CONCAT35((undefined3)uStack_108,CONCAT41(uStack_110._4_4_,uStack_110._3_1_))
              ;
              puVar2[1] = CONCAT35((undefined3)uStack_100,
                                   CONCAT41(uStack_108._4_4_,uStack_108._3_1_));
              *(undefined4 *)(lVar28 + 0x1c + lVar20) = uVar7;
              *(undefined4 *)(lVar28 + 0x20 + lVar20) = uVar4;
              *(undefined8 *)(lVar28 + 0x28 + lVar20) = 1;
              *(longlong *)(lVar28 + 0x30 + lVar20) = lVar9;
              *(undefined8 *)(lVar28 + 0x38 + lVar20) = uVar35;
              *(longlong *)(lVar32 + 0x30) = lVar29 + 1;
              *(longlong *)(lVar32 + 0x40) = *(longlong *)(lVar32 + 0x40) + 1;
              uStack_108._0_4_ = (undefined4)local_128;
              uStack_108._4_4_ = (undefined4)((ulonglong)local_128 >> 0x20);
              local_118._0_4_ = (undefined4)local_138;
              local_118._4_4_ = (float)((ulonglong)local_138 >> 0x20);
              uStack_110._0_4_ = (float)uStack_130;
              uStack_110._4_4_ = (float)((ulonglong)uStack_130 >> 0x20);
              uVar7 = *(undefined4 *)(lVar31 + -0x20);
              *(undefined4 *)in_stack_00000040[2] =
                   *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
              lVar32 = *in_stack_00000040;
              lVar31 = *(longlong *)(lVar32 + 0x30);
              uVar35 = *(undefined8 *)(lVar32 + 0x40);
              if (lVar31 == *(longlong *)(lVar32 + 0x20)) {
                alloc::raw_vec::RawVec::
                grow_one<country_core::render::render_config::LastKnownGraphicsDevice,alloc::alloc::Global>
                          ();
              }
              lVar29 = *(longlong *)(lVar32 + 0x28);
              lVar28 = lVar31 * 0x30;
              *(undefined4 *)(lVar29 + lVar28) = 1;
              puVar2 = (undefined8 *)(lVar29 + 4 + lVar28);
              *puVar2 = CONCAT44(local_118._4_4_,(undefined4)local_118);
              puVar2[1] = CONCAT44(uStack_110._4_4_,(float)uStack_110);
              *(ulonglong *)(lVar29 + 0x14 + lVar28) =
                   CONCAT44(uStack_108._4_4_,(undefined4)uStack_108);
              *(undefined4 *)(lVar29 + 0x1c + lVar28) = uVar7;
              *(undefined4 *)(lVar29 + 0x20 + lVar28) = 0x3fc00000;
              *(undefined1 *)(lVar29 + 0x24 + lVar28) = 0;
              goto code_r0x000140ad77d6;
            default:
              *(undefined4 *)in_stack_00000028[2] =
                   *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
              lVar32 = *in_stack_00000028;
              lVar29 = *(longlong *)(lVar32 + 0x30);
              uVar35 = *(undefined8 *)(lVar32 + 0x40);
              if (lVar29 == *(longlong *)(lVar32 + 0x20)) {
                alloc::raw_vec::RawVec::
                grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                          ();
              }
              *(undefined8 *)(*(longlong *)(lVar32 + 0x28) + lVar29 * 8) = uVar35;
              *(longlong *)(lVar32 + 0x30) = lVar29 + 1;
              *(longlong *)(lVar32 + 0x40) = *(longlong *)(lVar32 + 0x40) + 1;
              uStack_108._0_4_ = (undefined4)local_128;
              uStack_108._4_4_ = (undefined4)((ulonglong)local_128 >> 0x20);
              local_118._0_4_ = (undefined4)local_138;
              local_118._4_4_ = (float)((ulonglong)local_138 >> 0x20);
              uStack_110._0_4_ = (float)uStack_130;
              uStack_110._4_4_ = (float)((ulonglong)uStack_130 >> 0x20);
              uVar7 = *(undefined4 *)(lVar31 + -0x20);
              *(undefined4 *)in_stack_00000040[2] =
                   *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
              lVar32 = *in_stack_00000040;
              lVar31 = *(longlong *)(lVar32 + 0x30);
              uVar35 = *(undefined8 *)(lVar32 + 0x40);
              if (lVar31 == *(longlong *)(lVar32 + 0x20)) {
                alloc::raw_vec::RawVec::
                grow_one<country_core::render::render_config::LastKnownGraphicsDevice,alloc::alloc::Global>
                          ();
              }
              lVar29 = *(longlong *)(lVar32 + 0x28);
              lVar28 = lVar31 * 0x30;
              *(undefined4 *)(lVar29 + lVar28) = 1;
              puVar2 = (undefined8 *)(lVar29 + 4 + lVar28);
              *puVar2 = CONCAT44(local_118._4_4_,(undefined4)local_118);
              puVar2[1] = CONCAT44(uStack_110._4_4_,(float)uStack_110);
              *(ulonglong *)(lVar29 + 0x14 + lVar28) =
                   CONCAT44(uStack_108._4_4_,(undefined4)uStack_108);
              *(undefined4 *)(lVar29 + 0x1c + lVar28) = uVar7;
              *(undefined4 *)(lVar29 + 0x20 + lVar28) = 0x3fc00000;
              *(undefined1 *)(lVar29 + 0x24 + lVar28) = 1;
code_r0x000140ad77d6:
              *(undefined8 *)(lVar29 + 0x28 + lVar28) = uVar35;
              *(longlong *)(lVar32 + 0x30) = lVar31 + 1;
              *(longlong *)(lVar32 + 0x40) = *(longlong *)(lVar32 + 0x40) + 1;
            }
            *(undefined4 *)local_1a0[2] = *(undefined4 *)((longlong)local_1a0 + 0x1c);
            piVar27 = (int *)*local_1a0;
            puVar13 = (undefined1 *)CONCAT44(uStack_110._4_4_,(float)uStack_110);
            if (*piVar27 == 2) {
              local_118 = &local_244;
              uStack_100 = &local_128;
              lStack_f8 = (longlong)&local_128 + 4;
              uStack_110 = local_22c;
              uStack_108 = &local_1a8;
              uStack_178 = 0x40100000;
              fStack_174 = 15.0;
              fStack_170 = (float)((uint)fStack_170 & 0xffffff00);
              systems::ui::resources::slider3d::Slider3d::
              update<enum2_<country_core::systems::ui::resources::ui_state::DragState>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims::closure_env_0>
                        ();
              puVar13 = uStack_110;
              if (iStack_27c == 1) {
                uStack_148 = local_128;
                uStack_158 = local_138;
                uStack_150 = uStack_130;
                fVar46 = *(float *)((longlong)&uStack_148 +
                                   (ulonglong)((local_1a8 - 1U & 0xfffffffffffffffd) == 0) * 4);
                ::utils::geometry::rectangle::Rectangle2d::expand_along_edge();
                uStack_108._4_4_ = (undefined4)uStack_148;
                uStack_100._0_4_ = (undefined4)((ulonglong)uStack_148 >> 0x20);
                local_118._4_4_ = (float)uStack_158;
                uStack_110._0_4_ = (float)((ulonglong)uStack_158 >> 0x20);
                uStack_110._4_4_ = (float)uStack_150;
                uStack_108._0_4_ = (undefined4)((ulonglong)uStack_150 >> 0x20);
                local_118._0_4_ = 1;
                resources::glade_border::GladeBorder::is_shape_inside();
                if (extraout_AL_00 == '\0') {
                  *local_1c8 = local_18c;
                  lVar32 = *(longlong *)(local_180 + 0x30);
                  uVar35 = *(undefined8 *)(local_180 + 0x40);
                  if (lVar32 == *(longlong *)(local_180 + 0x20)) {
                    alloc::raw_vec::RawVec::
                    grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                              ();
                  }
                  *(undefined8 *)(*(longlong *)(local_180 + 0x28) + lVar32 * 8) = uVar35;
                  *(longlong *)(local_180 + 0x30) = lVar32 + 1;
                  *(longlong *)(local_180 + 0x40) = *(longlong *)(local_180 + 0x40) + 1;
                  *local_1d0 = local_190;
                  lVar32 = *(longlong *)(local_188 + 0x30);
                  uVar35 = *(undefined8 *)(local_188 + 0x40);
                  if (lVar32 == *(longlong *)(local_188 + 0x20)) {
                    alloc::raw_vec::RawVec::
                    grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                              ();
                  }
                  *(undefined8 *)(*(longlong *)(local_188 + 0x28) + lVar32 * 8) = uVar35;
                  *(longlong *)(local_188 + 0x30) = lVar32 + 1;
                  *(longlong *)(local_188 + 0x40) = *(longlong *)(local_188 + 0x40) + 1;
                  uStack_164 = (undefined4)*(undefined8 *)(piVar27 + 0x10);
                  uStack_160 = (undefined4)((ulonglong)*(undefined8 *)(piVar27 + 0x10) >> 0x20);
                  fStack_174 = (float)*(undefined8 *)(piVar27 + 0xc);
                  fStack_170 = (float)((ulonglong)*(undefined8 *)(piVar27 + 0xc) >> 0x20);
                  fStack_16c = (float)*(undefined8 *)(piVar27 + 0xe);
                  uStack_168 = (undefined4)((ulonglong)*(undefined8 *)(piVar27 + 0xe) >> 0x20);
                  uStack_178 = 1;
                  uStack_108._4_4_ = (undefined4)uStack_148;
                  uStack_100._0_4_ = (undefined4)((ulonglong)uStack_148 >> 0x20);
                  local_118._4_4_ = (float)uStack_158;
                  uStack_110._0_4_ = (float)((ulonglong)uStack_158 >> 0x20);
                  uStack_110._4_4_ = (float)uStack_150;
                  uStack_108._0_4_ = (undefined4)((ulonglong)uStack_150 >> 0x20);
                  local_118._0_4_ = 1;
                  resources::glade_border::GladeBorder::search_lerped_shape_that_fits_into_glade();
                  if ((uStack_20c & 1) == 0) {
                    local_118._0_4_ = 0x42c19950;
                    local_118._4_4_ = 1.4013e-45;
                    uStack_110._0_4_ = 1.4013e-45;
                    uStack_110._4_4_ = 0.0;
                    uStack_108._0_4_ = 8;
                    uStack_108._4_4_ = 0;
                    uStack_100._0_4_ = 0;
                    uStack_100._4_4_ = 0;
                    lStack_f8 = 0;
                    // WARNING: Subroutine does not return
                    core::panicking::panic_fmt();
                  }
                  uStack_148 = uStack_1f8;
                  uStack_158 = CONCAT44(uStack_204,uStack_208);
                  uStack_150 = uStack_200;
                }
                puVar13 = (undefined1 *)CONCAT44(uStack_110._4_4_,(float)uStack_110);
                uStack_110._0_4_ = (float)uStack_150;
                uStack_110._4_4_ = (float)((ulonglong)uStack_150 >> 0x20);
                fVar40 = (float)uStack_110 - (float)*(undefined8 *)(piVar27 + 0xe);
                fVar45 = uStack_110._4_4_ -
                         (float)((ulonglong)*(undefined8 *)(piVar27 + 0xe) >> 0x20);
                if (0.0 < SQRT(fVar45 * fVar45 + fVar40 * fVar40)) {
                  *(undefined8 *)(piVar27 + 0x10) = uStack_148;
                  *(undefined8 *)(piVar27 + 0xc) = uStack_158;
                  *(undefined8 *)(piVar27 + 0xe) = uStack_150;
                  uStack_108._0_4_ = (undefined4)uStack_148;
                  uStack_108._4_4_ = (undefined4)((ulonglong)uStack_148 >> 0x20);
                  local_118._0_4_ = (undefined4)uStack_158;
                  local_118._4_4_ = (float)((ulonglong)uStack_158 >> 0x20);
                  ::utils::swizzlers::impl$1::xvy();
                  uStack_168 = (undefined4)uStack_108;
                  uStack_164 = uStack_108._4_4_;
                  uStack_178 = (undefined4)local_118;
                  fStack_174 = local_118._4_4_;
                  fStack_170 = (float)uStack_110;
                  fStack_16c = uStack_110._4_4_;
                  uStack_1b8 = CONCAT44(uStack_208,uStack_20c);
                  uStack_1b0 = uStack_204;
                  *(undefined4 *)local_1c0[2] = *(undefined4 *)((longlong)local_1c0 + 0x1c);
                  lVar32 = *local_1c0;
                  lVar31 = *(longlong *)(lVar32 + 0x30);
                  uVar35 = *(undefined8 *)(lVar32 + 0x40);
                  if (lVar31 == *(longlong *)(lVar32 + 0x20)) {
                    alloc::raw_vec::RawVec::
                    grow_one<country_core::resources::window::AvailableMonitor,alloc::alloc::Global>
                              ();
                  }
                  puVar13 = (undefined1 *)CONCAT44(uStack_110._4_4_,(float)uStack_110);
                  lVar29 = *(longlong *)(lVar32 + 0x28);
                  lVar28 = lVar31 * 0x40;
                  *(undefined8 *)(lVar29 + lVar28) = uVar35;
                  *(longlong *)(lVar29 + 8 + lVar28) = lVar9;
                  puVar2 = (undefined8 *)(lVar29 + 0x10 + lVar28);
                  *puVar2 = CONCAT44(fStack_174,uStack_178);
                  puVar2[1] = CONCAT44(fStack_16c,fStack_170);
                  *(ulonglong *)(lVar29 + 0x20 + lVar28) = CONCAT44(uStack_164,uStack_168);
                  *(undefined8 *)(lVar29 + 0x28 + lVar28) = uStack_1b8;
                  *(undefined4 *)(lVar29 + 0x30 + lVar28) = uStack_1b0;
                  *(float *)(lVar29 + 0x34 + lVar28) = ABS(fStack_26c - fVar46);
                  *(undefined1 *)(lVar29 + 0x38 + lVar28) = 0;
                  *(longlong *)(lVar32 + 0x30) = lVar31 + 1;
                  *(longlong *)(lVar32 + 0x40) = *(longlong *)(lVar32 + 0x40) + 1;
                }
              }
            }
            if ((char)local_e8 == '\0') {
              return;
            }
            uStack_110 = puVar13;
            puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
            if (*extraout_RAX_04 == 1) {
              plVar21 = extraout_RAX_04 + 1;
            }
            else {
              if (*extraout_RAX_04 != 0) goto LAB_140ad7c1b;
              std::sys::thread_local::native::lazy::Storage::
              initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                        ();
              plVar21 = extraout_RAX_07;
              if (extraout_RAX_07 == (longlong *)0x0) goto LAB_140ad7c1b;
            }
            if (*plVar21 != 0) goto LAB_140ad7c27;
            *plVar21 = -1;
            local_e8 = plVar21;
            puffin::ThreadProfiler::end_scope();
            goto LAB_140ad7213;
          }
LAB_140ad7371:
          auVar51[0] = -(*pcVar3 == -1);
          auVar51[1] = -(pcVar3[1] == -1);
          auVar51[2] = -(pcVar3[2] == -1);
          auVar51[3] = -(pcVar3[3] == -1);
          auVar51[4] = -(pcVar3[4] == -1);
          auVar51[5] = -(pcVar3[5] == -1);
          auVar51[6] = -(pcVar3[6] == -1);
          auVar51[7] = -(pcVar3[7] == -1);
          auVar51[8] = -(pcVar3[8] == -1);
          auVar51[9] = -(pcVar3[9] == -1);
          auVar51[10] = -(pcVar3[10] == -1);
          auVar51[0xb] = -(pcVar3[0xb] == -1);
          auVar51[0xc] = -(pcVar3[0xc] == -1);
          auVar51[0xd] = -(pcVar3[0xd] == -1);
          auVar51[0xe] = -(pcVar3[0xe] == -1);
          auVar51[0xf] = -(pcVar3[0xf] == -1);
          if ((((((((((((((((SUB161(auVar51 >> 7,0) & 1) != 0 || (SUB161(auVar51 >> 0xf,0) & 1) != 0
                           ) || (SUB161(auVar51 >> 0x17,0) & 1) != 0) ||
                         (SUB161(auVar51 >> 0x1f,0) & 1) != 0) ||
                        (SUB161(auVar51 >> 0x27,0) & 1) != 0) ||
                       (SUB161(auVar51 >> 0x2f,0) & 1) != 0) || (SUB161(auVar51 >> 0x37,0) & 1) != 0
                      ) || (SUB161(auVar51 >> 0x3f,0) & 1) != 0) ||
                    (SUB161(auVar51 >> 0x47,0) & 1) != 0) || (SUB161(auVar51 >> 0x4f,0) & 1) != 0)
                  || (SUB161(auVar51 >> 0x57,0) & 1) != 0) || (SUB161(auVar51 >> 0x5f,0) & 1) != 0)
                || (SUB161(auVar51 >> 0x67,0) & 1) != 0) || (SUB161(auVar51 >> 0x6f,0) & 1) != 0) ||
              (SUB161(auVar51 >> 0x77,0) & 1) != 0) || auVar51[0xf] < '\0') break;
          uVar22 = uVar22 + lVar29 + 0x10;
          lVar29 = lVar29 + 0x10;
        } while( true );
      }
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
  }
  if ((char)local_e8 != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_02 == 1) {
      plVar21 = extraout_RAX_02 + 1;
    }
    else {
      if (*extraout_RAX_02 != 0) {
LAB_140ad7c1b:
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      std::sys::thread_local::native::lazy::Storage::
      initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                ();
      plVar21 = extraout_RAX_06;
      if (extraout_RAX_06 == (longlong *)0x0) goto LAB_140ad7c1b;
    }
    if (*plVar21 != 0) {
LAB_140ad7c27:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar21 = -1;
    local_e8 = plVar21;
    puffin::ThreadProfiler::end_scope();
LAB_140ad7213:
    *local_e8 = *local_e8 + 1;
  }
  return;
code_r0x000140ad7364:
  uVar30 = (ushort)(uVar18 - 1) & (ushort)uVar18;
  uVar18 = CONCAT22((short)(uVar18 - 1 >> 0x10),uVar30);
  if (uVar30 == 0) goto LAB_140ad7371;
  goto LAB_140ad7349;
}
