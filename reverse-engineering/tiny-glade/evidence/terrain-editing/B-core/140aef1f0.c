
/* WARNING: Instruction at (ram,0x000140af0380) overlaps instruction at (ram,0x000140af037c)
    */
/* WARNING: Removing unreachable block (ram,0x000140af0384) */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl country_core::systems::terrain::ui_edit_terrain::ui_edit_terrain(void)

{
  undefined8 *puVar1;
  undefined8 *puVar2;
  undefined4 uVar3;
  undefined1 uVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  undefined4 uVar11;
  undefined4 uVar12;
  int iVar13;
  undefined4 uVar14;
  undefined4 uVar15;
  undefined4 uVar16;
  longlong lVar17;
  ulonglong *puVar18;
  ulonglong uVar19;
  byte *pbVar20;
  undefined4 *puVar21;
  float *pfVar22;
  char *pcVar23;
  undefined8 uVar24;
  undefined4 *puVar25;
  undefined4 *puVar26;
  undefined1 (**ppauVar27) [16];
  undefined4 *puVar28;
  longlong lVar29;
  undefined4 *puVar30;
  undefined1 (*pauVar31) [16];
  undefined1 (*pauVar32) [16];
  longlong lVar33;
  uint uVar34;
  uint uVar35;
  undefined1 *puVar36;
  float fVar37;
  float fVar38;
  int iVar39;
  longlong lVar40;
  longlong extraout_RAX;
  char *pcVar41;
  ulonglong extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  undefined8 *in_RCX;
  byte bVar42;
  undefined8 *in_RDX;
  undefined1 (**ppauVar43) [16];
  undefined1 (*pauVar45) [16];
  longlong lVar46;
  undefined1 (*pauVar47) [16];
  ulonglong uVar48;
  undefined1 (*pauVar49) [16];
  undefined1 *puVar50;
  longlong *in_R8;
  ulonglong uVar51;
  int *piVar52;
  undefined1 (*pauVar53) [16];
  ulonglong uVar54;
  longlong lVar55;
  ulonglong uVar56;
  ushort uVar57;
  ulonglong uVar58;
  longlong lVar59;
  uint uVar60;
  uint uVar61;
  longlong lVar62;
  ulonglong uVar63;
  bool bVar64;
  bool bVar65;
  bool bVar66;
  bool bVar67;
  char cVar68;
  char cVar74;
  float fVar69;
  float fVar70;
  char cVar72;
  char cVar73;
  undefined1 extraout_XMM0 [16];
  undefined1 extraout_XMM0_00 [16];
  undefined1 auVar71 [16];
  undefined1 auVar75 [16];
  undefined1 auVar76 [16];
  float fVar77;
  float fVar78;
  float fVar79;
  float fVar80;
  float fVar81;
  float fVar82;
  undefined8 *in_stack_00000028;
  undefined8 *in_stack_00000030;
  longlong *in_stack_00000038;
  longlong *in_stack_00000040;
  longlong *in_stack_00000048;
  longlong *in_stack_00000050;
  undefined8 *in_stack_00000058;
  longlong *in_stack_00000060;
  undefined8 *in_stack_00000068;
  undefined8 *in_stack_00000070;
  undefined8 *in_stack_00000078;
  int local_228;
  undefined4 uStack_224;
  undefined4 local_220;
  undefined4 uStack_21c;
  undefined1 (**local_218) [16];
  ulonglong local_210;
  undefined8 local_208;
  undefined8 uStack_200;
  ulonglong local_1f8;
  undefined *puStack_1f0;
  undefined8 uStack_1e8;
  undefined4 local_1e0;
  undefined4 uStack_1dc;
  undefined *puStack_1d8;
  undefined8 uStack_1d0;
  undefined1 (**local_1c8) [16];
  undefined8 uStack_1c0;
  undefined8 uStack_1b8;
  undefined1 local_1b0;
  undefined1 local_1af;
  char local_1a0;
  undefined1 local_19f;
  undefined8 local_19c;
  undefined2 local_180;
  longlong local_178;
  undefined1 (**local_170) [16];
  undefined1 (*local_168) [16];
  byte *local_160;
  longlong local_158;
  int *local_150;
  undefined4 *local_148;
  undefined1 *local_140;
  undefined4 *local_138;
  longlong local_130;
  undefined4 *local_128;
  undefined4 *local_120;
  undefined8 *local_118;
  undefined1 (*local_110) [16];
  code *local_108;
  undefined1 (*local_100) [16];
  code *pcStack_f8;
  undefined1 (**ppauVar44) [16];
  
  if (*(int *)*in_RCX == 10) {
    lVar17 = in_stack_00000060[4];
    puVar18 = (ulonglong *)*in_stack_00000070;
    lVar62 = in_stack_00000070[1];
    uVar54 = *puVar18;
    uVar51 = 0;
    uVar58 = uVar54 - *(ulonglong *)(lVar62 + 0x18);
    if (uVar54 < *(ulonglong *)(lVar62 + 0x18)) {
      uVar58 = uVar51;
    }
    uVar56 = uVar54 - *(ulonglong *)(lVar62 + 0x38);
    if (uVar54 < *(ulonglong *)(lVar62 + 0x38)) {
      uVar56 = uVar51;
    }
    uVar54 = *(ulonglong *)(lVar62 + 0x10);
    uVar19 = *(ulonglong *)(lVar62 + 0x30);
    uVar48 = uVar19 - uVar56;
    if (uVar19 < uVar56) {
      uVar48 = uVar51;
    }
    uVar63 = uVar54 - uVar58;
    if (uVar54 < uVar58) {
      uVar63 = uVar51;
    }
    pbVar20 = (byte *)in_stack_00000070[5];
    uVar51 = *(ulonglong *)(lVar62 + 0x40);
    lVar46 = *(longlong *)(lVar62 + 8);
    lVar62 = *(longlong *)(lVar62 + 0x28);
    puVar21 = (undefined4 *)in_stack_00000060[6];
    uVar5 = *(undefined4 *)((longlong)in_stack_00000060 + 0x3c);
    pfVar22 = (float *)in_stack_00000060[8];
    *puVar18 = (uVar51 - uVar48) - uVar63;
    fVar77 = 0.0;
    fVar78 = 0.0;
    if (uVar54 >= uVar58 && uVar54 - uVar58 != 0 || uVar19 >= uVar56 && uVar19 - uVar56 != 0) {
      lVar62 = lVar62 + uVar56 * 0x10;
      lVar46 = lVar46 + uVar58 * 0x10;
      if (uVar54 < uVar58) {
        lVar46 = 8;
      }
      lVar40 = 0;
      if (uVar58 < uVar54) {
        lVar40 = lVar46 + -0x10 + uVar63 * 0x10;
      }
      if (uVar19 < uVar56) {
        lVar62 = 8;
      }
      lVar62 = lVar62 + -0x10 + uVar48 * 0x10;
      if (uVar19 <= uVar56) {
        lVar62 = lVar40;
      }
      *puVar18 = uVar51;
      fVar77 = (float)*(undefined8 *)(lVar62 + 8);
      fVar78 = (float)((ulonglong)*(undefined8 *)(lVar62 + 8) >> 0x20);
    }
    iVar39 = -1;
    if (*(char *)*in_stack_00000078 != '\0') {
      iVar39 = *(int *)((char *)*in_stack_00000078 + 8);
    }
    pcVar23 = (char *)*in_RDX;
    if ((iVar39 != -1) && (*pcVar23 == '\x01')) {
      *(undefined4 *)in_stack_00000060[2] = *(undefined4 *)((longlong)in_stack_00000060 + 0x1c);
      lVar62 = *in_stack_00000060;
      lVar46 = *(longlong *)(lVar62 + 0x30);
      uVar24 = *(undefined8 *)(lVar62 + 0x40);
      if (lVar46 == *(longlong *)(lVar62 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      lVar40 = *(longlong *)(lVar62 + 0x28);
      *(undefined8 *)(lVar40 + lVar46 * 0x10) = uVar24;
      *(undefined1 *)(lVar40 + 8 + lVar46 * 0x10) = 1;
      *(longlong *)(lVar62 + 0x30) = lVar46 + 1;
      *(longlong *)(lVar62 + 0x40) = *(longlong *)(lVar62 + 0x40) + 1;
    }
    lVar62 = in_stack_00000068[1];
    uVar54 = *(ulonglong *)*in_stack_00000068;
    uVar51 = 0;
    uVar58 = uVar54 - *(ulonglong *)(lVar62 + 0x18);
    if (uVar54 < *(ulonglong *)(lVar62 + 0x18)) {
      uVar58 = uVar51;
    }
    uVar56 = uVar54 - *(ulonglong *)(lVar62 + 0x38);
    if (uVar54 < *(ulonglong *)(lVar62 + 0x38)) {
      uVar56 = uVar51;
    }
    bVar64 = uVar56 <= *(ulonglong *)(lVar62 + 0x30);
    uVar56 = *(ulonglong *)(lVar62 + 0x30) - uVar56;
    bVar66 = uVar56 != 0;
    if (!bVar64) {
      uVar56 = uVar51;
    }
    bVar65 = uVar58 <= *(ulonglong *)(lVar62 + 0x10);
    uVar58 = *(ulonglong *)(lVar62 + 0x10) - uVar58;
    bVar67 = uVar58 != 0;
    if (!bVar65) {
      uVar58 = uVar51;
    }
    uVar54 = uVar58 + uVar56;
    if (bVar65 && bVar67 || bVar64 && bVar66) {
      uVar54 = uVar51;
    }
    *(ulonglong *)*in_stack_00000068 = *(longlong *)(lVar62 + 0x40) - uVar54;
    fVar38 = __real_41a00000;
    fVar37 = __real_3e99999a;
    lVar62 = *in_R8;
    puVar50 = *(undefined1 **)(lVar62 + 0x20);
    if (puVar50 != (undefined1 *)0x0) {
      uVar6 = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
      local_118 = (undefined8 *)*in_stack_00000030;
      local_120 = (undefined4 *)in_stack_00000030[2];
      puVar25 = (undefined4 *)*in_stack_00000028;
      puVar26 = (undefined4 *)in_stack_00000028[2];
      auVar75._0_4_ = fVar78 * fVar78 + fVar77 * fVar77;
      auVar75._4_4_ = fVar78 * fVar78;
      auVar75._8_8_ = 0;
      fVar79 = SQRT(auVar75._0_4_);
      fVar81 = (float)((uint)fVar77 & ___xmm_7fffffff7fffffff7fffffff7fffffff);
      fVar82 = (float)((uint)fVar78 & _UNK_14292e984);
      fVar77 = fVar77 * ___real_3b03126f;
      fVar80 = (float)(___xmm_7fffffff7fffffff7fffffff7fffffff & (uint)fVar77);
      fVar78 = (fVar78 * __real_3ca3d70a) / __real_41a00000;
      uVar7 = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
      uVar8 = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
      local_128 = (undefined4 *)in_stack_00000038[2];
      local_130 = *in_stack_00000038;
      pauVar49 = *(undefined1 (**) [16])(lVar62 + 8);
      auVar71 = *pauVar49;
      uVar60 = ~(uint)(ushort)((ushort)(SUB161(auVar71 >> 7,0) & 1) |
                               (ushort)(SUB161(auVar71 >> 0xf,0) & 1) << 1 |
                               (ushort)(SUB161(auVar71 >> 0x17,0) & 1) << 2 |
                               (ushort)(SUB161(auVar71 >> 0x1f,0) & 1) << 3 |
                               (ushort)(SUB161(auVar71 >> 0x27,0) & 1) << 4 |
                               (ushort)(SUB161(auVar71 >> 0x2f,0) & 1) << 5 |
                               (ushort)(SUB161(auVar71 >> 0x37,0) & 1) << 6 |
                               (ushort)(SUB161(auVar71 >> 0x3f,0) & 1) << 7 |
                               (ushort)(SUB161(auVar71 >> 0x47,0) & 1) << 8 |
                               (ushort)(SUB161(auVar71 >> 0x4f,0) & 1) << 9 |
                               (ushort)(SUB161(auVar71 >> 0x57,0) & 1) << 10 |
                               (ushort)(SUB161(auVar71 >> 0x5f,0) & 1) << 0xb |
                               (ushort)(SUB161(auVar71 >> 0x67,0) & 1) << 0xc |
                               (ushort)(SUB161(auVar71 >> 0x6f,0) & 1) << 0xd |
                               (ushort)(SUB161(auVar71 >> 0x77,0) & 1) << 0xe |
                              (ushort)(byte)(auVar71[0xf] >> 7) << 0xf);
      uVar9 = *(undefined4 *)((longlong)in_stack_00000058 + 0x1c);
      ppauVar27 = (undefined1 (**) [16])*in_stack_00000058;
      local_148 = (undefined4 *)in_stack_00000058[2];
      uVar10 = *(undefined4 *)((longlong)in_stack_00000048 + 0x1c);
      lVar46 = *in_stack_00000048;
      puVar28 = (undefined4 *)in_stack_00000048[2];
      uVar11 = *(undefined4 *)((longlong)in_stack_00000050 + 0x1c);
      lVar40 = *in_stack_00000050;
      local_138 = (undefined4 *)in_stack_00000050[2];
      uVar12 = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
      lVar29 = *in_stack_00000040;
      puVar30 = (undefined4 *)in_stack_00000040[2];
      local_150 = (int *)(longlong)iVar39;
      puVar1 = (undefined8 *)(puVar25 + 2);
      pauVar45 = pauVar49 + 1;
      local_170 = ppauVar27 + 4;
      local_158 = lVar40 + 0x20;
      local_178 = lVar62 + 0x28;
      local_160 = pbVar20 + 4;
      do {
        ppauVar43 = &local_100;
        if ((short)uVar60 == 0) {
          do {
            auVar71 = *pauVar45;
            uVar57 = (ushort)(SUB161(auVar71 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar71 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar71 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar71 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar71 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar71 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar71 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar71 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar71 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar71 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar71 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar71 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar71 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar71 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar71 >> 0x77,0) & 1) << 0xe |
                     (ushort)(byte)(auVar71[0xf] >> 7) << 0xf;
            pauVar49 = pauVar49 + -0x30;
            pauVar45 = pauVar45 + 1;
          } while (uVar57 == 0xffff);
          uVar61 = ~(uint)uVar57;
          uVar60 = uVar61 & -(uint)uVar57 - 2;
        }
        else {
          uVar61 = uVar60;
          uVar60 = uVar60 - 1 & uVar60;
        }
        uVar34 = 0;
        for (; (uVar61 & 1) == 0; uVar61 = uVar61 >> 1 | 0x80000000) {
          uVar34 = uVar34 + 1;
        }
        uVar54 = (ulonglong)uVar34;
        pauVar47 = pauVar49 + uVar54 * -3 + -3;
        local_108 = core::fmt::num::imp::impl$6::fmt;
        local_228 = 0x42ae3a08;
        uStack_224 = 1;
        local_220 = 2;
        uStack_21c = 0;
        local_218 = &local_110;
        local_210 = 1;
        local_208 = 0;
        local_110 = pauVar47;
        alloc::fmt::format::format_inner();
        local_1a0 = '\a';
        if (*pcVar23 == '\0') {
          local_1a0 = (iVar39 != *(int *)*pauVar47) * '\x03' + '\x04';
        }
        local_228 = 0;
        uStack_224 = 0;
        local_220 = 8;
        uStack_21c = 0;
        local_218 = (undefined1 (**) [16])0x0;
        local_210 = local_210 & 0xffffffff00000000;
        local_1f8 = local_1f8 & 0xffffffff00000000;
        local_1e0 = 0;
        local_1c8 = (undefined1 (**) [16])((ulonglong)local_1c8 & 0xffffffff00000000);
        local_1af = 0xb;
        local_19f = 0;
        local_19c = 0x4302000043020000;
        local_180 = 0x100;
        piVar52 = &local_228;
        local_1b0 = (!bVar65 || !bVar67) && (!bVar64 || !bVar66);
        systems::ui::resources::ui_state::UiState::add<alloc::string::String>();
        bVar42 = *(byte *)(extraout_RAX + 0x18);
        ppauVar44 = ppauVar43;
        auVar71 = extraout_XMM0;
        if (bVar42 == 1) {
          cVar68 = '\x02';
          if ((*(undefined1 (***) [16])(extraout_RAX + 0x38) == ppauVar43) &&
             (bVar42 = *(byte *)(extraout_RAX + 0x19) ^ 1 | iVar39 == -1 |
                       (bVar65 && bVar67 || bVar64 && bVar66),
             ppauVar44 = (undefined1 (**) [16])(ulonglong)bVar42, bVar42 == 0)) {
            *local_148 = uVar9;
            pauVar31 = ppauVar27[6];
            pauVar53 = ppauVar27[8];
            local_140 = puVar50;
            if (pauVar31 == ppauVar27[4]) {
              local_168 = ppauVar27[8];
              alloc::raw_vec::RawVec::
              grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                        ();
              pauVar53 = local_168;
              auVar71 = extraout_XMM0_00;
            }
            pauVar32 = ppauVar27[5];
            *(undefined1 (**) [16])pauVar32[(longlong)pauVar31] = pauVar53;
            *(int **)(pauVar32[(longlong)pauVar31] + 8) = local_150;
            ppauVar27[6] = (undefined1 (*) [16])(*pauVar31 + 1);
            ppauVar27[8] = (undefined1 (*) [16])(*ppauVar27[8] + 1);
            bVar42 = *(byte *)(extraout_RAX + 0x18);
            ppauVar44 = ppauVar27;
            puVar50 = local_140;
            piVar52 = local_150;
            goto LAB_140aef980;
          }
        }
        else {
LAB_140aef980:
          cVar68 = '\x02';
          if ((1 < bVar42) && (*(undefined1 (***) [16])(extraout_RAX + 0x38) == ppauVar43)) {
            cVar68 = *(char *)(extraout_RAX + 0x19);
          }
        }
        bVar42 = cVar68 - 2;
        if (2 < bVar42) {
          bVar42 = 3;
        }
        uVar58 = (ulonglong)*(int *)(&DAT_142ae404c + (ulonglong)bVar42 * 4);
        pcVar41 = &DAT_142ae404c + uVar58;
                    /* WARNING (jumptable): Sanity check requires truncation of jumptable */
                    /* WARNING: Could not find normalized switch variable to match jumptable */
        switch(bVar42) {
        case 0:
          break;
        case 1:
          *puVar28 = uVar10;
          lVar33 = *(longlong *)(lVar46 + 0x30);
          uVar24 = *(undefined8 *)(lVar46 + 0x40);
          if (lVar33 == *(longlong *)(lVar46 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                      ();
          }
          lVar55 = *(longlong *)(lVar46 + 0x28);
          *(undefined8 *)(lVar55 + lVar33 * 0x10) = uVar24;
          *(undefined1 *)(lVar55 + 8 + lVar33 * 0x10) = 0;
          *(longlong *)(lVar46 + 0x30) = lVar33 + 1;
          *(longlong *)(lVar46 + 0x40) = *(longlong *)(lVar46 + 0x40) + 1;
          uVar3 = *(undefined4 *)(pauVar49[uVar54 * -3 + -1] + 8);
          *puVar26 = uVar7;
          *puVar25 = uVar3;
          uVar3 = *(undefined4 *)pauVar49[uVar54 * -3 + -1];
          *puVar26 = uVar7;
          puVar25[1] = uVar3;
          if ((*pbVar20 & 1) == 0) {
            local_220 = 0;
            local_228 = 0;
            uStack_224 = 0;
          }
          else {
            local_220 = *(undefined4 *)(local_160 + 8);
            local_228 = (int)*(undefined8 *)local_160;
            uStack_224 = (undefined4)((ulonglong)*(undefined8 *)local_160 >> 0x20);
          }
          *puVar26 = uVar7;
          puVar25[4] = local_220;
          *puVar1 = CONCAT44(uStack_224,local_228);
          *local_128 = uVar8;
          *(undefined1 *)(local_130 + 0x1e0) = 1;
          *pfVar22 = 0.0;
          break;
        case 2:
          *local_138 = uVar11;
          lVar33 = *(longlong *)(lVar40 + 0x30);
          uVar24 = *(undefined8 *)(lVar40 + 0x40);
          if (lVar33 == *(longlong *)(lVar40 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                      ();
          }
          *(undefined8 *)(*(longlong *)(lVar40 + 0x28) + lVar33 * 8) = uVar24;
          *(longlong *)(lVar40 + 0x40) = *(longlong *)(lVar40 + 0x40) + 1;
          *(longlong *)(lVar40 + 0x30) = lVar33 + 1;
          if (0.0 < fVar79) {
            if (fVar82 <= fVar81) {
              uVar3 = *(undefined4 *)*pauVar47;
              uStack_21c = puVar25[4];
              uStack_224 = (undefined4)*puVar1;
              local_220 = (undefined4)((ulonglong)*puVar1 >> 0x20);
              *puVar30 = uVar12;
              lVar33 = *(longlong *)(lVar29 + 0x30);
              uVar24 = *(undefined8 *)(lVar29 + 0x40);
              if (lVar33 == *(longlong *)(lVar29 + 0x20)) {
                alloc::raw_vec::RawVec::
                grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                          ();
              }
              lVar55 = *(longlong *)(lVar29 + 0x28);
              *(undefined4 *)(lVar55 + lVar33 * 0x28) = 2;
              *(float *)(lVar55 + 4 + lVar33 * 0x28) = fVar77;
              *(undefined4 *)(lVar55 + 8 + lVar33 * 0x28) = 0;
              puVar2 = (undefined8 *)(lVar55 + 0xc + lVar33 * 0x28);
              *puVar2 = CONCAT44(uStack_224,local_228);
              puVar2[1] = CONCAT44(uStack_21c,local_220);
              *(undefined4 *)(lVar55 + 0x1c + lVar33 * 0x28) = uVar3;
              *(undefined8 *)(lVar55 + 0x20 + lVar33 * 0x28) = uVar24;
              *(longlong *)(lVar29 + 0x30) = lVar33 + 1;
              *(longlong *)(lVar29 + 0x40) = *(longlong *)(lVar29 + 0x40) + 1;
              fVar69 = *pfVar22 + fVar80;
              *pfVar22 = fVar69;
              if (fVar37 < fVar69) {
                *puVar21 = uVar5;
                lVar33 = *(longlong *)(lVar17 + 0x30);
                uVar24 = *(undefined8 *)(lVar17 + 0x40);
                if (lVar33 == *(longlong *)(lVar17 + 0x20)) {
                  alloc::raw_vec::RawVec::
                  grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                            ();
                }
                lVar55 = *(longlong *)(lVar17 + 0x28);
                *(undefined8 *)(lVar55 + lVar33 * 0x10) = uVar24;
                *(undefined1 *)(lVar55 + 8 + lVar33 * 0x10) = 1;
                *(longlong *)(lVar17 + 0x30) = lVar33 + 1;
                *(longlong *)(lVar17 + 0x40) = *(longlong *)(lVar17 + 0x40) + 1;
              }
            }
            else {
              iVar13 = *(int *)*pauVar47;
              local_228 = iVar13;
              if (*(longlong *)(lVar62 + 0x20) != 0) {
                core::hash::BuildHasher::
                hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>
                          ();
                lVar33 = *(longlong *)(lVar62 + 8);
                bVar42 = (byte)(extraout_RAX_00 >> 0x39);
                auVar71 = ZEXT216(CONCAT11(bVar42,bVar42));
                auVar71 = pshuflw(auVar71,auVar71,0);
                lVar55 = 0;
                uVar54 = extraout_RAX_00;
                while( true ) {
                  uVar54 = *(ulonglong *)(lVar62 + 0x10) & uVar54;
                  pcVar41 = (char *)(lVar33 + uVar54);
                  cVar68 = auVar71[0];
                  auVar76[0] = -(*pcVar41 == cVar68);
                  cVar72 = auVar71[1];
                  auVar76[1] = -(pcVar41[1] == cVar72);
                  cVar73 = auVar71[2];
                  auVar76[2] = -(pcVar41[2] == cVar73);
                  cVar74 = auVar71[3];
                  auVar76[3] = -(pcVar41[3] == cVar74);
                  auVar76[4] = -(pcVar41[4] == cVar68);
                  auVar76[5] = -(pcVar41[5] == cVar72);
                  auVar76[6] = -(pcVar41[6] == cVar73);
                  auVar76[7] = -(pcVar41[7] == cVar74);
                  auVar76[8] = -(pcVar41[8] == cVar68);
                  auVar76[9] = -(pcVar41[9] == cVar72);
                  auVar76[10] = -(pcVar41[10] == cVar73);
                  auVar76[0xb] = -(pcVar41[0xb] == cVar74);
                  auVar76[0xc] = -(pcVar41[0xc] == cVar68);
                  auVar76[0xd] = -(pcVar41[0xd] == cVar72);
                  auVar76[0xe] = -(pcVar41[0xe] == cVar73);
                  auVar76[0xf] = -(pcVar41[0xf] == cVar74);
                  uVar57 = (ushort)(SUB161(auVar76 >> 7,0) & 1) |
                           (ushort)(SUB161(auVar76 >> 0xf,0) & 1) << 1 |
                           (ushort)(SUB161(auVar76 >> 0x17,0) & 1) << 2 |
                           (ushort)(SUB161(auVar76 >> 0x1f,0) & 1) << 3 |
                           (ushort)(SUB161(auVar76 >> 0x27,0) & 1) << 4 |
                           (ushort)(SUB161(auVar76 >> 0x2f,0) & 1) << 5 |
                           (ushort)(SUB161(auVar76 >> 0x37,0) & 1) << 6 |
                           (ushort)(SUB161(auVar76 >> 0x3f,0) & 1) << 7 |
                           (ushort)(SUB161(auVar76 >> 0x47,0) & 1) << 8 |
                           (ushort)(SUB161(auVar76 >> 0x4f,0) & 1) << 9 |
                           (ushort)(SUB161(auVar76 >> 0x57,0) & 1) << 10 |
                           (ushort)(SUB161(auVar76 >> 0x5f,0) & 1) << 0xb |
                           (ushort)(SUB161(auVar76 >> 0x67,0) & 1) << 0xc |
                           (ushort)(SUB161(auVar76 >> 0x6f,0) & 1) << 0xd |
                           (ushort)(SUB161(auVar76 >> 0x77,0) & 1) << 0xe |
                           (ushort)(auVar76[0xf] >> 7) << 0xf;
                  uVar61 = (uint)uVar57;
                  while (uVar57 != 0) {
                    uVar34 = 0;
                    for (uVar35 = uVar61; (uVar35 & 1) == 0; uVar35 = uVar35 >> 1 | 0x80000000) {
                      uVar34 = uVar34 + 1;
                    }
                    lVar59 = (uVar34 + uVar54 & *(ulonglong *)(lVar62 + 0x10)) * -0x30;
                    if (*(int *)(lVar33 + -0x30 + lVar59) == iVar13) {
                      fVar69 = *(float *)(lVar33 + -8 + lVar59);
                      uVar3 = *(undefined4 *)*pauVar47;
                      local_220 = puVar25[4];
                      local_228 = (int)*puVar1;
                      uStack_224 = (undefined4)((ulonglong)*puVar1 >> 0x20);
                      *puVar30 = uVar12;
                      lVar33 = *(longlong *)(lVar29 + 0x30);
                      uVar24 = *(undefined8 *)(lVar29 + 0x40);
                      if (lVar33 == *(longlong *)(lVar29 + 0x20)) {
                        alloc::raw_vec::RawVec::
                        grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                                  ();
                      }
                      fVar70 = fVar69 / fVar38;
                      fVar70 = fVar70 * fVar70 - fVar78;
                      auVar75 = (undefined1  [16])0x0;
                      if (0.0 <= fVar70) {
                        auVar75._4_12_ = SUB1612((undefined1  [16])0x0,4);
                        auVar75._0_4_ = fVar70;
                      }
                      fVar70 = __real_3f800000;
                      if (auVar75._0_4_ <= __real_3f800000) {
                        fVar70 = auVar75._0_4_;
                      }
                      lVar55 = *(longlong *)(lVar29 + 0x28);
                      *(undefined4 *)(lVar55 + lVar33 * 0x28) = 0;
                      *(undefined4 *)(lVar55 + 8 + lVar33 * 0x28) = 2;
                      *(float *)(lVar55 + 0xc + lVar33 * 0x28) = SQRT(fVar70) * fVar38 - fVar69;
                      *(ulonglong *)(lVar55 + 0x10 + lVar33 * 0x28) = CONCAT44(uStack_224,local_228)
                      ;
                      *(undefined4 *)(lVar55 + 0x18 + lVar33 * 0x28) = local_220;
                      *(undefined4 *)(lVar55 + 0x1c + lVar33 * 0x28) = uVar3;
                      *(undefined8 *)(lVar55 + 0x20 + lVar33 * 0x28) = uVar24;
                      *(longlong *)(lVar29 + 0x30) = lVar33 + 1;
                      *(longlong *)(lVar29 + 0x40) = *(longlong *)(lVar29 + 0x40) + 1;
                      goto LAB_140aef71a;
                    }
                    uVar57 = (ushort)(uVar61 - 1) & (ushort)uVar61;
                    uVar61 = CONCAT22((short)(uVar61 - 1 >> 0x10),uVar57);
                  }
                  auVar75[0] = -(*pcVar41 == __xmm_ffffffffffffffffffffffffffffffff);
                  auVar75[1] = -(pcVar41[1] == UNK_142925991);
                  auVar75[2] = -(pcVar41[2] == UNK_142925992);
                  auVar75[3] = -(pcVar41[3] == UNK_142925993);
                  auVar75[4] = -(pcVar41[4] == UNK_142925994);
                  auVar75[5] = -(pcVar41[5] == UNK_142925995);
                  auVar75[6] = -(pcVar41[6] == UNK_142925996);
                  auVar75[7] = -(pcVar41[7] == UNK_142925997);
                  auVar75[8] = -(pcVar41[8] == UNK_142925998);
                  auVar75[9] = -(pcVar41[9] == UNK_142925999);
                  auVar75[10] = -(pcVar41[10] == UNK_14292599a);
                  auVar75[0xb] = -(pcVar41[0xb] == UNK_14292599b);
                  auVar75[0xc] = -(pcVar41[0xc] == UNK_14292599c);
                  auVar75[0xd] = -(pcVar41[0xd] == UNK_14292599d);
                  auVar75[0xe] = -(pcVar41[0xe] == UNK_14292599e);
                  auVar75[0xf] = -(pcVar41[0xf] == UNK_14292599f);
                  if ((((((((((((((((SUB161(auVar75 >> 7,0) & 1) != 0 ||
                                   (SUB161(auVar75 >> 0xf,0) & 1) != 0) ||
                                  (SUB161(auVar75 >> 0x17,0) & 1) != 0) ||
                                 (SUB161(auVar75 >> 0x1f,0) & 1) != 0) ||
                                (SUB161(auVar75 >> 0x27,0) & 1) != 0) ||
                               (SUB161(auVar75 >> 0x2f,0) & 1) != 0) ||
                              (SUB161(auVar75 >> 0x37,0) & 1) != 0) ||
                             (SUB161(auVar75 >> 0x3f,0) & 1) != 0) ||
                            (SUB161(auVar75 >> 0x47,0) & 1) != 0) ||
                           (SUB161(auVar75 >> 0x4f,0) & 1) != 0) ||
                          (SUB161(auVar75 >> 0x57,0) & 1) != 0) ||
                         (SUB161(auVar75 >> 0x5f,0) & 1) != 0) ||
                        (SUB161(auVar75 >> 0x67,0) & 1) != 0) ||
                       (SUB161(auVar75 >> 0x6f,0) & 1) != 0) || (SUB161(auVar75 >> 0x77,0) & 1) != 0
                      ) || auVar75[0xf] < '\0') break;
                  uVar54 = uVar54 + lVar55 + 0x10;
                  lVar55 = lVar55 + 0x10;
                }
              }
              if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
                pcStack_f8 = resources::terrain_editing::impl$11::fmt;
                local_100 = pauVar47;
                toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
                local_208 = *extraout_RAX_01;
                uStack_200 = extraout_RAX_01[1];
                uStack_1dc = *(undefined4 *)(extraout_RAX_01 + 2);
                local_1f8 = 1;
                puStack_1f0 = &UNK_142ae3aa0;
                uStack_1e8 = 0x2f;
                puStack_1d8 = &UNK_142ae3a68;
                uStack_1d0 = 2;
                local_1c8 = &local_100;
                uStack_1c0 = 1;
                uStack_1b8 = 0;
                local_228 = 0;
                uStack_224 = 0;
                local_220 = 0x42ae3aa0;
                uStack_21c = 1;
                local_218 = (undefined1 (**) [16])0x2f;
                local_210 = 0;
                local_1e0 = 1;
                ::log::__private_api::impl$2::log();
              }
            }
          }
          break;
        case 3:
          *puVar28 = uVar10;
          lVar33 = *(longlong *)(lVar46 + 0x30);
          uVar24 = *(undefined8 *)(lVar46 + 0x40);
          if (lVar33 == *(longlong *)(lVar46 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                      ();
          }
          lVar55 = *(longlong *)(lVar46 + 0x28);
          *(undefined8 *)(lVar55 + lVar33 * 0x10) = uVar24;
          *(undefined1 *)(lVar55 + 8 + lVar33 * 0x10) = 1;
          *(longlong *)(lVar46 + 0x30) = lVar33 + 1;
          *(longlong *)(lVar46 + 0x40) = *(longlong *)(lVar46 + 0x40) + 1;
          *puVar21 = uVar5;
          lVar33 = *(longlong *)(lVar17 + 0x30);
          uVar24 = *(undefined8 *)(lVar17 + 0x40);
          if (lVar33 == *(longlong *)(lVar17 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                      ();
          }
          lVar55 = *(longlong *)(lVar17 + 0x28);
          *(undefined8 *)(lVar55 + lVar33 * 0x10) = uVar24;
          *(undefined1 *)(lVar55 + 8 + lVar33 * 0x10) = 0;
          *(longlong *)(lVar17 + 0x30) = lVar33 + 1;
          *(longlong *)(lVar17 + 0x40) = *(longlong *)(lVar17 + 0x40) + 1;
          *local_120 = uVar6;
          uVar14 = *(undefined4 *)pauVar49[uVar54 * -3 + -3];
          uVar15 = *puVar25;
          uVar61 = puVar25[1];
          auVar75 = ZEXT416(uVar61);
          uVar16 = *(undefined4 *)(pauVar49[uVar54 * -3 + -1] + 8);
          uVar3 = *(undefined4 *)pauVar49[uVar54 * -3 + -1];
          local_208 = local_118[4];
          puVar36 = (undefined1 *)*local_118;
          local_218 = (undefined1 (**) [16])local_118[2];
          local_210 = local_118[3];
          local_228 = (int)puVar36;
          uStack_224 = (undefined4)((ulonglong)puVar36 >> 0x20);
          local_220 = (undefined4)local_118[1];
          uStack_21c = (undefined4)((ulonglong)local_118[1] >> 0x20);
          *local_118 = 0x8000000000000001;
          *(undefined4 *)(local_118 + 1) = uVar14;
          *(undefined4 *)((longlong)local_118 + 0xc) = uVar15;
          *(undefined4 *)(local_118 + 2) = uVar16;
          *(uint *)((longlong)local_118 + 0x14) = uVar61;
          *(undefined4 *)(local_118 + 3) = uVar3;
          if (puVar36 != &DAT_8000000000000002) {
            core::ptr::
            drop_in_place<enum2_<core::option::Option<enum2_<country_core::resources::history::terrain_edit::TerrainEdit>_>_>_>
                      ();
                    /* WARNING: Subroutine does not return */
            core::panicking::panic();
          }
          break;
        case 4:
                    /* WARNING: Could not recover jumptable at 0x000140af02ef. Too many branches */
                    /* WARNING: Treating indirect jump as call */
          (*(code *)((longlong)*(int *)((longlong)piVar52 * 4 + 0x42ae405c) + 0x42ae405c))();
          return;
        default:
          auVar71 = auVar71 & auVar75;
          if ((ushort)((ushort)(SUB161(auVar71 >> 7,0) & 1) |
                       (ushort)(SUB161(auVar71 >> 0xf,0) & 1) << 1 |
                       (ushort)(SUB161(auVar71 >> 0x17,0) & 1) << 2 |
                       (ushort)(SUB161(auVar71 >> 0x1f,0) & 1) << 3 |
                       (ushort)(SUB161(auVar71 >> 0x27,0) & 1) << 4 |
                       (ushort)(SUB161(auVar71 >> 0x2f,0) & 1) << 5 |
                       (ushort)(SUB161(auVar71 >> 0x37,0) & 1) << 6 |
                       (ushort)(SUB161(auVar71 >> 0x3f,0) & 1) << 7 |
                       (ushort)(SUB161(auVar71 >> 0x47,0) & 1) << 8 |
                       (ushort)(SUB161(auVar71 >> 0x4f,0) & 1) << 9 |
                       (ushort)(SUB161(auVar71 >> 0x57,0) & 1) << 10 |
                       (ushort)(SUB161(auVar71 >> 0x5f,0) & 1) << 0xb |
                       (ushort)(SUB161(auVar71 >> 0x67,0) & 1) << 0xc |
                       (ushort)(SUB161(auVar71 >> 0x6f,0) & 1) << 0xd |
                       (ushort)(SUB161(auVar71 >> 0x77,0) & 1) << 0xe |
                      (ushort)(byte)(auVar71[0xf] >> 7) << 0xf) == 0xffff) {
            (&UNK_142ae404d)[uVar58] = 1;
            *pcVar41 = '\0';
            return;
          }
          goto LAB_140af03b5;
        case 6:
          uVar4 = in((short)ppauVar44);
          *puVar50 = uVar4;
          out(*(undefined4 *)*pauVar49,(short)ppauVar44);
          if (uVar58 < 0xfffffffebd51bfb4) {
            if (((uint)ppauVar44 | 0x42ae404c) == 0) {
              (&UNK_142ae404d)[uVar58] = 0;
              *pcVar41 = '\0';
              return;
            }
            goto LAB_140af03b5;
          }
          *pcVar41 = *pcVar41 + (char)pcVar41;
        case 0xf:
LAB_140af03b5:
          (&UNK_142ae404d)[uVar58] = 6;
          *pcVar41 = '\0';
          return;
        case 8:
          if (*(int *)ppauVar44 == 0x42ae404c) {
            (&UNK_142ae404d)[uVar58] = 4;
            *pcVar41 = '\0';
            return;
          }
          goto LAB_140af03b5;
        case 0xc:
          if (ppauVar44 == (undefined1 (**) [16])0x0 && piVar52 == (int *)0x0) {
            (&UNK_142ae404d)[uVar58] = 5;
            *pcVar41 = '\0';
            return;
          }
          (&UNK_142ae404d)[uVar58] = 6;
          *pcVar41 = '\0';
          return;
        }
LAB_140aef71a:
        puVar50 = puVar50 + -1;
      } while (puVar50 != (undefined1 *)0x0);
    }
  }
  return;
}

