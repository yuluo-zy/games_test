// ADDR 0x1412b6420
// SIG void __cdecl cull_stair_attached_decorators_impl<system_decorator::cull_oob_decorators::cull_stair_attached_decorators_on_invalid_stairs::closure_env$0>(void)
// ORIG system::cull_stair_attached_decorators_impl<system_decorator::cull_oob_decorators::cull_stair_attached_decorators_on_invalid_stairs::closure_env$0>


// WARNING: Removing unreachable block (ram,0x0001412b6970)
// WARNING: Removing unreachable block (ram,0x0001412b6980)
// WARNING: Removing unreachable block (ram,0x0001412b699b)
// WARNING: Removing unreachable block (ram,0x0001412b6975)
// WARNING: Removing unreachable block (ram,0x0001412b69aa)
// WARNING: Removing unreachable block (ram,0x0001412b69d9)
// WARNING: Removing unreachable block (ram,0x0001412b6b2e)
// WARNING: Removing unreachable block (ram,0x0001412b69ef)
// WARNING: Removing unreachable block (ram,0x0001412b6a2a)
// WARNING: Removing unreachable block (ram,0x0001412b6a30)
// WARNING: Removing unreachable block (ram,0x0001412b6a39)
// WARNING: Removing unreachable block (ram,0x0001412b6884)
// WARNING: Removing unreachable block (ram,0x0001412b6ae8)
// WARNING: Removing unreachable block (ram,0x0001412b6af2)
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl
system_decorator::cull_oob_decorators::
cull_stair_attached_decorators_impl<system_decorator::cull_oob_decorators::cull_stair_attached_decorators_on_invalid_stairs::closure_env_0>
          (void)

