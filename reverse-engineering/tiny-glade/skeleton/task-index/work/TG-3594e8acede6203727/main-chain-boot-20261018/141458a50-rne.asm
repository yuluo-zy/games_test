0x141458a50 55 push rbp
0x141458a51 56 push rsi
0x141458a52 4883ec68 sub rsp, 0x68
0x141458a56 488d6c2460 lea rbp, [rsp + 0x60]
0x141458a5b 48c74500feffffff mov qword ptr [rbp], 0xfffffffffffffffe
0x141458a63 4889d6 mov rsi, rdx
0x141458a66 488b11 mov rdx, qword ptr [rcx]
0x141458a69 488d4dc0 lea rcx, [rbp - 0x40]
0x141458a6d e81ebbffff call 0x141454590
0x141458a72 48837dc000 cmp qword ptr [rbp - 0x40], 0
0x141458a77 746e je 0x141458ae7
0x141458a79 0f1045c0 movups xmm0, xmmword ptr [rbp - 0x40]
0x141458a7d 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
0x141458a81 488b45d0 mov rax, qword ptr [rbp - 0x30]
0x141458a85 488945f0 mov qword ptr [rbp - 0x10], rax
0x141458a89 488b4de8 mov rcx, qword ptr [rbp - 0x18]
0x141458a8d 4889f2 mov rdx, rsi
0x141458a90 e81b6a68ff call 0x140adf4b0
0x141458a95 488b75e0 mov rsi, qword ptr [rbp - 0x20]
0x141458a99 4889f1 mov rcx, rsi
0x141458a9c e86f9382ff call 0x140c81e10
0x141458aa1 4889f1 mov rcx, rsi
0x141458aa4 4883c901 or rcx, 1
0x141458aa8 e8639382ff call 0x140c81e10
0x141458aad 48c7c0f0ffffff mov rax, 0xfffffffffffffff0
0x141458ab4 f0480fc106 lock xadd qword ptr [rsi], rax
0x141458ab9 4883e0f2 and rax, 0xfffffffffffffff2
0x141458abd 4883f812 cmp rax, 0x12
0x141458ac1 742c je 0x141458aef
0x141458ac3 488b45f0 mov rax, qword ptr [rbp - 0x10]
0x141458ac7 48c745f000000000 mov qword ptr [rbp - 0x10], 0
0x141458acf 488d48f0 lea rcx, [rax - 0x10]
0x141458ad3 48894dd8 mov qword ptr [rbp - 0x28], rcx
0x141458ad7 f048ff48f0 lock dec qword ptr [rax - 0x10]
0x141458adc 7509 jne 0x141458ae7
0x141458ade 488d4dd8 lea rcx, [rbp - 0x28]
0x141458ae2 e8494070ff call 0x140b5cb30
0x141458ae7 90 nop 
0x141458ae8 4883c468 add rsp, 0x68
0x141458aec 5e pop rsi
0x141458aed 5d pop rbp
0x141458aee c3 ret 
0x141458aef 4889f1 mov rcx, rsi
0x141458af2 e859444701 call 0x1428ccf50
0x141458af7 ebca jmp 0x141458ac3
0x141458af9 0f1f8000000000 nop dword ptr [rax]
0x141458b00 4889542410 mov qword ptr [rsp + 0x10], rdx
0x141458b05 55 push rbp
0x141458b06 56 push rsi
0x141458b07 4883ec28 sub rsp, 0x28
0x141458b0b 488d6a60 lea rbp, [rdx + 0x60]
0x141458b0f 488d4de0 lea rcx, [rbp - 0x20]
0x141458b13 e8580f44ff call 0x140899a70
0x141458b18 90 nop 
0x141458b19 4883c428 add rsp, 0x28
0x141458b1d 5e pop rsi
0x141458b1e 5d pop rbp
0x141458b1f c3 ret 
