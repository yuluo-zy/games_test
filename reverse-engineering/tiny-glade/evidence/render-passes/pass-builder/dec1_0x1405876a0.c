// address: 0x1405876a0
// pdb name: new<alloc::string::String>
// signature: void __cdecl new<alloc::string::String>(void)

void __cdecl
country_core::render::pass_builder::compute_pass_builder::ComputePass::new<alloc::string::String>
          (void)

{
  undefined8 *puVar1;
  char cVar2;
  longlong lVar3;
  code *pcVar4;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  char *extraout_RAX_02;
  undefined8 in_RDX;
  undefined1 *puVar5;
  undefined1 uVar6;
  longlong *plVar7;
  undefined8 in_R8;
  longlong *in_R9;
  bool bVar8;
  undefined8 in_stack_00000030;
  undefined8 local_350;
  undefined8 uStack_348;
  undefined8 local_340;
  undefined8 uStack_338;
  undefined8 local_330;
  undefined8 uStack_328;
  undefined8 local_320;
  undefined8 local_310;
  longlong *local_2b0;
  longlong lStack_2a8;
  undefined8 local_2a0;
  undefined8 uStack_298;
  undefined8 local_290;
  undefined8 local_288;
  longlong lStack_280;
  longlong lStack_278;
  longlong lStack_270;
  undefined8 local_268;
  undefined8 uStack_260;
  undefined8 local_258;
  undefined8 uStack_250;
  undefined8 uStack_248;
  undefined8 local_240;
  undefined8 uStack_238;
  undefined8 local_230;
  undefined8 uStack_228;
  undefined8 local_220;
  undefined8 local_210;
  longlong *local_1a0;
  longlong lStack_198;
  undefined8 local_190;
  undefined6 uStack_188;
  undefined2 uStack_182;
  undefined6 uStack_180;
  undefined1 local_17a;
  undefined1 local_179;
  longlong *local_178;
  undefined8 local_168;
  undefined8 uStack_160;
  undefined8 local_158;
  undefined8 uStack_150;
  undefined8 local_148;
  undefined8 uStack_140;
  undefined8 local_138;
  undefined8 local_120;
  undefined8 uStack_118;
  undefined8 local_110;
  undefined8 uStack_108;
  undefined8 local_100;
  undefined8 uStack_f8;
  undefined8 local_f0;
  longlong *local_e8;
  longlong lStack_e0;
  undefined8 local_d8;
  undefined6 uStack_d0;
  undefined2 uStack_ca;
  undefined6 uStack_c8;
  longlong *local_c0;
  longlong *local_b8;
  longlong lStack_b0;
  undefined8 local_a8;
  undefined6 uStack_a0;
  undefined2 uStack_9a;
  undefined6 uStack_98;
  undefined1 uStack_92;
  byte bStack_91;
  longlong *local_90;
  undefined1 local_88 [8];
  longlong local_80;
  longlong lStack_78;
  longlong local_70;
  longlong *local_68;
  longlong *local_60;
  longlong *local_58;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  local_60 = in_R9;
  rhapsody::raw::managed::handle::impl$21::
  clone<array_<enum2_<rhapsody::raw::binding::DescriptorSetBinding>,2>,rhapsody::raw::managed::handle::RawManagedHandle>
            ();
  local_2b0 = (longlong *)local_60[1];
  lStack_2a8 = local_60[2];
  local_2a0 = 1;
  local_c0 = local_2b0;
  ::utils::calculate_hash<tuple_<ref_<str_>,ref_<tuple_<>_>_>_>();
  pass_resources::PassResources::new_from();
  bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::
  get_resource<country_core::render::GraphicsDeviceResource>();
  if (extraout_RAX == 0) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  render::impl$0::deref();
  puVar5 = local_88;
  asset_libraries::shader_library::AssetShaderLibrary::get_compute_by_name();
  if (puVar5 == (undefined1 *)0x0) {
    shader_uri::impl$1::as_compute_pipeline<alloc::string::String>();
    shader::ComputeShaderProgram::new();
    local_49 = 1;
    toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
    local_b8 = *(longlong **)(extraout_RAX_01 + 0x18);
    LOCK();
    lVar3 = *local_b8;
    *local_b8 = *local_b8 + 1;
    UNLOCK();
    if (*local_b8 != 0 && SCARRY8(lVar3,1) == *local_b8 < 0) {
      local_58 = local_b8;
      memcpy(&local_2b0,&local_350,0xa0);
      asset_libraries::shader_library::AssetShaderLibrary::add_compute();
      local_90 = local_58;
      goto LAB_14058782e;
    }
  }
  else {
    toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
    local_90 = *(longlong **)(extraout_RAX_00 + 0x18);
    LOCK();
    lVar3 = *local_90;
    *local_90 = *local_90 + 1;
    UNLOCK();
    if (*local_90 != 0 && SCARRY8(lVar3,1) == *local_90 < 0) {
LAB_14058782e:
      local_68 = local_90;
      rhapsody::rg::resource_info::impl$13::from();
      local_310 = in_stack_00000030;
      local_350 = 0;
      uStack_348 = 4;
      local_340 = 0;
      uStack_328 = 0;
      local_80 = 0;
      lStack_78 = 8;
      local_70 = 0;
      uStack_338 = in_RDX;
      local_330 = in_R8;
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      LOCK();
      cVar2 = *extraout_RAX_02;
      if (cVar2 == '\0') {
        *extraout_RAX_02 = '\x01';
      }
      UNLOCK();
      if (cVar2 != '\0') {
        parking_lot::raw_mutex::RawMutex::lock_slow();
      }
      parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
      local_58 = (longlong *)extraout_RAX_02;
      uniform_binder::UniformBinder::new();
      lVar3 = local_70;
      if ((int)lStack_280 == 10) {
        parking_lot_core::parking_lot::deadlock_impl::release_resource();
        LOCK();
        bVar8 = (char)*local_58 == '\x01';
        if (bVar8) {
          *(char *)local_58 = '\0';
        }
        UNLOCK();
        if (!bVar8) {
          parking_lot::raw_mutex::RawMutex::unlock_slow();
        }
        uVar6 = 8;
        plVar7 = local_58;
      }
      else {
        uStack_98 = (undefined6)local_290;
        uStack_92 = (undefined1)((ulonglong)local_290 >> 0x30);
        bStack_91 = (byte)((ulonglong)local_290 >> 0x38);
        local_a8 = local_2a0;
        uStack_a0 = (undefined6)uStack_298;
        uStack_9a = (undefined2)((ulonglong)uStack_298 >> 0x30);
        local_b8 = local_2b0;
        lStack_b0 = lStack_2a8;
        if (local_70 == local_80) {
          alloc::raw_vec::RawVec::
          grow_one<indexmap::Bucket<tuple_<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>,alloc::alloc::Global>
                    ();
        }
        uVar6 = uStack_92;
        *(undefined8 *)(lStack_78 + 0x40 + lVar3 * 0x48) = uStack_248;
        puVar1 = (undefined8 *)(lStack_78 + 0x30 + lVar3 * 0x48);
        *puVar1 = local_258;
        puVar1[1] = uStack_250;
        puVar1 = (undefined8 *)(lStack_78 + 0x20 + lVar3 * 0x48);
        *puVar1 = local_268;
        puVar1[1] = uStack_260;
        puVar1 = (undefined8 *)(lStack_78 + 0x10 + lVar3 * 0x48);
        *puVar1 = lStack_278;
        puVar1[1] = lStack_270;
        puVar1 = (undefined8 *)(lStack_78 + lVar3 * 0x48);
        *puVar1 = local_288;
        puVar1[1] = lStack_280;
        local_70 = lVar3 + 1;
        local_e8 = local_b8;
        lStack_e0 = lStack_b0;
        local_d8 = local_a8;
        uStack_d0 = uStack_a0;
        uStack_ca = uStack_9a;
        uStack_c8 = uStack_98;
        plVar7 = (longlong *)(ulonglong)bStack_91;
        parking_lot_core::parking_lot::deadlock_impl::release_resource();
        LOCK();
        bVar8 = (char)*local_58 == '\x01';
        if (bVar8) {
          *(char *)local_58 = '\0';
        }
        UNLOCK();
        if (!bVar8) {
          parking_lot::raw_mutex::RawMutex::unlock_slow();
        }
      }
      local_210 = local_310;
      local_220 = local_320;
      local_230 = local_330;
      uStack_228 = uStack_328;
      local_240 = local_340;
      uStack_238 = uStack_338;
      uStack_250 = local_350;
      uStack_248 = uStack_348;
      local_1a0 = local_e8;
      lStack_198 = lStack_e0;
      local_190 = local_d8;
      uStack_188 = uStack_d0;
      uStack_182 = uStack_ca;
      uStack_180 = uStack_c8;
      lStack_270 = local_70;
      lStack_280 = local_80;
      lStack_278 = lStack_78;
      local_178 = local_68;
      local_179 = SUB81(plVar7,0);
      local_2b0 = (longlong *)0x0;
      lStack_2a8 = 4;
      local_2a0 = 0;
      uStack_298 = 0;
      local_290 = 4;
      local_288 = 0;
      local_268 = 0;
      uStack_260 = 1;
      local_258 = 0;
      local_168 = local_120;
      uStack_160 = uStack_118;
      local_158 = local_110;
      uStack_150 = uStack_108;
      local_148 = local_100;
      uStack_140 = uStack_f8;
      local_138 = local_f0;
      local_17a = uVar6;
      if (*local_60 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      bind_resources::impl$0::
      bind_color_library<country_core::render::pass_builder::compute_pass_builder::ComputePass>();
      return;
    }
  }
                    // WARNING: Does not return
  pcVar4 = (code *)invalidInstructionException();
  (*pcVar4)();
}


