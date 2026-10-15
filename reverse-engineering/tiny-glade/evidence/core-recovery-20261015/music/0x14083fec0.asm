0x14083fec0 sub      rsp, 0x68
0x14083fec4 mov      rax, rdx
0x14083fec7 mov      edx, dword ptr [rcx]
0x14083fec9 lea      r8, [rip + 0x2253fbc]
0x14083fed0 movsxd   rdx, dword ptr [r8 + rdx*4]
0x14083fed4 add      rdx, r8
0x14083fed7 jmp      rdx
0x14083fed9 lea      rdx, [rip + 0x2253ef8] ; @0x142a93dd8 'SwitchTrack'
0x14083fee0 mov      r8d, 0xb
0x14083fee6 mov      rcx, rax
0x14083fee9 add      rsp, 0x68
0x14083feed jmp      0x140d86b90 ; _$LT$core..fmt..Formatter$u20$as$u20$core..fmt..Write$GT$::write_str::h5c3f9ed06cfb23bd / core::fmt::Formatter::write_str::h47944a04c0e87e3b
0x14083fef2 add      rcx, 4
0x14083fef6 mov      qword ptr [rsp + 0x60], rcx
0x14083fefb lea      rcx, [rip + 0x2253ee6]
0x14083ff02 mov      qword ptr [rsp + 0x30], rcx
0x14083ff07 lea      rcx, [rsp + 0x60]
0x14083ff0c mov      qword ptr [rsp + 0x28], rcx
0x14083ff11 mov      qword ptr [rsp + 0x20], 0xc
0x14083ff1a lea      rdx, [rip + 0x2253f4c] ; @0x142a93e6d 'Switching'
0x14083ff21 lea      r9, [rip + 0x2253ee7] ; @0x142a93e0f 'elapsed_time'
0x14083ff28 mov      r8d, 9
0x14083ff2e jmp      0x14083fffc ; 
0x14083ff33 lea      rdx, [rcx + 4]
0x14083ff37 add      rcx, 8
0x14083ff3b mov      qword ptr [rsp + 0x60], rcx
0x14083ff40 lea      rcx, [rip + 0x2253c91]
0x14083ff47 mov      qword ptr [rsp + 0x50], rcx
0x14083ff4c lea      rcx, [rsp + 0x60]
0x14083ff51 mov      qword ptr [rsp + 0x48], rcx
0x14083ff56 lea      rcx, [rip + 0x2253ef4] ; @0x142a93e51 'ticksSwitchAtNextOpportunitySwitching'
0x14083ff5d mov      qword ptr [rsp + 0x38], rcx
0x14083ff62 lea      rcx, [rip + 0x2253eb7]
0x14083ff69 mov      qword ptr [rsp + 0x30], rcx
0x14083ff6e mov      qword ptr [rsp + 0x28], rdx
0x14083ff73 mov      qword ptr [rsp + 0x40], 5
0x14083ff7c mov      qword ptr [rsp + 0x20], 0xc
0x14083ff85 lea      rdx, [rip + 0x2253eb4] ; @0x142a93e40 'PlayingLoopedOnceticksSwitchAtNextOpportunitySwitching'
0x14083ff8c lea      r9, [rip + 0x2253e7c] ; @0x142a93e0f 'elapsed_time'
0x14083ff93 mov      r8d, 0x11
0x14083ff99 mov      rcx, rax
0x14083ff9c call     0x140d86ca0 ; core::fmt::Formatter::debug_struct_field2_finish::h35c7e6c960cc525a
0x14083ffa1 nop      
0x14083ffa2 add      rsp, 0x68
0x14083ffa6 ret      
0x14083ffa7 lea      rdx, [rip + 0x2253ea8] ; @0x142a93e56 'SwitchAtNextOpportunitySwitching'
0x14083ffae mov      r8d, 0x17
0x14083ffb4 mov      rcx, rax
0x14083ffb7 add      rsp, 0x68
0x14083ffbb jmp      0x140d86b90 ; _$LT$core..fmt..Formatter$u20$as$u20$core..fmt..Write$GT$::write_str::h5c3f9ed06cfb23bd / core::fmt::Formatter::write_str::h47944a04c0e87e3b
0x14083ffc0 add      rcx, 4
0x14083ffc4 mov      qword ptr [rsp + 0x60], rcx
0x14083ffc9 lea      rcx, [rip + 0x2253e18]
0x14083ffd0 mov      qword ptr [rsp + 0x30], rcx
0x14083ffd5 lea      rcx, [rsp + 0x60]
0x14083ffda mov      qword ptr [rsp + 0x28], rcx
0x14083ffdf mov      qword ptr [rsp + 0x20], 0xc
0x14083ffe8 lea      rdx, [rip + 0x2253e19] ; @0x142a93e08 'Playingelapsed_time'
0x14083ffef lea      r9, [rip + 0x2253e19] ; @0x142a93e0f 'elapsed_time'
0x14083fff6 mov      r8d, 7
0x14083fffc mov      rcx, rax
0x14083ffff call     0x140d86bf0 ; core::fmt::Formatter::debug_struct_field1_finish::hd9497ec2d71b9977
0x140840004 nop      
0x140840005 add      rsp, 0x68
0x140840009 ret      