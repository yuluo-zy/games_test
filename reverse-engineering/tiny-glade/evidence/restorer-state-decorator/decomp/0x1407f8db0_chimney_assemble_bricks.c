
void __cdecl
system_decorator::decorator_visual::chimney_assemble_bricks::chimney_assemble_bricks(void)

{
  undefined4 *puVar1;
  longlong in_R9;
  undefined1 local_60 [8];
  undefined4 *local_58;
  undefined1 *local_30;
  
  puVar1 = *(undefined4 **)(in_R9 + 0x10);
  *puVar1 = *(undefined4 *)(in_R9 + 0x1c);
  country_core::resources::walls::decorator_subtype::impl$8::from();
  local_30 = local_60;
  local_58 = puVar1;
  deco_differential_state::WallAttachedDecoTracker::
  with_diffs<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyParams,system_decorator::decorator_visual::chimney_assemble_bricks::chimney_assemble_bricks::closure_env_0>
            ();
  return;
}

