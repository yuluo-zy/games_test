0x140a489e0 push     rsi
0x140a489e1 push     rdi
0x140a489e2 push     rbx
0x140a489e3 push     rax
0x140a489e4 add      qword ptr [rdx + 0x30], 4
0x140a489e9 mov      dword ptr [rsp + 4], ecx
0x140a489ed mov      r8, qword ptr [rdx + 0x40]
0x140a489f1 lea      r11, [rsp + 8]
0x140a489f6 test     r8, r8
0x140a489f9 je       0x140a48a8b
0x140a489ff mov      r10d, 8
0x140a48a05 sub      r10, r8
0x140a48a08 xor      esi, esi
0x140a48a0a xor      ebx, ebx
0x140a48a0c cmp      r10, 4
0x140a48a10 mov      edi, 4
0x140a48a15 cmovb    rdi, r10
0x140a48a19 mov      r9d, ecx
0x140a48a1c cmovae   rsi, r9
0x140a48a20 setae    bl
0x140a48a23 lea      eax, [rbx*4]
0x140a48a2a lea      ecx, [rbx*4 + 1]
0x140a48a31 cmp      ecx, edi
0x140a48a33 jae      0x140a48a54
0x140a48a35 xor      ecx, ecx
0x140a48a37 cmp      r10, 4
0x140a48a3b setae    cl
0x140a48a3e movzx    ebx, word ptr [rsp + rcx*4 + 4]
0x140a48a43 lea      ecx, [rax*8]
0x140a48a4a shl      rbx, cl
0x140a48a4d or       rsi, rbx
0x140a48a50 or       rax, 2
0x140a48a54 cmp      rax, rdi
0x140a48a57 jae      0x140a48a69
0x140a48a59 movzx    edi, byte ptr [rsp + rax + 4]
0x140a48a5e shl      eax, 3
0x140a48a61 mov      ecx, eax
0x140a48a63 shl      rdi, cl
0x140a48a66 or       rsi, rdi
0x140a48a69 lea      ecx, [r8*8]
0x140a48a71 shl      rsi, cl
0x140a48a74 or       rsi, qword ptr [rdx + 0x38]
0x140a48a78 mov      qword ptr [rdx + 0x38], rsi
0x140a48a7c cmp      r10, 4
0x140a48a80 jbe      0x140a48a90
0x140a48a82 add      r8, 4
0x140a48a86 jmp      0x140a48b37 ; 
0x140a48a8b mov      r9d, ecx
0x140a48a8e jmp      0x140a48aea ; 
0x140a48a90 mov      rcx, qword ptr [rdx + 0x18]
0x140a48a94 xor      rcx, rsi
0x140a48a97 mov      rdi, qword ptr [rdx + 0x10]
0x140a48a9b mov      rax, qword ptr [rdx]
0x140a48a9e add      rax, rdi
0x140a48aa1 mov      rbx, qword ptr [rdx + 8]
0x140a48aa5 add      rbx, rcx
0x140a48aa8 rol      rdi, 0xd
0x140a48aac rol      rcx, 0x10
0x140a48ab0 xor      rdi, rax
0x140a48ab3 xor      rcx, rbx
0x140a48ab6 rol      rax, 0x20
0x140a48aba add      rbx, rdi
0x140a48abd add      rax, rcx
0x140a48ac0 rol      rdi, 0x11
0x140a48ac4 xor      rdi, rbx
0x140a48ac7 mov      qword ptr [rdx + 0x10], rdi
0x140a48acb rol      rcx, 0x15
0x140a48acf xor      rcx, rax
0x140a48ad2 mov      qword ptr [rdx + 0x18], rcx
0x140a48ad6 rol      rbx, 0x20
0x140a48ada mov      qword ptr [rdx + 8], rbx
0x140a48ade xor      rax, rsi
0x140a48ae1 mov      qword ptr [rdx], rax
0x140a48ae4 cmp      r8, 8
0x140a48ae8 jne      0x140a48b43
0x140a48aea mov      r8d, 4
0x140a48af0 xor      r10d, r10d
0x140a48af3 mov      eax, 4
0x140a48af8 mov      rcx, rax
0x140a48afb or       rcx, 1
0x140a48aff cmp      rcx, r8
0x140a48b02 jae      0x140a48b1a
0x140a48b04 movzx    r11d, word ptr [r11 + r10]
0x140a48b09 lea      ecx, [rax*8]
0x140a48b10 shl      r11, cl
0x140a48b13 or       r9, r11
0x140a48b16 or       rax, 2
0x140a48b1a cmp      rax, r8
0x140a48b1d jae      0x140a48b33
0x140a48b1f add      r10, rax
0x140a48b22 movzx    r10d, byte ptr [rsp + r10 + 4]
0x140a48b28 shl      eax, 3
0x140a48b2b mov      ecx, eax
0x140a48b2d shl      r10, cl
0x140a48b30 or       r9, r10
0x140a48b33 mov      qword ptr [rdx + 0x38], r9
0x140a48b37 mov      qword ptr [rdx + 0x40], r8
0x140a48b3b add      rsp, 8
0x140a48b3f pop      rbx
0x140a48b40 pop      rdi
0x140a48b41 pop      rsi
0x140a48b42 ret      
0x140a48b43 add      r8, -4
0x140a48b47 lea      r11, [rsp + 4]
0x140a48b4c xor      r9d, r9d
0x140a48b4f xor      eax, eax
0x140a48b51 mov      rcx, rax
0x140a48b54 or       rcx, 1
0x140a48b58 cmp      rcx, r8
0x140a48b5b jb       0x140a48b04
0x140a48b5d jmp      0x140a48b1a ; 