
// WARNING: Type propagation algorithm not settling

void __cdecl
serde_json::value::de::impl$5::
struct_variant<country_core::resources::history::wall_compound_edit::_::impl_0::deserialize::impl_2::visit_enum::__Visitor>
          (void)

{
  undefined8 *puVar1;
  byte bVar2;
  longlong lVar3;
  longlong lVar4;
  undefined1 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  longlong lVar9;
  undefined8 extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong extraout_RAX_04;
  longlong lVar10;
  longlong extraout_RAX_05;
  longlong extraout_RAX_06;
  longlong extraout_RAX_07;
  longlong extraout_RAX_08;
  undefined8 extraout_RAX_09;
  longlong extraout_RAX_10;
  longlong extraout_RAX_11;
  longlong extraout_RAX_12;
  undefined8 extraout_RAX_13;
  undefined8 *in_RCX;
  byte *in_RDX;
  char cVar11;
  undefined8 local_2f8;
  longlong lStack_2f0;
  longlong local_2e8;
  undefined8 uStack_2e0;
  longlong local_2d8;
  undefined8 uStack_2d0;
  undefined8 local_2c8;
  undefined8 uStack_2c0;
  longlong local_2b8;
  longlong local_2b0;
  undefined8 local_2a8;
  longlong lStack_2a0;
  undefined8 local_298;
  undefined8 uStack_290;
  undefined8 local_288;
  longlong local_280;
  undefined8 local_1f8;
  undefined8 uStack_1f0;
  undefined8 local_1e8;
  longlong local_1e0;
  longlong local_1d8;
  undefined8 local_1d0;
  longlong lStack_1c8;
  undefined8 local_1c0;
  undefined8 uStack_1b8;
  undefined8 local_1b0;
  char local_1a8;
  undefined7 uStack_1a7;
  undefined1 uStack_1a0;
  undefined7 uStack_19f;
  undefined1 uStack_198;
  undefined7 uStack_197;
  longlong lStack_190;
  ulonglong local_188;
  undefined8 local_178;
  longlong local_170;
  longlong lStack_168;
  undefined8 local_160;
  longlong lStack_158;
  undefined8 local_150;
  undefined8 uStack_148;
  undefined8 local_140;
  longlong local_f8;
  char local_f0;
  undefined7 uStack_ef;
  undefined1 uStack_e8;
  undefined7 uStack_e7;
  undefined1 local_e0;
  undefined7 uStack_df;
  longlong lStack_d8;
  undefined8 uStack_d0;
  longlong local_c8;
  longlong local_c0;
  char local_b8;
  undefined7 uStack_b7;
  undefined1 uStack_b0;
  undefined7 uStack_af;
  undefined1 local_a8;
  undefined7 uStack_a7;
  longlong lStack_a0;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  ulonglong local_78;
  longlong local_70;
  longlong local_68;
  longlong *local_60;
  longlong local_58;
  undefined1 local_4a;
  undefined1 local_49;
  undefined8 local_48;
  char cVar12;
  
  local_48 = 0xfffffffffffffffe;
  bVar2 = *in_RDX;
  local_188 = (ulonglong)bVar2;
  if (bVar2 == 6) {
    local_2f8 = CONCAT71(local_2f8._1_7_,0xd);
    local_4a = 0;
    serde_json::error::impl$8::invalid_type();
    in_RCX[1] = extraout_RAX;
    *in_RCX = &DAT_8000000000000010;
  }
  else {
    if (bVar2 == 5) {
      local_1f8 = *(undefined8 *)(in_RDX + 8);
      uStack_1f0 = *(undefined8 *)(in_RDX + 0x10);
      local_1e8 = *(undefined8 *)(in_RDX + 0x18);
      local_4a = 0;
      local_178 = local_1e8;
      MapDeserializer::new();
      local_170 = -0x8000000000000000;
      local_1e0 = -0x8000000000000000;
      local_78 = 0;
      local_68 = -0x8000000000000000;
      local_58 = -0x8000000000000000;
LAB_1415d87c0:
      local_49 = 1;
      alloc::collections::btree::map::IntoIter::
      dying_next<alloc::string::String,enum2_<serde_json::value::Value>,alloc::alloc::Global>();
      lVar10 = CONCAT71(uStack_1a7,local_1a8);
      if (lVar10 == 0) {
LAB_1415d8c4c:
        if ((local_78 & 1) != 0) {
          uVar5 = !SBORROW8(0,local_68);
          if (SBORROW8(0,local_68)) {
            local_49 = 1;
            serde::de::Error::missing_field<serde_json::error::Error>();
            lVar10 = extraout_RAX_05;
          }
          else {
            local_88 = local_140;
            local_98 = local_150;
            uStack_90 = uStack_148;
            local_a8 = (undefined1)local_160;
            uStack_a7 = (undefined7)((ulonglong)local_160 >> 8);
            lStack_a0 = lStack_158;
            local_b8 = (char)local_170;
            uStack_b7 = (undefined7)((ulonglong)local_170 >> 8);
            uStack_b0 = (undefined1)lStack_168;
            uStack_af = (undefined7)((ulonglong)lStack_168 >> 8);
            if (!SBORROW8(0,local_58)) {
              local_288 = local_1b0;
              local_298 = local_1c0;
              uStack_290 = uStack_1b8;
              local_2a8 = local_1d0;
              lStack_2a0 = lStack_1c8;
              lStack_2f0 = local_170;
              local_2e8 = lStack_168;
              uStack_2e0 = local_160;
              local_2d8 = lStack_158;
              uStack_2d0 = local_150;
              local_2c8 = uStack_148;
              uStack_2c0 = local_140;
              local_2f8 = 0x800000000000000c;
              local_2b8 = local_58;
              local_2b0 = local_c8;
              local_280 = local_c0;
              if (local_f8 == 0) {
                memcpy(in_RCX,&local_2f8,0xf8);
              }
              else {
                serde::de::Error::invalid_length<serde_json::error::Error>();
                in_RCX[1] = extraout_RAX_13;
                *in_RCX = &DAT_8000000000000010;
                core::ptr::
                drop_in_place<enum2_<country_core::resources::history::wall_compound_edit::WallHistoryEdit>_>
                          ();
              }
              core::ptr::drop_in_place<serde_json::map::IntoIter>();
              local_4a = 0;
              core::ptr::
              drop_in_place<enum2_<core::option::Option<enum2_<serde_json::value::Value>_>_>_>();
              return;
            }
            serde::de::Error::missing_field<serde_json::error::Error>();
            local_49 = 0;
            core::ptr::
            drop_in_place<utils::curve::Curve<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>_>
                      ();
            lVar10 = extraout_RAX_02;
          }
          goto joined_r0x0001415d8f6d;
        }
        local_49 = 1;
        serde::de::Error::missing_field<serde_json::error::Error>();
        lVar10 = extraout_RAX_03;
      }
      else {
        lVar9 = CONCAT71(uStack_197,uStack_198);
        lVar3 = *(longlong *)(lVar10 + 0x168 + lVar9 * 0x18);
        local_60 = *(longlong **)(lVar10 + 0x170 + lVar9 * 0x18);
        lVar4 = *(longlong *)(lVar10 + 0x178 + lVar9 * 0x18);
        puVar1 = (undefined8 *)(lVar10 + lVar9 * 0x20);
        uVar6 = *puVar1;
        uVar7 = puVar1[1];
        puVar1 = (undefined8 *)(lVar10 + 0x10 + lVar9 * 0x20);
        uVar8 = *puVar1;
        lStack_a0 = puVar1[1];
        local_b8 = (char)uVar6;
        uStack_b7 = (undefined7)((ulonglong)uVar6 >> 8);
        uStack_b0 = (undefined1)uVar7;
        uStack_af = (undefined7)((ulonglong)uVar7 >> 8);
        local_a8 = (undefined1)uVar8;
        uStack_a7 = (undefined7)((ulonglong)uVar8 >> 8);
        if (SBORROW8(0,lVar3)) goto LAB_1415d8c4c;
        local_70 = lVar3;
        switch(local_f0) {
        case '\0':
        case '\x01':
        case '\x02':
        case '\x06':
          break;
        case '\x03':
          if (CONCAT71(uStack_e7,uStack_e8) != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          break;
        case '\x04':
          uStack_d0 = CONCAT71(uStack_df,local_e0);
          core::ptr::drop_in_place<slice2_<enum2_<serde_json::value::Value>_>_>();
          if (CONCAT71(uStack_e7,uStack_e8) != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          break;
        default:
          core::ptr::
          drop_in_place<serde_json::map::Map<alloc::string::String,enum2_<serde_json::value::Value>_>_>
                    ();
        }
        local_e0 = local_a8;
        uStack_df = uStack_a7;
        lStack_d8 = lStack_a0;
        local_f0 = local_b8;
        uStack_ef = uStack_b7;
        uStack_e8 = uStack_b0;
        uStack_e7 = uStack_af;
        if (lVar4 == 5) {
          if (*(char *)((longlong)local_60 + 4) != 'r' || (int)*local_60 != 0x65746661)
          goto LAB_1415d891b;
          cVar12 = '\x02';
          cVar11 = '\x02';
        }
        else if (lVar4 == 6) {
          if (*(short *)((longlong)local_60 + 4) != 0x6572 || (int)*local_60 != 0x6f666562)
          goto LAB_1415d891b;
          cVar12 = '\x01';
          cVar11 = '\x01';
        }
        else if ((lVar4 == 10) && ((short)local_60[1] == 0x7865 && *local_60 == 0x646e695f6c6c6177))
        {
          cVar12 = '\0';
          cVar11 = '\0';
        }
        else {
LAB_1415d891b:
          cVar12 = '\x03';
          cVar11 = '\x03';
        }
        lVar10 = local_70;
        if (local_70 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
          cVar11 = cVar12;
        }
        cVar12 = local_f0;
        if (cVar11 == '\x03') {
          local_f0 = '\x06';
          if (cVar12 == '\x06') {
            local_49 = 1;
            serde_json::error::impl$8::custom<ref_<str_>_>();
            lVar10 = extraout_RAX_04;
            goto LAB_1415d8d13;
          }
          local_b8 = cVar12;
          uStack_a7 = uStack_df;
          lStack_a0 = lStack_d8;
          uStack_b7 = uStack_ef;
          uStack_b0 = uStack_e8;
          uStack_af = uStack_e7;
          local_a8 = local_e0;
          local_49 = 1;
          de::impl$2::deserialize_ignored_any<serde::de::ignored_any::IgnoredAny>();
          if (extraout_RAX_00 != 0) {
            uVar5 = false;
            lVar10 = extraout_RAX_00;
joined_r0x0001415d8f6d:
            if (!SBORROW8(0,local_58)) {
joined_r0x0001415d8ffe:
              if (local_58 != 0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
              if (lStack_1c8 != 0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
            }
LAB_1415d8d94:
            if (!SBORROW8(0,local_170) && !(bool)uVar5) {
              if (local_170 != 0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
              if (lStack_158 != 0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
            }
            in_RCX[1] = lVar10;
            *in_RCX = &DAT_8000000000000010;
            core::ptr::drop_in_place<serde_json::map::IntoIter>();
            local_4a = 0;
            core::ptr::
            drop_in_place<enum2_<core::option::Option<enum2_<serde_json::value::Value>_>_>_>();
            return;
          }
          goto LAB_1415d87c0;
        }
        if (cVar11 == '\0') {
          if (local_78 == 0) {
            local_f0 = '\x06';
            if (cVar12 == '\x06') {
              local_49 = 1;
              serde_json::error::impl$8::custom<ref_<str_>_>();
              lVar10 = extraout_RAX_06;
              goto LAB_1415d8d13;
            }
            local_b8 = cVar12;
            uStack_a7 = uStack_df;
            lStack_a0 = lStack_d8;
            uStack_b7 = uStack_ef;
            uStack_b0 = uStack_e8;
            uStack_af = uStack_e7;
            local_a8 = local_e0;
            local_49 = 1;
            de::impl$2::deserialize_u64<serde::de::impls::impl_75::deserialize::PrimitiveVisitor>();
            local_c0 = lVar10;
            if (extraout_RAX_01 != 0) {
              uVar5 = false;
              goto joined_r0x0001415d8f6d;
            }
            local_78 = 1;
            goto LAB_1415d87c0;
          }
          local_49 = 1;
          serde::de::Error::duplicate_field<serde_json::error::Error>();
          lVar10 = extraout_RAX_11;
        }
        else {
          if (cVar11 != '\x01') {
            if (!SBORROW8(0,local_58)) {
              local_49 = 1;
              serde::de::Error::duplicate_field<serde_json::error::Error>();
              uVar5 = false;
              lVar10 = extraout_RAX_10;
              goto joined_r0x0001415d8ffe;
            }
            local_f0 = '\x06';
            if (cVar12 == '\x06') {
              local_49 = 1;
              serde_json::error::impl$8::custom<ref_<str_>_>();
              lVar10 = extraout_RAX_08;
            }
            else {
              local_1a8 = cVar12;
              uStack_197 = uStack_df;
              lStack_190 = lStack_d8;
              uStack_1a7 = uStack_ef;
              uStack_1a0 = uStack_e8;
              uStack_19f = uStack_e7;
              uStack_198 = local_e0;
              local_49 = 1;
              de::impl$2::
              deserialize_struct<utils::curve::_::impl_0::deserialize::__Visitor<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>_>
                        ();
              lVar3 = CONCAT71(uStack_b7,local_b8);
              lVar10 = CONCAT71(uStack_af,uStack_b0);
              if (!SBORROW8(0,lVar3)) {
                local_1b0 = local_88;
                local_1d0 = CONCAT71(uStack_a7,local_a8);
                lStack_1c8 = lStack_a0;
                local_1c0 = local_98;
                uStack_1b8 = uStack_90;
                local_1e0 = lVar3;
                local_1d8 = lVar10;
                local_c8 = lVar10;
                local_58 = lVar3;
                goto LAB_1415d87c0;
              }
            }
            uVar5 = false;
            goto LAB_1415d8d94;
          }
          if (SBORROW8(0,local_68)) {
            local_f0 = '\x06';
            if (cVar12 == '\x06') {
              local_49 = 1;
              serde_json::error::impl$8::custom<ref_<str_>_>();
              lVar10 = extraout_RAX_07;
            }
            else {
              local_1a8 = cVar12;
              uStack_197 = uStack_df;
              lStack_190 = lStack_d8;
              uStack_1a7 = uStack_ef;
              uStack_1a0 = uStack_e8;
              uStack_19f = uStack_e7;
              uStack_198 = local_e0;
              local_49 = 1;
              de::impl$2::
              deserialize_struct<utils::curve::_::impl_0::deserialize::__Visitor<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>_>
                        ();
              lVar3 = CONCAT71(uStack_b7,local_b8);
              lVar10 = CONCAT71(uStack_af,uStack_b0);
              if (!SBORROW8(0,lVar3)) {
                local_140 = local_88;
                local_160 = CONCAT71(uStack_a7,local_a8);
                lStack_158 = lStack_a0;
                local_150 = local_98;
                uStack_148 = uStack_90;
                local_170 = lVar3;
                lStack_168 = lVar10;
                local_68 = lVar3;
                goto LAB_1415d87c0;
              }
            }
            uVar5 = false;
            goto joined_r0x0001415d8f6d;
          }
          local_49 = 1;
          serde::de::Error::duplicate_field<serde_json::error::Error>();
          lVar10 = extraout_RAX_12;
        }
      }
LAB_1415d8d13:
      uVar5 = false;
      goto joined_r0x0001415d8f6d;
    }
    local_2f8 = *(undefined8 *)in_RDX;
    lStack_2f0 = *(longlong *)(in_RDX + 8);
    local_2e8 = *(longlong *)(in_RDX + 0x10);
    uStack_2e0 = *(undefined8 *)(in_RDX + 0x18);
    Value::unexpected();
    serde_json::error::impl$8::invalid_type();
    in_RCX[1] = extraout_RAX_09;
    *in_RCX = &DAT_8000000000000010;
    local_4a = 1;
    core::ptr::drop_in_place<enum2_<serde_json::value::Value>_>();
  }
  return;
}

