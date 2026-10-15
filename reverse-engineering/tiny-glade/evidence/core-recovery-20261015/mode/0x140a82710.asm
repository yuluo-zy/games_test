0x140a82710 push     rbp
0x140a82711 push     rsi
0x140a82712 push     rdi
0x140a82713 push     rbx
0x140a82714 sub      rsp, 0x48
0x140a82718 lea      rbp, [rsp + 0x40]
0x140a8271d mov      qword ptr [rbp], 0xfffffffffffffffe
0x140a82725 mov      rax, qword ptr [rcx]
0x140a82728 cmp      rax, 0xd
0x140a8272c ja       0x140a82862
0x140a82732 lea      rdx, [rip + 0x20560df]
0x140a82739 movsxd   rax, dword ptr [rdx + rax*4]
0x140a8273d add      rax, rdx
0x140a82740 jmp      rax
0x140a82742 mov      rax, qword ptr [rcx + 8]
0x140a82746 lock dec qword ptr [rax]
0x140a8274a jne      0x140a8289d
0x140a82750 add      rcx, 8
0x140a82754 add      rsp, 0x48
0x140a82758 pop      rbx
0x140a82759 pop      rdi
0x140a8275a pop      rsi
0x140a8275b pop      rbp
0x140a8275c jmp      0x140b5cfc0 ; alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::hbcb6927a13bbf4f2
0x140a82761 mov      rax, qword ptr [rcx + 8]
0x140a82765 lock dec qword ptr [rax]
0x140a82769 jne      0x140a8289d
0x140a8276f add      rcx, 8
0x140a82773 add      rsp, 0x48
0x140a82777 pop      rbx
0x140a82778 pop      rdi
0x140a82779 pop      rsi
0x140a8277a pop      rbp
0x140a8277b jmp      0x140b5d010 ; alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::hbe49ad65a4eee4c5 / alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::hf8b9db1fc7c70ae8
0x140a82780 mov      rax, qword ptr [rcx + 8]
0x140a82784 lock dec qword ptr [rax]
0x140a82788 jne      0x140a8289d
0x140a8278e add      rcx, 8
0x140a82792 add      rsp, 0x48
0x140a82796 pop      rbx
0x140a82797 pop      rdi
0x140a82798 pop      rsi
0x140a82799 pop      rbp
0x140a8279a jmp      0x140b5c040 ; alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::h145ea11deda7c183
0x140a8279f mov      rax, qword ptr [rcx + 8]
0x140a827a3 lock dec qword ptr [rax]
0x140a827a7 jne      0x140a8289d
0x140a827ad add      rcx, 8
0x140a827b1 add      rsp, 0x48
0x140a827b5 pop      rbx
0x140a827b6 pop      rdi
0x140a827b7 pop      rsi
0x140a827b8 pop      rbp
0x140a827b9 jmp      0x140b5cd40 ; alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::h8a504105d8a66ff8
0x140a827be mov      rax, qword ptr [rcx + 8]
0x140a827c2 lock dec qword ptr [rax]
0x140a827c6 jne      0x140a8289d
0x140a827cc add      rcx, 8
0x140a827d0 add      rsp, 0x48
0x140a827d4 pop      rbx
0x140a827d5 pop      rdi
0x140a827d6 pop      rsi
0x140a827d7 pop      rbp
0x140a827d8 jmp      0x140b5c2a0 ; alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::h28f4482cd6b59500
0x140a827dd mov      rax, qword ptr [rcx + 8]
0x140a827e1 lock dec qword ptr [rax]
0x140a827e5 jne      0x140a8289d
0x140a827eb add      rcx, 8
0x140a827ef add      rsp, 0x48
0x140a827f3 pop      rbx
0x140a827f4 pop      rdi
0x140a827f5 pop      rsi
0x140a827f6 pop      rbp
0x140a827f7 jmp      0x1402e5d80 ; alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::h359ec8eb5ea8a918 / alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::hc8cf01ba4f172df6
0x140a827fc mov      rax, qword ptr [rcx + 8]
0x140a82800 lock dec qword ptr [rax]
0x140a82804 jne      0x140a8289d
0x140a8280a add      rcx, 8
0x140a8280e add      rsp, 0x48
0x140a82812 pop      rbx
0x140a82813 pop      rdi
0x140a82814 pop      rsi
0x140a82815 pop      rbp
0x140a82816 jmp      0x140b5d010 ; alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::hbe49ad65a4eee4c5 / alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::hf8b9db1fc7c70ae8
0x140a8281b mov      rdx, qword ptr [rcx + 8]
0x140a8281f mov      rax, qword ptr [rcx + 0x10]
0x140a82823 mov      qword ptr [rbp - 0x10], rax
0x140a82827 mov      qword ptr [rbp - 0x18], 0
0x140a8282f mov      qword ptr [rbp - 8], rdx
0x140a82833 mov      rcx, rdx
0x140a82836 nop      word ptr cs:[rax + rax]
0x140a82840 mov      rdx, qword ptr [rbp - 0x10]
0x140a82844 mov      rax, qword ptr [rbp - 0x18]
0x140a82848 cmp      rdx, rax
0x140a8284b je       0x140a8287d
0x140a8284d inc      rax
0x140a82850 mov      qword ptr [rbp - 0x18], rax
0x140a82854 lea      rsi, [rcx + 0x10]
0x140a82858 call     0x140897380 ; core::ptr::drop_in_place$LT$country_core..resources..history..wall_compound_edit..ConcurrentEdits$GT$::h1b2099b7ae6b7c18
0x140a8285d mov      rcx, rsi
0x140a82860 jmp      0x140a82840 ; 
0x140a82862 mov      rax, qword ptr [rcx + 8]
0x140a82866 lock dec qword ptr [rax]
0x140a8286a jne      0x140a8289d
0x140a8286c add      rcx, 8
0x140a82870 add      rsp, 0x48
0x140a82874 pop      rbx
0x140a82875 pop      rdi
0x140a82876 pop      rsi
0x140a82877 pop      rbp
0x140a82878 jmp      0x1406978e0 ; _$LT$alloc..sync..Weak$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$::drop::h0d608b4fde675181 / alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::hda2fa96c289317cd
0x140a8287d test     rdx, rdx
0x140a82880 je       0x140a8289d
0x140a82882 shl      rdx, 4
0x140a82886 mov      r8d, 8
0x140a8288c mov      rcx, qword ptr [rbp - 8]
0x140a82890 add      rsp, 0x48
0x140a82894 pop      rbx
0x140a82895 pop      rdi
0x140a82896 pop      rsi
0x140a82897 pop      rbp
0x140a82898 jmp      0x140613c20 ; __rust_dealloc
0x140a8289d nop      
0x140a8289e add      rsp, 0x48
0x140a828a2 pop      rbx
0x140a828a3 pop      rdi
0x140a828a4 pop      rsi
0x140a828a5 pop      rbp
0x140a828a6 ret      
0x140a828a7 nop      word ptr [rax + rax]
0x140a828b0 mov      qword ptr [rsp + 0x10], rdx
0x140a828b5 push     rbp
0x140a828b6 push     rsi
0x140a828b7 push     rdi
0x140a828b8 push     rbx
0x140a828b9 sub      rsp, 0x28
0x140a828bd lea      rbp, [rdx + 0x40]
0x140a828c1 mov      rsi, qword ptr [rbp - 0x10]
0x140a828c5 mov      rbx, qword ptr [rbp - 0x18]
0x140a828c9 cmp      rsi, rbx
0x140a828cc mov      rdi, qword ptr [rbp - 8]
0x140a828d0 je       0x140a828e9
0x140a828d2 mov      rcx, rbx
0x140a828d5 shl      rcx, 4
0x140a828d9 add      rcx, rdi
0x140a828dc call     0x140897380 ; core::ptr::drop_in_place$LT$country_core..resources..history..wall_compound_edit..ConcurrentEdits$GT$::h1b2099b7ae6b7c18
0x140a828e1 inc      rbx
0x140a828e4 cmp      rbx, rsi
0x140a828e7 jne      0x140a828d2
0x140a828e9 add      rsp, 0x28
0x140a828ed pop      rbx
0x140a828ee pop      rdi
0x140a828ef pop      rsi
0x140a828f0 pop      rbp
0x140a828f1 ret      
0x140a828f2 nop      word ptr cs:[rax + rax]
0x140a82900 mov      qword ptr [rsp + 0x10], rdx
0x140a82905 push     rbp
0x140a82906 push     rsi
0x140a82907 push     rdi
0x140a82908 push     rbx
0x140a82909 sub      rsp, 0x28
0x140a8290d lea      rbp, [rdx + 0x40]
0x140a82911 mov      rdx, qword ptr [rbp - 0x10]
0x140a82915 shl      rdx, 4
0x140a82919 mov      r8d, 8
0x140a8291f mov      rcx, qword ptr [rbp - 8]
0x140a82923 call     0x140613c20 ; __rust_dealloc
0x140a82928 nop      
0x140a82929 add      rsp, 0x28
0x140a8292d pop      rbx
0x140a8292e pop      rdi
0x140a8292f pop      rsi
0x140a82930 pop      rbp
0x140a82931 ret      