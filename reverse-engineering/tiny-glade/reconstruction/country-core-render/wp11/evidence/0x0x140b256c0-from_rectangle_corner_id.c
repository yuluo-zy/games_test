// 0x140b256c0 from_rectangle_corner_id
// sig: void __cdecl from_rectangle_corner_id(void)

void __cdecl
country_core::resources::walls::wall_attachment::WallCornerAttachment::from_rectangle_corner_id
          (void)

{
  ulonglong in_RCX;
  
  if (in_RCX < 4) {
    return;
  }
                    // WARNING: Subroutine does not return
  core::panicking::panic();
}

