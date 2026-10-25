
// WARNING: Instruction at (ram,0x000140af0380) overlaps instruction at (ram,0x000140af037c)
// 
// WARNING: Removing unreachable block (ram,0x000140af0384)

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
  int iVar37;
  longlong lVar38;
  longlong extraout_RAX;
  char *pcVar39;
  ulonglong extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  undefined8 *in_RCX;
  byte bVar40;
  undefined8 *in_RDX;
  undefined1 (**ppauVar41) [16];
  undefined1 (*pauVar43) [16];
  longlong lVar44;
  undefined1 (*pauVar45) [16];
  ulonglong uVar46;
  undefined1 (*pauVar47) [16];
  undefined1 *puVar48;
  longlong *in_R8;
  ulonglong uVar49;
  int *piVar50;
  undefined1 (*pauVar51) [16];
  ulonglong uVar52;
  longlong lVar53;
  ulonglong uVar54;
  ushort uVar55;
  ulonglong uVar56;
  longlong lVar57;
  uint uVar58;
  uint uVar59;
  longlong lVar60;
  ulonglong uVar61;
  bool bVar62;
  bool bVar63;
  bool bVar64;
  bool bVar65;
  char cVar66;
  char cVar72;
  float fVar67;
  float fVar68;
  char cVar70;
  char cVar71;
  undefined1 extraout_XMM0 [16];
  undefined1 extraout_XMM0_00 [16];
  undefined1 auVar69 [16];
  undefined1 auVar73 [16];
  undefined1 auVar74 [16];
  float fVar75;
  float fVar76;
  float fVar77;
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
  undefined1 (**ppauVar42) [16];
  
  if (*(int *)*in_RCX == 10) {
    lVar17 = in_stack_00000060[4];
    puVar18 = (ulonglong *)*in_stack_00000070;
    lVar60 = in_stack_00000070[1];
    uVar52 = *puVar18;
    uVar49 = 0;
    uVar56 = uVar52 - *(ulonglong *)(lVar60 + 0x18);
    if (uVar52 < *(ulonglong *)(lVar60 + 0x18)) {
      uVar56 = uVar49;
    }
    uVar54 = uVar52 - *(ulonglong *)(lVar60 + 0x38);
    if (uVar52 < *(ulonglong *)(lVar60 + 0x38)) {
      uVar54 = uVar49;
    }
    uVar52 = *(ulonglong *)(lVar60 + 0x10);
    uVar19 = *(ulonglong *)(lVar60 + 0x30);
    uVar46 = uVar19 - uVar54;
    if (uVar19 < uVar54) {
      uVar46 = uVar49;
    }
    uVar61 = uVar52 - uVar56;
    if (uVar52 < uVar56) {
      uVar61 = uVar49;
    }
    pbVar20 = (byte *)in_stack_00000070[5];
    uVar49 = *(ulonglong *)(lVar60 + 0x40);
    lVar44 = *(longlong *)(lVar60 + 8);
    lVar60 = *(longlong *)(lVar60 + 0x28);
    puVar21 = (undefined4 *)in_stack_00000060[6];
    uVar5 = *(undefined4 *)((longlong)in_stack_00000060 + 0x3c);
    pfVar22 = (float *)in_stack_00000060[8];
    *puVar18 = (uVar49 - uVar46) - uVar61;
    fVar75 = 0.0;
    fVar76 = 0.0;
    if (uVar52 >= uVar56 && uVar52 - uVar56 != 0 || uVar19 >= uVar54 && uVar19 - uVar54 != 0) {
      lVar60 = lVar60 + uVar54 * 0x10;
      lVar44 = lVar44 + uVar56 * 0x10;
      if (uVar52 < uVar56) {
        lVar44 = 8;
      }
      lVar38 = 0;
      if (uVar56 < uVar52) {
        lVar38 = lVar44 + -0x10 + uVar61 * 0x10;
      }
      if (uVar19 < uVar54) {
        lVar60 = 8;
      }
      lVar60 = lVar60 + -0x10 + uVar46 * 0x10;
      if (uVar19 <= uVar54) {
        lVar60 = lVar38;
      }
      *puVar18 = uVar49;
      fVar75 = (float)*(undefined8 *)(lVar60 + 8);
      fVar76 = (float)((ulonglong)*(undefined8 *)(lVar60 + 8) >> 0x20);
    }
    iVar37 = -1;
    if (*(char *)*in_stack_00000078 != '\0') {
      iVar37 = *(int *)((char *)*in_stack_00000078 + 8);
    }
    pcVar23 = (char *)*in_RDX;
    if ((iVar37 != -1) && (*pcVar23 == '\x01')) {
      *(undefined4 *)in_stack_00000060[2] = *(undefined4 *)((longlong)in_stack_00000060 + 0x1c);
      lVar60 = *in_stack_00000060;
      lVar44 = *(longlong *)(lVar60 + 0x30);
      uVar24 = *(undefined8 *)(lVar60 + 0x40);
      if (lVar44 == *(longlong *)(lVar60 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      lVar38 = *(longlong *)(lVar60 + 0x28);
      *(undefined8 *)(lVar38 + lVar44 * 0x10) = uVar24;
      *(undefined1 *)(lVar38 + 8 + lVar44 * 0x10) = 1;
      *(longlong *)(lVar60 + 0x30) = lVar44 + 1;
      *(longlong *)(lVar60 + 0x40) = *(longlong *)(lVar60 + 0x40) + 1;
    }
    lVar60 = in_stack_00000068[1];
    uVar52 = *(ulonglong *)*in_stack_00000068;
    uVar49 = 0;
    uVar56 = uVar52 - *(ulonglong *)(lVar60 + 0x18);
    if (uVar52 < *(ulonglong *)(lVar60 + 0x18)) {
      uVar56 = uVar49;
    }
    uVar54 = uVar52 - *(ulonglong *)(lVar60 + 0x38);
    if (uVar52 < *(ulonglong *)(lVar60 + 0x38)) {
      uVar54 = uVar49;
    }
    bVar62 = uVar54 <= *(ulonglong *)(lVar60 + 0x30);
    uVar54 = *(ulonglong *)(lVar60 + 0x30) - uVar54;
    bVar64 = uVar54 != 0;
    if (!bVar62) {
      uVar54 = uVar49;
    }
    bVar63 = uVar56 <= *(ulonglong *)(lVar60 + 0x10);
    uVar56 = *(ulonglong *)(lVar60 + 0x10) - uVar56;
    bVar65 = uVar56 != 0;
    if (!bVar63) {
      uVar56 = uVar49;
    }
    uVar52 = uVar56 + uVar54;
    if (bVar63 && bVar65 || bVar62 && bVar64) {
      uVar52 = uVar49;
    }
    *(ulonglong *)*in_stack_00000068 = *(longlong *)(lVar60 + 0x40) - uVar52;
    lVar60 = *in_R8;
    puVar48 = *(undefined1 **)(lVar60 + 0x20);
    if (puVar48 != (undefined1 *)0x0) {
      uVar6 = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
      local_118 = (undefined8 *)*in_stack_00000030;
      local_120 = (undefined4 *)in_stack_00000030[2];
      puVar25 = (undefined4 *)*in_stack_00000028;
      puVar26 = (undefined4 *)in_stack_00000028[2];
      auVar73._0_4_ = fVar76 * fVar76 + fVar75 * fVar75;
      auVar73._4_4_ = fVar76 * fVar76;
      auVar73._8_8_ = 0;
      fVar77 = SQRT(auVar73._0_4_);
      uVar7 = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
      uVar8 = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
      local_128 = (undefined4 *)in_stack_00000038[2];
      local_130 = *in_stack_00000038;
      pauVar47 = *(undefined1 (**) [16])(lVar60 + 8);
      auVar69 = *pauVar47;
      uVar58 = ~(uint)(ushort)((ushort)(SUB161(auVar69 >> 7,0) & 1) |
                               (ushort)(SUB161(auVar69 >> 0xf,0) & 1) << 1 |
                               (ushort)(SUB161(auVar69 >> 0x17,0) & 1) << 2 |
                               (ushort)(SUB161(auVar69 >> 0x1f,0) & 1) << 3 |
                               (ushort)(SUB161(auVar69 >> 0x27,0) & 1) << 4 |
                               (ushort)(SUB161(auVar69 >> 0x2f,0) & 1) << 5 |
                               (ushort)(SUB161(auVar69 >> 0x37,0) & 1) << 6 |
                               (ushort)(SUB161(auVar69 >> 0x3f,0) & 1) << 7 |
                               (ushort)(SUB161(auVar69 >> 0x47,0) & 1) << 8 |
                               (ushort)(SUB161(auVar69 >> 0x4f,0) & 1) << 9 |
                               (ushort)(SUB161(auVar69 >> 0x57,0) & 1) << 10 |
                               (ushort)(SUB161(auVar69 >> 0x5f,0) & 1) << 0xb |
                               (ushort)(SUB161(auVar69 >> 0x67,0) & 1) << 0xc |
                               (ushort)(SUB161(auVar69 >> 0x6f,0) & 1) << 0xd |
                               (ushort)(SUB161(auVar69 >> 0x77,0) & 1) << 0xe |
                              (ushort)(byte)(auVar69[0xf] >> 7) << 0xf);
      uVar9 = *(undefined4 *)((longlong)in_stack_00000058 + 0x1c);
      ppauVar27 = (undefined1 (**) [16])*in_stack_00000058;
      local_148 = (undefined4 *)in_stack_00000058[2];
      uVar10 = *(undefined4 *)((longlong)in_stack_00000048 + 0x1c);
      lVar44 = *in_stack_00000048;
      puVar28 = (undefined4 *)in_stack_00000048[2];
      uVar11 = *(undefined4 *)((longlong)in_stack_00000050 + 0x1c);
      lVar38 = *in_stack_00000050;
      local_138 = (undefined4 *)in_stack_00000050[2];
      uVar12 = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
      lVar29 = *in_stack_00000040;
      puVar30 = (undefined4 *)in_stack_00000040[2];
      local_150 = (int *)(longlong)iVar37;
      puVar1 = (undefined8 *)(puVar25 + 2);
      pauVar43 = pauVar47 + 1;
      local_170 = ppauVar27 + 4;
      local_158 = lVar38 + 0x20;
      local_178 = lVar60 + 0x28;
      local_160 = pbVar20 + 4;
      do {
        ppauVar41 = &local_100;
        if ((short)uVar58 == 0) {
          do {
            auVar69 = *pauVar43;
            uVar55 = (ushort)(SUB161(auVar69 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar69 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar69 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar69 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar69 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar69 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar69 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar69 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar69 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar69 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar69 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar69 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar69 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar69 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar69 >> 0x77,0) & 1) << 0xe |
                     (ushort)(byte)(auVar69[0xf] >> 7) << 0xf;
            pauVar47 = pauVar47 + -0x30;
            pauVar43 = pauVar43 + 1;
          } while (uVar55 == 0xffff);
          uVar59 = ~(uint)uVar55;
          uVar58 = uVar59 & -(uint)uVar55 - 2;
        }
        else {
          uVar59 = uVar58;
          uVar58 = uVar58 - 1 & uVar58;
        }
        uVar34 = 0;
        for (; (uVar59 & 1) == 0; uVar59 = uVar59 >> 1 | 0x80000000) {
          uVar34 = uVar34 + 1;
        }
        uVar52 = (ulonglong)uVar34;
        pauVar45 = pauVar47 + uVar52 * -3 + -3;
        local_108 = core::fmt::num::imp::impl$6::fmt;
        local_228 = 0x42ae3a08;
        uStack_224 = 1;
        local_220 = 2;
        uStack_21c = 0;
        local_218 = &local_110;
        local_210 = 1;
        local_208 = 0;
        local_110 = pauVar45;
        alloc::fmt::format::format_inner();
        local_1a0 = '\a';
        if (*pcVar23 == '\0') {
          local_1a0 = (iVar37 != *(int *)*pauVar45) * '\x03' + '\x04';
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
        piVar50 = &local_228;
        local_1b0 = (!bVar63 || !bVar65) && (!bVar62 || !bVar64);
        systems::ui::resources::ui_state::UiState::add<alloc::string::String>();
        bVar40 = *(byte *)(extraout_RAX + 0x18);
        ppauVar42 = ppauVar41;
        auVar69 = extraout_XMM0;
        if (bVar40 == 1) {
          cVar66 = '\x02';
          if ((*(undefined1 (***) [16])(extraout_RAX + 0x38) == ppauVar41) &&
             (bVar40 = *(byte *)(extraout_RAX + 0x19) ^ 1 | iVar37 == -1 |
                       (bVar63 && bVar65 || bVar62 && bVar64),
             ppauVar42 = (undefined1 (**) [16])(ulonglong)bVar40, bVar40 == 0)) {
            *local_148 = uVar9;
            pauVar31 = ppauVar27[6];
            pauVar51 = ppauVar27[8];
            local_140 = puVar48;
            if (pauVar31 == ppauVar27[4]) {
              local_168 = ppauVar27[8];
              alloc::raw_vec::RawVec::
              grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                        ();
              pauVar51 = local_168;
              auVar69 = extraout_XMM0_00;
            }
            pauVar32 = ppauVar27[5];
            *(undefined1 (**) [16])pauVar32[(longlong)pauVar31] = pauVar51;
            *(int **)(pauVar32[(longlong)pauVar31] + 8) = local_150;
            ppauVar27[6] = (undefined1 (*) [16])(*pauVar31 + 1);
            ppauVar27[8] = (undefined1 (*) [16])(*ppauVar27[8] + 1);
            bVar40 = *(byte *)(extraout_RAX + 0x18);
            ppauVar42 = ppauVar27;
            puVar48 = local_140;
            piVar50 = local_150;
            goto LAB_140aef980;
          }
        }
        else {
LAB_140aef980:
          cVar66 = '\x02';
          if ((1 < bVar40) && (*(undefined1 (***) [16])(extraout_RAX + 0x38) == ppauVar41)) {
            cVar66 = *(char *)(extraout_RAX + 0x19);
          }
        }
        bVar40 = cVar66 - 2;
        if (2 < bVar40) {
          bVar40 = 3;
        }
        uVar56 = (ulonglong)*(int *)(&DAT_142ae404c + (ulonglong)bVar40 * 4);
        pcVar39 = &DAT_142ae404c + uVar56;
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
        switch(bVar40) {
        case 0:
          break;
        case 1:
          *puVar28 = uVar10;
          lVar33 = *(longlong *)(lVar44 + 0x30);
          uVar24 = *(undefined8 *)(lVar44 + 0x40);
          if (lVar33 == *(longlong *)(lVar44 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                      ();
          }
          lVar53 = *(longlong *)(lVar44 + 0x28);
          *(undefined8 *)(lVar53 + lVar33 * 0x10) = uVar24;
          *(undefined1 *)(lVar53 + 8 + lVar33 * 0x10) = 0;
          *(longlong *)(lVar44 + 0x30) = lVar33 + 1;
          *(longlong *)(lVar44 + 0x40) = *(longlong *)(lVar44 + 0x40) + 1;
          uVar3 = *(undefined4 *)(pauVar47[uVar52 * -3 + -1] + 8);
          *puVar26 = uVar7;
          *puVar25 = uVar3;
          uVar3 = *(undefined4 *)pauVar47[uVar52 * -3 + -1];
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
          lVar33 = *(longlong *)(lVar38 + 0x30);
          uVar24 = *(undefined8 *)(lVar38 + 0x40);
          if (lVar33 == *(longlong *)(lVar38 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                      ();
          }
          *(undefined8 *)(*(longlong *)(lVar38 + 0x28) + lVar33 * 8) = uVar24;
          *(longlong *)(lVar38 + 0x40) = *(longlong *)(lVar38 + 0x40) + 1;
          *(longlong *)(lVar38 + 0x30) = lVar33 + 1;
          if (0.0 < fVar77) {
            if (ABS(fVar76) <= ABS(fVar75)) {
              uVar3 = *(undefined4 *)*pauVar45;
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
              lVar53 = *(longlong *)(lVar29 + 0x28);
              *(undefined4 *)(lVar53 + lVar33 * 0x28) = 2;
              *(float *)(lVar53 + 4 + lVar33 * 0x28) = fVar75 * 0.002;
              *(undefined4 *)(lVar53 + 8 + lVar33 * 0x28) = 0;
              puVar2 = (undefined8 *)(lVar53 + 0xc + lVar33 * 0x28);
              *puVar2 = CONCAT44(uStack_224,local_228);
              puVar2[1] = CONCAT44(uStack_21c,local_220);
              *(undefined4 *)(lVar53 + 0x1c + lVar33 * 0x28) = uVar3;
              *(undefined8 *)(lVar53 + 0x20 + lVar33 * 0x28) = uVar24;
              *(longlong *)(lVar29 + 0x30) = lVar33 + 1;
              *(longlong *)(lVar29 + 0x40) = *(longlong *)(lVar29 + 0x40) + 1;
              fVar67 = *pfVar22 + ABS(fVar75 * 0.002);
              *pfVar22 = fVar67;
              if (0.3 < fVar67) {
                *puVar21 = uVar5;
                lVar33 = *(longlong *)(lVar17 + 0x30);
                uVar24 = *(undefined8 *)(lVar17 + 0x40);
                if (lVar33 == *(longlong *)(lVar17 + 0x20)) {
                  alloc::raw_vec::RawVec::
                  grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                            ();
                }
                lVar53 = *(longlong *)(lVar17 + 0x28);
                *(undefined8 *)(lVar53 + lVar33 * 0x10) = uVar24;
                *(undefined1 *)(lVar53 + 8 + lVar33 * 0x10) = 1;
                *(longlong *)(lVar17 + 0x30) = lVar33 + 1;
                *(longlong *)(lVar17 + 0x40) = *(longlong *)(lVar17 + 0x40) + 1;
              }
            }
            else {
              iVar13 = *(int *)*pauVar45;
              local_228 = iVar13;
              if (*(longlong *)(lVar60 + 0x20) != 0) {
                core::hash::BuildHasher::
                hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>
                          ();
                lVar33 = *(longlong *)(lVar60 + 8);
                bVar40 = (byte)(extraout_RAX_00 >> 0x39);
                auVar69 = ZEXT216(CONCAT11(bVar40,bVar40));
                auVar69 = pshuflw(auVar69,auVar69,0);
                lVar53 = 0;
                uVar52 = extraout_RAX_00;
                while( true ) {
                  uVar52 = *(ulonglong *)(lVar60 + 0x10) & uVar52;
                  pcVar39 = (char *)(lVar33 + uVar52);
                  cVar66 = auVar69[0];
                  auVar74[0] = -(*pcVar39 == cVar66);
                  cVar70 = auVar69[1];
                  auVar74[1] = -(pcVar39[1] == cVar70);
                  cVar71 = auVar69[2];
                  auVar74[2] = -(pcVar39[2] == cVar71);
                  cVar72 = auVar69[3];
                  auVar74[3] = -(pcVar39[3] == cVar72);
                  auVar74[4] = -(pcVar39[4] == cVar66);
                  auVar74[5] = -(pcVar39[5] == cVar70);
                  auVar74[6] = -(pcVar39[6] == cVar71);
                  auVar74[7] = -(pcVar39[7] == cVar72);
                  auVar74[8] = -(pcVar39[8] == cVar66);
                  auVar74[9] = -(pcVar39[9] == cVar70);
                  auVar74[10] = -(pcVar39[10] == cVar71);
                  auVar74[0xb] = -(pcVar39[0xb] == cVar72);
                  auVar74[0xc] = -(pcVar39[0xc] == cVar66);
                  auVar74[0xd] = -(pcVar39[0xd] == cVar70);
                  auVar74[0xe] = -(pcVar39[0xe] == cVar71);
                  auVar74[0xf] = -(pcVar39[0xf] == cVar72);
                  uVar55 = (ushort)(SUB161(auVar74 >> 7,0) & 1) |
                           (ushort)(SUB161(auVar74 >> 0xf,0) & 1) << 1 |
                           (ushort)(SUB161(auVar74 >> 0x17,0) & 1) << 2 |
                           (ushort)(SUB161(auVar74 >> 0x1f,0) & 1) << 3 |
                           (ushort)(SUB161(auVar74 >> 0x27,0) & 1) << 4 |
                           (ushort)(SUB161(auVar74 >> 0x2f,0) & 1) << 5 |
                           (ushort)(SUB161(auVar74 >> 0x37,0) & 1) << 6 |
                           (ushort)(SUB161(auVar74 >> 0x3f,0) & 1) << 7 |
                           (ushort)(SUB161(auVar74 >> 0x47,0) & 1) << 8 |
                           (ushort)(SUB161(auVar74 >> 0x4f,0) & 1) << 9 |
                           (ushort)(SUB161(auVar74 >> 0x57,0) & 1) << 10 |
                           (ushort)(SUB161(auVar74 >> 0x5f,0) & 1) << 0xb |
                           (ushort)(SUB161(auVar74 >> 0x67,0) & 1) << 0xc |
                           (ushort)(SUB161(auVar74 >> 0x6f,0) & 1) << 0xd |
                           (ushort)(SUB161(auVar74 >> 0x77,0) & 1) << 0xe |
                           (ushort)(auVar74[0xf] >> 7) << 0xf;
                  uVar59 = (uint)uVar55;
                  while (uVar55 != 0) {
                    uVar34 = 0;
                    for (uVar35 = uVar59; (uVar35 & 1) == 0; uVar35 = uVar35 >> 1 | 0x80000000) {
                      uVar34 = uVar34 + 1;
                    }
                    lVar57 = (uVar34 + uVar52 & *(ulonglong *)(lVar60 + 0x10)) * -0x30;
                    if (*(int *)(lVar33 + -0x30 + lVar57) == iVar13) {
                      fVar67 = *(float *)(lVar33 + -8 + lVar57);
                      uVar3 = *(undefined4 *)*pauVar45;
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
                      fVar68 = fVar67 / 20.0;
                      fVar68 = fVar68 * fVar68 - (fVar76 * 0.02) / 20.0;
                      auVar73 = (undefined1  [16])0x0;
                      if (0.0 <= fVar68) {
                        auVar73._4_12_ = SUB1612((undefined1  [16])0x0,4);
                        auVar73._0_4_ = fVar68;
                      }
                      fVar68 = 1.0;
                      if (auVar73._0_4_ <= 1.0) {
                        fVar68 = auVar73._0_4_;
                      }
                      lVar53 = *(longlong *)(lVar29 + 0x28);
                      *(undefined4 *)(lVar53 + lVar33 * 0x28) = 0;
                      *(undefined4 *)(lVar53 + 8 + lVar33 * 0x28) = 2;
                      *(float *)(lVar53 + 0xc + lVar33 * 0x28) = SQRT(fVar68) * 20.0 - fVar67;
                      *(ulonglong *)(lVar53 + 0x10 + lVar33 * 0x28) = CONCAT44(uStack_224,local_228)
                      ;
                      *(undefined4 *)(lVar53 + 0x18 + lVar33 * 0x28) = local_220;
                      *(undefined4 *)(lVar53 + 0x1c + lVar33 * 0x28) = uVar3;
                      *(undefined8 *)(lVar53 + 0x20 + lVar33 * 0x28) = uVar24;
                      *(longlong *)(lVar29 + 0x30) = lVar33 + 1;
                      *(longlong *)(lVar29 + 0x40) = *(longlong *)(lVar29 + 0x40) + 1;
                      goto LAB_140aef71a;
                    }
                    uVar55 = (ushort)(uVar59 - 1) & (ushort)uVar59;
                    uVar59 = CONCAT22((short)(uVar59 - 1 >> 0x10),uVar55);
                  }
                  auVar73[0] = -(*pcVar39 == -1);
                  auVar73[1] = -(pcVar39[1] == -1);
                  auVar73[2] = -(pcVar39[2] == -1);
                  auVar73[3] = -(pcVar39[3] == -1);
                  auVar73[4] = -(pcVar39[4] == -1);
                  auVar73[5] = -(pcVar39[5] == -1);
                  auVar73[6] = -(pcVar39[6] == -1);
                  auVar73[7] = -(pcVar39[7] == -1);
                  auVar73[8] = -(pcVar39[8] == -1);
                  auVar73[9] = -(pcVar39[9] == -1);
                  auVar73[10] = -(pcVar39[10] == -1);
                  auVar73[0xb] = -(pcVar39[0xb] == -1);
                  auVar73[0xc] = -(pcVar39[0xc] == -1);
                  auVar73[0xd] = -(pcVar39[0xd] == -1);
                  auVar73[0xe] = -(pcVar39[0xe] == -1);
                  auVar73[0xf] = -(pcVar39[0xf] == -1);
                  if ((((((((((((((((SUB161(auVar73 >> 7,0) & 1) != 0 ||
                                   (SUB161(auVar73 >> 0xf,0) & 1) != 0) ||
                                  (SUB161(auVar73 >> 0x17,0) & 1) != 0) ||
                                 (SUB161(auVar73 >> 0x1f,0) & 1) != 0) ||
                                (SUB161(auVar73 >> 0x27,0) & 1) != 0) ||
                               (SUB161(auVar73 >> 0x2f,0) & 1) != 0) ||
                              (SUB161(auVar73 >> 0x37,0) & 1) != 0) ||
                             (SUB161(auVar73 >> 0x3f,0) & 1) != 0) ||
                            (SUB161(auVar73 >> 0x47,0) & 1) != 0) ||
                           (SUB161(auVar73 >> 0x4f,0) & 1) != 0) ||
                          (SUB161(auVar73 >> 0x57,0) & 1) != 0) ||
                         (SUB161(auVar73 >> 0x5f,0) & 1) != 0) ||
                        (SUB161(auVar73 >> 0x67,0) & 1) != 0) ||
                       (SUB161(auVar73 >> 0x6f,0) & 1) != 0) || (SUB161(auVar73 >> 0x77,0) & 1) != 0
                      ) || auVar73[0xf] < '\0') break;
                  uVar52 = uVar52 + lVar53 + 0x10;
                  lVar53 = lVar53 + 0x10;
                }
              }
              if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
                pcStack_f8 = resources::terrain_editing::impl$11::fmt;
                local_100 = pauVar45;
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
          lVar33 = *(longlong *)(lVar44 + 0x30);
          uVar24 = *(undefined8 *)(lVar44 + 0x40);
          if (lVar33 == *(longlong *)(lVar44 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                      ();
          }
          lVar53 = *(longlong *)(lVar44 + 0x28);
          *(undefined8 *)(lVar53 + lVar33 * 0x10) = uVar24;
          *(undefined1 *)(lVar53 + 8 + lVar33 * 0x10) = 1;
          *(longlong *)(lVar44 + 0x30) = lVar33 + 1;
          *(longlong *)(lVar44 + 0x40) = *(longlong *)(lVar44 + 0x40) + 1;
          *puVar21 = uVar5;
          lVar33 = *(longlong *)(lVar17 + 0x30);
          uVar24 = *(undefined8 *)(lVar17 + 0x40);
          if (lVar33 == *(longlong *)(lVar17 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                      ();
          }
          lVar53 = *(longlong *)(lVar17 + 0x28);
          *(undefined8 *)(lVar53 + lVar33 * 0x10) = uVar24;
          *(undefined1 *)(lVar53 + 8 + lVar33 * 0x10) = 0;
          *(longlong *)(lVar17 + 0x30) = lVar33 + 1;
          *(longlong *)(lVar17 + 0x40) = *(longlong *)(lVar17 + 0x40) + 1;
          *local_120 = uVar6;
          uVar14 = *(undefined4 *)pauVar47[uVar52 * -3 + -3];
          uVar15 = *puVar25;
          uVar59 = puVar25[1];
          auVar73 = ZEXT416(uVar59);
          uVar16 = *(undefined4 *)(pauVar47[uVar52 * -3 + -1] + 8);
          uVar3 = *(undefined4 *)pauVar47[uVar52 * -3 + -1];
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
          *(uint *)((longlong)local_118 + 0x14) = uVar59;
          *(undefined4 *)(local_118 + 3) = uVar3;
          if (puVar36 != &DAT_8000000000000002) {
            core::ptr::
            drop_in_place<enum2_<core::option::Option<enum2_<country_core::resources::history::terrain_edit::TerrainEdit>_>_>_>
                      ();
                    // WARNING: Subroutine does not return
            core::panicking::panic();
          }
          break;
        case 4:
                    // WARNING: Could not recover jumptable at 0x000140af02ef. Too many branches
                    // WARNING: Treating indirect jump as call
          (*(code *)((longlong)*(int *)((longlong)piVar50 * 4 + 0x42ae405c) + 0x42ae405c))();
          return;
        default:
          auVar69 = auVar69 & auVar73;
          if ((ushort)((ushort)(SUB161(auVar69 >> 7,0) & 1) |
                       (ushort)(SUB161(auVar69 >> 0xf,0) & 1) << 1 |
                       (ushort)(SUB161(auVar69 >> 0x17,0) & 1) << 2 |
                       (ushort)(SUB161(auVar69 >> 0x1f,0) & 1) << 3 |
                       (ushort)(SUB161(auVar69 >> 0x27,0) & 1) << 4 |
                       (ushort)(SUB161(auVar69 >> 0x2f,0) & 1) << 5 |
                       (ushort)(SUB161(auVar69 >> 0x37,0) & 1) << 6 |
                       (ushort)(SUB161(auVar69 >> 0x3f,0) & 1) << 7 |
                       (ushort)(SUB161(auVar69 >> 0x47,0) & 1) << 8 |
                       (ushort)(SUB161(auVar69 >> 0x4f,0) & 1) << 9 |
                       (ushort)(SUB161(auVar69 >> 0x57,0) & 1) << 10 |
                       (ushort)(SUB161(auVar69 >> 0x5f,0) & 1) << 0xb |
                       (ushort)(SUB161(auVar69 >> 0x67,0) & 1) << 0xc |
                       (ushort)(SUB161(auVar69 >> 0x6f,0) & 1) << 0xd |
                       (ushort)(SUB161(auVar69 >> 0x77,0) & 1) << 0xe |
                      (ushort)(byte)(auVar69[0xf] >> 7) << 0xf) == 0xffff) {
            (&UNK_142ae404d)[uVar56] = 1;
            *pcVar39 = '\0';
            return;
          }
          goto LAB_140af03b5;
        case 6:
          uVar4 = in((short)ppauVar42);
          *puVar48 = uVar4;
          out(*(undefined4 *)*pauVar47,(short)ppauVar42);
          if (uVar56 < 0xfffffffebd51bfb4) {
            if (((uint)ppauVar42 | 0x42ae404c) == 0) {
              (&UNK_142ae404d)[uVar56] = 0;
              *pcVar39 = '\0';
              return;
            }
            goto LAB_140af03b5;
          }
          *pcVar39 = *pcVar39 + (char)pcVar39;
        case 0xf:
LAB_140af03b5:
          (&UNK_142ae404d)[uVar56] = 6;
          *pcVar39 = '\0';
          return;
        case 8:
          if (*(int *)ppauVar42 == 0x42ae404c) {
            (&UNK_142ae404d)[uVar56] = 4;
            *pcVar39 = '\0';
            return;
          }
          goto LAB_140af03b5;
        case 0xc:
          if (ppauVar42 == (undefined1 (**) [16])0x0 && piVar50 == (int *)0x0) {
            (&UNK_142ae404d)[uVar56] = 5;
            *pcVar39 = '\0';
            return;
          }
          (&UNK_142ae404d)[uVar56] = 6;
          *pcVar39 = '\0';
          return;
        }
LAB_140aef71a:
        puVar48 = puVar48 + -1;
      } while (puVar48 != (undefined1 *)0x0);
    }
  }
  return;
}


