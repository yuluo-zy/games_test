
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl
country_core::resources::walls::decorator_storage::Decorators::
tracked_clone_from<country_core::resources::walls::decorator_storage::DecoMoveToStorage>(void)

{
  ushort uVar1;
  uint uVar2;
  undefined8 uVar3;
  undefined1 (*_Dst) [16];
  uint uVar4;
  uint uVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  longlong extraout_RAX;
  longlong in_RCX;
  undefined8 *in_RDX;
  undefined8 *puVar8;
  undefined1 (*pauVar9) [16];
  undefined1 (*pauVar10) [16];
  longlong lVar11;
  longlong in_R8;
  undefined1 auVar12 [16];
  undefined **local_138;
  code *local_130;
  undefined ***local_128;
  undefined8 local_120;
  undefined8 local_118;
  longlong local_f0;
  undefined **local_e8;
  code *pcStack_e0;
  undefined ***local_d8;
  code *pcStack_d0;
  longlong local_c8;
  code *local_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined4 local_a8;
  undefined4 local_a4;
  undefined8 local_98;
  ulonglong local_90;
  undefined8 *local_88;
  ulonglong local_80;
  uint local_74;
  longlong local_70;
  undefined1 (*local_68) [16];
  undefined4 local_5c;
  undefined8 *local_58;
  longlong local_50;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  lVar11 = *(longlong *)(in_RCX + 0x48);
  local_70 = in_R8;
  local_58 = in_RDX;
  if ((*(longlong *)(in_RCX + 0x18) == 0 && *(longlong *)(in_RCX + 0x78) == 0) && lVar11 == 0) {
    hashbrown::raw::impl$13::
    clone_from<tuple_<country_core::resources::walls::decorator::DecoratorId,tuple_<>_>,alloc::alloc::Global>
              ();
    uVar3 = in_RDX[0x11];
    *(undefined8 *)(in_RCX + 0x80) = in_RDX[0x10];
    *(undefined8 *)(in_RCX + 0x88) = uVar3;
    hashbrown::raw::impl$13::
    clone_from<tuple_<country_core::resources::walls::decorator::DecoratorId,tuple_<>_>,alloc::alloc::Global>
              ();
    uVar3 = in_RDX[5];
    *(undefined8 *)(in_RCX + 0x20) = in_RDX[4];
    *(undefined8 *)(in_RCX + 0x28) = uVar3;
    hashbrown::raw::impl$13::
    clone_from<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,alloc::alloc::Global>
              ();
    uVar3 = in_RDX[0xb];
    *(undefined8 *)(in_RCX + 0x50) = in_RDX[10];
    *(undefined8 *)(in_RCX + 0x58) = uVar3;
  }
  else {
    puVar8 = in_RDX;
    if (*(longlong *)(in_RCX + 0x78) != 0) {
      uVar7 = *(ulonglong *)(in_RCX + 0x68);
      if (uVar7 != 0) {
        puVar8 = (undefined8 *)CONCAT71((int7)((ulonglong)in_RDX >> 8),0xff);
        memset(*(void **)(in_RCX + 0x60),(int)puVar8,uVar7 + 0x11);
        lVar11 = *(longlong *)(in_RCX + 0x48);
      }
      *(undefined8 *)(in_RCX + 0x78) = 0;
      uVar6 = (uVar7 + 1 & 0xfffffffffffffff8) - (uVar7 + 1 >> 3);
      if (uVar7 < 8) {
        uVar6 = uVar7;
      }
      *(ulonglong *)(in_RCX + 0x70) = uVar6;
    }
    local_88 = (undefined8 *)(in_RCX + 0x30);
    pauVar9 = *(undefined1 (**) [16])(in_RCX + 0x30);
    auVar12 = *pauVar9;
    local_f0 = in_RCX + 0x60;
    if (lVar11 == 0) {
      lVar11 = 0;
    }
    else {
      pauVar10 = pauVar9 + 1;
      uVar4 = ~(uint)(ushort)((ushort)(SUB161(auVar12 >> 7,0) & 1) |
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
      do {
        if ((short)uVar4 == 0) {
          do {
            auVar12 = *pauVar10;
            uVar1 = (ushort)(SUB161(auVar12 >> 7,0) & 1) |
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
                    (ushort)(byte)(auVar12[0xf] >> 7) << 0xf;
            pauVar9 = pauVar9 + -0x50;
            pauVar10 = pauVar10 + 1;
          } while (uVar1 == 0xffff);
          uVar5 = ~(uint)uVar1;
          uVar4 = ~(uint)uVar1 & -(uint)uVar1 - 2;
        }
        else {
          uVar5 = uVar4;
          uVar4 = uVar4 - 1 & uVar4;
        }
        uVar2 = 0;
        for (; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
          uVar2 = uVar2 + 1;
        }
        puVar8 = (undefined8 *)(ulonglong)*(uint *)pauVar9[(ulonglong)uVar2 * -5 + -5];
        hashbrown::map::HashMap::
        insert<country_core::resources::walls::decorator::DecoratorId,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
                  ();
        lVar11 = lVar11 + -1;
      } while (lVar11 != 0);
      pauVar9 = *(undefined1 (**) [16])(in_RCX + 0x30);
      lVar11 = *(longlong *)(in_RCX + 0x48);
      auVar12 = *pauVar9;
    }
    uVar3 = _UNK_142af4bc8;
    uVar7 = *(ulonglong *)(in_RCX + 0x38);
    pauVar10 = pauVar9 + 1;
    local_88[2] = DAT_142af4bc0;
    local_88[3] = uVar3;
    uVar3 = _UNK_142af4bb8;
    *local_88 = anon_6652254e5a159f1dca07c38add8ba93c_1_llvm_3554112267207565025;
    local_88[1] = uVar3;
    uVar4 = ~(uint)(ushort)((ushort)(SUB161(auVar12 >> 7,0) & 1) |
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
    _Dst = pauVar9;
    local_50 = in_RCX;
    while (local_80 = uVar7, local_68 = _Dst, lVar11 != 0) {
      lVar11 = lVar11 + -1;
      if ((short)uVar4 == 0) {
        do {
          auVar12 = *pauVar10;
          uVar1 = (ushort)(SUB161(auVar12 >> 7,0) & 1) | (ushort)(SUB161(auVar12 >> 0xf,0) & 1) << 1
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
          pauVar9 = pauVar9 + -0x50;
          pauVar10 = pauVar10 + 1;
        } while (uVar1 == 0xffff);
        uVar4 = ~(uint)uVar1;
        uVar5 = uVar4 & -(uint)uVar1 - 2;
      }
      else {
        uVar5 = uVar4 - 1 & uVar4;
      }
      uVar2 = 0;
      for (; (uVar4 & 1) == 0; uVar4 = uVar4 >> 1 | 0x80000000) {
        uVar2 = uVar2 + 1;
      }
      if (pauVar9[(ulonglong)uVar2 * -5 + -1][0xf] == '\x02') break;
      uVar7 = (ulonglong)local_98 >> 0x20;
      local_98 = CONCAT44((int)uVar7,*(undefined4 *)pauVar9[(ulonglong)uVar2 * -5 + -5]);
      puVar8 = *(undefined8 **)(local_70 + 0x10);
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
      hashbrown::raw::RawTable::
      remove_entry<tuple_<country_core::resources::walls::decorator::DecoratorId,enum2_<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>_>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,enum2_<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>_>_>
                ();
      uVar7 = local_80;
      uVar4 = uVar5;
      _Dst = local_68;
      if ((int)pcStack_e0 == 3) {
        local_130 = decorator::impl$27::fmt;
        local_e8 = &anon_087c9b6d159f545cda87747878cb277f_48_llvm_10190951805537787575;
        pcStack_e0 = (code *)0x1;
        local_c8 = 0;
        local_d8 = &local_138;
        pcStack_d0 = (code *)0x1;
        local_138 = (undefined **)&local_98;
                    /* WARNING: Subroutine does not return */
        core::panicking::panic_fmt();
      }
    }
    if (uVar7 == 0) {
      uVar6 = 0;
    }
    else {
      memset(_Dst,(int)CONCAT71((int7)((ulonglong)puVar8 >> 8),0xff),uVar7 + 0x11);
      uVar6 = (uVar7 + 1 & 0xfffffffffffffff8) - (uVar7 + 1 >> 3);
      if (uVar7 < 8) {
        uVar6 = uVar7;
      }
    }
    *(undefined1 (**) [16])(local_50 + 0x30) = _Dst;
    *(ulonglong *)(local_50 + 0x38) = uVar7;
    *(ulonglong *)(local_50 + 0x40) = uVar6;
    *(undefined8 *)(local_50 + 0x48) = 0;
    lVar11 = local_58[3];
    if (lVar11 != 0) {
      pauVar9 = (undefined1 (*) [16])*local_58 + 1;
      auVar12 = *(undefined1 (*) [16])*local_58;
      uVar4 = ~(uint)(ushort)((ushort)(SUB161(auVar12 >> 7,0) & 1) |
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
      do {
        uVar5 = uVar4;
        if ((short)uVar4 == 0) {
          do {
            auVar12 = *pauVar9;
            pauVar9 = pauVar9 + 1;
            uVar4 = (uint)(ushort)((ushort)(SUB161(auVar12 >> 7,0) & 1) |
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
          } while (uVar4 == 0xffff);
          uVar4 = ~uVar4;
          uVar5 = uVar4;
        }
        for (; (uVar4 & 1) == 0; uVar4 = uVar4 >> 1 | 0x80000000) {
        }
        uVar4 = uVar5 - 1 & uVar5;
        hashbrown::map::HashMap::
        insert<country_core::resources::walls::decorator::DecoratorId,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
                  ();
        lVar11 = lVar11 + -1;
      } while (lVar11 != 0);
    }
    lVar11 = local_58[9];
    if (lVar11 != 0) {
      pauVar10 = (undefined1 (*) [16])local_58[6];
      pauVar9 = pauVar10 + 1;
      auVar12 = *pauVar10;
      uVar4 = ~(uint)(ushort)((ushort)(SUB161(auVar12 >> 7,0) & 1) |
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
      do {
        if ((short)uVar4 == 0) {
          do {
            auVar12 = *pauVar9;
            uVar1 = (ushort)(SUB161(auVar12 >> 7,0) & 1) |
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
                    (ushort)(byte)(auVar12[0xf] >> 7) << 0xf;
            pauVar10 = pauVar10 + -0x50;
            pauVar9 = pauVar9 + 1;
          } while (uVar1 == 0xffff);
          uVar5 = ~(uint)uVar1;
          uVar4 = ~(uint)uVar1 & -(uint)uVar1 - 2;
        }
        else {
          uVar5 = uVar4;
          uVar4 = uVar4 - 1 & uVar4;
        }
        uVar2 = 0;
        for (; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
          uVar2 = uVar2 + 1;
        }
        uVar7 = (ulonglong)uVar2;
        local_5c = *(undefined4 *)pauVar10[uVar7 * -5 + -5];
        local_c0 = *(code **)pauVar10[uVar7 * -5 + -2];
        local_a4 = *(undefined4 *)(pauVar10[uVar7 * -5 + -1] + 0xc);
        local_a8 = *(undefined4 *)(pauVar10[uVar7 * -5 + -1] + 8);
        local_b8 = *(undefined8 *)(pauVar10[uVar7 * -5 + -2] + 8);
        uStack_b0 = *(undefined8 *)((longlong)(pauVar10[uVar7 * -5 + -2] + 8) + 8);
        local_e8 = *(undefined ***)(pauVar10[uVar7 * -5 + -5] + 8);
        pcStack_e0 = *(code **)((longlong)(pauVar10[uVar7 * -5 + -5] + 8) + 8);
        local_d8 = *(undefined ****)(pauVar10[uVar7 * -5 + -4] + 8);
        pcStack_d0 = *(code **)((longlong)(pauVar10[uVar7 * -5 + -4] + 8) + 8);
        local_c8 = *(longlong *)(pauVar10[uVar7 * -5 + -3] + 8);
        hashbrown::map::HashMap::
        insert<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,std::hash::random::RandomState,alloc::alloc::Global>
                  ();
        core::hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
        hashbrown::raw::RawTable::
        remove_entry<tuple_<country_core::resources::walls::decorator::DecoratorId,tuple_<>_>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,tuple_<>_>_>
                  ();
        hashbrown::map::HashMap::
        insert<country_core::resources::walls::decorator::DecoratorId,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
                  ();
        lVar11 = lVar11 + -1;
      } while (lVar11 != 0);
    }
  }
  lVar11 = local_58[9];
  if (lVar11 != 0) {
    pauVar10 = (undefined1 (*) [16])local_58[6];
    pauVar9 = pauVar10 + 1;
    auVar12 = *pauVar10;
    uVar4 = ~(uint)(ushort)((ushort)(SUB161(auVar12 >> 7,0) & 1) |
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
    do {
      if ((short)uVar4 == 0) {
        do {
          auVar12 = *pauVar9;
          uVar1 = (ushort)(SUB161(auVar12 >> 7,0) & 1) | (ushort)(SUB161(auVar12 >> 0xf,0) & 1) << 1
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
          pauVar10 = pauVar10 + -0x50;
          pauVar9 = pauVar9 + 1;
        } while (uVar1 == 0xffff);
        uVar5 = ~(uint)uVar1;
        uVar4 = uVar5 & -(uint)uVar1 - 2;
      }
      else {
        uVar5 = uVar4;
        uVar4 = uVar4 - 1 & uVar4;
      }
      uVar2 = 0;
      for (; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
        uVar2 = uVar2 + 1;
      }
      local_74 = *(uint *)pauVar10[(ulonglong)uVar2 * -5 + -5];
      uVar7 = (ulonglong)local_74;
      hashbrown::map::HashMap::
      insert<country_core::resources::walls::decorator::DecoratorId,enum2_<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,std::hash::random::RandomState,alloc::alloc::Global>
                ();
      if (extraout_RAX != 3) {
        local_e8 = (undefined **)&local_74;
        pcStack_e0 = decorator::impl$27::fmt;
        local_d8 = (undefined ***)&local_98;
        pcStack_d0 = decorator_storage::impl$25::fmt;
        local_c8 = local_70;
        local_c0 = decorator_storage::impl$25::fmt;
        local_138 = &anon_087c9b6d159f545cda87747878cb277f_52_llvm_10190951805537787575;
        local_130 = (code *)0x3;
        local_118 = 0;
        local_128 = &local_e8;
        local_120 = 3;
        local_90 = uVar7;
                    /* WARNING: Subroutine does not return */
        core::panicking::panic_fmt();
      }
      lVar11 = lVar11 + -1;
    } while (lVar11 != 0);
  }
  return;
}

