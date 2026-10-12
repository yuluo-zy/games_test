
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl utils::geometry::rectangle::Rectangle2d::get_closest_u_from_pos(void)

{
  longlong extraout_RAX;
  undefined8 *in_RCX;
  longlong lVar1;
  longlong in_RDX;
  
  get_closest_edge_pt_indices();
  if (0.0 < (float)in_RCX[2] * (float)((ulonglong)in_RCX[2] >> 0x20)) {
    lVar1 = 0;
    if (in_RDX != 4) {
      lVar1 = in_RDX;
    }
                    /* WARNING: Could not recover jumptable at 0x000140c9b0f9. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*(code *)(&DAT_142b2eb00 + *(int *)(&DAT_142b2eb00 + extraout_RAX * 4)))
              (lVar1,0,*in_RCX,in_RCX[1]);
    return;
  }
                    /* WARNING: Subroutine does not return */
  core::panicking::panic();
}

