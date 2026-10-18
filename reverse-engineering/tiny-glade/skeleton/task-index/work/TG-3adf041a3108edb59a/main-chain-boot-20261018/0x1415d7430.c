
void __cdecl
serde_json::value::de::impl$5::
struct_variant<country_core::resources::history::wall_compound_edit::_::impl_0::deserialize::impl_2::visit_enum::__Visitor>
          (void)

{
  undefined8 *puVar1;
  byte bVar2;
  longlong lVar3;
  longlong lVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 extraout_RAX;
  undefined8 extraout_RAX_00;
  longlong extraout_RAX_01;
  undefined8 extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong extraout_RAX_04;
  longlong extraout_RAX_05;
  longlong extraout_RAX_06;
  longlong extraout_RAX_07;
  longlong lVar8;
  longlong extraout_RAX_08;
  longlong extraout_RAX_09;
  longlong extraout_RAX_10;
  undefined8 extraout_RAX_11;
  longlong extraout_RAX_12;
  longlong extraout_RAX_13;
  longlong extraout_RAX_14;
  undefined8 extraout_RAX_15;
  undefined8 *in_RCX;
  byte *in_RDX;
  char cVar9;
  undefined1 *local_288;
  longlong lStack_280;
  undefined8 local_278;
  undefined8 uStack_270;
  undefined8 local_188;
  undefined8 uStack_180;
  undefined8 local_178;
  ulonglong local_168;
  undefined8 local_158;
  longlong local_110;
  char local_108;
  undefined7 uStack_107;
  undefined1 uStack_100;
  undefined7 uStack_ff;
  undefined1 local_f8;
  undefined7 uStack_f7;
  undefined8 uStack_f0;
  uint local_e8;
  undefined4 uStack_e4;
  longlong local_e0;
  longlong local_d8;
  undefined8 uStack_d0;
  ulonglong local_c8;
  longlong local_c0;
  char local_b8;
  undefined7 uStack_b7;
  undefined1 uStack_b0;
  undefined7 uStack_af;
  undefined1 local_a8;
  undefined7 uStack_a7;
  undefined8 uStack_a0;
  ulonglong local_98;
  longlong local_90;
  uint local_84;
  longlong *local_80;
  uint local_78;
  undefined1 local_71;
  undefined8 local_70;
  char cVar10;
  
  local_70 = 0xfffffffffffffffe;
  bVar2 = *in_RDX;
  local_168 = (ulonglong)bVar2;
  if (bVar2 == 6) {
    local_288 = (undefined1 *)CONCAT71(local_288._1_7_,0xd);
    local_71 = 0;
    serde_json::error::impl$8::invalid_type();
    in_RCX[1] = extraout_RAX;
    *in_RCX = &DAT_8000000000000010;
  }
  else if (bVar2 == 5) {
    local_188 = *(undefined8 *)(in_RDX + 8);
    uStack_180 = *(undefined8 *)(in_RDX + 0x10);
    local_178 = *(undefined8 *)(in_RDX + 0x18);
    local_71 = 0;
    local_158 = local_178;
    MapDeserializer::new();
    local_c8 = 0;
    local_84 = (uint)CONCAT71((int7)((ulonglong)extraout_RAX_00 >> 8),1);
    local_78 = 0;
    local_98 = 0;
LAB_1415d7540:
    do {
      alloc::collections::btree::map::IntoIter::
      dying_next<alloc::string::String,enum2_<serde_json::value::Value>,alloc::alloc::Global>();
      lVar8 = CONCAT44(uStack_e4,local_e8);
      if (lVar8 == 0) {
LAB_1415d78fd:
        if ((local_98 & 1) == 0) {
          serde::de::Error::missing_field<serde_json::error::Error>();
          lVar8 = extraout_RAX_04;
        }
        else if ((local_78 & 1) == 0) {
          serde::de::Error::missing_field<serde_json::error::Error>();
          lVar8 = extraout_RAX_06;
        }
        else {
          if ((local_c8 & 1) != 0) {
            local_288 = &DAT_8000000000000002;
            lStack_280 = local_c0;
            if (local_110 == 0) {
              memcpy(in_RCX,&local_288,0xf8);
            }
            else {
              serde::de::Error::invalid_length<serde_json::error::Error>();
              in_RCX[1] = extraout_RAX_15;
              *in_RCX = &DAT_8000000000000010;
              core::ptr::
              drop_in_place<enum2_<country_core::resources::history::wall_compound_edit::WallHistoryEdit>_>
                        ();
            }
            core::ptr::drop_in_place<serde_json::map::IntoIter>();
            local_71 = 0;
            core::ptr::
            drop_in_place<enum2_<core::option::Option<enum2_<serde_json::value::Value>_>_>_>();
            return;
          }
          serde::de::Error::missing_field<serde_json::error::Error>();
          lVar8 = extraout_RAX_07;
        }
        break;
      }
      lVar3 = *(longlong *)(lVar8 + 0x168 + local_d8 * 0x18);
      local_80 = *(longlong **)(lVar8 + 0x170 + local_d8 * 0x18);
      lVar4 = *(longlong *)(lVar8 + 0x178 + local_d8 * 0x18);
      puVar1 = (undefined8 *)(lVar8 + local_d8 * 0x20);
      uVar5 = *puVar1;
      uVar6 = puVar1[1];
      puVar1 = (undefined8 *)(lVar8 + 0x10 + local_d8 * 0x20);
      uVar7 = *puVar1;
      uStack_a0 = puVar1[1];
      local_b8 = (char)uVar5;
      uStack_b7 = (undefined7)((ulonglong)uVar5 >> 8);
      uStack_b0 = (undefined1)uVar6;
      uStack_af = (undefined7)((ulonglong)uVar6 >> 8);
      local_a8 = (undefined1)uVar7;
      uStack_a7 = (undefined7)((ulonglong)uVar7 >> 8);
      if (SBORROW8(0,lVar3)) goto LAB_1415d78fd;
      local_90 = lVar3;
      switch(local_108) {
      case '\0':
      case '\x01':
      case '\x02':
      case '\x06':
        break;
      case '\x03':
        if (CONCAT71(uStack_ff,uStack_100) != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        break;
      case '\x04':
        uStack_d0 = CONCAT71(uStack_f7,local_f8);
        core::ptr::drop_in_place<slice2_<enum2_<serde_json::value::Value>_>_>();
        if (CONCAT71(uStack_ff,uStack_100) != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        break;
      default:
        core::ptr::
        drop_in_place<serde_json::map::Map<alloc::string::String,enum2_<serde_json::value::Value>_>_>
                  ();
      }
      local_f8 = local_a8;
      uStack_f7 = uStack_a7;
      uStack_f0 = uStack_a0;
      local_108 = local_b8;
      uStack_107 = uStack_b7;
      uStack_100 = uStack_b0;
      uStack_ff = uStack_af;
      if (lVar4 == 5) {
        if (*(char *)((longlong)local_80 + 4) != 'r' || (int)*local_80 != 0x65746661)
        goto LAB_1415d769b;
        cVar10 = '\x02';
        cVar9 = '\x02';
      }
      else if (lVar4 == 6) {
        if (*(short *)((longlong)local_80 + 4) != 0x6572 || (int)*local_80 != 0x6f666562)
        goto LAB_1415d769b;
        cVar10 = '\x01';
        cVar9 = '\x01';
      }
      else if ((lVar4 == 10) && ((short)local_80[1] == 0x7865 && *local_80 == 0x646e695f6c6c6177)) {
        cVar10 = '\0';
        cVar9 = '\0';
      }
      else {
LAB_1415d769b:
        cVar10 = '\x03';
        cVar9 = '\x03';
      }
      lVar8 = local_90;
      if (local_90 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
        cVar9 = cVar10;
      }
      cVar10 = local_108;
      if (cVar9 != '\x03') {
        if (cVar9 == '\x02') {
          if ((local_84 & 1) == 0) {
            serde::de::Error::duplicate_field<serde_json::error::Error>();
            lVar8 = extraout_RAX_12;
            break;
          }
          local_108 = '\x06';
          if (cVar10 == '\x06') {
            serde_json::error::impl$8::custom<ref_<str_>_>();
            lVar8 = extraout_RAX_08;
            break;
          }
          local_b8 = cVar10;
          uStack_a7 = uStack_f7;
          uStack_a0 = uStack_f0;
          uStack_b7 = uStack_107;
          uStack_b0 = uStack_100;
          uStack_af = uStack_ff;
          local_a8 = local_f8;
          de::impl$2::deserialize_f32<serde::de::impls::impl_76::deserialize::PrimitiveVisitor>();
          lVar8 = local_e0;
          if ((local_e8 & 1) != 0) break;
          local_c8 = CONCAT71((int7)((ulonglong)extraout_RAX_02 >> 8),1);
          local_84 = 0;
        }
        else if (cVar9 == '\x01') {
          if (local_78 != 0) {
            serde::de::Error::duplicate_field<serde_json::error::Error>();
            lVar8 = extraout_RAX_14;
            break;
          }
          local_108 = '\x06';
          if (cVar10 == '\x06') {
            serde_json::error::impl$8::custom<ref_<str_>_>();
            lVar8 = extraout_RAX_10;
            break;
          }
          local_b8 = cVar10;
          uStack_a7 = uStack_f7;
          uStack_a0 = uStack_f0;
          uStack_b7 = uStack_107;
          uStack_b0 = uStack_100;
          uStack_af = uStack_ff;
          local_a8 = local_f8;
          de::impl$2::deserialize_f32<serde::de::impls::impl_76::deserialize::PrimitiveVisitor>();
          lVar8 = local_e0;
          if ((local_e8 & 1) != 0) break;
          local_78 = 1;
        }
        else {
          if (local_98 != 0) {
            serde::de::Error::duplicate_field<serde_json::error::Error>();
            lVar8 = extraout_RAX_13;
            break;
          }
          local_108 = '\x06';
          if (cVar10 == '\x06') {
            serde_json::error::impl$8::custom<ref_<str_>_>();
            lVar8 = extraout_RAX_09;
            break;
          }
          local_b8 = cVar10;
          uStack_a7 = uStack_f7;
          uStack_a0 = uStack_f0;
          uStack_b7 = uStack_107;
          uStack_b0 = uStack_100;
          uStack_af = uStack_ff;
          local_a8 = local_f8;
          de::impl$2::deserialize_u64<serde::de::impls::impl_75::deserialize::PrimitiveVisitor>();
          local_c0 = lVar8;
          if (extraout_RAX_03 != 0) break;
          local_98 = 1;
        }
        goto LAB_1415d7540;
      }
      local_108 = '\x06';
      if (cVar10 == '\x06') {
        serde_json::error::impl$8::custom<ref_<str_>_>();
        lVar8 = extraout_RAX_05;
        break;
      }
      local_b8 = cVar10;
      uStack_a7 = uStack_f7;
      uStack_a0 = uStack_f0;
      uStack_b7 = uStack_107;
      uStack_b0 = uStack_100;
      uStack_af = uStack_ff;
      local_a8 = local_f8;
      de::impl$2::deserialize_ignored_any<serde::de::ignored_any::IgnoredAny>();
      lVar8 = extraout_RAX_01;
    } while (extraout_RAX_01 == 0);
    in_RCX[1] = lVar8;
    *in_RCX = &DAT_8000000000000010;
    core::ptr::drop_in_place<serde_json::map::IntoIter>();
    local_71 = 0;
    core::ptr::drop_in_place<enum2_<core::option::Option<enum2_<serde_json::value::Value>_>_>_>();
  }
  else {
    local_288 = *(undefined1 **)in_RDX;
    lStack_280 = *(longlong *)(in_RDX + 8);
    local_278 = *(undefined8 *)(in_RDX + 0x10);
    uStack_270 = *(undefined8 *)(in_RDX + 0x18);
    Value::unexpected();
    serde_json::error::impl$8::invalid_type();
    in_RCX[1] = extraout_RAX_11;
    *in_RCX = &DAT_8000000000000010;
    local_71 = 1;
    core::ptr::drop_in_place<enum2_<serde_json::value::Value>_>();
  }
  return;
}

