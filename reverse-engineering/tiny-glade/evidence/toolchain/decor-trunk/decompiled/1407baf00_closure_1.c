// ADDR 0x1407baf00
// SIG void __cdecl closure$1(void)
// ORIG system::closure$1


void __cdecl system_decorator::load_decorators::load_decorators::closure_1(void)

{
  undefined4 uVar1;
  undefined8 uVar2;
  undefined1 **ppuVar3;
  undefined8 *in_RCX;
  undefined1 *local_b8;
  code *local_b0;
  undefined1 local_a8 [16];
  undefined1 **local_98;
  undefined8 *puStack_90;
  undefined1 **local_88;
  undefined8 uStack_80;
  undefined8 local_78;
  undefined8 local_20;
  
  local_20 = 0xfffffffffffffffe;
  local_b8 = local_a8;
  local_b0 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
  local_98 = (undefined1 **)&DAT_142a85490;
  puStack_90 = (undefined8 *)0x2;
  local_88 = &local_b8;
  uStack_80 = 1;
  local_78 = 0;
  alloc::fmt::format::format_inner();
  country_core::utils::load_json::load_json_as_mesh<alloc::string::String>();
  if (SBORROW8(0,(longlong)local_98)) {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  country_core::render::mesh::Mesh::positions_without_indices();
  alloc::vec::spec_from_iter::impl$0::
  from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array_<f32,3>_>_>,country_core::render::mesh::impl_0::normals_without_indices::closure_env_0>_>
            ();
  core::ptr::drop_in_place<country_core::render::mesh::Mesh>();
  ppuVar3 = local_88;
  if (local_88 != (undefined1 **)0x0) {
    uVar2 = *puStack_90;
    uVar1 = *(undefined4 *)(puStack_90 + 1);
    if (local_88 == local_98) {
      alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
    }
    *(undefined4 *)((longlong)puStack_90 + (longlong)ppuVar3 * 0xc + 8) = uVar1;
    *(undefined8 *)((longlong)puStack_90 + (longlong)ppuVar3 * 0xc) = uVar2;
    in_RCX[2] = (undefined *)((longlong)ppuVar3 + 1);
    *in_RCX = local_98;
    in_RCX[1] = puStack_90;
    return;
  }
                    // WARNING: Subroutine does not return
  core::panicking::panic_bounds_check();
}


