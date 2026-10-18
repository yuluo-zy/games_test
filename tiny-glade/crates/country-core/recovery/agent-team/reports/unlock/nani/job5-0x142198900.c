
void __cdecl
bevy_ecs::bundle::Bundles::register_info<country_core::components::drawable::DrawableMeshBundle>
          (void)

{
  char *pcVar1;
  undefined8 *puVar2;
  longlong lVar3;
  ulonglong uVar4;
  uint uVar5;
  uint uVar6;
  longlong *in_RCX;
  uint uVar7;
  ulonglong uVar8;
  longlong lVar9;
  longlong lVar10;
  ushort uVar11;
  ulonglong uVar12;
  undefined1 auVar13 [16];
  undefined1 auVar14 [16];
  longlong lStack_a0;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  undefined8 uStack_80;
  undefined8 local_78;
  undefined8 uStack_70;
  undefined8 local_68;
  undefined8 uStack_60;
  undefined8 local_58;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  lVar3 = in_RCX[3];
  uVar4 = in_RCX[4];
  uVar8 = uVar4 & 0x3267461d1a0401c7;
  lVar10 = 0;
  do {
    pcVar1 = (char *)(lVar3 + uVar8);
    auVar14[0] = -(*pcVar1 == '\x19');
    auVar14[1] = -(pcVar1[1] == '\x19');
    auVar14[2] = -(pcVar1[2] == '\x19');
    auVar14[3] = -(pcVar1[3] == '\x19');
    auVar14[4] = -(pcVar1[4] == '\x19');
    auVar14[5] = -(pcVar1[5] == '\x19');
    auVar14[6] = -(pcVar1[6] == '\x19');
    auVar14[7] = -(pcVar1[7] == '\x19');
    auVar14[8] = -(pcVar1[8] == '\x19');
    auVar14[9] = -(pcVar1[9] == '\x19');
    auVar14[10] = -(pcVar1[10] == '\x19');
    auVar14[0xb] = -(pcVar1[0xb] == '\x19');
    auVar14[0xc] = -(pcVar1[0xc] == '\x19');
    auVar14[0xd] = -(pcVar1[0xd] == '\x19');
    auVar14[0xe] = -(pcVar1[0xe] == '\x19');
    auVar14[0xf] = -(pcVar1[0xf] == '\x19');
    uVar11 = (ushort)(SUB161(auVar14 >> 7,0) & 1) | (ushort)(SUB161(auVar14 >> 0xf,0) & 1) << 1 |
             (ushort)(SUB161(auVar14 >> 0x17,0) & 1) << 2 |
             (ushort)(SUB161(auVar14 >> 0x1f,0) & 1) << 3 |
             (ushort)(SUB161(auVar14 >> 0x27,0) & 1) << 4 |
             (ushort)(SUB161(auVar14 >> 0x2f,0) & 1) << 5 |
             (ushort)(SUB161(auVar14 >> 0x37,0) & 1) << 6 |
             (ushort)(SUB161(auVar14 >> 0x3f,0) & 1) << 7 |
             (ushort)(SUB161(auVar14 >> 0x47,0) & 1) << 8 |
             (ushort)(SUB161(auVar14 >> 0x4f,0) & 1) << 9 |
             (ushort)(SUB161(auVar14 >> 0x57,0) & 1) << 10 |
             (ushort)(SUB161(auVar14 >> 0x5f,0) & 1) << 0xb |
             (ushort)(SUB161(auVar14 >> 0x67,0) & 1) << 0xc |
             (ushort)(SUB161(auVar14 >> 0x6f,0) & 1) << 0xd |
             (ushort)(SUB161(auVar14 >> 0x77,0) & 1) << 0xe | (ushort)(auVar14[0xf] >> 7) << 0xf;
    uVar7 = (uint)uVar11;
    while (uVar11 != 0) {
      uVar5 = 0;
      for (uVar6 = uVar7; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
        uVar5 = uVar5 + 1;
      }
      uVar12 = uVar5 + uVar8 & uVar4;
      if ((*(longlong *)(lVar3 + -0x18 + uVar12 * -0x18) == 0x5676f5709402b437) &&
         (*(longlong *)(lVar3 + -0x10 + uVar12 * -0x18) == 0x3267461d1a0401c7)) {
        return;
      }
      uVar11 = (ushort)(uVar7 - 1) & (ushort)uVar7;
      uVar7 = CONCAT22((short)(uVar7 - 1 >> 0x10),uVar11);
    }
    auVar13[0] = -(*pcVar1 == -1);
    auVar13[1] = -(pcVar1[1] == -1);
    auVar13[2] = -(pcVar1[2] == -1);
    auVar13[3] = -(pcVar1[3] == -1);
    auVar13[4] = -(pcVar1[4] == -1);
    auVar13[5] = -(pcVar1[5] == -1);
    auVar13[6] = -(pcVar1[6] == -1);
    auVar13[7] = -(pcVar1[7] == -1);
    auVar13[8] = -(pcVar1[8] == -1);
    auVar13[9] = -(pcVar1[9] == -1);
    auVar13[10] = -(pcVar1[10] == -1);
    auVar13[0xb] = -(pcVar1[0xb] == -1);
    auVar13[0xc] = -(pcVar1[0xc] == -1);
    auVar13[0xd] = -(pcVar1[0xd] == -1);
    auVar13[0xe] = -(pcVar1[0xe] == -1);
    auVar13[0xf] = -(pcVar1[0xf] == -1);
    if ((((((((((((((((SUB161(auVar13 >> 7,0) & 1) != 0 || (SUB161(auVar13 >> 0xf,0) & 1) != 0) ||
                    (SUB161(auVar13 >> 0x17,0) & 1) != 0) || (SUB161(auVar13 >> 0x1f,0) & 1) != 0)
                  || (SUB161(auVar13 >> 0x27,0) & 1) != 0) || (SUB161(auVar13 >> 0x2f,0) & 1) != 0)
                || (SUB161(auVar13 >> 0x37,0) & 1) != 0) || (SUB161(auVar13 >> 0x3f,0) & 1) != 0) ||
              (SUB161(auVar13 >> 0x47,0) & 1) != 0) || (SUB161(auVar13 >> 0x4f,0) & 1) != 0) ||
            (SUB161(auVar13 >> 0x57,0) & 1) != 0) || (SUB161(auVar13 >> 0x5f,0) & 1) != 0) ||
          (SUB161(auVar13 >> 0x67,0) & 1) != 0) || (SUB161(auVar13 >> 0x6f,0) & 1) != 0) ||
        (SUB161(auVar13 >> 0x77,0) & 1) != 0) || auVar13[0xf] < '\0') {
      local_68 = 0;
      uStack_60 = 8;
      local_58 = 0;
      local_49 = 1;
      country_core::components::drawable::impl$3::
      component_ids<bevy_ecs::bundle::impl_8::register_info::closure_0::closure_env_0<country_core::components::drawable::DrawableMeshBundle>_>
                ();
      lVar3 = in_RCX[2];
      local_49 = 0;
      BundleInfo::new();
      if (lVar3 == *in_RCX) {
        alloc::raw_vec::RawVec::
        grow_one<enum2_<core::option::Option<tuple_<enum2_<serde::__private::de::content::Content>,enum2_<serde::__private::de::content::Content>_>_>_>,alloc::alloc::Global>
                  ();
      }
      lVar10 = in_RCX[1];
      lVar9 = lVar3 * 0x40;
      puVar2 = (undefined8 *)(lVar10 + 0x30 + lVar9);
      *puVar2 = local_78;
      puVar2[1] = uStack_70;
      puVar2 = (undefined8 *)(lVar10 + 0x20 + lVar9);
      *puVar2 = local_88;
      puVar2[1] = uStack_80;
      puVar2 = (undefined8 *)(lVar10 + 0x10 + lVar9);
      *puVar2 = local_98;
      puVar2[1] = uStack_90;
      *(longlong *)(lVar10 + lVar9) = (longlong)&local_68;
      ((longlong *)(lVar10 + lVar9))[1] = lStack_a0;
      in_RCX[2] = lVar3 + 1;
      hashbrown::raw::RawTable::
      insert<tuple_<core::any::TypeId,bevy_ecs::bundle::BundleId>,allocator_api2::stable::alloc::global::Global,hashbrown::map::make_hasher::closure_env_0<core::any::TypeId,bevy_ecs::bundle::BundleId,bevy_platform::hash::NoOpHash>_>
                ();
      return;
    }
    lVar9 = uVar8 + lVar10;
    lVar10 = lVar10 + 0x10;
    uVar8 = lVar9 + 0x10U & uVar4;
  } while( true );
}

