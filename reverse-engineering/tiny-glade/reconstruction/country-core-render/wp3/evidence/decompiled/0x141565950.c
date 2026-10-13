// address 0x141565950
// name call_once<country_core::render::nani::systems::nani_maintain_instances::closure_env$1,tuple$<country_core::render::nani::InstanceHandle>_>
// signature void __cdecl call_once<country_core::render::nani::systems::nani_maintain_instances::closure_env$1,tuple$<country_core::render::nani::InstanceHandle>_>(void)

void __cdecl
core::ops::function::FnOnce::
call_once<country_core::render::nani::systems::nani_maintain_instances::closure_env_1,tuple_<country_core::render::nani::InstanceHandle>_>
          (void)

{
  undefined8 *puVar1;
  longlong *in_RCX;
  undefined8 *in_RDX;
  
  puVar1 = (undefined8 *)*in_RCX;
  *(undefined4 *)(puVar1 + 1) = *(undefined4 *)(in_RDX + 1);
  *puVar1 = *in_RDX;
  return;
}

