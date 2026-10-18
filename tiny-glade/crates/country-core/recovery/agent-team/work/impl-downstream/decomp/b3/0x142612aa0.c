
void __cdecl
std::collections::hash::map::impl$87::
from_iter<u64,ref_<system_clutter::autoclutter::AutoClutterObject>,std::hash::random::RandomState,core::iter::adapters::map::Map<core::slice::iter::Iter<system_clutter::autoclutter::AutoClutterObject>,system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities::closure_env_1>_>
          (void)

{
  byte *extraout_RAX;
  longlong extraout_RAX_00;
  longlong lVar1;
  undefined8 *in_RCX;
  longlong in_RDX;
  longlong lVar2;
  longlong in_R8;
  longlong lVar3;
  
  lVar2 = in_RDX;
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX & 1) == 0) {
    sys::random::hashmap_random_keys();
    extraout_RAX[0] = 1;
    extraout_RAX[1] = 0;
    extraout_RAX[2] = 0;
    extraout_RAX[3] = 0;
    extraout_RAX[4] = 0;
    extraout_RAX[5] = 0;
    extraout_RAX[6] = 0;
    extraout_RAX[7] = 0;
    *(longlong *)(extraout_RAX + 8) = extraout_RAX_00;
    *(longlong *)(extraout_RAX + 0x10) = lVar2;
    lVar1 = extraout_RAX_00;
  }
  else {
    lVar1 = *(longlong *)(extraout_RAX + 8);
    lVar2 = *(longlong *)(extraout_RAX + 0x10);
  }
  *(longlong *)(extraout_RAX + 8) = lVar1 + 1;
  if (in_R8 != in_RDX) {
    hashbrown::raw::RawTable::
    reserve_rehash<tuple_<country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>,alloc::boxed::Box<country_core::render::shader::ComputeShaderProgram,alloc::alloc::Global>_>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>,alloc::boxed::Box<country_core::render::shader::ComputeShaderProgram,alloc::alloc::Global>,std::hash::random::RandomState>_>
              ();
  }
  if (in_RDX != in_R8) {
    lVar3 = 0;
    do {
      hashbrown::map::HashMap::
      insert<u64,ref_<system_clutter::autoclutter::AutoClutterObject>,std::hash::random::RandomState,alloc::alloc::Global>
                ();
      lVar3 = lVar3 + 1;
    } while (lVar3 != ((ulonglong)(in_R8 - in_RDX) >> 3) * 0x4ec4ec4ec4ec4ec5);
  }
  in_RCX[4] = lVar1;
  in_RCX[5] = lVar2;
  in_RCX[2] = 0;
  in_RCX[3] = 0;
  *in_RCX = &anon_9ab52345996c812b6a825a941408fa1b_0_llvm_10263905268791311693;
  in_RCX[1] = 0;
  return;
}

