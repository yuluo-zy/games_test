0x1416ea920 4889c8 mov rax, rcx
0x1416ea923 4983c0fe add r8, -2
0x1416ea927 4983f80d cmp r8, 0xd
0x1416ea92b 0f87f5000000 ja 0x1416eaa26
0x1416ea931 488d0d68545a01 lea rcx, [rip + 0x15a5468]
0x1416ea938 4e630481 movsxd r8, dword ptr [rcx + r8*4]
0x1416ea93c 4901c8 add r8, rcx
0x1416ea93f 41ffe0 jmp r8
0x1416ea942 66813a6964 cmp word ptr [rdx], 0x6469
0x1416ea947 0f85d9000000 jne 0x1416eaa26
0x1416ea94d c6400100 mov byte ptr [rax + 1], 0
0x1416ea951 c60000 mov byte ptr [rax], 0
0x1416ea954 c3 ret 
0x1416ea955 48b96761626c655f7374 movabs rcx, 0x74735f656c626167
0x1416ea95f 48330a xor rcx, qword ptr [rdx]
0x1416ea962 49b86c655f7374796c65 movabs r8, 0x656c7974735f656c
0x1416ea96c 4c334203 xor r8, qword ptr [rdx + 3]
0x1416ea970 4909c8 or r8, rcx
0x1416ea973 0f85ad000000 jne 0x1416eaa26
0x1416ea979 c6400105 mov byte ptr [rax + 1], 5
0x1416ea97d c60000 mov byte ptr [rax], 0
0x1416ea980 c3 ret 
0x1416ea981 b970617261 mov ecx, 0x61726170
0x1416ea986 330a xor ecx, dword ptr [rdx]
0x1416ea988 0fb75204 movzx edx, word ptr [rdx + 4]
0x1416ea98c 81f26d730000 xor edx, 0x736d
0x1416ea992 09ca or edx, ecx
0x1416ea994 0f858c000000 jne 0x1416eaa26
0x1416ea99a c6400101 mov byte ptr [rax + 1], 1
0x1416ea99e c60000 mov byte ptr [rax], 0
0x1416ea9a1 c3 ret 
0x1416ea9a2 49b8666c61745f726f6f movabs r8, 0x6f6f725f74616c66
0x1416ea9ac 4c3302 xor r8, qword ptr [rdx]
0x1416ea9af 49b96c655f7374796c65 movabs r9, 0x656c7974735f656c
0x1416ea9b9 4981c103010000 add r9, 0x103
0x1416ea9c0 4c334a07 xor r9, qword ptr [rdx + 7]
0x1416ea9c4 488d4801 lea rcx, [rax + 1]
0x1416ea9c8 4d09c1 or r9, r8
0x1416ea9cb 7464 je 0x1416eaa31
0x1416ea9cd c60107 mov byte ptr [rcx], 7
0x1416ea9d0 c60000 mov byte ptr [rax], 0
0x1416ea9d3 c3 ret 
0x1416ea9d4 b97069766f mov ecx, 0x6f766970
0x1416ea9d9 330a xor ecx, dword ptr [rdx]
0x1416ea9db 41b86f745f79 mov r8d, 0x795f746f
0x1416ea9e1 44334203 xor r8d, dword ptr [rdx + 3]
0x1416ea9e5 4109c8 or r8d, ecx
0x1416ea9e8 753c jne 0x1416eaa26
0x1416ea9ea c6400103 mov byte ptr [rax + 1], 3
0x1416ea9ee c60000 mov byte ptr [rax], 0
0x1416ea9f1 c3 ret 
0x1416ea9f2 48b9636f6c6f725f6964 movabs rcx, 0x64695f726f6c6f63
0x1416ea9fc 48390a cmp qword ptr [rdx], rcx
0x1416ea9ff 7525 jne 0x1416eaa26
0x1416eaa01 c6400104 mov byte ptr [rax + 1], 4
0x1416eaa05 c60000 mov byte ptr [rax], 0
0x1416eaa08 c3 ret 
0x1416eaa09 48b977616c6c5f736861 movabs rcx, 0x6168735f6c6c6177
0x1416eaa13 48330a xor rcx, qword ptr [rdx]
0x1416eaa16 0fb75208 movzx edx, word ptr [rdx + 8]
0x1416eaa1a 4881f270650000 xor rdx, 0x6570
0x1416eaa21 4809ca or rdx, rcx
0x1416eaa24 7412 je 0x1416eaa38
0x1416eaa26 488d4801 lea rcx, [rax + 1]
0x1416eaa2a c60107 mov byte ptr [rcx], 7
0x1416eaa2d c60000 mov byte ptr [rax], 0
0x1416eaa30 c3 ret 
0x1416eaa31 c60106 mov byte ptr [rcx], 6
0x1416eaa34 c60000 mov byte ptr [rax], 0
0x1416eaa37 c3 ret 
0x1416eaa38 c6400102 mov byte ptr [rax + 1], 2
0x1416eaa3c c60000 mov byte ptr [rax], 0
0x1416eaa3f c3 ret 
