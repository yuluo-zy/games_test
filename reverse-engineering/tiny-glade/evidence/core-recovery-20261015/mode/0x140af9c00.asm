0x140af9c00 push     r15
0x140af9c02 push     r14
0x140af9c04 push     rsi
0x140af9c05 push     rdi
0x140af9c06 push     rbp
0x140af9c07 push     rbx
0x140af9c08 mov      r11, qword ptr [rcx]
0x140af9c0b cmp      dword ptr [r11 + 0x78], 3
0x140af9c10 mov      r10, qword ptr [rsp + 0x60]
0x140af9c15 mov      eax, dword ptr [rdx + 0x1c]
0x140af9c18 mov      rcx, qword ptr [rdx + 0x10]
0x140af9c1c mov      dword ptr [rcx], eax
0x140af9c1e mov      rdx, qword ptr [rdx]
0x140af9c21 setb     byte ptr [rdx]
0x140af9c24 mov      esi, dword ptr [r11 + 0x78]
0x140af9c28 lea      rdi, [rip + 0x1fec321]
0x140af9c2f movsxd   rsi, dword ptr [rdi + rsi*4]
0x140af9c33 add      rsi, rdi
0x140af9c36 jmp      rsi
0x140af9c38 xor      r11d, r11d
0x140af9c3b jmp      0x140af9c4e ; 
0x140af9c3d xorps    xmm0, xmm0
0x140af9c40 ucomiss  xmm0, dword ptr [r11 + 0x7c]
0x140af9c45 setae    r11b
0x140af9c49 jmp      0x140af9c4e ; 
0x140af9c4b mov      r11b, 1
0x140af9c4e mov      dword ptr [rcx], eax
0x140af9c50 mov      byte ptr [rdx + 1], r11b
0x140af9c54 mov      r10, qword ptr [r10]
0x140af9c57 mov      r11d, dword ptr [r9]
0x140af9c5a lea      esi, [r11 - 6]
0x140af9c5e mov      rdi, r11
0x140af9c61 add      rdi, -5
0x140af9c65 xor      ebx, ebx
0x140af9c67 cmp      esi, 0xf
0x140af9c6a cmovae   rdi, rbx
0x140af9c6e mov      esi, dword ptr [r10]
0x140af9c71 lea      r14, [rsi - 5]
0x140af9c75 cmp      rsi, 6
0x140af9c79 cmovb    r14, rbx
0x140af9c7d cmp      rdi, r14
0x140af9c80 jne      0x140af9d93
0x140af9c86 cmp      rdi, 6
0x140af9c8a ja       0x140af9d38
0x140af9c90 lea      rbx, [rip + 0x1fec2cd]
0x140af9c97 movsxd   rdi, dword ptr [rbx + rdi*4]
0x140af9c9b add      rdi, rbx
0x140af9c9e jmp      rdi
0x140af9ca0 movzx    r11d, byte ptr [r9 + 4]
0x140af9ca5 cmp      r11b, byte ptr [r10 + 4]
0x140af9ca9 je       0x140af9d38
0x140af9caf jmp      0x140af9d93 ; 
0x140af9cb4 cmp      esi, 5
0x140af9cb7 ja       0x140af9d38
0x140af9cb9 cmp      r11d, esi
0x140af9cbc jne      0x140af9d93
0x140af9cc2 mov      r11d, r11d
0x140af9cc5 lea      rsi, [rip + 0x1fec2b4]
0x140af9ccc movsxd   r11, dword ptr [rsi + r11*4]
0x140af9cd0 add      r11, rsi
0x140af9cd3 jmp      r11
0x140af9cd6 mov      r11d, dword ptr [r9 + 4]
0x140af9cda cmp      r11d, dword ptr [r10 + 4]
0x140af9cde je       0x140af9d38
0x140af9ce0 jmp      0x140af9d93 ; 
0x140af9ce5 mov      r11, qword ptr [r9 + 8]
0x140af9ce9 cmp      r11, qword ptr [r10 + 8]
0x140af9ced je       0x140af9d38
0x140af9cef jmp      0x140af9d93 ; 
0x140af9cf4 movzx    r11d, byte ptr [r9 + 4]
0x140af9cf9 lea      esi, [r11 - 0x38]
0x140af9cfd cmp      sil, 8
0x140af9d01 movzx    esi, sil
0x140af9d05 mov      ebx, 6
0x140af9d0a cmovae   esi, ebx
0x140af9d0d movzx    edi, byte ptr [r10 + 4]
0x140af9d12 lea      ebp, [rdi - 0x38]
0x140af9d15 cmp      bpl, 8
0x140af9d19 movzx    ebp, bpl
0x140af9d1d cmovae   ebp, ebx
0x140af9d20 cmp      sil, bpl
0x140af9d23 jne      0x140af9d93
0x140af9d25 cmp      sil, 6
0x140af9d29 sete     bl
0x140af9d2c cmp      r11b, dil
0x140af9d2f setne    r11b
0x140af9d33 test     r11b, bl
0x140af9d36 jne      0x140af9d93
0x140af9d38 mov      rsi, qword ptr [rsp + 0x58]
0x140af9d3d mov      r11, qword ptr [rsi]
0x140af9d40 mov      rsi, qword ptr [rsi + 8]
0x140af9d44 mov      rdi, qword ptr [r11]
0x140af9d47 xor      ebx, ebx
0x140af9d49 mov      r14, rdi
0x140af9d4c sub      r14, qword ptr [rsi + 0x18]
0x140af9d50 cmovb    r14, rbx
0x140af9d54 sub      rdi, qword ptr [rsi + 0x38]
0x140af9d58 cmovb    rdi, rbx
0x140af9d5c mov      r15, qword ptr [rsi + 0x30]
0x140af9d60 sub      r15, rdi
0x140af9d63 cmovb    r15, rbx
0x140af9d67 mov      rdi, qword ptr [rsi + 0x10]
0x140af9d6b setbe    bpl
0x140af9d6f sub      rdi, r14
0x140af9d72 cmovb    rdi, rbx
0x140af9d76 setbe    r14b
0x140af9d7a add      rdi, r15
0x140af9d7d test     r14b, bpl
0x140af9d80 cmove    rdi, rbx
0x140af9d84 mov      rsi, qword ptr [rsi + 0x40]
0x140af9d88 sub      rsi, rdi
0x140af9d8b mov      qword ptr [r11], rsi
0x140af9d8e test     r14b, bpl
0x140af9d91 jne      0x140af9d98
0x140af9d93 mov      byte ptr [r9 + 0x10], 0
0x140af9d98 movups   xmm0, xmmword ptr [r10]
0x140af9d9c movups   xmmword ptr [r9], xmm0
0x140af9da0 mov      r8, qword ptr [r8]
0x140af9da3 mov      r10d, dword ptr [r8 + 0x138]
0x140af9daa lea      r11, [rip + 0x1fec1e7]
0x140af9db1 movsxd   r10, dword ptr [r11 + r10*4]
0x140af9db5 add      r10, r11
0x140af9db8 jmp      r10
0x140af9dbb movzx    r8d, byte ptr [r9 + 0x10]
0x140af9dc0 jmp      0x140af9de6 ; 
0x140af9dc2 xorps    xmm0, xmm0
0x140af9dc5 ucomiss  xmm0, dword ptr [r8 + 0x13c]
0x140af9dcd movzx    r8d, byte ptr [r9 + 0x10]
0x140af9dd2 jae      0x140af9ddb
0x140af9dd4 jmp      0x140af9de6 ; 
0x140af9dd6 movzx    r8d, byte ptr [r9 + 0x10]
0x140af9ddb add      r9, 0x10
0x140af9ddf xor      r8b, 1
0x140af9de3 mov      byte ptr [r9], r8b
0x140af9de6 test     r8b, 1
0x140af9dea je       0x140af9df1
0x140af9dec mov      dword ptr [rcx], eax
0x140af9dee mov      byte ptr [rdx], 1
0x140af9df1 pop      rbx
0x140af9df2 pop      rbp
0x140af9df3 pop      rdi
0x140af9df4 pop      rsi
0x140af9df5 pop      r14
0x140af9df7 pop      r15
0x140af9df9 ret      