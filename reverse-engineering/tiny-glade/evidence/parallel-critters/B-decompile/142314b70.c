
void __cdecl system_critters::duck::update_duck_ripples(void)

{
  undefined8 *puVar1;
  float fVar2;
  undefined8 uVar3;
  undefined *puVar4;
  float fVar5;
  int iVar6;
  undefined8 *extraout_RAX;
  longlong lVar7;
  ulonglong uVar8;
  longlong *in_RCX;
  ulonglong uVar9;
  longlong lVar10;
  longlong *in_R9;
  undefined *local_558 [66];
  undefined4 local_348;
  undefined8 local_344;
  undefined4 local_33c;
  undefined8 local_338;
  undefined1 *local_330;
  float *local_328;
  float *local_320;
  longlong local_318;
  longlong local_310;
  longlong local_308;
  longlong local_300;
  longlong local_2f8;
  longlong local_2f0;
  undefined8 local_2e8;
  undefined8 local_2e0;
  undefined8 local_2d8;
  undefined8 local_2d0;
  undefined8 uStack_2c8;
  undefined8 local_2c0;
  undefined8 uStack_2b8;
  undefined8 local_2b0;
  undefined1 local_2a8;
  uint local_128;
  undefined4 uStack_124;
  longlong lStack_120;
  undefined8 local_118;
  undefined8 uStack_110;
  longlong local_108;
  undefined8 uStack_100;
  undefined8 local_f8;
  undefined8 uStack_f0;
  undefined8 local_e8;
  undefined8 uStack_e0;
  longlong lStack_d0;
  undefined8 local_c8;
  undefined8 uStack_c0;
  longlong local_b8;
  undefined8 uStack_b0;
  undefined8 local_a8;
  undefined8 uStack_a0;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined8 local_80;
  undefined8 local_78;
  longlong local_70;
  longlong local_68;
  ulonglong local_60;
  float local_58;
  float local_54;
  undefined8 local_50;
  
  local_50 = 0xfffffffffffffffe;
  fVar2 = *(float *)(*in_R9 + 0x28);
  local_54 = expf(__real_c0200000 * fVar2);
  fVar5 = __real_3f800000;
  local_54 = __real_3f800000 - local_54;
  local_58 = expf(fVar2 * __real_c0a00000);
  local_58 = fVar5 - local_58;
  local_300 = *in_RCX;
  local_318 = in_RCX[1];
  local_310 = local_318 + 0x1a0;
  local_308 = local_318 + 0xf8;
  local_2f8 = *(longlong *)(local_300 + 0xf8);
  local_2f0 = local_2f8 + *(longlong *)(local_300 + 0x100) * 4;
  local_2a8 = *(undefined1 *)(local_300 + 0x108);
  local_2c0 = 0;
  uStack_2b8 = 0;
  local_2d0 = 0;
  uStack_2c8 = 0;
  local_2b0 = 0;
  local_338 = 0x20;
  local_330 = &stack0x00000028;
  local_328 = &local_58;
  local_320 = &local_54;
  local_2e8 = 8;
  local_2e0 = 0;
  local_2d8 = 8;
  alloc::vec::spec_from_iter::impl$0::
  from_iter<array_<u32,4>,core::iter::adapters::take::Take<core::iter::adapters::filter_map::FilterMap<bevy_ecs::query::iter::QueryIter<tuple_<bevy_ecs::entity::Entity,ref_<utils::components::transform::Transform>,ref_<enum2_<country_core::resources::critters::DuckState>_>_>,tuple_<>_>,system_critters::duck::update_duck_ripples::closure_env_0>_>_>
            ();
  frozen::FrozenMap::
  get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
            ();
  if (extraout_RAX != (undefined8 *)0x0) {
    local_558[2] = &anon_75d1013106e08612ad9ec328065f3af1_0_llvm_9468065370954387650;
    local_558[3] = (undefined *)0x30;
    if ((extraout_RAX[3] != 0x30) ||
       (iVar6 = memcmp(&anon_75d1013106e08612ad9ec328065f3af1_0_llvm_9468065370954387650,
                       (void *)extraout_RAX[2],0x30), iVar6 != 0)) {
      local_338 = 0;
                    /* WARNING: Subroutine does not return */
      core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
    }
    lVar7 = (**(code **)(extraout_RAX[1] + 0x18))(*extraout_RAX);
    if (lVar7 != 0) {
      memset(local_558 + 2,0,0x200);
      local_348 = (undefined4)local_60;
      local_344 = 0;
      local_33c = 0;
      uVar8 = 0x20;
      if (local_60 < 0x20) {
        uVar8 = local_60;
      }
      if (local_60 != 0) {
        if (local_60 == 1) {
          uVar9 = 0;
        }
        else {
          lVar10 = 0x10;
          uVar9 = 0;
          do {
            puVar1 = (undefined8 *)(local_68 + -0x10 + lVar10);
            uVar3 = puVar1[1];
            *(undefined8 *)((longlong)local_558 + lVar10) = *puVar1;
            *(undefined8 *)((longlong)local_558 + lVar10 + 8) = uVar3;
            uVar9 = uVar9 + 2;
            uVar3 = ((undefined8 *)(local_68 + lVar10))[1];
            *(undefined8 *)((longlong)local_558 + lVar10 + 0x10) =
                 *(undefined8 *)(local_68 + lVar10);
            *(undefined8 *)((longlong)local_558 + lVar10 + 0x18) = uVar3;
            lVar10 = lVar10 + 0x20;
          } while (((uint)uVar8 & 0x3e) != uVar9);
        }
        if ((uVar8 & 1) != 0) {
          puVar1 = (undefined8 *)(local_68 + uVar9 * 0x10);
          puVar4 = (undefined *)puVar1[1];
          local_558[uVar9 * 2 + 2] = (undefined *)*puVar1;
          local_558[uVar9 * 2 + 3] = puVar4;
        }
      }
      country_core::render::impl$0::deref();
      memcpy(&local_338,local_558 + 2,0x210);
      if (0x20f < *(ulonglong *)(lVar7 + 0x18)) {
        local_80 = 0;
        local_78 = 0x210;
        local_558[1] = (undefined *)0x0;
        local_558[0] = (undefined *)0x1;
        rhapsody::raw::device::GraphicsDevice::update_buffer_bytes();
        if (local_128 != 0xb) {
          uVar8 = CONCAT44(uStack_124,local_128);
          lStack_d0 = lStack_120;
          local_98 = local_e8;
          uStack_90 = uStack_e0;
          local_a8 = local_f8;
          uStack_a0 = uStack_f0;
          local_b8 = local_108;
          uStack_b0 = uStack_100;
          local_c8 = local_118;
          uStack_c0 = uStack_110;
          if (1 < uVar8 - 0xb) {
            lVar7 = 0;
            if (uVar8 - 9 < 2) {
              lVar7 = uVar8 - 8;
            }
            if (lVar7 == 0) {
              if (((uVar8 < 6) && ((0x32UL >> ((ulonglong)local_128 & 0x3f) & 1) != 0)) &&
                 (lStack_120 != 0)) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
                local_108 = local_b8;
              }
            }
            else {
              local_108 = lStack_120;
              if (lVar7 == 1) {
                core::ptr::drop_in_place<backtrace::capture::Backtrace>();
                goto LAB_142314f6f;
              }
            }
            if (local_108 != 0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
          }
        }
      }
LAB_142314f6f:
      if (local_70 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      return;
    }
  }
                    /* WARNING: Subroutine does not return */
  core::option::unwrap_failed();
}

