0x140fe4110 sub rsp, 0x28
0x140fe4114 lea rax, [r8 - 1]
0x140fe4118 cmp rax, rdx
0x140fe411b jae 0x140fe41dd
0x140fe4121 cmp r8, rdx
0x140fe4124 jne 0x140fe412b
0x140fe4126 add rsp, 0x28
0x140fe412a ret 
0x140fe412b lea rax, [rdx + rdx*4]
0x140fe412f lea rax, [rcx + rax*8]
0x140fe4133 shl r8, 3
0x140fe4137 lea rdx, [r8 + r8*4]
0x140fe413b lea r8, [rcx + rdx]
0x140fe413f jmp 0x140fe4176
0x140fe4141 nop word ptr cs:[rax + rax]
0x140fe4150 mov r10, rcx
0x140fe4153 mov qword ptr [r10], r9
0x140fe4156 movaps xmm0, xmmword ptr [rsp]
0x140fe415a movaps xmm1, xmmword ptr [rsp + 0x10]
0x140fe415f movups xmmword ptr [r10 + 8], xmm0
0x140fe4164 movups xmmword ptr [r10 + 0x18], xmm1
0x140fe4169 add r8, 0x28
0x140fe416d add rdx, 0x28
0x140fe4171 cmp r8, rax
0x140fe4174 je 0x140fe4126
0x140fe4176 mov r9, qword ptr [r8]
0x140fe4179 cmp r9, qword ptr [r8 - 0x28]
0x140fe417d jae 0x140fe4169
0x140fe417f movups xmm0, xmmword ptr [r8 + 8]
0x140fe4184 movups xmm1, xmmword ptr [r8 + 0x18]
0x140fe4189 movaps xmmword ptr [rsp + 0x10], xmm1
0x140fe418e movaps xmmword ptr [rsp], xmm0
0x140fe4192 mov r11, rdx
0x140fe4195 nop word ptr cs:[rax + rax]
0x140fe41a0 mov r10, qword ptr [rcx + r11 - 8]
0x140fe41a5 mov qword ptr [rcx + r11 + 0x20], r10
0x140fe41aa movups xmm0, xmmword ptr [rcx + r11 - 0x28]
0x140fe41b0 movups xmm1, xmmword ptr [rcx + r11 - 0x18]
0x140fe41b6 movups xmmword ptr [rcx + r11 + 0x10], xmm1
0x140fe41bc movups xmmword ptr [rcx + r11], xmm0
0x140fe41c1 cmp r11, 0x28
0x140fe41c5 je 0x140fe4150
0x140fe41c7 lea r10, [r11 - 0x28]
0x140fe41cb cmp r9, qword ptr [rcx + r11 - 0x50]
0x140fe41d0 mov r11, r10
0x140fe41d3 jb 0x140fe41a0
0x140fe41d5 add r10, rcx
0x140fe41d8 jmp 0x140fe4153
0x140fe41dd ud2 
