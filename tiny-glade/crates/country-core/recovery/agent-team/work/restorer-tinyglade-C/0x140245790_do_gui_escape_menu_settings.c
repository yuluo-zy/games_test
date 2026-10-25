// do_gui_escape_menu_settings @ 0x140245790
// sig: void __cdecl do_gui_escape_menu_settings(void)
// size: 4290


// WARNING: Instruction at (ram,0x0001429a16b9) overlaps instruction at (ram,0x0001429a16b8)
// 
// WARNING: Control flow encountered bad instruction data
// WARNING: Removing unreachable block (ram,0x00014024b429)
// WARNING: Removing unreachable block (ram,0x0001429a16dd)

void __cdecl tiny_glade::gui::settings::do_gui_escape_menu_settings(void)

{
  byte *pbVar1;
  byte bVar2;
  uint uVar3;
  uint uVar4;
  undefined4 uVar5;
  undefined1 uVar6;
  uint uVar7;
  uint uVar8;
  longlong lVar9;
  int *piVar10;
  int *piVar11;
  uint *puVar12;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  byte extraout_AL_02;
  byte extraout_AL_03;
  byte extraout_AL_04;
  byte extraout_AL_05;
  char extraout_AL_06;
  char cVar13;
  undefined1 uVar14;
  uint extraout_EAX;
  undefined8 extraout_RAX;
  ulonglong extraout_RAX_00;
  longlong extraout_RAX_01;
  ulonglong extraout_RAX_02;
  ulonglong extraout_RAX_03;
  longlong extraout_RAX_04;
  ulonglong extraout_RAX_05;
  longlong extraout_RAX_06;
  byte *pbVar15;
  longlong lVar16;
  byte bVar17;
  int iVar18;
  undefined8 *in_RCX;
  char *pcVar19;
  uint uVar20;
  undefined8 *in_RDX;
  int ***pppiVar21;
  undefined8 uVar22;
  undefined8 uVar23;
  longlong *plVar24;
  undefined1 *puVar25;
  undefined1 *puVar26;
  undefined1 *puVar27;
  undefined1 *puVar28;
  ushort uVar29;
  int **ppiVar30;
  int **ppiVar31;
  undefined4 *puVar32;
  int ***pppiVar33;
  longlong *in_R8;
  ulonglong uVar34;
  undefined1 (*pauVar35) [16];
  undefined8 *in_R9;
  undefined *puVar36;
  int **in_R10;
  uint in_R11D;
  ulonglong uVar37;
  code cVar38;
  int ****ppppiVar39;
  int *piVar40;
  bool bVar41;
  bool bVar42;
  bool bVar43;
  byte bVar44;
  bool bVar45;
  char cVar50;
  undefined4 extraout_XMM0_Da;
  undefined4 extraout_XMM0_Da_00;
  undefined4 extraout_XMM0_Da_01;
  undefined4 extraout_XMM0_Da_02;
  float fVar46;
  char cVar48;
  char cVar49;
  undefined1 auVar47 [16];
  undefined1 auVar51 [16];
  undefined1 auVar52 [16];
  float unaff_XMM6_Da;
  uint unaff_XMM6_Db;
  uint unaff_XMM6_Dc;
  uint unaff_XMM6_Dd;
  uint unaff_XMM7_Da;
  undefined8 *in_stack_00000028;
  longlong *in_stack_00000030;
  undefined8 *in_stack_00000038;
  code *in_stack_00000040;
  longlong *in_stack_00000048;
  longlong *in_stack_00000050;
  longlong *in_stack_00000058;
  undefined8 uStack_540;
  undefined1 auStack_538 [4];
  undefined4 uStack_534;
  ushort auStack_530 [12];
  int ***local_518;
  undefined8 *local_510;
  undefined4 local_508;
  undefined8 local_4f8;
  undefined8 uStack_4f0;
  undefined8 local_4e8;
  undefined8 uStack_4e0;
  undefined8 local_4d8;
  undefined8 uStack_4d0;
  undefined8 local_4c8;
  undefined8 uStack_4c0;
  undefined8 local_4b8;
  undefined8 uStack_4b0;
  undefined8 local_4a8;
  undefined8 uStack_4a0;
  undefined8 local_490;
  ulonglong uStack_488;
  int ***local_480;
  undefined8 local_478;
  longlong lStack_470;
  undefined8 local_468;
  undefined8 uStack_460;
  longlong local_458;
  longlong lStack_450;
  longlong local_448;
  longlong lStack_440;
  int ***local_438;
  longlong lStack_430;
  int ***local_428;
  longlong lStack_420;
  uint *local_418;
  int ***pppiStack_410;
  longlong local_408;
  longlong lStack_400;
  int ***local_3f8;
  longlong lStack_3f0;
  longlong local_3e8;
  undefined8 uStack_3e0;
  byte *local_3d8;
  longlong lStack_3d0;
  undefined8 local_3c8;
  undefined8 uStack_3c0;
  undefined8 local_3b8;
  undefined8 uStack_3b0;
  int *****local_3a8;
  undefined8 uStack_3a0;
  ulonglong local_398;
  undefined8 uStack_390;
  undefined8 local_388;
  undefined8 uStack_380;
  longlong local_370;
  longlong local_368;
  undefined8 local_360;
  longlong local_358;
  longlong lStack_350;
  longlong local_348;
  longlong lStack_340;
  longlong *local_338;
  undefined8 *local_330;
  int *local_328;
  int ***local_320;
  longlong *local_318;
  ulonglong uStack_310;
  byte *local_308;
  undefined8 uStack_300;
  int **local_2f8;
  undefined8 uStack_2f0;
  int ***local_2e8;
  uint **ppuStack_2e0;
  longlong lStack_2d8;
  longlong lStack_2d0;
  undefined8 local_2c8;
  undefined8 uStack_2c0;
  undefined8 local_2b8;
  undefined4 *puStack_2b0;
  longlong *local_2a8;
  longlong lStack_2a0;
  int ****local_298;
  longlong *plStack_290;
  int ****local_288;
  undefined8 uStack_280;
  ulonglong local_278;
  undefined8 uStack_270;
  int ***local_268;
  undefined8 uStack_260;
  int ***local_258;
  longlong lStack_250;
  int ***local_248;
  longlong lStack_240;
  longlong local_238;
  longlong lStack_230;
  longlong local_228;
  longlong lStack_220;
  int ***local_218;
  undefined8 *local_210;
  int ***local_208;
  undefined8 *local_200;
  undefined8 *local_1f8;
  undefined8 *local_1f0;
  undefined8 local_1e8;
  longlong lStack_1e0;
  undefined8 local_1d8;
  undefined8 uStack_1d0;
  int ***local_1c8;
  longlong lStack_1c0;
  longlong local_1b8;
  undefined8 uStack_1b0;
  int *local_1a8;
  code *local_1a0;
  int *local_198;
  longlong lStack_190;
  int **local_188;
  longlong lStack_180;
  undefined8 local_170;
  uint local_168;
  uint local_164;
  int **local_160;
  code *pcStack_158;
  int ***local_150;
  uint **ppuStack_148;
  undefined8 local_140;
  longlong local_138;
  longlong local_130;
  undefined4 local_124;
  int ***local_120;
  int **local_118;
  longlong lStack_110;
  longlong local_108;
  longlong *local_f8;
  ulonglong uStack_f0;
  longlong local_e8;
  uint local_e0;
  undefined4 local_dc;
  undefined4 local_c4;
  int **local_c0;
  char local_b3;
  undefined1 local_b2;
  undefined8 local_b0;
  
  bVar44 = 0;
  puVar26 = auStack_538;
  local_b0 = 0xfffffffffffffffe;
  local_398 = in_RCX[4];
  uStack_390 = in_RCX[5];
  local_388 = in_RCX[6];
  uStack_380 = in_RCX[7];
  local_3b8 = (int ***)in_RCX[8];
  uStack_3b0 = in_RCX[9];
  local_3a8 = (int *****)in_RCX[10];
  uStack_3a0 = in_RCX[0xb];
  local_3d8 = (byte *)in_RCX[0xc];
  lStack_3d0 = in_RCX[0xd];
  local_3c8 = in_RCX[0xe];
  uStack_3c0 = in_RCX[0xf];
  local_3f8 = (int ***)in_RCX[0x10];
  lStack_3f0 = in_RCX[0x11];
  local_3e8 = in_RCX[0x12];
  uStack_3e0 = in_RCX[0x13];
  local_418 = (uint *)*in_R8;
  pppiStack_410 = (int ***)in_R8[1];
  local_408 = in_R8[2];
  lStack_400 = in_R8[3];
  local_358 = in_R8[4];
  lStack_350 = in_R8[5];
  local_348 = in_R8[6];
  lStack_340 = in_R8[7];
  local_438 = (int ***)in_R8[8];
  lStack_430 = in_R8[9];
  local_428 = (int ***)in_R8[10];
  lStack_420 = in_R8[0xb];
  local_458 = in_R8[0xc];
  lStack_450 = in_R8[0xd];
  local_448 = in_R8[0xe];
  lStack_440 = in_R8[0xf];
  local_4b8 = *in_stack_00000028;
  uStack_4b0 = in_stack_00000028[1];
  local_4a8 = in_stack_00000028[2];
  uStack_4a0 = in_stack_00000028[3];
  local_4d8 = in_stack_00000028[4];
  uStack_4d0 = in_stack_00000028[5];
  local_4c8 = in_stack_00000028[6];
  uStack_4c0 = in_stack_00000028[7];
  local_478 = in_stack_00000028[8];
  lStack_470 = in_stack_00000028[9];
  local_468 = in_stack_00000028[10];
  uStack_460 = in_stack_00000028[0xb];
  local_4f8 = in_stack_00000028[0xc];
  uStack_4f0 = in_stack_00000028[0xd];
  local_4e8 = in_stack_00000028[0xe];
  uStack_4e0 = in_stack_00000028[0xf];
  piVar40 = (int *)*in_RDX;
  country_core::resources::gui::EscMenuState::is_closed();
  puVar12 = local_418;
  if (extraout_AL != '\0') {
    *in_stack_00000058 = 0;
    in_stack_00000058[1] = 3;
    *(undefined4 *)(in_stack_00000058 + 2) = 0xc3;
    *(undefined2 *)((longlong)in_stack_00000058 + 0x1c) = 6;
    return;
  }
  *(undefined4 *)in_stack_00000038[2] = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
  local_120 = (int ***)*in_stack_00000038;
  *(undefined4 *)in_R9[2] = *(undefined4 *)((longlong)in_R9 + 0x1c);
  plVar24 = (longlong *)*in_R9;
  local_330 = in_R9;
  if (SBORROW8(0,*plVar24)) {
    uVar20 = local_418[0x1a];
    if (uVar20 == 1) {
      local_f8 = *(longlong **)(local_418 + 0x1b);
      uStack_f0 = *(ulonglong *)(local_418 + 0x1d);
    }
    uVar7 = local_418[0x25];
    local_c4 = CONCAT31(local_c4._1_3_,(char)local_418[0x1f]);
    local_138 = CONCAT71(local_138._1_7_,*(undefined1 *)((longlong)local_418 + 0x7d));
    local_130 = *(longlong *)(local_418 + 0x18);
    local_c0 = *(int ***)(local_418 + 0xe);
    if (SBORROW8(0,(longlong)local_c0)) {
LAB_1402459f7:
      unaff_XMM6_Da = (float)puVar12[0x14];
      unaff_XMM6_Db = puVar12[0x15];
      unaff_XMM6_Dc = puVar12[0x16];
      unaff_XMM6_Dd = puVar12[0x17];
    }
    else if (local_c0 != (int **)0x8000000000000001) {
      alloc::string::impl$6::clone();
      local_1a0 = (code *)uStack_2f0;
      local_320 = local_2e8;
      local_c0 = local_2f8;
      goto LAB_1402459f7;
    }
    local_318 = local_f8;
    uStack_310 = uStack_f0;
    local_170._4_4_ = puVar12[0x20];
    uVar3 = puVar12[0x21];
    local_168 = puVar12[0x22];
    uVar4 = puVar12[0x23];
    unaff_XMM7_Da = (uint)*(undefined8 *)(puVar12 + 0x2a);
    local_b2 = *(undefined1 *)((longlong)puVar12 + 0xc3);
    lVar16 = *(longlong *)(puVar12 + 0x26);
    lVar9 = *(longlong *)(puVar12 + 0x28);
    alloc::vec::impl$11::
    clone<country_core::render::render_config::LastKnownGraphicsDevice,alloc::alloc::Global>();
    if ((char)puVar12[0x2b] == '\x01') {
      local_118 = *(int ***)((longlong)puVar12 + 0xad);
      lStack_110 = *(longlong *)((longlong)puVar12 + 0xb5);
      local_124 = (undefined4)CONCAT71((int7)((ulonglong)extraout_RAX >> 8),1);
    }
    else {
      local_124 = 0;
    }
    uVar5 = *(undefined4 *)((longlong)puVar12 + 0xbd);
    uVar14 = *(undefined1 *)((longlong)puVar12 + 0xc1);
    uVar6 = *(undefined1 *)((longlong)puVar12 + 0xc2);
    uVar8 = puVar12[0xc];
    local_328 = piVar40;
    local_164 = uVar7;
    alloc::vec::impl$11::
    clone<enum2_<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>();
    uVar7 = puVar12[0x24];
    local_2e8 = local_150;
    local_2f8 = local_160;
    uStack_2f0 = (int ***)pcStack_158;
    lStack_2d0 = local_e8;
    local_2c8 = CONCAT44(local_dc,uVar8);
    local_188 = local_118;
    lStack_180 = lStack_110;
    ppuStack_2e0 = (uint **)local_f8;
    lStack_2d8 = uStack_f0;
    plVar24[6] = local_2c8;
    plVar24[4] = uStack_f0;
    plVar24[5] = local_e8;
    plVar24[2] = (longlong)local_150;
    plVar24[3] = (longlong)local_f8;
    *plVar24 = (longlong)local_160;
    plVar24[1] = (longlong)pcStack_158;
    plVar24[7] = (longlong)local_c0;
    plVar24[8] = (longlong)local_1a0;
    plVar24[9] = (longlong)local_320;
    *(float *)(plVar24 + 10) = unaff_XMM6_Da;
    *(uint *)((longlong)plVar24 + 0x54) = unaff_XMM6_Db;
    *(uint *)(plVar24 + 0xb) = unaff_XMM6_Dc;
    *(uint *)((longlong)plVar24 + 0x5c) = unaff_XMM6_Dd;
    plVar24[0xc] = local_130;
    *(uint *)(plVar24 + 0xd) = (uint)(uVar20 == 1);
    *(longlong **)((longlong)plVar24 + 0x6c) = local_318;
    *(ulonglong *)((longlong)plVar24 + 0x74) = uStack_310;
    *(undefined1 *)((longlong)plVar24 + 0x7c) = (undefined1)local_c4;
    *(undefined1 *)((longlong)plVar24 + 0x7d) = (undefined1)local_138;
    *(uint *)(plVar24 + 0x10) = local_170._4_4_;
    *(uint *)((longlong)plVar24 + 0x84) = uVar3;
    *(uint *)(plVar24 + 0x11) = local_168;
    *(uint *)((longlong)plVar24 + 0x8c) = uVar4;
    *(uint *)(plVar24 + 0x12) = uVar7;
    *(uint *)((longlong)plVar24 + 0x94) = local_164;
    plVar24[0x13] = lVar16;
    plVar24[0x14] = lVar9;
    *(uint *)(plVar24 + 0x15) = unaff_XMM7_Da;
    *(char *)((longlong)plVar24 + 0xac) = (char)local_124;
    *(int ***)((longlong)plVar24 + 0xad) = local_118;
    *(longlong *)((longlong)plVar24 + 0xb5) = lStack_110;
    *(undefined4 *)((longlong)plVar24 + 0xbd) = uVar5;
    *(undefined1 *)((longlong)plVar24 + 0xc1) = uVar14;
    *(undefined1 *)((longlong)plVar24 + 0xc2) = uVar6;
    *(undefined1 *)((longlong)plVar24 + 0xc3) = local_b2;
    piVar40 = local_328;
    local_e0 = uVar8;
  }
  if (*piVar40 != 7) {
    return;
  }
  local_1a8 = piVar40 + 2;
  cVar13 = *(char *)(*in_stack_00000050 + 0x58);
  fVar46 = *(float *)(*in_stack_00000048 + 8);
  if (cVar13 == '\x01') {
    if (((0.35 <= fVar46) && (fVar46 <= 0.5)) || ((0.94 <= fVar46 && (fVar46 <= 0.98)))) {
      local_198 = (int *)&DAT_1429a09ee;
      lStack_190 = 0xc;
    }
    else if ((fVar46 < 0.5) || (0.84 < fVar46)) {
      lStack_190 = (ulonglong)(fVar46 <= 0.94 && 0.83 <= fVar46) * 4 + 9;
      local_198 = (int *)&DAT_1429a09cd;
      if (fVar46 <= 0.94 && 0.83 <= fVar46) {
        local_198 = (int *)&DAT_1429a09d6;
      }
    }
    else {
      local_198 = (int *)&DAT_1429a09e3;
      lStack_190 = 0xb;
    }
  }
  else {
    local_198 = (int *)&DAT_1429a09bf;
    lStack_190 = 6;
    if (((fVar46 < 0.345) || (0.46 < fVar46)) && ((fVar46 < 0.913 || (0.926 < fVar46)))) {
      if (((0.46 <= fVar46) && (fVar46 <= 0.48)) || ((0.84 <= fVar46 && (fVar46 <= 0.913)))) {
        lStack_190 = 4;
        local_198 = (int *)&DAT_1429a09c9;
      }
      else if ((fVar46 < 0.48) || (0.84 < fVar46)) {
        if (cVar13 != '\x03') {
          lStack_190 = (ulonglong)(cVar13 == '\x04') * 5 + 5;
          local_198 = (int *)&DAT_1429a09b0;
          if (cVar13 == '\x04') {
            local_198 = (int *)&DAT_1429a09b5;
          }
        }
      }
      else {
        local_198 = (int *)&DAT_1429a09c5;
        lStack_190 = 4;
      }
    }
  }
  local_c0 = (int **)local_1a8;
  country_core::resources::gui::EscMenuState::can_limit_frame_rate();
  if (extraout_AL_00 != '\0') {
    local_c4 = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
    puVar32 = (undefined4 *)in_stack_00000030[2];
    *puVar32 = local_c4;
    plVar24 = (longlong *)*in_stack_00000030;
    local_2f8 = (int **)&DAT_1429a09fa;
    uStack_2f0 = (int ***)0x13;
    toolbar::mei::core::impl$0::as_id();
    uVar37 = extraout_RAX_00 ^ 0xb1695320004fb443;
    uVar34 = uVar37;
    toolbar::mei::core::MeiContext::reuse_or_create_node();
    *(undefined1 *)(extraout_RAX_01 + 0x5d) = 0;
    *(undefined1 *)(extraout_RAX_01 + 0xb7) = 1;
    if ((uVar34 & 1) == 0) {
      lVar16 = plVar24[2];
      if (lVar16 == *plVar24) {
        alloc::raw_vec::RawVec::grow_one<enum2_<iced_core::event::Event>,alloc::alloc::Global>();
      }
      lVar9 = plVar24[1];
      *(undefined4 *)(lVar9 + lVar16 * 0x38) = 0;
      *(ulonglong *)(lVar9 + 0x30 + lVar16 * 0x38) = uVar37;
      plVar24[2] = lVar16 + 1;
    }
    local_e8 = 0;
    local_160 = &local_198;
    pcStack_158 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
    local_2f8 = (int **)&PTR_s_menu_settings__1429a0a38;
    uStack_2f0 = (int ***)0x1;
    local_2e8 = &local_160;
    ppuStack_2e0 = (uint **)0x1;
    lStack_2d8 = 0;
    local_f8 = plVar24;
    uStack_f0 = uVar37;
    alloc::fmt::format::format_inner();
    local_138 = local_370;
    local_510 = (undefined8 *)local_360;
    local_130 = local_368;
    local_518 = (int ***)local_368;
    local_508 = 0x447a0000;
    toolbar::mei::widgets::impl$26::image<ref_<str_>_>();
    toolbar::mei::widgets::ImageView::tint();
    toolbar::mei::widgets::ImageView::blur_extend();
    toolbar::mei::widgets::ActionHintView::into_inner();
    ppiVar30 = local_160;
    if (local_160[0x37] != (int *)0x0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      piVar10 = ppiVar30[0x34];
      piVar11 = ppiVar30[0x35];
      bVar17 = (byte)(extraout_RAX_02 >> 0x39);
      auVar47 = ZEXT216(CONCAT11(bVar17,bVar17));
      auVar47 = pshuflw(auVar47,auVar47,0);
      in_R10 = (int **)0x0;
      uVar34 = extraout_RAX_02;
      while( true ) {
        uVar34 = (ulonglong)piVar11 & uVar34;
        pcVar19 = (char *)((longlong)piVar10 + uVar34);
        cVar13 = auVar47[0];
        auVar52[0] = -(*pcVar19 == cVar13);
        cVar48 = auVar47[1];
        auVar52[1] = -(pcVar19[1] == cVar48);
        cVar49 = auVar47[2];
        auVar52[2] = -(pcVar19[2] == cVar49);
        cVar50 = auVar47[3];
        auVar52[3] = -(pcVar19[3] == cVar50);
        auVar52[4] = -(pcVar19[4] == cVar13);
        auVar52[5] = -(pcVar19[5] == cVar48);
        auVar52[6] = -(pcVar19[6] == cVar49);
        auVar52[7] = -(pcVar19[7] == cVar50);
        auVar52[8] = -(pcVar19[8] == cVar13);
        auVar52[9] = -(pcVar19[9] == cVar48);
        auVar52[10] = -(pcVar19[10] == cVar49);
        auVar52[0xb] = -(pcVar19[0xb] == cVar50);
        auVar52[0xc] = -(pcVar19[0xc] == cVar13);
        auVar52[0xd] = -(pcVar19[0xd] == cVar48);
        auVar52[0xe] = -(pcVar19[0xe] == cVar49);
        auVar52[0xf] = -(pcVar19[0xf] == cVar50);
        uVar29 = (ushort)(SUB161(auVar52 >> 7,0) & 1) | (ushort)(SUB161(auVar52 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar52 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar52 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar52 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar52 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar52 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar52 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar52 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar52 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar52 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar52 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar52 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar52 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar52 >> 0x77,0) & 1) << 0xe | (ushort)(auVar52[0xf] >> 7) << 0xf
        ;
        in_R11D = (uint)uVar29;
        while (uVar29 != 0) {
          uVar20 = 0;
          for (uVar7 = in_R11D; (uVar7 & 1) == 0; uVar7 = uVar7 >> 1 | 0x80000000) {
            uVar20 = uVar20 + 1;
          }
          uVar37 = uVar20 + uVar34 & (ulonglong)piVar11;
          if (pcStack_158 == *(code **)(piVar10 + uVar37 * -0x30 + -0x30)) {
            piVar10[uVar37 * -0x30 + -0x1a] = 0x3f800000;
            if (local_138 != 0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
            goto LAB_140246182;
          }
          uVar29 = (ushort)(in_R11D - 1) & (ushort)in_R11D;
          in_R11D = CONCAT22((short)(in_R11D - 1 >> 0x10),uVar29);
        }
        auVar51[0] = -(*pcVar19 == -1);
        auVar51[1] = -(pcVar19[1] == -1);
        auVar51[2] = -(pcVar19[2] == -1);
        auVar51[3] = -(pcVar19[3] == -1);
        auVar51[4] = -(pcVar19[4] == -1);
        auVar51[5] = -(pcVar19[5] == -1);
        auVar51[6] = -(pcVar19[6] == -1);
        auVar51[7] = -(pcVar19[7] == -1);
        auVar51[8] = -(pcVar19[8] == -1);
        auVar51[9] = -(pcVar19[9] == -1);
        auVar51[10] = -(pcVar19[10] == -1);
        auVar51[0xb] = -(pcVar19[0xb] == -1);
        auVar51[0xc] = -(pcVar19[0xc] == -1);
        auVar51[0xd] = -(pcVar19[0xd] == -1);
        auVar51[0xe] = -(pcVar19[0xe] == -1);
        auVar51[0xf] = -(pcVar19[0xf] == -1);
        if ((((((((((((((((SUB161(auVar51 >> 7,0) & 1) != 0 || (SUB161(auVar51 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar51 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar51 >> 0x1f,0) & 1) != 0) || (SUB161(auVar51 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar51 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar51 >> 0x37,0) & 1) != 0) || (SUB161(auVar51 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar51 >> 0x47,0) & 1) != 0) || (SUB161(auVar51 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar51 >> 0x57,0) & 1) != 0) || (SUB161(auVar51 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar51 >> 0x67,0) & 1) != 0) || (SUB161(auVar51 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar51 >> 0x77,0) & 1) != 0) || auVar51[0xf] < '\0') break;
        uVar34 = (longlong)in_R10 + uVar34 + 0x10;
        in_R10 = in_R10 + 2;
      }
    }
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  local_c4 = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
  puVar32 = (undefined4 *)in_stack_00000030[2];
  *puVar32 = local_c4;
  plVar24 = (longlong *)*in_stack_00000030;
  local_2f8 = (int **)&DAT_1429a09fa;
  uStack_2f0 = (int ***)0x13;
  toolbar::mei::core::impl$0::as_id();
  uVar37 = extraout_RAX_03 ^ 0xb1695320004fb443;
  uVar34 = uVar37;
  toolbar::mei::core::MeiContext::reuse_or_create_node();
  *(undefined1 *)(extraout_RAX_04 + 0x5d) = 0;
  *(undefined1 *)(extraout_RAX_04 + 0xb7) = 1;
  if ((uVar34 & 1) == 0) {
    lVar16 = plVar24[2];
    if (lVar16 == *plVar24) {
      alloc::raw_vec::RawVec::grow_one<enum2_<iced_core::event::Event>,alloc::alloc::Global>();
    }
    lVar9 = plVar24[1];
    *(undefined4 *)(lVar9 + lVar16 * 0x38) = 0;
    *(ulonglong *)(lVar9 + 0x30 + lVar16 * 0x38) = uVar37;
    plVar24[2] = lVar16 + 1;
  }
  local_e8 = 0;
  local_f8 = plVar24;
  uStack_f0 = uVar37;
  toolbar::mei::config::impl$18::template();
LAB_140246182:
  *puVar32 = local_c4;
  local_2f8 = (int **)&DAT_1429a0a4a;
  uStack_2f0 = (int ***)0x8;
  toolbar::mei::core::impl$0::as_id();
  uVar37 = extraout_RAX_05 ^ 0xb1695320004fb443;
  uVar34 = uVar37;
  toolbar::mei::core::MeiContext::reuse_or_create_node();
  pppiVar33 = local_120;
  *(undefined1 *)(extraout_RAX_06 + 0x5d) = 0;
  *(undefined1 *)(extraout_RAX_06 + 0xb7) = 1;
  if ((uVar34 & 1) == 0) {
    lVar16 = plVar24[2];
    if (lVar16 == *plVar24) {
      alloc::raw_vec::RawVec::grow_one<enum2_<iced_core::event::Event>,alloc::alloc::Global>();
    }
    lVar9 = plVar24[1];
    *(undefined4 *)(lVar9 + lVar16 * 0x38) = 1;
    *(ulonglong *)(lVar9 + 0x30 + lVar16 * 0x38) = uVar37;
    plVar24[2] = lVar16 + 1;
  }
  local_308 = (byte *)0x0;
  if (*piVar40 == 7) {
    bVar17 = *(char *)local_c0 - 3;
    if (bVar17 < 8 && bVar17 != 1) {
      bVar41 = false;
    }
    else {
      bVar41 = *(char *)local_c0 == '\x01';
    }
  }
  else {
    bVar41 = false;
  }
  local_318 = plVar24;
  uStack_310 = uVar37;
  local_b3 = bVar41;
  toolbar::mei::widgets::impl$26::tab_switcher<ref_<str_>_>();
  toolbar::mei::widgets::TabSwitcherView::delta();
  local_2e8 = local_480;
  local_2f8 = (int **)local_490;
  uStack_2f0 = (int ***)uStack_488;
  toolbar::mei::widgets::ActionHintView::into_inner();
  toolbar::mei::config::impl$18::template();
  if (*(char *)local_c0 != '\n') {
    if (bVar41 == false) {
      toolbar::mei::config::TemplateView::
      with_slot<tiny_glade::gui::settings::do_gui_escape_menu_settings::closure_env_1,ref_<str_>_>()
      ;
      local_2f8 = local_c0;
      uStack_2f0 = pppiVar33;
      local_2e8 = (int ***)&local_170;
      toolbar::mei::config::TemplateView::
      with_slot<tiny_glade::gui::settings::do_gui_escape_menu_settings::closure_env_2,ref_<str_>_>()
      ;
    }
    local_118 = (int **)0x0;
    lStack_110 = 1;
    local_108 = 0;
    local_188 = (int **)CONCAT71(local_188._1_7_,1);
    local_2f8 = &local_1a8;
    uStack_2f0 = (int ***)in_stack_00000040;
    ppuStack_2e0 = &local_418;
    lStack_2d8 = local_478;
    lStack_2d0 = lStack_470;
    local_2c8 = local_468;
    uStack_2c0 = uStack_460;
    local_2a8 = (longlong *)local_3a8;
    lStack_2a0 = uStack_3a0;
    local_2b8 = local_3b8;
    puStack_2b0 = (undefined4 *)uStack_3b0;
    local_298 = (int ****)*in_RCX;
    plStack_290 = (longlong *)in_RCX[1];
    local_288 = (int ****)in_RCX[2];
    uStack_280 = in_RCX[3];
    local_278 = local_398;
    uStack_270 = uStack_390;
    local_268 = (int ***)local_388;
    uStack_260 = uStack_380;
    local_248 = local_428;
    lStack_240 = lStack_420;
    local_258 = local_438;
    lStack_250 = lStack_430;
    local_228 = local_448;
    lStack_220 = lStack_440;
    local_238 = local_458;
    lStack_230 = lStack_450;
    local_218 = pppiVar33;
    local_210 = &local_4f8;
    local_208 = &local_188;
    local_1d8 = local_3c8;
    uStack_1d0 = uStack_3c0;
    local_1e8 = local_3d8;
    lStack_1e0 = lStack_3d0;
    local_1b8 = local_3e8;
    uStack_1b0 = uStack_3e0;
    local_1c8 = local_3f8;
    lStack_1c0 = lStack_3f0;
    local_200 = local_330;
    local_1f0 = &local_4d8;
    local_2e8 = &local_118;
    local_1f8 = &local_4b8;
    toolbar::mei::config::TemplateView::
    with_slot<tiny_glade::gui::settings::do_gui_escape_menu_settings::closure_env_3,ref_<str_>_>();
    local_518 = pppiVar33;
    toolbar::mei::config::TemplateView::
    with_slot<tiny_glade::gui::settings::do_gui_escape_menu_settings::closure_env_4,ref_<str_>_>();
    if ((char)local_188 != '\0') {
      lStack_2d8 = local_140;
      local_2e8 = local_150;
      ppuStack_2e0 = ppuStack_148;
      local_2f8 = local_160;
      uStack_2f0 = (int ***)pcStack_158;
      toolbar::mei::config::TemplateView::into_slot<ref_<str_>_>();
      local_518 = (int ***)&DAT_1429a0ab5;
      local_510 = (undefined8 *)0xa;
      toolbar::mei::config::impl$18::template_with_id<ref_<str_>_>();
      if (local_108 != 0) {
        local_518 = &local_118;
        toolbar::mei::config::TemplateView::
        with_slot<tiny_glade::gui::settings::do_gui_escape_menu_settings::closure_env_5,ref_<str_>_>
                  ();
      }
    }
    if (local_118 == (int **)0x0) {
      return;
    }
    std::alloc::__default_lib_allocator::__rust_dealloc();
    return;
  }
  toolbar::mei::config::TemplateView::
  with_slot<tiny_glade::gui::settings::ui_settings_keybindings::closure_env_0,ref_<str_>_>();
  plVar24 = in_stack_00000058 + 2;
  ppiVar30 = (int **)((longlong)in_stack_00000058 + 0x1c);
  local_338 = plVar24;
  local_188 = ppiVar30;
  if ((char)in_stack_00000058[1] == '\x03') {
    if ((int)extraout_EAX < 1) {
      if (-1 < (int)extraout_EAX) goto LAB_140246c9c;
      lVar16 = *in_stack_00000058;
      uVar20 = extraout_EAX;
      if ((-extraout_EAX & 3) != 0) {
        iVar18 = 0;
        do {
          lVar16 = *(longlong *)(&DAT_1429a1680 + lVar16 * 8);
          iVar18 = iVar18 + -1;
        } while (-(-extraout_EAX & 3) != iVar18);
        *in_stack_00000058 = lVar16;
        uVar20 = extraout_EAX - iVar18;
      }
      if (0xfffffffc < extraout_EAX) goto LAB_140246c9c;
      do {
        lVar16 = *(longlong *)
                  (&DAT_1429a1680 +
                  *(longlong *)
                   (&DAT_1429a1680 +
                   *(longlong *)(&DAT_1429a1680 + *(longlong *)(&DAT_1429a1680 + lVar16 * 8) * 8) *
                   8) * 8);
        uVar20 = uVar20 + 4;
      } while (uVar20 != 0);
    }
    else {
      lVar16 = *in_stack_00000058;
      uVar20 = extraout_EAX;
      if ((extraout_EAX & 3) != 0) {
        uVar20 = 0;
        do {
          lVar16 = *(longlong *)(&DAT_1429a1648 + lVar16 * 8);
          uVar20 = uVar20 + 1;
        } while ((extraout_EAX & 3) != uVar20);
        *in_stack_00000058 = lVar16;
        uVar20 = extraout_EAX - uVar20;
      }
      if (extraout_EAX < 4) goto LAB_140246c9c;
      iVar18 = uVar20 + 4;
      do {
        lVar16 = *(longlong *)
                  (&DAT_1429a1648 +
                  *(longlong *)
                   (&DAT_1429a1648 +
                   *(longlong *)(&DAT_1429a1648 + *(longlong *)(&DAT_1429a1648 + lVar16 * 8) * 8) *
                   8) * 8);
        iVar18 = iVar18 + -4;
      } while (4 < iVar18);
    }
    *in_stack_00000058 = lVar16;
LAB_140246c9c:
    toolbar::mei::config::TemplateView::
    with_slot<tiny_glade::gui::settings::ui_settings_keybindings::closure_env_7,ref_<str_>_>();
    toolbar::mei::config::TemplateView::
    with_slot<tiny_glade::gui::settings::ui_settings_keybindings::closure_env_8,ref_<str_>_>();
    toolbar::mei::config::TemplateView::
    with_slot<tiny_glade::gui::settings::ui_settings_keybindings::closure_env_9,ref_<str_>_>();
    return;
  }
  ppppiVar39 = (int ****)(in_stack_00000058 + 1);
  pauVar35 = (undefined1 (*) [16])&DAT_1429a0a4a;
  pppiVar21 = &local_160;
  puVar36 = (undefined *)0x8;
  toolbar::mei::config::TemplateView::slot<ref_<str_>_>();
  pcVar19 = &DAT_1429a16b8;
  uVar34 = (ulonglong)*(int *)(&DAT_1429a16b8 + (ulonglong)*(byte *)ppppiVar39 * 4);
  bVar41 = 0xfffffffebd65e947 < uVar34;
  bVar45 = SCARRY8(uVar34,0x1429a16b8);
  pbVar15 = &DAT_1429a16b8 + uVar34;
  bVar43 = (longlong)pbVar15 < 0;
  bVar42 = pbVar15 == (byte *)0x0;
  cVar13 = (char)pbVar15;
  puVar27 = auStack_538;
  puVar28 = auStack_538;
  puVar25 = auStack_538;
  ppiVar31 = ppiVar30;
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
  switch(*(byte *)ppppiVar39) {
  case 4:
                    // WARNING: Could not emulate address calculation at 0x000140249e6c
                    // WARNING: Treating indirect jump as call
    (*(code *)(&DAT_1429a16c8 + *(int *)(&DAT_1429a16c8 + (longlong)pbVar15 * 4)))();
    return;
  default:
    *pbVar15 = *pbVar15 + cVar13;
    goto LAB_140249ec6;
  case 0x18:
code_r0x000140249eaa:
    goto LAB_140249ec6;
  case 0x22:
    if (!bVar42) {
      pbVar15 = &DAT_00000008;
      goto code_r0x000140249eaa;
    }
    pbVar15 = &DAT_00000007;
  case 0x10:
LAB_140249ec6:
    local_308 = pbVar15;
    lang_lib::lookup();
    lang_lib::lookup();
    toolbar::mei::widgets::tweaker::HintTracker::text();
    toolbar::mei::widgets::tweaker::impl$11::default();
    plStack_290 = (longlong *)0x9;
    local_288 = (int ****)&local_308;
    local_518 = (int ***)CONCAT44(local_518._4_4_,extraout_XMM0_Da_02);
    local_298 = ppppiVar39;
    toolbar::mei::widgets::tweaker::impl$13::
    tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2_<tiny_glade::gui::settings::FpsCap>_>_>
              ();
    if (extraout_AL_06 != '\0') {
      *local_1a0 = (code)0x1;
      if (&DAT_00000008 < local_308) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      fVar46 = 0.0;
      if ((*(byte *)(&local_3b8 + (longlong)local_308) & 1) == 0) {
        fVar46 = (float)*(uint *)((longlong)&local_3b8 + (longlong)local_308 * 8 + 4);
      }
      *puStack_2b0 = (int)ppiVar30;
      *(float *)(pppiVar33 + 0x13) = fVar46;
    }
    return;
  case 0x23:
  case 0x26:
    goto code_r0x000140248929;
  case 0x24:
    *pbVar15 = *pbVar15 + cVar13;
    pcVar19 = (char *)((longlong)
                       &
                       PTR_run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void_(*)(bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::structural_graph::StairsStructuralGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>),void_(*)(bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::structural_graph::StairsStructuralGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>)>_>_>_1429a9f70
                      + uVar34 + 7);
    *pcVar19 = *pcVar19 + cVar13;
    *pbVar15 = *pbVar15 + cVar13;
code_r0x000140248929:
    local_398 = local_398 & 0xffffffff00000000;
    alloc::fmt::format::format_inner();
    local_3f8 = pppiStack_410;
    lStack_3f0 = local_408;
    local_3e8 = lStack_400;
    if (*(char *)(pppiVar33 + 0x11) == '\0') {
code_r0x000140248971:
      fVar46 = 1.0;
    }
    else {
      fVar46 = *(float *)((longlong)pppiVar33 + 0x8c);
    }
    local_418 = (uint *)((longlong)local_268 + 0x18);
    if (*(float *)((longlong)local_268 + 0x18) <= fVar46 &&
        fVar46 != *(float *)((longlong)local_268 + 0x18)) {
                    // WARNING: Could not recover jumptable at 0x0001402489b3. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_1429a1744 + *(int *)(&DAT_1429a1744 + *local_2a8 * 4)))();
      return;
    }
    lStack_2d8 = lStack_400;
    local_2e8 = pppiStack_410;
    ppuStack_2e0 = (uint **)local_408;
    uStack_2f0 = (int ***)((ulonglong)uStack_2f0 & 0xffffffff);
    local_298 = &local_2e8;
    local_3b8 = (int ***)&DAT_1429a11a0;
    uStack_3b0 = 2;
    local_3a8 = &local_298;
    uStack_3a0 = 1;
    local_398 = 0;
    plStack_290 = plVar24;
    alloc::fmt::format::format_inner();
    lStack_2a0 = local_458;
    local_258 = (int ***)lStack_450;
    uVar22 = 0x1f;
    lang_lib::lookup();
    local_298 = &local_428;
    local_288 = &local_438;
    local_3b8 = (int ***)&DAT_1429a1020;
    uStack_3b0 = 2;
    local_3a8 = &local_298;
    uStack_3a0 = 2;
    local_398 = 0;
    lStack_430 = uVar22;
    alloc::fmt::format::format_inner();
    local_3d8 = local_308;
    local_268 = (int ***)uStack_300;
    toolbar::mei::widgets::tweaker::HintTracker::text();
    toolbar::mei::widgets::tweaker::impl$11::default();
    fVar46 = unaff_XMM6_Da;
    if (0.1 <= unaff_XMM6_Da) {
      fVar46 = 0.1;
    }
    local_3b8 = (int ***)((longlong)&uStack_3c0 + 4);
    local_3a8._0_5_ = (uint5)unaff_XMM7_Da;
    uStack_3b0 = CONCAT44(fVar46,0x3dcccccd);
    local_518 = (int ***)CONCAT44(local_518._4_4_,extraout_XMM0_Da);
    local_510 = &local_3b8;
    toolbar::mei::widgets::tweaker::impl$13::
    tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep>();
    if (extraout_AL_01 == '\0') {
      if (local_3d8 != (byte *)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
    else {
      if (local_3d8 != (byte *)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      *(uint *)ppiVar30 = (uint)local_248;
      *(undefined4 *)(pppiVar33 + 0x11) = 1;
      *(float *)((longlong)pppiVar33 + 0x8c) = uStack_3c0._4_4_;
      fVar46 = unaff_XMM6_Da;
      if (uStack_3c0._4_4_ <= unaff_XMM6_Da) {
        fVar46 = uStack_3c0._4_4_;
      }
      *local_188[2] = *(uint *)((longlong)local_188 + 0x1c);
      *local_418 = ~-(uint)NAN(uStack_3c0._4_4_) & (uint)fVar46 |
                   -(uint)NAN(uStack_3c0._4_4_) & (uint)unaff_XMM6_Da;
      **(undefined4 **)(lStack_180 + 0x10) = *(undefined4 *)(lStack_180 + 0x1c);
      *local_2a8 = 0;
      *local_1a0 = (code)0x1;
      ppppiVar39 = (int ****)0x1;
    }
    if (lStack_2a0 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if (local_2e8 != (int ***)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if ((uStack_2f0._4_1_ != '\0') && (local_3f8 != (int ***)0x0)) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    toolbar::mei::widgets::impl$26::space();
    uVar22 = 0x17;
    lang_lib::lookup();
    uVar23 = 9;
    lStack_430 = uVar22;
    lang_lib::lookup();
    local_298 = &local_438;
    local_288 = &local_3f8;
    local_3b8 = (int ***)&DAT_1429a11f0;
    uStack_3b0 = 3;
    uStack_3a0 = 2;
    local_398 = 0;
    lStack_3f0 = uVar23;
    local_3a8 = &local_298;
    alloc::fmt::format::format_inner();
    local_268 = local_2e8;
    local_258 = (int ***)ppuStack_2e0;
    toolbar::mei::widgets::tweaker::HintTracker::text();
    toolbar::mei::widgets::tweaker::impl$11::default();
    *(uint *)ppiVar30 = (uint)local_248;
    local_510 = (undefined8 *)((longlong)pppiVar33 + 0xaa);
    local_518 = (int ***)CONCAT44(local_518._4_4_,extraout_XMM0_Da_00);
    toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableBool>
              ();
    cVar38 = (code)((byte)ppppiVar39 | extraout_AL_02);
    *local_1a0 = cVar38;
    if (local_268 != (int ***)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    lang_lib::lookup();
    toolbar::mei::widgets::tweaker::HintTracker::text();
    toolbar::mei::widgets::tweaker::impl$11::default();
    *(uint *)ppiVar30 = (uint)local_248;
    local_3b8 = pppiVar33 + 0x14;
    local_3a8._0_5_ = 0x40000000;
    uStack_3b0 = 0xc00000003e4ccccd;
    local_510 = &local_3b8;
    toolbar::mei::widgets::tweaker::impl$13::
    tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep>();
    cVar38 = (code)((byte)cVar38 | extraout_AL_03);
    *local_1a0 = cVar38;
    lang_lib::lookup();
    toolbar::mei::widgets::tweaker::HintTracker::text();
    toolbar::mei::widgets::tweaker::impl$11::default();
    *(uint *)ppiVar30 = (uint)local_248;
    local_3b8 = (int ***)((longlong)pppiVar33 + 0xa4);
    local_3a8._0_5_ = 0x3f000000;
    uStack_3b0 = 0xbf0000003d4ccccd;
    local_510 = &local_3b8;
    toolbar::mei::widgets::tweaker::impl$13::
    tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep>();
    cVar38 = (code)((byte)cVar38 | extraout_AL_04);
    *local_1a0 = cVar38;
    lang_lib::lookup();
    lang_lib::lookup();
    toolbar::mei::widgets::tweaker::HintTracker::text();
    toolbar::mei::widgets::tweaker::impl$11::default();
    *(uint *)ppiVar30 = (uint)local_248;
    local_3b8 = (int ***)((longlong)pppiVar33 + 0x9c);
    local_3a8 = (int *****)CONCAT35(local_3a8._5_3_,0x3f800000);
    uStack_3b0 = 0x3dcccccd;
    local_518 = (int ***)CONCAT44(local_518._4_4_,extraout_XMM0_Da_01);
    local_510 = &local_3b8;
    toolbar::mei::widgets::tweaker::impl$13::
    tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep>();
    *local_1a0 = (code)((byte)cVar38 | extraout_AL_05);
    if (lStack_3d0 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if (lStack_470 == 0) {
      return;
    }
    std::alloc::__default_lib_allocator::__rust_dealloc();
    return;
  case 0x25:
    goto code_r0x000140248971;
  case 0x27:
    pcVar19 = (char *)0xfd8a87ddfd8a87dd;
    pauVar35 = (undefined1 (*) [16])0xfd8a87ddfd8a87ed;
    ppiVar30 = (int **)(ulonglong)
                       ~(uint)(ushort)((ushort)(SUB161(auRamfd8a87ddfd8a87dd >> 7,0) & 1) |
                                       (ushort)(SUB161(auRamfd8a87ddfd8a87dd >> 0xf,0) & 1) << 1 |
                                       (ushort)(SUB161(auRamfd8a87ddfd8a87dd >> 0x17,0) & 1) << 2 |
                                       (ushort)(SUB161(auRamfd8a87ddfd8a87dd >> 0x1f,0) & 1) << 3 |
                                       (ushort)(SUB161(auRamfd8a87ddfd8a87dd >> 0x27,0) & 1) << 4 |
                                       (ushort)(SUB161(auRamfd8a87ddfd8a87dd >> 0x2f,0) & 1) << 5 |
                                       (ushort)(SUB161(auRamfd8a87ddfd8a87dd >> 0x37,0) & 1) << 6 |
                                       (ushort)(SUB161(auRamfd8a87ddfd8a87dd >> 0x3f,0) & 1) << 7 |
                                       (ushort)(SUB161(auRamfd8a87ddfd8a87dd >> 0x47,0) & 1) << 8 |
                                       (ushort)(SUB161(auRamfd8a87ddfd8a87dd >> 0x4f,0) & 1) << 9 |
                                       (ushort)(SUB161(auRamfd8a87ddfd8a87dd >> 0x57,0) & 1) << 10 |
                                       (ushort)(SUB161(auRamfd8a87ddfd8a87dd >> 0x5f,0) & 1) << 0xb
                                       | (ushort)(SUB161(auRamfd8a87ddfd8a87dd >> 0x67,0) & 1) <<
                                         0xc | (ushort)(SUB161(auRamfd8a87ddfd8a87dd >> 0x6f,0) & 1)
                                               << 0xd |
                                       (ushort)(SUB161(auRamfd8a87ddfd8a87dd >> 0x77,0) & 1) << 0xe
                                      | (ushort)(byte)(auRamfd8a87ddfd8a87dd[0xf] >> 7) << 0xf);
    in_R10 = *pppiVar21;
    puVar36 = &DAT_1429a1754;
    do {
      if ((short)ppiVar30 == 0) {
        do {
          auVar47 = *pauVar35;
          pcVar19 = pcVar19 + -0xc0;
          pauVar35 = pauVar35 + 1;
          in_R11D = (uint)(ushort)((ushort)(SUB161(auVar47 >> 7,0) & 1) |
                                   (ushort)(SUB161(auVar47 >> 0xf,0) & 1) << 1 |
                                   (ushort)(SUB161(auVar47 >> 0x17,0) & 1) << 2 |
                                   (ushort)(SUB161(auVar47 >> 0x1f,0) & 1) << 3 |
                                   (ushort)(SUB161(auVar47 >> 0x27,0) & 1) << 4 |
                                   (ushort)(SUB161(auVar47 >> 0x2f,0) & 1) << 5 |
                                   (ushort)(SUB161(auVar47 >> 0x37,0) & 1) << 6 |
                                   (ushort)(SUB161(auVar47 >> 0x3f,0) & 1) << 7 |
                                   (ushort)(SUB161(auVar47 >> 0x47,0) & 1) << 8 |
                                   (ushort)(SUB161(auVar47 >> 0x4f,0) & 1) << 9 |
                                   (ushort)(SUB161(auVar47 >> 0x57,0) & 1) << 10 |
                                   (ushort)(SUB161(auVar47 >> 0x5f,0) & 1) << 0xb |
                                   (ushort)(SUB161(auVar47 >> 0x67,0) & 1) << 0xc |
                                   (ushort)(SUB161(auVar47 >> 0x6f,0) & 1) << 0xd |
                                   (ushort)(SUB161(auVar47 >> 0x77,0) & 1) << 0xe |
                                  (ushort)(byte)(auVar47[0xf] >> 7) << 0xf);
        } while (in_R11D == 0xffff);
        in_R11D = ~in_R11D;
      }
      else {
        in_R11D = (uint)ppiVar30;
      }
joined_r0x00014024b455:
      if (in_R10 < (int **)0xb) {
        uVar20 = 0;
        for (uVar7 = in_R11D; (uVar7 & 1) == 0; uVar7 = uVar7 >> 1 | 0x80000000) {
          uVar20 = uVar20 + 1;
        }
        if (*(int *)(auStack_530 + (longlong)in_R10 * 6 + -2) ==
            *(int *)(pcVar19 + (ulonglong)uVar20 * -0xc + -0xc)) {
          if (*(int *)(auStack_530 + (longlong)in_R10 * 6 + -2) != 0) {
            in_R10 = (int **)((longlong)in_R10 + 1);
            *pppiVar21 = in_R10;
            if (in_R10 == (int **)0xb) {
              SHOW_DEVELOPER_SETTINGS = 1;
              in_R10 = (int **)0xb;
            }
            goto LAB_14024b40a;
          }
          if (auStack_530[(longlong)in_R10 * 6] ==
              *(ushort *)(pcVar19 + (ulonglong)uVar20 * -0xc + -8)) {
                    // WARNING: Could not recover jumptable at 0x00014024b4b3. Too many branches
                    // WARNING: Treating indirect jump as call
            (*(code *)(puVar36 +
                      *(int *)(puVar36 + (ulonglong)auStack_530[(longlong)in_R10 * 6] * 4)))();
            return;
          }
        }
        *pppiVar21 = (int **)0x0;
        in_R10 = (int **)0x0;
      }
LAB_14024b40a:
      ppiVar30 = (int **)(ulonglong)(in_R11D - 1 & in_R11D);
      pbVar15 = pbVar15 + -1;
      bVar42 = pbVar15 == (byte *)0x0;
code_r0x00014024b414:
      if (bVar42) {
        return;
      }
    } while( true );
  case 0x28:
  case 0x2b:
    *(char *)((longlong)in_stack_00000058 + -0x5f) =
         *(char *)((longlong)in_stack_00000058 + -0x5f) + (char)((ulonglong)pbVar15 >> 8);
    pbVar15 = (byte *)CONCAT62((int6)((ulonglong)pbVar15 >> 0x10),
                               CONCAT11((char)((ushort)pbVar15 %
                                              (ushort)*(byte *)((longlong)pppiVar21 + 0x1429a16f9)),
                                        (char)((ushort)pbVar15 /
                                              (ushort)*(byte *)((longlong)pppiVar21 + 0x1429a16f9)))
                              );
    goto joined_r0x00014024b455;
  case 0x29:
  case 0x2a:
    pcVar19 = "";
    goto code_r0x00014024b414;
  case 0x2c:
  case 0x2d:
  case 0x2e:
  case 0x32:
  case 0x33:
  case 0x35:
  case 0x37:
    pbVar1 = &UNK_1429a1642 + uVar34;
    bVar41 = false;
    bVar45 = false;
    *pbVar1 = *pbVar1 | (byte)pppiVar21;
    bVar2 = *pbVar1;
    bVar17 = *pbVar1;
    bVar44 = 1;
    while (bVar43 = (char)bVar2 < '\0', (POPCOUNT(bVar17) & 1U) == 0) {
      plVar24 = (longlong *)CONCAT62((int6)((ulonglong)plVar24 >> 0x10),(ushort)(byte)plVar24);
      puVar27 = puVar26;
code_r0x0001429a16c0:
      if (bVar45 == bVar43) goto code_r0x0001429a1714;
      plVar24 = (longlong *)CONCAT62((int6)((ulonglong)plVar24 >> 0x10),(ushort)(byte)plVar24);
      *(char *)((longlong)pppiVar33 + -0x76) =
           *(char *)((longlong)pppiVar33 + -0x76) + (byte)pppiVar21;
      bVar44 = 1;
      puVar28 = puVar27;
      ppiVar31 = ppiVar30;
code_r0x0001429a1681:
      ppiVar30 = (int **)((longlong)ppiVar31 + ((ulonglong)bVar44 * -2 + 1) * 4);
      uVar20 = *(uint *)ppiVar31;
      pbVar15 = (byte *)(ulonglong)uVar20;
      LOCK();
      uVar5 = *(undefined4 *)((longlong)pppiVar21 + -0x75782203);
      *(int *)((longlong)pppiVar21 + -0x75782203) = (int)pcVar19;
      pcVar19 = (char *)CONCAT44((int)((ulonglong)pcVar19 >> 0x20),uVar5);
      UNLOCK();
      bVar44 = 1;
      plVar24 = (longlong *)
                CONCAT62((int6)((ulonglong)plVar24 >> 0x10),
                         CONCAT11((char)((uint)uVar5 >> 8),(char)plVar24));
      cVar13 = (char)uVar20;
      *pbVar15 = *pbVar15 + cVar13;
      *pbVar15 = *pbVar15 + cVar13;
      *pbVar15 = *pbVar15 + cVar13;
      *pbVar15 = *pbVar15 + cVar13;
      *pbVar15 = *pbVar15 + cVar13;
      *pbVar15 = *pbVar15 + cVar13;
      *pbVar15 = *pbVar15 + cVar13;
      *pcVar19 = *pcVar19 + cVar13;
      *pbVar15 = *pbVar15 + cVar13;
      *pbVar15 = *pbVar15 + cVar13;
      *pbVar15 = *pbVar15 + cVar13;
      *(char *)pppiVar21 = *(char *)pppiVar21 + cVar13;
      *pbVar15 = *pbVar15 + cVar13;
      *pbVar15 = *pbVar15 + cVar13;
      *pbVar15 = *pbVar15 + cVar13;
      *(char *)plVar24 = (char)*plVar24 + cVar13;
      *pbVar15 = *pbVar15 + cVar13;
      *pbVar15 = *pbVar15 + cVar13;
      *pbVar15 = *pbVar15 + cVar13;
      *(char *)((longlong)pbVar15 * 2) = *(char *)((longlong)pbVar15 * 2) + cVar13;
      *pbVar15 = *pbVar15 + cVar13;
      *pbVar15 = *pbVar15 + cVar13;
      *pbVar15 = *pbVar15 + cVar13;
      *pbVar15 = *pbVar15 + cVar13;
      puVar25 = puVar28;
code_r0x0001429a16b7:
      bVar17 = (byte)pcVar19;
      bVar41 = CARRY1(*pbVar15,bVar17);
      bVar45 = SCARRY1(*pbVar15,bVar17);
      *pbVar15 = *pbVar15 + bVar17;
      bVar2 = *pbVar15;
      bVar17 = *pbVar15;
      puVar26 = puVar25 + -8;
      *(byte **)(puVar25 + -8) = pbVar15;
      plVar24 = (longlong *)CONCAT62((int6)((ulonglong)plVar24 >> 0x10),(ushort)(byte)plVar24);
    }
    LOCK();
    uVar20 = *(uint *)((longlong)pppiVar21 + -0x75782203);
    *(uint *)((longlong)pppiVar21 + -0x75782203) = (uint)pcVar19;
    UNLOCK();
    XACQUIRE();
    LOCK();
    uVar7 = *(uint *)((longlong)pppiVar21 + -0x75782203);
    *(uint *)((longlong)pppiVar21 + -0x75782203) = uVar20;
    pcVar19 = (char *)(ulonglong)uVar7;
    UNLOCK();
    bVar44 = 1;
    uVar14 = in(0x87);
    pbVar15 = (byte *)CONCAT71((int7)((ulonglong)pbVar15 >> 8),uVar14);
code_r0x0001429a1714:
    while( true ) {
      if (!bVar45) {
                    // WARNING: Bad instruction - Truncating control flow here
        halt_baddata();
      }
      uVar34 = (ulonglong)bVar44;
      *(int *)pppiVar33 = (int)pbVar15;
      if (!bVar41) break;
      LOCK();
      uVar20 = *(uint *)((longlong)pppiVar21 + -0x75782203);
      *(uint *)((longlong)pppiVar21 + -0x75782203) = (uint)pcVar19;
      UNLOCK();
      bVar44 = 1;
      pcVar19 = (char *)(ulonglong)uVar20;
      pppiVar33 = (int ***)((longlong)pppiVar33 + uVar34 * -8 + 4);
    }
    out(*(uint *)ppiVar30,(short)pppiVar21);
    out(*(uint *)((longlong)ppiVar30 + -4),(short)pppiVar21);
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x2f:
  case 0x30:
  case 0x31:
    goto code_r0x0001429a16b7;
  case 0x34:
    goto code_r0x0001429a1681;
  case 0x36:
    goto code_r0x0001429a16c0;
  }
}

