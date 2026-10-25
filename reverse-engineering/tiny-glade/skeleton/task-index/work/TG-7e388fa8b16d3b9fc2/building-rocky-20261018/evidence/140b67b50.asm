0x140b67b50 56 push rsi
0x140b67b51 57 push rdi
0x140b67b52 53 push rbx
0x140b67b53 4881ecc0000000 sub rsp, 0xc0
0x140b67b5a 4889ce mov rsi, rcx
0x140b67b5d 4883c138 add rcx, 0x38
0x140b67b61 48ba84a72b9651d21b8e movabs rdx, 0x8e1bd251962ba784
0x140b67b6b 49b89b52e6c07423786a movabs r8, 0x6a782374c0e6529b
0x140b67b75 e8765b57ff call 0x1400dd6f0
0x140b67b7a 4885c0 test rax, rax
0x140b67b7d 0f849b000000 je 0x140b67c1e
0x140b67b83 48399618020000 cmp qword ptr [rsi + 0x218], rdx
0x140b67b8a 0f868e000000 jbe 0x140b67c1e
0x140b67b90 488b8610020000 mov rax, qword ptr [rsi + 0x210]
0x140b67b97 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x140b67b9b 4885c9 test rcx, rcx
0x140b67b9e 747e je 0x140b67c1e
0x140b67ba0 488b86e0010000 mov rax, qword ptr [rsi + 0x1e0]
0x140b67ba7 48f7d1 not rcx
0x140b67baa 488d0c49 lea rcx, [rcx + rcx*2]
0x140b67bae 48c1e105 shl rcx, 5
0x140b67bb2 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x140b67bb8 7464 je 0x140b67c1e
0x140b67bba 4801c8 add rax, rcx
0x140b67bbd 488b4810 mov rcx, qword ptr [rax + 0x10]
0x140b67bc1 8b96b0050000 mov edx, dword ptr [rsi + 0x5b0]
0x140b67bc7 89505c mov dword ptr [rax + 0x5c], edx
0x140b67bca e851a0b300 call 0x1416a1c20
0x140b67bcf 4889c7 mov rdi, rax
0x140b67bd2 488d5c2450 lea rbx, [rsp + 0x50]
0x140b67bd7 4889d9 mov rcx, rbx
0x140b67bda e8d13ce3ff call 0x14099b8b0
0x140b67bdf 488d4c2430 lea rcx, [rsp + 0x30]
0x140b67be4 4889fa mov rdx, rdi
0x140b67be7 4989d8 mov r8, rbx
0x140b67bea 4531c9 xor r9d, r9d
0x140b67bed e83e71b300 call 0x14169ed30
0x140b67bf2 488b542430 mov rdx, qword ptr [rsp + 0x30]
0x140b67bf7 4885d2 test rdx, rdx
0x140b67bfa 7414 je 0x140b67c10
0x140b67bfc 488b4c2438 mov rcx, qword ptr [rsp + 0x38]
0x140b67c01 48c1e204 shl rdx, 4
0x140b67c05 41b808000000 mov r8d, 8
0x140b67c0b e810c0aaff call 0x140613c20
0x140b67c10 4889f0 mov rax, rsi
0x140b67c13 4881c4c0000000 add rsp, 0xc0
0x140b67c1a 5b pop rbx
0x140b67c1b 5f pop rdi
0x140b67c1c 5e pop rsi
0x140b67c1d c3 ret 
0x140b67c1e 488d059bb0f801 lea rax, [rip + 0x1f8b09b]
0x140b67c25 4889442430 mov qword ptr [rsp + 0x30], rax
0x140b67c2a 48c744243827000000 mov qword ptr [rsp + 0x38], 0x27
0x140b67c33 488d442430 lea rax, [rsp + 0x30]
0x140b67c38 4889442420 mov qword ptr [rsp + 0x20], rax
0x140b67c3d 488d054c225aff lea rax, [rip - 0xa5ddb4]
0x140b67c44 4889442428 mov qword ptr [rsp + 0x28], rax
0x140b67c49 488d05a0b1f801 lea rax, [rip + 0x1f8b1a0]
0x140b67c50 4889442450 mov qword ptr [rsp + 0x50], rax
0x140b67c55 48c744245802000000 mov qword ptr [rsp + 0x58], 2
0x140b67c5e 48c744247000000000 mov qword ptr [rsp + 0x70], 0
0x140b67c67 488d442420 lea rax, [rsp + 0x20]
0x140b67c6c 4889442460 mov qword ptr [rsp + 0x60], rax
0x140b67c71 48c744246801000000 mov qword ptr [rsp + 0x68], 1
0x140b67c7a 488d1527b0f801 lea rdx, [rip + 0x1f8b027]
0x140b67c81 488d4c2450 lea rcx, [rsp + 0x50]
0x140b67c86 e80517d701 call 0x1428d9390
0x140b67c8b cc int3 
