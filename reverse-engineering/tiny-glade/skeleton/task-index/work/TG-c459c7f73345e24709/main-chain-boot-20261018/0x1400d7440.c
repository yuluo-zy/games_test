
// WARNING: Function: __chkstk replaced with injection: alloca_probe

void __cdecl
std::sys::backtrace::
__rust_begin_short_backtrace<tiny_glade::run_game::run_game_thread::closure_env_8,tuple_<>_>(void)

{
  void *_Src;
  uint *puVar1;
  char *pcVar2;
  uint uVar3;
  longlong *plVar4;
  uint uVar5;
  undefined *puVar6;
  byte *extraout_RAX;
  longlong extraout_RAX_00;
  byte *extraout_RAX_01;
  undefined **extraout_RAX_02;
  byte *extraout_RAX_03;
  longlong extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  longlong in_RCX;
  longlong lVar7;
  undefined **_Src_00;
  ulonglong uVar8;
  longlong lVar9;
  longlong lVar10;
  uint uVar11;
  ushort uVar12;
  ulonglong uVar13;
  ulonglong uVar14;
  undefined1 auVar15 [16];
  undefined1 auVar16 [16];
  undefined1 auVar17 [16];
  undefined1 auVar18 [16];
  undefined1 local_1058 [208];
  undefined1 local_f88 [240];
  undefined1 local_e98 [240];
  undefined *local_da8;
  undefined *puStack_da0;
  undefined8 local_d98;
  undefined8 uStack_d90;
  longlong local_d88;
  undefined **ppuStack_d80;
  undefined *local_778;
  undefined *puStack_770;
  undefined8 local_768;
  undefined8 uStack_760;
  longlong local_758;
  undefined **ppuStack_750;
  undefined *local_748;
  undefined *puStack_740;
  undefined8 local_738;
  undefined8 uStack_730;
  undefined **local_728;
  undefined **ppuStack_720;
  undefined8 local_718;
  undefined1 local_710 [16];
  undefined8 local_148;
  undefined8 local_140;
  undefined8 local_138;
  undefined *local_130;
  undefined *puStack_128;
  undefined8 local_120;
  undefined8 uStack_118;
  longlong local_110;
  undefined **ppuStack_108;
  undefined *local_100;
  undefined *puStack_f8;
  undefined8 local_f0;
  undefined8 uStack_e8;
  undefined **local_e0;
  undefined **ppuStack_d8;
  undefined *local_d0;
  undefined8 uStack_c8;
  undefined8 local_c0;
  undefined8 uStack_b8;
  longlong local_b0;
  undefined **local_a8;
  undefined8 *local_78;
  longlong local_70;
  undefined1 local_63;
  undefined1 local_62;
  undefined1 local_61;
  undefined8 local_60;
  
  local_60 = 0xfffffffffffffffe;
  local_78 = (undefined8 *)(in_RCX + 0x638);
  local_63 = 1;
  local_70 = in_RCX;
  rhapsody::rg::executor::GraphExecutor::new();
  rhapsody::rg::state::impl$0::default();
  rhapsody::rg::state::impl$0::default();
  memcpy(local_1058,&local_148,0xd0);
  memcpy(local_f88,&local_da8,0xf0);
  _Src_00 = &local_778;
  memcpy(local_e98,_Src_00,0xf0);
  hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX & 1) == 0) {
    local_62 = 1;
    random::hashmap_random_keys();
    extraout_RAX[0] = 1;
    extraout_RAX[1] = 0;
    extraout_RAX[2] = 0;
    extraout_RAX[3] = 0;
    extraout_RAX[4] = 0;
    extraout_RAX[5] = 0;
    extraout_RAX[6] = 0;
    extraout_RAX[7] = 0;
    *(longlong *)(extraout_RAX + 8) = extraout_RAX_00;
    *(undefined ***)(extraout_RAX + 0x10) = _Src_00;
    local_d88 = extraout_RAX_00;
  }
  else {
    local_d88 = *(longlong *)(extraout_RAX + 8);
    _Src_00 = *(undefined ***)(extraout_RAX + 0x10);
  }
  *(longlong *)(extraout_RAX + 8) = local_d88 + 1;
  local_d98 = 0;
  uStack_d90 = 0;
  local_da8 = &anon_5d8eb308df20e26c7ef69d65e24a3f84_4_llvm_17447356362437668641;
  puStack_da0 = (undefined *)0x0;
  ppuStack_d80 = _Src_00;
  hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX_01 & 1) == 0) {
    random::hashmap_random_keys();
    extraout_RAX_01[0] = 1;
    extraout_RAX_01[1] = 0;
    extraout_RAX_01[2] = 0;
    extraout_RAX_01[3] = 0;
    extraout_RAX_01[4] = 0;
    extraout_RAX_01[5] = 0;
    extraout_RAX_01[6] = 0;
    extraout_RAX_01[7] = 0;
    *(undefined ***)(extraout_RAX_01 + 8) = extraout_RAX_02;
    *(undefined ***)(extraout_RAX_01 + 0x10) = _Src_00;
    local_728 = extraout_RAX_02;
  }
  else {
    local_728 = *(undefined ***)(extraout_RAX_01 + 8);
    _Src_00 = *(undefined ***)(extraout_RAX_01 + 0x10);
  }
  *(longlong *)(extraout_RAX_01 + 8) = (longlong)local_728 + 1;
  local_758 = local_d88;
  ppuStack_750 = ppuStack_d80;
  local_768 = local_d98;
  uStack_760 = uStack_d90;
  local_778 = local_da8;
  puStack_770 = puStack_da0;
  local_748 = &anon_5d8eb308df20e26c7ef69d65e24a3f84_4_llvm_17447356362437668641;
  puStack_740 = (undefined *)0x0;
  local_738 = 0;
  uStack_730 = 0;
  ppuStack_720 = _Src_00;
  hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX_03 & 1) == 0) {
    random::hashmap_random_keys();
    extraout_RAX_03[0] = 1;
    extraout_RAX_03[1] = 0;
    extraout_RAX_03[2] = 0;
    extraout_RAX_03[3] = 0;
    extraout_RAX_03[4] = 0;
    extraout_RAX_03[5] = 0;
    extraout_RAX_03[6] = 0;
    extraout_RAX_03[7] = 0;
    *(longlong *)(extraout_RAX_03 + 8) = extraout_RAX_04;
    *(undefined ***)(extraout_RAX_03 + 0x10) = _Src_00;
    local_b0 = extraout_RAX_04;
  }
  else {
    local_b0 = *(longlong *)(extraout_RAX_03 + 8);
    _Src_00 = *(undefined ***)(extraout_RAX_03 + 0x10);
  }
  *(longlong *)(extraout_RAX_03 + 8) = local_b0 + 1;
  local_e0 = local_728;
  ppuStack_d8 = ppuStack_720;
  local_f0 = local_738;
  uStack_e8 = uStack_730;
  local_100 = local_748;
  puStack_f8 = puStack_740;
  local_110 = local_758;
  ppuStack_108 = ppuStack_750;
  local_120 = local_768;
  uStack_118 = uStack_760;
  local_130 = local_778;
  puStack_128 = puStack_770;
  local_d0 = &anon_5d8eb308df20e26c7ef69d65e24a3f84_4_llvm_17447356362437668641;
  uStack_c8 = 0;
  local_c0 = 0;
  uStack_b8 = 0;
  local_148 = 0;
  local_140 = 8;
  local_138 = 0;
  _Src = (void *)(local_70 + 0x10);
  puVar1 = (uint *)(local_70 + 0xa0);
  local_a8 = _Src_00;
  do {
    local_61 = 1;
    bevy_app::sub_app::SubApp::update();
    if (*(longlong *)(local_70 + 0x98) != 0) {
      lVar7 = *(longlong *)(local_70 + 0x80);
      uVar14 = *(ulonglong *)(local_70 + 0x88);
      uVar8 = uVar14 & 0x5846f8d25a034525;
      lVar10 = 0;
      while( true ) {
        pcVar2 = (char *)(lVar7 + uVar8);
        auVar17[0] = -(*pcVar2 == ',');
        auVar17[1] = -(pcVar2[1] == ',');
        auVar17[2] = -(pcVar2[2] == ',');
        auVar17[3] = -(pcVar2[3] == ',');
        auVar17[4] = -(pcVar2[4] == ',');
        auVar17[5] = -(pcVar2[5] == ',');
        auVar17[6] = -(pcVar2[6] == ',');
        auVar17[7] = -(pcVar2[7] == ',');
        auVar17[8] = -(pcVar2[8] == ',');
        auVar17[9] = -(pcVar2[9] == ',');
        auVar17[10] = -(pcVar2[10] == ',');
        auVar17[0xb] = -(pcVar2[0xb] == ',');
        auVar17[0xc] = -(pcVar2[0xc] == ',');
        auVar17[0xd] = -(pcVar2[0xd] == ',');
        auVar17[0xe] = -(pcVar2[0xe] == ',');
        auVar17[0xf] = -(pcVar2[0xf] == ',');
        uVar12 = (ushort)(SUB161(auVar17 >> 7,0) & 1) | (ushort)(SUB161(auVar17 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar17 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar17 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar17 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar17 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar17 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar17 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar17 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar17 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar17 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar17 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar17 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar17 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar17 >> 0x77,0) & 1) << 0xe | (ushort)(auVar17[0xf] >> 7) << 0xf
        ;
        uVar11 = (uint)uVar12;
        while (uVar12 != 0) {
          uVar3 = 0;
          for (uVar5 = uVar11; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
            uVar3 = uVar3 + 1;
          }
          uVar13 = uVar3 + uVar8 & uVar14;
          if ((*(longlong *)(lVar7 + -0x18 + uVar13 * -0x18) == 0x4d00324fb817a8a1) &&
             (*(longlong *)(lVar7 + -0x10 + uVar13 * -0x18) == 0x5846f8d25a034525)) {
            uVar14 = *(ulonglong *)(lVar7 + uVar13 * -0x18 + -8);
            goto LAB_1400d78c7;
          }
          uVar12 = (ushort)(uVar11 - 1) & (ushort)uVar11;
          uVar11 = CONCAT22((short)(uVar11 - 1 >> 0x10),uVar12);
        }
        auVar15[0] = -(*pcVar2 == -1);
        auVar15[1] = -(pcVar2[1] == -1);
        auVar15[2] = -(pcVar2[2] == -1);
        auVar15[3] = -(pcVar2[3] == -1);
        auVar15[4] = -(pcVar2[4] == -1);
        auVar15[5] = -(pcVar2[5] == -1);
        auVar15[6] = -(pcVar2[6] == -1);
        auVar15[7] = -(pcVar2[7] == -1);
        auVar15[8] = -(pcVar2[8] == -1);
        auVar15[9] = -(pcVar2[9] == -1);
        auVar15[10] = -(pcVar2[10] == -1);
        auVar15[0xb] = -(pcVar2[0xb] == -1);
        auVar15[0xc] = -(pcVar2[0xc] == -1);
        auVar15[0xd] = -(pcVar2[0xd] == -1);
        auVar15[0xe] = -(pcVar2[0xe] == -1);
        auVar15[0xf] = -(pcVar2[0xf] == -1);
        if ((((((((((((((((SUB161(auVar15 >> 7,0) & 1) != 0 || (SUB161(auVar15 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar15 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar15 >> 0x1f,0) & 1) != 0) || (SUB161(auVar15 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar15 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar15 >> 0x37,0) & 1) != 0) || (SUB161(auVar15 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar15 >> 0x47,0) & 1) != 0) || (SUB161(auVar15 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar15 >> 0x57,0) & 1) != 0) || (SUB161(auVar15 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar15 >> 0x67,0) & 1) != 0) || (SUB161(auVar15 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar15 >> 0x77,0) & 1) != 0) || auVar15[0xf] < '\0') break;
        lVar9 = uVar8 + lVar10;
        lVar10 = lVar10 + 0x10;
        uVar8 = lVar9 + 0x10U & uVar14;
      }
    }
    uVar11 = *puVar1;
    if (uVar11 < 0x3ffffffe) {
      LOCK();
      uVar3 = *puVar1;
      if (uVar11 == uVar3) {
        *puVar1 = uVar11 + 1;
      }
      UNLOCK();
      if (uVar11 != uVar3) goto LAB_1400d7932;
    }
    else {
LAB_1400d7932:
      local_61 = 1;
      sync::rwlock::futex::RwLock::read_contended();
    }
    if (*(longlong *)(local_70 + 0x100) == 0) {
LAB_1400d7bf6:
      LOCK();
      uVar11 = *puVar1;
      *puVar1 = *puVar1 - 1;
      UNLOCK();
      if (SBORROW4(0,uVar11 - 1 & 0xbfffffff)) {
        local_61 = 1;
        sync::rwlock::futex::RwLock::wake_writer_or_readers();
      }
LAB_1400d7c0c:
      local_61 = 1;
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    lVar7 = *(longlong *)(local_70 + 0xe8);
    uVar14 = *(ulonglong *)(local_70 + 0xf0);
    uVar8 = uVar14 & 0x5846f8d25a034525;
    lVar10 = 0;
    while( true ) {
      pcVar2 = (char *)(lVar7 + uVar8);
      auVar18[0] = -(*pcVar2 == ',');
      auVar18[1] = -(pcVar2[1] == ',');
      auVar18[2] = -(pcVar2[2] == ',');
      auVar18[3] = -(pcVar2[3] == ',');
      auVar18[4] = -(pcVar2[4] == ',');
      auVar18[5] = -(pcVar2[5] == ',');
      auVar18[6] = -(pcVar2[6] == ',');
      auVar18[7] = -(pcVar2[7] == ',');
      auVar18[8] = -(pcVar2[8] == ',');
      auVar18[9] = -(pcVar2[9] == ',');
      auVar18[10] = -(pcVar2[10] == ',');
      auVar18[0xb] = -(pcVar2[0xb] == ',');
      auVar18[0xc] = -(pcVar2[0xc] == ',');
      auVar18[0xd] = -(pcVar2[0xd] == ',');
      auVar18[0xe] = -(pcVar2[0xe] == ',');
      auVar18[0xf] = -(pcVar2[0xf] == ',');
      uVar12 = (ushort)(SUB161(auVar18 >> 7,0) & 1) | (ushort)(SUB161(auVar18 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar18 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar18 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar18 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar18 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar18 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar18 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar18 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar18 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar18 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar18 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar18 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar18 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar18 >> 0x77,0) & 1) << 0xe | (ushort)(auVar18[0xf] >> 7) << 0xf;
      uVar11 = (uint)uVar12;
      if (uVar12 != 0) break;
LAB_1400d784b:
      auVar16[0] = -(*pcVar2 == -1);
      auVar16[1] = -(pcVar2[1] == -1);
      auVar16[2] = -(pcVar2[2] == -1);
      auVar16[3] = -(pcVar2[3] == -1);
      auVar16[4] = -(pcVar2[4] == -1);
      auVar16[5] = -(pcVar2[5] == -1);
      auVar16[6] = -(pcVar2[6] == -1);
      auVar16[7] = -(pcVar2[7] == -1);
      auVar16[8] = -(pcVar2[8] == -1);
      auVar16[9] = -(pcVar2[9] == -1);
      auVar16[10] = -(pcVar2[10] == -1);
      auVar16[0xb] = -(pcVar2[0xb] == -1);
      auVar16[0xc] = -(pcVar2[0xc] == -1);
      auVar16[0xd] = -(pcVar2[0xd] == -1);
      auVar16[0xe] = -(pcVar2[0xe] == -1);
      auVar16[0xf] = -(pcVar2[0xf] == -1);
      if ((((((((((((((((SUB161(auVar16 >> 7,0) & 1) != 0 || (SUB161(auVar16 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar16 >> 0x17,0) & 1) != 0) || (SUB161(auVar16 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar16 >> 0x27,0) & 1) != 0) || (SUB161(auVar16 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar16 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar16 >> 0x3f,0) & 1) != 0) || (SUB161(auVar16 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar16 >> 0x4f,0) & 1) != 0) || (SUB161(auVar16 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar16 >> 0x5f,0) & 1) != 0) || (SUB161(auVar16 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar16 >> 0x6f,0) & 1) != 0) || (SUB161(auVar16 >> 0x77,0) & 1) != 0) ||
          auVar16[0xf] < '\0') goto LAB_1400d7bf6;
      lVar9 = uVar8 + lVar10;
      lVar10 = lVar10 + 0x10;
      uVar8 = lVar9 + 0x10U & uVar14;
    }
    while( true ) {
      uVar3 = 0;
      for (uVar5 = uVar11; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
        uVar3 = uVar3 + 1;
      }
      lVar9 = (uVar3 + uVar8 & uVar14) * -0x88;
      if ((*(longlong *)(lVar7 + -0x88 + lVar9) == 0x4d00324fb817a8a1) &&
         (*(longlong *)(lVar7 + -0x80 + lVar9) == 0x5846f8d25a034525)) break;
      uVar12 = (ushort)(uVar11 - 1) & (ushort)uVar11;
      uVar11 = CONCAT22((short)(uVar11 - 1 >> 0x10),uVar12);
      if (uVar12 == 0) goto LAB_1400d784b;
    }
    uVar14 = *(ulonglong *)(lVar7 + lVar9 + -8);
    LOCK();
    uVar11 = *puVar1;
    *puVar1 = *puVar1 - 1;
    UNLOCK();
    if (SBORROW4(0,uVar11 - 1 & 0xbfffffff)) {
      local_61 = 1;
      sync::rwlock::futex::RwLock::wake_writer_or_readers();
    }
LAB_1400d78c7:
    puVar6 = __imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E;
    if ((*(ulonglong *)(local_70 + 0x228) <= uVar14) ||
       (uVar14 = *(ulonglong *)(*(longlong *)(local_70 + 0x220) + uVar14 * 8), uVar14 == 0))
    goto LAB_1400d7c0c;
    lVar7 = ~uVar14 * 0x60;
    if (*(longlong *)(*(longlong *)(local_70 + 0x1f0) + 0x20 + lVar7) == 0) goto LAB_1400d7c0c;
    lVar7 = *(longlong *)(*(longlong *)(local_70 + 0x1f0) + lVar7 + 0x10);
    if (*(longlong *)(lVar7 + 0x30) != 0 || *(longlong *)(lVar7 + 0x10) != 0) {
      if (2 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
        local_61 = 1;
        toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
        local_758 = *extraout_RAX_05;
        ppuStack_750 = (undefined **)extraout_RAX_05[1];
        local_748 = (undefined *)0x3;
        puStack_740 = &DAT_14293a620;
        local_738 = 0x14;
        local_728 = &PTR_s_Game_claims_loading_complete__Re_14293a5f8;
        ppuStack_720 = (undefined **)0x1;
        local_718 = 8;
        local_778 = (undefined *)0x0;
        local_710 = (undefined1  [16])0x0;
        puStack_770 = &DAT_14293a620;
        local_768 = 0x14;
        uStack_760 = 0;
        uStack_730 = CONCAT44((int)extraout_RAX_05[2],1);
        local_61 = 1;
        ::log::__private_api::impl$2::log();
      }
      local_61 = 1;
      tiny_glade::render_loop::render();
      if (2 < *(ulonglong *)puVar6) {
        local_61 = 1;
        toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
        local_758 = *extraout_RAX_06;
        ppuStack_750 = (undefined **)extraout_RAX_06[1];
        local_748 = (undefined *)0x3;
        puStack_740 = &DAT_14293a620;
        local_738 = 0x14;
        local_728 = &PTR_DAT_14293a668;
        ppuStack_720 = (undefined **)0x1;
        local_718 = 8;
        local_778 = (undefined *)0x0;
        local_710 = (undefined1  [16])0x0;
        puStack_770 = &DAT_14293a620;
        local_768 = 0x14;
        uStack_760 = 0;
        uStack_730 = CONCAT44((int)extraout_RAX_06[2],1);
        local_61 = 1;
        ::log::__private_api::impl$2::log();
      }
      memcpy(&local_778,_Src,0x628);
      local_61 = 0;
      std::sync::mpmc::Sender::send<bevy_app::sub_app::SubApp>();
      if (!SBORROW8(0,(longlong)local_da8)) {
        local_61 = 0;
        core::ptr::drop_in_place<bevy_app::sub_app::SubApp>();
      }
      local_62 = 0;
      core::ptr::drop_in_place<tiny_glade::run_game::RenderResourceCache>();
      local_63 = 0;
      core::ptr::drop_in_place<tiny_glade::run_game::RenderGraphs>();
      plVar4 = (longlong *)*local_78;
      LOCK();
      *plVar4 = *plVar4 + -1;
      UNLOCK();
      if (*plVar4 == 0) {
        ::alloc::sync::Arc::drop_slow<rhapsody::raw::device::GraphicsDevice,alloc::alloc::Global>();
      }
      core::ptr::drop_in_place<std::sync::mpsc::Sender<bevy_app::sub_app::SubApp>_>();
      return;
    }
  } while( true );
}


