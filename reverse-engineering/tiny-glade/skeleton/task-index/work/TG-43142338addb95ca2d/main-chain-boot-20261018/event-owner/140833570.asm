140833570 488b01 mov rax, qword ptr [rcx]
140833573 488d50fd lea rdx, [rax - 3]
140833577 4883c0fe add rax, -2
14083357b 4531c0 xor r8d, r8d
14083357e 4883fa02 cmp rdx, 2
140833582 4c0f42c0 cmovb r8, rax
140833586 488d05ebf82502 lea rax, [rip + 0x225f8eb]
14083358d 4a8b04c0 mov rax, qword ptr [rax + r8*8]
140833591 8b0401 mov eax, dword ptr [rcx + rax]
140833594 c3 ret 
