0x140af89d0 push     r15
0x140af89d2 push     r14
0x140af89d4 push     r13
0x140af89d6 push     r12
0x140af89d8 push     rsi
0x140af89d9 push     rdi
0x140af89da push     rbp
0x140af89db push     rbx
0x140af89dc sub      rsp, 0x10
0x140af89e0 mov      r8, qword ptr [rcx + 0x10]
0x140af89e4 test     r8, r8
0x140af89e7 je       0x140af8bfe
0x140af89ed mov      rcx, qword ptr [rcx + 8]
0x140af89f1 mov      edi, dword ptr [rdx]
0x140af89f3 lea      rax, [rdi - 5]
0x140af89f7 xor      r10d, r10d
0x140af89fa cmp      rdi, 6
0x140af89fe cmovae   r10, rax
0x140af8a02 movzx    r9d, byte ptr [rdx + 4]
0x140af8a07 lea      eax, [r9 - 0x38]
0x140af8a0b cmp      al, 8
0x140af8a0d movzx    r11d, al
0x140af8a11 mov      ebx, 6
0x140af8a16 cmovae   r11d, ebx
0x140af8a1a mov      rsi, qword ptr [rdx + 8]
0x140af8a1e cmp      rdi, 5
0x140af8a22 ja       0x140af8b0b
0x140af8a28 mov      eax, dword ptr [rdx + 4]
0x140af8a2b mov      dword ptr [rsp + 4], eax
0x140af8a2f shl      r8, 4
0x140af8a33 xor      r14d, r14d
0x140af8a36 lea      r15, [rip + 0x1fed493]
0x140af8a3d mov      eax, edi
0x140af8a3f mov      qword ptr [rsp + 8], rax
0x140af8a44 xor      ebp, ebp
0x140af8a46 jmp      0x140af8a6d ; 
0x140af8a48 cmp      qword ptr [rcx + rbp + 8], rsi
0x140af8a4d je       0x140af8c00
0x140af8a53 nop      word ptr cs:[rax + rax]
0x140af8a60 add      rbp, 0x10
0x140af8a64 cmp      r8, rbp
0x140af8a67 je       0x140af8bfe
0x140af8a6d mov      edx, dword ptr [rcx + rbp]
0x140af8a70 lea      eax, [rdx - 6]
0x140af8a73 mov      r13, rdx
0x140af8a76 add      r13, -5
0x140af8a7a cmp      eax, 0xf
0x140af8a7d cmovae   r13, r14
0x140af8a81 cmp      r13, r10
0x140af8a84 jne      0x140af8a60
0x140af8a86 mov      al, 1
0x140af8a88 cmp      r10, 6
0x140af8a8c ja       0x140af8c00
0x140af8a92 movsxd   r13, dword ptr [r15 + r10*4]
0x140af8a96 add      r13, r15
0x140af8a99 jmp      r13
0x140af8a9c cmp      byte ptr [rcx + rbp + 4], r9b
0x140af8aa1 jne      0x140af8a60
0x140af8aa3 jmp      0x140af8c00 ; 
0x140af8aa8 cmp      edx, edi
0x140af8aaa jne      0x140af8a60
0x140af8aac mov      rdx, qword ptr [rsp + 8]
0x140af8ab1 lea      r12, [rip + 0x1fed434]
0x140af8ab8 movsxd   rdx, dword ptr [r12 + rdx*4]
0x140af8abc add      rdx, r12
0x140af8abf jmp      rdx
0x140af8ac1 mov      edx, dword ptr [rsp + 4]
0x140af8ac5 cmp      dword ptr [rcx + rbp + 4], edx
0x140af8ac9 jne      0x140af8a60
0x140af8acb jmp      0x140af8c00 ; 
0x140af8ad0 cmp      r11b, 6
0x140af8ad4 setne    dl
0x140af8ad7 movzx    r13d, byte ptr [rcx + rbp + 4]
0x140af8add lea      r12d, [r13 - 0x38]
0x140af8ae1 cmp      r12b, 8
0x140af8ae5 movzx    r12d, r12b
0x140af8ae9 cmovae   r12d, ebx
0x140af8aed cmp      r13b, r9b
0x140af8af0 sete     r13b
0x140af8af4 cmp      r12b, r11b
0x140af8af7 jne      0x140af8a60
0x140af8afd or       dl, r13b
0x140af8b00 je       0x140af8a60
0x140af8b06 jmp      0x140af8c00 ; 
0x140af8b0b lea      eax, [rdi - 6]
0x140af8b0e cmp      eax, 5
0x140af8b11 ja       0x140af8c11
0x140af8b17 lea      rdx, [rip + 0x1fed39a]
0x140af8b1e movsxd   rax, dword ptr [rdx + rax*4]
0x140af8b22 add      rax, rdx
0x140af8b25 jmp      rax
0x140af8b27 shl      r8, 4
0x140af8b2b xor      eax, eax
0x140af8b2d jmp      0x140af8b3d ; 
0x140af8b2f nop      
0x140af8b30 add      rax, 0x10
0x140af8b34 cmp      r8, rax
0x140af8b37 je       0x140af8bfe
0x140af8b3d cmp      dword ptr [rcx + rax], 6
0x140af8b41 jne      0x140af8b30
0x140af8b43 cmp      byte ptr [rcx + rax + 4], r9b
0x140af8b48 jne      0x140af8b30
0x140af8b4a jmp      0x140af8bfa ; 
0x140af8b4f shl      r8, 4
0x140af8b53 xor      eax, eax
0x140af8b55 jmp      0x140af8b6d ; 
0x140af8b57 nop      word ptr [rax + rax]
0x140af8b60 add      rax, 0x10
0x140af8b64 cmp      r8, rax
0x140af8b67 je       0x140af8bfe
0x140af8b6d cmp      dword ptr [rcx + rax], 9
0x140af8b71 jne      0x140af8b60
0x140af8b73 cmp      byte ptr [rcx + rax + 4], r9b
0x140af8b78 jne      0x140af8b60
0x140af8b7a jmp      0x140af8bfa ; 
0x140af8b7c shl      r8, 4
0x140af8b80 xor      eax, eax
0x140af8b82 jmp      0x140af8b99 ; 
0x140af8b84 nop      word ptr cs:[rax + rax]
0x140af8b90 add      rax, 0x10
0x140af8b94 cmp      r8, rax
0x140af8b97 je       0x140af8bfe
0x140af8b99 cmp      dword ptr [rcx + rax], 8
0x140af8b9d jne      0x140af8b90
0x140af8b9f cmp      qword ptr [rcx + rax + 8], rsi
0x140af8ba4 jne      0x140af8b90
0x140af8ba6 jmp      0x140af8bfa ; 
0x140af8ba8 shl      r8, 4
0x140af8bac xor      eax, eax
0x140af8bae mov      edx, 6
0x140af8bb3 jmp      0x140af8bc9 ; 
0x140af8bb5 nop      word ptr cs:[rax + rax]
0x140af8bc0 add      rax, 0x10
0x140af8bc4 cmp      r8, rax
0x140af8bc7 je       0x140af8bfe
0x140af8bc9 cmp      dword ptr [rcx + rax], 0xb
0x140af8bcd jne      0x140af8bc0
0x140af8bcf cmp      r11b, 6
0x140af8bd3 setne    r10b
0x140af8bd7 movzx    esi, byte ptr [rcx + rax + 4]
0x140af8bdc lea      edi, [rsi - 0x38]
0x140af8bdf cmp      dil, 8
0x140af8be3 movzx    edi, dil
0x140af8be7 cmovae   edi, edx
0x140af8bea cmp      sil, r9b
0x140af8bed sete     bl
0x140af8bf0 cmp      dil, r11b
0x140af8bf3 jne      0x140af8bc0
0x140af8bf5 or       r10b, bl
0x140af8bf8 je       0x140af8bc0
0x140af8bfa mov      al, 1
0x140af8bfc jmp      0x140af8c00 ; 
0x140af8bfe xor      eax, eax
0x140af8c00 add      rsp, 0x10
0x140af8c04 pop      rbx
0x140af8c05 pop      rbp
0x140af8c06 pop      rdi
0x140af8c07 pop      rsi
0x140af8c08 pop      r12
0x140af8c0a pop      r13
0x140af8c0c pop      r14
0x140af8c0e pop      r15
0x140af8c10 ret      
0x140af8c11 shl      r8, 4
0x140af8c15 add      r8, -0x10
0x140af8c19 xor      edx, edx
0x140af8c1b xor      r9d, r9d
0x140af8c1e nop      
0x140af8c20 mov      eax, dword ptr [rcx + r9]
0x140af8c24 lea      r11d, [rax - 6]
0x140af8c28 add      rax, -5
0x140af8c2c cmp      r11d, 0xf
0x140af8c30 cmovae   rax, rdx
0x140af8c34 cmp      rax, r10
0x140af8c37 sete     al
0x140af8c3a je       0x140af8c00
0x140af8c3c lea      r11, [r9 + 0x10]
0x140af8c40 cmp      r8, r9
0x140af8c43 mov      r9, r11
0x140af8c46 jne      0x140af8c20
0x140af8c48 jmp      0x140af8c00 ; 