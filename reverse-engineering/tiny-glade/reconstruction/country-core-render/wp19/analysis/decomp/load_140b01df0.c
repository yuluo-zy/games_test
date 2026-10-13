// load  0x140b01df0
// signature: void __cdecl load(void)
// callees: [{"name": "__rust_dealloc", "address": "140613c20"}, {"name": "call_mut<tuple$<enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error>_>_>,country_core::resources::grading_lut::impl$3::load::closure_env$0>", "address": "140ae5bb0"}, {"name": "drop", "address": "140d685b0"}, {"name": "drop_slow<std::path::PathBuf,alloc::alloc::Global>", "address": "140d53bc0"}, {"name": "grow_one<enum2$<iced_core::event::Event>,alloc::alloc::Global>", "address": "1406319d0"}, {"name": "grow_one<indexmap::Bucket<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2$<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>", "address": "140562ab0"}, {"name": "memcpy", "address": "142923c00"}, {"name": "next", "address": "140d59e80"}, {"name": "readdir", "address": "140d68f80"}, {"name": "unwrap_failed", "address": "1428d9760"}]


void __cdecl country_core::resources::grading_lut::GradingLuts::load(void)

{
  undefined8 *puVar1;
  longlong *plVar2;
  longlong lVar3;
  longlong *in_RCX;
  longlong lVar4;
  longlong local_868;
  longlong *plStack_860;
  longlong local_858;
  longlong lStack_850;
  undefined8 local_848;
  undefined8 uStack_840;
  undefined8 local_838;
  longlong *local_610;
  longlong *local_608;
  longlong *local_600 [76];
  undefined1 local_3a0 [8];
  longlong local_398;
  longlong *plStack_390;
  longlong local_388;
  longlong lStack_380;
  undefined8 local_378;
  undefined8 uStack_370;
  undefined8 local_368;
  longlong local_128;
  undefined8 *local_120;
  undefined8 local_118;
  undefined8 uStack_110;
  undefined8 local_108;
  longlong local_100;
  longlong *plStack_f8;
  longlong local_f0;
  longlong lStack_e8;
  undefined8 local_e0;
  undefined8 uStack_d8;
  undefined8 local_d0;
  undefined8 local_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
  undefined1 *local_a8;
  longlong *local_a0;
  longlong local_98;
  longlong local_90;
  longlong lStack_88;
  longlong local_80;
  longlong local_78;
  longlong lStack_70;
  longlong local_68;
  undefined8 *local_60;
  longlong local_58;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  local_78 = 0;
  lStack_70 = 8;
  local_68 = 0;
  local_90 = 0;
  lStack_88 = 8;
  local_80 = 0;
  local_a0 = in_RCX;
  std::sys::pal::windows::fs::readdir();
  plVar2 = plStack_390;
  lVar4 = local_398;
  if (local_398 == 2) {
    local_610 = plStack_390;
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  memcpy(local_600,&local_388,0x260);
  local_610 = (longlong *)lVar4;
  local_608 = plVar2;
  do {
    local_a8 = local_3a0;
    do {
      std::fs::impl$30::next();
      if (local_398 != 1) {
        if (local_398 != 0) {
          if (plStack_390 == (longlong *)0x0) {
            if (((uint)local_388 & 3) == 1) {
              local_98 = local_388 + -1;
              local_58 = *(longlong *)(local_388 + -1);
              local_60 = *(undefined8 **)(local_388 + 7);
              if ((code *)*local_60 != (code *)0x0) {
                (*(code *)*local_60)(local_58);
              }
              if (local_60[1] != 0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
          }
          else {
            LOCK();
            *plStack_390 = *plStack_390 + -1;
            UNLOCK();
            if (*plStack_390 == 0) {
              alloc::sync::Arc::drop_slow<std::path::PathBuf,alloc::alloc::Global>();
            }
          }
        }
        if (local_610 != (longlong *)0x0) {
          std::sys::pal::windows::fs::impl$7::drop();
        }
        LOCK();
        *local_600[0] = *local_600[0] + -1;
        UNLOCK();
        if (*local_600[0] == 0) {
          alloc::sync::Arc::drop_slow<std::path::PathBuf,alloc::alloc::Global>();
        }
        local_a0[2] = local_68;
        *local_a0 = local_78;
        local_a0[1] = lStack_70;
        local_a0[3] = local_90;
        local_a0[4] = lStack_88;
        local_a0[5] = local_80;
        return;
      }
      memcpy(&local_868,&plStack_390,600);
      core::ops::function::impls::impl$3::
      call_mut<tuple_<enum2_<core::result::Result<std::fs::DirEntry,std::io::error::Error>_>_>,country_core::resources::grading_lut::impl_3::load::closure_env_0>
                ();
      lVar4 = local_68;
    } while (SBORROW8(0,local_128));
    local_58 = local_128;
    local_60 = local_120;
    local_c8 = local_118;
    uStack_c0 = uStack_110;
    local_b8 = local_108;
    local_838 = local_d0;
    local_848 = local_e0;
    uStack_840 = uStack_d8;
    local_858 = local_f0;
    lStack_850 = lStack_e8;
    local_868 = local_100;
    plStack_860 = plStack_f8;
    if (local_68 == local_78) {
      alloc::raw_vec::RawVec::
      grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                ();
    }
    lVar3 = local_80;
    *(longlong *)(lStack_70 + lVar4 * 0x28) = local_58;
    *(undefined8 **)(lStack_70 + 8 + lVar4 * 0x28) = local_60;
    puVar1 = (undefined8 *)(lStack_70 + 0x10 + lVar4 * 0x28);
    *puVar1 = local_c8;
    puVar1[1] = uStack_c0;
    *(undefined8 *)(lStack_70 + 0x20 + lVar4 * 0x28) = local_b8;
    local_68 = lVar4 + 1;
    local_368 = local_838;
    local_378 = local_848;
    uStack_370 = uStack_840;
    local_388 = local_858;
    lStack_380 = lStack_850;
    local_398 = local_868;
    plStack_390 = plStack_860;
    if (local_80 == local_90) {
      alloc::raw_vec::RawVec::grow_one<enum2_<iced_core::event::Event>,alloc::alloc::Global>();
    }
    lVar4 = lVar3 * 0x38;
    *(undefined8 *)(lStack_88 + 0x30 + lVar4) = local_368;
    puVar1 = (undefined8 *)(lStack_88 + 0x20 + lVar4);
    *puVar1 = local_378;
    puVar1[1] = uStack_370;
    plVar2 = (longlong *)(lStack_88 + 0x10 + lVar4);
    *plVar2 = local_388;
    plVar2[1] = lStack_380;
    *(longlong *)(lStack_88 + lVar4) = local_398;
    ((longlong *)(lStack_88 + lVar4))[1] = (longlong)plStack_390;
    local_80 = lVar3 + 1;
  } while( true );
}

