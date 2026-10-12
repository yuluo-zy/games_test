
void __cdecl country_core::resources::terrain_editing::TerrainEditingData::new_stroke(void)

{
  undefined8 *extraout_RAX;
  longlong in_RCX;
  undefined8 *in_RDX;
  longlong local_88;
  
  if (*(ulonglong *)(in_RCX + 0x38) < 0x3fffff) {
    *(ulonglong *)(in_RCX + 0x38) = *(ulonglong *)(in_RCX + 0x38) + 1;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error();
    }
    *(undefined4 *)(extraout_RAX + 1) = *(undefined4 *)(in_RDX + 1);
    *extraout_RAX = *in_RDX;
    hashbrown::map::HashMap::
    insert<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState,alloc::alloc::Global>
              ();
    if (!SBORROW8(0,local_88)) {
      core::ptr::
      drop_in_place<enum2_<core::option::Option<country_core::resources::terrain_stroke::TerrainStroke>_>_>
                ();
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_fmt();
    }
  }
  return;
}

