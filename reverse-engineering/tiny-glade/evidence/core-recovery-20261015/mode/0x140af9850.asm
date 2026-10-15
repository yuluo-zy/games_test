0x140af9850 push     r15
0x140af9852 push     r14
0x140af9854 push     r13
0x140af9856 push     r12
0x140af9858 push     rsi
0x140af9859 push     rdi
0x140af985a push     rbp
0x140af985b push     rbx
0x140af985c sub      rsp, 0x88
0x140af9863 mov      rdi, rdx
0x140af9866 mov      rax, rcx
0x140af9869 mov      rcx, qword ptr [rcx + 8]
0x140af986d mov      qword ptr [rsp + 0x40], rax
0x140af9872 mov      rsi, qword ptr [rax + 0x10]
0x140af9876 shl      rsi, 5
0x140af987a mov      ebx, 1
0x140af987f nop      
0x140af9880 test     rsi, rsi
0x140af9883 je       0x140af9b80
0x140af9889 lea      r14, [rcx + 0x20]
0x140af988d mov      rdx, rdi
0x140af9890 call     0x140af89d0 ; country_core::resources::toolbar::ToolbarCategoryModes::contains::hb49862b03274002a
0x140af9895 add      rsi, -0x20
0x140af9899 dec      rbx
0x140af989c mov      rcx, r14
0x140af989f test     al, al
0x140af98a1 je       0x140af9880
0x140af98a3 mov      rax, rbx
0x140af98a6 neg      rax
0x140af98a9 cmp      rax, 0xb
0x140af98ad jae      0x140af9bd3
0x140af98b3 mov      rax, qword ptr [r14 - 0x10]
0x140af98b7 test     rax, rax
0x140af98ba je       0x140af9b74
0x140af98c0 mov      rcx, qword ptr [r14 - 0x18]
0x140af98c4 mov      r9d, dword ptr [rdi]
0x140af98c7 lea      rdx, [r9 - 5]
0x140af98cb xor      r10d, r10d
0x140af98ce cmp      r9, 6
0x140af98d2 cmovae   r10, rdx
0x140af98d6 movzx    r8d, byte ptr [rdi + 4]
0x140af98db lea      edx, [r8 - 0x38]
0x140af98df cmp      dl, 8
0x140af98e2 movzx    edx, dl
0x140af98e5 mov      r11d, 6
0x140af98eb cmovae   edx, r11d
0x140af98ef mov      dword ptr [rsp + 0x34], edx
0x140af98f3 mov      r15, qword ptr [rdi + 8]
0x140af98f7 cmp      r9, 5
0x140af98fb ja       0x140af9a0e
0x140af9901 mov      edx, dword ptr [rdi + 4]
0x140af9904 mov      dword ptr [rsp + 0x30], edx
0x140af9908 shl      rax, 4
0x140af990c xor      r12d, r12d
0x140af990f lea      r13, [rip + 0x1fec606]
0x140af9916 mov      edx, r9d
0x140af9919 mov      qword ptr [rsp + 0x38], rdx
0x140af991e xor      edi, edi
0x140af9920 xor      edx, edx
0x140af9922 jmp      0x140af9940 ; 
0x140af9924 cmp      qword ptr [rcx + rdi + 8], r15
0x140af9929 je       0x140af9b15
0x140af992f nop      
0x140af9930 inc      rdx
0x140af9933 add      rdi, 0x10
0x140af9937 cmp      rax, rdi
0x140af993a je       0x140af9b74
0x140af9940 mov      r11d, dword ptr [rcx + rdi]
0x140af9944 lea      ebp, [r11 - 6]
0x140af9948 mov      rsi, r11
0x140af994b add      rsi, -5
0x140af994f cmp      ebp, 0xf
0x140af9952 cmovae   rsi, r12
0x140af9956 cmp      rsi, r10
0x140af9959 jne      0x140af9930
0x140af995b cmp      r10, 6
0x140af995f ja       0x140af9b15
0x140af9965 movsxd   rsi, dword ptr [r13 + r10*4]
0x140af996a add      rsi, r13
0x140af996d jmp      rsi
0x140af996f cmp      byte ptr [rcx + rdi + 4], r8b
0x140af9974 jne      0x140af9930
0x140af9976 jmp      0x140af9b15 ; 
0x140af997b cmp      r11d, r9d
0x140af997e jne      0x140af9930
0x140af9980 mov      r11, qword ptr [rsp + 0x38]
0x140af9985 lea      rsi, [rip + 0x1fec5ac]
0x140af998c movsxd   r11, dword ptr [rsi + r11*4]
0x140af9990 add      r11, rsi
0x140af9993 jmp      r11
0x140af9996 mov      r11d, dword ptr [rsp + 0x30]
0x140af999b cmp      dword ptr [rcx + rdi + 4], r11d
0x140af99a0 jne      0x140af9930
0x140af99a2 jmp      0x140af9b15 ; 
0x140af99a7 mov      r12, r8
0x140af99aa mov      r8d, dword ptr [rsp + 0x34]
0x140af99af cmp      r8b, 6
0x140af99b3 setne    byte ptr [rsp + 0x2f]
0x140af99b8 movzx    r11d, byte ptr [rcx + rdi + 4]
0x140af99be lea      ebp, [r11 - 0x38]
0x140af99c2 cmp      bpl, 8
0x140af99c6 mov      rsi, r13
0x140af99c9 mov      r13, r9
0x140af99cc mov      r9, r15
0x140af99cf movzx    r15d, bpl
0x140af99d3 mov      ebp, 6
0x140af99d8 cmovae   r15d, ebp
0x140af99dc cmp      r11b, r12b
0x140af99df sete     bpl
0x140af99e3 cmp      r15b, r8b
0x140af99e6 mov      r8, r12
0x140af99e9 mov      r12d, 0
0x140af99ef mov      r15, r9
0x140af99f2 mov      r9, r13
0x140af99f5 mov      r13, rsi
0x140af99f8 jne      0x140af9930
0x140af99fe or       byte ptr [rsp + 0x2f], bpl
0x140af9a03 je       0x140af9930
0x140af9a09 jmp      0x140af9b15 ; 
0x140af9a0e lea      edx, [r9 - 6]
0x140af9a12 cmp      edx, 5
0x140af9a15 ja       0x140af9b37
0x140af9a1b lea      rsi, [rip + 0x1fec4e2]
0x140af9a22 movsxd   rdx, dword ptr [rsi + rdx*4]
0x140af9a26 add      rdx, rsi
0x140af9a29 jmp      rdx
0x140af9a2b shl      rax, 4
0x140af9a2f xor      r9d, r9d
0x140af9a32 xor      edx, edx
0x140af9a34 jmp      0x140af9a50 ; 
0x140af9a36 nop      word ptr cs:[rax + rax]
0x140af9a40 inc      rdx
0x140af9a43 add      r9, 0x10
0x140af9a47 cmp      rax, r9
0x140af9a4a je       0x140af9b74
0x140af9a50 cmp      dword ptr [rcx + r9], 6
0x140af9a55 jne      0x140af9a40
0x140af9a57 cmp      byte ptr [rcx + r9 + 4], r8b
0x140af9a5c jne      0x140af9a40
0x140af9a5e jmp      0x140af9b15 ; 
0x140af9a63 shl      rax, 4
0x140af9a67 xor      r9d, r9d
0x140af9a6a xor      edx, edx
0x140af9a6c jmp      0x140af9a80 ; 
0x140af9a6e nop      
0x140af9a70 inc      rdx
0x140af9a73 add      r9, 0x10
0x140af9a77 cmp      rax, r9
0x140af9a7a je       0x140af9b74
0x140af9a80 cmp      dword ptr [rcx + r9], 9
0x140af9a85 jne      0x140af9a70
0x140af9a87 cmp      byte ptr [rcx + r9 + 4], r8b
0x140af9a8c jne      0x140af9a70
0x140af9a8e jmp      0x140af9b15 ; 
0x140af9a93 shl      rax, 4
0x140af9a97 xor      r8d, r8d
0x140af9a9a xor      edx, edx
0x140af9a9c jmp      0x140af9ab0 ; 
0x140af9a9e nop      
0x140af9aa0 inc      rdx
0x140af9aa3 add      r8, 0x10
0x140af9aa7 cmp      rax, r8
0x140af9aaa je       0x140af9b74
0x140af9ab0 cmp      dword ptr [rcx + r8], 8
0x140af9ab5 jne      0x140af9aa0
0x140af9ab7 cmp      qword ptr [rcx + r8 + 8], r15
0x140af9abc jne      0x140af9aa0
0x140af9abe jmp      0x140af9b15 ; 
0x140af9ac0 shl      rax, 4
0x140af9ac4 xor      r10d, r10d
0x140af9ac7 xor      edx, edx
0x140af9ac9 mov      r9d, dword ptr [rsp + 0x34]
0x140af9ace jmp      0x140af9ae0 ; 
0x140af9ad0 inc      rdx
0x140af9ad3 add      r10, 0x10
0x140af9ad7 cmp      rax, r10
0x140af9ada je       0x140af9b74
0x140af9ae0 cmp      dword ptr [rcx + r10], 0xb
0x140af9ae5 jne      0x140af9ad0
0x140af9ae7 cmp      r9b, 6
0x140af9aeb setne    sil
0x140af9aef movzx    edi, byte ptr [rcx + r10 + 4]
0x140af9af5 lea      ebp, [rdi - 0x38]
0x140af9af8 cmp      bpl, 8
0x140af9afc movzx    ebp, bpl
0x140af9b00 cmovae   ebp, r11d
0x140af9b04 cmp      dil, r8b
0x140af9b07 sete     dil
0x140af9b0b cmp      bpl, r9b
0x140af9b0e jne      0x140af9ad0
0x140af9b10 or       sil, dil
0x140af9b13 je       0x140af9ad0
0x140af9b15 mov      qword ptr [r14 - 8], rdx
0x140af9b19 neg      bl
0x140af9b1b mov      rax, qword ptr [rsp + 0x40]
0x140af9b20 mov      byte ptr [rax + 0x18], bl
0x140af9b23 add      rsp, 0x88
0x140af9b2a pop      rbx
0x140af9b2b pop      rbp
0x140af9b2c pop      rdi
0x140af9b2d pop      rsi
0x140af9b2e pop      r12
0x140af9b30 pop      r13
0x140af9b32 pop      r14
0x140af9b34 pop      r15
0x140af9b36 ret      
0x140af9b37 shl      rax, 4
0x140af9b3b xor      r8d, r8d
0x140af9b3e xor      r9d, r9d
0x140af9b41 xor      edx, edx
0x140af9b43 nop      word ptr cs:[rax + rax]
0x140af9b50 mov      r11d, dword ptr [rcx + r9]
0x140af9b54 lea      esi, [r11 - 6]
0x140af9b58 add      r11, -5
0x140af9b5c cmp      esi, 0xf
0x140af9b5f cmovae   r11, r8
0x140af9b63 cmp      r11, r10
0x140af9b66 je       0x140af9b15
0x140af9b68 inc      rdx
0x140af9b6b add      r9, 0x10
0x140af9b6f cmp      rax, r9
0x140af9b72 jne      0x140af9b50
0x140af9b74 lea      rcx, [rip + 0x1febb15]
0x140af9b7b call     0x1428d9310 ; core::option::unwrap_failed::h837cefa1ab459360
0x140af9b80 mov      qword ptr [rsp + 0x48], rdi
0x140af9b85 lea      rax, [rip + 0x2a14]
0x140af9b8c mov      qword ptr [rsp + 0x50], rax
0x140af9b91 lea      rax, [rip + 0x1febb58]
0x140af9b98 mov      qword ptr [rsp + 0x58], rax
0x140af9b9d mov      qword ptr [rsp + 0x60], 2
0x140af9ba6 mov      qword ptr [rsp + 0x78], 0
0x140af9baf lea      rax, [rsp + 0x48]
0x140af9bb4 mov      qword ptr [rsp + 0x68], rax
0x140af9bb9 mov      qword ptr [rsp + 0x70], 1
0x140af9bc2 lea      rdx, [rip + 0x1febb47]
0x140af9bc9 lea      rcx, [rsp + 0x58]
0x140af9bce call     0x1428d9390 ; core::panicking::panic_fmt::h57d10e7f426973d3
0x140af9bd3 lea      rcx, [rip + 0x1feba76] ; @0x142ae5650 'internal error: entered unreachable code'
0x140af9bda lea      r8, [rip + 0x1feba97]
0x140af9be1 mov      edx, 0x28
0x140af9be6 call     0x1428d9430 ; core::panicking::panic::hb210ae7b817a9f13
0x140af9beb int3     