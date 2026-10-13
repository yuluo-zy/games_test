// address: 0x140968960
// pdb name: parse
// signature: void __cdecl parse(void)

void __cdecl country_core::render::pass_builder::shader_uri::ShaderUri::parse(void)

{
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  ulonglong *in_RCX;
  ulonglong in_RDX;
  ulonglong uVar1;
  ulonglong in_R8;
  ulonglong uVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  bool bVar6;
  
  uVar3 = 0;
  do {
    uVar2 = in_RDX;
    core::slice::memchr::memrchr();
    uVar1 = in_R8;
    if (extraout_RAX != 1) break;
    if ((uVar2 < in_R8) && (*(char *)(in_RDX + uVar2) == ':')) {
      uVar3 = uVar2 + 1;
      bVar6 = uVar3 != in_R8;
      if (uVar3 < in_R8) {
        if (*(char *)(in_RDX + uVar3) < -0x40) {
LAB_140968a9a:
                    // WARNING: Subroutine does not return
          core::str::slice_error_fail();
        }
      }
      else {
        uVar3 = in_R8;
        if (bVar6) goto LAB_140968a9a;
      }
      uVar1 = in_R8 - uVar3;
      uVar3 = uVar3 + in_RDX;
      in_R8 = uVar2;
      break;
    }
  } while (uVar2 <= in_R8);
  do {
    uVar2 = in_RDX;
    core::slice::memchr::memrchr();
    uVar5 = in_RDX;
    uVar4 = in_R8;
    if (extraout_RAX_00 != 1) break;
    if ((uVar2 < in_R8) && (*(char *)(in_RDX + uVar2) == '/')) {
      uVar2 = uVar2 + 1;
      bVar6 = uVar2 != in_R8;
      if (uVar2 < in_R8) {
        if (*(char *)(in_RDX + uVar2) < -0x40) {
LAB_140968ab4:
                    // WARNING: Subroutine does not return
          core::str::slice_error_fail();
        }
      }
      else {
        uVar2 = in_R8;
        if (bVar6) goto LAB_140968ab4;
      }
      uVar5 = uVar2 + in_RDX;
      uVar4 = in_R8 - uVar2;
      break;
    }
  } while (uVar2 <= in_R8);
  do {
    uVar2 = uVar5;
    core::slice::memchr::memrchr();
    if (extraout_RAX_01 != 1) break;
    if ((uVar2 < uVar4) && (*(char *)(uVar5 + uVar2) == '.')) {
      *in_RCX = in_RDX;
      in_RCX[1] = in_R8;
      in_RCX[2] = uVar5;
      in_RCX[3] = uVar2;
      in_RCX[4] = uVar3;
      in_RCX[5] = uVar1;
      return;
    }
  } while (uVar2 <= uVar4);
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}


