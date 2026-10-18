
void __cdecl
serde::de::impls::impl$19::deserialize::impl$0::
visit_seq<tuple_<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,ref_mut_<serde_json::value::de::SeqDeserializer>_>
          (void)

{
  undefined8 *puVar1;
  char *pcVar2;
  ulonglong extraout_RAX;
  ulonglong extraout_RAX_00;
  ulonglong uVar3;
  ulonglong *in_RCX;
  ulonglong uVar4;
  longlong lVar5;
  ulonglong in_RDX;
  ulonglong local_190;
  ulonglong local_188;
  undefined8 local_180;
  undefined8 uStack_178;
  undefined8 local_170;
  undefined8 uStack_168;
  undefined8 local_160;
  undefined8 uStack_158;
  undefined8 local_150;
  undefined8 uStack_148;
  undefined8 local_140;
  undefined8 uStack_138;
  undefined8 local_130;
  undefined8 uStack_128;
  undefined8 local_120;
  ulonglong local_68;
  
  uVar3 = in_RDX;
  serde_json::value::de::impl$7::size_hint();
  uVar4 = 0x2222;
  if (uVar3 < 0x2222) {
    uVar4 = uVar3;
  }
  uVar3 = 0;
  if ((extraout_RAX & 1) != 0) {
    uVar3 = uVar4;
  }
  if (uVar3 == 0) {
    uVar3 = 8;
    uVar4 = 0;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    uVar3 = extraout_RAX_00;
    if (extraout_RAX_00 == 0) {
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
  }
  local_68 = 0;
  while( true ) {
    pcVar2 = *(char **)(in_RDX + 8);
    if ((pcVar2 == *(char **)(in_RDX + 0x18)) ||
       (*(char **)(in_RDX + 8) = pcVar2 + 0x20, *pcVar2 == '\x06')) {
      in_RCX[2] = local_68;
      *in_RCX = uVar4;
      in_RCX[1] = uVar3;
      return;
    }
    serde_json::value::de::impl$2::
    deserialize_tuple<serde::de::impls::impl_188::deserialize::TupleVisitor<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>_>
              ();
    if (SBORROW8(0,local_188)) break;
    if (local_68 == uVar4) {
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateStairPoint>,alloc::alloc::Global>
                ();
    }
    lVar5 = local_68 * 0x78;
    *(undefined8 *)(uVar3 + 0x70 + lVar5) = local_120;
    puVar1 = (undefined8 *)(uVar3 + 0x60 + lVar5);
    *puVar1 = local_130;
    puVar1[1] = uStack_128;
    puVar1 = (undefined8 *)(uVar3 + 0x50 + lVar5);
    *puVar1 = local_140;
    puVar1[1] = uStack_138;
    puVar1 = (undefined8 *)(uVar3 + 0x40 + lVar5);
    *puVar1 = local_150;
    puVar1[1] = uStack_148;
    puVar1 = (undefined8 *)(uVar3 + 0x30 + lVar5);
    *puVar1 = local_160;
    puVar1[1] = uStack_158;
    puVar1 = (undefined8 *)(uVar3 + 0x20 + lVar5);
    *puVar1 = local_170;
    puVar1[1] = uStack_168;
    puVar1 = (undefined8 *)(uVar3 + 0x10 + lVar5);
    *puVar1 = local_180;
    puVar1[1] = uStack_178;
    *(ulonglong *)(uVar3 + lVar5) = local_190;
    ((ulonglong *)(uVar3 + lVar5))[1] = local_188;
    local_68 = local_68 + 1;
  }
  in_RCX[1] = local_190;
  *in_RCX = 0x8000000000000000;
  if (local_68 != 0) {
    lVar5 = uVar3 + 0x28;
    do {
      if (*(longlong *)(lVar5 + -0x20) != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if (*(longlong *)(lVar5 + -8) != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      lVar5 = lVar5 + 0x78;
      local_68 = local_68 - 1;
    } while (local_68 != 0);
  }
  if (uVar4 == 0) {
    return;
  }
  std::alloc::__default_lib_allocator::__rust_dealloc();
  return;
}

