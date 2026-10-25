// address 0x141572440
// name import_to_rg<country_core::render::nani::instance_data::WrenInstanceData>
// signature void __cdecl import_to_rg<country_core::render::nani::instance_data::WrenInstanceData>(void)

void __cdecl
country_core::render::nani::gpu_world::impl$14::
import_to_rg<country_core::render::nani::instance_data::WrenInstanceData>(void)

{
  undefined8 *in_RCX;
  undefined8 local_20;
  undefined8 uStack_18;
  undefined8 local_10;
  
  if (*(char *)((longlong)in_RCX + 0xe) == '\x05') {
    rhapsody::rg::import_export::impl$1::import_unowned();
    in_RCX[2] = local_10;
    *in_RCX = local_20;
    in_RCX[1] = uStack_18;
    return;
  }
                    // WARNING: Subroutine does not return
  core::panicking::panic();
}

