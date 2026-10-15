0x1416f2800 sub      rsp, 0x88
0x1416f2807 mov      rax, rdx
0x1416f280a cmp      byte ptr [rcx], 0
0x1416f280d je       0x1416f28b2
0x1416f2813 lea      rdx, [rcx + 4]
0x1416f2817 lea      r8, [rcx + 8]
0x1416f281b inc      rcx
0x1416f281e mov      qword ptr [rsp + 0x80], rcx
0x1416f2826 lea      rcx, [rip + 0x159dddb] ; 0x142c90608: b'\x00\x00\x00\x00\x00\x00\x00\x00\x08\x00\x00\x00\x00\x00\x00\x00\x08\x00\x00\x00\x00\x00\x00\x00\x908S@\x01\x00\x00\x00Walkinggoal_chan'
0x1416f282d mov      qword ptr [rsp + 0x70], rcx
0x1416f2832 lea      rcx, [rsp + 0x80]
0x1416f283a mov      qword ptr [rsp + 0x68], rcx
0x1416f283f lea      rcx, [rip + 0x159ddfc] ; 0x142c90642: b'force_reach_goal\x00\x00\x00\x00\x00\x00e\x03\xc9B\x01\x00\x00\x007\x00\x00\x00\x00\x00\x00\x00\x19\x00\x00\x00\n\x00\x00\x00fi'
0x1416f2846 mov      qword ptr [rsp + 0x58], rcx
0x1416f284b lea      rcx, [rip + 0x159dd96] ; 0x142c905e8: b'\x00\x00\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x90\xc8\xd8@\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x08\x00\x00\x00\x00\x00\x00\x00'
0x1416f2852 mov      qword ptr [rsp + 0x50], rcx
0x1416f2857 mov      qword ptr [rsp + 0x48], r8
0x1416f285c lea      r8, [rip + 0x159ddcc] ; 0x142c9062f: b'goal_change_timeoutforce_reach_goal\x00\x00\x00\x00\x00\x00e\x03\xc9B\x01\x00\x00'
0x1416f2863 mov      qword ptr [rsp + 0x38], r8
0x1416f2868 mov      qword ptr [rsp + 0x30], rcx
0x1416f286d mov      qword ptr [rsp + 0x28], rdx
0x1416f2872 mov      qword ptr [rsp + 0x60], 0x10
0x1416f287b mov      qword ptr [rsp + 0x40], 0x13
0x1416f2884 mov      qword ptr [rsp + 0x20], 9
0x1416f288d lea      rdx, [rip + 0x159dd94] ; 0x142c90628: b'Walkinggoal_change_timeoutforce_reach_goal\x00\x00\x00\x00\x00\x00'
0x1416f2894 lea      r9, [rip + 0x159dd41] ; 0x142c905dc: b'timeout_s\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x90\xc8\xd8@\x01\x00\x00\x00\x00\x00\x00\x00'
0x1416f289b mov      r8d, 7
0x1416f28a1 mov      rcx, rax
0x1416f28a4 call     0x140d86d90 ; core::fmt::Formatter::debug_struct_field3_finish::h024add6f4e520141
0x1416f28a9 nop      
0x1416f28aa add      rsp, 0x88
0x1416f28b1 ret      
0x1416f28b2 add      rcx, 4
0x1416f28b6 mov      qword ptr [rsp + 0x80], rcx
0x1416f28be lea      rcx, [rip + 0x159dcf3] ; 0x142c905b8: b'\x00\x00\x00\x00\x00\x00\x00\x00\x08\x00\x00\x00\x00\x00\x00\x00\x08\x00\x00\x00\x00\x00\x00\x00\x107S@\x01\x00\x00\x00Idletimeout_s\x00\x00\x00'
0x1416f28c5 mov      qword ptr [rsp + 0x30], rcx
0x1416f28ca lea      rcx, [rsp + 0x80]
0x1416f28d2 mov      qword ptr [rsp + 0x28], rcx
0x1416f28d7 mov      qword ptr [rsp + 0x20], 9
0x1416f28e0 lea      rdx, [rip + 0x159dcf1] ; 0x142c905d8: b'Idletimeout_s\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x90\xc8\xd8@\x01\x00\x00\x00'
0x1416f28e7 lea      r9, [rip + 0x159dcee] ; 0x142c905dc: b'timeout_s\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x90\xc8\xd8@\x01\x00\x00\x00\x00\x00\x00\x00'
0x1416f28ee mov      r8d, 4
0x1416f28f4 mov      rcx, rax
0x1416f28f7 call     0x140d86bf0 ; core::fmt::Formatter::debug_struct_field1_finish::hd9497ec2d71b9977
0x1416f28fc nop      
0x1416f28fd add      rsp, 0x88
0x1416f2904 ret      