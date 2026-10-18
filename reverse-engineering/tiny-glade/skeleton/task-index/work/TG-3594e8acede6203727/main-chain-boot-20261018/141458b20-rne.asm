0x141458b20 55 push rbp
0x141458b21 56 push rsi
0x141458b22 4883ec68 sub rsp, 0x68
0x141458b26 488d6c2460 lea rbp, [rsp + 0x60]
0x141458b2b 48c74500feffffff mov qword ptr [rbp], 0xfffffffffffffffe
0x141458b33 4889d6 mov rsi, rdx
0x141458b36 488b11 mov rdx, qword ptr [rcx]
0x141458b39 488d4dc0 lea rcx, [rbp - 0x40]
0x141458b3d e84ebaffff call 0x141454590
0x141458b42 48837dc000 cmp qword ptr [rbp - 0x40], 0
0x141458b47 746e je 0x141458bb7
0x141458b49 0f1045c0 movups xmm0, xmmword ptr [rbp - 0x40]
0x141458b4d 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
0x141458b51 488b45d0 mov rax, qword ptr [rbp - 0x30]
0x141458b55 488945f0 mov qword ptr [rbp - 0x10], rax
0x141458b59 488b4de8 mov rcx, qword ptr [rbp - 0x18]
0x141458b5d 4889f2 mov rdx, rsi
0x141458b60 e83b6368ff call 0x140adeea0
0x141458b65 488b75e0 mov rsi, qword ptr [rbp - 0x20]
0x141458b69 4889f1 mov rcx, rsi
0x141458b6c e89f9282ff call 0x140c81e10
0x141458b71 4889f1 mov rcx, rsi
0x141458b74 4883c901 or rcx, 1
0x141458b78 e8939282ff call 0x140c81e10
0x141458b7d 48c7c0f0ffffff mov rax, 0xfffffffffffffff0
0x141458b84 f0480fc106 lock xadd qword ptr [rsi], rax
0x141458b89 4883e0f2 and rax, 0xfffffffffffffff2
0x141458b8d 4883f812 cmp rax, 0x12
0x141458b91 742c je 0x141458bbf
0x141458b93 488b45f0 mov rax, qword ptr [rbp - 0x10]
0x141458b97 48c745f000000000 mov qword ptr [rbp - 0x10], 0
0x141458b9f 488d48f0 lea rcx, [rax - 0x10]
0x141458ba3 48894dd8 mov qword ptr [rbp - 0x28], rcx
0x141458ba7 f048ff48f0 lock dec qword ptr [rax - 0x10]
0x141458bac 7509 jne 0x141458bb7
0x141458bae 488d4dd8 lea rcx, [rbp - 0x28]
0x141458bb2 e8793f70ff call 0x140b5cb30
0x141458bb7 90 nop 
0x141458bb8 4883c468 add rsp, 0x68
0x141458bbc 5e pop rsi
0x141458bbd 5d pop rbp
0x141458bbe c3 ret 
0x141458bbf 4889f1 mov rcx, rsi
0x141458bc2 e889434701 call 0x1428ccf50
0x141458bc7 ebca jmp 0x141458b93
0x141458bc9 0f1f8000000000 nop dword ptr [rax]
0x141458bd0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x141458bd5 55 push rbp
0x141458bd6 56 push rsi
0x141458bd7 4883ec28 sub rsp, 0x28
0x141458bdb 488d6a60 lea rbp, [rdx + 0x60]
0x141458bdf 488d4de0 lea rcx, [rbp - 0x20]
0x141458be3 e8880e44ff call 0x140899a70
0x141458be8 90 nop 
0x141458be9 4883c428 add rsp, 0x28
0x141458bed 5e pop rsi
0x141458bee 5d pop rbp
0x141458bef c3 ret 
