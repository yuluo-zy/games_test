
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl country_core::systems::terrain::ui_edit_terrain::ui_edit_terrain(void)

{
  longlong *plVar1;
  char *pcVar2;
  longlong *plVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  undefined4 uVar11;
  undefined4 uVar12;
  undefined4 uVar13;
  int iVar14;
  undefined4 uVar15;
  undefined4 uVar16;
  undefined4 uVar17;
  longlong lVar18;
  ulonglong *puVar19;
  byte *pbVar20;
  undefined4 *puVar21;
  float *pfVar22;
  undefined8 uVar23;
  undefined8 *puVar24;
  undefined4 *puVar25;
  undefined4 *puVar26;
  undefined4 *puVar27;
  undefined4 *puVar28;
  longlong lVar29;
  undefined4 *puVar30;
  longlong lVar31;
  undefined4 *puVar32;
  longlong lVar33;
  undefined4 *puVar34;
  longlong lVar35;
  undefined4 *puVar36;
  longlong lVar37;
  uint uVar38;
  uint uVar39;
  undefined1 *puVar40;
  float fVar41;
  float fVar42;
  byte bVar43;
  int iVar44;
  longlong lVar45;
  longlong extraout_RAX;
  ulonglong extraout_RAX_00;
  undefined8 *in_RCX;
  undefined8 *in_RDX;
  undefined1 (**ppauVar46) [16];
  undefined1 (*pauVar47) [16];
  longlong lVar48;
  undefined1 (*pauVar49) [16];
  ulonglong uVar50;
  undefined1 (*pauVar51) [16];
  longlong *in_R8;
  ulonglong uVar52;
  ulonglong uVar53;
  longlong lVar54;
  ulonglong uVar55;
  ushort uVar56;
  ulonglong uVar57;
  ulonglong uVar58;
  longlong lVar59;
  uint uVar60;
  uint uVar61;
  longlong lVar62;
  ulonglong uVar63;
  bool bVar64;
  bool bVar65;
  char cVar66;
  char cVar73;
  float fVar67;
  float fVar68;
  float fVar69;
  char cVar71;
  char cVar72;
  undefined1 auVar70 [16];
  undefined1 auVar74 [16];
  undefined1 auVar75 [16];
  float fVar76;
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
  longlong *in_stack_00000058;
  longlong *in_stack_00000060;
  undefined8 *in_stack_00000068;
  undefined8 *in_stack_00000070;
  undefined8 *in_stack_00000078;
  undefined4 local_228;
  undefined4 uStack_224;
  undefined4 local_220;
  undefined1 (*local_100) [16];
  code *pcStack_f8;
  
  if (*(int *)*in_RCX == 10) {
    lVar18 = in_stack_00000060[4];
    puVar19 = (ulonglong *)*in_stack_00000070;
    lVar62 = in_stack_00000070[1];
    uVar53 = *puVar19;
    uVar52 = 0;
    uVar57 = uVar53 - *(ulonglong *)(lVar62 + 0x18);
    if (uVar53 < *(ulonglong *)(lVar62 + 0x18)) {
      uVar57 = uVar52;
    }
    uVar55 = uVar53 - *(ulonglong *)(lVar62 + 0x38);
    if (uVar53 < *(ulonglong *)(lVar62 + 0x38)) {
      uVar55 = uVar52;
    }
    uVar53 = *(ulonglong *)(lVar62 + 0x10);
    uVar58 = *(ulonglong *)(lVar62 + 0x30);
    uVar50 = uVar58 - uVar55;
    if (uVar58 < uVar55) {
      uVar50 = uVar52;
    }
    uVar63 = uVar53 - uVar57;
    if (uVar53 < uVar57) {
      uVar63 = uVar52;
    }
    pbVar20 = (byte *)in_stack_00000070[5];
    uVar52 = *(ulonglong *)(lVar62 + 0x40);
    lVar48 = *(longlong *)(lVar62 + 8);
    lVar62 = *(longlong *)(lVar62 + 0x28);
    puVar21 = (undefined4 *)in_stack_00000060[6];
    uVar6 = *(undefined4 *)((longlong)in_stack_00000060 + 0x3c);
    pfVar22 = (float *)in_stack_00000060[8];
    *puVar19 = (uVar52 - uVar50) - uVar63;
    fVar76 = 0.0;
    fVar78 = 0.0;
    if (uVar53 >= uVar57 && uVar53 - uVar57 != 0 || uVar58 >= uVar55 && uVar58 - uVar55 != 0) {
      lVar62 = lVar62 + uVar55 * 0x10;
      lVar48 = lVar48 + uVar57 * 0x10;
      if (uVar53 < uVar57) {
        lVar48 = 8;
      }
      lVar45 = 0;
      if (uVar57 < uVar53) {
        lVar45 = lVar48 + -0x10 + uVar63 * 0x10;
      }
      if (uVar58 < uVar55) {
        lVar62 = 8;
      }
      lVar62 = lVar62 + -0x10 + uVar50 * 0x10;
      if (uVar58 <= uVar55) {
        lVar62 = lVar45;
      }
      *puVar19 = uVar52;
      fVar76 = (float)*(undefined8 *)(lVar62 + 8);
      fVar78 = (float)((ulonglong)*(undefined8 *)(lVar62 + 8) >> 0x20);
    }
    iVar44 = -1;
    if (*(char *)*in_stack_00000078 != '\0') {
      iVar44 = *(int *)((char *)*in_stack_00000078 + 8);
    }
    if ((iVar44 != -1) && (*(char *)*in_RDX == '\x01')) {
      *(undefined4 *)in_stack_00000060[2] = *(undefined4 *)((longlong)in_stack_00000060 + 0x1c);
      lVar62 = *in_stack_00000060;
      lVar48 = *(longlong *)(lVar62 + 0x30);
      uVar23 = *(undefined8 *)(lVar62 + 0x40);
      if (lVar48 == *(longlong *)(lVar62 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      lVar45 = *(longlong *)(lVar62 + 0x28);
      *(undefined8 *)(lVar45 + lVar48 * 0x10) = uVar23;
      *(undefined1 *)(lVar45 + 8 + lVar48 * 0x10) = 1;
      *(longlong *)(lVar62 + 0x30) = lVar48 + 1;
      *(longlong *)(lVar62 + 0x40) = *(longlong *)(lVar62 + 0x40) + 1;
    }
    lVar62 = in_stack_00000068[1];
    uVar53 = *(ulonglong *)*in_stack_00000068;
    uVar52 = 0;
    uVar57 = uVar53 - *(ulonglong *)(lVar62 + 0x18);
    if (uVar53 < *(ulonglong *)(lVar62 + 0x18)) {
      uVar57 = uVar52;
    }
    uVar55 = uVar53 - *(ulonglong *)(lVar62 + 0x38);
    if (uVar53 < *(ulonglong *)(lVar62 + 0x38)) {
      uVar55 = uVar52;
    }
    bVar64 = uVar55 <= *(ulonglong *)(lVar62 + 0x30);
    uVar55 = *(ulonglong *)(lVar62 + 0x30) - uVar55;
    uVar53 = uVar55;
    if (!bVar64) {
      uVar53 = uVar52;
    }
    bVar65 = uVar57 <= *(ulonglong *)(lVar62 + 0x10);
    uVar57 = *(ulonglong *)(lVar62 + 0x10) - uVar57;
    uVar58 = uVar57;
    if (!bVar65) {
      uVar58 = uVar52;
    }
    uVar53 = uVar58 + uVar53;
    if (bVar65 && uVar57 != 0 || bVar64 && uVar55 != 0) {
      uVar53 = uVar52;
    }
    *(ulonglong *)*in_stack_00000068 = *(longlong *)(lVar62 + 0x40) - uVar53;
    fVar42 = __real_41a00000;
    fVar41 = __real_3e99999a;
    lVar62 = *in_R8;
    lVar48 = *(longlong *)(lVar62 + 0x20);
    if (lVar48 != 0) {
      uVar7 = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
      puVar24 = (undefined8 *)*in_stack_00000030;
      puVar25 = (undefined4 *)in_stack_00000030[2];
      puVar26 = (undefined4 *)*in_stack_00000028;
      puVar27 = (undefined4 *)in_stack_00000028[2];
      fVar80 = (float)((uint)fVar76 & ___xmm_7fffffff7fffffff7fffffff7fffffff);
      fVar81 = (float)((uint)fVar78 & _UNK_14292e984);
      fVar77 = fVar76 * ___real_3b03126f;
      fVar79 = (float)(___xmm_7fffffff7fffffff7fffffff7fffffff & (uint)fVar77);
      fVar82 = (fVar78 * __real_3ca3d70a) / __real_41a00000;
      uVar8 = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
      uVar9 = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
      puVar28 = (undefined4 *)in_stack_00000038[2];
      lVar45 = *in_stack_00000038;
      pauVar51 = *(undefined1 (**) [16])(lVar62 + 8);
      auVar70 = *pauVar51;
      uVar60 = ~(uint)(ushort)((ushort)(SUB161(auVar70 >> 7,0) & 1) |
                               (ushort)(SUB161(auVar70 >> 0xf,0) & 1) << 1 |
                               (ushort)(SUB161(auVar70 >> 0x17,0) & 1) << 2 |
                               (ushort)(SUB161(auVar70 >> 0x1f,0) & 1) << 3 |
                               (ushort)(SUB161(auVar70 >> 0x27,0) & 1) << 4 |
                               (ushort)(SUB161(auVar70 >> 0x2f,0) & 1) << 5 |
                               (ushort)(SUB161(auVar70 >> 0x37,0) & 1) << 6 |
                               (ushort)(SUB161(auVar70 >> 0x3f,0) & 1) << 7 |
                               (ushort)(SUB161(auVar70 >> 0x47,0) & 1) << 8 |
                               (ushort)(SUB161(auVar70 >> 0x4f,0) & 1) << 9 |
                               (ushort)(SUB161(auVar70 >> 0x57,0) & 1) << 10 |
                               (ushort)(SUB161(auVar70 >> 0x5f,0) & 1) << 0xb |
                               (ushort)(SUB161(auVar70 >> 0x67,0) & 1) << 0xc |
                               (ushort)(SUB161(auVar70 >> 0x6f,0) & 1) << 0xd |
                               (ushort)(SUB161(auVar70 >> 0x77,0) & 1) << 0xe |
                              (ushort)(byte)(auVar70[0xf] >> 7) << 0xf);
      uVar10 = *(undefined4 *)((longlong)in_stack_00000058 + 0x1c);
      lVar29 = *in_stack_00000058;
      puVar30 = (undefined4 *)in_stack_00000058[2];
      uVar11 = *(undefined4 *)((longlong)in_stack_00000048 + 0x1c);
      lVar31 = *in_stack_00000048;
      puVar32 = (undefined4 *)in_stack_00000048[2];
      uVar12 = *(undefined4 *)((longlong)in_stack_00000050 + 0x1c);
      lVar33 = *in_stack_00000050;
      puVar34 = (undefined4 *)in_stack_00000050[2];
      uVar13 = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
      lVar35 = *in_stack_00000040;
      puVar36 = (undefined4 *)in_stack_00000040[2];
      plVar1 = (longlong *)(puVar26 + 2);
      pauVar47 = pauVar51 + 1;
      do {
        ppauVar46 = &local_100;
        if ((short)uVar60 == 0) {
          do {
            auVar70 = *pauVar47;
            uVar56 = (ushort)(SUB161(auVar70 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar70 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar70 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar70 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar70 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar70 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar70 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar70 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar70 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar70 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar70 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar70 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar70 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar70 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar70 >> 0x77,0) & 1) << 0xe |
                     (ushort)(byte)(auVar70[0xf] >> 7) << 0xf;
            pauVar51 = pauVar51 + -0x30;
            pauVar47 = pauVar47 + 1;
          } while (uVar56 == 0xffff);
          uVar61 = ~(uint)uVar56;
          uVar60 = uVar61 & -(uint)uVar56 - 2;
        }
        else {
          uVar61 = uVar60;
          uVar60 = uVar60 - 1 & uVar60;
        }
        uVar38 = 0;
        for (; (uVar61 & 1) == 0; uVar61 = uVar61 >> 1 | 0x80000000) {
          uVar38 = uVar38 + 1;
        }
        uVar53 = (ulonglong)uVar38;
        pauVar49 = pauVar51 + uVar53 * -3 + -3;
        alloc::fmt::format::format_inner();
        systems::ui::resources::ui_state::UiState::add<alloc::string::String>();
        bVar43 = *(byte *)(extraout_RAX + 0x18);
        if (bVar43 == 1) {
          cVar66 = '\x02';
          if ((*(undefined1 (***) [16])(extraout_RAX + 0x38) == ppauVar46) &&
             ((*(char *)(extraout_RAX + 0x19) == '\x01' && iVar44 != -1) &&
              ((!bVar65 || uVar57 == 0) && (!bVar64 || uVar55 == 0)))) {
            *puVar30 = uVar10;
            lVar37 = *(longlong *)(lVar29 + 0x30);
            uVar23 = *(undefined8 *)(lVar29 + 0x40);
            if (lVar37 == *(longlong *)(lVar29 + 0x20)) {
              alloc::raw_vec::RawVec::
              grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                        ();
            }
            lVar54 = *(longlong *)(lVar29 + 0x28);
            *(undefined8 *)(lVar54 + lVar37 * 0x10) = uVar23;
            *(longlong *)(lVar54 + 8 + lVar37 * 0x10) = (longlong)iVar44;
            *(longlong *)(lVar29 + 0x30) = lVar37 + 1;
            *(longlong *)(lVar29 + 0x40) = *(longlong *)(lVar29 + 0x40) + 1;
            bVar43 = *(byte *)(extraout_RAX + 0x18);
            goto LAB_140aef980;
          }
        }
        else {
LAB_140aef980:
          cVar66 = '\x02';
          if ((1 < bVar43) && (*(undefined1 (***) [16])(extraout_RAX + 0x38) == ppauVar46)) {
            cVar66 = *(char *)(extraout_RAX + 0x19);
          }
        }
        bVar43 = cVar66 - 2;
        if (2 < bVar43) {
          bVar43 = 3;
        }
                    /* WARNING: Switch is manually overridden */
        switch(bVar43) {
        case 1:
          *puVar32 = uVar11;
          lVar37 = *(longlong *)(lVar31 + 0x30);
          uVar23 = *(undefined8 *)(lVar31 + 0x40);
          if (lVar37 == *(longlong *)(lVar31 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                      ();
          }
          lVar54 = *(longlong *)(lVar31 + 0x28);
          *(undefined8 *)(lVar54 + lVar37 * 0x10) = uVar23;
          *(undefined1 *)(lVar54 + 8 + lVar37 * 0x10) = 0;
          *(longlong *)(lVar31 + 0x30) = lVar37 + 1;
          *(longlong *)(lVar31 + 0x40) = *(longlong *)(lVar31 + 0x40) + 1;
          uVar4 = *(undefined4 *)(pauVar51[uVar53 * -3 + -1] + 8);
          *puVar27 = uVar8;
          *puVar26 = uVar4;
          uVar4 = *(undefined4 *)pauVar51[uVar53 * -3 + -1];
          *puVar27 = uVar8;
          puVar26[1] = uVar4;
          if ((*pbVar20 & 1) == 0) {
            local_220 = 0;
            local_228 = 0;
            uStack_224 = 0;
          }
          else {
            local_220 = *(undefined4 *)(pbVar20 + 0xc);
            local_228 = (undefined4)*(undefined8 *)(pbVar20 + 4);
            uStack_224 = (undefined4)((ulonglong)*(undefined8 *)(pbVar20 + 4) >> 0x20);
          }
          *puVar27 = uVar8;
          puVar26[4] = local_220;
          *plVar1 = CONCAT44(uStack_224,local_228);
          *puVar28 = uVar9;
          *(undefined1 *)(lVar45 + 0x1e0) = 1;
          *pfVar22 = 0.0;
          break;
        case 2:
          *puVar34 = uVar12;
          lVar37 = *(longlong *)(lVar33 + 0x30);
          uVar23 = *(undefined8 *)(lVar33 + 0x40);
          if (lVar37 == *(longlong *)(lVar33 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                      ();
          }
          *(undefined8 *)(*(longlong *)(lVar33 + 0x28) + lVar37 * 8) = uVar23;
          *(longlong *)(lVar33 + 0x40) = *(longlong *)(lVar33 + 0x40) + 1;
          *(longlong *)(lVar33 + 0x30) = lVar37 + 1;
          if (0.0 < SQRT(fVar78 * fVar78 + fVar76 * fVar76)) {
            if (fVar81 <= fVar80) {
              uVar4 = *(undefined4 *)*pauVar49;
              uVar5 = puVar26[4];
              lVar37 = *plVar1;
              local_220 = (undefined4)((ulonglong)lVar37 >> 0x20);
              *puVar36 = uVar13;
              lVar54 = *(longlong *)(lVar35 + 0x30);
              uVar23 = *(undefined8 *)(lVar35 + 0x40);
              if (lVar54 == *(longlong *)(lVar35 + 0x20)) {
                alloc::raw_vec::RawVec::
                grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                          ();
              }
              lVar59 = *(longlong *)(lVar35 + 0x28);
              *(undefined4 *)(lVar59 + lVar54 * 0x28) = 2;
              *(float *)(lVar59 + 4 + lVar54 * 0x28) = fVar77;
              *(undefined4 *)(lVar59 + 8 + lVar54 * 0x28) = 0;
              plVar3 = (longlong *)(lVar59 + 0xc + lVar54 * 0x28);
              *plVar3 = lVar37 << 0x20;
              plVar3[1] = CONCAT44(uVar5,local_220);
              *(undefined4 *)(lVar59 + 0x1c + lVar54 * 0x28) = uVar4;
              *(undefined8 *)(lVar59 + 0x20 + lVar54 * 0x28) = uVar23;
              *(longlong *)(lVar35 + 0x30) = lVar54 + 1;
              *(longlong *)(lVar35 + 0x40) = *(longlong *)(lVar35 + 0x40) + 1;
              fVar67 = *pfVar22 + fVar79;
              *pfVar22 = fVar67;
              if (fVar41 < fVar67) {
                *puVar21 = uVar6;
                lVar37 = *(longlong *)(lVar18 + 0x30);
                uVar23 = *(undefined8 *)(lVar18 + 0x40);
                if (lVar37 == *(longlong *)(lVar18 + 0x20)) {
                  alloc::raw_vec::RawVec::
                  grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                            ();
                }
                lVar54 = *(longlong *)(lVar18 + 0x28);
                *(undefined8 *)(lVar54 + lVar37 * 0x10) = uVar23;
                *(undefined1 *)(lVar54 + 8 + lVar37 * 0x10) = 1;
                *(longlong *)(lVar18 + 0x30) = lVar37 + 1;
                *(longlong *)(lVar18 + 0x40) = *(longlong *)(lVar18 + 0x40) + 1;
              }
            }
            else {
              iVar14 = *(int *)*pauVar49;
              if (*(longlong *)(lVar62 + 0x20) != 0) {
                core::hash::BuildHasher::
                hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>
                          ();
                lVar37 = *(longlong *)(lVar62 + 8);
                bVar43 = (byte)(extraout_RAX_00 >> 0x39);
                auVar70 = ZEXT216(CONCAT11(bVar43,bVar43));
                auVar70 = pshuflw(auVar70,auVar70,0);
                lVar54 = 0;
                uVar53 = extraout_RAX_00;
                while( true ) {
                  uVar53 = *(ulonglong *)(lVar62 + 0x10) & uVar53;
                  pcVar2 = (char *)(lVar37 + uVar53);
                  cVar66 = auVar70[0];
                  auVar75[0] = -(*pcVar2 == cVar66);
                  cVar71 = auVar70[1];
                  auVar75[1] = -(pcVar2[1] == cVar71);
                  cVar72 = auVar70[2];
                  auVar75[2] = -(pcVar2[2] == cVar72);
                  cVar73 = auVar70[3];
                  auVar75[3] = -(pcVar2[3] == cVar73);
                  auVar75[4] = -(pcVar2[4] == cVar66);
                  auVar75[5] = -(pcVar2[5] == cVar71);
                  auVar75[6] = -(pcVar2[6] == cVar72);
                  auVar75[7] = -(pcVar2[7] == cVar73);
                  auVar75[8] = -(pcVar2[8] == cVar66);
                  auVar75[9] = -(pcVar2[9] == cVar71);
                  auVar75[10] = -(pcVar2[10] == cVar72);
                  auVar75[0xb] = -(pcVar2[0xb] == cVar73);
                  auVar75[0xc] = -(pcVar2[0xc] == cVar66);
                  auVar75[0xd] = -(pcVar2[0xd] == cVar71);
                  auVar75[0xe] = -(pcVar2[0xe] == cVar72);
                  auVar75[0xf] = -(pcVar2[0xf] == cVar73);
                  uVar56 = (ushort)(SUB161(auVar75 >> 7,0) & 1) |
                           (ushort)(SUB161(auVar75 >> 0xf,0) & 1) << 1 |
                           (ushort)(SUB161(auVar75 >> 0x17,0) & 1) << 2 |
                           (ushort)(SUB161(auVar75 >> 0x1f,0) & 1) << 3 |
                           (ushort)(SUB161(auVar75 >> 0x27,0) & 1) << 4 |
                           (ushort)(SUB161(auVar75 >> 0x2f,0) & 1) << 5 |
                           (ushort)(SUB161(auVar75 >> 0x37,0) & 1) << 6 |
                           (ushort)(SUB161(auVar75 >> 0x3f,0) & 1) << 7 |
                           (ushort)(SUB161(auVar75 >> 0x47,0) & 1) << 8 |
                           (ushort)(SUB161(auVar75 >> 0x4f,0) & 1) << 9 |
                           (ushort)(SUB161(auVar75 >> 0x57,0) & 1) << 10 |
                           (ushort)(SUB161(auVar75 >> 0x5f,0) & 1) << 0xb |
                           (ushort)(SUB161(auVar75 >> 0x67,0) & 1) << 0xc |
                           (ushort)(SUB161(auVar75 >> 0x6f,0) & 1) << 0xd |
                           (ushort)(SUB161(auVar75 >> 0x77,0) & 1) << 0xe |
                           (ushort)(auVar75[0xf] >> 7) << 0xf;
                  uVar61 = (uint)uVar56;
                  while (uVar56 != 0) {
                    uVar38 = 0;
                    for (uVar39 = uVar61; (uVar39 & 1) == 0; uVar39 = uVar39 >> 1 | 0x80000000) {
                      uVar38 = uVar38 + 1;
                    }
                    lVar59 = (uVar38 + uVar53 & *(ulonglong *)(lVar62 + 0x10)) * -0x30;
                    if (*(int *)(lVar37 + -0x30 + lVar59) == iVar14) {
                      fVar67 = *(float *)(lVar37 + -8 + lVar59);
                      uVar4 = *(undefined4 *)*pauVar49;
                      uVar5 = puVar26[4];
                      lVar37 = *plVar1;
                      *puVar36 = uVar13;
                      lVar54 = *(longlong *)(lVar35 + 0x30);
                      uVar23 = *(undefined8 *)(lVar35 + 0x40);
                      if (lVar54 == *(longlong *)(lVar35 + 0x20)) {
                        alloc::raw_vec::RawVec::
                        grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                                  ();
                      }
                      fVar68 = fVar67 / fVar42;
                      fVar69 = fVar68 * fVar68 - fVar82;
                      fVar68 = 0.0;
                      if (0.0 <= fVar69) {
                        fVar68 = fVar69;
                      }
                      fVar69 = __real_3f800000;
                      if (fVar68 <= __real_3f800000) {
                        fVar69 = fVar68;
                      }
                      lVar59 = *(longlong *)(lVar35 + 0x28);
                      *(undefined4 *)(lVar59 + lVar54 * 0x28) = 0;
                      *(undefined4 *)(lVar59 + 8 + lVar54 * 0x28) = 2;
                      *(float *)(lVar59 + 0xc + lVar54 * 0x28) = SQRT(fVar69) * fVar42 - fVar67;
                      *(longlong *)(lVar59 + 0x10 + lVar54 * 0x28) = lVar37;
                      *(undefined4 *)(lVar59 + 0x18 + lVar54 * 0x28) = uVar5;
                      *(undefined4 *)(lVar59 + 0x1c + lVar54 * 0x28) = uVar4;
                      *(undefined8 *)(lVar59 + 0x20 + lVar54 * 0x28) = uVar23;
                      *(longlong *)(lVar35 + 0x30) = lVar54 + 1;
                      *(longlong *)(lVar35 + 0x40) = *(longlong *)(lVar35 + 0x40) + 1;
                      goto override_jmp_140aef9c3_case_0;
                    }
                    uVar56 = (ushort)(uVar61 - 1) & (ushort)uVar61;
                    uVar61 = CONCAT22((short)(uVar61 - 1 >> 0x10),uVar56);
                  }
                  auVar74[0] = -(*pcVar2 == __xmm_ffffffffffffffffffffffffffffffff);
                  auVar74[1] = -(pcVar2[1] == UNK_142925991);
                  auVar74[2] = -(pcVar2[2] == UNK_142925992);
                  auVar74[3] = -(pcVar2[3] == UNK_142925993);
                  auVar74[4] = -(pcVar2[4] == UNK_142925994);
                  auVar74[5] = -(pcVar2[5] == UNK_142925995);
                  auVar74[6] = -(pcVar2[6] == UNK_142925996);
                  auVar74[7] = -(pcVar2[7] == UNK_142925997);
                  auVar74[8] = -(pcVar2[8] == UNK_142925998);
                  auVar74[9] = -(pcVar2[9] == UNK_142925999);
                  auVar74[10] = -(pcVar2[10] == UNK_14292599a);
                  auVar74[0xb] = -(pcVar2[0xb] == UNK_14292599b);
                  auVar74[0xc] = -(pcVar2[0xc] == UNK_14292599c);
                  auVar74[0xd] = -(pcVar2[0xd] == UNK_14292599d);
                  auVar74[0xe] = -(pcVar2[0xe] == UNK_14292599e);
                  auVar74[0xf] = -(pcVar2[0xf] == UNK_14292599f);
                  if ((((((((((((((((SUB161(auVar74 >> 7,0) & 1) != 0 ||
                                   (SUB161(auVar74 >> 0xf,0) & 1) != 0) ||
                                  (SUB161(auVar74 >> 0x17,0) & 1) != 0) ||
                                 (SUB161(auVar74 >> 0x1f,0) & 1) != 0) ||
                                (SUB161(auVar74 >> 0x27,0) & 1) != 0) ||
                               (SUB161(auVar74 >> 0x2f,0) & 1) != 0) ||
                              (SUB161(auVar74 >> 0x37,0) & 1) != 0) ||
                             (SUB161(auVar74 >> 0x3f,0) & 1) != 0) ||
                            (SUB161(auVar74 >> 0x47,0) & 1) != 0) ||
                           (SUB161(auVar74 >> 0x4f,0) & 1) != 0) ||
                          (SUB161(auVar74 >> 0x57,0) & 1) != 0) ||
                         (SUB161(auVar74 >> 0x5f,0) & 1) != 0) ||
                        (SUB161(auVar74 >> 0x67,0) & 1) != 0) ||
                       (SUB161(auVar74 >> 0x6f,0) & 1) != 0) || (SUB161(auVar74 >> 0x77,0) & 1) != 0
                      ) || auVar74[0xf] < '\0') break;
                  uVar53 = uVar53 + lVar54 + 0x10;
                  lVar54 = lVar54 + 0x10;
                }
              }
              if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
                pcStack_f8 = resources::terrain_editing::impl$11::fmt;
                local_100 = pauVar49;
                toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
                ::log::__private_api::impl$2::log();
              }
            }
          }
          break;
        case 3:
          *puVar32 = uVar11;
          lVar37 = *(longlong *)(lVar31 + 0x30);
          uVar23 = *(undefined8 *)(lVar31 + 0x40);
          if (lVar37 == *(longlong *)(lVar31 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                      ();
          }
          lVar54 = *(longlong *)(lVar31 + 0x28);
          *(undefined8 *)(lVar54 + lVar37 * 0x10) = uVar23;
          *(undefined1 *)(lVar54 + 8 + lVar37 * 0x10) = 1;
          *(longlong *)(lVar31 + 0x30) = lVar37 + 1;
          *(longlong *)(lVar31 + 0x40) = *(longlong *)(lVar31 + 0x40) + 1;
          *puVar21 = uVar6;
          lVar37 = *(longlong *)(lVar18 + 0x30);
          uVar23 = *(undefined8 *)(lVar18 + 0x40);
          if (lVar37 == *(longlong *)(lVar18 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                      ();
          }
          lVar54 = *(longlong *)(lVar18 + 0x28);
          *(undefined8 *)(lVar54 + lVar37 * 0x10) = uVar23;
          *(undefined1 *)(lVar54 + 8 + lVar37 * 0x10) = 0;
          *(longlong *)(lVar18 + 0x30) = lVar37 + 1;
          *(longlong *)(lVar18 + 0x40) = *(longlong *)(lVar18 + 0x40) + 1;
          *puVar25 = uVar7;
          uVar15 = *(undefined4 *)pauVar51[uVar53 * -3 + -3];
          uVar16 = *puVar26;
          uVar4 = puVar26[1];
          uVar17 = *(undefined4 *)(pauVar51[uVar53 * -3 + -1] + 8);
          uVar5 = *(undefined4 *)pauVar51[uVar53 * -3 + -1];
          puVar40 = (undefined1 *)*puVar24;
          *puVar24 = 0x8000000000000001;
          *(undefined4 *)(puVar24 + 1) = uVar15;
          *(undefined4 *)((longlong)puVar24 + 0xc) = uVar16;
          *(undefined4 *)(puVar24 + 2) = uVar17;
          *(undefined4 *)((longlong)puVar24 + 0x14) = uVar4;
          *(undefined4 *)(puVar24 + 3) = uVar5;
          if (puVar40 != &DAT_8000000000000002) {
            core::ptr::
            drop_in_place<enum2_<core::option::Option<enum2_<country_core::resources::history::terrain_edit::TerrainEdit>_>_>_>
                      ();
                    /* WARNING: Subroutine does not return */
            core::panicking::panic();
          }
        }
override_jmp_140aef9c3_case_0:
        lVar48 = lVar48 + -1;
      } while (lVar48 != 0);
    }
  }
  return;
}

