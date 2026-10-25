// ADDR 0x140822530
// SIG void __cdecl op_confirm_audio(void)
// ORIG system::op_confirm_audio


// WARNING: Instruction at (ram,0x000140822cdf) overlaps instruction at (ram,0x000140822cde)
// 
// WARNING: Control flow encountered bad instruction data
// WARNING: Removing unreachable block (ram,0x000140822c99)
// WARNING: Removing unreachable block (ram,0x000140822c9b)
// WARNING: Removing unreachable block (ram,0x000140822c9a)
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl system_decorator::audio::op_confirm_audio::op_confirm_audio(void)

{
  char *pcVar1;
  void *pvVar2;
  longlong lVar3;
  ulonglong uVar4;
  int iVar5;
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
  longlong lVar21;
  longlong lVar22;
  longlong lVar23;
  byte bVar24;
  byte bVar25;
  char extraout_AL;
  uint extraout_EAX;
  uint uVar26;
  ulonglong uVar27;
  undefined1 (*pauVar28) [16];
  undefined1 *extraout_RAX;
  undefined1 (*pauVar29) [16];
  undefined8 extraout_RAX_00;
  longlong *in_RCX;
  undefined1 (*pauVar30) [16];
  undefined1 (*pauVar31) [16];
  undefined8 *in_RDX;
  undefined4 *puVar32;
  undefined1 (*pauVar33) [16];
  uint uVar34;
  undefined1 **ppuVar36;
  longlong *plVar37;
  undefined1 **ppuVar38;
  undefined1 (**ppauVar39) [16];
  undefined1 **ppuVar40;
  longlong *plVar41;
  longlong lVar42;
  undefined1 (*pauVar43) [16];
  undefined1 (*pauVar44) [16];
  undefined8 *in_R8;
  undefined1 (*pauVar45) [16];
  undefined1 (*pauVar46) [16];
  undefined1 *puVar47;
  char cVar48;
  ushort uVar49;
  int iVar50;
  undefined1 *in_R9;
  undefined1 (*pauVar51) [16];
  undefined1 (*pauVar52) [16];
  undefined1 uVar53;
  undefined1 (*pauVar54) [16];
  bool bVar55;
  bool bVar56;
  byte bVar57;
  char cVar58;
  char cVar60;
  char cVar61;
  char cVar62;
  undefined1 auVar59 [16];
  undefined1 auVar63 [16];
  undefined1 auVar64 [16];
  char unaff_XMM6_Ba;
  char unaff_XMM6_Bb;
  char unaff_XMM6_Bc;
  char unaff_XMM6_Bd;
  char unaff_XMM6_Be;
  char unaff_XMM6_Bf;
  char unaff_XMM6_Bg;
  char unaff_XMM6_Bh;
  char unaff_XMM6_Bi;
  char unaff_XMM6_Bj;
  char unaff_XMM6_Bk;
  char unaff_XMM6_Bl;
  char unaff_XMM6_Bm;
  char unaff_XMM6_Bn;
  char unaff_XMM6_Bo;
  char unaff_XMM6_Bp;
  undefined4 unaff_XMM7_Da;
  byte abStack_e29c [57708];
  undefined1 (*pauStack_130) [16];
  undefined1 *apuStack_128 [4];
  undefined1 *local_108;
  uint local_100;
  byte bStack_f4;
  undefined8 *local_f0;
  undefined1 *local_e8;
  longlong *local_e0;
  undefined1 local_d8;
  byte local_d7;
  byte local_d6;
  undefined5 uStack_d5;
  undefined8 local_c8;
  undefined4 local_c0;
  char local_bc;
  longlong *local_b8;
  undefined6 uStack_b0;
  undefined1 (*local_a0) [16];
  undefined1 (*local_98) [16];
  longlong *local_90;
  undefined6 local_88;
  ulonglong *puVar35;
  
  ppuVar36 = apuStack_128;
  ppuVar40 = apuStack_128;
  ppauVar39 = (undefined1 (**) [16])apuStack_128;
  ppuVar38 = apuStack_128;
  puVar32 = (undefined4 *)in_RCX[2];
  *puVar32 = *(undefined4 *)((longlong)in_RCX + 0x1c);
  plVar41 = (longlong *)*in_RCX;
  pvVar2 = (void *)*plVar41;
  lVar3 = plVar41[6];
  plVar41[6] = (longlong)pvVar2;
  lVar42 = plVar41[8];
  uVar4 = plVar41[1];
  *plVar41 = lVar3;
  plVar41[1] = plVar41[7];
  plVar41[7] = uVar4;
  plVar41[8] = plVar41[2];
  lVar21 = plVar41[10];
  lVar22 = plVar41[0xb];
  lVar23 = plVar41[3];
  lVar3 = plVar41[3];
  plVar41[2] = lVar42;
  plVar41[3] = plVar41[9];
  plVar41[9] = lVar23;
  plVar41[10] = plVar41[4];
  plVar41[0xb] = plVar41[5];
  plVar41[4] = lVar21;
  plVar41[5] = lVar22;
  local_f0 = in_R8;
  local_e8 = in_R9;
  if (lVar3 != 0) {
    if (uVar4 != 0) {
      puVar32 = (undefined4 *)CONCAT71((int7)((ulonglong)puVar32 >> 8),0xff);
      pauStack_130 = (undefined1 (*) [16])0x1408225c3;
      memset(pvVar2,(int)puVar32,uVar4 + 0x11);
    }
    plVar41[9] = 0;
    uVar27 = (uVar4 + 1 & 0xfffffffffffffff8) - (uVar4 + 1 >> 3);
    if (uVar4 < 8) {
      uVar27 = uVar4;
    }
    plVar41[8] = uVar27;
  }
  pvVar2 = (void *)plVar41[0xc];
  lVar3 = plVar41[0x12];
  plVar41[0x12] = (longlong)pvVar2;
  lVar42 = plVar41[0x14];
  uVar4 = plVar41[0xd];
  plVar41[0xc] = lVar3;
  plVar41[0xd] = plVar41[0x13];
  plVar41[0x13] = uVar4;
  plVar41[0x14] = plVar41[0xe];
  lVar21 = plVar41[0x16];
  lVar22 = plVar41[0x17];
  lVar23 = plVar41[0xf];
  lVar3 = plVar41[0xf];
  plVar41[0xe] = lVar42;
  plVar41[0xf] = plVar41[0x15];
  plVar41[0x15] = lVar23;
  plVar41[0x16] = plVar41[0x10];
  plVar41[0x17] = plVar41[0x11];
  plVar41[0x10] = lVar21;
  plVar41[0x11] = lVar22;
  if (lVar3 != 0) {
    if (uVar4 != 0) {
      pauStack_130 = (undefined1 (*) [16])0x14082265e;
      memset(pvVar2,(int)CONCAT71((int7)((ulonglong)puVar32 >> 8),0xff),uVar4 + 0x11);
    }
    plVar41[0x15] = 0;
    uVar27 = (uVar4 + 1 & 0xfffffffffffffff8) - (uVar4 + 1 >> 3);
    if (uVar4 < 8) {
      uVar27 = uVar4;
    }
    plVar41[0x14] = uVar27;
  }
  local_e0 = plVar41 + 6;
  puVar35 = (ulonglong *)*in_RDX;
  lVar3 = in_RDX[1];
  uVar4 = *puVar35;
  pauVar51 = (undefined1 (*) [16])0x0;
  pauVar29 = (undefined1 (*) [16])(uVar4 - *(ulonglong *)(lVar3 + 0x18));
  if (uVar4 < *(ulonglong *)(lVar3 + 0x18)) {
    pauVar29 = pauVar51;
  }
  local_90 = plVar41 + 4;
  pauVar30 = (undefined1 (*) [16])(uVar4 - *(ulonglong *)(lVar3 + 0x38));
  if (uVar4 < *(ulonglong *)(lVar3 + 0x38)) {
    pauVar30 = pauVar51;
  }
  pauVar44 = (undefined1 (*) [16])((longlong)pauVar29 * 0x90 + *(longlong *)(lVar3 + 8));
  pauVar46 = (undefined1 (*) [16])
             ((longlong)*(undefined1 (**) [16])(lVar3 + 0x10) - (longlong)pauVar29);
  if (*(undefined1 (**) [16])(lVar3 + 0x10) < pauVar29) {
    pauVar44 = (undefined1 (*) [16])&DAT_00000008;
    pauVar46 = pauVar51;
  }
  pauVar54 = (undefined1 (*) [16])((longlong)pauVar30 * 0x90 + *(longlong *)(lVar3 + 0x28));
  pauVar29 = (undefined1 (*) [16])
             ((longlong)*(undefined1 (**) [16])(lVar3 + 0x30) - (longlong)pauVar30);
  if (*(undefined1 (**) [16])(lVar3 + 0x30) < pauVar30) {
    pauVar54 = (undefined1 (*) [16])&DAT_00000008;
    pauVar29 = pauVar51;
  }
  *puVar35 = (*(longlong *)(lVar3 + 0x40) - (longlong)pauVar29) - (longlong)pauVar46;
  pauVar30 = pauVar44 + (longlong)pauVar46 * 9;
  pauVar51 = pauVar54 + (longlong)pauVar29 * 9;
  local_b8 = plVar41;
  while( true ) {
    pauVar52 = (undefined1 (*) [16])0x0;
    do {
      do {
        if ((pauVar44 == (undefined1 (*) [16])0x0) || (pauVar44 == pauVar30)) {
          if ((pauVar54 == (undefined1 (*) [16])0x0) || (pauVar54 == pauVar51)) {
            pauVar51 = (undefined1 (*) [16])plVar41[9];
            if (pauVar51 == (undefined1 (*) [16])0x0) {
              return;
            }
            pauVar30 = (undefined1 (*) [16])*local_e0;
            pauVar45 = pauVar30 + 1;
            auVar59 = *pauVar30;
            uVar26 = ~(uint)(ushort)((ushort)(SUB161(auVar59 >> 7,0) & 1) |
                                     (ushort)(SUB161(auVar59 >> 0xf,0) & 1) << 1 |
                                     (ushort)(SUB161(auVar59 >> 0x17,0) & 1) << 2 |
                                     (ushort)(SUB161(auVar59 >> 0x1f,0) & 1) << 3 |
                                     (ushort)(SUB161(auVar59 >> 0x27,0) & 1) << 4 |
                                     (ushort)(SUB161(auVar59 >> 0x2f,0) & 1) << 5 |
                                     (ushort)(SUB161(auVar59 >> 0x37,0) & 1) << 6 |
                                     (ushort)(SUB161(auVar59 >> 0x3f,0) & 1) << 7 |
                                     (ushort)(SUB161(auVar59 >> 0x47,0) & 1) << 8 |
                                     (ushort)(SUB161(auVar59 >> 0x4f,0) & 1) << 9 |
                                     (ushort)(SUB161(auVar59 >> 0x57,0) & 1) << 10 |
                                     (ushort)(SUB161(auVar59 >> 0x5f,0) & 1) << 0xb |
                                     (ushort)(SUB161(auVar59 >> 0x67,0) & 1) << 0xc |
                                     (ushort)(SUB161(auVar59 >> 0x6f,0) & 1) << 0xd |
                                     (ushort)(SUB161(auVar59 >> 0x77,0) & 1) << 0xe |
                                    (ushort)(byte)(auVar59[0xf] >> 7) << 0xf);
            local_e0 = (longlong *)
                       CONCAT44(local_e0._4_4_,*(undefined4 *)((longlong)local_f0 + 0x1c));
            pauVar44 = (undefined1 (*) [16])*local_f0;
            local_e8 = (undefined1 *)local_f0[2];
            local_98 = pauVar44 + 2;
            iVar5 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                                     CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba)))
                           == CONCAT13(unaff_XMM6_Bd,
                                       CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))
                                      ));
            unaff_XMM6_Ba = (char)iVar5;
            unaff_XMM6_Bb = (char)((uint)iVar5 >> 8);
            unaff_XMM6_Bc = (char)((uint)iVar5 >> 0x10);
            unaff_XMM6_Bd = (char)((uint)iVar5 >> 0x18);
            iVar5 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                                     CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be)))
                           == CONCAT13(unaff_XMM6_Bh,
                                       CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))
                                      ));
            unaff_XMM6_Be = (char)iVar5;
            unaff_XMM6_Bf = (char)((uint)iVar5 >> 8);
            unaff_XMM6_Bg = (char)((uint)iVar5 >> 0x10);
            unaff_XMM6_Bh = (char)((uint)iVar5 >> 0x18);
            iVar5 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                                     CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi)))
                           == CONCAT13(unaff_XMM6_Bl,
                                       CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))
                                      ));
            unaff_XMM6_Bi = (char)iVar5;
            unaff_XMM6_Bj = (char)((uint)iVar5 >> 8);
            unaff_XMM6_Bk = (char)((uint)iVar5 >> 0x10);
            unaff_XMM6_Bl = (char)((uint)iVar5 >> 0x18);
            iVar5 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                                     CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm)))
                           == CONCAT13(unaff_XMM6_Bp,
                                       CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))
                                      ));
            unaff_XMM6_Bm = (char)iVar5;
            unaff_XMM6_Bn = (char)((uint)iVar5 >> 8);
            unaff_XMM6_Bo = (char)((uint)iVar5 >> 0x10);
            unaff_XMM6_Bp = (char)((uint)iVar5 >> 0x18);
            local_a0 = pauVar44;
            goto LAB_140822aa7;
          }
          *puVar35 = *puVar35 + 1;
          cVar48 = pauVar54[8][0];
          pauVar45 = pauVar54 + 9;
          pauVar33 = (undefined1 (*) [16])0x0;
          pauVar43 = pauVar54;
        }
        else {
          *puVar35 = *puVar35 + 1;
          cVar48 = pauVar44[8][0];
          pauVar45 = pauVar54;
          pauVar33 = pauVar44 + 9;
          pauVar43 = pauVar44;
        }
        pauVar44 = pauVar33;
        pauVar54 = pauVar45;
      } while (cVar48 != '\0');
      bVar57 = pauVar43[8][1];
      pauVar33 = (undefined1 (*) [16])(ulonglong)bVar57;
    } while (bVar57 == 0x3f);
    pauVar45 = (undefined1 (*) [16])(*(longlong *)*pauVar43 + -2);
    if (1 < *(longlong *)*pauVar43 - 3U) {
      pauVar45 = pauVar52;
    }
    if (pauVar45 != (undefined1 (*) [16])0x0) break;
    if (pauVar43[6][0] != '\0') {
      local_d7 = pauVar43[6][1];
      local_d6 = bVar57;
      uVar53 = 2;
      goto LAB_14082290b;
    }
    if (*(int *)pauVar43[1] != 3) {
      pauVar46 = (undefined1 (*) [16])(ulonglong)(byte)pauVar43[4][8];
      local_100 = *(uint *)(pauVar43[6] + 8);
      local_108 = local_e8;
      pauStack_130 = (undefined1 (*) [16])0x1408228ea;
      place_sound();
      uVar53 = local_d8;
      goto LAB_14082290b;
    }
    if (pauVar43[6][2] != '\0') {
      uVar53 = 8;
      if (bVar57 == 0x3d) {
        uVar53 = 0x19;
      }
      goto LAB_14082290b;
    }
    bVar57 = pauVar43[6][5];
    if (bVar57 != 0x40) {
      uVar53 = 8;
      if ((byte)(bVar57 - 0x38) == 6 || 7 < (byte)(bVar57 - 0x38)) {
        local_d7 = bVar57;
        uVar53 = 0x23;
      }
LAB_14082290b:
      pauVar29 = (undefined1 (*) [16])(ulonglong)*(uint *)(pauVar43[6] + 8);
      local_88 = CONCAT51(uStack_d5,local_d6);
      plVar41 = local_b8;
      bVar57 = local_d7;
LAB_140822935:
      uStack_b0 = local_88;
      if (pauVar43[7][0xc] != '\x02') {
        local_c0 = *(undefined4 *)(pauVar43[7] + 8);
        local_c8 = *(undefined8 *)pauVar43[7];
        local_d6 = (byte)local_88;
        uStack_d5 = (undefined5)((uint6)local_88 >> 8);
        local_108 = &local_d8;
        pauStack_130 = (undefined1 (*) [16])0x1408229ac;
        local_d8 = uVar53;
        local_d7 = bVar57;
        local_bc = pauVar43[7][0xc];
        hashbrown::map::HashMap::
        insert<system_decorator::audio::op_confirm_audio::AudioEventKey,country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd,std::hash::random::RandomState,alloc::alloc::Global>
                  ();
        pauVar46 = pauVar45;
      }
    }
  }
  if (pauVar45 == (undefined1 (*) [16])0x1) {
    if (pauVar43[4][4] == '\0') {
      pauStack_130 = (undefined1 (*) [16])0x1408229bb;
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      bVar57 = (byte)pauVar33;
      uVar53 = 0;
      pauVar29 = (undefined1 (*) [16])(ulonglong)extraout_EAX;
    }
    else {
      uVar26 = *(uint *)pauVar43[4];
      pauVar46 = (undefined1 (*) [16])(ulonglong)(byte)pauVar43[4][5];
      local_108 = local_e8;
      pauStack_130 = (undefined1 (*) [16])0x140822843;
      local_100 = uVar26;
      place_sound();
      pauVar29 = (undefined1 (*) [16])(ulonglong)uVar26;
      local_88 = CONCAT51(uStack_d5,local_d6);
      plVar41 = local_b8;
      bVar57 = local_d7;
      uVar53 = local_d8;
    }
    goto LAB_140822935;
  }
  pauVar29 = (undefined1 (*) [16])(ulonglong)*(uint *)pauVar43[6];
  cVar48 = (char)*(uint *)pauVar43[6];
  bVar24 = bVar57 - 0x38;
  if (7 < bVar24) {
    bVar24 = 6;
  }
  uVar53 = 4;
  pauVar31 = (undefined1 (*) [16])&DAT_142a90ff0;
  pauVar28 = (undefined1 (*) [16])
             (&DAT_142a90ff0 + *(int *)(&DAT_142a90ff0 + (ulonglong)bVar24 * 4));
  bVar55 = pauVar28 == (undefined1 (*) [16])0x0;
  bVar25 = (byte)pauVar28;
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
  switch(bVar24) {
  case 0:
  case 1:
  case 2:
    break;
  case 3:
    uVar53 = 0x17;
    break;
  case 4:
    uVar53 = 0x10;
    break;
  case 5:
    uVar53 = 0x1e;
    break;
  case 6:
    uVar53 = 0x20;
    bStack_f4 = bVar57;
    goto LAB_140822935;
  default:
    ppauVar39 = (undefined1 (**) [16])apuStack_128;
    goto LAB_140822be0;
  case 8:
    goto code_r0x000140822c84;
  case 9:
    goto code_r0x000140822c6d;
  case 0xd:
  case 0x14:
  case 0x19:
  case 0x1f:
  case 0x24:
  case 0x29:
    (*pauVar28)[0] = (*pauVar28)[0] + bVar25;
    (*pauVar28)[0] = (*pauVar28)[0] + bVar25;
    (*pauVar44)[0] = (*pauVar44)[0] + -0x10;
    bVar55 = (*pauVar44)[0] == '\0';
    goto code_r0x000140822be3;
  case 0xe:
  case 0x13:
  case 0x1d:
  case 0x2b:
    ppauVar39 = (undefined1 (**) [16])apuStack_128;
    goto LAB_140822be0;
  case 0x12:
  case 0x18:
  case 0x21:
  case 0x27:
  case 0x2a:
    ppauVar39 = &pauStack_130;
    pauStack_130 = pauVar43;
code_r0x000140822be3:
    if (!bVar55) {
      plVar37 = (longlong *)((longlong)ppauVar39 + -8);
      *(longlong **)((longlong)ppauVar39 + -8) = plVar41;
      cVar48 = '\x12';
      do {
        plVar41 = plVar41 + -1;
        plVar37 = plVar37 + -1;
        *plVar37 = *plVar41;
        cVar48 = cVar48 + -1;
      } while ('\0' < cVar48);
      *(undefined1 **)((longlong)ppauVar39 + -0xa0) = (undefined1 *)((longlong)ppauVar39 + -8);
      unaff_XMM7_Da = *(undefined4 *)(pauVar43[-2] + 4);
      ppuVar38 = (undefined1 **)((longlong)ppauVar39 + -0xe19f);
      goto code_r0x000140822c0f;
    }
LAB_140822be0:
    do {
      cVar48 = (char)pauVar29;
      bVar57 = pauVar43[-2][0];
      pauVar33 = (undefined1 (*) [16])(ulonglong)bVar57;
      pauVar31 = (undefined1 (*) [16])&DAT_142a9100c;
      bVar55 = 0xfffffffebd56eff3 <
               (ulonglong)(longlong)*(int *)(&DAT_142a9100c + (longlong)pauVar33 * 4);
      pauVar28 = (undefined1 (*) [16])
                 (&DAT_142a9100c + *(int *)(&DAT_142a9100c + (longlong)pauVar33 * 4));
      bVar56 = pauVar28 == (undefined1 (*) [16])0x0;
      ppuVar38 = (undefined1 **)ppauVar39;
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
      switch(bVar57) {
      case 1:
        pauVar46 = (undefined1 (*) [16])(ulonglong)(byte)pauVar43[-2][1];
        pauVar29 = (undefined1 (*) [16])(ulonglong)(byte)pauVar43[-2][2];
        pauVar52 = *(undefined1 (**) [16])(pauVar43[-2] + 8);
        break;
      case 2:
        pauVar46 = (undefined1 (*) [16])(ulonglong)(byte)pauVar43[-2][1];
        pauVar29 = (undefined1 (*) [16])(ulonglong)(byte)pauVar43[-2][2];
        break;
      case 6:
      case 0xd:
      case 0x12:
      case 0x18:
      case 0x1d:
      case 0x22:
        unaff_XMM7_Da = *(undefined4 *)(pauVar43[-2] + 4);
        break;
      case 7:
      case 0xc:
      case 0x16:
      case 0x24:
        unaff_XMM7_Da = *(undefined4 *)(pauVar43[-2] + 4);
        pauVar46 = (undefined1 (*) [16])(ulonglong)(byte)pauVar43[-2][1];
        break;
      case 0xb:
      case 0x11:
      case 0x1a:
      case 0x20:
      case 0x23:
        pauVar46 = (undefined1 (*) [16])(ulonglong)(byte)pauVar43[-2][1];
        break;
      case 0x1c:
        pauVar46 = (undefined1 (*) [16])(ulonglong)(byte)pauVar43[-2][1];
        goto code_r0x000140822cb5;
      case 0x26:
      case 0x27:
      case 0x28:
        pauVar44 = (undefined1 (*) [16])(ulonglong)*(uint *)((longlong)ppauVar39 + 0x38);
        apuStack_128[0] = (undefined1 *)ppauVar39;
        goto code_r0x000140822c84;
      case 0x29:
code_r0x000140822cb5:
        unaff_XMM7_Da = *(undefined4 *)(pauVar43[-2] + 4);
        break;
      case 0x2a:
code_r0x000140822d26:
        pauVar29 = (undefined1 (*) [16])0x1;
        if ((char)pauVar28 != '\x01') {
          pauVar29 = pauVar33;
        }
        (*pauVar31)[2] = cVar48 == '\x02';
        *(undefined1 (**) [16])(*pauVar31 + 8) = pauVar29;
        (*pauVar31)[1] = (char)pauVar46;
        (*pauVar31)[0] = 1;
        return;
      case 0x2b:
        ppauVar39 = (undefined1 (**) [16])((longlong)ppauVar39 + -0x38);
        pauVar28 = (undefined1 (*) [16])(ulonglong)(bVar57 - 0x38);
        bVar55 = (byte)(bVar57 - 0x38) < 8;
      case 0x2e:
      case 0x30:
        pauVar52 = (undefined1 (*) [16])((ulonglong)pauVar28 & 0xff);
        pauVar28 = (undefined1 (*) [16])&DAT_00000006;
        goto code_r0x000140822d04;
      case 0x2c:
        goto code_r0x000140822d1e;
      case 0x2d:
        pauVar31 = (undefined1 (*) [16])((longlong)&DAT_142a91008 + 3);
        if (-1 < (longlong)pauVar28) goto code_r0x000140822d55;
        goto code_r0x000140822d99;
      case 0x2f:
        return;
      }
code_r0x000140822c0f:
      *(undefined4 *)((longlong)ppuVar38 + 0x58) = *(undefined4 *)(pauVar43[-1] + 8);
      *(undefined8 *)((longlong)ppuVar38 + 0x50) = *(undefined8 *)pauVar43[-1];
      bVar24 = pauVar43[-1][0xc];
      plVar41 = (longlong *)(ulonglong)bVar24;
      **(undefined4 **)((longlong)ppuVar38 + 0x40) = *(undefined4 *)((longlong)ppuVar38 + 0x48);
      pauVar43 = *(undefined1 (**) [16])pauVar44[3];
      pauVar54 = *(undefined1 (**) [16])pauVar44[4];
      ppuVar36 = ppuVar38;
      if (pauVar43 == *(undefined1 (**) [16])pauVar44[2]) {
        *(undefined1 (**) [16])((longlong)ppuVar38 + 0x38) = pauVar51;
        *(int *)((longlong)ppuVar38 + 0x34) = (int)puVar35;
        *(byte *)((longlong)ppuVar38 + 0x33) = bVar57;
        pauVar51 = (undefined1 (*) [16])((ulonglong)pauVar46 & 0xffffffff);
        puVar35 = (ulonglong *)((ulonglong)pauVar29 & 0xffffffff);
        *(undefined **)((longlong)ppuVar38 + -8) = &UNK_140822c6a;
        alloc::raw_vec::RawVec::
        grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                  ();
code_r0x000140822c6d:
        pauVar29 = (undefined1 (*) [16])((ulonglong)puVar35 & 0xffffffff);
        pauVar46 = (undefined1 (*) [16])((ulonglong)pauVar51 & 0xffffffff);
        bVar57 = *(byte *)((longlong)ppuVar38 + 0x33);
        pauVar44 = *(undefined1 (**) [16])((longlong)ppuVar38 + 0x88);
        puVar35 = (ulonglong *)(ulonglong)*(uint *)((longlong)ppuVar38 + 0x34);
        apuStack_128[0] = (undefined1 *)ppuVar38;
code_r0x000140822c84:
        bVar24 = (byte)plVar41;
        pauVar51 = *(undefined1 (**) [16])(apuStack_128[0] + 0x38);
        ppuVar36 = (undefined1 **)apuStack_128[0];
      }
      lVar3 = *(longlong *)(pauVar44[2] + 8);
      *(undefined1 (**) [16])(lVar3 + (longlong)pauVar43 * 0x28) = pauVar54;
      *(byte *)(lVar3 + 8 + (longlong)pauVar43 * 0x28) = bVar57;
      *(char *)(lVar3 + 9 + (longlong)pauVar43 * 0x28) = (char)pauVar46;
      *(char *)(lVar3 + 10 + (longlong)pauVar43 * 0x28) = (char)pauVar29;
      *(undefined4 *)(lVar3 + 0xc + (longlong)pauVar43 * 0x28) = unaff_XMM7_Da;
      *(undefined1 (**) [16])(lVar3 + 0x10 + (longlong)pauVar43 * 0x28) = pauVar52;
      *(undefined8 *)(lVar3 + 0x18 + (longlong)pauVar43 * 0x28) =
           *(undefined8 *)((longlong)ppuVar36 + 0x50);
      *(undefined4 *)(lVar3 + 0x20 + (longlong)pauVar43 * 0x28) =
           *(undefined4 *)((longlong)ppuVar36 + 0x58);
      *(byte *)(lVar3 + 0x24 + (longlong)pauVar43 * 0x28) = bVar24;
      *(undefined1 **)pauVar44[3] = *pauVar43 + 1;
      *(longlong *)pauVar44[4] = *(longlong *)pauVar44[4] + 1;
LAB_140822a97:
      uVar26 = (uint)puVar35;
      pauVar51 = (undefined1 (*) [16])(pauVar51[-1] + 0xf);
      plVar41 = *(longlong **)((longlong)ppuVar36 + 0x70);
      if (pauVar51 == (undefined1 (*) [16])0x0) {
        return;
      }
LAB_140822aa7:
      if ((short)uVar26 == 0) {
        do {
          auVar59 = *pauVar45;
          uVar49 = (ushort)(SUB161(auVar59 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar59 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar59 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar59 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar59 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar59 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar59 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar59 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar59 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar59 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar59 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar59 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar59 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar59 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar59 >> 0x77,0) & 1) << 0xe |
                   (ushort)(byte)(auVar59[0xf] >> 7) << 0xf;
          pauVar30 = pauVar30 + -0x30;
          pauVar45 = pauVar45 + 1;
        } while (uVar49 == 0xffff);
        uVar26 = ~(uint)uVar49;
        uVar34 = uVar26 & -(uint)uVar49 - 2;
      }
      else {
        uVar34 = uVar26 - 1 & uVar26;
      }
      puVar35 = (ulonglong *)(ulonglong)uVar34;
      uVar34 = 0;
      for (; (uVar26 & 1) == 0; uVar26 = uVar26 >> 1 | 0x80000000) {
        uVar34 = uVar34 + 1;
      }
      pauVar43 = pauVar30 + (ulonglong)uVar34 * -3;
      ppauVar39 = (undefined1 (**) [16])ppuVar36;
      if (plVar41[3] != 0) {
        pauVar54 = pauVar30 + (ulonglong)uVar34 * -3 + -3;
        *(undefined8 *)((longlong)ppuVar36 + -8) = 0x140822b1f;
        core::hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<system_decorator::audio::op_confirm_audio::AudioEventKey>_>
                  ();
        lVar3 = *plVar41;
        uVar4 = plVar41[1];
        bVar57 = (byte)((ulonglong)extraout_RAX >> 0x39);
        auVar59 = ZEXT216(CONCAT11(bVar57,bVar57));
        auVar59 = pshuflw(auVar59,auVar59,0);
        plVar41 = (longlong *)0x0;
        puVar47 = extraout_RAX;
        while( true ) {
          pauVar46 = (undefined1 (*) [16])(uVar4 & (ulonglong)puVar47);
          pcVar1 = *pauVar46 + lVar3;
          cVar48 = *pcVar1;
          cVar6 = pcVar1[1];
          cVar7 = pcVar1[2];
          cVar8 = pcVar1[3];
          cVar9 = pcVar1[4];
          cVar10 = pcVar1[5];
          cVar11 = pcVar1[6];
          cVar12 = pcVar1[7];
          cVar13 = pcVar1[8];
          cVar14 = pcVar1[9];
          cVar15 = pcVar1[10];
          cVar16 = pcVar1[0xb];
          cVar17 = pcVar1[0xc];
          cVar18 = pcVar1[0xd];
          cVar19 = pcVar1[0xe];
          cVar20 = pcVar1[0xf];
          cVar58 = auVar59[0];
          auVar64[0] = -(cVar48 == cVar58);
          cVar60 = auVar59[1];
          auVar64[1] = -(cVar6 == cVar60);
          cVar61 = auVar59[2];
          auVar64[2] = -(cVar7 == cVar61);
          cVar62 = auVar59[3];
          auVar64[3] = -(cVar8 == cVar62);
          auVar64[4] = -(cVar9 == cVar58);
          auVar64[5] = -(cVar10 == cVar60);
          auVar64[6] = -(cVar11 == cVar61);
          auVar64[7] = -(cVar12 == cVar62);
          auVar64[8] = -(cVar13 == cVar58);
          auVar64[9] = -(cVar14 == cVar60);
          auVar64[10] = -(cVar15 == cVar61);
          auVar64[0xb] = -(cVar16 == cVar62);
          auVar64[0xc] = -(cVar17 == cVar58);
          auVar64[0xd] = -(cVar18 == cVar60);
          auVar64[0xe] = -(cVar19 == cVar61);
          auVar64[0xf] = -(cVar20 == cVar62);
          uVar49 = (ushort)(SUB161(auVar64 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar64 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar64 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar64 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar64 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar64 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar64 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar64 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar64 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar64 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar64 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar64 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar64 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar64 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar64 >> 0x77,0) & 1) << 0xe |
                   (ushort)(auVar64[0xf] >> 7) << 0xf;
          pauVar52 = (undefined1 (*) [16])(ulonglong)uVar49;
          if (uVar49 != 0) {
            iVar5 = *(int *)(pauVar43[-3] + 8);
            *(longlong **)((longlong)ppuVar36 + 0x38) = plVar41;
            do {
              uVar26 = 0;
              for (uVar34 = (uint)pauVar52; (uVar34 & 1) == 0; uVar34 = uVar34 >> 1 | 0x80000000) {
                uVar26 = uVar26 + 1;
              }
              lVar42 = ((ulonglong)(*pauVar46 + uVar26) & uVar4) * -0x30;
              if ((iVar5 == *(int *)(lVar3 + -0x28 + lVar42)) &&
                 (pauVar29 = *(undefined1 (**) [16])*pauVar54,
                 pauVar29 == *(undefined1 (**) [16])(lVar42 + lVar3 + -0x30))) goto LAB_140822a97;
              iVar50 = (uint)pauVar52 - 1;
              uVar49 = (ushort)iVar50 & (ushort)pauVar52;
              pauVar52 = (undefined1 (*) [16])
                         (ulonglong)CONCAT22((short)((uint)iVar50 >> 0x10),uVar49);
              plVar41 = *(longlong **)((longlong)ppuVar36 + 0x38);
            } while (uVar49 != 0);
          }
          auVar63[0] = -(cVar48 == unaff_XMM6_Ba);
          auVar63[1] = -(cVar6 == unaff_XMM6_Bb);
          auVar63[2] = -(cVar7 == unaff_XMM6_Bc);
          auVar63[3] = -(cVar8 == unaff_XMM6_Bd);
          auVar63[4] = -(cVar9 == unaff_XMM6_Be);
          auVar63[5] = -(cVar10 == unaff_XMM6_Bf);
          auVar63[6] = -(cVar11 == unaff_XMM6_Bg);
          auVar63[7] = -(cVar12 == unaff_XMM6_Bh);
          auVar63[8] = -(cVar13 == unaff_XMM6_Bi);
          auVar63[9] = -(cVar14 == unaff_XMM6_Bj);
          auVar63[10] = -(cVar15 == unaff_XMM6_Bk);
          auVar63[0xb] = -(cVar16 == unaff_XMM6_Bl);
          auVar63[0xc] = -(cVar17 == unaff_XMM6_Bm);
          auVar63[0xd] = -(cVar18 == unaff_XMM6_Bn);
          auVar63[0xe] = -(cVar19 == unaff_XMM6_Bo);
          auVar63[0xf] = -(cVar20 == unaff_XMM6_Bp);
          uVar49 = (ushort)(SUB161(auVar63 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar63 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar63 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar63 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar63 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar63 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar63 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar63 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar63 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar63 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar63 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar63 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar63 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar63 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar63 >> 0x77,0) & 1) << 0xe |
                   (ushort)(auVar63[0xf] >> 7) << 0xf;
          pauVar29 = (undefined1 (*) [16])(ulonglong)uVar49;
          if (uVar49 != 0) break;
          puVar47 = (undefined1 *)((longlong)plVar41 + (longlong)(*pauVar46 + 0x10));
          plVar41 = plVar41 + 2;
        }
      }
    } while( true );
  case 0x23:
    ppuVar38 = apuStack_128;
    goto code_r0x000140822c0f;
  case 0x2d:
  case 0x2e:
  case 0x2f:
                    // WARNING (jumptable): Read-only address (ram,0x000142a90f79) is written
                    // WARNING: Read-only address (ram,0x000142a90f79) is written
    _UNK_142a90f79 = 0x65746171;
    ppuVar38 = apuStack_128;
    goto code_r0x000140822c6d;
  case 0x30:
    apuStack_128[0] = (undefined1 *)apuStack_128;
    goto code_r0x000140822c84;
  case 0x31:
    pauVar33 = (undefined1 (*) [16])(ulonglong)CONCAT11(0xd0,bVar57);
    goto code_r0x000140822d08;
  case 0x32:
                    // WARNING: Read-only address (ram,0x000142a90f79) is written
    return;
  case 0x33:
    bVar55 = CARRY1((*pauVar28)[0],bVar25);
    (*pauVar28)[0] = (*pauVar28)[0] + bVar25;
    ppauVar39 = (undefined1 (**) [16])apuStack_128;
code_r0x000140822d04:
    pauVar28 = (undefined1 (*) [16])((ulonglong)pauVar28 & 0xffffffff);
    ppuVar40 = (undefined1 **)ppauVar39;
    if (bVar55) {
      pauVar28 = pauVar52;
    }
code_r0x000140822d08:
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
    switch((ulonglong)pauVar28 & 0xff) {
    default:
      pauVar33 = (undefined1 (*) [16])0x0;
      bVar56 = (char)pauVar28 == '\x02';
code_r0x000140822d1e:
      pcVar1 = *pauVar28 + (longlong)*pauVar33;
      *pcVar1 = *pcVar1 >> ((byte)pauVar31 & 0x1f);
      bVar55 = ((ulonglong)pauVar31 & 0x1f) != 0;
      pauVar33 = (undefined1 (*) [16])
                 (ulonglong)((uint)(byte)(!bVar55 & bVar56 | (bVar55 && *pcVar1 == '\0')) * 2);
      goto code_r0x000140822d26;
    case 3:
      ppauVar39 = (undefined1 (**) [16])ppuVar40;
      pauVar44 = pauVar31;
code_r0x000140822d55:
      *(undefined **)((longlong)ppauVar39 + -8) = &UNK_140822d5a;
      country_core::resources::walls::decorator_subtype::impl$8::from();
      *(undefined8 *)((longlong)ppauVar39 + 0x28) = extraout_RAX_00;
      *(undefined1 (**) [16])((longlong)ppauVar39 + 0x30) = pauVar33;
      *(undefined **)((longlong)ppauVar39 + -8) = &UNK_140822d70;
      country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype();
      if (2 < (byte)(extraout_AL - 0x13U)) {
                    // WARNING: Subroutine does not return
        *(undefined **)((longlong)ppauVar39 + -8) = &UNK_140822df4;
        core::panicking::panic();
      }
      (*pauVar44)[1] = (char)(0x100 >> ((extraout_AL - 0x13U & 3) << 3));
      (*pauVar44)[0] = 0x11;
      break;
    case 4:
      (*pauVar31)[1] = cVar48 == '\x03';
      (*pauVar31)[0] = 0xb;
      break;
    case 5:
      pauVar28 = (undefined1 (*) [16])(ulonglong)(byte)(cVar48 - 2);
      pauVar33 = (undefined1 (*) [16])(*pauVar28 + 1);
      goto code_r0x000140822d96;
    case 6:
      (*pauVar31)[1] = (char)pauVar33;
      (*pauVar31)[0] = 0x23;
      break;
    case 7:
      break;
    case 8:
    case 10:
      pauVar33 = (undefined1 (*) [16])(ulonglong)((int)pauVar28 + 1);
code_r0x000140822d96:
      pauVar46 = (undefined1 (*) [16])0x0;
code_r0x000140822d99:
      if ((byte)pauVar28 < 3) {
        pauVar46 = pauVar33;
      }
                    // WARNING: Could not recover jumptable at 0x000140822dad. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&UNK_142a910c0 + *(int *)(&UNK_142a910c0 + (longlong)pauVar46 * 4)))
                (pauVar31,&UNK_142a910c0 + *(int *)(&UNK_142a910c0 + (longlong)pauVar46 * 4));
      return;
    case 9:
      *(int *)puVar35 = (int)*puVar35 + (int)pauVar31 + *(int *)*pauVar44;
    }
                    // WARNING: Read-only address (ram,0x000142a90f79) is written
    return;
  case 0x34:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x35:
  case 0x37:
    return;
  case 0x36:
    return;
  }
  bVar57 = bStack_f4;
  goto LAB_140822935;
}


