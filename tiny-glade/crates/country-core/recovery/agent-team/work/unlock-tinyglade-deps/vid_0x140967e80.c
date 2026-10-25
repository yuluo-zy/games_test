
void __cdecl
serde::de::impls::impl$19::deserialize::impl$0::
visit_seq<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>,ref_mut_<serde_json::value::de::SeqDeserializer>_>
          (void)

{
  undefined8 *puVar1;
  ulonglong extraout_RAX;
  ulonglong extraout_RAX_00;
  ulonglong uVar2;
  char *pcVar3;
  ulonglong *in_RCX;
  ulonglong uVar4;
  longlong lVar5;
  ulonglong in_RDX;
  ulonglong local_130;
  undefined8 local_128;
  undefined8 uStack_120;
  undefined8 local_118;
  undefined8 uStack_110;
  undefined8 local_108;
  undefined8 uStack_100;
  undefined8 uStack_f8;
  undefined6 uStack_f0;
  undefined2 local_ea;
  undefined6 uStack_e8;
  undefined8 uStack_e2;
  char local_da;
  undefined1 local_d9;
  ulonglong local_50;
  
  uVar2 = in_RDX;
  serde_json::value::de::impl$7::size_hint();
  uVar4 = 0x2e8b;
  if (uVar2 < 0x2e8b) {
    uVar4 = uVar2;
  }
  uVar2 = 0;
  if ((extraout_RAX & 1) != 0) {
    uVar2 = uVar4;
  }
  if (uVar2 == 0) {
    uVar2 = 8;
    uVar4 = 0;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    uVar2 = extraout_RAX_00;
    if (extraout_RAX_00 == 0) {
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
  }
  local_50 = 0;
  pcVar3 = *(char **)(in_RDX + 8);
  if (pcVar3 != *(char **)(in_RDX + 0x18)) {
    do {
      *(char **)(in_RDX + 8) = pcVar3 + 0x20;
      if (*pcVar3 == '\x06') break;
      serde_json::value::de::impl$2::
      deserialize_tuple<serde::de::impls::impl_188::deserialize::TupleVisitor<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>_>
                ();
      if (local_da == '\x02') {
        in_RCX[1] = local_130;
        *in_RCX = 0x8000000000000000;
        if (uVar4 == 0) {
          return;
        }
        std::alloc::__default_lib_allocator::__rust_dealloc();
        return;
      }
      if (local_50 == uVar4) {
        alloc::raw_vec::RawVec::
        grow_one<gpu_profiler::backend::ash::ProfilerOutputBuffer<rhapsody::raw::profiler::ProfilerBuffer>,alloc::alloc::Global>
                  ();
      }
      lVar5 = local_50 * 0x58;
      *(ulonglong *)(uVar2 + lVar5) = local_130;
      puVar1 = (undefined8 *)(uVar2 + 8 + lVar5);
      *puVar1 = local_128;
      puVar1[1] = uStack_120;
      puVar1 = (undefined8 *)(uVar2 + 0x18 + lVar5);
      *puVar1 = local_118;
      puVar1[1] = uStack_110;
      puVar1 = (undefined8 *)(uVar2 + 0x28 + lVar5);
      *puVar1 = local_108;
      puVar1[1] = uStack_100;
      puVar1 = (undefined8 *)(uVar2 + 0x38 + lVar5);
      *puVar1 = uStack_f8;
      puVar1[1] = CONCAT26(local_ea,uStack_f0);
      puVar1 = (undefined8 *)(uVar2 + 0x46 + lVar5);
      *puVar1 = CONCAT62(uStack_e8,local_ea);
      puVar1[1] = uStack_e2;
      *(char *)(uVar2 + 0x56 + lVar5) = local_da;
      *(undefined1 *)(uVar2 + 0x57 + lVar5) = local_d9;
      local_50 = local_50 + 1;
      pcVar3 = *(char **)(in_RDX + 8);
    } while (pcVar3 != *(char **)(in_RDX + 0x18));
  }
  in_RCX[2] = local_50;
  *in_RCX = uVar4;
  in_RCX[1] = uVar2;
  return;
}

