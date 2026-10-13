// address 0x1415724a0
// name export_from_rg<country_core::render::nani::instance_data::SolidAlbedoMapEmissiveInstanceData>
// signature void __cdecl export_from_rg<country_core::render::nani::instance_data::SolidAlbedoMapEmissiveInstanceData>(void)

void __cdecl
country_core::render::nani::gpu_world::impl$14::
export_from_rg<country_core::render::nani::instance_data::SolidAlbedoMapEmissiveInstanceData>(void)

{
  char cVar1;
  longlong in_RCX;
  
  cVar1 = *(char *)(in_RCX + 0xe);
  *(undefined1 *)(in_RCX + 0xe) = 5;
  if (cVar1 != '\x05') {
    rhapsody::rg::import_export::impl$1::export();
  }
  return;
}

