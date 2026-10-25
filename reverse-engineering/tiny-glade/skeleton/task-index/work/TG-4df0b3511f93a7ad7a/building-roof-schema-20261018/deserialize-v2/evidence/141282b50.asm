0x141282b50 56 push rsi
0x141282b51 57 push rdi
0x141282b52 4883ec48 sub rsp, 0x48
0x141282b56 4889c8 mov rax, rcx
0x141282b59 0fb60a movzx ecx, byte ptr [rdx]
0x141282b5c ffc9 dec ecx
0x141282b5e 83f90e cmp ecx, 0xe
0x141282b61 0f875b010000 ja 0x141282cc2
0x141282b67 4c8d0dda4e9a01 lea r9, [rip + 0x19a4eda]
0x141282b6e 49630c89 movsxd rcx, dword ptr [r9 + rcx*4]
0x141282b72 4c01c9 add rcx, r9
0x141282b75 ffe1 jmp rcx
0x141282b77 0fb64a01 movzx ecx, byte ptr [rdx + 1]
0x141282b7b 85c9 test ecx, ecx
0x141282b7d 0f8499000000 je 0x141282c1c
0x141282b83 83f901 cmp ecx, 1
0x141282b86 0f84e0000000 je 0x141282c6c
0x141282b8c eb4a jmp 0x141282bd8
0x141282b8e 488b4a10 mov rcx, qword ptr [rdx + 0x10]
0x141282b92 488b5218 mov rdx, qword ptr [rdx + 0x18]
0x141282b96 4883fa06 cmp rdx, 6
0x141282b9a 0f859f000000 jne 0x141282c3f
0x141282ba0 448b09 mov r9d, dword ptr [rcx]
0x141282ba3 44330d82d0a001 xor r9d, dword ptr [rip + 0x1a0d082]
0x141282baa 440fb75104 movzx r10d, word ptr [rcx + 4]
0x141282baf 6644331579d0a001 xor r10w, word ptr [rip + 0x1a0d079]
0x141282bb7 450fb7d2 movzx r10d, r10w
0x141282bbb 4509ca or r10d, r9d
0x141282bbe 745c je 0x141282c1c
0x141282bc0 e932010000 jmp 0x141282cf7
0x141282bc5 488b4a08 mov rcx, qword ptr [rdx + 8]
0x141282bc9 4885c9 test rcx, rcx
0x141282bcc 744e je 0x141282c1c
0x141282bce 4883f901 cmp rcx, 1
0x141282bd2 0f8494000000 je 0x141282c6c
0x141282bd8 48894c2430 mov qword ptr [rsp + 0x30], rcx
0x141282bdd c644242801 mov byte ptr [rsp + 0x28], 1
0x141282be2 488d15efcfa001 lea rdx, [rip + 0x1a0cfef]
0x141282be9 4c8d0d08c6a001 lea r9, [rip + 0x1a0c608]
0x141282bf0 488d4c2428 lea rcx, [rsp + 0x28]
0x141282bf5 4c89c7 mov rdi, r8
0x141282bf8 4d89c8 mov r8, r9
0x141282bfb 4889c6 mov rsi, rax
0x141282bfe e83d6f6701 call 0x1428f9b40
0x141282c03 4989f8 mov r8, rdi
0x141282c06 4889c1 mov rcx, rax
0x141282c09 4889f0 mov rax, rsi
0x141282c0c 48894c2430 mov qword ptr [rsp + 0x30], rcx
0x141282c11 b101 mov cl, 1
0x141282c13 884c2428 mov byte ptr [rsp + 0x28], cl
0x141282c17 e981000000 jmp 0x141282c9d
0x141282c1c c644242900 mov byte ptr [rsp + 0x29], 0
0x141282c21 eb4e jmp 0x141282c71
0x141282c23 4c8b4a10 mov r9, qword ptr [rdx + 0x10]
0x141282c27 4c8b5218 mov r10, qword ptr [rdx + 0x18]
0x141282c2b eb54 jmp 0x141282c81
0x141282c2d 488b4a08 mov rcx, qword ptr [rdx + 8]
0x141282c31 488b5210 mov rdx, qword ptr [rdx + 0x10]
0x141282c35 4883fa06 cmp rdx, 6
0x141282c39 0f8461ffffff je 0x141282ba0
0x141282c3f 4883fa09 cmp rdx, 9
0x141282c43 0f85ae000000 jne 0x141282cf7
0x141282c49 4c8b09 mov r9, qword ptr [rcx]
0x141282c4c 4c330ddfcfa001 xor r9, qword ptr [rip + 0x1a0cfdf]
0x141282c53 440fb65108 movzx r10d, byte ptr [rcx + 8]
0x141282c58 443215dbcfa001 xor r10b, byte ptr [rip + 0x1a0cfdb]
0x141282c5f 450fb6d2 movzx r10d, r10b
0x141282c63 4d09ca or r10, r9
0x141282c66 0f858b000000 jne 0x141282cf7
0x141282c6c c644242901 mov byte ptr [rsp + 0x29], 1
0x141282c71 31c9 xor ecx, ecx
0x141282c73 884c2428 mov byte ptr [rsp + 0x28], cl
0x141282c77 eb24 jmp 0x141282c9d
0x141282c79 4c8b4a08 mov r9, qword ptr [rdx + 8]
0x141282c7d 4c8b5210 mov r10, qword ptr [rdx + 0x10]
0x141282c81 488d4c2428 lea rcx, [rsp + 0x28]
0x141282c86 4c89ca mov rdx, r9
0x141282c89 4c89c6 mov rsi, r8
0x141282c8c 4d89d0 mov r8, r10
0x141282c8f 4889c7 mov rdi, rax
0x141282c92 e859a24600 call 0x1416ecef0
0x141282c97 4989f0 mov r8, rsi
0x141282c9a 4889f8 mov rax, rdi
0x141282c9d 807c242800 cmp byte ptr [rsp + 0x28], 0
0x141282ca2 4c0f45442430 cmovne r8, qword ptr [rsp + 0x30]
0x141282ca8 0fb64c2429 movzx ecx, byte ptr [rsp + 0x29]
0x141282cad ba02000000 mov edx, 2
0x141282cb2 0f44d1 cmove edx, ecx
0x141282cb5 4c894008 mov qword ptr [rax + 8], r8
0x141282cb9 8810 mov byte ptr [rax], dl
0x141282cbb 4883c448 add rsp, 0x48
0x141282cbf 5f pop rdi
0x141282cc0 5e pop rsi
0x141282cc1 c3 ret 
0x141282cc2 4c8d0d17499a01 lea r9, [rip + 0x19a4917]
0x141282cc9 4c8d542447 lea r10, [rsp + 0x47]
0x141282cce 4889d1 mov rcx, rdx
0x141282cd1 4c89d2 mov rdx, r10
0x141282cd4 4c89c7 mov rdi, r8
0x141282cd7 4d89c8 mov r8, r9
0x141282cda 4889c6 mov rsi, rax
0x141282cdd e80e536701 call 0x1428f7ff0
0x141282ce2 4989f8 mov r8, rdi
0x141282ce5 4889c1 mov rcx, rax
0x141282ce8 4889f0 mov rax, rsi
0x141282ceb 48894c2430 mov qword ptr [rsp + 0x30], rcx
0x141282cf0 c644242801 mov byte ptr [rsp + 0x28], 1
0x141282cf5 eba6 jmp 0x141282c9d
0x141282cf7 4c8d159acfa001 lea r10, [rip + 0x1a0cf9a]
0x141282cfe 41b902000000 mov r9d, 2
0x141282d04 4c89c7 mov rdi, r8
0x141282d07 4d89d0 mov r8, r10
0x141282d0a 4889c6 mov rsi, rax
0x141282d0d e80e716701 call 0x1428f9e20
0x141282d12 e9ecfeffff jmp 0x141282c03
