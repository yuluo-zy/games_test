
void __cdecl system_tree::process_tree_cmds::process_tree_cmds(void)

{
  undefined8 *puVar1;
  float fVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  byte extraout_AL;
  byte extraout_AL_00;
  char extraout_AL_01;
  uint uVar5;
  longlong extraout_RAX;
  undefined8 *puVar6;
  ulonglong *extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  longlong lVar7;
  ulonglong uVar8;
  ulonglong *puVar9;
  undefined8 extraout_RAX_02;
  undefined8 extraout_RAX_03;
  ulonglong extraout_RAX_04;
  undefined8 *extraout_RAX_05;
  undefined8 extraout_RAX_06;
  ulonglong extraout_RAX_07;
  ulonglong uVar10;
  undefined8 *in_RCX;
  ulonglong uVar11;
  ulonglong uVar12;
  longlong lVar13;
  longlong *in_RDX;
  byte bVar14;
  longlong lVar15;
  undefined4 uVar16;
  longlong lVar17;
  char *pcVar18;
  ulonglong *puVar19;
  undefined1 uVar20;
  undefined8 *in_R8;
  undefined8 *in_R9;
  ulonglong *puVar21;
  int iVar22;
  ulonglong *puVar23;
  longlong lVar24;
  longlong *plVar25;
  longlong lVar26;
  longlong lVar27;
  longlong *plVar28;
  bool bVar29;
  char cVar30;
  char cVar34;
  char cVar35;
  undefined4 extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  undefined4 extraout_XMM0_Da_03;
  char cVar37;
  char cVar38;
  char cVar39;
  char cVar40;
  char cVar41;
  char cVar42;
  char cVar43;
  char cVar44;
  char cVar45;
  char cVar46;
  char cVar47;
  undefined1 auVar31 [16];
  char cVar36;
  char cVar48;
  undefined1 auVar32 [16];
  undefined1 auVar33 [16];
  undefined1 auVar49 [16];
  undefined1 auVar50 [16];
  undefined1 auVar51 [16];
  float fVar52;
  undefined8 *in_stack_00000028;
  longlong *in_stack_00000030;
  longlong in_stack_00000038;
  longlong *in_stack_00000040;
  ulonglong *in_stack_00000048;
  longlong *in_stack_00000058;
  ulonglong local_268;
  undefined8 local_260;
  undefined8 local_258;
  ulonglong *local_250;
  undefined4 *local_248;
  longlong local_240;
  ulonglong *local_238;
  ulonglong *local_230;
  longlong local_228;
  undefined4 *local_220;
  longlong local_218;
  ulonglong local_210;
  undefined8 *local_208;
  ulonglong *local_200;
  code *local_1f8;
  longlong local_1f0;
  longlong local_1e8;
  undefined4 local_1dc;
  undefined4 local_1d8;
  undefined4 local_1d4;
  undefined8 local_1d0;
  undefined4 *local_1c8;
  ulonglong *local_1c0;
  undefined4 *local_1b8;
  longlong local_1b0;
  longlong local_1a8;
  undefined8 local_1a0;
  longlong *plStack_198;
  longlong *local_190;
  ulonglong uStack_188;
  undefined8 local_180;
  ulonglong uStack_178;
  undefined4 local_170;
  undefined4 uStack_16c;
  undefined4 uStack_168;
  undefined4 uStack_164;
  undefined4 uStack_160;
  int iStack_15c;
  undefined8 uStack_158;
  undefined **local_150;
  undefined8 local_148;
  ulonglong **local_140;
  undefined8 local_138;
  undefined8 local_130;
  undefined8 local_120;
  ulonglong *local_118;
  longlong local_110;
  longlong local_108;
  undefined4 local_fc;
  undefined4 local_f8;
  byte local_f2;
  byte local_f1;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  local_208 = in_R9;
  toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
  puVar1 = (undefined8 *)(extraout_RAX + 0x20);
  puVar6 = (undefined8 *)(extraout_RAX + 0x28);
  if (*(char *)(extraout_RAX + 0x298) == '\0') {
    puVar6 = (undefined8 *)(extraout_RAX + 0x10);
    puVar1 = (undefined8 *)(extraout_RAX + 8);
  }
  local_258 = *puVar6;
  local_260 = *puVar1;
  puVar21 = (ulonglong *)*in_R8;
  lVar24 = in_R8[1];
  uVar10 = *puVar21;
  uVar11 = 0;
  uVar8 = uVar10 - *(ulonglong *)(lVar24 + 0x18);
  if (uVar10 < *(ulonglong *)(lVar24 + 0x18)) {
    uVar8 = uVar11;
  }
  uVar12 = uVar10 - *(ulonglong *)(lVar24 + 0x38);
  if (uVar10 < *(ulonglong *)(lVar24 + 0x38)) {
    uVar12 = uVar11;
  }
  lVar26 = uVar8 * 0x20 + *(longlong *)(lVar24 + 8);
  uVar10 = *(ulonglong *)(lVar24 + 0x10) - uVar8;
  if (*(ulonglong *)(lVar24 + 0x10) < uVar8) {
    lVar26 = 8;
    uVar10 = uVar11;
  }
  lVar17 = uVar12 * 0x20 + *(longlong *)(lVar24 + 0x28);
  uVar8 = *(ulonglong *)(lVar24 + 0x30) - uVar12;
  if (*(ulonglong *)(lVar24 + 0x30) < uVar12) {
    lVar17 = 8;
    uVar8 = uVar11;
  }
  *puVar21 = (*(longlong *)(lVar24 + 0x40) - uVar8) - uVar10;
  local_fc = *(undefined4 *)((longlong)in_RCX + 0x1c);
  local_1d0 = *in_RCX;
  local_1c8 = (undefined4 *)in_RCX[2];
  local_1e8 = *in_RDX;
  local_1a8 = in_RDX[1];
  local_108 = CONCAT44(local_108._4_4_,*(undefined4 *)((longlong)in_RDX + 0x14));
  local_f8 = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
  local_110 = *in_stack_00000040;
  local_1b8 = (undefined4 *)in_stack_00000040[2];
  local_1b0 = local_110 + 0x20;
  lVar15 = lVar17;
  lVar24 = lVar26;
  puVar9 = (ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E;
  do {
    while( true ) {
      if ((lVar24 == 0) || (lVar24 == uVar10 * 0x20 + lVar26)) {
        if ((lVar15 == 0) || (lVar15 == uVar8 * 0x20 + lVar17)) {
          local_250 = (ulonglong *)*in_stack_00000028;
          lVar24 = in_stack_00000028[1];
          uVar10 = *local_250;
          uVar11 = 0;
          uVar8 = uVar10 - *(ulonglong *)(lVar24 + 0x18);
          if (uVar10 < *(ulonglong *)(lVar24 + 0x18)) {
            uVar8 = uVar11;
          }
          uVar12 = uVar10 - *(ulonglong *)(lVar24 + 0x38);
          if (uVar10 < *(ulonglong *)(lVar24 + 0x38)) {
            uVar12 = uVar11;
          }
          puVar21 = (ulonglong *)(uVar8 * 0x48 + *(longlong *)(lVar24 + 8));
          uVar10 = *(ulonglong *)(lVar24 + 0x10) - uVar8;
          if (*(ulonglong *)(lVar24 + 0x10) < uVar8) {
            puVar21 = (ulonglong *)&DAT_00000008;
            uVar10 = uVar11;
          }
          puVar9 = (ulonglong *)(uVar12 * 0x48 + *(longlong *)(lVar24 + 0x28));
          uVar8 = *(ulonglong *)(lVar24 + 0x30) - uVar12;
          if (*(ulonglong *)(lVar24 + 0x30) < uVar12) {
            puVar9 = (ulonglong *)&DAT_00000008;
            uVar8 = uVar11;
          }
          *local_250 = (*(longlong *)(lVar24 + 0x40) - uVar8) - uVar10;
          local_238 = puVar21 + uVar10 * 9;
          local_230 = puVar9 + uVar8 * 9;
          local_210 = *in_stack_00000048;
          local_1d8 = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
          local_108 = *in_stack_00000030;
          local_248 = (undefined4 *)in_stack_00000030[2];
          local_1d4 = *(undefined4 *)((longlong)in_stack_00000058 + 0x1c);
          local_1f0 = *in_stack_00000058;
          local_220 = (undefined4 *)in_stack_00000058[2];
          local_240 = local_210 + 0x58;
          local_228 = local_108 + 0x20;
          local_218 = local_1f0 + 0x20;
          goto LAB_142267a29;
        }
        lVar13 = lVar15 + 0x20;
        lVar27 = 0;
        lVar24 = lVar15;
      }
      else {
        lVar27 = lVar24 + 0x20;
        lVar13 = lVar15;
      }
      *puVar21 = *puVar21 + 1;
      uVar3 = *(undefined8 *)(lVar24 + 0x10);
      uVar16 = *(undefined4 *)(lVar24 + 0x18);
      local_268 = *(ulonglong *)(lVar24 + 8);
      *local_1c8 = local_fc;
      country_core::resources::tree_storage::TreeStorage::get();
      lVar24 = lVar27;
      lVar15 = lVar13;
      if (extraout_RAX_00 != (ulonglong *)0x0) break;
      if (1 < *puVar9) {
        local_200 = &local_268;
        local_1f8 = country_core::components::impl$13::fmt;
        toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
        local_180 = (longlong *)*extraout_RAX_01;
        uStack_178 = extraout_RAX_01[1];
        local_170 = 2;
        uStack_16c = 0;
        uStack_168 = 0x43034fe0;
        uStack_164 = 1;
        uStack_160 = 0x1e;
        iStack_15c = 0;
        local_150 = &PTR_s_MoveTreeCmd_references_a_non_exi_143035048;
        local_148 = 1;
        local_140 = &local_200;
        local_138 = 1;
        local_130 = 0;
        local_1a0 = 0;
        plStack_198 = (longlong *)0x143034fe0;
        local_190 = (longlong *)&DAT_0000001e;
        uStack_188 = 0;
        uStack_158 = CONCAT44(*(undefined4 *)(extraout_RAX_01 + 2),1);
        ::log::__private_api::impl$2::log();
      }
    }
    uVar11 = *extraout_RAX_00;
    uVar12 = uVar11 & 0xffffffff;
    if ((*(ulonglong *)(local_1a8 + 0x10) <= uVar12) ||
       (*(int *)(*(longlong *)(local_1a8 + 8) + uVar12 * 0x14) != (int)(uVar11 >> 0x20))) {
LAB_1422685b1:
      local_1a0 = ((ulonglong)local_118 & 0xffffffff00000000) + 1;
      plStack_198 = (longlong *)uVar11;
      goto LAB_142268583;
    }
    lVar27 = *(longlong *)(local_1a8 + 8) + uVar12 * 0x14;
    uVar12 = (ulonglong)*(uint *)(lVar27 + 4);
    if (uVar12 == 0xffffffff) goto LAB_1422685b1;
    if ((*(ulonglong *)(local_1e8 + 0x38) <= uVar12) ||
       ((*(ulonglong *)
          (*(longlong *)(local_1e8 + 0x28) + (ulonglong)(*(uint *)(lVar27 + 4) >> 6) * 8) >>
         (uVar12 & 0x3f) & 1) == 0)) goto LAB_1422685a4;
    uVar11 = (ulonglong)*(uint *)(lVar27 + 0x10);
    lVar13 = *(longlong *)
              (*(longlong *)(local_1a8 + 0x1a8) + 0x18 + (ulonglong)*(uint *)(lVar27 + 0xc) * 0x48);
    lVar7 = ~*(ulonglong *)
              (*(longlong *)
                (*(longlong *)(local_1a8 + 0x1a8) + 0x38 + (ulonglong)*(uint *)(lVar27 + 0xc) * 0x48
                ) + *(longlong *)(local_1e8 + 0x110) * 8) * 0x30;
    lVar27 = *(longlong *)(lVar13 + 0x10 + lVar7);
    local_118 = (ulonglong *)(uVar11 * 4 + *(longlong *)(lVar13 + 0x20 + lVar7));
    *(undefined4 *)(*(longlong *)(lVar13 + 0x28 + lVar7) + uVar11 * 4) = (undefined4)local_108;
    *(undefined8 *)(lVar27 + uVar11 * 0x30) = uVar3;
    *(undefined4 *)(lVar27 + 8 + uVar11 * 0x30) = uVar16;
    ::utils::inverse_lerp();
    ::utils::float_grid::impl$0::
    bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
    *local_1b8 = local_f8;
    lVar27 = *(longlong *)(local_110 + 0x30);
    uVar4 = *(undefined8 *)(local_110 + 0x40);
    uVar20 = 0.0 < extraout_XMM0_Da_00;
    if (lVar27 == *(longlong *)(local_110 + 0x20)) {
      local_1c0 = (ulonglong *)CONCAT71(local_1c0._1_7_,0.0 < extraout_XMM0_Da_00);
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                ();
      uVar20 = local_1c0._0_1_;
    }
    lVar13 = *(longlong *)(local_110 + 0x28);
    lVar7 = lVar27 * 0x20;
    *(undefined8 *)(lVar13 + lVar7) = uVar4;
    *(undefined1 *)(lVar13 + 8 + lVar7) = 1;
    *(undefined1 *)(lVar13 + 9 + lVar7) = uVar20;
    *(undefined4 *)(lVar13 + 0xc + lVar7) = extraout_XMM0_Da;
    *(undefined8 *)(lVar13 + 0x10 + lVar7) = uVar3;
    *(undefined4 *)(lVar13 + 0x18 + lVar7) = uVar16;
    *(longlong *)(local_110 + 0x30) = lVar27 + 1;
    *(longlong *)(local_110 + 0x40) = *(longlong *)(local_110 + 0x40) + 1;
    puVar9 = (ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E;
  } while( true );
LAB_142267a29:
  if ((puVar21 == (ulonglong *)0x0) || (puVar21 == local_238)) {
    if ((puVar9 == (ulonglong *)0x0) || (puVar9 == local_230)) goto LAB_142268201;
    puVar19 = puVar9 + 9;
    puVar23 = (ulonglong *)0x0;
  }
  else {
    puVar23 = puVar21 + 9;
    puVar19 = puVar9;
    puVar9 = puVar21;
  }
  *local_250 = *local_250 + 1;
  ::utils::inverse_lerp();
  country_core::systems::ui::resources::ui_state::ActiveWidget::is_dragging();
  if ((char)extraout_RAX_02 == '\0') {
    local_120 = (ulonglong)local_120._4_4_ << 0x20;
    pcVar18 = (char *)puVar9[1];
    uVar10 = puVar9[2];
    if (uVar10 == 0x19) goto LAB_142267b4a;
LAB_142267ad2:
    if (uVar10 == 0x27) {
      iVar22 = memcmp(pcVar18,&DAT_143034f28,0x27);
      if (iVar22 != 0) {
        iVar22 = memcmp(pcVar18,&DAT_143034f4f,0x27);
        bVar29 = iVar22 == 0;
        goto LAB_142267ba0;
      }
LAB_142267bb4:
      uVar16 = (undefined4)CONCAT71((int7)((ulonglong)pcVar18 >> 8),1);
    }
    else {
      if (uVar10 == 0x1a) {
        auVar49[0] = -(pcVar18[10] == 'a');
        auVar49[1] = -(pcVar18[0xb] == 'k');
        auVar49[2] = -(pcVar18[0xc] == 'e');
        auVar49[3] = -(pcVar18[0xd] == 'd');
        auVar49[4] = -(pcVar18[0xe] == '_');
        auVar49[5] = -(pcVar18[0xf] == 't');
        auVar49[6] = -(pcVar18[0x10] == 'r');
        auVar49[7] = -(pcVar18[0x11] == 'e');
        auVar49[8] = -(pcVar18[0x12] == 'e');
        auVar49[9] = -(pcVar18[0x13] == '/');
        auVar49[10] = -(pcVar18[0x14] == 'n');
        auVar49[0xb] = -(pcVar18[0x15] == 'a');
        auVar49[0xc] = -(pcVar18[0x16] == 'k');
        auVar49[0xd] = -(pcVar18[0x17] == 'e');
        auVar49[0xe] = -(pcVar18[0x18] == 'd');
        auVar49[0xf] = -(pcVar18[0x19] == '1');
        auVar31[0] = -(*pcVar18 == 'c');
        auVar31[1] = -(pcVar18[1] == 'l');
        auVar31[2] = -(pcVar18[2] == 'e');
        auVar31[3] = -(pcVar18[3] == 'a');
        auVar31[4] = -(pcVar18[4] == 'r');
        auVar31[5] = -(pcVar18[5] == 'i');
        auVar31[6] = -(pcVar18[6] == 'n');
        auVar31[7] = -(pcVar18[7] == 'g');
        auVar31[8] = -(pcVar18[8] == '/');
        auVar31[9] = -(pcVar18[9] == 'n');
        auVar31[10] = -(pcVar18[10] == 'a');
        auVar31[0xb] = -(pcVar18[0xb] == 'k');
        auVar31[0xc] = -(pcVar18[0xc] == 'e');
        auVar31[0xd] = -(pcVar18[0xd] == 'd');
        auVar31[0xe] = -(pcVar18[0xe] == '_');
        auVar31[0xf] = -(pcVar18[0xf] == 't');
        auVar31 = auVar31 & auVar49;
        if ((ushort)((ushort)(SUB161(auVar31 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar31 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar31 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar31 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar31 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar31 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar31 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar31 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar31 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar31 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar31 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar31 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar31 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar31 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar31 >> 0x77,0) & 1) << 0xe |
                    (ushort)(byte)(auVar31[0xf] >> 7) << 0xf) == 0xffff) goto LAB_142267bb4;
        cVar30 = *pcVar18;
        cVar34 = pcVar18[1];
        cVar35 = pcVar18[2];
        cVar36 = pcVar18[3];
        cVar37 = pcVar18[4];
        cVar38 = pcVar18[5];
        cVar39 = pcVar18[6];
        cVar40 = pcVar18[7];
        cVar41 = pcVar18[8];
        cVar42 = pcVar18[9];
        cVar43 = pcVar18[10];
        cVar44 = pcVar18[0xb];
        cVar45 = pcVar18[0xc];
        cVar46 = pcVar18[0xd];
        cVar47 = pcVar18[0xe];
        cVar48 = pcVar18[0xf];
        auVar50[0] = -(pcVar18[10] == 'a');
        auVar50[1] = -(pcVar18[0xb] == 'k');
        auVar50[2] = -(pcVar18[0xc] == 'e');
        auVar50[3] = -(pcVar18[0xd] == 'd');
        auVar50[4] = -(pcVar18[0xe] == '_');
        auVar50[5] = -(pcVar18[0xf] == 't');
        auVar50[6] = -(pcVar18[0x10] == 'r');
        auVar50[7] = -(pcVar18[0x11] == 'e');
        auVar50[8] = -(pcVar18[0x12] == 'e');
        auVar50[9] = -(pcVar18[0x13] == '/');
        auVar50[10] = -(pcVar18[0x14] == 'n');
        auVar50[0xb] = -(pcVar18[0x15] == 'a');
        auVar50[0xc] = -(pcVar18[0x16] == 'k');
        auVar50[0xd] = -(pcVar18[0x17] == 'e');
        auVar50[0xe] = -(pcVar18[0x18] == 'd');
        auVar50[0xf] = -(pcVar18[0x19] == '2');
        goto LAB_142267b5b;
      }
      uVar16 = 0;
    }
  }
  else {
    local_120 = CONCAT44(local_120._4_4_,
                         (int)CONCAT71((int7)((ulonglong)extraout_RAX_02 >> 8),
                                       (*(byte *)((longlong)puVar9 + 0x2c) & 3) != 0));
    pcVar18 = (char *)puVar9[1];
    uVar10 = puVar9[2];
    if (uVar10 != 0x19) goto LAB_142267ad2;
LAB_142267b4a:
    cVar30 = *pcVar18;
    cVar34 = pcVar18[1];
    cVar35 = pcVar18[2];
    cVar36 = pcVar18[3];
    cVar37 = pcVar18[4];
    cVar38 = pcVar18[5];
    cVar39 = pcVar18[6];
    cVar40 = pcVar18[7];
    cVar41 = pcVar18[8];
    cVar42 = pcVar18[9];
    cVar43 = pcVar18[10];
    cVar44 = pcVar18[0xb];
    cVar45 = pcVar18[0xc];
    cVar46 = pcVar18[0xd];
    cVar47 = pcVar18[0xe];
    cVar48 = pcVar18[0xf];
    auVar50[0] = -(pcVar18[9] == 'n');
    auVar50[1] = -(pcVar18[10] == 'a');
    auVar50[2] = -(pcVar18[0xb] == 'k');
    auVar50[3] = -(pcVar18[0xc] == 'e');
    auVar50[4] = -(pcVar18[0xd] == 'd');
    auVar50[5] = -(pcVar18[0xe] == '_');
    auVar50[6] = -(pcVar18[0xf] == 't');
    auVar50[7] = -(pcVar18[0x10] == 'r');
    auVar50[8] = -(pcVar18[0x11] == 'e');
    auVar50[9] = -(pcVar18[0x12] == 'e');
    auVar50[10] = -(pcVar18[0x13] == '/');
    auVar50[0xb] = -(pcVar18[0x14] == 'l');
    auVar50[0xc] = -(pcVar18[0x15] == 'a');
    auVar50[0xd] = -(pcVar18[0x16] == 'r');
    auVar50[0xe] = -(pcVar18[0x17] == 'g');
    auVar50[0xf] = -(pcVar18[0x18] == 'e');
LAB_142267b5b:
    auVar32[0] = -(cVar30 == 'c');
    auVar32[1] = -(cVar34 == 'l');
    auVar32[2] = -(cVar35 == 'e');
    auVar32[3] = -(cVar36 == 'a');
    auVar32[4] = -(cVar37 == 'r');
    auVar32[5] = -(cVar38 == 'i');
    auVar32[6] = -(cVar39 == 'n');
    auVar32[7] = -(cVar40 == 'g');
    auVar32[8] = -(cVar41 == '/');
    auVar32[9] = -(cVar42 == 'n');
    auVar32[10] = -(cVar43 == 'a');
    auVar32[0xb] = -(cVar44 == 'k');
    auVar32[0xc] = -(cVar45 == 'e');
    auVar32[0xd] = -(cVar46 == 'd');
    auVar32[0xe] = -(cVar47 == '_');
    auVar32[0xf] = -(cVar48 == 't');
    auVar32 = auVar32 & auVar50;
    bVar29 = (ushort)((ushort)(SUB161(auVar32 >> 7,0) & 1) |
                      (ushort)(SUB161(auVar32 >> 0xf,0) & 1) << 1 |
                      (ushort)(SUB161(auVar32 >> 0x17,0) & 1) << 2 |
                      (ushort)(SUB161(auVar32 >> 0x1f,0) & 1) << 3 |
                      (ushort)(SUB161(auVar32 >> 0x27,0) & 1) << 4 |
                      (ushort)(SUB161(auVar32 >> 0x2f,0) & 1) << 5 |
                      (ushort)(SUB161(auVar32 >> 0x37,0) & 1) << 6 |
                      (ushort)(SUB161(auVar32 >> 0x3f,0) & 1) << 7 |
                      (ushort)(SUB161(auVar32 >> 0x47,0) & 1) << 8 |
                      (ushort)(SUB161(auVar32 >> 0x4f,0) & 1) << 9 |
                      (ushort)(SUB161(auVar32 >> 0x57,0) & 1) << 10 |
                      (ushort)(SUB161(auVar32 >> 0x5f,0) & 1) << 0xb |
                      (ushort)(SUB161(auVar32 >> 0x67,0) & 1) << 0xc |
                      (ushort)(SUB161(auVar32 >> 0x6f,0) & 1) << 0xd |
                      (ushort)(SUB161(auVar32 >> 0x77,0) & 1) << 0xe |
                     (ushort)(byte)(auVar32[0xf] >> 7) << 0xf) == 0xffff;
LAB_142267ba0:
    uVar16 = (undefined4)CONCAT71((int7)((ulonglong)pcVar18 >> 8),bVar29);
  }
  local_1c0 = puVar23;
  local_118 = puVar19;
  country_core::systems::save::load_completion_checker::SessionLoadReadiness::deny_load();
  country_core::systems::save::save_backup_list::BackupSaveFileKind::is_corrupted();
  fVar2 = *(float *)((longlong)puVar9 + 0x24);
  fVar52 = fVar2;
  if (fVar2 <= 0.7) {
    fVar52 = 0.7;
  }
  bVar14 = (byte)uVar16 & extraout_AL;
  if (bVar14 == 0) {
    fVar52 = fVar2;
  }
  uVar10 = puVar9[7];
  uVar8 = puVar9[6];
  local_1dc = uVar16;
  local_f2 = extraout_AL;
  local_f1 = extraout_AL_00;
  bevy_ecs::system::commands::Commands::spawn_empty();
  plVar25 = local_180;
  plVar28 = plStack_198;
  if (plStack_198 == (longlong *)0x0) {
    plVar28 = local_190;
  }
  lVar24 = plVar28[2];
  if ((ulonglong)(*plVar28 - lVar24) < 0x30) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  lVar26 = plVar28[1];
  *(code **)(lVar26 + lVar24) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::TreeComponent>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  *(code **)(lVar26 + 8 + lVar24) = bevy_ecs::error::handler::panic;
  *(longlong **)(lVar26 + 0x10 + lVar24) = plVar25;
  *(ulonglong *)(lVar26 + 0x18 + lVar24) = uVar10;
  *(char *)(lVar26 + 0x20 + lVar24) = (char)uVar8;
  *(undefined1 *)(lVar26 + 0x28 + lVar24) = 0;
  lVar26 = lVar24 + 0x30;
  plVar28[2] = lVar26;
  if ((ulonglong)(*plVar28 - lVar26) < 0x30) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  lVar17 = plVar28[1];
  *(code **)(lVar17 + 0x30 + lVar24) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::TreeAnimation>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  *(code **)(lVar17 + 0x38 + lVar24) = bevy_ecs::error::handler::panic;
  *(longlong **)(lVar17 + 0x40 + lVar24) = plVar25;
  *(float *)(lVar17 + 0x48 + lVar24) =
       extraout_XMM0_Da_01 * 0.75 + (1.0 - extraout_XMM0_Da_01) * 0.55;
  *(undefined1 *)(lVar17 + 0x24 + lVar26) = 0;
  *(undefined8 *)(lVar17 + 0x1c + lVar26) = 0;
  *(undefined1 *)(lVar17 + 0x58 + lVar24) = 0;
  plVar28[2] = lVar24 + 0x60;
  uVar10 = puVar9[3];
  uVar8 = puVar9[4];
  if ((ulonglong)(*plVar28 - (lVar24 + 0x60)) < 0x28) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  lVar26 = plVar28[1];
  *(code **)(lVar26 + 0x60 + lVar24) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::TreeMoveBend>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  *(code **)(lVar26 + 0x68 + lVar24) = bevy_ecs::error::handler::panic;
  *(longlong **)(lVar26 + 0x70 + lVar24) = plVar25;
  *(int *)(lVar26 + 0x78 + lVar24) = (int)uVar10;
  *(int *)(lVar26 + 0x7c + lVar24) = (int)uVar8;
  *(undefined1 *)(lVar26 + 0x80 + lVar24) = 0;
  plVar28[2] = lVar24 + 0x88;
  uVar10 = puVar9[3];
  fVar2 = *(float *)(puVar9 + 4);
  if ((ulonglong)(*plVar28 - (lVar24 + 0x88)) < 0x28) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  lVar26 = plVar28[1];
  *(code **)(lVar26 + 0x88 + lVar24) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::TreeAmbience>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  *(code **)(lVar26 + 0x90 + lVar24) = bevy_ecs::error::handler::panic;
  *(longlong **)(lVar26 + 0x98 + lVar24) = plVar25;
  *(ulonglong *)(lVar26 + 0xa0 + lVar24) =
       CONCAT44(fVar52 * 10.0 + (float)(uVar10 >> 0x20),fVar52 * 0.0 + (float)uVar10);
  *(float *)(lVar26 + 0xa8 + lVar24) = fVar2 + fVar52 * 0.0;
  *(undefined1 *)(lVar26 + 0xac + lVar24) = 0;
  plVar28[2] = lVar24 + 0xb0;
  uVar10 = puVar9[3];
  uVar11 = puVar9[4];
  uVar8 = puVar9[5];
  iVar22 = 3;
  if (bVar14 == 0) {
    iVar22 = (byte)local_120 + 1;
    if ((char)puVar9[6] != '\0') {
      iVar22 = 0;
    }
  }
  puVar19 = puVar9 + 7;
  pcVar18 = (char *)puVar9[1];
  uVar12 = puVar9[2];
  if (uVar12 == 0x18) {
    auVar33[0] = -(*pcVar18 == 'M');
    auVar33[1] = -(pcVar18[1] == 'I');
    auVar33[2] = -(pcVar18[2] == 'S');
    auVar33[3] = -(pcVar18[3] == 'S');
    auVar33[4] = -(pcVar18[4] == 'I');
    auVar33[5] = -(pcVar18[5] == 'N');
    auVar33[6] = -(pcVar18[6] == 'G');
    auVar33[7] = -(pcVar18[7] == '_');
    auVar33[8] = -(pcVar18[8] == 'T');
    auVar33[9] = -(pcVar18[9] == 'R');
    auVar33[10] = -(pcVar18[10] == 'E');
    auVar33[0xb] = -(pcVar18[0xb] == 'E');
    auVar33[0xc] = -(pcVar18[0xc] == '_');
    auVar33[0xd] = -(pcVar18[0xd] == 'P');
    auVar33[0xe] = -(pcVar18[0xe] == 'R');
    auVar33[0xf] = -(pcVar18[0xf] == 'E');
    uVar3 = *(undefined8 *)(pcVar18 + 0x10);
    auVar51[0] = -((char)uVar3 == 'F');
    auVar51[1] = -((char)((ulonglong)uVar3 >> 8) == 'A');
    auVar51[2] = -((char)((ulonglong)uVar3 >> 0x10) == 'B');
    auVar51[3] = -((char)((ulonglong)uVar3 >> 0x18) == '_');
    auVar51[4] = -((char)((ulonglong)uVar3 >> 0x20) == 'N');
    auVar51[5] = -((char)((ulonglong)uVar3 >> 0x28) == 'A');
    auVar51[6] = -((char)((ulonglong)uVar3 >> 0x30) == 'M');
    auVar51[7] = -((char)((ulonglong)uVar3 >> 0x38) == 'E');
    auVar51[8] = 0xff;
    auVar51[9] = 0xff;
    auVar51[10] = 0xff;
    auVar51[0xb] = 0xff;
    auVar51[0xc] = 0xff;
    auVar51[0xd] = 0xff;
    auVar51[0xe] = 0xff;
    auVar51[0xf] = 0xff;
    auVar51 = auVar51 & auVar33;
    if ((ushort)((ushort)(SUB161(auVar51 >> 7,0) & 1) | (ushort)(SUB161(auVar51 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar51 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar51 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar51 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar51 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar51 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar51 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar51 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar51 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar51 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar51 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar51 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar51 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar51 >> 0x77,0) & 1) << 0xe |
                (ushort)(byte)(auVar51[0xf] >> 7) << 0xf) != 0xffff) goto LAB_142267e32;
    uVar12 = local_210;
    ui_place_tree::random_tree_prefab();
    local_1a0 = extraout_RAX_06;
  }
  else {
LAB_142267e32:
    asset::prefab::PrefabName::new();
    local_1a0 = extraout_RAX_03;
  }
  *local_248 = local_1d8;
  uStack_158 = *(undefined8 *)(local_108 + 0x40);
  local_190 = plVar25;
  local_180 = (longlong *)CONCAT44(fVar52,(int)uVar11);
  local_170 = 0x3f800000;
  uStack_16c = 0;
  uStack_168 = 0;
  uStack_164 = 0;
  uStack_160 = 0;
  lVar24 = *(longlong *)(local_108 + 0x30);
  plStack_198 = (longlong *)uVar12;
  uStack_188 = uVar10;
  uStack_178 = uVar8;
  iStack_15c = iVar22;
  local_120 = local_1a0;
  if (lVar24 == *(longlong *)(local_108 + 0x20)) {
    alloc::raw_vec::RawVec::grow_one<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>
              ();
  }
  lVar26 = *(longlong *)(local_108 + 0x28);
  lVar17 = lVar24 * 0x50;
  puVar1 = (undefined8 *)(lVar26 + 0x40 + lVar17);
  *puVar1 = CONCAT44(iStack_15c,uStack_160);
  puVar1[1] = uStack_158;
  puVar1 = (undefined8 *)(lVar26 + 0x30 + lVar17);
  *puVar1 = CONCAT44(uStack_16c,local_170);
  puVar1[1] = CONCAT44(uStack_164,uStack_168);
  puVar1 = (undefined8 *)(lVar26 + 0x20 + lVar17);
  *puVar1 = local_180;
  puVar1[1] = uStack_178;
  puVar1 = (undefined8 *)(lVar26 + 0x10 + lVar17);
  *puVar1 = local_190;
  puVar1[1] = uStack_188;
  *(undefined8 *)(lVar26 + lVar17) = local_1a0;
  ((undefined8 *)(lVar26 + lVar17))[1] = plStack_198;
  *(longlong *)(local_108 + 0x30) = lVar24 + 1;
  *(longlong *)(local_108 + 0x40) = *(longlong *)(local_108 + 0x40) + 1;
  *local_1c8 = local_fc;
  country_core::resources::tree_storage::TreeStorage::insert();
  ::utils::float_grid::impl$0::
  bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
  local_1a0 = puVar9[3];
  plStack_198 = (longlong *)CONCAT44(plStack_198._4_4_,(int)puVar9[4]);
  *local_1b8 = local_f8;
  lVar24 = *(longlong *)(local_110 + 0x30);
  uVar3 = *(undefined8 *)(local_110 + 0x40);
  if (lVar24 == *(longlong *)(local_110 + 0x20)) {
    alloc::raw_vec::RawVec::
    grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
              ();
  }
  puVar9 = local_118;
  puVar21 = local_1c0;
  lVar26 = *(longlong *)(local_110 + 0x28);
  lVar17 = lVar24 * 0x20;
  *(undefined8 *)(lVar26 + lVar17) = uVar3;
  *(undefined1 *)(lVar26 + 8 + lVar17) = 0;
  *(bool *)(lVar26 + 9 + lVar17) = 0.0 < extraout_XMM0_Da_02;
  *(byte *)(lVar26 + 10 + lVar17) = (byte)local_1dc & (local_f2 | local_f1);
  *(float *)(lVar26 + 0xc + lVar17) = extraout_XMM0_Da_01;
  *(undefined4 *)(lVar26 + 0x18 + lVar17) = plStack_198._0_4_;
  *(ulonglong *)(lVar26 + 0x10 + lVar17) = local_1a0;
  *(longlong *)(local_110 + 0x30) = lVar24 + 1;
  *(longlong *)(local_110 + 0x40) = *(longlong *)(local_110 + 0x40) + 1;
  if (extraout_RAX_04 >> 0x20 != 0) {
    if (1 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
      local_1f8 = core::fmt::num::imp::impl$9::fmt;
      local_200 = puVar19;
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      local_180 = (longlong *)*extraout_RAX_05;
      uStack_178 = extraout_RAX_05[1];
      local_170 = 2;
      uStack_16c = 0;
      uStack_168 = 0x43034fe0;
      uStack_164 = 1;
      uStack_160 = 0x1e;
      iStack_15c = 0;
      local_150 = &PTR_DAT_143034fa8;
      local_148 = 2;
      local_140 = &local_200;
      local_138 = 1;
      local_130 = 0;
      local_1a0 = 0;
      plStack_198 = (longlong *)0x143034fe0;
      local_190 = (longlong *)&DAT_0000001e;
      uStack_188 = 0;
      uStack_158 = CONCAT44(*(undefined4 *)(extraout_RAX_05 + 2),1);
      ::log::__private_api::impl$2::log();
    }
    *local_220 = local_1d4;
    lVar24 = *(longlong *)(local_1f0 + 0x30);
    uVar3 = *(undefined8 *)(local_1f0 + 0x40);
    if (lVar24 == *(longlong *)(local_1f0 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                ();
    }
    lVar26 = *(longlong *)(local_1f0 + 0x28);
    *(undefined8 *)(lVar26 + lVar24 * 0x10) = uVar3;
    *(ulonglong *)(lVar26 + 8 + lVar24 * 0x10) = extraout_RAX_04;
    *(longlong *)(local_1f0 + 0x30) = lVar24 + 1;
    *(longlong *)(local_1f0 + 0x40) = *(longlong *)(local_1f0 + 0x40) + 1;
  }
  goto LAB_142267a29;
LAB_1422685a4:
  local_1a0 = uVar12 << 0x20;
  plStack_198 = (longlong *)uVar11;
  goto LAB_142268583;
LAB_142268201:
  puVar21 = (ulonglong *)*local_208;
  lVar24 = local_208[1];
  uVar10 = *puVar21;
  uVar11 = 0;
  uVar8 = uVar10 - *(ulonglong *)(lVar24 + 0x18);
  if (uVar10 < *(ulonglong *)(lVar24 + 0x18)) {
    uVar8 = uVar11;
  }
  uVar12 = uVar10 - *(ulonglong *)(lVar24 + 0x38);
  if (uVar10 < *(ulonglong *)(lVar24 + 0x38)) {
    uVar12 = uVar11;
  }
  lVar26 = uVar8 * 0x10 + *(longlong *)(lVar24 + 8);
  uVar10 = *(ulonglong *)(lVar24 + 0x10) - uVar8;
  if (*(ulonglong *)(lVar24 + 0x10) < uVar8) {
    lVar26 = 8;
    uVar10 = uVar11;
  }
  lVar17 = uVar12 * 0x10 + *(longlong *)(lVar24 + 0x28);
  uVar8 = *(ulonglong *)(lVar24 + 0x30) - uVar12;
  if (*(ulonglong *)(lVar24 + 0x30) < uVar12) {
    lVar17 = 8;
    uVar8 = uVar11;
  }
  *puVar21 = (*(longlong *)(lVar24 + 0x40) - uVar8) - uVar10;
  lVar24 = uVar10 * 0x10 + lVar26;
  local_108 = uVar8 * 0x10 + lVar17;
  local_118 = puVar21;
  while( true ) {
    do {
      if ((lVar26 == 0) || (lVar26 == lVar24)) {
        if ((lVar17 == 0) || (lVar17 == local_108)) {
          return;
        }
        lVar17 = lVar17 + 0x10;
        lVar26 = 0;
      }
      else {
        lVar26 = lVar26 + 0x10;
      }
      *puVar21 = *puVar21 + 1;
      *local_1c8 = local_fc;
      country_core::resources::tree_storage::TreeStorage::remove();
    } while ((extraout_RAX_07 >> 0x20 == 0) ||
            (bevy_ecs::entity::Entities::contains(), extraout_AL_01 == '\0'));
    plVar25 = *(longlong **)(in_stack_00000038 + 8);
    if (plVar25 == (longlong *)0x0) {
      plVar25 = *(longlong **)(in_stack_00000038 + 0x10);
    }
    lVar15 = plVar25[2];
    if ((ulonglong)(*plVar25 - lVar15) < 0x20) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    puVar21 = local_118;
    uVar10 = plVar25[1];
    *(code **)(uVar10 + lVar15) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::TreeMarkedForDeletion>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(uVar10 + 8 + lVar15) = bevy_ecs::error::handler::panic;
    *(ulonglong *)(uVar10 + 0x10 + lVar15) = extraout_RAX_07;
    *(undefined1 *)(uVar10 + 0x18 + lVar15) = 0;
    plVar25[2] = lVar15 + 0x20;
    uVar8 = extraout_RAX_07 & 0xffffffff;
    if ((*(ulonglong *)(local_1a8 + 0x10) <= uVar8) ||
       (uVar10 = *(ulonglong *)(local_1a8 + 8),
       *(int *)(uVar10 + uVar8 * 0x14) != (int)(extraout_RAX_07 >> 0x20))) break;
    lVar15 = uVar10 + uVar8 * 0x14;
    uVar5 = *(uint *)(lVar15 + 4);
    uVar10 = (ulonglong)uVar5;
    if (uVar10 == 0xffffffff) break;
    uVar16 = 0;
    if ((*(ulonglong *)(local_1e8 + 0x38) <= uVar10) ||
       ((*(ulonglong *)(*(longlong *)(local_1e8 + 0x28) + (ulonglong)(uVar5 >> 6) * 8) >>
         (uVar10 & 0x3f) & 1) == 0)) goto LAB_14226856f;
    lVar27 = *(longlong *)
              (*(longlong *)
                (*(longlong *)(local_1a8 + 0x1a8) + 0x18 + (ulonglong)*(uint *)(lVar15 + 0xc) * 0x48
                ) + 0x10 +
              ~*(ulonglong *)
                (*(longlong *)
                  (*(longlong *)(local_1a8 + 0x1a8) + 0x38 +
                  (ulonglong)*(uint *)(lVar15 + 0xc) * 0x48) + *(longlong *)(local_1e8 + 0x110) * 8)
              * 0x30);
    lVar15 = (ulonglong)*(uint *)(lVar15 + 0x10) * 0x30;
    ::utils::inverse_lerp();
    uVar3 = *(undefined8 *)(lVar27 + lVar15);
    uVar16 = *(undefined4 *)(lVar27 + 8 + lVar15);
    *local_1b8 = local_f8;
    lVar15 = *(longlong *)(local_110 + 0x30);
    uVar4 = *(undefined8 *)(local_110 + 0x40);
    if (lVar15 == *(longlong *)(local_110 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                ();
    }
    lVar27 = *(longlong *)(local_110 + 0x28);
    lVar13 = lVar15 * 0x20;
    *(undefined8 *)(lVar27 + lVar13) = uVar4;
    *(undefined1 *)(lVar27 + 8 + lVar13) = 2;
    *(undefined4 *)(lVar27 + 0xc + lVar13) = extraout_XMM0_Da_03;
    *(undefined8 *)(lVar27 + 0x10 + lVar13) = uVar3;
    *(undefined4 *)(lVar27 + 0x18 + lVar13) = uVar16;
    *(longlong *)(local_110 + 0x30) = lVar15 + 1;
    *(longlong *)(local_110 + 0x40) = *(longlong *)(local_110 + 0x40) + 1;
  }
  uVar5 = (uint)uVar10;
  uVar16 = 1;
LAB_14226856f:
  local_1a0 = CONCAT44(uVar5,uVar16);
  plStack_198 = (longlong *)extraout_RAX_07;
LAB_142268583:
                    // WARNING: Subroutine does not return
  core::result::unwrap_failed();
}

