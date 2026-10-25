0x1413ff520 push     r14
0x1413ff522 push     rsi
0x1413ff523 push     rdi
0x1413ff524 push     rbx
0x1413ff525 sub      rsp, 0x38
0x1413ff529 mov      rdi, rdx
0x1413ff52c mov      rsi, rcx
0x1413ff52f cmp      byte ptr [rcx], 0
0x1413ff532 je       0x1413ff675
0x1413ff538 lea      rdx, [rdi + 8]
0x1413ff53c mov      rbx, qword ptr [rdi]
0x1413ff53f mov      rax, qword ptr [rbx + 0x10]
0x1413ff543 cmp      qword ptr [rbx], rax
0x1413ff546 je       0x1413ff77f
0x1413ff54c mov      rcx, qword ptr [rbx + 8]
0x1413ff550 mov      byte ptr [rcx + rax], 0x7b
0x1413ff554 inc      rax
0x1413ff557 mov      qword ptr [rbx + 0x10], rax
0x1413ff55b lea      r8, [rip + 0x185bf7a] ; 0x142c5b4dc: b'Walkinggoal_change_timeoutforce_reach_goal\x00\x00\xcf\xb4\xc5B'
0x1413ff562 mov      r9d, 7
0x1413ff568 mov      rcx, rdi
0x1413ff56b call     0x141580d20 ; serde_json::ser::format_escaped_str::hbe26d48cff9c903f
0x1413ff570 test     rax, rax
0x1413ff573 jne      0x1413ff76c
0x1413ff579 mov      rbx, qword ptr [rdi]
0x1413ff57c mov      rdx, qword ptr [rbx + 0x10]
0x1413ff580 cmp      qword ptr [rbx], rdx
0x1413ff583 je       0x1413ff7dd
0x1413ff589 mov      rax, qword ptr [rbx + 8]
0x1413ff58d mov      byte ptr [rax + rdx], 0x3a
0x1413ff591 inc      rdx
0x1413ff594 mov      qword ptr [rbx + 0x10], rdx
0x1413ff598 mov      rbx, qword ptr [rdi]
0x1413ff59b mov      rdx, qword ptr [rbx + 0x10]
0x1413ff59f cmp      qword ptr [rbx], rdx
0x1413ff5a2 je       0x1413ff803
0x1413ff5a8 mov      rax, qword ptr [rbx + 8]
0x1413ff5ac mov      byte ptr [rax + rdx], 0x7b
0x1413ff5b0 inc      rdx
0x1413ff5b3 mov      qword ptr [rbx + 0x10], rdx
0x1413ff5b7 lea      r9, [rsi + 4]
0x1413ff5bb mov      qword ptr [rsp + 0x28], rdi
0x1413ff5c0 mov      byte ptr [rsp + 0x30], 1
0x1413ff5c5 lea      rdx, [rip + 0x185bf07] ; 0x142c5b4d3: b'timeout_sWalkinggoal_change_timeoutforce_reach_g'
0x1413ff5cc lea      rcx, [rsp + 0x28]
0x1413ff5d1 mov      r8d, 9
0x1413ff5d7 call     0x141586090 ; serde::ser::SerializeMap::serialize_entry::h56bd0d280a1272c9
0x1413ff5dc test     rax, rax
0x1413ff5df jne      0x1413ff774
0x1413ff5e5 lea      r9, [rsi + 8]
0x1413ff5e9 lea      rdx, [rip + 0x185bef3] ; 0x142c5b4e3: b'goal_change_timeoutforce_reach_goal\x00\x00\xcf\xb4\xc5B\x01\x00\x00\x00\x04\x00\x00'
0x1413ff5f0 lea      rcx, [rsp + 0x28]
0x1413ff5f5 mov      r8d, 0x13
0x1413ff5fb call     0x141586090 ; serde::ser::SerializeMap::serialize_entry::h56bd0d280a1272c9
0x1413ff600 test     rax, rax
0x1413ff603 jne      0x1413ff774
0x1413ff609 inc      rsi
0x1413ff60c lea      rdx, [rip + 0x185bee3] ; 0x142c5b4f6: b'force_reach_goal\x00\x00\xcf\xb4\xc5B\x01\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\xdc\xb4\xc5B\x01\x00\x00\x00\x07\x00\x00\x00\x00\x00'
0x1413ff613 lea      rcx, [rsp + 0x28]
0x1413ff618 mov      r8d, 0x10
0x1413ff61e mov      r9, rsi
0x1413ff621 call     0x141588150 ; serde::ser::SerializeMap::serialize_entry::h87d27dc0dbe641ff
0x1413ff626 test     rax, rax
0x1413ff629 jne      0x1413ff774
0x1413ff62f mov      rax, qword ptr [rsp + 0x28]
0x1413ff634 mov      rsi, qword ptr [rax]
0x1413ff637 cmp      byte ptr [rsp + 0x30], 0
0x1413ff63c jne      0x1413ff734
0x1413ff642 mov      rdx, qword ptr [rsi + 0x10]
0x1413ff646 cmp      qword ptr [rsi], rdx
0x1413ff649 jne      0x1413ff759
0x1413ff64f mov      qword ptr [rsp + 0x20], 1
0x1413ff658 mov      r8d, 1
0x1413ff65e mov      r9d, 1
0x1413ff664 mov      rcx, rsi
0x1413ff667 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x1413ff66c mov      rdx, qword ptr [rsi + 0x10]
0x1413ff670 jmp      0x1413ff759
0x1413ff675 lea      rdx, [rdi + 8]
0x1413ff679 mov      rbx, qword ptr [rdi]
0x1413ff67c mov      rax, qword ptr [rbx + 0x10]
0x1413ff680 cmp      qword ptr [rbx], rax
0x1413ff683 je       0x1413ff7ae
0x1413ff689 mov      rcx, qword ptr [rbx + 8]
0x1413ff68d mov      byte ptr [rcx + rax], 0x7b
0x1413ff691 inc      rax
0x1413ff694 mov      qword ptr [rbx + 0x10], rax
0x1413ff698 lea      r8, [rip + 0x185be30] ; 0x142c5b4cf: b'Idletimeout_sWalkinggoal_change_timeoutforce_rea'
0x1413ff69f mov      r9d, 4
0x1413ff6a5 mov      rcx, rdi
0x1413ff6a8 call     0x141580d20 ; serde_json::ser::format_escaped_str::hbe26d48cff9c903f
0x1413ff6ad test     rax, rax
0x1413ff6b0 jne      0x1413ff76c
0x1413ff6b6 mov      rbx, qword ptr [rdi]
0x1413ff6b9 mov      rdx, qword ptr [rbx + 0x10]
0x1413ff6bd cmp      qword ptr [rbx], rdx
0x1413ff6c0 je       0x1413ff829
0x1413ff6c6 mov      rax, qword ptr [rbx + 8]
0x1413ff6ca mov      byte ptr [rax + rdx], 0x3a
0x1413ff6ce inc      rdx
0x1413ff6d1 mov      qword ptr [rbx + 0x10], rdx
0x1413ff6d5 mov      rbx, qword ptr [rdi]
0x1413ff6d8 mov      rdx, qword ptr [rbx + 0x10]
0x1413ff6dc cmp      qword ptr [rbx], rdx
0x1413ff6df je       0x1413ff84f
0x1413ff6e5 mov      rax, qword ptr [rbx + 8]
0x1413ff6e9 mov      byte ptr [rax + rdx], 0x7b
0x1413ff6ed inc      rdx
0x1413ff6f0 mov      qword ptr [rbx + 0x10], rdx
0x1413ff6f4 add      rsi, 4
0x1413ff6f8 mov      qword ptr [rsp + 0x28], rdi
0x1413ff6fd mov      byte ptr [rsp + 0x30], 1
0x1413ff702 lea      rdx, [rip + 0x185bdca] ; 0x142c5b4d3: b'timeout_sWalkinggoal_change_timeoutforce_reach_g'
0x1413ff709 lea      rcx, [rsp + 0x28]
0x1413ff70e mov      r8d, 9
0x1413ff714 mov      r9, rsi
0x1413ff717 call     0x141586090 ; serde::ser::SerializeMap::serialize_entry::h56bd0d280a1272c9
0x1413ff71c test     rax, rax
0x1413ff71f jne      0x1413ff774
0x1413ff721 mov      rax, qword ptr [rsp + 0x28]
0x1413ff726 mov      rsi, qword ptr [rax]
0x1413ff729 cmp      byte ptr [rsp + 0x30], 0
0x1413ff72e je       0x1413ff642
0x1413ff734 mov      rdx, qword ptr [rsi + 0x10]
0x1413ff738 cmp      qword ptr [rsi], rdx
0x1413ff73b je       0x1413ff875
0x1413ff741 mov      rax, qword ptr [rsi + 8]
0x1413ff745 mov      byte ptr [rax + rdx], 0x7d
0x1413ff749 inc      rdx
0x1413ff74c mov      qword ptr [rsi + 0x10], rdx
0x1413ff750 cmp      qword ptr [rsi], rdx
0x1413ff753 je       0x1413ff64f
0x1413ff759 mov      rax, qword ptr [rsi + 8]
0x1413ff75d mov      byte ptr [rax + rdx], 0x7d
0x1413ff761 inc      rdx
0x1413ff764 mov      qword ptr [rsi + 0x10], rdx
0x1413ff768 xor      eax, eax
0x1413ff76a jmp      0x1413ff774
0x1413ff76c mov      rcx, rax
0x1413ff76f call     0x1428c98a0 ; serde_json::error::Error::io::hfcf27584d0d6aa9f
0x1413ff774 nop      
0x1413ff775 add      rsp, 0x38
0x1413ff779 pop      rbx
0x1413ff77a pop      rdi
0x1413ff77b pop      rsi
0x1413ff77c pop      r14
0x1413ff77e ret      
0x1413ff77f mov      qword ptr [rsp + 0x20], 1
0x1413ff788 mov      r8d, 1
0x1413ff78e mov      r9d, 1
0x1413ff794 mov      rcx, rbx
0x1413ff797 mov      r14, rdx
0x1413ff79a mov      rdx, rax