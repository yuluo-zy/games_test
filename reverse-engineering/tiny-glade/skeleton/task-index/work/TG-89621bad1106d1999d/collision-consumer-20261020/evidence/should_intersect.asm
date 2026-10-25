0x140a9b3c0 4883ec28 sub rsp, 0x28
0x140a9b3c4 4c8b09 mov r9, qword ptr [rcx]
0x140a9b3c7 498b4810 mov rcx, qword ptr [r8 + 0x10]
0x140a9b3cb 4589ca mov r10d, r9d
0x140a9b3ce 4c39d1 cmp rcx, r10
0x140a9b3d1 0f86d4000000 jbe 0x140a9b4ab
0x140a9b3d7 498b4008 mov rax, qword ptr [r8 + 8]
0x140a9b3db 4885c0 test rax, rax
0x140a9b3de 0f84c7000000 je 0x140a9b4ab
0x140a9b3e4 49c1e920 shr r9, 0x20
0x140a9b3e8 4183c901 or r9d, 1
0x140a9b3ec 4f8d0492 lea r8, [r10 + r10*4]
0x140a9b3f0 46394cc020 cmp dword ptr [rax + r8*8 + 0x20], r9d
0x140a9b3f5 0f85b0000000 jne 0x140a9b4ab
0x140a9b3fb 488b12 mov rdx, qword ptr [rdx]
0x140a9b3fe 4189d1 mov r9d, edx
0x140a9b401 4c39c9 cmp rcx, r9
0x140a9b404 0f86b9000000 jbe 0x140a9b4c3
0x140a9b40a 48c1ea20 shr rdx, 0x20
0x140a9b40e 83ca01 or edx, 1
0x140a9b411 4b8d0c89 lea rcx, [r9 + r9*4]
0x140a9b415 3954c820 cmp dword ptr [rax + rcx*8 + 0x20], edx
0x140a9b419 0f85a4000000 jne 0x140a9b4c3
0x140a9b41f 4e8d04c0 lea r8, [rax + r8*8]
0x140a9b423 450fb610 movzx r10d, byte ptr [r8]
0x140a9b427 498b5008 mov rdx, qword ptr [r8 + 8]
0x140a9b42b 488d04c8 lea rax, [rax + rcx*8]
0x140a9b42f 440fb608 movzx r9d, byte ptr [rax]
0x140a9b433 0fb64801 movzx ecx, byte ptr [rax + 1]
0x140a9b437 488b4008 mov rax, qword ptr [rax + 8]
0x140a9b43b 4585d2 test r10d, r10d
0x140a9b43e 7435 je 0x140a9b475
0x140a9b440 4183fa02 cmp r10d, 2
0x140a9b444 7517 jne 0x140a9b45d
0x140a9b446 450fb64001 movzx r8d, byte ptr [r8 + 1]
0x140a9b44b 450fb6c9 movzx r9d, r9b
0x140a9b44f 4183f902 cmp r9d, 2
0x140a9b453 7439 je 0x140a9b48e
0x140a9b455 4489c1 mov ecx, r8d
0x140a9b458 4585c9 test r9d, r9d
0x140a9b45b 741e je 0x140a9b47b
0x140a9b45d 488d0db4ff0302 lea rcx, [rip + 0x203ffb4]
0x140a9b464 4c8d05d5ff0302 lea r8, [rip + 0x203ffd5]
0x140a9b46b ba28000000 mov edx, 0x28
0x140a9b470 e8bbdfe301 call 0x1428d9430
0x140a9b475 4180f902 cmp r9b, 2
0x140a9b479 75e2 jne 0x140a9b45d
0x140a9b47b 4839c2 cmp rdx, rax
0x140a9b47e 7407 je 0x140a9b487
0x140a9b480 b001 mov al, 1
0x140a9b482 f6c101 test cl, 1
0x140a9b485 7402 je 0x140a9b489
0x140a9b487 31c0 xor eax, eax
0x140a9b489 4883c428 add rsp, 0x28
0x140a9b48d c3 ret
0x140a9b48e 4589c1 mov r9d, r8d
0x140a9b491 4180f101 xor r9b, 1
0x140a9b495 4839c2 cmp rdx, rax
0x140a9b498 0f95c2 setne dl
0x140a9b49b 4138c8 cmp r8b, cl
0x140a9b49e 0f94c0 sete al
0x140a9b4a1 4420c8 and al, r9b
0x140a9b4a4 20d0 and al, dl
0x140a9b4a6 4883c428 add rsp, 0x28
0x140a9b4aa c3 ret
0x140a9b4ab 488d0d2e241d02 lea rcx, [rip + 0x21d242e]
0x140a9b4b2 4c8d052fff0302 lea r8, [rip + 0x203ff2f]
0x140a9b4b9 ba18000000 mov edx, 0x18
0x140a9b4be e862d2e301 call 0x1428d8725
0x140a9b4c3 488d0d16241d02 lea rcx, [rip + 0x21d2416]
0x140a9b4ca 4c8d052fff0302 lea r8, [rip + 0x203ff2f]
0x140a9b4d1 ba18000000 mov edx, 0x18
0x140a9b4d6 e84ad2e301 call 0x1428d8725
0x140a9b4db cc int3
