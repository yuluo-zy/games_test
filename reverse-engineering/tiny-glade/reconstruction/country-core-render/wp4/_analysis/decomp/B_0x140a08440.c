// ADDRESS: 0x140a08440
// SIZE: 170
// SIGNATURE: void __cdecl append(void)

void __cdecl country_core::render::mesh::VertexAttributeValues::append(void)

{
  longlong *in_RCX;
  longlong *in_RDX;
  longlong local_80;
  longlong local_78;
  longlong *local_70;
  code *local_68;
  longlong *local_60;
  code *local_58;
  undefined **local_50;
  undefined8 local_48;
  undefined8 **local_40;
  undefined8 local_38;
  undefined8 local_30;
  
  local_80 = *in_RCX;
  local_78 = *in_RDX;
  if (local_80 == local_78) {
                    // WARNING: Could not recover jumptable at 0x000140a08477. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(&DAT_142ac7058 + *(int *)(&DAT_142ac7058 + local_80 * 4)))
              (&DAT_142ac7058 + *(int *)(&DAT_142ac7058 + local_80 * 4),in_RDX[2]);
    return;
  }
  local_70 = &local_80;
  local_68 = core::mem::impl$5::fmt<enum2_<country_core::render::mesh::VertexAttributeValues>_>;
  local_60 = &local_78;
  local_58 = core::mem::impl$5::fmt<enum2_<country_core::render::mesh::VertexAttributeValues>_>;
  local_50 = &anon_cc634b371bd8d6b6c1e8ab9dcbca5d5b_89_llvm_9402988993337875171;
  local_48 = 2;
  local_30 = 0;
  local_40 = &local_70;
  local_38 = 2;
                    // WARNING: Subroutine does not return
  core::panicking::panic_fmt();
}

