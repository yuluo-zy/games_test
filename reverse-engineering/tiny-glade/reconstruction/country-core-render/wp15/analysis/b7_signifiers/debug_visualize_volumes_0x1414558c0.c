// addr=0x1414558c0 name=debug_visualize_volumes
// signature=void __cdecl debug_visualize_volumes(void)


// WARNING: Control flow encountered bad instruction data
// WARNING: Instruction at (ram,0x000141459730) overlaps instruction at (ram,0x00014145972f)
// 
// WARNING: Type propagation algorithm not settling
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl country_core::systems::ui::debug_visualize_volumes::debug_visualize_volumes(void)

{
  ulonglong *puVar1;
  byte *pbVar2;
  int *piVar3;
  uint ****ppppuVar4;
  byte bVar5;
  char cVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  int iVar10;
  ulonglong uVar11;
  longlong lVar12;
  code *pcVar13;
  ushort uVar14;
  int unaff_0034ab00;
  void *pvVar15;
  char cVar16;
  int iVar17;
  uint uVar18;
  ulonglong uVar19;
  uint *puVar20;
  uint ****extraout_RAX;
  uint ***extraout_RAX_00;
  uint *puVar21;
  undefined4 uVar24;
  uint ****extraout_RAX_01;
  uint ***extraout_RAX_02;
  uint ****extraout_RAX_03;
  uint ***extraout_RAX_04;
  uint ****extraout_RAX_05;
  uint ***extraout_RAX_06;
  uint ****extraout_RAX_07;
  uint ***extraout_RAX_08;
  char *pcVar22;
  longlong *in_RCX;
  uint *puVar25;
  longlong *in_RDX;
  longlong *plVar26;
  uint ****ppppuVar27;
  undefined1 (*unaff_RBX) [16];
  undefined **ppuVar28;
  uint ****ppppuVar29;
  uint *unaff_RSI;
  uint ****ppppuVar31;
  longlong lVar32;
  uint *unaff_RDI;
  uint ***pppuVar33;
  uint uVar34;
  undefined8 *in_R8;
  uint ****ppppuVar35;
  uint ****ppppuVar36;
  uint ****ppppuVar37;
  undefined1 (*unaff_R13) [16];
  undefined8 *puVar38;
  uint *****unaff_R15;
  uint ****ppppuVar39;
  uint ****ppppuVar40;
  bool bVar41;
  bool bVar42;
  float fVar43;
  undefined4 unaff_XMM7_Da;
  undefined4 unaff_XMM7_Db;
  undefined4 unaff_XMM7_Dc;
  undefined4 unaff_XMM7_Dd;
  float unaff_XMM8_Da;
  float unaff_XMM9_Da;
  uint unaff_XMM10_Da;
  uint unaff_XMM10_Db;
  undefined1 auVar44 [16];
  undefined1 auVar45 [16];
  float fVar46;
  float fVar47;
  undefined *puStack_380;
  undefined1 auStack_378 [32];
  undefined8 uStack_358;
  uint *puStack_350;
  undefined *puStack_348;
  undefined8 uStack_338;
  void *pvStack_320;
  uint uStack_318;
  undefined4 uStack_314;
  uint ****ppppuStack_310;
  uint ***pppuStack_308;
  undefined4 uStack_300;
  undefined4 uStack_2fc;
  undefined4 local_2f8;
  undefined4 uStack_2f4;
  undefined8 uStack_2f0;
  undefined4 uStack_2e8;
  void *pvStack_2e0;
  uint ****ppppuStack_2d8;
  uint ****ppppuStack_2d0;
  void *pvStack_2c8;
  uint ****ppppuStack_2c0;
  uint ****ppppuStack_2b8;
  void *pvStack_2b0;
  uint ****ppppuStack_2a8;
  uint ****ppppuStack_2a0;
  uint ****ppppuStack_298;
  void *pvStack_290;
  uint ****ppppuStack_288;
  undefined8 *puStack_280;
  uint ****ppppuStack_278;
  ulonglong uStack_270;
  uint ***pppuStack_268;
  uint uStack_260;
  longlong *plStack_258;
  uint *puStack_250;
  ulonglong *puStack_248;
  undefined8 uStack_240;
  float fStack_238;
  uint ****ppppuStack_230;
  uint ****ppppuStack_228;
  longlong lStack_220;
  uint ****ppppuStack_218;
  uint ***pppuStack_210;
  uint ****ppppuStack_208;
  uint ****ppppuStack_200;
  uint ****ppppuStack_1f8;
  code *pcStack_1f0;
  uint ****ppppuStack_1e8;
  code *pcStack_1e0;
  undefined8 uStack_1d8;
  longlong lStack_198;
  longlong lStack_190;
  uint ****ppppuStack_188;
  undefined4 uStack_180;
  uint ***pppuStack_178;
  uint ***pppuStack_170;
  undefined8 uStack_168;
  uint *****pppppuStack_160;
  code *pcStack_158;
  uint ****ppppuStack_150;
  code *pcStack_148;
  undefined4 uStack_140;
  undefined4 uStack_13c;
  uint ****ppppuStack_138;
  uint *puStack_130;
  uint ****ppppuStack_128;
  uint ****ppppuStack_120;
  uint ****ppppuStack_118;
  void *pvStack_110;
  uint ****ppppuStack_108;
  uint ****ppppuStack_100;
  undefined1 uStack_f2;
  undefined1 uStack_f1;
  undefined8 local_f0;
  undefined7 uVar23;
  undefined1 *puVar30;
  
  local_f0 = 0xfffffffffffffffe;
  lVar32 = *in_RCX;
  if (*(char *)(lVar32 + 0xf2) != '\x01') {
LAB_1414569f1:
    _DAT_142c6203c = 0xfe7f3921;
    UNK_142c62050 = 0xd5;
                    // WARNING: Read-only address (ram,0x000142c6203c) is written
                    // WARNING: Read-only address (ram,0x000142c62050) is written
    return;
  }
  puVar25 = (uint *)&DAT_142c6203c;
  uVar19 = (ulonglong)*(int *)(&DAT_142c6203c + (ulonglong)*(byte *)(lVar32 + 0x102) * 4);
  bVar41 = 0xfffffffebd39dfc3 < uVar19;
  puVar20 = (uint *)(&DAT_142c6203c + uVar19);
  bVar42 = puVar20 == (uint *)0x0;
  cVar16 = (char)puVar20;
  iVar17 = (int)puVar20;
  uVar23 = (undefined7)((ulonglong)puVar20 >> 8);
  ppuVar28 = (undefined **)auStack_378;
  puVar30 = auStack_378;
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Read-only address (ram,0x000142c6203d) is written
                    // WARNING: Could not find normalized switch variable to match jumptable
  switch(*(byte *)(lVar32 + 0x102)) {
  case 0:
    uStack_140 = 0;
    goto code_r0x000141455983;
  case 1:
    uStack_140 = (undefined4)CONCAT71(uVar23,1);
code_r0x000141455983:
    uStack_13c = 0;
code_r0x00014145598d:
    lVar12 = *in_RDX;
    puVar1 = (ulonglong *)(lVar12 + 0x60);
    uVar19 = *(ulonglong *)(lVar12 + 0x60);
    if (uVar19 < 0xfffffffffffffff0 && (char)(((uint)uVar19 & 8) >> 3) == '\0') {
      LOCK();
      uVar11 = *puVar1;
      if (uVar19 == uVar11) {
        *puVar1 = uVar19 + 0x10;
      }
      UNLOCK();
      if (uVar19 != uVar11) goto code_r0x0001414559ae;
    }
    else {
code_r0x0001414559ae:
      puStack_380 = &UNK_1414559be;
      parking_lot::raw_rwlock::RawRwLock::lock_shared_slow();
    }
    puStack_380 = &UNK_1414559d3;
    parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
    uStack_270 = (ulonglong)puVar1 | 1;
    puStack_380 = &UNK_1414559ed;
    puStack_248 = puVar1;
    parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
    unaff_RBX = *(undefined1 (**) [16])(lVar12 + 0x68);
    plStack_258 = *(longlong **)(lVar12 + 0x80);
    auVar45 = *unaff_RBX;
    puStack_250 = (uint *)(ulonglong)
                          ~(uint)(ushort)((ushort)(SUB161(auVar45 >> 7,0) & 1) |
                                          (ushort)(SUB161(auVar45 >> 0xf,0) & 1) << 1 |
                                          (ushort)(SUB161(auVar45 >> 0x17,0) & 1) << 2 |
                                          (ushort)(SUB161(auVar45 >> 0x1f,0) & 1) << 3 |
                                          (ushort)(SUB161(auVar45 >> 0x27,0) & 1) << 4 |
                                          (ushort)(SUB161(auVar45 >> 0x2f,0) & 1) << 5 |
                                          (ushort)(SUB161(auVar45 >> 0x37,0) & 1) << 6 |
                                          (ushort)(SUB161(auVar45 >> 0x3f,0) & 1) << 7 |
                                          (ushort)(SUB161(auVar45 >> 0x47,0) & 1) << 8 |
                                          (ushort)(SUB161(auVar45 >> 0x4f,0) & 1) << 9 |
                                          (ushort)(SUB161(auVar45 >> 0x57,0) & 1) << 10 |
                                          (ushort)(SUB161(auVar45 >> 0x5f,0) & 1) << 0xb |
                                          (ushort)(SUB161(auVar45 >> 0x67,0) & 1) << 0xc |
                                          (ushort)(SUB161(auVar45 >> 0x6f,0) & 1) << 0xd |
                                          (ushort)(SUB161(auVar45 >> 0x77,0) & 1) << 0xe |
                                         (ushort)(byte)(auVar45[0xf] >> 7) << 0xf);
    unaff_R13 = unaff_RBX + 1;
    ppppuStack_138 = (uint ****)*in_R8;
    ppppuStack_278 = (uint ****)(lVar32 + 0xe8);
    unaff_R15 = &ppppuStack_218;
    unaff_XMM7_Da = 0x40400000;
    unaff_XMM7_Db = 0x40400000;
    unaff_XMM7_Dc = 0;
    unaff_XMM7_Dd = 0;
    unaff_XMM8_Da = 3.0;
    unaff_XMM9_Da = 0.0;
    unaff_XMM10_Da = 0x80000000;
    unaff_XMM10_Db = 0x80000000;
code_r0x000141455a60:
    do {
      puVar1 = puStack_248;
      if (plStack_258 == (longlong *)0x0) {
        puStack_380 = &UNK_1414569cf;
        parking_lot_core::parking_lot::deadlock_impl::release_resource();
        puStack_380 = &UNK_1414569db;
        parking_lot_core::parking_lot::deadlock_impl::release_resource();
        LOCK();
        uVar19 = *puVar1;
        *puVar1 = *puVar1 - 0x10;
        UNLOCK();
        if ((uVar19 & 0xfffffffffffffff2) != 0x12) {
          return;
        }
        puStack_380 = &UNK_141456a5b;
        parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow();
        return;
      }
      uVar34 = (uint)puStack_250;
      if ((short)puStack_250 == 0) {
        do {
          auVar45 = *unaff_R13;
          uVar14 = (ushort)(SUB161(auVar45 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar45 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar45 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar45 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar45 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar45 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar45 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar45 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar45 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar45 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar45 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar45 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar45 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar45 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar45 >> 0x77,0) & 1) << 0xe |
                   (ushort)(byte)(auVar45[0xf] >> 7) << 0xf;
          unaff_RBX = unaff_RBX + -0x10;
          unaff_R13 = unaff_R13 + 1;
        } while (uVar14 == 0xffff);
        uVar34 = ~(uint)uVar14;
        uVar18 = uVar34 & -(uint)uVar14 - 2;
      }
      else {
        uVar18 = uVar34 - 1 & uVar34;
      }
      puVar20 = (uint *)(ulonglong)uVar18;
      unaff_0034ab00 = 0;
      for (; (uVar34 & 1) == 0; uVar34 = uVar34 >> 1 | 0x80000000) {
        unaff_0034ab00 = unaff_0034ab00 + 1;
      }
code_r0x000141455ac2:
      plVar26 = (longlong *)((longlong)plStack_258 + -1);
      unaff_RSI = (uint *)((longlong)unaff_RBX - (ulonglong)(uint)(unaff_0034ab00 << 4));
      pppuStack_178 = (uint ***)(unaff_RSI + -4);
      unaff_RDI = *(uint **)(unaff_RSI + -2);
      puVar21 = (uint *)(ulonglong)(byte)unaff_RDI[0x26];
      plStack_258 = plVar26;
      puStack_250 = puVar20;
      puStack_130 = unaff_RDI;
      if ((char)uStack_140 == '\0') goto code_r0x000141455d10;
    } while ((byte)unaff_RDI[0x26] != 2);
    goto code_r0x000141455b12;
  case 2:
  case 4:
    goto LAB_1414569f1;
  case 3:
    uStack_140 = (undefined4)CONCAT71(uVar23,1);
    uStack_13c = (undefined4)CONCAT71(uVar23,1);
    goto code_r0x00014145598d;
  case 5:
    plVar26 = (longlong *)CONCAT62((int6)((ulonglong)in_RDX >> 0x10),CONCAT11(0xc0,(char)in_RDX));
    puVar21 = puVar20;
    goto code_r0x000141455d10;
  case 6:
    *(char *)puVar20 = (char)*puVar20 + cVar16;
    (&UNK_142c61fc7)[uVar19] = (&UNK_142c61fc7)[uVar19] + '<';
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 7:
    plStack_258 = in_RDX;
    goto code_r0x000141455ac2;
  default:
    *(int *)(uVar19 + 0x2858c40bc) = *(int *)(uVar19 + 0x2858c40bc) + -1;
    goto code_r0x000141455a60;
  case 0xb:
    *(char *)puVar20 = (char)*puVar20 + cVar16;
    (&UNK_142c61fc7)[uVar19] = (&UNK_142c61fc7)[uVar19] + '<';
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0xd:
    if (!bVar41 && !bVar42) {
      uStack_358 = CONCAT44(uStack_358._4_4_,0xe);
      puStack_380 = (undefined *)0x141459639;
      core::fmt::Formatter::debug_struct_field4_finish();
      return;
    }
    puStack_380 = (undefined *)0x14145963e;
    pcVar22 = (char *)(**(code **)(((ulonglong)in_RDX & 0xffffffff) - 0x573b7eb8))
                                (&DAT_142c6203c,(ulonglong)puVar20 & 0xffffffff);
    *pcVar22 = *pcVar22 + (char)pcVar22;
    pcVar13 = (code *)swi(3);
    (*pcVar13)();
    return;
  case 0xe:
    goto code_r0x000141459780;
  case 0xf:
                    // WARNING (jumptable): Read-only address (ram,0x000142c6203c) is written
                    // WARNING: Read-only address (ram,0x000142c6203c) is written
    _DAT_142c6203c = 0x8bc73921;
    puVar20 = (uint *)(ulonglong)(iVar17 + 0x18086cb);
    goto code_r0x00014145991b;
  case 0x10:
    *puVar20 = *puVar20 + iVar17;
    pbVar2 = &UNK_142c61fc9 + uVar19;
    bVar5 = *pbVar2;
    *pbVar2 = *pbVar2 + 0x3c;
    puVar20 = (uint *)(ulonglong)(iVar17 + 0x180874b + (uint)(0xc3 < bVar5));
    goto code_r0x00014145988c;
  case 0x11:
    _DAT_142c6203c = 0xfe7f3921;
    UNK_142c62050 = 0xd5;
    return;
  case 0x12:
    goto code_r0x0001414597f9;
  case 0x13:
    goto code_r0x000141459823;
  case 0x14:
    *(char *)puVar20 = (char)*puVar20 + cVar16;
    (&UNK_142c61fc5)[uVar19] = (&UNK_142c61fc5)[uVar19] + '<';
    puVar25 = (uint *)(&UNK_142c61fbf + uVar19);
    *puVar25 = *puVar25 >> 4 | *puVar25 << 0x1c;
    bVar41 = (int)*puVar25 < 0;
    goto code_r0x000141459780;
  case 0x15:
    *(char *)puVar20 = (char)*puVar20 + cVar16;
  case 0x2b:
code_r0x00014145986a:
    ppppuVar29 = (uint ****)&ppppuStack_310;
    goto core_fmt_Formatter_write_str;
  case 0x16:
    *unaff_RDI = *unaff_RSI;
    *puVar20 = *puVar20 + iVar17;
    pbVar2 = &UNK_142c61fc9 + uVar19;
    bVar5 = *pbVar2;
    *pbVar2 = *pbVar2 + 0x3c;
    puVar20 = (uint *)(ulonglong)(iVar17 + 0x1808813 + (uint)(0xc3 < bVar5));
    goto code_r0x000141459839;
  case 0x17:
                    // WARNING (jumptable): Read-only address (ram,0x000142c6203c) is written
                    // WARNING: Read-only address (ram,0x000142c6203c) is written
    _DAT_142c6203c = 0xfe7f390c;
    goto code_r0x0001414598a5;
  case 0x18:
    *(char *)puVar20 = (char)*puVar20 + cVar16;
    goto code_r0x0001414596ed;
  case 0x19:
    if (!bVar42 && -0x142c6203d < (longlong)uVar19) {
      pbVar2 = &UNK_142c61fc9 + uVar19;
      bVar5 = *pbVar2;
      *pbVar2 = *pbVar2 + 0x3c;
      puVar20 = (uint *)(ulonglong)(iVar17 + 0x1808717 + (uint)(0xc3 < bVar5));
    }
code_r0x00014145970b:
    *(char *)puVar20 = (char)*puVar20 + (char)puVar20;
    ppuVar28 = (undefined **)auStack_378;
    break;
  case 0x1a:
    puVar25 = puVar20;
    goto code_r0x000141459823;
  case 0x1b:
    goto code_r0x0001414596ed;
  case 0x1c:
    *puVar20 = *puVar20 + iVar17;
    pbVar2 = &UNK_142c61fc9 + uVar19;
    bVar5 = *pbVar2;
    *pbVar2 = *pbVar2 + 0x3c;
    puVar25 = (uint *)(ulonglong)(iVar17 + 0x1808764 + (uint)(0xc3 < bVar5));
  case 0x2a:
code_r0x0001414598e0:
    puVar20 = puVar25;
    ppppuVar29 = (uint ****)&ppppuStack_310;
    goto core_fmt_Formatter_write_str;
  case 0x1d:
    uStack_338 = 3;
    uStack_358 = 5;
code_r0x0001414596ed:
    puStack_380 = &UNK_1414596fb;
    core::fmt::Formatter::debug_struct_field2_finish();
                    // WARNING: Read-only address (ram,0x000142c6203c) is written
                    // WARNING: Read-only address (ram,0x000142c62050) is written
    return;
  case 0x1e:
    puVar20 = (uint *)((ulonglong)puVar20 & 0xffffffffffffff20);
    *puVar20 = *puVar20 | (uint)puVar20;
    *(char *)puVar20 = (char)*puVar20 + (char)puVar20;
    goto code_r0x0001414597ce;
  case 0x1f:
    if (bVar42 || (longlong)uVar19 < -0x142c6203c) {
      *puVar20 = *puVar20 >> 8 | *puVar20 << 0x18;
      uStack_318 = 0x42c6203c;
      puStack_348 = &UNK_142c61d70;
      puVar25 = &uStack_318;
      goto code_r0x0001414597ce;
    }
                    // WARNING (jumptable): Read-only address (ram,0x000142c6203c) is written
                    // WARNING: Read-only address (ram,0x000142c6203c) is written
    _DAT_142c6203c = 0xfe7f3962;
    puVar20 = (uint *)&DAT_0000000b;
code_r0x000141459839:
    ppppuVar29 = (uint ****)&ppppuStack_310;
    goto core_fmt_Formatter_write_str;
  case 0x20:
    piVar3 = (int *)(((ulonglong)in_RDX & 0xffffffff) - 0x73);
    *piVar3 = *piVar3 + -1;
    puVar20 = (uint *)(ulonglong)((int)in_RDX + 0x180866c + (uint)bVar41);
    ppuVar28 = (undefined **)auStack_378;
    break;
  case 0x21:
    uVar34 = *unaff_RSI;
    pcVar22 = (char *)(ulonglong)uVar34;
    *(char *)unaff_RDI = (char)uVar34;
    *pcVar22 = *pcVar22 + (char)uVar34;
    pcVar22[-0x77] = pcVar22[-0x77] + '<';
    uVar18 = *(uint *)((longlong)in_RDX + -0x7cb76f01);
    *(uint *)((longlong)in_RDX + -0x7cb76f01) =
         uVar18 << 0x1c | (uint)(CONCAT14((uVar34 >> 0x17 & 1) != 0,uVar18) >> 5);
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x22:
code_r0x000141459811:
code_r0x00014145988c:
    ppppuVar29 = (uint ****)&ppppuStack_310;
    goto core_fmt_Formatter_write_str;
  case 0x23:
    piVar3 = (int *)(((ulonglong)in_RDX & 0xffffffff) - 0x73);
    *piVar3 = *piVar3 + -1;
    puVar20 = (uint *)(ulonglong)((int)in_RDX + 0x1808665 + (uint)bVar41);
    goto code_r0x0001414598a5;
  case 0x24:
    break;
  case 0x25:
    if (bVar42) {
      pbVar2 = &UNK_142c61fc9 + uVar19;
      bVar5 = *pbVar2;
      *pbVar2 = *pbVar2 + 0x3c;
      puVar20 = (uint *)(ulonglong)((0xc3 < bVar5) + 0x1808670);
    }
code_r0x00014145972f:
    *(char *)puVar20 = (char)*puVar20 + (char)puVar20;
    ppuVar28 = (undefined **)auStack_378;
    break;
  case 0x26:
    _DAT_142c6203c = 0xfe7f3921;
    UNK_142c62050 = 0xd5;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x27:
code_r0x0001414598f9:
code_r0x00014145991b:
    ppppuVar29 = (uint ****)&ppppuStack_310;
    goto core_fmt_Formatter_write_str;
  case 0x28:
    pcVar13 = (code *)swi(3);
    (*pcVar13)();
    return;
  case 0x29:
    goto code_r0x0001414598c7;
  case 0x2c:
    puVar25 = (uint *)((ulonglong)&uStack_318 & 0xffffffff);
code_r0x0001414597ce:
    uStack_358 = 9;
    puStack_350 = puVar25;
code_r0x0001414597ea:
code_r0x0001414597f3:
    puStack_380 = &UNK_1414597f8;
    core::fmt::Formatter::debug_struct_field1_finish();
code_r0x0001414597f9:
    return;
  case 0x2d:
code_r0x000141459883:
code_r0x0001414598a5:
    ppppuVar29 = (uint ****)&ppppuStack_310;
    goto core_fmt_Formatter_write_str;
  case 0x2e:
    _DAT_142c6203c = 0xfe7f3921;
    UNK_142c62050 = 0xd5;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x2f:
code_r0x000141459659:
                    // WARNING: Could not recover jumptable at 0x000141459667. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(&DAT_142c62070 + *(int *)(&DAT_142c62070 + (longlong)in_RDX * 4)))
              (puVar25,&DAT_142c62070 + *(int *)(&DAT_142c62070 + (longlong)in_RDX * 4));
    return;
  case 0x30:
    UNK_142c62050 = 0xd5;
    _DAT_142c6203c = 0xfe7f3921;
                    // WARNING: Read-only address (ram,0x000142c6203c) is written
                    // WARNING: Read-only address (ram,0x000142c62050) is written
    return;
  }
code_r0x0001414598be:
  puVar25 = puVar20;
  puVar30 = (undefined1 *)ppuVar28;
code_r0x0001414598c7:
  puVar20 = puVar25;
  ppppuVar29 = (uint ****)(puVar30 + 0x68);
core_fmt_Formatter_write_str:
  *(undefined4 **)((longlong)ppppuVar29 + -8) = &local_2f8;
                    // WARNING: Could not recover jumptable at 0x000140d86ba4. Too many branches
                    // WARNING: Treating indirect jump as call
  (**(code **)(*(longlong *)(puVar20 + 0xe) + 0x18))(*(undefined8 *)(puVar20 + 0xc));
  return;
code_r0x000141459823:
  puVar20 = puVar25;
  ppppuVar29 = (uint ****)&ppppuStack_310;
  goto core_fmt_Formatter_write_str;
code_r0x000141459780:
  ppuVar28 = &puStack_380;
  puStack_380 = (undefined *)0xffffffff92d40ae9;
  *(int *)(&UNK_142c61fc9 + uVar19) = *(int *)(&UNK_142c61fc9 + uVar19) + -1;
  puVar20 = (uint *)(ulonglong)(iVar17 + 0x18086a7 + (uint)bVar41);
  goto code_r0x0001414598be;
code_r0x000141455d10:
  puVar25 = (uint *)&UNK_142c62050;
  uVar19 = (ulonglong)*(int *)(&UNK_142c62050 + ((ulonglong)puVar21 & 0xff) * 4);
  bVar41 = 0xfffffffebd39dfaf < uVar19;
  puVar20 = (uint *)(&UNK_142c62050 + uVar19);
  cVar16 = (char)puVar20;
  iVar17 = (int)puVar20;
  uVar24 = (undefined4)((ulonglong)puVar20 >> 0x20);
  uVar23 = (undefined7)((ulonglong)puVar20 >> 8);
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
  switch((ulonglong)puVar21 & 0xff) {
  case 0:
    ppppuStack_230 = (uint ****)CONCAT44(ppppuStack_230._4_4_,unaff_RDI[0x27]);
    ppppuStack_120 = (uint ****)CONCAT44(ppppuStack_120._4_4_,unaff_RDI[0x2a]);
    ppppuStack_128 = *(uint *****)(unaff_RDI + 0x28);
    pppuStack_170 = (uint ***)&pppuStack_178;
    uStack_168 = core::fmt::impl$74::fmt<country_core::systems::ui::resources::ui_state::WidgetId>;
    pppppuStack_160 = &ppppuStack_230;
    pcStack_158 = core::fmt::float::impl$5::fmt;
    ppppuStack_150 = (uint ****)&ppppuStack_128;
    pcStack_148 = glam::f32::vec3::impl$92::fmt;
    ppppuStack_218 = (uint ****)&UNK_142c61890;
    pppuStack_210 = (uint ***)0x3;
    ppppuStack_208 = &pppuStack_170;
    ppppuStack_200 = (uint ****)0x3;
    ppppuStack_1f8 = (uint ****)0x0;
    puStack_380 = &UNK_141455dec;
    alloc::fmt::format::format_inner();
    ppppuVar31 = ppppuStack_288;
    pvVar15 = pvStack_290;
    ppppuStack_100 = ppppuStack_298;
    uVar24 = ppppuStack_230._0_4_;
    ppppuStack_218 = ppppuStack_128;
    pppuStack_210 = (uint ***)CONCAT44(pppuStack_210._4_4_,ppppuStack_120._0_4_);
    puStack_380 = &UNK_141455e38;
    std::alloc::__default_lib_allocator::__rust_alloc();
    pvStack_110 = pvVar15;
    if (extraout_RAX_01 == (uint ****)0x0) {
                    // WARNING: Subroutine does not return
      puStack_380 = &UNK_141456c3b;
      alloc::alloc::handle_alloc_error();
    }
    *extraout_RAX_01 = (uint ***)0x1;
    extraout_RAX_01[1] = (uint ***)0x1;
    *(undefined4 *)(extraout_RAX_01 + 2) = uVar24;
    *(uint *****)((longlong)extraout_RAX_01 + 0x14) = ppppuStack_218;
    *(float *)((longlong)extraout_RAX_01 + 0x1c) = pppuStack_210._0_4_;
    pppuStack_210 = (uint ***)&UNK_142c618c0;
    if (-1 < (longlong)ppppuVar31) {
      if (ppppuVar31 == (uint ****)0x0) {
        pppuVar33 = (uint ***)0x1;
      }
      else {
        puStack_380 = &UNK_141455eb5;
        std::alloc::__default_lib_allocator::__rust_alloc();
        pppuVar33 = extraout_RAX_02;
        if (extraout_RAX_02 == (uint ***)0x0) goto code_r0x000141456c11;
      }
      puStack_380 = &UNK_1414568b8;
      memcpy(pppuVar33,pvVar15,(size_t)ppppuVar31);
      ppppuStack_218 = ppppuVar31;
      ppppuStack_208 = ppppuVar31;
      ppppuStack_1f8 = (uint ****)&UNK_142c618c0;
      unaff_R15 = &ppppuStack_218;
      puStack_380 = &UNK_1414568f8;
      pppuStack_210 = pppuVar33;
      ppppuStack_200 = extraout_RAX_01;
      append_only_vec::AppendOnlyVec::
      push<country_core::systems::ui::signifiers::signifier_create::UiAppendSignifier>();
      if (ppppuStack_100 != (uint ****)0x0) {
        puStack_380 = &UNK_14145691a;
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      goto code_r0x000141455a60;
    }
code_r0x000141456c11:
                    // WARNING: Subroutine does not return
    puStack_380 = &UNK_141456c2a;
    alloc::raw_vec::handle_error();
  case 1:
    ppppuStack_188 = (uint ****)CONCAT44(ppppuStack_188._4_4_,unaff_RDI[0x27]);
    puVar25 = *(uint **)(unaff_RDI + 0x2a);
    puStack_130 = puVar25 + *(longlong *)(unaff_RDI + 0x2c) * 3;
    ppppuVar31 = (uint ****)0x0;
    while (puVar25 != puStack_130) {
      ppppuStack_120 = (uint ****)CONCAT44(ppppuStack_120._4_4_,puVar25[2]);
      ppppuStack_128 = *(uint *****)puVar25;
      ppppuStack_218 = &pppuStack_178;
      pppuStack_210 =
           (uint ***)
           core::fmt::impl$74::fmt<country_core::systems::ui::resources::ui_state::WidgetId>;
      ppppuStack_208 = (uint ****)&ppppuStack_230;
      ppppuStack_200 = (uint ****)core::fmt::num::imp::impl$9::fmt;
      ppppuStack_1f8 = (uint ****)&ppppuStack_188;
      pcStack_1f0 = core::fmt::float::impl$5::fmt;
      ppppuStack_1e8 = (uint ****)&ppppuStack_128;
      pcStack_1e0 = glam::f32::vec3::impl$93::fmt;
      pppuStack_170 = (uint ***)&UNK_142c618f8;
      uStack_168 = (code *)0x4;
      pcStack_158 = (code *)0x4;
      ppppuStack_150 = (uint ****)0x0;
      puStack_380 = &UNK_141456310;
      ppppuStack_230 = ppppuVar31;
      pppppuStack_160 = unaff_R15;
      alloc::fmt::format::format_inner();
      ppppuVar4 = ppppuStack_2a0;
      pvVar15 = pvStack_2b0;
      ppppuStack_100 = ppppuStack_2a8;
      uVar24 = ppppuStack_188._0_4_;
      ppppuStack_218 = ppppuStack_128;
      pppuStack_210 = (uint ***)CONCAT44(pppuStack_210._4_4_,ppppuStack_120._0_4_);
      puStack_380 = &UNK_14145635c;
      std::alloc::__default_lib_allocator::__rust_alloc();
      pvStack_110 = pvVar15;
      if (extraout_RAX_05 == (uint ****)0x0) {
                    // WARNING: Subroutine does not return
        puStack_380 = &UNK_141456ad7;
        alloc::alloc::handle_alloc_error();
      }
      puVar25 = puVar25 + 3;
      ppppuVar31 = (uint ****)((longlong)ppppuVar31 + 1);
      *extraout_RAX_05 = (uint ***)0x1;
      extraout_RAX_05[1] = (uint ***)0x1;
      *(undefined4 *)(extraout_RAX_05 + 2) = uVar24;
      *(uint *****)((longlong)extraout_RAX_05 + 0x14) = ppppuStack_218;
      *(float *)((longlong)extraout_RAX_05 + 0x1c) = pppuStack_210._0_4_;
      pppuStack_210 = (uint ***)&UNK_142c61938;
      if ((longlong)ppppuVar4 < 0) {
code_r0x000141456ab1:
                    // WARNING: Subroutine does not return
        puStack_380 = &UNK_141456ac3;
        alloc::raw_vec::handle_error();
      }
      if (ppppuVar4 == (uint ****)0x0) {
        pppuVar33 = (uint ***)0x1;
        ppppuVar39 = extraout_RAX_05;
      }
      else {
        puStack_380 = &UNK_1414563e1;
        ppppuStack_108 = extraout_RAX_05;
        std::alloc::__default_lib_allocator::__rust_alloc();
        pppuVar33 = extraout_RAX_06;
        ppppuVar39 = ppppuStack_108;
        if (extraout_RAX_06 == (uint ***)0x0) goto code_r0x000141456ab1;
      }
      puStack_380 = &UNK_141456417;
      memcpy(pppuVar33,ppppuStack_100,(size_t)ppppuVar4);
      ppppuStack_218 = ppppuVar4;
      ppppuStack_208 = ppppuVar4;
      ppppuStack_1f8 = (uint ****)&UNK_142c61938;
      unaff_R15 = &ppppuStack_218;
      puStack_380 = &UNK_141456457;
      pppuStack_210 = pppuVar33;
      ppppuStack_200 = ppppuVar39;
      append_only_vec::AppendOnlyVec::
      push<country_core::systems::ui::signifiers::signifier_create::UiAppendSignifier>();
      if (pvStack_110 != (void *)0x0) {
        puStack_380 = &UNK_141456479;
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
    goto code_r0x000141455a60;
  case 2:
    goto code_r0x000141455b12;
  default:
    goto code_r0x000141455a60;
  case 6:
    puStack_380 = &UNK_141456494;
    ::utils::geometry::circle::Circle2d::new();
    puStack_380 = &UNK_1414564a6;
    ::utils::geometry::circle::Circle2d::as_points3();
    puStack_380 = &UNK_1414564bd;
    alloc::vec::spec_from_iter::impl$0::
    from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl_0::as_points2_ptcount::closure_env_0>,utils::geometry::circle::impl_0::as_points3::closure_env_0>_>
              ();
    ppppuStack_128 = (uint ****)0x0;
    ppppuStack_120 = (uint ****)&DAT_00000004;
    ppppuStack_118 = (uint ****)0x0;
    ppppuVar31 = (uint ****)((longlong)ppppuStack_228 + lStack_220 * 0xc);
    bVar41 = true;
    ppppuStack_100 = (uint ****)&DAT_00000004;
    ppppuVar39 = (uint ****)&DAT_00000004;
    ppppuVar40 = ppppuStack_118;
    ppppuStack_108 = ppppuStack_228;
    ppppuVar35 = ppppuStack_228;
    ppppuVar4 = ppppuStack_228;
    lVar32 = lStack_220;
    while (ppppuStack_118 = ppppuVar40, lVar32 != 0) {
      lVar32 = lVar32 + -1;
      ppppuVar36 = ppppuVar4;
      if (ppppuVar4 == ppppuVar31) {
        ppppuVar36 = ppppuVar35;
      }
      ppppuVar4 = (uint ****)((longlong)ppppuVar36 + 0xc);
      ppppuVar27 = ppppuVar35;
      if (ppppuVar4 != ppppuVar31) {
        ppppuVar27 = ppppuVar4;
      }
      ppppuVar37 = ppppuVar36;
      if (bVar41) {
        ppppuVar37 = ppppuVar27;
        ppppuVar39 = ppppuVar36;
        ppppuVar4 = (uint ****)((longlong)ppppuVar27 + 0xc);
      }
      if (ppppuVar39 == (uint ****)0x0) break;
      uStack_300 = *(undefined4 *)(ppppuVar39 + 1);
      pppuStack_308 = *ppppuVar39;
      uStack_2f4 = *(undefined4 *)(ppppuVar37 + 1);
      uStack_2fc = SUB84(*ppppuVar37,0);
      local_2f8 = (undefined4)((ulonglong)*ppppuVar37 >> 0x20);
      uStack_2e8 = 0;
      uStack_2f0 = 0;
      if ((ulonglong)((longlong)ppppuStack_128 - (longlong)ppppuVar40) < 3) {
        uStack_f2 = 1;
        uStack_358 = 0xc;
        puStack_380 = &UNK_141456601;
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        ppppuStack_100 = ppppuStack_120;
        ppppuVar35 = ppppuStack_108;
      }
      *(undefined4 *)((longlong)ppppuStack_100 + (longlong)ppppuStack_118 * 0xc + 0x20) = uStack_2e8
      ;
      puVar38 = (undefined8 *)((longlong)ppppuStack_100 + (longlong)ppppuStack_118 * 0xc + 0x10);
      *puVar38 = CONCAT44(uStack_2f4,local_2f8);
      puVar38[1] = uStack_2f0;
      puVar38 = (undefined8 *)((longlong)ppppuStack_100 + (longlong)ppppuStack_118 * 0xc);
      *puVar38 = pppuStack_308;
      puVar38[1] = CONCAT44(uStack_2fc,uStack_300);
      ppppuVar40 = (uint ****)((longlong)ppppuStack_118 + 3);
      bVar41 = false;
      ppppuVar39 = ppppuVar37;
    }
    uStack_260 = puStack_130[0x29];
    pppuStack_268 = *(uint ****)(puStack_130 + 0x27);
    fStack_238 = (float)puStack_130[0x2c];
    uStack_240 = *(undefined8 *)(puStack_130 + 0x2a);
    ppppuStack_188 = (uint ****)0x3f80000000000000;
    uStack_180 = 0;
    fVar43 = (float)((ulonglong)uStack_240 >> 0x20);
    if (unaff_XMM9_Da <= fStack_238 * unaff_XMM9_Da + (float)uStack_240 * unaff_XMM9_Da + fVar43) {
      uStack_f2 = 1;
      puStack_380 = &UNK_141456705;
      glam::f32::sse2::quat::Quat::from_rotation_arc();
    }
    else {
      pppuStack_170 =
           (uint ***)
           CONCAT44((uint)fVar43 ^ unaff_XMM10_Db,(uint)(float)uStack_240 ^ unaff_XMM10_Da);
      uVar19 = (ulonglong)uStack_168 >> 0x20;
      uStack_168 = (code *)CONCAT44((int)uVar19,(uint)fStack_238 ^ unaff_XMM10_Da);
      uStack_f2 = 1;
      puStack_380 = &UNK_1414566e2;
      glam::f32::sse2::quat::Quat::from_rotation_arc();
    }
    pppuVar33 = pppuStack_210;
    ppppuVar31 = ppppuStack_218;
    ppppuStack_188 = ppppuVar40;
    if (ppppuVar40 == (uint ****)0x0) {
      uStack_f2 = 1;
                    // WARNING: Subroutine does not return
      puStack_380 = &UNK_141456c8a;
      core::panicking::panic_bounds_check();
    }
    pppuStack_170 = (uint ***)&pppuStack_178;
    uStack_168 = core::fmt::impl$74::fmt<country_core::systems::ui::resources::ui_state::WidgetId>;
    pppppuStack_160 = &ppppuStack_188;
    pcStack_158 = core::fmt::num::imp::impl$9::fmt;
    ppppuStack_150 = ppppuStack_100;
    pcStack_148 = glam::f32::vec3::impl$92::fmt;
    ppppuStack_218 = (uint ****)&UNK_142c61b60;
    pppuStack_210 = (uint ***)0x3;
    ppppuStack_208 = &pppuStack_170;
    ppppuStack_200 = (uint ****)0x3;
    ppppuStack_1f8 = (uint ****)0x0;
    uStack_f2 = 1;
    puStack_380 = &UNK_1414567c5;
    alloc::fmt::format::format_inner();
    ppppuVar39 = ppppuStack_128;
    ppppuVar4 = ppppuStack_310;
    pvStack_110 = pvStack_320;
    puVar25 = (uint *)CONCAT44(uStack_314,uStack_318);
    puStack_380 = &UNK_1414567f5;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_07 == (uint ****)0x0) {
      ppppuStack_108 = ppppuVar39;
                    // WARNING: Subroutine does not return
      puStack_380 = &UNK_141456c71;
      alloc::alloc::handle_alloc_error();
    }
    *extraout_RAX_07 = (uint ***)0x1;
    extraout_RAX_07[1] = (uint ***)0x1;
    extraout_RAX_07[2] = (uint ***)ppppuVar31;
    extraout_RAX_07[3] = pppuVar33;
    extraout_RAX_07[4] = (uint ***)ppppuVar39;
    extraout_RAX_07[5] = (uint ***)ppppuStack_100;
    extraout_RAX_07[6] = (uint ***)ppppuVar40;
    extraout_RAX_07[7] = pppuStack_268;
    *(uint *)(extraout_RAX_07 + 8) = uStack_260;
    pppuStack_210 = (uint ***)&UNK_142c61b90;
    if (-1 < (longlong)ppppuVar4) {
      puStack_130 = puVar25;
      if (ppppuVar4 == (uint ****)0x0) {
        pppuVar33 = (uint ***)0x1;
        ppppuVar31 = extraout_RAX_07;
      }
      else {
        puStack_380 = &UNK_14145688d;
        ppppuStack_100 = extraout_RAX_07;
        std::alloc::__default_lib_allocator::__rust_alloc();
        ppppuVar31 = ppppuStack_100;
        pppuVar33 = extraout_RAX_08;
        if (extraout_RAX_08 == (uint ***)0x0) goto code_r0x000141456c47;
      }
      puStack_380 = &UNK_141456933;
      memcpy(pppuVar33,puVar25,(size_t)ppppuVar4);
      ppppuStack_218 = ppppuVar4;
      ppppuStack_208 = ppppuVar4;
      ppppuStack_1f8 = (uint ****)&UNK_142c61b90;
      unaff_R15 = &ppppuStack_218;
      puStack_380 = &UNK_141456973;
      pppuStack_210 = pppuVar33;
      ppppuStack_200 = ppppuVar31;
      append_only_vec::AppendOnlyVec::
      push<country_core::systems::ui::signifiers::signifier_create::UiAppendSignifier>();
      if (pvStack_110 != (void *)0x0) {
        puStack_380 = &UNK_141456991;
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if (ppppuStack_230 != (uint ****)0x0) {
        puStack_380 = &UNK_1414569bb;
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      goto code_r0x000141455a60;
    }
code_r0x000141456c47:
                    // WARNING: Subroutine does not return
    puStack_380 = &UNK_141456c59;
    alloc::raw_vec::handle_error();
  case 8:
    break;
  case 9:
    puVar20 = (uint *)(ulonglong)(iVar17 + 0x1808608 + (uint)bVar41);
  case 0xf:
                    // WARNING: Read-only address (ram,0x000142c62050) is written
    UNK_142c62050 = 0xbe;
    cVar16 = (char)puVar20 + -1;
    puVar20 = (uint *)CONCAT71((int7)((ulonglong)puVar20 >> 8),cVar16);
    *(char *)puVar20 = (char)*puVar20 + cVar16;
    ppppuVar29 = (uint ****)&ppppuStack_310;
    goto core_fmt_Formatter_write_str;
  case 10:
    LOCK();
    cVar6 = *(char *)(uVar19 + 0x1448d0951);
    *(char *)(uVar19 + 0x1448d0951) = cVar16;
    puVar25 = (uint *)CONCAT71(uVar23,cVar6);
    UNLOCK();
    *(char *)puVar25 = (char)*puVar25 + cVar6;
    goto code_r0x0001414598e0;
  case 0xb:
                    // WARNING: Read-only address (ram,0x000142c62050) is written
    UNK_142c62050 = 0x16;
    puVar20 = (uint *)&DAT_00000004;
    ppppuVar29 = (uint ****)&ppppuStack_310;
    goto core_fmt_Formatter_write_str;
  case 0xc:
    LOCK();
    iVar10 = *(int *)(uVar19 + 0x1446c0951);
    *(int *)(uVar19 + 0x1446c0951) = iVar17;
    puVar20 = (uint *)CONCAT44(uVar24,iVar10);
    UNLOCK();
    *(char *)puVar20 = (char)*puVar20 + (char)iVar10;
    goto code_r0x000141459839;
  case 0xd:
    puVar20 = (uint *)(ulonglong)(iVar17 + 0x1808639 + (uint)bVar41);
    goto code_r0x000141459811;
  case 0xe:
    *(char *)puVar20 = (char)*puVar20 + cVar16;
    goto code_r0x000141459839;
  case 0x10:
    puVar20 = (uint *)(ulonglong)(iVar17 + 0x18085ed + (uint)bVar41);
    goto code_r0x00014145986a;
  case 0x11:
    LOCK();
    cVar6 = *(char *)(uVar19 + 0x1443b0951);
    *(char *)(uVar19 + 0x1443b0951) = cVar16;
    puVar20 = (uint *)CONCAT71(uVar23,cVar6);
    UNLOCK();
    goto code_r0x00014145972f;
  case 0x12:
                    // WARNING: Read-only address (ram,0x000142c62050) is written
    UNK_142c62050 = 0xbe;
    puVar20 = (uint *)0x48000000;
    goto code_r0x00014145986a;
  case 0x13:
    LOCK();
    iVar10 = *(int *)(uVar19 + 0x144460951);
    *(int *)(uVar19 + 0x144460951) = iVar17;
    puVar20 = (uint *)CONCAT44(uVar24,iVar10);
    UNLOCK();
    goto code_r0x00014145970b;
  case 0x14:
    *(char *)(uVar19 + 0x143d50951) = cVar16;
    *(char *)puVar20 = (char)*puVar20 + cVar16;
    goto code_r0x0001414598a5;
  case 0x15:
    puVar20 = (uint *)(ulonglong)(iVar17 + 0x180857e + (uint)bVar41);
    goto code_r0x000141459839;
  case 0x16:
    goto code_r0x0001414596ed;
  case 0x17:
                    // WARNING: Read-only address (ram,0x000142c62050) is written
    UNK_142c62050 = 0xbe;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x18:
    goto code_r0x0001414596ed;
  case 0x19:
    goto code_r0x0001414597ea;
  case 0x1a:
    piVar3 = (int *)(((ulonglong)plVar26 & 0xffffffff) - 0x73);
    *piVar3 = *piVar3 + -1;
    puVar20 = (uint *)(ulonglong)((int)plVar26 + 0x180863c + (uint)bVar41);
    goto code_r0x00014145988c;
  case 0x1b:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x1c:
    goto code_r0x0001414597f3;
  case 0x1d:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x1e:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x1f:
    puVar25 = (uint *)(ulonglong)(iVar17 + 0x18085be + (uint)bVar41);
    goto code_r0x0001414598e0;
  case 0x20:
    LOCK();
    cVar6 = *(char *)(uVar19 + 0x144990951);
    *(char *)(uVar19 + 0x144990951) = cVar16;
    puVar20 = (uint *)CONCAT71(uVar23,cVar6);
    UNLOCK();
    *(char *)puVar20 = (char)*puVar20 + cVar6;
    goto code_r0x00014145991b;
  case 0x21:
    LOCK();
    cVar6 = *(char *)(uVar19 + 0x144480951);
    *(char *)(uVar19 + 0x144480951) = cVar16;
    puVar20 = (uint *)CONCAT71(uVar23,cVar6);
    UNLOCK();
    *(char *)puVar20 = (char)*puVar20 + cVar6;
    goto code_r0x00014145991b;
  case 0x22:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x23:
    in_RDX = (longlong *)(ulonglong)puVar20[(longlong)plVar26];
    goto code_r0x000141459659;
  case 0x24:
    *(char *)puVar20 = (char)*puVar20 + cVar16;
    puVar25 = puVar20;
    goto code_r0x0001414598e0;
  case 0x25:
    puVar20 = (uint *)(ulonglong)(iVar17 + 0x18085ac + (uint)bVar41);
    goto code_r0x0001414598f9;
  case 0x26:
    puVar20 = (uint *)(ulonglong)(iVar17 + 0x180856d + (uint)bVar41);
    goto code_r0x000141459883;
  case 0x27:
    goto code_r0x0001414597ea;
  case 0x28:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x29:
    puStack_380 = &UNK_141459700;
    (**(code **)(((ulonglong)plVar26 & 0xffffffff) + 0x68c48348))
              (&UNK_142c62050,(ulonglong)puVar20 & 0xffffffff);
    return;
  case 0x2a:
    *(char *)puVar20 = (char)*puVar20 + cVar16;
    goto code_r0x0001414598a5;
  case 0x2b:
    in_RDX = (longlong *)0xfe7f3cd5;
    goto code_r0x000141459659;
  }
  pcVar13 = (code *)swi(3);
  (*pcVar13)();
  return;
code_r0x000141455b12:
  if (*(longlong *)(unaff_RDI + 0x2c) == 0) {
    puStack_380 = &UNK_141456b5a;
    alloc::vec::spec_from_iter::impl$0::
    from_iter<ref_<str_>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui::signifiers::SignifierFactory>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_volumes::closure_env_6>_>
              ();
    lVar32 = *(longlong *)(unaff_RSI + -2);
    ppppuStack_208 = (uint ****)(lVar32 + 0xb9);
    ppppuStack_1f8 = (uint ****)(lVar32 + 0x28);
    ppppuStack_1e8 = (uint ****)(lVar32 + 0x88);
    pppuStack_210 = (uint ***)alloc::vec::impl$27::fmt<ref_<str_>,alloc::alloc::Global>;
    ppppuStack_200 = (uint ****)resources::widget_params::impl$19::fmt;
    pcStack_1f0 = resources::widget_params::impl$40::fmt;
    pcStack_1e0 = resources::widget_params::impl$38::fmt;
    pppuStack_170 = (uint ***)&UNK_142c61ae8;
    uStack_168 = (code *)0x4;
    ppppuStack_150 = (uint ****)0x0;
    pcStack_158 = (code *)0x4;
                    // WARNING: Subroutine does not return
    puStack_380 = &UNK_141456c0c;
    ppppuStack_218 = (uint ****)&ppppuStack_128;
    pppppuStack_160 = unaff_R15;
    core::panicking::panic_fmt();
  }
  lStack_198 = *(longlong *)(unaff_RDI + 0x2a);
  lStack_190 = lStack_198 + *(longlong *)(unaff_RDI + 0x2c) * 0x24;
  ppppuStack_218 = (uint ****)0x0;
  uStack_1d8 = 0;
  puStack_380 = &UNK_141455b6b;
  alloc::vec::spec_from_iter::impl$0::
  from_iter<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array_<glam::f32::vec3::Vec3,3>,country_core::systems::ui::debug_visualize_volumes::debug_visualize_volumes::closure_env_2>_>
            ();
  ppppuStack_230 = *(uint *****)(unaff_RDI + 0x2c);
  if (ppppuStack_230 == (uint ****)0x0) {
    uStack_f1 = 1;
                    // WARNING: Subroutine does not return
    puStack_380 = &UNK_141456b2f;
    core::panicking::panic_bounds_check();
  }
  ppppuStack_150 = *(uint *****)(unaff_RDI + 0x2a);
  pppuStack_170 = (uint ***)&pppuStack_178;
  uStack_168 = core::fmt::impl$74::fmt<country_core::systems::ui::resources::ui_state::WidgetId>;
  pppppuStack_160 = &ppppuStack_230;
  pcStack_158 = core::fmt::num::imp::impl$9::fmt;
  pcStack_148 = glam::f32::vec3::impl$92::fmt;
  ppppuStack_218 = (uint ****)&UNK_142c61988;
  pppuStack_210 = (uint ***)0x3;
  ppppuStack_208 = &pppuStack_170;
  ppppuStack_200 = (uint ****)0x3;
  ppppuStack_1f8 = (uint ****)0x0;
  uStack_f1 = 1;
  puStack_380 = &UNK_141455c26;
  alloc::fmt::format::format_inner();
  ppppuVar40 = ppppuStack_118;
  ppppuVar39 = ppppuStack_128;
  ppppuVar31 = ppppuStack_2b8;
  pvStack_110 = pvStack_2c8;
  ppppuStack_100 = ppppuStack_2c0;
  ppppuStack_108 = ppppuStack_120;
  puStack_380 = &UNK_141455c79;
  std::alloc::__default_lib_allocator::__rust_alloc();
  ppppuVar4 = ppppuStack_278;
  if (extraout_RAX == (uint ****)0x0) {
    ppppuStack_138 = ppppuVar39;
                    // WARNING: Subroutine does not return
    puStack_380 = &UNK_141456b13;
    alloc::alloc::handle_alloc_error();
  }
  *extraout_RAX = (uint ***)0x1;
  extraout_RAX[1] = (uint ***)0x1;
  extraout_RAX[2] = (uint ***)ppppuVar39;
  extraout_RAX[3] = (uint ***)ppppuStack_108;
  extraout_RAX[4] = (uint ***)ppppuVar40;
  pppuStack_210 = (uint ***)&UNK_142c619b8;
  if (-1 < (longlong)ppppuVar31) {
    if (ppppuVar31 == (uint ****)0x0) {
      pppuVar33 = (uint ***)0x1;
      ppppuVar39 = extraout_RAX;
    }
    else {
      puStack_380 = &UNK_141455cf3;
      ppppuStack_108 = extraout_RAX;
      std::alloc::__default_lib_allocator::__rust_alloc();
      pppuVar33 = extraout_RAX_00;
      ppppuVar39 = ppppuStack_108;
      if (extraout_RAX_00 == (uint ***)0x0) goto code_r0x000141456ae6;
    }
    puStack_380 = &UNK_141455ede;
    memcpy(pppuVar33,ppppuStack_100,(size_t)ppppuVar31);
    ppppuStack_218 = ppppuVar31;
    ppppuStack_208 = ppppuVar31;
    ppppuStack_1f8 = (uint ****)&UNK_142c619b8;
    unaff_R15 = &ppppuStack_218;
    puStack_380 = &UNK_141455f1e;
    pppuStack_210 = pppuVar33;
    ppppuStack_200 = ppppuVar39;
    append_only_vec::AppendOnlyVec::
    push<country_core::systems::ui::signifiers::signifier_create::UiAppendSignifier>();
    puVar25 = puStack_130;
    if (pvStack_110 != (void *)0x0) {
      puStack_380 = &UNK_141455f43;
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if ((char)uStack_13c != '\0') {
      puVar38 = *(undefined8 **)(puVar25 + 0x2a);
      puStack_280 = (undefined8 *)((longlong)puVar38 + *(longlong *)(puVar25 + 0x2c) * 0x24);
      while (unaff_R15 = &ppppuStack_218, puVar38 != puStack_280) {
        fVar43 = *(float *)(puVar38 + 1);
        fVar7 = *(float *)((longlong)puVar38 + 0x14);
        fVar8 = *(float *)(puVar38 + 4);
        auVar44._0_4_ =
             (float)puVar38[3] + (float)*(undefined8 *)((longlong)puVar38 + 0xc) + (float)*puVar38;
        auVar44._4_4_ =
             (float)((ulonglong)puVar38[3] >> 0x20) +
             (float)((ulonglong)*(undefined8 *)((longlong)puVar38 + 0xc) >> 0x20) +
             (float)((ulonglong)*puVar38 >> 0x20);
        auVar44._8_8_ = 0;
        auVar45._4_4_ = unaff_XMM7_Db;
        auVar45._0_4_ = unaff_XMM7_Da;
        auVar45._8_4_ = unaff_XMM7_Dc;
        auVar45._12_4_ = unaff_XMM7_Dd;
        auVar45 = divps(auVar44,auVar45);
        uStack_f1 = 0;
        puStack_380 = &UNK_141455fbf;
        ::utils::geometry::intersect::Triangle::normal();
        fVar9 = *(float *)ppppuVar4;
        fVar47 = SUB84(ppppuStack_218,0);
        uVar19 = (ulonglong)ppppuStack_218 >> 0x20;
        fVar46 = pppuStack_210._0_4_ * fVar9;
        ppppuStack_230 = *(uint *****)(puVar25 + 0x2c);
        if (ppppuStack_230 == (uint ****)0x0) {
          uStack_f1 = 0;
                    // WARNING: Subroutine does not return
          puStack_380 = &UNK_141456a74;
          core::panicking::panic_bounds_check();
        }
        ppppuStack_1f8 = *(uint *****)(puVar25 + 0x2a);
        ppppuStack_218 = &pppuStack_178;
        pppuStack_210 =
             (uint ***)
             core::fmt::impl$74::fmt<country_core::systems::ui::resources::ui_state::WidgetId>;
        ppppuStack_208 = (uint ****)&ppppuStack_230;
        ppppuStack_200 = (uint ****)core::fmt::num::imp::impl$9::fmt;
        pcStack_1f0 = glam::f32::vec3::impl$92::fmt;
        ppppuStack_1e8 = ppppuVar4;
        pcStack_1e0 = core::fmt::float::impl$5::fmt;
        pppuStack_170 = (uint ***)&UNK_142c61a10;
        uStack_168 = (code *)0x4;
        pcStack_158 = (code *)0x4;
        ppppuStack_150 = (uint ****)0x0;
        uStack_f1 = 0;
        puStack_380 = &UNK_1414560a8;
        pppppuStack_160 = unaff_R15;
        alloc::fmt::format::format_inner();
        ppppuVar31 = ppppuStack_2d0;
        pvStack_110 = pvStack_2e0;
        ppppuStack_100 = ppppuStack_2d8;
        puStack_380 = &UNK_1414560d8;
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (extraout_RAX_03 == (uint ****)0x0) {
                    // WARNING: Subroutine does not return
          puStack_380 = &UNK_141456a88;
          alloc::alloc::handle_alloc_error();
        }
        puVar38 = (undefined8 *)((longlong)puVar38 + 0x24);
        *extraout_RAX_03 = (uint ***)0x1;
        extraout_RAX_03[1] = (uint ***)0x1;
        extraout_RAX_03[2] = auVar45._0_8_;
        *(float *)(extraout_RAX_03 + 3) = (fVar43 + fVar7 + fVar8) / unaff_XMM8_Da;
        *(ulonglong *)((longlong)extraout_RAX_03 + 0x1c) =
             CONCAT44(fVar9 * (float)uVar19,fVar9 * fVar47);
        *(float *)((longlong)extraout_RAX_03 + 0x24) = fVar46;
        pppuStack_210 = (uint ***)&UNK_142c61a50;
        if ((longlong)ppppuVar31 < 0) {
code_r0x000141456a90:
                    // WARNING: Subroutine does not return
          puStack_380 = &UNK_141456aa2;
          alloc::raw_vec::handle_error();
        }
        ppppuStack_108 = extraout_RAX_03;
        if (ppppuVar31 == (uint ****)0x0) {
          pppuVar33 = (uint ***)0x1;
        }
        else {
          puStack_380 = &UNK_14145614c;
          std::alloc::__default_lib_allocator::__rust_alloc();
          pppuVar33 = extraout_RAX_04;
          if (extraout_RAX_04 == (uint ***)0x0) goto code_r0x000141456a90;
        }
        puStack_380 = &UNK_141456187;
        memcpy(pppuVar33,ppppuStack_100,(size_t)ppppuVar31);
        ppppuStack_218 = ppppuVar31;
        ppppuStack_208 = ppppuVar31;
        ppppuStack_200 = ppppuStack_108;
        ppppuStack_1f8 = (uint ****)&UNK_142c61a50;
        puStack_380 = &UNK_1414561c7;
        pppuStack_210 = pppuVar33;
        append_only_vec::AppendOnlyVec::
        push<country_core::systems::ui::signifiers::signifier_create::UiAppendSignifier>();
        puVar25 = puStack_130;
        if (pvStack_110 != (void *)0x0) {
          puStack_380 = &UNK_1414561f0;
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
      }
    }
    goto code_r0x000141455a60;
  }
code_r0x000141456ae6:
                    // WARNING: Subroutine does not return
  puStack_380 = &UNK_141456af8;
  alloc::raw_vec::handle_error();
}


