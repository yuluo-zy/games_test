// extract_diff  size=854
// void __cdecl extract_diff(void)

void __cdecl
country_core::resources::walls::decorator_archivist::DecoratorArchivist::extract_diff(void)

{
  longlong lVar1;
  undefined1 auVar2 [16];
  ulonglong uVar3;
  ulonglong uVar4;
  longlong *extraout_RAX;
  undefined8 *in_RCX;
  undefined8 *in_RDX;
  undefined8 *puVar5;
  undefined1 in_R9B;
  undefined8 local_278;
  undefined8 *puStack_270;
  undefined1 (*local_268) [16];
  undefined1 (*pauStack_260) [16];
  undefined1 (*local_258) [16];
  undefined1 *puStack_250;
  undefined8 local_248;
  undefined8 uStack_240;
  undefined8 local_1e0;
  undefined1 local_1d8 [144];
  undefined1 (*local_148) [16];
  undefined1 (*local_140) [16];
  undefined1 *local_138;
  ushort local_130;
  undefined8 local_128;
  undefined8 local_120;
  undefined8 uStack_118;
  undefined8 local_110;
  undefined8 uStack_108;
  undefined8 *local_f0;
  undefined1 (*local_e8) [16];
  longlong local_e0;
  undefined8 local_d0;
  undefined *local_b8;
  undefined8 uStack_b0;
  undefined8 local_a8;
  undefined8 uStack_a0;
  longlong local_98;
  longlong lStack_90;
  undefined8 local_88;
  undefined8 *puStack_80;
  undefined1 (*local_78) [16];
  undefined1 (*pauStack_70) [16];
  undefined1 (*local_68) [16];
  undefined1 *puStack_60;
  ushort local_58;
  undefined6 uStack_56;
  undefined8 uStack_50;
  undefined1 local_42;
  undefined1 local_41;
  undefined8 local_40;
  
  local_40 = 0xfffffffffffffffe;
  local_88 = CONCAT71(local_88._1_7_,in_R9B);
  hashbrown::raw::RawTable::
  clear<tuple_<country_core::resources::walls::decorator::DecoratorId,tuple_<country_core::systems::collision::world::OffsetColliderTransform,country_core::systems::collision::world::RaycastMesh>_>,alloc::alloc::Global>
            ();
  hashbrown::raw::RawTable::
  clear<tuple_<country_core::resources::walls::decorator::DecoratorId,tuple_<country_core::systems::collision::world::OffsetColliderTransform,country_core::systems::collision::world::RaycastMesh>_>,alloc::alloc::Global>
            ();
  memcpy(local_1d8,in_RDX + 0x18,0x90);
  in_RDX[0x18] = 0;
  memcpy(&puStack_270,in_RDX + 0x2a,0x90);
  in_RDX[0x2a] = 0;
  local_148 = (undefined1 (*) [16])*in_RDX;
  local_138 = *local_148 + in_RDX[1] + 1;
  auVar2 = *local_148;
  local_130 = ~((ushort)(SUB161(auVar2 >> 7,0) & 1) | (ushort)(SUB161(auVar2 >> 0xf,0) & 1) << 1 |
                (ushort)(SUB161(auVar2 >> 0x17,0) & 1) << 2 |
                (ushort)(SUB161(auVar2 >> 0x1f,0) & 1) << 3 |
                (ushort)(SUB161(auVar2 >> 0x27,0) & 1) << 4 |
                (ushort)(SUB161(auVar2 >> 0x2f,0) & 1) << 5 |
                (ushort)(SUB161(auVar2 >> 0x37,0) & 1) << 6 |
                (ushort)(SUB161(auVar2 >> 0x3f,0) & 1) << 7 |
                (ushort)(SUB161(auVar2 >> 0x47,0) & 1) << 8 |
                (ushort)(SUB161(auVar2 >> 0x4f,0) & 1) << 9 |
                (ushort)(SUB161(auVar2 >> 0x57,0) & 1) << 10 |
                (ushort)(SUB161(auVar2 >> 0x5f,0) & 1) << 0xb |
                (ushort)(SUB161(auVar2 >> 0x67,0) & 1) << 0xc |
                (ushort)(SUB161(auVar2 >> 0x6f,0) & 1) << 0xd |
                (ushort)(SUB161(auVar2 >> 0x77,0) & 1) << 0xe |
               (ushort)(byte)(auVar2[0xf] >> 7) << 0xf);
  local_128 = in_RDX[3];
  local_110 = in_RDX[2];
  uStack_108 = in_RDX[3];
  local_120 = *in_RDX;
  uStack_118 = in_RDX[1];
  in_RDX[2] = 0;
  in_RDX[3] = 0;
  *in_RDX = &anon_6652254e5a159f1dca07c38add8ba93c_0_llvm_3554112267207565025;
  in_RDX[1] = 0;
  local_140 = local_148 + 1;
  local_278 = 1;
  local_1e0 = 1;
  local_f0 = &local_88;
  puVar5 = &local_278;
  std::collections::hash::map::impl$87::
  from_iter<enum2_<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple_<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl_0::extract_archived_decorators::closure_env_0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl_0::extract_archived_decorators::closure_env_1>_>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl_0::extract_archived_decorators::closure_env_2>_>,country_core::resources::walls::decorator_archivist::impl_0::extract_before_after_all::closure_env_0>_>
            ();
  if (in_RDX[3] != 0) {
    local_41 = 1;
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  if (in_RDX[0x18] != 0) {
    local_41 = 1;
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  if (in_RDX[0x2a] != 0) {
    local_41 = 1;
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  if (in_RDX[9] != 0) {
    uVar3 = in_RDX[7];
    if (uVar3 != 0) {
      puVar5 = (undefined8 *)CONCAT71((int7)((ulonglong)puVar5 >> 8),0xff);
      memset((void *)in_RDX[6],(int)puVar5,uVar3 + 0x11);
    }
    in_RDX[9] = 0;
    uVar4 = (uVar3 + 1 & 0xfffffffffffffff8) - (uVar3 + 1 >> 3);
    if (uVar3 < 8) {
      uVar4 = uVar3;
    }
    in_RDX[8] = uVar4;
  }
  *(undefined1 *)(in_RDX + 0x3c) = 0;
  lVar1 = local_e0 + 1;
  auVar2 = *local_e8;
  if (local_e0 == 0) {
    local_88 = 0;
    local_78 = (undefined1 (*) [16])0x0;
  }
  else {
    puVar5 = (undefined8 *)(local_e0 + lVar1 * 0x70 + 0x11);
    local_78 = local_e8 + lVar1 * -7;
    local_88 = 0x10;
  }
  local_68 = local_e8 + 1;
  local_58 = ~((ushort)(SUB161(auVar2 >> 7,0) & 1) | (ushort)(SUB161(auVar2 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar2 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar2 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar2 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar2 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar2 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar2 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar2 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar2 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar2 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar2 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar2 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar2 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar2 >> 0x77,0) & 1) << 0xe |
              (ushort)(byte)(auVar2[0xf] >> 7) << 0xf);
  puStack_60 = *local_e8 + lVar1;
  pauStack_70 = local_e8;
  uStack_50 = local_d0;
  local_42 = 1;
  puStack_80 = puVar5;
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX != (longlong *)0x0) {
    local_98 = *extraout_RAX;
    lStack_90 = extraout_RAX[1];
    *extraout_RAX = *extraout_RAX + 1;
    local_a8 = 0;
    uStack_a0 = 0;
    local_b8 = &anon_2ad33cd064e2b7a85895a07d96050156_27_llvm_3451300626392288586;
    uStack_b0 = 0;
    local_248 = CONCAT62(uStack_56,local_58);
    uStack_240 = uStack_50;
    local_258 = local_68;
    puStack_250 = puStack_60;
    local_268 = local_78;
    pauStack_260 = pauStack_70;
    local_278 = local_88;
    puStack_270 = puStack_80;
    hashbrown::map::impl$82::
    extend<enum2_<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::IntoIter<enum2_<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple_<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>_>,country_core::resources::walls::decorator_archivist::impl_0::extract_diff::closure_env_0>_>
              ();
    in_RCX[4] = local_98;
    in_RCX[5] = lStack_90;
    in_RCX[2] = local_a8;
    in_RCX[3] = uStack_a0;
    *in_RCX = local_b8;
    in_RCX[1] = uStack_b0;
    return;
  }
  local_42 = 1;
                    // WARNING: Subroutine does not return
  std::thread::local::panic_access_error();
}



// CALLEES: [{'name': 'call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64>_>_>_>_>_>_>_>_>', 'address': '1400d9b20'}, {'name': 'clear<tuple$<country_core::resources::walls::decorator::DecoratorId,tuple$<country_core::systems::collision::world::OffsetColliderTransform,country_core::systems::collision::world::RaycastMesh>_>,alloc::alloc::Global>', 'address': '140b79960'}, {'name': 'extend<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::IntoIter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>_>,country_core::resources::walls::decorator_archivist::impl$0::extract_diff::closure_env$0>_>', 'address': '1415237c0'}, {'name': 'from_iter<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,tuple$<country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState,country_core::resources::history::decorator_edit::DecoratorStorageAddrHistoryState>,std::hash::random::RandomState,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Drain<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$0>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$1>_>,core::iter::adapters::map::Map<core::option::IntoIter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_archivist::impl$0::extract_archived_decorators::closure_env$2>_>,country_core::resources::walls::decorator_archivist::impl$0::extract_before_after_all::closure_env$0>_>', 'address': '141449c20'}, {'name': 'memcpy', 'address': '142923c00'}, {'name': 'memset', 'address': '142923c20'}, {'name': 'panic', 'address': '1428d9430'}, {'name': 'panic_access_error', 'address': '1428d80d0'}]