{
  undefined8 *puVar1;
  ushort uVar2;
  char extraout_AL;
  byte *extraout_RAX;
  longlong extraout_RAX_00;
  longlong lVar3;
  undefined8 *extraout_RAX_01;
  longlong lVar4;
  uint uVar5;
  longlong *in_RCX;
  longlong lVar6;
  uint uVar7;
  undefined8 in_RDX;
  undefined **ppuVar8;
  uint uVar9;
  uint uVar10;
  undefined1 (*pauVar11) [16];
  int iVar13;
  undefined1 auVar12 [16];
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
  pauVar11 = (undefined1 (*) [16])CONCAT44(uStack_1ac,local_1b0);
  local_80 = local_198;
  lVar3 = *in_RCX;
  uVar9 = (uint)local_178;
  local_90 = local_170;
  uVar10 = (uint)local_1a0;
  do {
    uVar7 = uVar10;
    if ((local_1c8 & 1) == 0) {
      if (pauStack_1b8 != (undefined1 (*) [16])0x0) {
        if (local_80 != 0) goto LAB_1412b661f;
        local_80 = 0;
      }
joined_r0x0001412b6668:
      if (local_190 == 0) {
        return;
      }
LAB_1412b666e:
      if (local_90 == 0) {
        return;
      }
      if ((short)uVar9 == 0) {
        do {
          auVar12 = *local_188;
          uVar2 = (ushort)(SUB161(auVar12 >> 7,0) & 1) | (ushort)(SUB161(auVar12 >> 0xf,0) & 1) << 1
                  | (ushort)(SUB161(auVar12 >> 0x17,0) & 1) << 2 |
                  (ushort)(SUB161(auVar12 >> 0x1f,0) & 1) << 3 |
                  (ushort)(SUB161(auVar12 >> 0x27,0) & 1) << 4 |
                  (ushort)(SUB161(auVar12 >> 0x2f,0) & 1) << 5 |
                  (ushort)(SUB161(auVar12 >> 0x37,0) & 1) << 6 |
                  (ushort)(SUB161(auVar12 >> 0x3f,0) & 1) << 7 |
                  (ushort)(SUB161(auVar12 >> 0x47,0) & 1) << 8 |
                  (ushort)(SUB161(auVar12 >> 0x4f,0) & 1) << 9 |
                  (ushort)(SUB161(auVar12 >> 0x57,0) & 1) << 10 |
                  (ushort)(SUB161(auVar12 >> 0x5f,0) & 1) << 0xb |
                  (ushort)(SUB161(auVar12 >> 0x67,0) & 1) << 0xc |
                  (ushort)(SUB161(auVar12 >> 0x6f,0) & 1) << 0xd |
                  (ushort)(SUB161(auVar12 >> 0x77,0) & 1) << 0xe |
                  (ushort)(byte)(auVar12[0xf] >> 7) << 0xf;
          local_190 = local_190 + -0x500;
          local_188 = local_188 + 1;
        } while (uVar2 == 0xffff);
        uVar10 = ~(uint)uVar2;
        uVar5 = uVar10 & -(uint)uVar2 - 2;
      }
      else {
        uVar5 = uVar9 - 1 & uVar9;
        uVar10 = uVar9;
      }
      local_90 = local_90 + -1;
      pauStack_1b8 = (undefined1 (*) [16])0x0;
      uVar9 = uVar5;
    }
    else if (pauStack_1b8 == (undefined1 (*) [16])0x0) {
LAB_1412b65eb:
      if (local_98 == 0) {
        local_98 = 0;
        if (local_190 == 0) {
          return;
        }
        goto LAB_1412b666e;
      }
      pauStack_1b8 = *(undefined1 (**) [16])(local_98 + 0x30);
      local_80 = *(longlong *)(local_98 + 0x48);
      auVar12 = *pauStack_1b8;
      uVar10 = ~(uint)(ushort)((ushort)(SUB161(auVar12 >> 7,0) & 1) |
                               (ushort)(SUB161(auVar12 >> 0xf,0) & 1) << 1 |
                               (ushort)(SUB161(auVar12 >> 0x17,0) & 1) << 2 |
                               (ushort)(SUB161(auVar12 >> 0x1f,0) & 1) << 3 |
                               (ushort)(SUB161(auVar12 >> 0x27,0) & 1) << 4 |
                               (ushort)(SUB161(auVar12 >> 0x2f,0) & 1) << 5 |
                               (ushort)(SUB161(auVar12 >> 0x37,0) & 1) << 6 |
                               (ushort)(SUB161(auVar12 >> 0x3f,0) & 1) << 7 |
                               (ushort)(SUB161(auVar12 >> 0x47,0) & 1) << 8 |
                               (ushort)(SUB161(auVar12 >> 0x4f,0) & 1) << 9 |
                               (ushort)(SUB161(auVar12 >> 0x57,0) & 1) << 10 |
                               (ushort)(SUB161(auVar12 >> 0x5f,0) & 1) << 0xb |
                               (ushort)(SUB161(auVar12 >> 0x67,0) & 1) << 0xc |
                               (ushort)(SUB161(auVar12 >> 0x6f,0) & 1) << 0xd |
                               (ushort)(SUB161(auVar12 >> 0x77,0) & 1) << 0xe |
                              (ushort)(byte)(auVar12[0xf] >> 7) << 0xf);
      pauVar11 = pauStack_1b8 + 1;
      if (local_80 == 0) {
        local_80 = 0;
        local_98 = 0;
        uVar7 = uVar10;
        goto joined_r0x0001412b6668;
      }
      local_98 = 0;
      if ((short)uVar10 != 0) goto LAB_1412b662f;
LAB_1412b6710:
      do {
        auVar12 = *pauVar11;
        uVar2 = (ushort)(SUB161(auVar12 >> 7,0) & 1) | (ushort)(SUB161(auVar12 >> 0xf,0) & 1) << 1 |
                (ushort)(SUB161(auVar12 >> 0x17,0) & 1) << 2 |
                (ushort)(SUB161(auVar12 >> 0x1f,0) & 1) << 3 |
                (ushort)(SUB161(auVar12 >> 0x27,0) & 1) << 4 |
                (ushort)(SUB161(auVar12 >> 0x2f,0) & 1) << 5 |
                (ushort)(SUB161(auVar12 >> 0x37,0) & 1) << 6 |
                (ushort)(SUB161(auVar12 >> 0x3f,0) & 1) << 7 |
                (ushort)(SUB161(auVar12 >> 0x47,0) & 1) << 8 |
                (ushort)(SUB161(auVar12 >> 0x4f,0) & 1) << 9 |
                (ushort)(SUB161(auVar12 >> 0x57,0) & 1) << 10 |
                (ushort)(SUB161(auVar12 >> 0x5f,0) & 1) << 0xb |
                (ushort)(SUB161(auVar12 >> 0x67,0) & 1) << 0xc |
                (ushort)(SUB161(auVar12 >> 0x6f,0) & 1) << 0xd |
                (ushort)(SUB161(auVar12 >> 0x77,0) & 1) << 0xe |
                (ushort)(byte)(auVar12[0xf] >> 7) << 0xf;
        pauStack_1b8 = pauStack_1b8 + -0x50;
        pauVar11 = pauVar11 + 1;
      } while (uVar2 == 0xffff);
      uVar10 = ~(uint)uVar2;
      uVar7 = uVar10 & -(uint)uVar2 - 2;
      local_80 = local_80 + -1;
    }
    else {
      if (local_80 == 0) {
        local_80 = 0;
        goto LAB_1412b65eb;
      }
LAB_1412b661f:
      if ((short)uVar10 == 0) goto LAB_1412b6710;
LAB_1412b662f:
      uVar7 = uVar10 - 1 & uVar10;
      local_80 = local_80 + -1;
    }
    for (; (uVar10 & 1) == 0; uVar10 = uVar10 >> 1 | 0x80000000) {
    }
    country_core::resources::walls::decorator_archivist::DecoratorArchivist::
    is_decorator_cull_allowed();
    uVar10 = uVar7;
    if (extraout_AL != '\0') {
      ppuVar8 = &PTR_s_crates_systems_decorator_src_cul_142c2bee0;
      country_core::resources::walls::decorator::DecoratorInfo::dst_as_stair();
      lVar4 = *(longlong *)(lVar3 + 8);
      lVar6 = *(longlong *)(lVar3 + 0x10) * 0x150;
      do {
        if (lVar6 == 0) {
          country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::
          try_position();
          hashbrown::map::HashMap::
          insert<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::cull_oob_decorators::cull_oob_decorators::DecoratorRemoveInfo,std::hash::random::RandomState,alloc::alloc::Global>
                    ();
          break;
        }
        puVar1 = (undefined8 *)(lVar4 + 0x128);
        lVar4 = lVar4 + 0x150;
        auVar12._0_4_ = -(uint)((int)*puVar1 == (int)*extraout_RAX_01);
        iVar13 = -(uint)((int)((ulonglong)*puVar1 >> 0x20) ==
                        (int)((ulonglong)*extraout_RAX_01 >> 0x20));
        auVar12._4_4_ = auVar12._0_4_;
        auVar12._8_4_ = iVar13;
        auVar12._12_4_ = iVar13;
        uVar7 = movmskpd((int)ppuVar8,auVar12);
        ppuVar8 = (undefined **)(ulonglong)uVar7;
        lVar6 = lVar6 + -0x150;
      } while (uVar7 != 3);
    }
  } while( true );
}


