
void __cdecl system_tree::process_tree_cmds::process_tree_cmds(void)

{
  undefined8 *puVar1;
  float fVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  char cVar5;
  char cVar6;
  char cVar7;
  char cVar8;
  char cVar9;
  char cVar10;
  char cVar11;
  char cVar12;
  char cVar13;
  char cVar14;
  char cVar15;
  char cVar16;
  char cVar17;
  char cVar18;
  char cVar19;
  char cVar20;
  byte extraout_AL;
  byte extraout_AL_00;
  char extraout_AL_01;
  uint uVar21;
  longlong extraout_RAX;
  undefined8 *puVar22;
  ulonglong *extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  longlong lVar23;
  ulonglong *puVar24;
  undefined8 extraout_RAX_02;
  ulonglong uVar25;
  undefined8 extraout_RAX_03;
  ulonglong extraout_RAX_04;
  undefined8 *extraout_RAX_05;
  undefined8 extraout_RAX_06;
  ulonglong extraout_RAX_07;
  ulonglong uVar26;
  undefined8 *in_RCX;
  ulonglong uVar27;
  ulonglong uVar28;
  longlong lVar29;
  longlong *in_RDX;
  ulonglong uVar30;
  byte bVar31;
  longlong lVar32;
  undefined4 uVar33;
  longlong lVar34;
  char *pcVar35;
  ulonglong *puVar36;
  undefined1 uVar37;
  undefined8 *in_R8;
  undefined8 *in_R9;
  ulonglong *puVar38;
  int iVar39;
  ulonglong *puVar40;
  longlong lVar41;
  longlong *plVar42;
  longlong lVar43;
  longlong lVar44;
  longlong *plVar45;
  bool bVar46;
  char cVar47;
  char cVar51;
  char cVar52;
  undefined4 extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  undefined4 extraout_XMM0_Da_03;
  char cVar54;
  char cVar55;
  char cVar56;
  char cVar57;
  char cVar58;
  char cVar59;
  char cVar60;
  char cVar61;
  char cVar62;
  char cVar63;
  char cVar64;
  undefined1 auVar48 [16];
  char cVar53;
  char cVar65;
  undefined1 auVar49 [16];
  undefined1 auVar50 [16];
  undefined1 auVar66 [16];
  undefined1 auVar67 [16];
  undefined1 auVar68 [16];
  float fVar69;
  float fVar70;
  float fVar71;
  float fVar72;
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
  puVar22 = (undefined8 *)(extraout_RAX + 0x28);
  if (*(char *)(extraout_RAX + 0x298) == '\0') {
    puVar22 = (undefined8 *)(extraout_RAX + 0x10);
    puVar1 = (undefined8 *)(extraout_RAX + 8);
  }
  local_258 = *puVar22;
  local_260 = *puVar1;
  puVar38 = (ulonglong *)*in_R8;
  lVar41 = in_R8[1];
  uVar26 = *puVar38;
  uVar27 = 0;
  uVar25 = uVar26 - *(ulonglong *)(lVar41 + 0x18);
  if (uVar26 < *(ulonglong *)(lVar41 + 0x18)) {
    uVar25 = uVar27;
  }
  uVar28 = uVar26 - *(ulonglong *)(lVar41 + 0x38);
  if (uVar26 < *(ulonglong *)(lVar41 + 0x38)) {
    uVar28 = uVar27;
  }
  lVar43 = uVar25 * 0x20 + *(longlong *)(lVar41 + 8);
  uVar26 = *(ulonglong *)(lVar41 + 0x10) - uVar25;
  if (*(ulonglong *)(lVar41 + 0x10) < uVar25) {
    lVar43 = 8;
    uVar26 = uVar27;
  }
  lVar34 = uVar28 * 0x20 + *(longlong *)(lVar41 + 0x28);
  uVar25 = *(ulonglong *)(lVar41 + 0x30) - uVar28;
  if (*(ulonglong *)(lVar41 + 0x30) < uVar28) {
    lVar34 = 8;
    uVar25 = uVar27;
  }
  *puVar38 = (*(longlong *)(lVar41 + 0x40) - uVar25) - uVar26;
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
  lVar32 = lVar34;
  lVar41 = lVar43;
  puVar24 = (ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E;
  do {
    while( true ) {
      if ((lVar41 == 0) || (lVar41 == uVar26 * 0x20 + lVar43)) {
        if ((lVar32 == 0) || (lVar32 == uVar25 * 0x20 + lVar34)) {
          local_250 = (ulonglong *)*in_stack_00000028;
          lVar41 = in_stack_00000028[1];
          uVar26 = *local_250;
          uVar27 = 0;
          uVar25 = uVar26 - *(ulonglong *)(lVar41 + 0x18);
          if (uVar26 < *(ulonglong *)(lVar41 + 0x18)) {
            uVar25 = uVar27;
          }
          uVar28 = uVar26 - *(ulonglong *)(lVar41 + 0x38);
          if (uVar26 < *(ulonglong *)(lVar41 + 0x38)) {
            uVar28 = uVar27;
          }
          puVar38 = (ulonglong *)(uVar25 * 0x48 + *(longlong *)(lVar41 + 8));
          uVar26 = *(ulonglong *)(lVar41 + 0x10) - uVar25;
          if (*(ulonglong *)(lVar41 + 0x10) < uVar25) {
            puVar38 = (ulonglong *)&DAT_00000008;
            uVar26 = uVar27;
          }
          puVar24 = (ulonglong *)(uVar28 * 0x48 + *(longlong *)(lVar41 + 0x28));
          uVar25 = *(ulonglong *)(lVar41 + 0x30) - uVar28;
          if (*(ulonglong *)(lVar41 + 0x30) < uVar28) {
            puVar24 = (ulonglong *)&DAT_00000008;
            uVar25 = uVar27;
          }
          *local_250 = (*(longlong *)(lVar41 + 0x40) - uVar25) - uVar26;
          cVar20 = UNK_143034d4f;
          cVar19 = UNK_143034d4e;
          cVar18 = UNK_143034d4d;
          cVar17 = UNK_143034d4c;
          cVar16 = UNK_143034d4b;
          cVar15 = UNK_143034d4a;
          cVar14 = UNK_143034d49;
          cVar13 = UNK_143034d48;
          cVar12 = UNK_143034d47;
          cVar11 = UNK_143034d46;
          cVar10 = UNK_143034d45;
          cVar9 = UNK_143034d44;
          cVar8 = UNK_143034d43;
          cVar7 = UNK_143034d42;
          cVar6 = UNK_143034d41;
          cVar5 = __xmm_745f64656b616e2f676e697261656c63;
          local_238 = puVar38 + uVar26 * 9;
          local_230 = puVar24 + uVar25 * 9;
          local_210 = *in_stack_00000048;
          local_1d8 = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
          local_108 = *in_stack_00000030;
          local_248 = (undefined4 *)in_stack_00000030[2];
          local_1d4 = *(undefined4 *)((longlong)in_stack_00000058 + 0x1c);
          local_1f0 = *in_stack_00000058;
          local_220 = (undefined4 *)in_stack_00000058[2];
          fVar72 = (float)__xmm_00000000000000004120000000000000;
          uVar26 = (ulonglong)__xmm_00000000000000004120000000000000 >> 0x20;
          local_240 = local_210 + 0x58;
          local_228 = local_108 + 0x20;
          local_218 = local_1f0 + 0x20;
          goto LAB_142267a29;
        }
        lVar29 = lVar32 + 0x20;
        lVar44 = 0;
        lVar41 = lVar32;
      }
      else {
        lVar44 = lVar41 + 0x20;
        lVar29 = lVar32;
      }
      *puVar38 = *puVar38 + 1;
      uVar3 = *(undefined8 *)(lVar41 + 0x10);
      uVar33 = *(undefined4 *)(lVar41 + 0x18);
      local_268 = *(ulonglong *)(lVar41 + 8);
      *local_1c8 = local_fc;
      country_core::resources::tree_storage::TreeStorage::get();
      lVar41 = lVar44;
      lVar32 = lVar29;
      if (extraout_RAX_00 != (ulonglong *)0x0) break;
      if (1 < *puVar24) {
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
    uVar27 = *extraout_RAX_00;
    uVar28 = uVar27 & 0xffffffff;
    if ((*(ulonglong *)(local_1a8 + 0x10) <= uVar28) ||
       (*(int *)(*(longlong *)(local_1a8 + 8) + uVar28 * 0x14) != (int)(uVar27 >> 0x20))) {
LAB_1422685b1:
      local_1a0 = ((ulonglong)local_118 & 0xffffffff00000000) + 1;
      plStack_198 = (longlong *)uVar27;
      goto LAB_142268583;
    }
    lVar44 = *(longlong *)(local_1a8 + 8) + uVar28 * 0x14;
    uVar28 = (ulonglong)*(uint *)(lVar44 + 4);
    if (uVar28 == 0xffffffff) goto LAB_1422685b1;
    if ((*(ulonglong *)(local_1e8 + 0x38) <= uVar28) ||
       ((*(ulonglong *)
          (*(longlong *)(local_1e8 + 0x28) + (ulonglong)(*(uint *)(lVar44 + 4) >> 6) * 8) >>
         (uVar28 & 0x3f) & 1) == 0)) goto LAB_1422685a4;
    uVar27 = (ulonglong)*(uint *)(lVar44 + 0x10);
    lVar29 = *(longlong *)
              (*(longlong *)(local_1a8 + 0x1a8) + 0x18 + (ulonglong)*(uint *)(lVar44 + 0xc) * 0x48);
    lVar23 = ~*(ulonglong *)
               (*(longlong *)
                 (*(longlong *)(local_1a8 + 0x1a8) + 0x38 +
                 (ulonglong)*(uint *)(lVar44 + 0xc) * 0x48) + *(longlong *)(local_1e8 + 0x110) * 8)
             * 0x30;
    lVar44 = *(longlong *)(lVar29 + 0x10 + lVar23);
    local_118 = (ulonglong *)(uVar27 * 4 + *(longlong *)(lVar29 + 0x20 + lVar23));
    *(undefined4 *)(*(longlong *)(lVar29 + 0x28 + lVar23) + uVar27 * 4) = (undefined4)local_108;
    *(undefined8 *)(lVar44 + uVar27 * 0x30) = uVar3;
    *(undefined4 *)(lVar44 + 8 + uVar27 * 0x30) = uVar33;
    ::utils::inverse_lerp();
    ::utils::float_grid::impl$0::
    bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
    *local_1b8 = local_f8;
    lVar44 = *(longlong *)(local_110 + 0x30);
    uVar4 = *(undefined8 *)(local_110 + 0x40);
    uVar37 = 0.0 < extraout_XMM0_Da_00;
    if (lVar44 == *(longlong *)(local_110 + 0x20)) {
      local_1c0 = (ulonglong *)CONCAT71(local_1c0._1_7_,0.0 < extraout_XMM0_Da_00);
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                ();
      uVar37 = local_1c0._0_1_;
    }
    lVar29 = *(longlong *)(local_110 + 0x28);
    lVar23 = lVar44 * 0x20;
    *(undefined8 *)(lVar29 + lVar23) = uVar4;
    *(undefined1 *)(lVar29 + 8 + lVar23) = 1;
    *(undefined1 *)(lVar29 + 9 + lVar23) = uVar37;
    *(undefined4 *)(lVar29 + 0xc + lVar23) = extraout_XMM0_Da;
    *(undefined8 *)(lVar29 + 0x10 + lVar23) = uVar3;
    *(undefined4 *)(lVar29 + 0x18 + lVar23) = uVar33;
    *(longlong *)(local_110 + 0x30) = lVar44 + 1;
    *(longlong *)(local_110 + 0x40) = *(longlong *)(local_110 + 0x40) + 1;
    puVar24 = (ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E;
  } while( true );
LAB_142267a29:
  if ((puVar38 == (ulonglong *)0x0) || (puVar38 == local_238)) {
    if ((puVar24 == (ulonglong *)0x0) || (puVar24 == local_230)) goto LAB_142268201;
    puVar36 = puVar24 + 9;
    puVar40 = (ulonglong *)0x0;
  }
  else {
    puVar40 = puVar38 + 9;
    puVar36 = puVar24;
    puVar24 = puVar38;
  }
  *local_250 = *local_250 + 1;
  ::utils::inverse_lerp();
  fVar70 = (__real_3f800000 - extraout_XMM0_Da_01) * __real_3f0ccccd;
  fVar69 = extraout_XMM0_Da_01 * __real_3f400000;
  country_core::systems::ui::resources::ui_state::ActiveWidget::is_dragging();
  if ((char)extraout_RAX_02 == '\0') {
    local_120 = (ulonglong)local_120._4_4_ << 0x20;
    pcVar35 = (char *)puVar24[1];
    uVar25 = puVar24[2];
    if (uVar25 == 0x19) goto LAB_142267b4a;
LAB_142267ad2:
    if (uVar25 == 0x27) {
      iVar39 = memcmp(pcVar35,&DAT_143034f28,0x27);
      if (iVar39 != 0) {
        iVar39 = memcmp(pcVar35,&DAT_143034f4f,0x27);
        bVar46 = iVar39 == 0;
        goto LAB_142267ba0;
      }
LAB_142267bb4:
      uVar33 = (undefined4)CONCAT71((int7)((ulonglong)pcVar35 >> 8),1);
    }
    else {
      if (uVar25 == 0x1a) {
        auVar66[0] = -(pcVar35[10] == __xmm_3164656b616e2f656572745f64656b61);
        auVar66[1] = -(pcVar35[0xb] == UNK_143034d51);
        auVar66[2] = -(pcVar35[0xc] == UNK_143034d52);
        auVar66[3] = -(pcVar35[0xd] == UNK_143034d53);
        auVar66[4] = -(pcVar35[0xe] == UNK_143034d54);
        auVar66[5] = -(pcVar35[0xf] == UNK_143034d55);
        auVar66[6] = -(pcVar35[0x10] == UNK_143034d56);
        auVar66[7] = -(pcVar35[0x11] == UNK_143034d57);
        auVar66[8] = -(pcVar35[0x12] == UNK_143034d58);
        auVar66[9] = -(pcVar35[0x13] == UNK_143034d59);
        auVar66[10] = -(pcVar35[0x14] == UNK_143034d5a);
        auVar66[0xb] = -(pcVar35[0x15] == UNK_143034d5b);
        auVar66[0xc] = -(pcVar35[0x16] == UNK_143034d5c);
        auVar66[0xd] = -(pcVar35[0x17] == UNK_143034d5d);
        auVar66[0xe] = -(pcVar35[0x18] == UNK_143034d5e);
        auVar66[0xf] = -(pcVar35[0x19] == UNK_143034d5f);
        auVar48[0] = -(*pcVar35 == cVar5);
        auVar48[1] = -(pcVar35[1] == cVar6);
        auVar48[2] = -(pcVar35[2] == cVar7);
        auVar48[3] = -(pcVar35[3] == cVar8);
        auVar48[4] = -(pcVar35[4] == cVar9);
        auVar48[5] = -(pcVar35[5] == cVar10);
        auVar48[6] = -(pcVar35[6] == cVar11);
        auVar48[7] = -(pcVar35[7] == cVar12);
        auVar48[8] = -(pcVar35[8] == cVar13);
        auVar48[9] = -(pcVar35[9] == cVar14);
        auVar48[10] = -(pcVar35[10] == cVar15);
        auVar48[0xb] = -(pcVar35[0xb] == cVar16);
        auVar48[0xc] = -(pcVar35[0xc] == cVar17);
        auVar48[0xd] = -(pcVar35[0xd] == cVar18);
        auVar48[0xe] = -(pcVar35[0xe] == cVar19);
        auVar48[0xf] = -(pcVar35[0xf] == cVar20);
        auVar48 = auVar48 & auVar66;
        if ((ushort)((ushort)(SUB161(auVar48 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar48 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar48 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar48 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar48 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar48 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar48 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar48 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar48 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar48 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar48 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar48 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar48 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar48 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar48 >> 0x77,0) & 1) << 0xe |
                    (ushort)(byte)(auVar48[0xf] >> 7) << 0xf) == 0xffff) goto LAB_142267bb4;
        cVar47 = *pcVar35;
        cVar51 = pcVar35[1];
        cVar52 = pcVar35[2];
        cVar53 = pcVar35[3];
        cVar54 = pcVar35[4];
        cVar55 = pcVar35[5];
        cVar56 = pcVar35[6];
        cVar57 = pcVar35[7];
        cVar58 = pcVar35[8];
        cVar59 = pcVar35[9];
        cVar60 = pcVar35[10];
        cVar61 = pcVar35[0xb];
        cVar62 = pcVar35[0xc];
        cVar63 = pcVar35[0xd];
        cVar64 = pcVar35[0xe];
        cVar65 = pcVar35[0xf];
        auVar67[0] = -(pcVar35[10] == __xmm_3264656b616e2f656572745f64656b61);
        auVar67[1] = -(pcVar35[0xb] == UNK_143034d61);
        auVar67[2] = -(pcVar35[0xc] == UNK_143034d62);
        auVar67[3] = -(pcVar35[0xd] == UNK_143034d63);
        auVar67[4] = -(pcVar35[0xe] == UNK_143034d64);
        auVar67[5] = -(pcVar35[0xf] == UNK_143034d65);
        auVar67[6] = -(pcVar35[0x10] == UNK_143034d66);
        auVar67[7] = -(pcVar35[0x11] == UNK_143034d67);
        auVar67[8] = -(pcVar35[0x12] == UNK_143034d68);
        auVar67[9] = -(pcVar35[0x13] == UNK_143034d69);
        auVar67[10] = -(pcVar35[0x14] == UNK_143034d6a);
        auVar67[0xb] = -(pcVar35[0x15] == UNK_143034d6b);
        auVar67[0xc] = -(pcVar35[0x16] == UNK_143034d6c);
        auVar67[0xd] = -(pcVar35[0x17] == UNK_143034d6d);
        auVar67[0xe] = -(pcVar35[0x18] == UNK_143034d6e);
        auVar67[0xf] = -(pcVar35[0x19] == UNK_143034d6f);
        goto LAB_142267b5b;
      }
      uVar33 = 0;
    }
  }
  else {
    local_120 = CONCAT44(local_120._4_4_,
                         (int)CONCAT71((int7)((ulonglong)extraout_RAX_02 >> 8),
                                       (*(byte *)((longlong)puVar24 + 0x2c) & 3) != 0));
    pcVar35 = (char *)puVar24[1];
    uVar25 = puVar24[2];
    if (uVar25 != 0x19) goto LAB_142267ad2;
LAB_142267b4a:
    cVar47 = *pcVar35;
    cVar51 = pcVar35[1];
    cVar52 = pcVar35[2];
    cVar53 = pcVar35[3];
    cVar54 = pcVar35[4];
    cVar55 = pcVar35[5];
    cVar56 = pcVar35[6];
    cVar57 = pcVar35[7];
    cVar58 = pcVar35[8];
    cVar59 = pcVar35[9];
    cVar60 = pcVar35[10];
    cVar61 = pcVar35[0xb];
    cVar62 = pcVar35[0xc];
    cVar63 = pcVar35[0xd];
    cVar64 = pcVar35[0xe];
    cVar65 = pcVar35[0xf];
    auVar67[0] = -(pcVar35[9] == __xmm_656772616c2f656572745f64656b616e);
    auVar67[1] = -(pcVar35[10] == UNK_143034d31);
    auVar67[2] = -(pcVar35[0xb] == UNK_143034d32);
    auVar67[3] = -(pcVar35[0xc] == UNK_143034d33);
    auVar67[4] = -(pcVar35[0xd] == UNK_143034d34);
    auVar67[5] = -(pcVar35[0xe] == UNK_143034d35);
    auVar67[6] = -(pcVar35[0xf] == UNK_143034d36);
    auVar67[7] = -(pcVar35[0x10] == UNK_143034d37);
    auVar67[8] = -(pcVar35[0x11] == UNK_143034d38);
    auVar67[9] = -(pcVar35[0x12] == UNK_143034d39);
    auVar67[10] = -(pcVar35[0x13] == UNK_143034d3a);
    auVar67[0xb] = -(pcVar35[0x14] == UNK_143034d3b);
    auVar67[0xc] = -(pcVar35[0x15] == UNK_143034d3c);
    auVar67[0xd] = -(pcVar35[0x16] == UNK_143034d3d);
    auVar67[0xe] = -(pcVar35[0x17] == UNK_143034d3e);
    auVar67[0xf] = -(pcVar35[0x18] == UNK_143034d3f);
LAB_142267b5b:
    auVar49[0] = -(cVar47 == cVar5);
    auVar49[1] = -(cVar51 == cVar6);
    auVar49[2] = -(cVar52 == cVar7);
    auVar49[3] = -(cVar53 == cVar8);
    auVar49[4] = -(cVar54 == cVar9);
    auVar49[5] = -(cVar55 == cVar10);
    auVar49[6] = -(cVar56 == cVar11);
    auVar49[7] = -(cVar57 == cVar12);
    auVar49[8] = -(cVar58 == cVar13);
    auVar49[9] = -(cVar59 == cVar14);
    auVar49[10] = -(cVar60 == cVar15);
    auVar49[0xb] = -(cVar61 == cVar16);
    auVar49[0xc] = -(cVar62 == cVar17);
    auVar49[0xd] = -(cVar63 == cVar18);
    auVar49[0xe] = -(cVar64 == cVar19);
    auVar49[0xf] = -(cVar65 == cVar20);
    auVar49 = auVar49 & auVar67;
    bVar46 = (ushort)((ushort)(SUB161(auVar49 >> 7,0) & 1) |
                      (ushort)(SUB161(auVar49 >> 0xf,0) & 1) << 1 |
                      (ushort)(SUB161(auVar49 >> 0x17,0) & 1) << 2 |
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
                      (ushort)(SUB161(auVar49 >> 0x77,0) & 1) << 0xe |
                     (ushort)(byte)(auVar49[0xf] >> 7) << 0xf) == 0xffff;
LAB_142267ba0:
    uVar33 = (undefined4)CONCAT71((int7)((ulonglong)pcVar35 >> 8),bVar46);
  }
  local_1c0 = puVar40;
  local_118 = puVar36;
  country_core::systems::save::load_completion_checker::SessionLoadReadiness::deny_load();
  country_core::systems::save::save_backup_list::BackupSaveFileKind::is_corrupted();
  fVar2 = *(float *)((longlong)puVar24 + 0x24);
  fVar71 = fVar2;
  if (fVar2 <= __real_3f333333) {
    fVar71 = __real_3f333333;
  }
  bVar31 = (byte)uVar33 & extraout_AL;
  if (bVar31 == 0) {
    fVar71 = fVar2;
  }
  uVar25 = puVar24[7];
  uVar27 = puVar24[6];
  local_1dc = uVar33;
  local_f2 = extraout_AL;
  local_f1 = extraout_AL_00;
  bevy_ecs::system::commands::Commands::spawn_empty();
  plVar42 = local_180;
  plVar45 = plStack_198;
  if (plStack_198 == (longlong *)0x0) {
    plVar45 = local_190;
  }
  lVar41 = plVar45[2];
  if ((ulonglong)(*plVar45 - lVar41) < 0x30) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  lVar43 = plVar45[1];
  *(code **)(lVar43 + lVar41) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::TreeComponent>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  *(code **)(lVar43 + 8 + lVar41) = bevy_ecs::error::handler::panic;
  *(longlong **)(lVar43 + 0x10 + lVar41) = plVar42;
  *(ulonglong *)(lVar43 + 0x18 + lVar41) = uVar25;
  *(char *)(lVar43 + 0x20 + lVar41) = (char)uVar27;
  *(undefined1 *)(lVar43 + 0x28 + lVar41) = 0;
  lVar43 = lVar41 + 0x30;
  plVar45[2] = lVar43;
  if ((ulonglong)(*plVar45 - lVar43) < 0x30) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  lVar34 = plVar45[1];
  *(code **)(lVar34 + 0x30 + lVar41) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::TreeAnimation>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  *(code **)(lVar34 + 0x38 + lVar41) = bevy_ecs::error::handler::panic;
  *(longlong **)(lVar34 + 0x40 + lVar41) = plVar42;
  *(float *)(lVar34 + 0x48 + lVar41) = fVar69 + fVar70;
  *(undefined1 *)(lVar34 + 0x24 + lVar43) = 0;
  *(undefined8 *)(lVar34 + 0x1c + lVar43) = 0;
  *(undefined1 *)(lVar34 + 0x58 + lVar41) = 0;
  plVar45[2] = lVar41 + 0x60;
  uVar25 = puVar24[3];
  uVar27 = puVar24[4];
  if ((ulonglong)(*plVar45 - (lVar41 + 0x60)) < 0x28) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  lVar43 = plVar45[1];
  *(code **)(lVar43 + 0x60 + lVar41) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::TreeMoveBend>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  *(code **)(lVar43 + 0x68 + lVar41) = bevy_ecs::error::handler::panic;
  *(longlong **)(lVar43 + 0x70 + lVar41) = plVar42;
  *(int *)(lVar43 + 0x78 + lVar41) = (int)uVar25;
  *(int *)(lVar43 + 0x7c + lVar41) = (int)uVar27;
  *(undefined1 *)(lVar43 + 0x80 + lVar41) = 0;
  plVar45[2] = lVar41 + 0x88;
  uVar25 = puVar24[3];
  fVar70 = fVar71 * fVar72;
  fVar69 = *(float *)(puVar24 + 4);
  if ((ulonglong)(*plVar45 - (lVar41 + 0x88)) < 0x28) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  lVar43 = plVar45[1];
  *(code **)(lVar43 + 0x88 + lVar41) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::TreeAmbience>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  *(code **)(lVar43 + 0x90 + lVar41) = bevy_ecs::error::handler::panic;
  *(longlong **)(lVar43 + 0x98 + lVar41) = plVar42;
  *(ulonglong *)(lVar43 + 0xa0 + lVar41) =
       CONCAT44(fVar71 * (float)uVar26 + (float)(uVar25 >> 0x20),fVar70 + (float)uVar25);
  *(float *)(lVar43 + 0xa8 + lVar41) = fVar69 + fVar70;
  *(undefined1 *)(lVar43 + 0xac + lVar41) = 0;
  plVar45[2] = lVar41 + 0xb0;
  uVar25 = puVar24[3];
  uVar28 = puVar24[4];
  uVar27 = puVar24[5];
  iVar39 = 3;
  if (bVar31 == 0) {
    iVar39 = (byte)local_120 + 1;
    if ((char)puVar24[6] != '\0') {
      iVar39 = 0;
    }
  }
  puVar36 = puVar24 + 7;
  pcVar35 = (char *)puVar24[1];
  uVar30 = puVar24[2];
  if (uVar30 == 0x18) {
    auVar50[0] = -(*pcVar35 == __xmm_4552505f454552545f474e495353494d);
    auVar50[1] = -(pcVar35[1] == UNK_143034d81);
    auVar50[2] = -(pcVar35[2] == UNK_143034d82);
    auVar50[3] = -(pcVar35[3] == UNK_143034d83);
    auVar50[4] = -(pcVar35[4] == UNK_143034d84);
    auVar50[5] = -(pcVar35[5] == UNK_143034d85);
    auVar50[6] = -(pcVar35[6] == UNK_143034d86);
    auVar50[7] = -(pcVar35[7] == UNK_143034d87);
    auVar50[8] = -(pcVar35[8] == UNK_143034d88);
    auVar50[9] = -(pcVar35[9] == UNK_143034d89);
    auVar50[10] = -(pcVar35[10] == UNK_143034d8a);
    auVar50[0xb] = -(pcVar35[0xb] == UNK_143034d8b);
    auVar50[0xc] = -(pcVar35[0xc] == UNK_143034d8c);
    auVar50[0xd] = -(pcVar35[0xd] == UNK_143034d8d);
    auVar50[0xe] = -(pcVar35[0xe] == UNK_143034d8e);
    auVar50[0xf] = -(pcVar35[0xf] == UNK_143034d8f);
    uVar3 = *(undefined8 *)(pcVar35 + 0x10);
    auVar68[0] = -((char)uVar3 == __xmm_0000000000000000454d414e5f424146[0]);
    auVar68[1] = -((char)((ulonglong)uVar3 >> 8) == __xmm_0000000000000000454d414e5f424146[1]);
    auVar68[2] = -((char)((ulonglong)uVar3 >> 0x10) == __xmm_0000000000000000454d414e5f424146[2]);
    auVar68[3] = -((char)((ulonglong)uVar3 >> 0x18) == __xmm_0000000000000000454d414e5f424146[3]);
    auVar68[4] = -((char)((ulonglong)uVar3 >> 0x20) == __xmm_0000000000000000454d414e5f424146[4]);
    auVar68[5] = -((char)((ulonglong)uVar3 >> 0x28) == __xmm_0000000000000000454d414e5f424146[5]);
    auVar68[6] = -((char)((ulonglong)uVar3 >> 0x30) == __xmm_0000000000000000454d414e5f424146[6]);
    auVar68[7] = -((char)((ulonglong)uVar3 >> 0x38) == __xmm_0000000000000000454d414e5f424146[7]);
    auVar68[8] = -(__xmm_0000000000000000454d414e5f424146[8] == '\0');
    auVar68[9] = -(UNK_143034d79 == '\0');
    auVar68[10] = -(UNK_143034d7a == '\0');
    auVar68[0xb] = -(UNK_143034d7b == '\0');
    auVar68[0xc] = -(UNK_143034d7c == '\0');
    auVar68[0xd] = -(UNK_143034d7d == '\0');
    auVar68[0xe] = -(UNK_143034d7e == '\0');
    auVar68[0xf] = -(UNK_143034d7f == '\0');
    auVar68 = auVar68 & auVar50;
    if ((ushort)((ushort)(SUB161(auVar68 >> 7,0) & 1) | (ushort)(SUB161(auVar68 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar68 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar68 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar68 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar68 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar68 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar68 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar68 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar68 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar68 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar68 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar68 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar68 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar68 >> 0x77,0) & 1) << 0xe |
                (ushort)(byte)(auVar68[0xf] >> 7) << 0xf) != 0xffff) goto LAB_142267e32;
    uVar30 = local_210;
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
  local_190 = plVar42;
  local_180 = (longlong *)CONCAT44(fVar71,(int)uVar28);
  local_170 = 0x3f800000;
  uStack_16c = 0;
  uStack_168 = 0;
  uStack_164 = 0;
  uStack_160 = 0;
  lVar41 = *(longlong *)(local_108 + 0x30);
  plStack_198 = (longlong *)uVar30;
  uStack_188 = uVar25;
  uStack_178 = uVar27;
  iStack_15c = iVar39;
  local_120 = local_1a0;
  if (lVar41 == *(longlong *)(local_108 + 0x20)) {
    alloc::raw_vec::RawVec::grow_one<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>
              ();
  }
  lVar43 = *(longlong *)(local_108 + 0x28);
  lVar34 = lVar41 * 0x50;
  puVar1 = (undefined8 *)(lVar43 + 0x40 + lVar34);
  *puVar1 = CONCAT44(iStack_15c,uStack_160);
  puVar1[1] = uStack_158;
  puVar1 = (undefined8 *)(lVar43 + 0x30 + lVar34);
  *puVar1 = CONCAT44(uStack_16c,local_170);
  puVar1[1] = CONCAT44(uStack_164,uStack_168);
  puVar1 = (undefined8 *)(lVar43 + 0x20 + lVar34);
  *puVar1 = local_180;
  puVar1[1] = uStack_178;
  puVar1 = (undefined8 *)(lVar43 + 0x10 + lVar34);
  *puVar1 = local_190;
  puVar1[1] = uStack_188;
  *(undefined8 *)(lVar43 + lVar34) = local_1a0;
  ((undefined8 *)(lVar43 + lVar34))[1] = plStack_198;
  *(longlong *)(local_108 + 0x30) = lVar41 + 1;
  *(longlong *)(local_108 + 0x40) = *(longlong *)(local_108 + 0x40) + 1;
  *local_1c8 = local_fc;
  country_core::resources::tree_storage::TreeStorage::insert();
  ::utils::float_grid::impl$0::
  bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
  local_1a0 = puVar24[3];
  plStack_198 = (longlong *)CONCAT44(plStack_198._4_4_,(int)puVar24[4]);
  *local_1b8 = local_f8;
  lVar41 = *(longlong *)(local_110 + 0x30);
  uVar3 = *(undefined8 *)(local_110 + 0x40);
  if (lVar41 == *(longlong *)(local_110 + 0x20)) {
    alloc::raw_vec::RawVec::
    grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
              ();
  }
  puVar24 = local_118;
  puVar38 = local_1c0;
  lVar43 = *(longlong *)(local_110 + 0x28);
  lVar34 = lVar41 * 0x20;
  *(undefined8 *)(lVar43 + lVar34) = uVar3;
  *(undefined1 *)(lVar43 + 8 + lVar34) = 0;
  *(bool *)(lVar43 + 9 + lVar34) = 0.0 < extraout_XMM0_Da_02;
  *(byte *)(lVar43 + 10 + lVar34) = (byte)local_1dc & (local_f2 | local_f1);
  *(float *)(lVar43 + 0xc + lVar34) = extraout_XMM0_Da_01;
  *(undefined4 *)(lVar43 + 0x18 + lVar34) = plStack_198._0_4_;
  *(ulonglong *)(lVar43 + 0x10 + lVar34) = local_1a0;
  *(longlong *)(local_110 + 0x30) = lVar41 + 1;
  *(longlong *)(local_110 + 0x40) = *(longlong *)(local_110 + 0x40) + 1;
  if (extraout_RAX_04 >> 0x20 != 0) {
    if (1 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
      local_1f8 = core::fmt::num::imp::impl$9::fmt;
      local_200 = puVar36;
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
    lVar41 = *(longlong *)(local_1f0 + 0x30);
    uVar3 = *(undefined8 *)(local_1f0 + 0x40);
    if (lVar41 == *(longlong *)(local_1f0 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                ();
    }
    lVar43 = *(longlong *)(local_1f0 + 0x28);
    *(undefined8 *)(lVar43 + lVar41 * 0x10) = uVar3;
    *(ulonglong *)(lVar43 + 8 + lVar41 * 0x10) = extraout_RAX_04;
    *(longlong *)(local_1f0 + 0x30) = lVar41 + 1;
    *(longlong *)(local_1f0 + 0x40) = *(longlong *)(local_1f0 + 0x40) + 1;
  }
  goto LAB_142267a29;
LAB_1422685a4:
  local_1a0 = uVar28 << 0x20;
  plStack_198 = (longlong *)uVar27;
  goto LAB_142268583;
LAB_142268201:
  puVar38 = (ulonglong *)*local_208;
  lVar41 = local_208[1];
  uVar26 = *puVar38;
  uVar27 = 0;
  uVar25 = uVar26 - *(ulonglong *)(lVar41 + 0x18);
  if (uVar26 < *(ulonglong *)(lVar41 + 0x18)) {
    uVar25 = uVar27;
  }
  uVar28 = uVar26 - *(ulonglong *)(lVar41 + 0x38);
  if (uVar26 < *(ulonglong *)(lVar41 + 0x38)) {
    uVar28 = uVar27;
  }
  lVar43 = uVar25 * 0x10 + *(longlong *)(lVar41 + 8);
  uVar26 = *(ulonglong *)(lVar41 + 0x10) - uVar25;
  if (*(ulonglong *)(lVar41 + 0x10) < uVar25) {
    lVar43 = 8;
    uVar26 = uVar27;
  }
  lVar34 = uVar28 * 0x10 + *(longlong *)(lVar41 + 0x28);
  uVar25 = *(ulonglong *)(lVar41 + 0x30) - uVar28;
  if (*(ulonglong *)(lVar41 + 0x30) < uVar28) {
    lVar34 = 8;
    uVar25 = uVar27;
  }
  *puVar38 = (*(longlong *)(lVar41 + 0x40) - uVar25) - uVar26;
  lVar41 = uVar26 * 0x10 + lVar43;
  local_108 = uVar25 * 0x10 + lVar34;
  local_118 = puVar38;
  while( true ) {
    do {
      if ((lVar43 == 0) || (lVar43 == lVar41)) {
        if ((lVar34 == 0) || (lVar34 == local_108)) {
          return;
        }
        lVar34 = lVar34 + 0x10;
        lVar43 = 0;
      }
      else {
        lVar43 = lVar43 + 0x10;
      }
      *puVar38 = *puVar38 + 1;
      *local_1c8 = local_fc;
      country_core::resources::tree_storage::TreeStorage::remove();
    } while ((extraout_RAX_07 >> 0x20 == 0) ||
            (bevy_ecs::entity::Entities::contains(), extraout_AL_01 == '\0'));
    plVar42 = *(longlong **)(in_stack_00000038 + 8);
    if (plVar42 == (longlong *)0x0) {
      plVar42 = *(longlong **)(in_stack_00000038 + 0x10);
    }
    lVar32 = plVar42[2];
    if ((ulonglong)(*plVar42 - lVar32) < 0x20) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    puVar38 = local_118;
    uVar26 = plVar42[1];
    *(code **)(uVar26 + lVar32) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::TreeMarkedForDeletion>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(uVar26 + 8 + lVar32) = bevy_ecs::error::handler::panic;
    *(ulonglong *)(uVar26 + 0x10 + lVar32) = extraout_RAX_07;
    *(undefined1 *)(uVar26 + 0x18 + lVar32) = 0;
    plVar42[2] = lVar32 + 0x20;
    uVar25 = extraout_RAX_07 & 0xffffffff;
    if ((*(ulonglong *)(local_1a8 + 0x10) <= uVar25) ||
       (uVar26 = *(ulonglong *)(local_1a8 + 8),
       *(int *)(uVar26 + uVar25 * 0x14) != (int)(extraout_RAX_07 >> 0x20))) break;
    lVar32 = uVar26 + uVar25 * 0x14;
    uVar21 = *(uint *)(lVar32 + 4);
    uVar26 = (ulonglong)uVar21;
    if (uVar26 == 0xffffffff) break;
    uVar33 = 0;
    if ((*(ulonglong *)(local_1e8 + 0x38) <= uVar26) ||
       ((*(ulonglong *)(*(longlong *)(local_1e8 + 0x28) + (ulonglong)(uVar21 >> 6) * 8) >>
         (uVar26 & 0x3f) & 1) == 0)) goto LAB_14226856f;
    lVar44 = *(longlong *)
              (*(longlong *)
                (*(longlong *)(local_1a8 + 0x1a8) + 0x18 + (ulonglong)*(uint *)(lVar32 + 0xc) * 0x48
                ) + 0x10 +
              ~*(ulonglong *)
                (*(longlong *)
                  (*(longlong *)(local_1a8 + 0x1a8) + 0x38 +
                  (ulonglong)*(uint *)(lVar32 + 0xc) * 0x48) + *(longlong *)(local_1e8 + 0x110) * 8)
              * 0x30);
    lVar32 = (ulonglong)*(uint *)(lVar32 + 0x10) * 0x30;
    ::utils::inverse_lerp();
    uVar3 = *(undefined8 *)(lVar44 + lVar32);
    uVar33 = *(undefined4 *)(lVar44 + 8 + lVar32);
    *local_1b8 = local_f8;
    lVar32 = *(longlong *)(local_110 + 0x30);
    uVar4 = *(undefined8 *)(local_110 + 0x40);
    if (lVar32 == *(longlong *)(local_110 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                ();
    }
    lVar44 = *(longlong *)(local_110 + 0x28);
    lVar29 = lVar32 * 0x20;
    *(undefined8 *)(lVar44 + lVar29) = uVar4;
    *(undefined1 *)(lVar44 + 8 + lVar29) = 2;
    *(undefined4 *)(lVar44 + 0xc + lVar29) = extraout_XMM0_Da_03;
    *(undefined8 *)(lVar44 + 0x10 + lVar29) = uVar3;
    *(undefined4 *)(lVar44 + 0x18 + lVar29) = uVar33;
    *(longlong *)(local_110 + 0x30) = lVar32 + 1;
    *(longlong *)(local_110 + 0x40) = *(longlong *)(local_110 + 0x40) + 1;
  }
  uVar21 = (uint)uVar26;
  uVar33 = 1;
LAB_14226856f:
  local_1a0 = CONCAT44(uVar21,uVar33);
  plStack_198 = (longlong *)extraout_RAX_07;
LAB_142268583:
                    /* WARNING: Subroutine does not return */
  core::result::unwrap_failed();
}

