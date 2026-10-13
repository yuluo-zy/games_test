// address: 0x140905be0
// pdb name: new
// signature: void __cdecl new(void)

void __cdecl country_core::render::pass_builder::draw_builder::DrawBuilder::new(void)

{
  char cVar1;
  longlong lVar2;
  longlong lVar3;
  code *pcVar4;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  void *in_RCX;
  void *in_RDX;
  ulonglong uVar5;
  undefined1 uVar6;
  longlong in_R8;
  byte in_R9B;
  longlong *plVar7;
  char *pcVar8;
  bool bVar9;
  undefined1 local_298 [184];
  int local_1e0;
  undefined8 local_1dc;
  undefined8 uStack_1a8;
  uint local_1a0;
  undefined8 uStack_198;
  undefined8 local_190;
  undefined8 uStack_188;
  undefined8 local_180;
  undefined8 uStack_178;
  undefined8 local_170;
  undefined8 uStack_168;
  undefined8 local_158;
  undefined8 uStack_124;
  undefined8 local_118;
  undefined8 uStack_110;
  undefined8 local_108;
  undefined8 uStack_100;
  undefined8 local_f8;
  undefined8 local_e8;
  undefined8 uStack_e0;
  undefined8 local_d8;
  undefined6 uStack_d0;
  undefined2 uStack_ca;
  undefined6 uStack_c8;
  undefined1 uStack_c2;
  byte bStack_c1;
  uint local_b8;
  undefined4 uStack_b4;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined2 uStack_9c;
  undefined2 uStack_9a;
  undefined4 uStack_98;
  undefined2 local_94;
  undefined2 uStack_92;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined8 uStack_80;
  undefined8 local_78;
  undefined8 uStack_70;
  undefined8 local_68;
  undefined8 uStack_60;
  undefined8 local_58;
  longlong *local_48;
  char *local_40;
  undefined8 local_38;
  
  local_38 = 0xfffffffffffffffe;
  lVar3 = *(longlong *)(in_R8 + 0x1f0);
  *(byte *)((longlong)in_RDX + 0xb0) = in_R9B;
  memcpy(local_298,in_RDX,0xb8);
  local_b8 = (uint)local_298._160_8_;
  uStack_b4 = SUB84(local_298._160_8_,4);
  frozen::FrozenMap::
  get<country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,alloc::boxed::Box<country_core::render::shader::RasterShaderProgram,alloc::alloc::Global>,std::hash::random::RandomState,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>_>
            ();
  if (extraout_RAX == 0) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  plVar7 = *(longlong **)(extraout_RAX + 0x58 + (ulonglong)in_R9B * 0x40);
  LOCK();
  lVar2 = *plVar7;
  *plVar7 = *plVar7 + 1;
  UNLOCK();
  if (*plVar7 == 0 || SCARRY8(lVar2,1) != *plVar7 < 0) {
                    // WARNING: Does not return
    pcVar4 = (code *)invalidInstructionException();
    (*pcVar4)();
  }
  uVar5 = 0x2ed189581b92decf;
  local_48 = plVar7;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX_00 == 0) || (*(ulonglong *)(lVar3 + 0x218) <= uVar5)) ||
      (uVar5 = *(ulonglong *)(*(longlong *)(lVar3 + 0x210) + uVar5 * 8), uVar5 == 0)) ||
     (*(longlong *)(*(longlong *)(lVar3 + 0x1e0) + 0x20 + ~uVar5 * 0x60) == 0)) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  pcVar8 = *(char **)(in_R8 + 0x1d0);
  LOCK();
  cVar1 = *pcVar8;
  if (cVar1 == '\0') {
    *pcVar8 = '\x01';
  }
  UNLOCK();
  if (cVar1 != '\0') {
    parking_lot::raw_mutex::RawMutex::lock_slow();
  }
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  if (plVar7[0x1d] != 0) {
    local_40 = pcVar8;
    rhapsody::raw::dynamic_constants::DynamicConstants::allocate();
    rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_uniform_buffer_binding_and_data();
    local_158 = CONCAT44(uStack_b0,uStack_b4);
    uStack_124 = uStack_80;
    local_118 = local_78;
    uStack_110 = uStack_70;
    local_108 = local_68;
    uStack_100 = uStack_60;
    local_f8 = local_58;
    pcVar8 = local_40;
    if (local_b8 != 10) {
      uStack_c8 = (undefined6)local_58;
      uStack_c2 = (undefined1)((ulonglong)local_58 >> 0x30);
      bStack_c1 = (byte)((ulonglong)local_58 >> 0x38);
      local_d8 = local_68;
      uStack_d0 = (undefined6)uStack_60;
      uStack_ca = (undefined2)((ulonglong)uStack_60 >> 0x30);
      local_e8 = local_78;
      uStack_e0 = uStack_70;
      local_1e0 = local_b8;
      uStack_1a8 = uStack_80;
      local_1dc = local_158;
      hashbrown::map::HashMap::
      insert<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2_<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>,std::hash::random::RandomState,alloc::alloc::Global>
                ();
      if (local_1a0 != 10) {
        uStack_88 = (undefined4)local_170;
        uStack_84 = (undefined4)((ulonglong)local_170 >> 0x20);
        uStack_80 = uStack_168;
        uStack_98 = (undefined4)local_180;
        local_94 = (undefined2)((ulonglong)local_180 >> 0x20);
        uStack_92 = (undefined2)((ulonglong)local_180 >> 0x30);
        uStack_90 = (undefined4)uStack_178;
        uStack_8c = (undefined4)((ulonglong)uStack_178 >> 0x20);
        uStack_a8 = (undefined4)local_190;
        uStack_a4 = (undefined4)((ulonglong)local_190 >> 0x20);
        uStack_a0 = (undefined4)uStack_188;
        uStack_9c = (undefined2)((ulonglong)uStack_188 >> 0x20);
        uStack_9a = (undefined2)((ulonglong)uStack_188 >> 0x30);
        local_b8 = local_1a0;
        uStack_b0 = (undefined4)uStack_198;
        uStack_ac = (undefined4)((ulonglong)uStack_198 >> 0x20);
        rhapsody::raw::managed::handle::impl$6::drop();
        if (((local_b8 & 0xfffffffe) == 2) && (CONCAT44(uStack_ac,uStack_b0) != 0)) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
      }
      pcVar8 = local_40;
      uVar6 = uStack_c2;
      local_b8 = (uint)local_e8;
      uStack_b4 = (undefined4)((ulonglong)local_e8 >> 0x20);
      uStack_b0 = (undefined4)uStack_e0;
      uStack_ac = (undefined4)((ulonglong)uStack_e0 >> 0x20);
      uStack_a8 = (undefined4)local_d8;
      uStack_a4 = (undefined4)((ulonglong)local_d8 >> 0x20);
      uStack_a0 = (undefined4)uStack_d0;
      uStack_9c = (undefined2)((uint6)uStack_d0 >> 0x20);
      uStack_9a = uStack_ca;
      uStack_98 = (undefined4)uStack_c8;
      local_94 = (undefined2)((uint6)uStack_c8 >> 0x20);
      plVar7 = (longlong *)(ulonglong)bStack_c1;
      parking_lot_core::parking_lot::deadlock_impl::release_resource();
      LOCK();
      bVar9 = *pcVar8 == '\x01';
      if (bVar9) {
        *pcVar8 = '\0';
      }
      UNLOCK();
      if (!bVar9) {
        parking_lot::raw_mutex::RawMutex::unlock_slow();
      }
      goto LAB_140905f2c;
    }
  }
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  LOCK();
  cVar1 = *pcVar8;
  if (cVar1 == '\x01') {
    *pcVar8 = '\0';
  }
  UNLOCK();
  if (cVar1 != '\x01') {
    parking_lot::raw_mutex::RawMutex::unlock_slow();
  }
  uVar6 = 8;
LAB_140905f2c:
  memcpy(in_RCX,local_298,0xb8);
  *(ulonglong *)((longlong)in_RCX + 0xb8) = CONCAT44(uStack_b4,local_b8);
  *(ulonglong *)((longlong)in_RCX + 0xc0) = CONCAT44(uStack_ac,uStack_b0);
  *(ulonglong *)((longlong)in_RCX + 200) = CONCAT44(uStack_a4,uStack_a8);
  *(ulonglong *)((longlong)in_RCX + 0xd0) = CONCAT26(uStack_9a,CONCAT24(uStack_9c,uStack_a0));
  *(ulonglong *)((longlong)in_RCX + 0xd6) = CONCAT26(local_94,CONCAT42(uStack_98,uStack_9a));
  *(undefined1 *)((longlong)in_RCX + 0xde) = uVar6;
  *(char *)((longlong)in_RCX + 0xdf) = (char)plVar7;
  *(longlong *)((longlong)in_RCX + 0xe0) = in_R8;
  *(longlong **)((longlong)in_RCX + 0xe8) = local_48;
  *(undefined1 *)((longlong)in_RCX + 0xf0) = 0;
  return;
}


