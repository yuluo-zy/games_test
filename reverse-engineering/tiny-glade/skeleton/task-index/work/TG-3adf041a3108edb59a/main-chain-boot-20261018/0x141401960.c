
// WARNING: Instruction at (ram,0x000141402746) overlaps instruction at (ram,0x000141402745)
// 
// WARNING: Control flow encountered bad instruction data

void __cdecl
country_core::resources::history::wall_compound_edit::_::impl$0::
deserialize<enum2_<serde_json::value::Value>_>(void)

{
  ulonglong *puVar1;
  undefined8 *puVar2;
  longlong lVar3;
  undefined8 uVar4;
  longlong lVar5;
  uint uVar6;
  int iVar7;
  undefined8 extraout_RAX;
  undefined8 extraout_RAX_00;
  ulonglong uVar8;
  int *piVar9;
  undefined8 extraout_RAX_01;
  undefined8 extraout_RAX_02;
  undefined1 uVar10;
  undefined8 *in_RCX;
  undefined *puVar11;
  byte *in_RDX;
  int **_Buf1;
  int *piVar12;
  int *piVar13;
  int *piVar14;
  int *in_R8;
  byte *in_R9;
  byte *pbVar15;
  byte bVar16;
  ulonglong in_MM1;
  undefined1 auVar17 [16];
  undefined1 auVar18 [16];
  undefined1 auVar19 [16];
  undefined1 auVar20 [16];
  undefined1 auVar21 [16];
  undefined1 auVar22 [16];
  undefined1 auVar23 [16];
  undefined1 auStack_178 [40];
  longlong local_150;
  longlong local_140;
  undefined8 local_138;
  undefined7 uStack_130;
  byte bStack_129;
  undefined7 uStack_128;
  longlong lStack_121;
  ulonglong local_118;
  undefined8 uStack_110;
  undefined8 local_108;
  byte bStack_100;
  undefined7 uStack_ff;
  longlong local_f8;
  undefined8 local_f0;
  undefined8 uStack_e8;
  int *local_d8;
  byte *pbStack_d0;
  int *local_c8;
  byte bStack_c0;
  undefined7 uStack_bf;
  undefined1 uStack_b8;
  undefined7 uStack_b7;
  byte bStack_b0;
  undefined7 uStack_af;
  int *piStack_a8;
  byte *local_a0;
  int *local_98;
  byte *local_88;
  byte *local_80;
  int *local_78;
  int *local_70;
  byte local_68;
  char cStack_67;
  char cStack_66;
  char cStack_65;
  char cStack_64;
  char cStack_63;
  char cStack_62;
  char cStack_61;
  char cStack_60;
  char cStack_5f;
  char cStack_5e;
  char cStack_5d;
  char cStack_5c;
  char cStack_5b;
  char cStack_5a;
  char cStack_59;
  byte local_58;
  undefined7 uStack_57;
  longlong lStack_50;
  undefined1 local_41;
  undefined8 local_40;
  
  local_40 = 0xfffffffffffffffe;
  bVar16 = *in_RDX;
  local_78 = (int *)CONCAT44(local_78._4_4_,(uint)bVar16);
  if (bVar16 != 3) {
    if (bVar16 == 5) {
      piVar12 = *(int **)(in_RDX + 8);
      pbVar15 = *(byte **)(in_RDX + 0x10);
      piVar14 = *(int **)(in_RDX + 0x18);
      uStack_b8 = piVar12 != (int *)0x0;
      local_d8 = (int *)(ulonglong)(byte)uStack_b8;
      local_98 = piVar14;
      if (!(bool)uStack_b8) {
        local_98 = (int *)0x0;
      }
      pbStack_d0 = (byte *)0x0;
      bStack_c0 = (byte)pbVar15;
      uStack_bf = (undefined7)((ulonglong)pbVar15 >> 8);
      uStack_b7 = 0;
      bStack_b0 = 0;
      uStack_af = 0;
      local_c8 = piVar12;
      piStack_a8 = piVar12;
      local_a0 = pbVar15;
      alloc::collections::btree::map::IntoIter::
      dying_next<alloc::string::String,enum2_<serde_json::value::Value>,alloc::alloc::Global>();
      lVar3 = CONCAT17(cStack_61,
                       CONCAT16(cStack_62,
                                CONCAT15(cStack_63,
                                         CONCAT14(cStack_64,
                                                  CONCAT13(cStack_65,
                                                           CONCAT12(cStack_66,
                                                                    CONCAT11(cStack_67,local_68)))))
                               ));
      if (lVar3 != 0) {
        lVar5 = CONCAT71(uStack_57,local_58);
        in_R8 = *(int **)(lVar3 + 0x168 + lVar5 * 0x18);
        in_R9 = *(byte **)(lVar3 + 0x170 + lVar5 * 0x18);
        piVar13 = *(int **)(lVar3 + 0x178 + lVar5 * 0x18);
        puVar1 = (ulonglong *)(lVar3 + lVar5 * 0x20);
        local_118 = *puVar1;
        uStack_110 = puVar1[1];
        puVar2 = (undefined8 *)(lVar3 + 0x10 + lVar5 * 0x20);
        local_108 = (int *)*puVar2;
        lVar3 = puVar2[1];
        bStack_100 = (byte)lVar3;
        uStack_ff = (undefined7)((ulonglong)lVar3 >> 8);
        if (!SBORROW8(0,(longlong)in_R8)) {
          local_58 = (byte)local_108;
          uStack_57 = (undefined7)((ulonglong)local_108 >> 8);
          local_68 = (byte)local_118;
          cStack_67 = (char)(local_118 >> 8);
          cStack_66 = (char)(local_118 >> 0x10);
          cStack_65 = (char)(local_118 >> 0x18);
          cStack_64 = (char)(local_118 >> 0x20);
          cStack_63 = (char)(local_118 >> 0x28);
          cStack_62 = (char)(local_118 >> 0x30);
          cStack_61 = (char)(local_118 >> 0x38);
          cStack_60 = (char)uStack_110;
          cStack_5f = (char)(uStack_110 >> 8);
          cStack_5e = (char)(uStack_110 >> 0x10);
          cStack_5d = (char)(uStack_110 >> 0x18);
          cStack_5c = (char)(uStack_110 >> 0x20);
          cStack_5b = (char)(uStack_110 >> 0x28);
          cStack_5a = (char)(uStack_110 >> 0x30);
          cStack_59 = (char)(uStack_110 >> 0x38);
          local_88 = in_R9;
          local_70 = in_R8;
          lStack_50 = lVar3;
          alloc::collections::btree::map::IntoIter::
          dying_next<alloc::string::String,enum2_<serde_json::value::Value>,alloc::alloc::Global>();
          bVar16 = local_68;
          if (local_150 != 0) {
            in_R8 = *(int **)(local_150 + 0x178 + local_140 * 0x18);
            puVar1 = (ulonglong *)(local_150 + 0x168 + local_140 * 0x18);
            local_118 = *puVar1;
            uStack_110 = puVar1[1];
            puVar2 = (undefined8 *)(local_150 + local_140 * 0x20);
            uVar4 = *puVar2;
            local_f8 = puVar2[1];
            puVar2 = (undefined8 *)(local_150 + 0x10 + local_140 * 0x20);
            local_f0 = *puVar2;
            uStack_e8 = puVar2[1];
            bStack_100 = (byte)uVar4;
            uStack_ff = (undefined7)((ulonglong)uVar4 >> 8);
            local_108 = in_R8;
            if (!SBORROW8(0,local_118)) {
              if (local_118 != 0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
              if (2 < bStack_100) {
                if (bStack_100 == 3) {
                  if (local_f8 != 0) {
                    std::alloc::__default_lib_allocator::__rust_dealloc();
                  }
                }
                else if (bStack_100 == 4) {
                  core::ptr::drop_in_place<serde_json::value::ser::SerializeVec>();
                }
                else {
                  core::ptr::
                  drop_in_place<serde_json::map::Map<alloc::string::String,enum2_<serde_json::value::Value>_>_>
                            ();
                }
              }
              local_118 = CONCAT71(local_118._1_7_,0xb);
              serde::de::Error::invalid_value<serde_json::error::Error>();
              in_RCX[1] = extraout_RAX_01;
              *in_RCX = &DAT_8000000000000010;
              if (2 < local_68) {
                if (local_68 == 3) {
                  if (CONCAT17(cStack_59,
                               CONCAT16(cStack_5a,
                                        CONCAT15(cStack_5b,
                                                 CONCAT14(cStack_5c,
                                                          CONCAT13(cStack_5d,
                                                                   CONCAT12(cStack_5e,
                                                                            CONCAT11(cStack_5f,
                                                                                     cStack_60))))))
                              ) != 0) {
                    std::alloc::__default_lib_allocator::__rust_dealloc();
                  }
                }
                else if (local_68 == 4) {
                  core::ptr::drop_in_place<serde_json::value::ser::SerializeVec>();
                }
                else {
                  core::ptr::
                  drop_in_place<serde_json::map::Map<alloc::string::String,enum2_<serde_json::value::Value>_>_>
                            ();
                }
              }
              if (local_70 != (int *)0x0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
              goto LAB_141401a7f;
            }
          }
          local_138 = CONCAT17(cStack_60,
                               CONCAT16(cStack_61,
                                        CONCAT15(cStack_62,
                                                 CONCAT14(cStack_63,
                                                          CONCAT13(cStack_64,
                                                                   CONCAT12(cStack_65,
                                                                            CONCAT11(cStack_66,
                                                                                     cStack_67))))))
                              );
          uStack_130 = CONCAT16(cStack_59,
                                CONCAT15(cStack_5a,
                                         CONCAT14(cStack_5b,
                                                  CONCAT13(cStack_5c,
                                                           CONCAT12(cStack_5d,
                                                                    CONCAT11(cStack_5e,cStack_5f))))
                                        ));
          bStack_129 = local_58;
          uStack_128 = uStack_57;
          lStack_121 = lStack_50;
          local_41 = 1;
          core::ptr::drop_in_place<serde_json::map::IntoIter>();
          local_d8 = local_70;
          pbStack_d0 = local_88;
          goto LAB_141401aac;
        }
      }
      local_68 = 0xb;
      serde::de::Error::invalid_value<serde_json::error::Error>();
      in_RCX[1] = extraout_RAX;
      *in_RCX = &DAT_8000000000000010;
LAB_141401a7f:
      local_41 = 1;
      core::ptr::drop_in_place<serde_json::map::IntoIter>();
      return;
    }
    local_d8 = *(int **)in_RDX;
    pbStack_d0 = *(byte **)(in_RDX + 8);
    local_c8 = *(int **)(in_RDX + 0x10);
    bStack_c0 = (byte)*(undefined8 *)(in_RDX + 0x18);
    uStack_bf = (undefined7)((ulonglong)*(undefined8 *)(in_RDX + 0x18) >> 8);
    local_80 = in_RDX;
    serde_json::value::Value::unexpected();
    serde_json::error::impl$8::invalid_type();
    in_RCX[1] = extraout_RAX_00;
    *in_RCX = &DAT_8000000000000010;
    if ((byte)local_d8 < 3) {
      return;
    }
    if ((byte)local_d8 != 3) {
      if ((byte)local_d8 != 4) {
        local_41 = 0;
        core::ptr::
        drop_in_place<serde_json::map::Map<alloc::string::String,enum2_<serde_json::value::Value>_>_>
                  ();
        return;
      }
      local_70 = local_c8;
      core::ptr::drop_in_place<slice2_<enum2_<serde_json::value::Value>_>_>();
    }
    if (pbStack_d0 == (byte *)0x0) {
      return;
    }
    goto LAB_141401d09;
  }
  local_d8 = *(int **)(in_RDX + 8);
  pbStack_d0 = *(byte **)(in_RDX + 0x10);
  piVar13 = *(int **)(in_RDX + 0x18);
  bVar16 = 6;
  piVar14 = piVar13;
  pbVar15 = pbStack_d0;
  piVar12 = local_d8;
LAB_141401aac:
  uStack_bf = (undefined7)local_138;
  uStack_b8 = (undefined1)((ulonglong)local_138 >> 0x38);
  uStack_b7 = uStack_130;
  bStack_b0 = bStack_129;
  uStack_af = uStack_128;
  piStack_a8 = (int *)lStack_121;
  local_41 = 1;
  _Buf1 = &local_d8;
  local_c8 = piVar13;
  bStack_c0 = bVar16;
  serde_json::value::de::impl$3::
  variant_seed<core::marker::PhantomData<country_core::resources::history::wall_compound_edit::_::impl_0::deserialize::__Field>_>
            ();
  if ((byte)uStack_110 == 7) {
    in_RCX[1] = local_118;
    *in_RCX = &DAT_8000000000000010;
    goto code_r0x000141401b02;
  }
  auVar21._1_7_ = uStack_ff;
  auVar21[0] = bStack_100;
  auVar21._8_8_ = local_f8;
  uStack_57 = uStack_ff;
  lStack_50 = local_f8;
  cStack_67 = uStack_110._1_1_;
  cStack_66 = uStack_110._2_1_;
  cStack_65 = uStack_110._3_1_;
  cStack_64 = uStack_110._4_1_;
  cStack_63 = uStack_110._5_1_;
  cStack_62 = uStack_110._6_1_;
  cStack_61 = uStack_110._7_1_;
  cStack_60 = (char)local_108;
  cStack_5f = local_108._1_1_;
  cStack_5e = local_108._2_1_;
  cStack_5d = local_108._3_1_;
  cStack_5c = local_108._4_1_;
  cStack_5b = local_108._5_1_;
  cStack_5a = local_108._6_1_;
  cStack_59 = local_108._7_1_;
  local_58 = bStack_100;
  local_68 = (byte)uStack_110;
  puVar11 = &DAT_142c5bb20;
  uVar8 = (ulonglong)*(int *)(&DAT_142c5bb20 + (local_118 & 0xff) * 4);
  piVar9 = (int *)(&DAT_142c5bb20 + uVar8);
  uVar6 = (uint)piVar9;
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
  switch(local_118 & 0xff) {
  case 0:
    local_41 = 1;
    serde_json::value::de::impl$5::
    struct_variant<country_core::resources::history::wall_compound_edit::_::impl_0::deserialize::impl_2::visit_enum::__Visitor>
              ();
    break;
  case 1:
    local_41 = 1;
    serde_json::value::de::impl$5::
    struct_variant<country_core::resources::history::wall_compound_edit::_::impl_0::deserialize::impl_2::visit_enum::__Visitor>
              ();
    break;
  case 2:
    local_41 = 1;
    serde_json::value::de::impl$5::
    struct_variant<country_core::resources::history::wall_compound_edit::_::impl_0::deserialize::impl_2::visit_enum::__Visitor>
              ();
    break;
  case 3:
    local_41 = 1;
    serde_json::value::de::impl$5::
    struct_variant<country_core::resources::history::wall_compound_edit::_::impl_0::deserialize::impl_2::visit_enum::__Visitor>
              ();
    break;
  case 4:
    local_41 = 1;
    serde_json::value::de::impl$5::
    struct_variant<country_core::resources::history::wall_compound_edit::_::impl_0::deserialize::impl_2::visit_enum::__Visitor>
              ();
    break;
  case 5:
    local_41 = 1;
    serde_json::value::de::impl$5::
    struct_variant<country_core::resources::history::wall_compound_edit::_::impl_0::deserialize::impl_2::visit_enum::__Visitor>
              ();
    break;
  case 6:
    local_41 = 1;
    serde_json::value::de::impl$5::
    struct_variant<country_core::resources::history::wall_compound_edit::_::impl_0::deserialize::impl_2::visit_enum::__Visitor>
              ();
    break;
  case 7:
    local_41 = 1;
    serde_json::value::de::impl$5::
    struct_variant<country_core::resources::history::wall_compound_edit::_::impl_0::deserialize::impl_2::visit_enum::__Visitor>
              ();
    break;
  case 8:
    local_41 = 1;
    serde_json::value::de::impl$5::
    struct_variant<country_core::resources::history::wall_compound_edit::_::impl_0::deserialize::impl_2::visit_enum::__Visitor>
              ();
    break;
  case 9:
    local_41 = 1;
    serde_json::value::de::impl$5::
    struct_variant<country_core::resources::history::wall_compound_edit::_::impl_0::deserialize::impl_2::visit_enum::__Visitor>
              ();
    break;
  case 10:
    local_41 = 1;
    serde_json::value::de::impl$5::
    struct_variant<country_core::resources::history::wall_compound_edit::_::impl_0::deserialize::impl_2::visit_enum::__Visitor>
              ();
    break;
  case 0xb:
    local_41 = 1;
    serde_json::value::de::impl$5::
    struct_variant<country_core::resources::history::wall_compound_edit::_::impl_0::deserialize::impl_2::visit_enum::__Visitor>
              ();
    break;
  case 0xc:
    local_41 = 1;
    serde_json::value::de::impl$5::
    struct_variant<country_core::resources::history::wall_compound_edit::_::impl_0::deserialize::impl_2::visit_enum::__Visitor>
              ();
    break;
  case 0xd:
    local_41 = 1;
    serde_json::value::de::impl$5::
    struct_variant<country_core::resources::history::wall_compound_edit::_::impl_0::deserialize::impl_2::visit_enum::__Visitor>
              ();
    break;
  case 0xe:
    local_41 = 1;
    serde_json::value::de::impl$5::
    struct_variant<country_core::resources::history::wall_compound_edit::_::impl_0::deserialize::impl_2::visit_enum::__Visitor>
              ();
    break;
  case 0xf:
    local_41 = 1;
    serde_json::value::de::impl$5::
    struct_variant<country_core::resources::history::wall_compound_edit::_::impl_0::deserialize::impl_2::visit_enum::__Visitor>
              ();
    break;
  case 0x10:
    local_41 = 1;
    serde_json::value::de::impl$5::
    struct_variant<country_core::resources::history::wall_compound_edit::_::impl_0::deserialize::impl_2::visit_enum::__Visitor>
              ();
    break;
  case 0x11:
    (**(code **)(in_R8 + 0x83120d2))();
    return;
  default:
code_r0x0001414025d1:
    if ((ushort)((ushort)(SUB161(auVar21 >> 7,0) & 1) | (ushort)(SUB161(auVar21 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar21 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar21 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar21 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar21 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar21 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar21 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar21 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar21 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar21 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar21 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar21 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar21 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar21 >> 0x77,0) & 1) << 0xe |
                (ushort)(byte)(auVar21[0xf] >> 7) << 0xf) != 0xffff) goto code_r0x000141402628;
    (&UNK_142c5bb21)[uVar8] = 0xd;
    goto code_r0x000141402666;
  case 0x16:
    goto code_r0x000141402628;
  case 0x17:
    if ((*(ulonglong *)((longlong)_Buf1 + 7) ^ 0x6e6f69746176656c | 0x142c5bb20) == 0) {
      (&UNK_142c5bb21)[uVar8] = 6;
    }
    else {
      if (*(longlong *)((longlong)_Buf1 + 7) != 0x6870726f4d646574 ||
          *_Buf1 != (int *)0x7461636572706544) goto code_r0x000141402628;
      (&UNK_142c5bb21)[uVar8] = 0xe;
    }
    goto code_r0x000141402666;
  case 0x18:
    piVar9 = (int *)CONCAT71((int7)((ulonglong)piVar9 >> 8),(char)piVar9 + (char)*in_R8);
    *piVar14 = *piVar14 + 0x42c5bb20;
    goto code_r0x000141402666;
  case 0x19:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x1a:
    if ((*(ulonglong *)((longlong)_Buf1 + 3) ^ 0x656c79745365676e | 0x142c5bb20) == 0) {
      (&UNK_142c5bb21)[uVar8] = 9;
    }
    else {
      if (*(longlong *)((longlong)_Buf1 + 3) != 0x7472696b5365676e ||
          *_Buf1 != (int *)0x6b5365676e616843) goto code_r0x000141402628;
      (&UNK_142c5bb21)[uVar8] = 0x10;
    }
    goto code_r0x000141402666;
  case 0x1b:
    *piVar9 = *piVar9 + uVar6;
    *piVar13 = *piVar13 + uVar6;
    goto code_r0x000141402666;
  case 0x1c:
    *(undefined1 *)_Buf1 = *(undefined1 *)_Buf1;
    piVar12 = (int *)CONCAT62((int6)((ulonglong)_Buf1 >> 0x10),
                              CONCAT11((char)((ulonglong)_Buf1 >> 8) + (char)piVar9,(char)_Buf1));
    *piVar12 = *piVar12 + uVar6;
    goto code_r0x000141402666;
  case 0x1d:
    iVar7 = memcmp(_Buf1,piVar9,(size_t)in_R8);
    piVar9 = piVar13;
    if (iVar7 == 0) {
      *(undefined1 *)((longlong)piVar13 + 1) = 8;
    }
    else {
      iVar7 = memcmp((void *)((ulonglong)_Buf1 & 0xffffffff),
                     &anon_27e668faf3910a1f44e0c6601a137ba6_88_llvm_16848830187404718073,
                     (size_t)in_R8);
      if (iVar7 != 0) goto code_r0x000141402628;
      *(undefined1 *)((longlong)piVar13 + 1) = 0xc;
    }
    goto code_r0x000141402666;
  case 0x21:
    if ((byte)((byte)(in_MM1 >> 7) & 1 | ((byte)(in_MM1 >> 0xf) & 1) << 1 |
               ((byte)(in_MM1 >> 0x17) & 1) << 2 | ((byte)(in_MM1 >> 0x1f) & 1) << 3 |
               ((byte)(in_MM1 >> 0x27) & 1) << 4 | ((byte)(in_MM1 >> 0x2f) & 1) << 5 |
               ((byte)(in_MM1 >> 0x37) & 1) << 6 | (byte)(in_MM1 >> 0x38) & 0x80) != 0xffff) {
      auVar17[0] = -(*(char *)_Buf1 == 'M');
      auVar17[1] = -(*(char *)((longlong)_Buf1 + 1) == 'o');
      auVar17[2] = -(*(char *)((longlong)_Buf1 + 2) == 'r');
      auVar17[3] = -(*(char *)((longlong)_Buf1 + 3) == 'p');
      auVar17[4] = -(*(char *)((longlong)_Buf1 + 4) == 'h');
      auVar17[5] = -(*(char *)((longlong)_Buf1 + 5) == 'F');
      auVar17[6] = -(*(char *)((longlong)_Buf1 + 6) == 'r');
      auVar17[7] = -(*(char *)((longlong)_Buf1 + 7) == 'e');
      auVar17[8] = -(*(char *)(_Buf1 + 1) == 'e');
      auVar17[9] = -(*(char *)((longlong)_Buf1 + 9) == 'h');
      auVar17[10] = -(*(char *)((longlong)_Buf1 + 10) == 'a');
      auVar17[0xb] = -(*(char *)((longlong)_Buf1 + 0xb) == 'n');
      auVar17[0xc] = -(*(char *)((longlong)_Buf1 + 0xc) == 'd');
      auVar17[0xd] = -(*(char *)((longlong)_Buf1 + 0xd) == 'W');
      auVar17[0xe] = -(*(char *)((longlong)_Buf1 + 0xe) == 'a');
      auVar17[0xf] = -(*(char *)((longlong)_Buf1 + 0xf) == 'l');
      auVar20[0] = -(*(char *)(_Buf1 + 2) == 'l');
      auVar20[1] = 0xff;
      auVar20[2] = 0xff;
      auVar20[3] = 0xff;
      auVar20[4] = 0xff;
      auVar20[5] = 0xff;
      auVar20[6] = 0xff;
      auVar20[7] = 0xff;
      auVar20[8] = 0xff;
      auVar20[9] = 0xff;
      auVar20[10] = 0xff;
      auVar20[0xb] = 0xff;
      auVar20[0xc] = 0xff;
      auVar20[0xd] = 0xff;
      auVar20[0xe] = 0xff;
      auVar20[0xf] = 0xff;
      auVar21 = auVar20 & auVar17;
      goto code_r0x0001414025d1;
    }
    (&UNK_142c5bb21)[uVar8] = 5;
    goto code_r0x000141402666;
  case 0x23:
                    // WARNING: Could not recover jumptable at 0x0001414022e6. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(in_R9 + 0x142c5bb20))(in_R9 + 0x142c5bb20);
    return;
  case 0x24:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x25:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x26:
  case 0x27:
  case 0x2c:
  case 0x2d:
  case 0x2e:
  case 0x2f:
  case 0x30:
  case 0x31:
                    // WARNING: Read-only address (ram,0x000142c5bb20) is written
    return;
  case 0x28:
    piVar9 = (int *)&DAT_142c5bb20;
    if (&DAT_0000000d < (undefined1 *)((longlong)in_R8 + -5)) goto LAB_141402773;
    puVar11 = &DAT_142c5bbb4;
    in_R8 = (int *)(longlong)*(int *)(&DAT_142c5bbb4 + ((longlong)in_R8 + -5) * 4);
    goto code_r0x00014140269c;
  case 0x29:
code_r0x000141402628:
    serde::de::Error::unknown_variant<serde_json::error::Error>();
    *(undefined8 *)(piVar9 + 2) = extraout_RAX_02;
    uVar10 = 1;
    goto LAB_141402668;
  case 0x2a:
code_r0x00014140269c:
                    // WARNING: Could not recover jumptable at 0x00014140269f. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)((longlong)in_R8 + (longlong)puVar11))();
    return;
  case 0x2b:
    if (0xfffffffebd3a44df < uVar8 || piVar9 == (int *)0x0) {
      if (*_Buf1 == in_R8) {
                    // WARNING (jumptable): Read-only address (ram,0x000142c5bb20) is written
                    // WARNING: Read-only address (ram,0x000142c5bb20) is written
        DAT_142c5bb20 = 4;
        *(undefined1 *)piVar9 = 0;
        return;
      }
                    // WARNING (jumptable): Read-only address (ram,0x000142c5bb20) is written
                    // WARNING: Read-only address (ram,0x000142c5bb20) is written
      DAT_142c5bb20 = 5;
      *(undefined1 *)piVar9 = 0;
      return;
    }
    if (piVar9 != (int *)0x0) {
      out(*(undefined4 *)in_RCX,(short)_Buf1);
      if (*(char *)(_Buf1 + 1) == 'n' && *_Buf1 == (int *)&DAT_142c5bb20) {
        (&UNK_142c5bb21)[uVar8] = 2;
        *(undefined1 *)piVar9 = 0;
        return;
      }
      goto LAB_141402773;
    }
  case 0x35:
    piVar9 = (int *)(ulonglong)(uVar6 + 0xc30000c6);
    if (*(short *)(_Buf1 + 1) == 0x7865 && *_Buf1 == (int *)0x646e695f6c6c6177) {
      *(undefined1 *)((longlong)piVar9 + 1) = 0;
      *(undefined1 *)piVar9 = 0;
      return;
    }
LAB_141402773:
    *(undefined1 *)((longlong)piVar9 + 1) = 5;
    *(undefined1 *)piVar9 = 0;
    return;
  case 0x32:
    *piVar9 = *piVar9 + uVar6;
code_r0x000141402666:
    uVar10 = 0;
LAB_141402668:
    *(undefined1 *)piVar9 = uVar10;
    return;
  case 0x33:
    *(char *)((longlong)in_RCX + 0xf) =
         *(char *)((longlong)in_RCX + 0xf) + (char)auStack_178 + (0xfffffffebd3a44df < uVar8);
    out(*(undefined1 *)in_RCX,(short)_Buf1);
    auVar18[0] = -(uStack_110._1_1_ == 'r');
    auVar18[1] = -(uStack_110._2_1_ == 'e');
    auVar18[2] = -(uStack_110._3_1_ == 'd');
    auVar18[3] = -(uStack_110._4_1_ == 'o');
    auVar18[4] = -(uStack_110._5_1_ == '_');
    auVar18[5] = -(uStack_110._6_1_ == 'i');
    auVar18[6] = -(uStack_110._7_1_ == 'n');
    auVar18[7] = -((char)local_108 == 's');
    auVar18[8] = -(local_108._1_1_ == 't');
    auVar18[9] = -(local_108._2_1_ == 'a');
    auVar18[10] = -(local_108._3_1_ == 'n');
    auVar18[0xb] = -(local_108._4_1_ == 't');
    auVar18[0xc] = -(local_108._5_1_ == '_');
    auVar18[0xd] = -(local_108._6_1_ == 's');
    auVar18[0xe] = -(local_108._7_1_ == 't');
    auVar18[0xf] = -(bStack_100 == 0x61);
    auVar22[0] = -(bStack_100 == 0x74);
    auVar22[1] = -((char)uStack_ff == 'e');
    auVar22[2] = -((char)((uint7)uStack_ff >> 8) == '\0');
    auVar22[3] = -((char)((uint7)uStack_ff >> 0x10) == '\0');
    auVar22[4] = -((char)((uint7)uStack_ff >> 0x18) == '\0');
    auVar22[5] = -((char)((uint7)uStack_ff >> 0x20) == '\0');
    auVar22[6] = -((char)((uint7)uStack_ff >> 0x28) == '\0');
    auVar22[7] = -((char)((uint7)uStack_ff >> 0x30) == '\0');
    auVar22[8] = -((char)local_f8 == '\0');
    auVar22[9] = -((char)((ulonglong)local_f8 >> 8) == '\0');
    auVar22[10] = -((char)((ulonglong)local_f8 >> 0x10) == '\0');
    auVar22[0xb] = -((char)((ulonglong)local_f8 >> 0x18) == '\0');
    auVar22[0xc] = -((char)((ulonglong)local_f8 >> 0x20) == '\0');
    auVar22[0xd] = -((char)((ulonglong)local_f8 >> 0x28) == '\0');
    auVar22[0xe] = -((char)((ulonglong)local_f8 >> 0x30) == '\0');
    auVar22[0xf] = -((char)((ulonglong)local_f8 >> 0x38) == '\0');
    auVar22 = auVar22 & auVar18;
    puVar11 = (undefined *)
              (ulonglong)
              (ushort)((ushort)(SUB161(auVar22 >> 7,0) & 1) |
                       (ushort)(SUB161(auVar22 >> 0xf,0) & 1) << 1 |
                       (ushort)(SUB161(auVar22 >> 0x17,0) & 1) << 2 |
                       (ushort)(SUB161(auVar22 >> 0x1f,0) & 1) << 3 |
                       (ushort)(SUB161(auVar22 >> 0x27,0) & 1) << 4 |
                       (ushort)(SUB161(auVar22 >> 0x2f,0) & 1) << 5 |
                       (ushort)(SUB161(auVar22 >> 0x37,0) & 1) << 6 |
                       (ushort)(SUB161(auVar22 >> 0x3f,0) & 1) << 7 |
                       (ushort)(SUB161(auVar22 >> 0x47,0) & 1) << 8 |
                       (ushort)(SUB161(auVar22 >> 0x4f,0) & 1) << 9 |
                       (ushort)(SUB161(auVar22 >> 0x57,0) & 1) << 10 |
                       (ushort)(SUB161(auVar22 >> 0x5f,0) & 1) << 0xb |
                       (ushort)(SUB161(auVar22 >> 0x67,0) & 1) << 0xc |
                       (ushort)(SUB161(auVar22 >> 0x6f,0) & 1) << 0xd |
                       (ushort)(SUB161(auVar22 >> 0x77,0) & 1) << 0xe |
                      (ushort)(byte)(auVar22[0xf] >> 7) << 0xf);
  case 0x37:
    if ((int)puVar11 == 0xffff) {
      (&UNK_142c5bb21)[uVar8] = 3;
      *(undefined1 *)piVar9 = 0;
      return;
    }
    goto LAB_141402773;
  case 0x34:
  case 0x36:
  case 0x39:
  case 0x3a:
  case 0x3b:
  case 0x3c:
  case 0x3d:
  case 0x3e:
    switch(in_R8) {
    case (int *)0x0:
      if (*(short *)((longlong)_Buf1 + 4) == 0x6572 && *(int *)_Buf1 == 0x6f666562) {
code_r0x00014140286e:
        (&UNK_142c5bb21)[uVar8] = 2;
        *(undefined1 *)piVar9 = 0;
        return;
      }
      break;
    case (int *)0x2:
      if (*_Buf1 == (int *)0x64695f726f6c6f63) {
        (&UNK_142c5bb21)[uVar8] = 3;
        *(undefined1 *)piVar9 = 0;
        return;
      }
      (&UNK_142c5bb21)[uVar8] = 4;
      *(undefined1 *)piVar9 = 0;
      return;
    case (int *)0x4:
      if (*(short *)(_Buf1 + 1) == 0x7865 && *_Buf1 == (int *)0x646e695f6c6c6177) {
        (&UNK_142c5bb21)[uVar8] = 0;
        *(undefined1 *)piVar9 = 0;
        return;
      }
      break;
    case (int *)0x5:
      if (*(longlong *)((longlong)_Buf1 + 3) == 0x7468676965685f6c &&
          *_Buf1 == (int *)0x6965685f6c6c6177) {
        (&UNK_142c5bb21)[uVar8] = 1;
        *(undefined1 *)piVar9 = 0;
        return;
      }
      break;
    case (int *)0xc:
      auVar19[0] = -(*(char *)_Buf1 == 'u');
      auVar19[1] = -(*(char *)((longlong)_Buf1 + 1) == 'n');
      auVar19[2] = -(*(char *)((longlong)_Buf1 + 2) == 'd');
      auVar19[3] = -(*(char *)((longlong)_Buf1 + 3) == 'o');
      auVar19[4] = -(*(char *)((longlong)_Buf1 + 4) == '_');
      auVar19[5] = -(*(char *)((longlong)_Buf1 + 5) == 'i');
      auVar19[6] = -(*(char *)((longlong)_Buf1 + 6) == 'n');
      auVar19[7] = -(*(char *)((longlong)_Buf1 + 7) == 's');
      auVar19[8] = -(*(char *)(_Buf1 + 1) == 't');
      auVar19[9] = -(*(char *)((longlong)_Buf1 + 9) == 'a');
      auVar19[10] = -(*(char *)((longlong)_Buf1 + 10) == 'n');
      auVar19[0xb] = -(*(char *)((longlong)_Buf1 + 0xb) == 't');
      auVar19[0xc] = -(*(char *)((longlong)_Buf1 + 0xc) == '_');
      auVar19[0xd] = -(*(char *)((longlong)_Buf1 + 0xd) == 's');
      auVar19[0xe] = -(*(char *)((longlong)_Buf1 + 0xe) == 't');
      auVar19[0xf] = -(*(char *)((longlong)_Buf1 + 0xf) == 'a');
      auVar23[0] = -((char)*(undefined2 *)(_Buf1 + 2) == 't');
      auVar23[1] = -((char)((ushort)*(undefined2 *)(_Buf1 + 2) >> 8) == 'e');
      auVar23[2] = 0xff;
      auVar23[3] = 0xff;
      auVar23[4] = 0xff;
      auVar23[5] = 0xff;
      auVar23[6] = 0xff;
      auVar23[7] = 0xff;
      auVar23[8] = 0xff;
      auVar23[9] = 0xff;
      auVar23[10] = 0xff;
      auVar23[0xb] = 0xff;
      auVar23[0xc] = 0xff;
      auVar23[0xd] = 0xff;
      auVar23[0xe] = 0xff;
      auVar23[0xf] = 0xff;
      auVar23 = auVar23 & auVar19;
      if ((ushort)((ushort)(SUB161(auVar23 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar23 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar23 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar23 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar23 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar23 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar23 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar23 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar23 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar23 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar23 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar23 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar23 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar23 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar23 >> 0x77,0) & 1) << 0xe |
                  (ushort)(byte)(auVar23[0xf] >> 7) << 0xf) == 0xffff) goto code_r0x00014140286e;
    }
    (&UNK_142c5bb21)[uVar8] = 4;
    *(undefined1 *)piVar9 = 0;
    return;
  case 0x38:
    (&UNK_142c5bb29)[uVar8] = (&UNK_142c5bb29)[uVar8] + ' ';
    return;
  case 0x3f:
    piVar9 = (int *)(ulonglong)(uVar6 ^ *(uint *)((longlong)_Buf1 + 3));
    if (((ulonglong)in_R8 | 0x142c5bb20) == 0) {
      *(undefined1 *)((longlong)piVar9 + 1) = 1;
      *(undefined1 *)piVar9 = 0;
      return;
    }
    goto LAB_141402773;
  }
code_r0x000141401b02:
  if ((((int)local_78 != 3) && ((int)local_78 != 5)) &&
     (local_80 = pbVar15, local_78 = piVar12,
     core::ptr::drop_in_place<slice2_<enum2_<serde_json::value::Value>_>_>(), local_78 != (int *)0x0
     )) {
LAB_141401d09:
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
                    // WARNING: Read-only address (ram,0x000142c5bb20) is written
  return;
}

