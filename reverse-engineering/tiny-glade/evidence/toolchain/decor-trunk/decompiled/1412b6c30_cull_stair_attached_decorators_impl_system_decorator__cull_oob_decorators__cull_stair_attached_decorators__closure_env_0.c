// ADDR 0x1412b6c30
// SIG void __cdecl cull_stair_attached_decorators_impl<system_decorator::cull_oob_decorators::cull_stair_attached_decorators::closure_env$0>(void)
// ORIG system::cull_stair_attached_decorators_impl<system_decorator::cull_oob_decorators::cull_stair_attached_decorators::closure_env$0>


// WARNING: Removing unreachable block (ram,0x0001412b7142)
// WARNING: Removing unreachable block (ram,0x0001412b7150)
// WARNING: Removing unreachable block (ram,0x0001412b7160)
// WARNING: Removing unreachable block (ram,0x0001412b717b)
// WARNING: Removing unreachable block (ram,0x0001412b7155)
// WARNING: Removing unreachable block (ram,0x0001412b718a)
// WARNING: Removing unreachable block (ram,0x0001412b71b9)
// WARNING: Removing unreachable block (ram,0x0001412b730e)
// WARNING: Removing unreachable block (ram,0x0001412b71cf)
// WARNING: Removing unreachable block (ram,0x0001412b720a)
// WARNING: Removing unreachable block (ram,0x0001412b7210)
// WARNING: Removing unreachable block (ram,0x0001412b7219)
// WARNING: Removing unreachable block (ram,0x0001412b7057)
// WARNING: Removing unreachable block (ram,0x0001412b72c8)
// WARNING: Removing unreachable block (ram,0x0001412b72d2)
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl
system_decorator::cull_oob_decorators::
cull_stair_attached_decorators_impl<system_decorator::cull_oob_decorators::cull_stair_attached_decorators::closure_env_0>
          (void)

