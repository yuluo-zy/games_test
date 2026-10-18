
void __cdecl
serde_json::value::de::
visit_array<country_core::resources::walls::inner_wall_state::_::impl_0::deserialize::__Visitor>
          (void)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  longlong lVar7;
  undefined4 uVar8;
  int iVar9;
  undefined4 uVar10;
  undefined4 uVar11;
  char cVar12;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong extraout_RAX_02;
  char cVar13;
  char cVar14;
  undefined1 uVar15;
  char cVar16;
  longlong *in_RCX;
  longlong lVar17;
  char cVar18;
  undefined4 uVar19;
  undefined4 uStack_144;
  undefined4 uStack_140;
  undefined4 uStack_13c;
  undefined4 uStack_138;
  undefined4 local_11e;
  undefined2 local_11a;
  undefined4 uStack_104;
  undefined1 uStack_100;
  undefined7 uStack_ff;
  byte *local_e0;
  longlong local_d8;
  byte *local_d0;
  byte local_c8;
  char cStack_c7;
  undefined1 uStack_c6;
  undefined1 uStack_c5;
  int iStack_c4;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined1 uStack_b8;
  undefined7 uStack_b7;
  longlong lStack_b0;
  longlong local_a8;
  longlong lStack_a0;
  longlong local_98;
  
  SeqDeserializer::new();
  if (local_e0 != local_d0) {
    bVar2 = *local_e0;
    if (bVar2 != 6) {
      uStack_104 = (undefined4)((ulonglong)*(undefined8 *)(local_e0 + 1) >> 0x18);
      uVar8 = uStack_104;
      uStack_100 = (undefined1)((ulonglong)*(undefined8 *)(local_e0 + 1) >> 0x38);
      uStack_ff = (undefined7)*(undefined8 *)(local_e0 + 9);
      ::utils::curve::curve2_serialized_as_curve3::deserialize<enum2_<serde_json::value::Value>_>();
      lVar7 = CONCAT44(iStack_c4,
                       CONCAT13(uStack_c5,CONCAT12(uStack_c6,CONCAT11(cStack_c7,local_c8))));
      lVar17 = CONCAT44(uStack_bc,uStack_c0);
      if (SBORROW8(0,lVar7)) goto LAB_1415b2446;
      if (local_e0 + 0x20 == local_d0) {
LAB_1415b2554:
        serde::de::Error::invalid_length<serde_json::error::Error>();
        lVar17 = extraout_RAX_00;
      }
      else {
        bVar3 = local_e0[0x20];
        if (bVar3 == 6) goto LAB_1415b2554;
        uVar4 = *(undefined8 *)(local_e0 + 0x21);
        uVar5 = *(undefined8 *)(local_e0 + 0x30);
        uVar6 = *(undefined8 *)(local_e0 + 0x38);
        cStack_c7 = (char)uVar4;
        uStack_c6 = (undefined1)((ulonglong)uVar4 >> 8);
        iStack_c4 = (int)((ulonglong)uVar4 >> 0x18);
        iVar9 = iStack_c4;
        uStack_c0._0_1_ = (undefined1)((ulonglong)uVar4 >> 0x38);
        uStack_c0._1_3_ = (undefined3)*(undefined8 *)(local_e0 + 0x29);
        uVar10 = uStack_c0;
        uStack_bc = (undefined4)((ulonglong)*(undefined8 *)(local_e0 + 0x29) >> 0x18);
        uVar11 = uStack_bc;
        impl$2::deserialize_f32<serde::de::impls::impl_76::deserialize::PrimitiveVisitor>();
        if ((bVar2 & 1) == 0) {
          if (local_e0 + 0x40 != local_d0) {
            pbVar1 = local_e0 + 0x60;
            bVar2 = local_e0[0x40];
            if (bVar2 != 6) {
              uStack_104 = (undefined4)((ulonglong)*(undefined8 *)(local_e0 + 0x41) >> 0x18);
              uStack_100 = (undefined1)((ulonglong)*(undefined8 *)(local_e0 + 0x41) >> 0x38);
              uStack_ff = (undefined7)*(undefined8 *)(local_e0 + 0x49);
              country_core::systems::wall::valid_enclosure::_::impl$0::
              deserialize<enum2_<serde_json::value::Value>_>();
              if ((bVar3 & 1) == 0) {
                if (iStack_c4 == 4) goto LAB_1415b260d;
                if (pbVar1 == local_d0) {
LAB_1415b26a7:
                  local_e0 = pbVar1;
                  uStack_104 = 0;
                  local_c8 = bVar3;
                }
                else {
                  local_c8 = *pbVar1;
                  pbVar1 = local_e0 + 0x80;
                  if (local_c8 == 6) goto LAB_1415b26a7;
                  uVar4 = *(undefined8 *)(local_e0 + 0x61);
                  cStack_c7 = (char)uVar4;
                  uStack_c6 = (undefined1)((ulonglong)uVar4 >> 8);
                  iStack_c4 = (int)((ulonglong)uVar4 >> 0x18);
                  uStack_c0._0_1_ = (undefined1)((ulonglong)uVar4 >> 0x38);
                  uStack_c0._1_3_ = (undefined3)*(undefined8 *)(local_e0 + 0x69);
                  uStack_bc = (undefined4)((ulonglong)*(undefined8 *)(local_e0 + 0x69) >> 0x18);
                  impl$2::deserialize_f32<serde::de::impls::impl_76::deserialize::PrimitiveVisitor>
                            ();
                  local_e0 = local_e0 + 0x80;
                  if ((bVar2 & 1) != 0) goto LAB_1415b25df;
                }
                serde::de::impl$6::next_element<serde_json::value::de::SeqDeserializer,f32>();
                if ((local_c8 & 1) == 0) {
                  uVar19 = uStack_c0;
                  if ((char)iStack_c4 == '\0') {
                    uVar19 = 0x40333333;
                  }
                  serde::de::impl$6::
                  next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::inner_wall_state::ArchType>
                            ();
                  if (local_c8 == 0) {
                    cVar18 = '\0';
                    if (cStack_c7 != '\x02') {
                      cVar18 = cStack_c7;
                    }
                    serde::de::impl$6::
                    next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::inner_wall_state::_::impl_0::deserialize::impl_2::visit_seq::__DeserializeWith>
                              ();
                    cVar13 = '\0';
                    if (cStack_c7 != '\x03') {
                      cVar13 = cStack_c7;
                    }
                    serde::de::impl$6::
                    next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::inner_wall_state::StructureType>
                              ();
                    cVar14 = '\0';
                    if (cStack_c7 != '\x02') {
                      cVar14 = cStack_c7;
                    }
                    serde::de::impl$6::
                    next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::decorator::ClutterVersion>
                              ();
                    uVar15 = 0;
                    if (cStack_c7 != '\0') {
                      uVar15 = uStack_c6;
                    }
                    serde::de::impl$6::
                    next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::inner_wall_state::ArchType>
                              ();
                    cVar16 = '\0';
                    if (cStack_c7 != '\x02') {
                      cVar16 = cStack_c7;
                    }
                    serde::de::impl$6::
                    next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::inner_wall_state::PlasterPeelAmount>
                              ();
                    if ((char)iStack_c4 == '\0') {
                      uStack_c0 = 0;
                    }
                    serde::de::impl$6::next_element<serde_json::value::de::SeqDeserializer,bool>();
                    uStack_144 = (undefined4)uVar5;
                    uStack_140 = (undefined4)((ulonglong)uVar5 >> 0x20);
                    uStack_13c = (undefined4)uVar6;
                    uStack_138 = (undefined4)((ulonglong)uVar6 >> 0x20);
                    cVar12 = '\0';
                    if (cStack_c7 != '\x02') {
                      cVar12 = cStack_c7;
                    }
                    if (local_d0 == local_e0) {
                      *(undefined4 *)(in_RCX + 0xc) = uStack_c0;
                      *(char *)((longlong)in_RCX + 100) = cVar18;
                      *(char *)((longlong)in_RCX + 0x65) = cVar14;
                      *(char *)((longlong)in_RCX + 0x66) = cVar16;
                      *(char *)((longlong)in_RCX + 0x67) = cVar12;
                      *(char *)(in_RCX + 0xd) = cVar13;
                      *(undefined1 *)((longlong)in_RCX + 0x69) = uVar15;
                      *(undefined4 *)((longlong)in_RCX + 0x6a) = local_11e;
                      *(undefined2 *)((longlong)in_RCX + 0x6e) = local_11a;
                      in_RCX[10] = CONCAT44(uVar8,uStack_138);
                      in_RCX[0xb] = CONCAT44(uVar19,uStack_104);
                      in_RCX[8] = CONCAT44(uStack_144,uVar11);
                      in_RCX[9] = CONCAT44(uStack_13c,uStack_140);
                      in_RCX[6] = local_98;
                      in_RCX[7] = CONCAT44(uVar10,iVar9);
                      in_RCX[4] = local_a8;
                      in_RCX[5] = lStack_a0;
                      in_RCX[2] = CONCAT71(uStack_b7,uStack_b8);
                      in_RCX[3] = lStack_b0;
                      *in_RCX = lVar7;
                      in_RCX[1] = lVar17;
                    }
                    else {
                      serde::de::Error::invalid_length<serde_json::error::Error>();
                      in_RCX[1] = extraout_RAX_02;
                      *in_RCX = -0x8000000000000000;
                      core::ptr::
                      drop_in_place<utils::curve::Curve<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>_>
                                ();
                    }
                    core::ptr::drop_in_place<serde_json::value::de::SeqDeserializer>();
                    return;
                  }
                }
              }
              lVar17 = CONCAT44(uStack_bc,uStack_c0);
              goto LAB_1415b256f;
            }
          }
LAB_1415b260d:
          serde::de::Error::invalid_length<serde_json::error::Error>();
          lVar17 = extraout_RAX_01;
        }
        else {
LAB_1415b25df:
          lVar17 = CONCAT71(uStack_ff,uStack_100);
        }
      }
LAB_1415b256f:
      if (lVar7 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if (lStack_b0 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      goto LAB_1415b2446;
    }
  }
  serde::de::Error::invalid_length<serde_json::error::Error>();
  lVar17 = extraout_RAX;
LAB_1415b2446:
  in_RCX[1] = lVar17;
  *in_RCX = -0x8000000000000000;
  core::ptr::drop_in_place<slice2_<enum2_<serde_json::value::Value>_>_>();
  if (local_d8 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}

