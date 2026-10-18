0x140ee1d50 4156 push r14
0x140ee1d52 56 push rsi
0x140ee1d53 57 push rdi
0x140ee1d54 53 push rbx
0x140ee1d55 4883ec28 sub rsp, 0x28
0x140ee1d59 4c89c6 mov rsi, r8
0x140ee1d5c 4889d7 mov rdi, rdx
0x140ee1d5f 488b01 mov rax, qword ptr [rcx]
0x140ee1d62 4889c2 mov rdx, rax
0x140ee1d65 48f7da neg rdx
0x140ee1d68 7130 jno 0x140ee1d9a
0x140ee1d6a 488b4118 mov rax, qword ptr [rcx + 0x18]
0x140ee1d6e 4885c0 test rax, rax
0x140ee1d71 745d je 0x140ee1dd0
0x140ee1d73 488b4910 mov rcx, qword ptr [rcx + 0x10]
0x140ee1d77 486bd870 imul rbx, rax, 0x70
0x140ee1d7b 0f1f440000 nop dword ptr [rax + rax]
0x140ee1d80 4c8d7170 lea r14, [rcx + 0x70]
0x140ee1d84 4889fa mov rdx, rdi
0x140ee1d87 4989f0 mov r8, rsi
0x140ee1d8a e8c1ffffff call 0x140ee1d50
0x140ee1d8f 4c89f1 mov rcx, r14
0x140ee1d92 4883c390 add rbx, -0x70
0x140ee1d96 75e8 jne 0x140ee1d80
0x140ee1d98 eb36 jmp 0x140ee1dd0
0x140ee1d9a 4c8b7110 mov r14, qword ptr [rcx + 0x10]
0x140ee1d9e 4939c6 cmp r14, rax
0x140ee1da1 7512 jne 0x140ee1db5
0x140ee1da3 488d15a657cb01 lea rdx, [rip + 0x1cb57a6]
0x140ee1daa 4889cb mov rbx, rcx
0x140ee1dad e89ec77cff call 0x1406ae550
0x140ee1db2 4889d9 mov rcx, rbx
0x140ee1db5 488b4108 mov rax, qword ptr [rcx + 8]
0x140ee1db9 4c89f2 mov rdx, r14
0x140ee1dbc 48c1e204 shl rdx, 4
0x140ee1dc0 48893c10 mov qword ptr [rax + rdx], rdi
0x140ee1dc4 4889741008 mov qword ptr [rax + rdx + 8], rsi
0x140ee1dc9 49ffc6 inc r14
0x140ee1dcc 4c897110 mov qword ptr [rcx + 0x10], r14
0x140ee1dd0 4883c428 add rsp, 0x28
0x140ee1dd4 5b pop rbx
0x140ee1dd5 5f pop rdi
0x140ee1dd6 5e pop rsi
0x140ee1dd7 415e pop r14
0x140ee1dd9 c3 ret 
