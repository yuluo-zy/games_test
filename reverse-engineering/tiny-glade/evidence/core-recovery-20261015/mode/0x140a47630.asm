0x140a47630 push     r14
0x140a47632 push     rsi
0x140a47633 push     rdi
0x140a47634 push     rbx
0x140a47635 sub      rsp, 0x38
0x140a47639 mov      rsi, rdx
0x140a4763c movzx    eax, byte ptr [rcx]
0x140a4763f add      al, 0xc8
0x140a47641 cmp      al, 8
0x140a47643 movzx    eax, al
0x140a47646 mov      edx, 6
0x140a4764b cmovb    edx, eax
0x140a4764e movzx    eax, dl
0x140a47651 lea      rdx, [rip + 0x20880d0]
0x140a47658 movsxd   rax, dword ptr [rdx + rax*4]
0x140a4765c add      rax, rdx
0x140a4765f jmp      rax
0x140a47661 lea      rdx, [rsi + 8]
0x140a47665 lea      r8, [rip + 0x20876bc] ; @0x142aced28 'CottageWindowGothicWindowArrowSlitChimneyLanternFlag'
0x140a4766c mov      r9d, 0xd
0x140a47672 jmp      0x140a476e4 ; 
0x140a47674 lea      rdx, [rsi + 8]
0x140a47678 lea      r8, [rip + 0x20876d2] ; @0x142aced51 'LanternFlag'
0x140a4767f mov      r9d, 7
0x140a47685 jmp      0x140a476e4 ; 
0x140a47687 lea      rdx, [rsi + 8]
0x140a4768b lea      r8, [rip + 0x20876af] ; @0x142aced41 'ArrowSlitChimneyLanternFlag'
0x140a47692 mov      r9d, 9
0x140a47698 jmp      0x140a476e4 ; 
0x140a4769a lea      rdx, [rsi + 8]
0x140a4769e lea      r8, [rip + 0x20876a5] ; @0x142aced4a 'ChimneyLanternFlag'
0x140a476a5 mov      r9d, 7
0x140a476ab jmp      0x140a476e4 ; 
0x140a476ad lea      rdx, [rsi + 8]
0x140a476b1 lea      r8, [rip + 0x20876d1] ; @0x142aced89 'StairsDecoratorType'
0x140a476b8 mov      r9d, 6
0x140a476be jmp      0x140a476e4 ; 
0x140a476c0 lea      rdx, [rsi + 8]
0x140a476c4 lea      r8, [rip + 0x208766a] ; @0x142aced35 'GothicWindowArrowSlitChimneyLanternFlag'
0x140a476cb mov      r9d, 0xc
0x140a476d1 jmp      0x140a476e4 ; 
0x140a476d3 lea      rdx, [rsi + 8]
0x140a476d7 lea      r8, [rip + 0x208767a] ; @0x142aced58 'Flag'
0x140a476de mov      r9d, 4
0x140a476e4 mov      rcx, rsi
0x140a476e7 call     0x141580d20 ; serde_json::ser::format_escaped_str::hbe26d48cff9c903f
0x140a476ec test     rax, rax
0x140a476ef jne      0x140a477dd
0x140a476f5 xor      eax, eax
0x140a476f7 add      rsp, 0x38
0x140a476fb pop      rbx
0x140a476fc pop      rdi
0x140a476fd pop      rsi
0x140a476fe pop      r14
0x140a47700 ret      
0x140a47701 mov      rdi, rcx
0x140a47704 lea      rdx, [rsi + 8]
0x140a47708 mov      rbx, qword ptr [rsi]
0x140a4770b mov      rax, qword ptr [rbx + 0x10]
0x140a4770f cmp      qword ptr [rbx], rax
0x140a47712 je       0x140a4781c
0x140a47718 mov      rcx, qword ptr [rbx + 8]
0x140a4771c mov      byte ptr [rcx + rax], 0x7b
0x140a47720 inc      rax
0x140a47723 mov      qword ptr [rbx + 0x10], rax
0x140a47727 lea      r8, [rip + 0x2087652] ; @0x142aced80 'CluttertyStairsDecoratorType'
0x140a4772e mov      r9d, 7
0x140a47734 mov      rcx, rsi
0x140a47737 call     0x141580d20 ; serde_json::ser::format_escaped_str::hbe26d48cff9c903f
0x140a4773c test     rax, rax
0x140a4773f jne      0x140a4780f
0x140a47745 mov      rbx, qword ptr [rsi]
0x140a47748 mov      rdx, qword ptr [rbx + 0x10]
0x140a4774c cmp      qword ptr [rbx], rdx
0x140a4774f je       0x140a4784b
0x140a47755 mov      rax, qword ptr [rbx + 8]
0x140a47759 mov      byte ptr [rax + rdx], 0x3a
0x140a4775d inc      rdx
0x140a47760 mov      qword ptr [rbx + 0x10], rdx
0x140a47764 mov      rbx, qword ptr [rsi]
0x140a47767 mov      rdx, qword ptr [rbx + 0x10]
0x140a4776b cmp      qword ptr [rbx], rdx
0x140a4776e je       0x140a47871
0x140a47774 mov      rax, qword ptr [rbx + 8]
0x140a47778 mov      byte ptr [rax + rdx], 0x7b
0x140a4777c inc      rdx
0x140a4777f mov      qword ptr [rbx + 0x10], rdx
0x140a47783 mov      qword ptr [rsp + 0x28], rsi
0x140a47788 mov      byte ptr [rsp + 0x30], 1
0x140a4778d lea      rdx, [rip + 0x20875f3] ; @0x142aced87 'tyStairsDecoratorType'
0x140a47794 lea      rcx, [rsp + 0x28]
0x140a47799 mov      r8d, 2
0x140a4779f mov      r9, rdi
0x140a477a2 call     0x14158a340 ; serde::ser::SerializeMap::serialize_entry::hb1e1f0d3b82b6344
0x140a477a7 test     rax, rax
0x140a477aa jne      0x140a476f7
0x140a477b0 mov      rax, qword ptr [rsp + 0x28]
0x140a477b5 mov      rsi, qword ptr [rax]
0x140a477b8 cmp      byte ptr [rsp + 0x30], 0
0x140a477bd je       0x140a477ee
0x140a477bf mov      rdx, qword ptr [rsi + 0x10]
0x140a477c3 cmp      qword ptr [rsi], rdx
0x140a477c6 je       0x140a478bd
0x140a477cc mov      rax, qword ptr [rsi + 8]
0x140a477d0 mov      byte ptr [rax + rdx], 0x7d
0x140a477d4 inc      rdx
0x140a477d7 mov      qword ptr [rsi + 0x10], rdx
0x140a477db jmp      0x140a477f2 ; 
0x140a477dd mov      rcx, rax
0x140a477e0 add      rsp, 0x38
0x140a477e4 pop      rbx
0x140a477e5 pop      rdi
0x140a477e6 pop      rsi
0x140a477e7 pop      r14
0x140a477e9 jmp      0x1428c98a0 ; serde_json::error::Error::io::hfcf27584d0d6aa9f
0x140a477ee mov      rdx, qword ptr [rsi + 0x10]
0x140a477f2 cmp      qword ptr [rsi], rdx
0x140a477f5 je       0x140a47897
0x140a477fb mov      rax, qword ptr [rsi + 8]
0x140a477ff mov      byte ptr [rax + rdx], 0x7d
0x140a47803 inc      rdx
0x140a47806 mov      qword ptr [rsi + 0x10], rdx
0x140a4780a jmp      0x140a476f5 ; 
0x140a4780f mov      rcx, rax
0x140a47812 call     0x1428c98a0 ; serde_json::error::Error::io::hfcf27584d0d6aa9f
0x140a47817 jmp      0x140a476f7 ; 
0x140a4781c mov      qword ptr [rsp + 0x20], 1
0x140a47825 mov      r8d, 1
0x140a4782b mov      r9d, 1
0x140a47831 mov      rcx, rbx
0x140a47834 mov      r14, rdx
0x140a47837 mov      rdx, rax
0x140a4783a call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140a4783f mov      rdx, r14
0x140a47842 mov      rax, qword ptr [rbx + 0x10]
0x140a47846 jmp      0x140a47718 ; 
0x140a4784b mov      qword ptr [rsp + 0x20], 1
0x140a47854 mov      r8d, 1
0x140a4785a mov      r9d, 1
0x140a47860 mov      rcx, rbx
0x140a47863 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140a47868 mov      rdx, qword ptr [rbx + 0x10]
0x140a4786c jmp      0x140a47755 ; 
0x140a47871 mov      qword ptr [rsp + 0x20], 1
0x140a4787a mov      r8d, 1
0x140a47880 mov      r9d, 1
0x140a47886 mov      rcx, rbx
0x140a47889 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140a4788e mov      rdx, qword ptr [rbx + 0x10]
0x140a47892 jmp      0x140a47774 ; 
0x140a47897 mov      qword ptr [rsp + 0x20], 1
0x140a478a0 mov      r8d, 1
0x140a478a6 mov      r9d, 1
0x140a478ac mov      rcx, rsi
0x140a478af call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140a478b4 mov      rdx, qword ptr [rsi + 0x10]
0x140a478b8 jmp      0x140a477fb ; 
0x140a478bd mov      qword ptr [rsp + 0x20], 1
0x140a478c6 mov      r8d, 1
0x140a478cc mov      r9d, 1
0x140a478d2 mov      rcx, rsi
0x140a478d5 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140a478da mov      rdx, qword ptr [rsi + 0x10]
0x140a478de jmp      0x140a477cc ; 