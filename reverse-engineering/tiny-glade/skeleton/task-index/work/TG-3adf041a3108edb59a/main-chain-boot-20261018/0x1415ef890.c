
// WARNING: Instruction at (ram,0x0001415f16a4) overlaps instruction at (ram,0x0001415f16a3)
// 
// WARNING: Control flow encountered bad instruction data
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl
serde_json::value::de::impl$5::
struct_variant<country_core::resources::history::wall_compound_edit::_::impl_0::deserialize::impl_2::visit_enum::__Visitor>
          (void)

{
  int *piVar1;
  byte *pbVar2;
  undefined8 *puVar3;
  uint *puVar4;
  uint uVar5;
  byte bVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  longlong lVar10;
  uint uVar11;
  uint uVar12;
  uint uVar13;
  ulonglong uVar14;
  ulonglong extraout_RAX;
  ulonglong extraout_RAX_00;
  longlong lVar15;
  char *pcVar16;
  ulonglong extraout_RAX_01;
  ulonglong extraout_RAX_02;
  ulonglong extraout_RAX_03;
  ulonglong extraout_RAX_04;
  ulonglong extraout_RAX_05;
  ulonglong extraout_RAX_06;
  longlong extraout_RAX_07;
  ulonglong extraout_RAX_08;
  ulonglong extraout_RAX_09;
  ulonglong extraout_RAX_10;
  ulonglong extraout_RAX_11;
  ulonglong extraout_RAX_12;
  ulonglong extraout_RAX_13;
  ulonglong extraout_RAX_14;
  ulonglong extraout_RAX_15;
  ulonglong extraout_RAX_16;
  ulonglong extraout_RAX_17;
  ulonglong extraout_RAX_18;
  ulonglong extraout_RAX_19;
  ulonglong extraout_RAX_20;
  ulonglong extraout_RAX_21;
  ulonglong extraout_RAX_22;
  longlong extraout_RAX_23;
  ulonglong extraout_RAX_24;
  ulonglong extraout_RAX_25;
  ulonglong extraout_RAX_26;
  ulonglong extraout_RAX_27;
  ulonglong uVar17;
  ulonglong extraout_RAX_28;
  ulonglong extraout_RAX_29;
  ulonglong extraout_RAX_30;
  ulonglong extraout_RAX_31;
  ulonglong extraout_RAX_32;
  ulonglong extraout_RAX_33;
  ulonglong extraout_RAX_34;
  ulonglong *in_RCX;
  undefined *puVar18;
  byte *in_RDX;
  undefined *puVar19;
  undefined1 *puVar20;
  undefined1 *puVar21;
  char cVar22;
  uint *puVar24;
  ulonglong uVar25;
  undefined4 unaff_XMM6_Da;
  undefined4 unaff_XMM7_Da;
  undefined1 auStack_3f8 [32];
  undefined8 uStack_3d8;
  ulonglong uStack_3d0;
  ulonglong local_3c8;
  longlong lStack_3c0;
  undefined8 local_3b8;
  longlong lStack_3b0;
  uint local_3a8;
  undefined4 uStack_3a4;
  ulonglong uStack_3a0;
  longlong *local_398;
  ulonglong uStack_390;
  ulonglong local_388;
  ulonglong uStack_380;
  undefined8 local_378;
  undefined8 uStack_370;
  undefined8 local_368;
  undefined8 uStack_360;
  undefined8 local_358;
  longlong lStack_350;
  undefined8 local_348;
  undefined8 uStack_340;
  longlong lStack_338;
  byte local_330;
  undefined7 uStack_32f;
  undefined1 uStack_328;
  undefined7 uStack_327;
  byte local_320;
  undefined7 uStack_31f;
  ulonglong uStack_318;
  ulonglong local_310;
  ulonglong uStack_308;
  undefined8 local_300;
  undefined8 uStack_2f8;
  ulonglong local_2f0;
  undefined8 uStack_2e8;
  ulonglong local_2e0;
  undefined8 local_2d8;
  undefined4 local_2d0;
  uint uStack_2cc;
  int *piStack_2c8;
  uint uStack_2c0;
  undefined1 uStack_2b9;
  longlong lStack_268;
  byte bStack_260;
  undefined7 uStack_25f;
  undefined1 uStack_258;
  undefined7 uStack_257;
  byte bStack_250;
  undefined7 uStack_24f;
  ulonglong uStack_248;
  byte bStack_240;
  undefined1 uStack_23f;
  undefined6 uStack_23e;
  ulonglong local_238;
  undefined *puStack_230;
  undefined8 local_228;
  ulonglong local_220;
  byte *local_218;
  undefined8 local_210;
  uint local_208;
  uint uStack_204;
  uint uStack_200;
  uint uStack_1fc;
  byte local_1f8;
  undefined7 uStack_1f7;
  ulonglong uStack_1f0;
  longlong *local_1e8;
  undefined8 uStack_1e0;
  ulonglong local_1d8;
  ulonglong uStack_1d0;
  undefined8 local_1c8;
  undefined8 uStack_1c0;
  undefined8 local_1b8;
  undefined8 uStack_1b0;
  char local_198;
  byte bStack_197;
  undefined6 uStack_196;
  ulonglong local_190;
  longlong local_188;
  undefined8 local_130;
  undefined7 uStack_128;
  byte bStack_121;
  undefined7 uStack_120;
  ulonglong uStack_119;
  longlong local_f0;
  char local_e8;
  undefined7 uStack_e7;
  undefined1 uStack_e0;
  undefined7 uStack_df;
  byte local_d8;
  undefined7 uStack_d7;
  ulonglong uStack_d0;
  undefined8 uStack_c8;
  undefined4 local_bc;
  ulonglong local_b8;
  ulonglong local_b0;
  undefined8 local_a8;
  longlong local_a0;
  undefined8 local_98;
  uint local_90;
  uint local_8c;
  ulonglong local_88;
  longlong local_80;
  undefined1 local_71;
  undefined8 local_70;
  char cVar23;
  
  puVar20 = auStack_3f8;
  local_70 = 0xfffffffffffffffe;
  bVar6 = *in_RDX;
  local_220 = (ulonglong)bVar6;
  if (bVar6 == 6) {
    local_358 = CONCAT71(local_358._1_7_,0xd);
    local_71 = 0;
    serde_json::error::impl$8::invalid_type();
    in_RCX[1] = extraout_RAX;
    *in_RCX = (ulonglong)&DAT_8000000000000010;
  }
  else {
    if (bVar6 == 5) {
      local_238 = *(ulonglong *)(in_RDX + 8);
      puStack_230 = *(undefined **)(in_RDX + 0x10);
      local_228 = *(undefined8 *)(in_RDX + 0x18);
      local_71 = 0;
      local_218 = in_RDX;
      local_210 = local_228;
      MapDeserializer::new();
      local_3c8 = 0x8000000000000001;
      local_b0 = 0;
      puVar24 = &local_208;
      puVar19 = &DAT_142c7c08c;
      local_80 = -0x7fffffffffffffff;
      local_8c = 0;
      local_90 = 0;
      local_b8 = 0;
LAB_1415ef9c0:
      *(undefined8 *)(puVar20 + -8) = 0x1415ef9cb;
      alloc::collections::btree::map::IntoIter::
      dying_next<alloc::string::String,enum2_<serde_json::value::Value>,alloc::alloc::Global>();
      lVar10 = CONCAT62(uStack_196,CONCAT11(bStack_197,local_198));
      if (lVar10 != 0) {
        local_a0 = *(longlong *)(lVar10 + 0x168 + local_188 * 0x18);
        local_a8 = *(undefined8 *)(lVar10 + 0x170 + local_188 * 0x18);
        uVar17 = *(ulonglong *)(lVar10 + 0x178 + local_188 * 0x18);
        puVar3 = (undefined8 *)(lVar10 + local_188 * 0x20);
        uVar7 = *puVar3;
        uVar8 = puVar3[1];
        puVar3 = (undefined8 *)(lVar10 + 0x10 + local_188 * 0x20);
        uVar9 = *puVar3;
        uStack_1f0 = puVar3[1];
        local_208 = (uint)uVar7;
        uStack_204 = (uint)((ulonglong)uVar7 >> 0x20);
        uStack_200 = (uint)uVar8;
        uStack_1fc = (uint)((ulonglong)uVar8 >> 0x20);
        local_1f8 = (byte)uVar9;
        uStack_1f7 = (undefined7)((ulonglong)uVar9 >> 8);
        if (!SBORROW8(0,local_a0)) {
          switch(local_e8) {
          case '\0':
          case '\x01':
          case '\x02':
          case '\x06':
            goto LAB_1415efa4d;
          case '\x03':
            goto code_r0x0001415efb07;
          case '\x04':
            goto code_r0x0001415efb2e;
          default:
            goto LAB_1415efb73;
          }
        }
      }
      if ((local_b8 & 1) == 0) {
        *(undefined8 *)(puVar20 + -8) = 0x1415f0022;
        serde::de::Error::missing_field<serde_json::error::Error>();
        uVar14 = extraout_RAX_01;
      }
      else {
        if ((local_90 & 1) != 0) {
          if ((local_8c & 1) == 0) {
            unaff_XMM6_Da = 0;
          }
          lStack_350 = -0x8000000000000000;
          if (local_80 != -0x7fffffffffffffff) {
            local_1b8 = local_368;
            uStack_1b0 = uStack_360;
            local_1c8 = local_378;
            uStack_1c0 = uStack_370;
            local_1d8 = local_388;
            uStack_1d0 = uStack_380;
            local_1e8 = local_398;
            uStack_1e0 = uStack_390;
            local_1f8 = (byte)local_3a8;
            uStack_1f7 = (undefined7)(CONCAT44(uStack_3a4,local_3a8) >> 8);
            uStack_1f0 = uStack_3a0;
            local_208 = (uint)local_3b8;
            uStack_204 = (uint)((ulonglong)local_3b8 >> 0x20);
            uStack_200 = (uint)lStack_3b0;
            uStack_1fc = (uint)((ulonglong)lStack_3b0 >> 0x20);
            lStack_350 = local_80;
          }
          local_2d0 = 0;
          if ((local_b0 & 1) != 0) {
            local_2d0 = local_bc;
          }
          local_348 = local_98;
          uStack_340 = CONCAT44(uStack_204,local_208);
          lStack_338 = CONCAT44(uStack_1fc,uStack_200);
          local_330 = local_1f8;
          uStack_32f = uStack_1f7;
          uStack_328 = (undefined1)uStack_1f0;
          uStack_327 = (undefined7)(uStack_1f0 >> 8);
          local_320 = (byte)local_1e8;
          uStack_31f = (undefined7)((ulonglong)local_1e8 >> 8);
          uStack_318 = uStack_1e0;
          local_310 = local_1d8;
          uStack_308 = uStack_1d0;
          local_300 = local_1c8;
          uStack_2f8 = uStack_1c0;
          local_2f0 = local_1b8;
          uStack_2e8 = uStack_1b0;
          local_358 = 0x8000000000000000;
          local_2e0 = local_88;
          local_2d8 = CONCAT44(unaff_XMM6_Da,unaff_XMM7_Da);
          if (local_f0 == 0) {
            *(undefined8 *)(puVar20 + -8) = 0x1415effed;
            memcpy(in_RCX,&local_358,0xf8);
          }
          else {
            *(undefined8 *)(puVar20 + -8) = 0x1415f02db;
            serde::de::Error::invalid_length<serde_json::error::Error>();
            in_RCX[1] = extraout_RAX_05;
            *in_RCX = (ulonglong)&DAT_8000000000000010;
            *(undefined8 *)(puVar20 + -8) = 0x1415f02f5;
            core::ptr::
            drop_in_place<enum2_<country_core::resources::history::wall_compound_edit::WallHistoryEdit>_>
                      ();
          }
          *(undefined8 *)(puVar20 + -8) = 0x1415efff9;
          core::ptr::drop_in_place<serde_json::map::IntoIter>();
          local_71 = 0;
          *(undefined8 *)(puVar20 + -8) = 0x1415f000c;
          core::ptr::
          drop_in_place<enum2_<core::option::Option<enum2_<serde_json::value::Value>_>_>_>();
          return;
        }
        *(undefined8 *)(puVar20 + -8) = 0x1415f0079;
        serde::de::Error::missing_field<serde_json::error::Error>();
        uVar14 = extraout_RAX_03;
      }
      goto joined_r0x0001415f0046;
    }
    local_358 = *(undefined8 *)in_RDX;
    lStack_350 = *(longlong *)(in_RDX + 8);
    local_348 = *(undefined8 *)(in_RDX + 0x10);
    uStack_340 = *(undefined8 *)(in_RDX + 0x18);
    Value::unexpected();
    serde_json::error::impl$8::invalid_type();
    in_RCX[1] = extraout_RAX_04;
    *in_RCX = (ulonglong)&DAT_8000000000000010;
    local_71 = 1;
    core::ptr::drop_in_place<enum2_<serde_json::value::Value>_>();
  }
                    // WARNING: Read-only address (ram,0x000142c7c0b5) is written
  return;
LAB_1415efa4d:
  local_d8 = local_1f8;
  uStack_d7 = uStack_1f7;
  uStack_d0 = uStack_1f0;
  local_e8 = (char)local_208;
  uStack_e7 = (undefined7)(CONCAT44(uStack_204,local_208) >> 8);
  uStack_e0 = (undefined1)uStack_200;
  uStack_df = (undefined7)(CONCAT44(uStack_1fc,uStack_200) >> 8);
  *(undefined8 *)(puVar20 + -8) = 0x1415efa7b;
  uVar25 = uVar17;
  country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::
  impl$3::visit_str<serde_json::error::Error>();
  if (local_a0 != 0) {
    uVar25 = 1;
    *(undefined8 *)(puVar20 + -8) = 0x1415efa99;
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  lVar10 = local_80;
  cVar22 = local_e8;
  uVar14 = local_190;
  if (local_198 != '\0') goto joined_r0x0001415f0046;
  if (bStack_197 != 5) {
    lVar15 = (longlong)*(int *)(&DAT_142c7c0a8 + (ulonglong)(uint)bStack_197 * 4);
    pcVar16 = &DAT_142c7c0a8 + lVar15;
    cVar22 = (char)pcVar16;
    puVar21 = puVar20;
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
    switch(bStack_197) {
    case 6:
    case 7:
    case 0xb:
                    // WARNING: Bad instruction - Truncating control flow here
      halt_baddata();
    case 8:
                    // WARNING: Bad instruction - Truncating control flow here
      halt_baddata();
    case 9:
      *(undefined **)(puVar20 + -8) = &UNK_1415f0a2c;
      std::alloc::__default_lib_allocator::__rust_dealloc();
      break;
    case 10:
      *(undefined **)(puVar20 + -8) = &UNK_1415f0a62;
      std::alloc::__default_lib_allocator::__rust_dealloc();
      break;
    default:
      goto code_r0x0001415f10c9;
    case 0xf:
      *(char *)(lVar15 + 0xd01030ed) = *(char *)(lVar15 + 0xd01030ed) + -0x58;
                    // WARNING: Bad instruction - Truncating control flow here
      halt_baddata();
    case 0x10:
      goto code_r0x0001415f1159;
    case 0x11:
      *pcVar16 = *pcVar16 + cVar22;
      lVar15 = CONCAT71((int7)((ulonglong)pcVar16 >> 8),cVar22 + -0x40);
                    // WARNING (jumptable): Read-only address (ram,0x000142c7c0b5) is written
                    // WARNING: Read-only address (ram,0x000142c7c0b5) is written
      _UNK_142c7c0b5 = 0x59fe97;
      piVar1 = (int *)(lVar15 + -0x75);
      *piVar1 = *piVar1 + -1;
      *(undefined8 **)(puVar20 + -8) = &local_378;
      *(longlong *)(puVar20 + -0x10) = lVar15;
      puVar21 = puVar20 + -0x10;
      if (lVar10 == 0) goto LAB_1415f10f9;
      *(undefined **)(puVar20 + -0x18) = &UNK_1415f11c0;
      std::alloc::__default_lib_allocator::__rust_dealloc();
      puVar21 = puVar20 + -0x10;
      goto LAB_1415f10f9;
    case 0x13:
    case 0x14:
    case 0x15:
    case 0x19:
      if ((longlong)pcVar16 < 0) {
        pbVar2 = &UNK_142c7c069 + lVar15;
        bVar6 = *pbVar2;
        *pbVar2 = *pbVar2 + 0xa8;
        puVar18 = &UNK_142c7c0a7;
        if (*pbVar2 != 0) goto code_r0x0001415f16a9;
        *(char *)(lVar15 + 0x2858f815f) = *(char *)(lVar15 + 0x2858f815f) + -0x59 + (0x57 < bVar6);
        goto code_r0x0001415f16a9;
      }
      *pcVar16 = *pcVar16 + cVar22;
      puVar18 = &DAT_142c7c0a8;
      goto code_r0x0001415f16a5;
    case 0x16:
      *pcVar16 = *pcVar16 + cVar22;
      goto code_r0x0001415f17df;
    case 0x17:
      uVar5 = *puVar24;
      *puVar24 = *puVar24 + 0x42c7c0a8;
      (&UNK_142c7c0b8)[lVar15] = (&UNK_142c7c0b8)[lVar15] + -0x58 + (0xbd383f57 < uVar5);
      goto code_r0x0001415f17f5;
    case 0x18:
      puVar19 = &UNK_142c7808c;
      goto LAB_1415f16ed;
    }
    do {
      uVar25 = local_2d8;
      local_320 = (byte)uStack_2f8;
      uStack_31f = (undefined7)((ulonglong)uStack_2f8 >> 8);
      uStack_318 = local_2f0;
      local_330 = (byte)uStack_308;
      uStack_32f = (undefined7)(uStack_308 >> 8);
      uStack_328 = (undefined1)local_300;
      uStack_327 = (undefined7)((ulonglong)local_300 >> 8);
      if (puVar24 == (uint *)0x2) {
        if ((short)*piStack_2c8 != 0x6f74) goto LAB_1415f09ae;
        cVar23 = '\x02';
        cVar22 = '\x02';
      }
      else if (puVar24 == (uint *)&DAT_00000004) {
        if (*piStack_2c8 != 0x6d6f7266) goto LAB_1415f09ae;
        cVar23 = '\x01';
        cVar22 = '\x01';
      }
      else if ((puVar24 == (uint *)&DAT_00000007) &&
              (*(int *)((longlong)piStack_2c8 + 3) == 0x64695f6c && *piStack_2c8 == 0x6c6c6177)) {
        cVar23 = '\0';
        cVar22 = '\0';
      }
      else {
LAB_1415f09ae:
        cVar23 = '\x03';
        cVar22 = '\x03';
      }
      if (local_2d8 != 0) {
        *(undefined8 *)(puVar20 + -8) = 0x1415f09c8;
        std::alloc::__default_lib_allocator::__rust_dealloc();
        cVar22 = cVar23;
      }
      uVar13 = uStack_1fc;
      uVar12 = uStack_200;
      uVar11 = uStack_204;
      uVar5 = local_208;
      bVar6 = local_330;
      local_208._1_3_ = (undefined3)uStack_32f;
      uStack_204 = (uint)((uint7)uStack_32f >> 0x18);
      uStack_200._1_3_ = (undefined3)uStack_327;
      uStack_1fc = (uint)((uint7)uStack_327 >> 0x18);
      local_208 = uVar5;
      uStack_200 = uVar12;
      if (cVar22 == '\x03') {
        local_330 = '\x06';
        if (bVar6 == '\x06') {
          uStack_204 = uVar11;
          uStack_1fc = uVar13;
          *(undefined8 *)(puVar20 + -8) = 0x1415f0c3d;
          serde_json::error::impl$8::custom<ref_<str_>_>();
          uVar25 = extraout_RAX_09;
          goto LAB_1415f0c63;
        }
        uStack_308 = CONCAT71(uStack_308._1_7_,bVar6);
        uStack_1f7 = uStack_31f;
        uStack_1f0 = uStack_318;
        uStack_200 = CONCAT31(uStack_200._1_3_,uStack_328);
        local_1f8 = local_320;
        *(undefined8 *)(puVar20 + -8) = 0x1415f0a02;
        de::impl$2::deserialize_ignored_any<serde::de::ignored_any::IgnoredAny>();
        uVar25 = extraout_RAX_06;
        if (extraout_RAX_06 != 0) goto LAB_1415f0c63;
      }
      else if (cVar22 == '\x02') {
        if ((uStack_2cc & 1) == 0) {
          uStack_204 = uVar11;
          uStack_1fc = uVar13;
          *(undefined8 *)(puVar20 + -8) = 0x1415f0d3c;
          serde::de::Error::duplicate_field<serde_json::error::Error>();
          uVar25 = extraout_RAX_15;
          goto LAB_1415f0c63;
        }
        local_330 = '\x06';
        if (bVar6 == '\x06') {
          uStack_204 = uVar11;
          uStack_1fc = uVar13;
          *(undefined8 *)(puVar20 + -8) = 0x1415f0cb0;
          serde_json::error::impl$8::custom<ref_<str_>_>();
          uVar25 = extraout_RAX_12;
          goto LAB_1415f0c63;
        }
        uStack_308 = CONCAT71(uStack_308._1_7_,bVar6);
        uStack_1f7 = uStack_31f;
        uStack_1f0 = uStack_318;
        uStack_200 = CONCAT31(uStack_200._1_3_,uStack_328);
        local_1f8 = local_320;
        *(undefined8 *)(puVar20 + -8) = 0x1415f0b3a;
        de::impl$2::deserialize_u32<serde::de::impls::impl_69::deserialize::PrimitiveVisitor>();
        uVar25 = uStack_3a0;
        if ((local_3a8 & 1) != 0) goto LAB_1415f0c63;
        uStack_2e8 = CONCAT44(uStack_2e8._4_4_,uStack_3a4);
        local_388 = CONCAT71((uint7)(uint3)((uint)uStack_3a4 >> 8),1);
        uStack_2cc = 0;
      }
      else if (cVar22 == '\x01') {
        if (uStack_2c0 != 0) {
          uStack_204 = uVar11;
          uStack_1fc = uVar13;
          *(undefined8 *)(puVar20 + -8) = 0x1415f0d68;
          serde::de::Error::duplicate_field<serde_json::error::Error>();
          uVar25 = extraout_RAX_17;
          goto LAB_1415f0c63;
        }
        local_330 = '\x06';
        if (bVar6 == '\x06') {
          uStack_204 = uVar11;
          uStack_1fc = uVar13;
          *(undefined8 *)(puVar20 + -8) = 0x1415f0cd6;
          serde_json::error::impl$8::custom<ref_<str_>_>();
          uVar25 = extraout_RAX_14;
          goto LAB_1415f0c63;
        }
        uStack_308 = CONCAT71(uStack_308._1_7_,bVar6);
        uStack_1f7 = uStack_31f;
        uStack_1f0 = uStack_318;
        uStack_200 = CONCAT31(uStack_200._1_3_,uStack_328);
        local_1f8 = local_320;
        *(undefined8 *)(puVar20 + -8) = 0x1415f0ad8;
        de::impl$2::deserialize_u32<serde::de::impls::impl_69::deserialize::PrimitiveVisitor>();
        uVar25 = uStack_3a0;
        if ((local_3a8 & 1) != 0) goto LAB_1415f0c63;
        uStack_2e8 = CONCAT44(uStack_3a4,(undefined4)uStack_2e8);
        uStack_2c0 = 1;
      }
      else {
        if (local_2e0 != 0) {
          uStack_204 = uVar11;
          uStack_1fc = uVar13;
          *(undefined8 *)(puVar20 + -8) = 0x1415f0d52;
          serde::de::Error::duplicate_field<serde_json::error::Error>();
          uVar25 = extraout_RAX_16;
          goto LAB_1415f0c63;
        }
        local_330 = '\x06';
        if (bVar6 == '\x06') {
          uStack_204 = uVar11;
          uStack_1fc = uVar13;
          *(undefined8 *)(puVar20 + -8) = 0x1415f0cc3;
          serde_json::error::impl$8::custom<ref_<str_>_>();
          uVar25 = extraout_RAX_13;
          goto LAB_1415f0c63;
        }
        uStack_308 = CONCAT71(uStack_308._1_7_,bVar6);
        uStack_1f7 = uStack_31f;
        uStack_1f0 = uStack_318;
        uStack_200 = CONCAT31(uStack_200._1_3_,uStack_328);
        local_1f8 = local_320;
        *(undefined8 *)(puVar20 + -8) = 0x1415f0ba0;
        de::impl$2::deserialize_u64<serde::de::impls::impl_75::deserialize::PrimitiveVisitor>();
        uStack_380 = uVar25;
        if (extraout_RAX_07 != 0) goto LAB_1415f0c63;
        local_2e0 = 1;
      }
      *(undefined8 *)(puVar20 + -8) = 0x1415f089b;
      alloc::collections::btree::map::IntoIter::
      dying_next<alloc::string::String,enum2_<serde_json::value::Value>,alloc::alloc::Global>();
      lVar10 = CONCAT44(uStack_3a4,local_3a8);
      if (lVar10 == 0) {
LAB_1415f0bbe:
        if ((local_2e0 & 1) == 0) {
          *(undefined8 *)(puVar20 + -8) = 0x1415f0c2a;
          serde::de::Error::missing_field<serde_json::error::Error>();
          uVar25 = extraout_RAX_08;
        }
        else if ((uStack_2c0 & 1) == 0) {
          *(undefined8 *)(puVar20 + -8) = 0x1415f0c50;
          serde::de::Error::missing_field<serde_json::error::Error>();
          uVar25 = extraout_RAX_10;
        }
        else {
          if ((local_388 & 1) != 0) {
            if (lStack_338 == 0) {
              *(undefined1 *)(in_RCX + 1) = 4;
              in_RCX[2] = uStack_380;
              *(undefined4 *)(in_RCX + 3) = uStack_2e8._4_4_;
              *(undefined4 *)((longlong)in_RCX + 0x1c) = (undefined4)uStack_2e8;
            }
            else {
              *(undefined8 *)(puVar20 + -8) = 0x1415f0d84;
              serde::de::Error::invalid_length<serde_json::error::Error>();
              *in_RCX = extraout_RAX_18;
              *(undefined1 *)(in_RCX + 1) = 0x11;
            }
            *(undefined8 *)(puVar20 + -8) = 0x1415f0c07;
            core::ptr::drop_in_place<serde_json::map::IntoIter>();
            uStack_2b9 = 0;
            *(undefined8 *)(puVar20 + -8) = 0x1415f0c17;
            core::ptr::
            drop_in_place<enum2_<core::option::Option<enum2_<serde_json::value::Value>_>_>_>();
            return;
          }
          *(undefined8 *)(puVar20 + -8) = 0x1415f0c63;
          serde::de::Error::missing_field<serde_json::error::Error>();
          uVar25 = extraout_RAX_11;
        }
LAB_1415f0c63:
        *in_RCX = uVar25;
        *(undefined1 *)(in_RCX + 1) = 0x11;
        *(undefined8 *)(puVar20 + -8) = 0x1415f0c72;
        core::ptr::drop_in_place<serde_json::map::IntoIter>();
        uStack_2b9 = 0;
        *(undefined8 *)(puVar20 + -8) = 0x1415f0c82;
        core::ptr::drop_in_place<enum2_<core::option::Option<enum2_<serde_json::value::Value>_>_>_>
                  ();
                    // WARNING: Read-only address (ram,0x000142c7c0b5) is written
        return;
      }
      uVar25 = *(ulonglong *)(lVar10 + 0x168 + (longlong)local_398 * 0x18);
      piStack_2c8 = *(int **)(lVar10 + 0x170 + (longlong)local_398 * 0x18);
      puVar24 = *(uint **)(lVar10 + 0x178 + (longlong)local_398 * 0x18);
      puVar3 = (undefined8 *)(lVar10 + (longlong)local_398 * 0x20);
      uStack_308 = *puVar3;
      local_300 = puVar3[1];
      puVar3 = (undefined8 *)(lVar10 + 0x10 + (longlong)local_398 * 0x20);
      uStack_2f8 = *puVar3;
      local_2f0 = puVar3[1];
      if (SBORROW8(0,uVar25)) goto LAB_1415f0bbe;
      local_2d8 = uVar25;
      switch(local_330) {
      case 0:
      case 1:
      case 2:
      case 6:
        goto LAB_1415efa4d;
      case 3:
        goto code_r0x0001415efb07;
      case 4:
        goto code_r0x0001415efb2e;
      case 5:
        goto LAB_1415efb73;
      default:
        *(undefined8 *)(puVar20 + -8) = 0x1415f0a70;
        core::ptr::
        drop_in_place<serde_json::map::Map<alloc::string::String,enum2_<serde_json::value::Value>_>_>
                  ();
      }
    } while( true );
  }
  local_e8 = '\x06';
  if (cVar22 == '\x06') {
    *(undefined8 *)(puVar20 + -8) = 0x1415f005f;
    serde_json::error::impl$8::custom<ref_<str_>_>();
    uVar14 = extraout_RAX_02;
    goto joined_r0x0001415f0046;
  }
  uStack_1f7 = uStack_d7;
  uStack_1f0 = uStack_d0;
  local_208._1_3_ = (undefined3)uStack_e7;
  local_208 = CONCAT31(local_208._1_3_,cVar22);
  uStack_204 = (uint)((uint7)uStack_e7 >> 0x18);
  uStack_200._1_3_ = (undefined3)uStack_df;
  uStack_200 = CONCAT31(uStack_200._1_3_,uStack_e0);
  uStack_1fc = (uint)((uint7)uStack_df >> 0x18);
  local_1f8 = local_d8;
  *(undefined8 *)(puVar20 + -8) = 0x1415efaf9;
  de::impl$2::deserialize_ignored_any<serde::de::ignored_any::IgnoredAny>();
  uVar14 = extraout_RAX_00;
  if (extraout_RAX_00 != 0) {
joined_r0x0001415f0046:
    local_88 = uVar14;
    if (-0x7fffffffffffffff < local_80) {
      if (local_80 != 0) {
        *(undefined8 *)(puVar20 + -8) = 0x1415f00c1;
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if (lStack_3b0 != 0) {
        *(undefined8 *)(puVar20 + -8) = 0x1415f00dd;
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
    in_RCX[1] = local_88;
    *in_RCX = (ulonglong)&DAT_8000000000000010;
    *(undefined8 *)(puVar20 + -8) = 0x1415f0101;
    core::ptr::drop_in_place<serde_json::map::IntoIter>();
    local_71 = 0;
    *(undefined8 *)(puVar20 + -8) = 0x1415f0114;
    core::ptr::drop_in_place<enum2_<core::option::Option<enum2_<serde_json::value::Value>_>_>_>();
    return;
  }
  goto LAB_1415ef9c0;
code_r0x0001415f16a5:
  puVar18 = (undefined *)((longlong)puVar18 << 5);
code_r0x0001415f16a9:
  local_218 = *(byte **)(pcVar16 + (longlong)puVar18);
  local_210 = *(undefined8 *)(pcVar16 + (longlong)puVar18 + 8);
  puVar4 = (uint *)(pcVar16 + (longlong)(puVar18 + 0x10));
  local_208 = *puVar4;
  uStack_204 = puVar4[1];
  uStack_200 = puVar4[2];
  uStack_1fc = puVar4[3];
  if (SBORROW8(0,uVar25)) {
LAB_1415f1a0a:
    if ((local_1f8 & 1) == 0) {
      *(undefined8 *)(puVar20 + -8) = 0x1415f1a43;
      serde::de::Error::missing_field<serde_json::error::Error>();
      uVar17 = extraout_RAX_25;
    }
    else if (uStack_1e0._5_1_ == '\x02') {
      *(undefined8 *)(puVar20 + -8) = 0x1415f1a30;
      serde::de::Error::missing_field<serde_json::error::Error>();
      uVar17 = extraout_RAX_24;
    }
    else {
      if (uStack_1e0._6_1_ != '\x02') {
        uStack_3d8 = 0x800000000000000f;
        uStack_3d0 = local_220;
        local_3c8 = CONCAT71(local_3c8._1_7_,uStack_1e0._5_1_) & 0xffffffffffffff01;
        local_3c8 = CONCAT62(local_3c8._2_6_,CONCAT11(uStack_1e0._6_1_,(undefined1)local_3c8)) &
                    0xffffffffffff01ff;
        if (lStack_268 == 0) {
          *(undefined8 *)(puVar20 + -8) = 0x1415f1b01;
          memcpy(in_RCX,&uStack_3d8,0xf8);
        }
        else {
          *(undefined8 *)(puVar20 + -8) = 0x1415f1c37;
          serde::de::Error::invalid_length<serde_json::error::Error>();
          in_RCX[1] = extraout_RAX_34;
          *in_RCX = (ulonglong)&DAT_8000000000000010;
          *(undefined8 *)(puVar20 + -8) = 0x1415f1c51;
          core::ptr::
          drop_in_place<enum2_<country_core::resources::history::wall_compound_edit::WallHistoryEdit>_>
                    ();
        }
        *(undefined8 *)(puVar20 + -8) = 0x1415f1b0d;
        core::ptr::drop_in_place<serde_json::map::IntoIter>();
        uStack_1e0 = uStack_1e0 & 0xffffffffffffff;
        *(undefined8 *)(puVar20 + -8) = 0x1415f1b20;
        core::ptr::drop_in_place<enum2_<core::option::Option<enum2_<serde_json::value::Value>_>_>_>
                  ();
        return;
      }
      *(undefined8 *)(puVar20 + -8) = 0x1415f1a75;
      serde::de::Error::missing_field<serde_json::error::Error>();
      uVar17 = extraout_RAX_27;
    }
LAB_1415f1a75:
    in_RCX[1] = uVar17;
    *in_RCX = (ulonglong)&DAT_8000000000000010;
    *(undefined8 *)(puVar20 + -8) = 0x1415f1a92;
    core::ptr::drop_in_place<serde_json::map::IntoIter>();
    uStack_1e0 = uStack_1e0 & 0xffffffffffffff;
    *(undefined8 *)(puVar20 + -8) = 0x1415f1aa5;
    core::ptr::drop_in_place<enum2_<core::option::Option<enum2_<serde_json::value::Value>_>_>_>();
                    // WARNING: Read-only address (ram,0x000142c7c0b5) is written
    return;
  }
  uStack_1f0 = uVar25;
  if ((ulonglong)bStack_260 < 7) {
                    // WARNING: Could not recover jumptable at 0x0001415f16eb. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(puVar19 + *(int *)(puVar19 + (ulonglong)bStack_260 * 4)))();
    return;
  }
  *(undefined8 *)(puVar20 + -8) = 0x1415f18a1;
  core::ptr::
  drop_in_place<serde_json::map::Map<alloc::string::String,enum2_<serde_json::value::Value>_>_>();
LAB_1415f16ed:
  uVar17 = uStack_1f0;
  uStack_248 = CONCAT44(uStack_1fc,uStack_200);
  bStack_250 = (byte)local_208;
  uStack_24f = (undefined7)(CONCAT44(uStack_204,local_208) >> 8);
  bStack_260 = (byte)local_218;
  uStack_25f = (undefined7)((ulonglong)local_218 >> 8);
  uStack_258 = (undefined1)local_210;
  uStack_257 = (undefined7)((ulonglong)local_210 >> 8);
  if (puVar24 == (uint *)&DAT_00000005) {
    if (*(char *)((longlong)local_1e8 + 4) != 'r' || (int)*local_1e8 != 0x65746661)
    goto LAB_1415f17bb;
    cVar23 = '\x02';
    cVar22 = '\x02';
  }
  else if (puVar24 == (uint *)&DAT_00000006) {
    if (*(short *)((longlong)local_1e8 + 4) != 0x6572 || (int)*local_1e8 != 0x6f666562)
    goto LAB_1415f17bb;
    cVar23 = '\x01';
    cVar22 = '\x01';
  }
  else if ((puVar24 == (uint *)&DAT_0000000a) &&
          ((short)local_1e8[1] == 0x7865 && *local_1e8 == 0x646e695f6c6c6177)) {
    cVar23 = '\0';
    cVar22 = '\0';
  }
  else {
LAB_1415f17bb:
    cVar23 = '\x03';
    cVar22 = '\x03';
  }
  if (uStack_1f0 != 0) {
    *(undefined8 *)(puVar20 + -8) = 0x1415f17d5;
    std::alloc::__default_lib_allocator::__rust_dealloc();
    cVar22 = cVar23;
  }
  uVar13 = uStack_1fc;
  uVar12 = uStack_200;
  uVar11 = uStack_204;
  uVar5 = local_208;
  bVar6 = bStack_260;
  if (cVar22 == '\x03') {
code_r0x0001415f17df:
    bVar6 = bStack_260;
    pcVar16 = (char *)(ulonglong)bStack_260;
    bStack_260 = 6;
    if (bVar6 == 6) {
      *(undefined8 *)(puVar20 + -8) = 0x1415f1a56;
      serde_json::error::impl$8::custom<ref_<str_>_>();
      uVar17 = extraout_RAX_26;
      goto LAB_1415f1a75;
    }
code_r0x0001415f17f5:
    local_218 = (byte *)CONCAT71(local_218._1_7_,(char)pcVar16);
    uStack_1f7 = uStack_24f;
    uStack_1f0 = uStack_248;
    local_208._1_3_ = (undefined3)uStack_25f;
    uStack_204 = (uint)((uint7)uStack_25f >> 0x18);
    uStack_200._1_3_ = (undefined3)uStack_257;
    uStack_200 = CONCAT31(uStack_200._1_3_,uStack_258);
    uStack_1fc = (uint)((uint7)uStack_257 >> 0x18);
    local_1f8 = bStack_250;
    *(undefined8 *)(puVar20 + -8) = 0x1415f181b;
    de::impl$2::deserialize_ignored_any<serde::de::ignored_any::IgnoredAny>();
    uVar17 = extraout_RAX_22;
    if (extraout_RAX_22 != 0) goto LAB_1415f1a75;
  }
  else {
    local_208._1_3_ = (undefined3)uStack_25f;
    uStack_204 = (uint)((uint7)uStack_25f >> 0x18);
    uStack_200._1_3_ = (undefined3)uStack_257;
    uStack_1fc = (uint)((uint7)uStack_257 >> 0x18);
    local_208 = uVar5;
    uStack_200 = uVar12;
    if (cVar22 == '\x02') {
      if (uStack_1e0._6_1_ != '\x02') {
        uStack_204 = uVar11;
        uStack_1fc = uVar13;
        *(undefined8 *)(puVar20 + -8) = 0x1415f1bec;
        serde::de::Error::duplicate_field<serde_json::error::Error>();
        uVar17 = extraout_RAX_31;
        goto LAB_1415f1a75;
      }
      bStack_260 = 6;
      if (bVar6 == 6) {
        uStack_204 = uVar11;
        uStack_1fc = uVar13;
        *(undefined8 *)(puVar20 + -8) = 0x1415f1b3f;
        serde_json::error::impl$8::custom<ref_<str_>_>();
        uVar17 = extraout_RAX_28;
        goto LAB_1415f1a75;
      }
      local_218 = (byte *)CONCAT71(local_218._1_7_,bVar6);
      uStack_1f7 = uStack_24f;
      uStack_1f0 = uStack_248;
      uStack_200 = CONCAT31(uStack_200._1_3_,uStack_258);
      local_1f8 = bStack_250;
      *(undefined8 *)(puVar20 + -8) = 0x1415f1980;
      de::impl$2::deserialize_bool<serde::de::impls::BoolVisitor>();
      uVar17 = local_238;
      if ((bStack_240 & 1) != 0) goto LAB_1415f1a75;
      uStack_1e0._0_7_ = CONCAT16(uStack_23f,(undefined6)uStack_1e0);
    }
    else if (cVar22 == '\x01') {
      if (uStack_1e0._5_1_ != '\x02') {
        uStack_204 = uVar11;
        uStack_1fc = uVar13;
        *(undefined8 *)(puVar20 + -8) = 0x1415f1c18;
        serde::de::Error::duplicate_field<serde_json::error::Error>();
        uVar17 = extraout_RAX_33;
        goto LAB_1415f1a75;
      }
      bStack_260 = 6;
      if (bVar6 == 6) {
        uStack_204 = uVar11;
        uStack_1fc = uVar13;
        *(undefined8 *)(puVar20 + -8) = 0x1415f1b6b;
        serde_json::error::impl$8::custom<ref_<str_>_>();
        uVar17 = extraout_RAX_30;
        goto LAB_1415f1a75;
      }
      local_218 = (byte *)CONCAT71(local_218._1_7_,bVar6);
      uStack_1f7 = uStack_24f;
      uStack_1f0 = uStack_248;
      uStack_200 = CONCAT31(uStack_200._1_3_,uStack_258);
      local_1f8 = bStack_250;
      *(undefined8 *)(puVar20 + -8) = 0x1415f1915;
      de::impl$2::deserialize_bool<serde::de::impls::BoolVisitor>();
      uVar17 = local_238;
      if ((bStack_240 & 1) != 0) goto LAB_1415f1a75;
      uStack_1e0._0_6_ = CONCAT15(uStack_23f,(undefined5)uStack_1e0);
    }
    else {
      if (CONCAT71(uStack_1f7,local_1f8) != 0) {
        uStack_204 = uVar11;
        uStack_1fc = uVar13;
        *(undefined8 *)(puVar20 + -8) = 0x1415f1c02;
        serde::de::Error::duplicate_field<serde_json::error::Error>();
        uVar17 = extraout_RAX_32;
        goto LAB_1415f1a75;
      }
      bStack_260 = 6;
      if (bVar6 == 6) {
        uStack_204 = uVar11;
        uStack_1fc = uVar13;
        *(undefined8 *)(puVar20 + -8) = 0x1415f1b55;
        serde_json::error::impl$8::custom<ref_<str_>_>();
        uVar17 = extraout_RAX_29;
        goto LAB_1415f1a75;
      }
      local_218 = (byte *)CONCAT71(local_218._1_7_,bVar6);
      uStack_1f7 = uStack_24f;
      uStack_1f0 = uStack_248;
      uStack_200 = CONCAT31(uStack_200._1_3_,uStack_258);
      local_1f8 = bStack_250;
      *(undefined8 *)(puVar20 + -8) = 0x1415f19e9;
      de::impl$2::deserialize_u64<serde::de::impls::impl_75::deserialize::PrimitiveVisitor>();
      local_220 = uVar17;
      if (extraout_RAX_23 != 0) goto LAB_1415f1a75;
      local_1f8 = 1;
      uStack_1f7 = 0;
    }
  }
  *(undefined8 *)(puVar20 + -8) = 0x1415f166b;
  alloc::collections::btree::map::IntoIter::
  dying_next<alloc::string::String,enum2_<serde_json::value::Value>,alloc::alloc::Global>();
  pcVar16 = (char *)CONCAT62(uStack_23e,CONCAT11(uStack_23f,bStack_240));
  if (pcVar16 == (char *)0x0) goto LAB_1415f1a0a;
  uVar25 = *(ulonglong *)(pcVar16 + (longlong)puStack_230 * 0x18 + 0x168);
  local_1e8 = *(longlong **)(pcVar16 + (longlong)puStack_230 * 0x18 + 0x170);
  puVar24 = *(uint **)(pcVar16 + (longlong)puStack_230 * 0x18 + 0x178);
  puVar18 = puStack_230;
  goto code_r0x0001415f16a5;
code_r0x0001415f10c9:
  uStack_2f8 = CONCAT44(uStack_2f8._4_4_,(int)uStack_2f8 + -0x42c7c0a8);
  do {
    puVar21 = puVar20;
    if (SBORROW8(0,uVar25)) {
LAB_1415f11d3:
      uStack_3a0 = 0xb;
      if (lStack_338 == 0) {
        in_RCX[2] = uStack_390;
        *in_RCX = 0xb;
        in_RCX[1] = (ulonglong)local_398;
      }
      else {
        *(undefined8 *)(puVar21 + -8) = 0x1415f12c6;
        serde::de::Error::invalid_length<serde_json::error::Error>();
        in_RCX[1] = extraout_RAX_21;
        *in_RCX = 0xf;
        *(undefined8 *)(puVar21 + -8) = 0x1415f12da;
        core::ptr::drop_in_place<enum2_<country_core::resources::history::EditType>_>();
      }
      *(undefined8 *)(puVar21 + -8) = 0x1415f1205;
      core::ptr::drop_in_place<serde_json::map::IntoIter>();
      uStack_2cc = uStack_2cc & 0xffffff;
      *(undefined8 *)(puVar21 + -8) = 0x1415f1215;
      core::ptr::drop_in_place<enum2_<core::option::Option<enum2_<serde_json::value::Value>_>_>_>();
      return;
    }
    local_2d8 = uVar25;
    switch(local_330) {
    case 0:
    case 1:
    case 2:
    case 6:
      goto LAB_1415efa4d;
    case 3:
      goto code_r0x0001415efb07;
    case 4:
      goto code_r0x0001415efb2e;
    case 5:
      goto LAB_1415efb73;
    default:
      *(undefined8 *)(puVar20 + -8) = 0x1415f11ce;
      core::ptr::
      drop_in_place<serde_json::map::Map<alloc::string::String,enum2_<serde_json::value::Value>_>_>
                ();
LAB_1415f10f9:
      local_320 = (byte)uStack_2f8;
      uStack_31f = (undefined7)((ulonglong)uStack_2f8 >> 8);
      uStack_318 = local_2f0;
      local_330 = (byte)uStack_308;
      uStack_32f = (undefined7)(uStack_308 >> 8);
      uStack_328 = (undefined1)local_300;
      uStack_327 = (undefined7)((ulonglong)local_300 >> 8);
      if (local_2d8 != 0) {
        *(undefined8 *)(puVar21 + -8) = 0x1415f112d;
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      bVar6 = local_330;
      local_330 = '\x06';
      if (bVar6 == '\x06') {
        *(undefined8 *)(puVar21 + -8) = 0x1415f1228;
        serde_json::error::impl$8::custom<ref_<str_>_>();
        uVar25 = extraout_RAX_20;
LAB_1415f1228:
        in_RCX[1] = uVar25;
        *in_RCX = 0xf;
        *(undefined8 *)(puVar21 + -8) = 0x1415f123b;
        core::ptr::drop_in_place<serde_json::map::IntoIter>();
        uStack_2cc = uStack_2cc & 0xffffff;
        *(undefined8 *)(puVar21 + -8) = 0x1415f124b;
        core::ptr::drop_in_place<enum2_<core::option::Option<enum2_<serde_json::value::Value>_>_>_>
                  ();
                    // WARNING: Read-only address (ram,0x000142c7c0b5) is written
        return;
      }
      uStack_308 = CONCAT71(uStack_308._1_7_,bVar6);
      local_130 = CONCAT17(uStack_328,uStack_32f);
      uStack_120 = uStack_31f;
      uStack_119 = uStack_318;
      uStack_128 = uStack_327;
      bStack_121 = local_320;
code_r0x0001415f1159:
      *(undefined8 *)(puVar21 + -8) = 0x1415f115e;
      de::impl$2::deserialize_ignored_any<serde::de::ignored_any::IgnoredAny>();
      uVar25 = extraout_RAX_19;
      if (extraout_RAX_19 != 0) goto LAB_1415f1228;
      *(undefined8 *)(puVar21 + -8) = 0x1415f108b;
      alloc::collections::btree::map::IntoIter::
      dying_next<alloc::string::String,enum2_<serde_json::value::Value>,alloc::alloc::Global>();
      if (uStack_3d0 == 0) goto LAB_1415f11d3;
      uVar25 = *(ulonglong *)(uStack_3d0 + 0x168 + lStack_3c0 * 0x18);
      uStack_2e8 = *(undefined8 *)(uStack_3d0 + 0x170 + lStack_3c0 * 0x18);
      puVar3 = (undefined8 *)(uStack_3d0 + lStack_3c0 * 0x20);
      uStack_308 = *puVar3;
      local_300 = puVar3[1];
      puVar3 = (undefined8 *)(uStack_3d0 + 0x10 + lStack_3c0 * 0x20);
      uStack_2f8 = *puVar3;
      local_2f0 = puVar3[1];
      puVar20 = puVar21;
    }
  } while( true );
LAB_1415efb73:
  *(undefined8 *)(puVar20 + -8) = 0x1415efb7f;
  core::ptr::
  drop_in_place<serde_json::map::Map<alloc::string::String,enum2_<serde_json::value::Value>_>_>();
  goto LAB_1415efa4d;
code_r0x0001415efb2e:
  uStack_c8 = CONCAT71(uStack_d7,local_d8);
  *(undefined **)(puVar20 + -8) = &UNK_1415efb48;
  core::ptr::drop_in_place<slice2_<enum2_<serde_json::value::Value>_>_>();
  if (CONCAT71(uStack_df,uStack_e0) != 0) {
    *(undefined **)(puVar20 + -8) = &UNK_1415efb6e;
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  goto LAB_1415efa4d;
code_r0x0001415efb07:
  if (CONCAT71(uStack_df,uStack_e0) != 0) {
    *(undefined **)(puVar20 + -8) = &UNK_1415efb29;
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  goto LAB_1415efa4d;
}