{
  undefined1 auVar1 [16];
  ushort uVar2;
  char extraout_AL;
  char extraout_AL_00;
  byte *extraout_RAX;
  longlong extraout_RAX_00;
  longlong lVar3;
  uint uVar4;
  uint uVar5;
  undefined8 in_RDX;
  uint uVar6;
  uint uVar7;
  undefined1 (*pauVar8) [16];
  ulonglong local_1c8;
  longlong local_1c0;
  undefined1 (*pauStack_1b8) [16];
  undefined4 local_1b0;
  undefined4 uStack_1ac;
  ushort local_1a0;
  longlong local_198;
  longlong local_190;
  undefined1 (*local_188) [16];
  ushort local_178;
  longlong local_170;
  longlong local_98;
  longlong local_90;
  longlong local_80;
  
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX & 1) == 0) {
    std::sys::random::hashmap_random_keys();
    extraout_RAX[0] = 1;
    extraout_RAX[1] = 0;
    extraout_RAX[2] = 0;
    extraout_RAX[3] = 0;
    extraout_RAX[4] = 0;
    extraout_RAX[5] = 0;
    extraout_RAX[6] = 0;
    extraout_RAX[7] = 0;
    *(longlong *)(extraout_RAX + 8) = extraout_RAX_00;
    *(undefined8 *)(extraout_RAX + 0x10) = in_RDX;
    lVar3 = extraout_RAX_00;
  }
  else {
    lVar3 = *(longlong *)(extraout_RAX + 8);
  }
  *(longlong *)(extraout_RAX + 8) = lVar3 + 1;
  country_core::resources::walls::decorator_storage::DecoratorStorage::iter_stair_decorators();
  local_98 = local_1c0;
  pauVar8 = (undefined1 (*) [16])CONCAT44(uStack_1ac,local_1b0);
  local_80 = local_198;
  uVar6 = (uint)local_178;
  local_90 = local_170;
  uVar7 = (uint)local_1a0;
  do {
    uVar5 = uVar7;
    if ((local_1c8 & 1) == 0) {
      if (pauStack_1b8 != (undefined1 (*) [16])0x0) {
        if (local_80 != 0) goto LAB_1412b6e2f;
        local_80 = 0;
      }
joined_r0x0001412b6e78:
      if (local_190 == 0) {
        return;
      }
LAB_1412b6e7e:
      if (local_90 == 0) {
        return;
      }
      if ((short)uVar6 == 0) {
        do {
          auVar1 = *local_188;
          uVar2 = (ushort)(SUB161(auVar1 >> 7,0) & 1) | (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1 |
                  (ushort)(SUB161(auVar1 >> 0x17,0) & 1) << 2 |
                  (ushort)(SUB161(auVar1 >> 0x1f,0) & 1) << 3 |
                  (ushort)(SUB161(auVar1 >> 0x27,0) & 1) << 4 |
                  (ushort)(SUB161(auVar1 >> 0x2f,0) & 1) << 5 |
                  (ushort)(SUB161(auVar1 >> 0x37,0) & 1) << 6 |
                  (ushort)(SUB161(auVar1 >> 0x3f,0) & 1) << 7 |
                  (ushort)(SUB161(auVar1 >> 0x47,0) & 1) << 8 |
                  (ushort)(SUB161(auVar1 >> 0x4f,0) & 1) << 9 |
                  (ushort)(SUB161(auVar1 >> 0x57,0) & 1) << 10 |
                  (ushort)(SUB161(auVar1 >> 0x5f,0) & 1) << 0xb |
                  (ushort)(SUB161(auVar1 >> 0x67,0) & 1) << 0xc |
                  (ushort)(SUB161(auVar1 >> 0x6f,0) & 1) << 0xd |
                  (ushort)(SUB161(auVar1 >> 0x77,0) & 1) << 0xe |
                  (ushort)(byte)(auVar1[0xf] >> 7) << 0xf;
          local_190 = local_190 + -0x500;
          local_188 = local_188 + 1;
        } while (uVar2 == 0xffff);
        uVar7 = ~(uint)uVar2;
        uVar4 = uVar7 & -(uint)uVar2 - 2;
      }
      else {
        uVar4 = uVar6 - 1 & uVar6;
        uVar7 = uVar6;
      }
      local_90 = local_90 + -1;
      pauStack_1b8 = (undefined1 (*) [16])0x0;
      uVar6 = uVar4;
    }
    else if (pauStack_1b8 == (undefined1 (*) [16])0x0) {
LAB_1412b6dfb:
      if (local_98 == 0) {
        local_98 = 0;
        if (local_190 == 0) {
          return;
        }
        goto LAB_1412b6e7e;
      }
      pauStack_1b8 = *(undefined1 (**) [16])(local_98 + 0x30);
      local_80 = *(longlong *)(local_98 + 0x48);
      auVar1 = *pauStack_1b8;
      uVar7 = ~(uint)(ushort)((ushort)(SUB161(auVar1 >> 7,0) & 1) |
                              (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1 |
                              (ushort)(SUB161(auVar1 >> 0x17,0) & 1) << 2 |
                              (ushort)(SUB161(auVar1 >> 0x1f,0) & 1) << 3 |
                              (ushort)(SUB161(auVar1 >> 0x27,0) & 1) << 4 |
                              (ushort)(SUB161(auVar1 >> 0x2f,0) & 1) << 5 |
                              (ushort)(SUB161(auVar1 >> 0x37,0) & 1) << 6 |
                              (ushort)(SUB161(auVar1 >> 0x3f,0) & 1) << 7 |
                              (ushort)(SUB161(auVar1 >> 0x47,0) & 1) << 8 |
                              (ushort)(SUB161(auVar1 >> 0x4f,0) & 1) << 9 |
                              (ushort)(SUB161(auVar1 >> 0x57,0) & 1) << 10 |
                              (ushort)(SUB161(auVar1 >> 0x5f,0) & 1) << 0xb |
                              (ushort)(SUB161(auVar1 >> 0x67,0) & 1) << 0xc |
                              (ushort)(SUB161(auVar1 >> 0x6f,0) & 1) << 0xd |
                              (ushort)(SUB161(auVar1 >> 0x77,0) & 1) << 0xe |
                             (ushort)(byte)(auVar1[0xf] >> 7) << 0xf);
      pauVar8 = pauStack_1b8 + 1;
      if (local_80 == 0) {
        local_80 = 0;
        local_98 = 0;
        uVar5 = uVar7;
        goto joined_r0x0001412b6e78;
      }
      local_98 = 0;
      if ((short)uVar7 != 0) goto LAB_1412b6e3f;
LAB_1412b6f20:
      do {
        auVar1 = *pauVar8;
        uVar2 = (ushort)(SUB161(auVar1 >> 7,0) & 1) | (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1 |
                (ushort)(SUB161(auVar1 >> 0x17,0) & 1) << 2 |
                (ushort)(SUB161(auVar1 >> 0x1f,0) & 1) << 3 |
                (ushort)(SUB161(auVar1 >> 0x27,0) & 1) << 4 |
                (ushort)(SUB161(auVar1 >> 0x2f,0) & 1) << 5 |
                (ushort)(SUB161(auVar1 >> 0x37,0) & 1) << 6 |
                (ushort)(SUB161(auVar1 >> 0x3f,0) & 1) << 7 |
                (ushort)(SUB161(auVar1 >> 0x47,0) & 1) << 8 |
                (ushort)(SUB161(auVar1 >> 0x4f,0) & 1) << 9 |
                (ushort)(SUB161(auVar1 >> 0x57,0) & 1) << 10 |
                (ushort)(SUB161(auVar1 >> 0x5f,0) & 1) << 0xb |
                (ushort)(SUB161(auVar1 >> 0x67,0) & 1) << 0xc |
                (ushort)(SUB161(auVar1 >> 0x6f,0) & 1) << 0xd |
                (ushort)(SUB161(auVar1 >> 0x77,0) & 1) << 0xe |
                (ushort)(byte)(auVar1[0xf] >> 7) << 0xf;
        pauStack_1b8 = pauStack_1b8 + -0x50;
        pauVar8 = pauVar8 + 1;
      } while (uVar2 == 0xffff);
      uVar7 = ~(uint)uVar2;
      uVar5 = uVar7 & -(uint)uVar2 - 2;
      local_80 = local_80 + -1;
    }
    else {
      if (local_80 == 0) {
        local_80 = 0;
        goto LAB_1412b6dfb;
      }
LAB_1412b6e2f:
      if ((short)uVar7 == 0) goto LAB_1412b6f20;
LAB_1412b6e3f:
      uVar5 = uVar7 - 1 & uVar7;
      local_80 = local_80 + -1;
    }
    for (; (uVar7 & 1) == 0; uVar7 = uVar7 >> 1 | 0x80000000) {
    }
    country_core::resources::walls::decorator_archivist::DecoratorArchivist::
    is_decorator_cull_allowed();
    uVar7 = uVar5;
    if (extraout_AL != '\0') {
      country_core::resources::walls::decorator::DecoratorInfo::dst_as_stair();
      country_core::resources::stairs::stairs_state::StairsState::contains_edge_or_stub();
      if (extraout_AL_00 == '\0') {
        country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::
        try_position();
        hashbrown::map::HashMap::
        insert<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::cull_oob_decorators::cull_oob_decorators::DecoratorRemoveInfo,std::hash::random::RandomState,alloc::alloc::Global>
                  ();
      }
    }
  } while( true );
}


