0x1412d8b00 56 push rsi
0x1412d8b01 4883ec30 sub rsp, 0x30
0x1412d8b05 49830010 add qword ptr [r8], 0x10
0x1412d8b09 4885d2 test rdx, rdx
0x1412d8b0c 7432 je 0x1412d8b40
0x1412d8b0e 4889d6 mov rsi, rdx
0x1412d8b11 0f1001 movups xmm0, xmmword ptr [rcx]
0x1412d8b14 0f29442420 movaps xmmword ptr [rsp + 0x20], xmm0
0x1412d8b19 488d4c2420 lea rcx, [rsp + 0x20]
0x1412d8b1e e83d04d800 call 0x142058f60
0x1412d8b23 4889f1 mov rcx, rsi
0x1412d8b26 e885f6a1ff call 0x140cf81b0
0x1412d8b2b 4889f1 mov rcx, rsi
0x1412d8b2e e81df7a1ff call 0x140cf8250
0x1412d8b33 4889f1 mov rcx, rsi
0x1412d8b36 4883c430 add rsp, 0x30
0x1412d8b3a 5e pop rsi
0x1412d8b3b e9c0f6a1ff jmp 0x140cf8200
0x1412d8b40 90 nop 
0x1412d8b41 4883c430 add rsp, 0x30
0x1412d8b45 5e pop rsi
0x1412d8b46 c3 ret 
