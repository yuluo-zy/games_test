0x141740a20 55 push rbp
0x141740a21 56 push rsi
0x141740a22 57 push rdi
0x141740a23 53 push rbx
0x141740a24 4881ec78020000 sub rsp, 0x278
0x141740a2b 488dac2480000000 lea rbp, [rsp + 0x80]
0x141740a33 48c785f0010000feffffff mov qword ptr [rbp + 0x1f0], 0xfffffffffffffffe
0x141740a3e 4889ce mov rsi, rcx
0x141740a41 0f57c0 xorps xmm0, xmm0
0x141740a44 0f114530 movups xmmword ptr [rbp + 0x30], xmm0
0x141740a48 0f114548 movups xmmword ptr [rbp + 0x48], xmm0
0x141740a4c 0f114560 movups xmmword ptr [rbp + 0x60], xmm0
0x141740a50 0f114578 movups xmmword ptr [rbp + 0x78], xmm0
0x141740a54 0f118590000000 movups xmmword ptr [rbp + 0x90], xmm0
0x141740a5b c785a000000000000000 mov dword ptr [rbp + 0xa0], 0
0x141740a65 0f1185c8000000 movups xmmword ptr [rbp + 0xc8], xmm0
0x141740a6c 0f1185e0000000 movups xmmword ptr [rbp + 0xe0], xmm0
0x141740a73 0f1185f8000000 movups xmmword ptr [rbp + 0xf8], xmm0
0x141740a7a 0f118510010000 movups xmmword ptr [rbp + 0x110], xmm0
0x141740a81 0f118528010000 movups xmmword ptr [rbp + 0x128], xmm0
0x141740a88 c7853801000000000000 mov dword ptr [rbp + 0x138], 0
0x141740a92 48c7451000000000 mov qword ptr [rbp + 0x10], 0
0x141740a9a 48c7451808000000 mov qword ptr [rbp + 0x18], 8
0x141740aa2 48c7452000000000 mov qword ptr [rbp + 0x20], 0
0x141740aaa 48c7452810000000 mov qword ptr [rbp + 0x28], 0x10
0x141740ab2 48c7454010000000 mov qword ptr [rbp + 0x40], 0x10
0x141740aba 48c7455810000000 mov qword ptr [rbp + 0x58], 0x10
0x141740ac2 48c7457010000000 mov qword ptr [rbp + 0x70], 0x10
0x141740aca 48c7858800000010000000 mov qword ptr [rbp + 0x88], 0x10
0x141740ad5 48b80000000000000080 movabs rax, 0x8000000000000000
0x141740adf 488985a8000000 mov qword ptr [rbp + 0xa8], rax
0x141740ae6 488d0dacf05801 lea rcx, [rip + 0x158f0ac]
0x141740aed 48898db0000000 mov qword ptr [rbp + 0xb0], rcx
0x141740af4 48c785b800000039000000 mov qword ptr [rbp + 0xb8], 0x39
0x141740aff 48c785c000000010000000 mov qword ptr [rbp + 0xc0], 0x10
0x141740b0a 48c785d800000010000000 mov qword ptr [rbp + 0xd8], 0x10
0x141740b15 48c785f000000010000000 mov qword ptr [rbp + 0xf0], 0x10
0x141740b20 48c7850801000010000000 mov qword ptr [rbp + 0x108], 0x10
0x141740b2b 48c7852001000010000000 mov qword ptr [rbp + 0x120], 0x10
0x141740b36 c7854001000000000000 mov dword ptr [rbp + 0x140], 0
0x141740b40 66c785440100000100 mov word ptr [rbp + 0x144], 1
0x141740b49 488985b0010000 mov qword ptr [rbp + 0x1b0], rax
0x141740b50 c785e801000000000000 mov dword ptr [rbp + 0x1e8], 0
0x141740b5a 0fb605a840ef01 movzx eax, byte ptr [rip + 0x1ef40a8]
0x141740b61 b9e0010000 mov ecx, 0x1e0
0x141740b66 ba08000000 mov edx, 8
0x141740b6b e8a030edfe call 0x140613c10
0x141740b70 4885c0 test rax, rax
0x141740b73 7445 je 0x141740bba
0x141740b75 4889c7 mov rdi, rax
0x141740b78 488d5510 lea rdx, [rbp + 0x10]
0x141740b7c 41b8e0010000 mov r8d, 0x1e0
0x141740b82 4889c1 mov rcx, rax
0x141740b85 e876301e01 call 0x142923c00
0x141740b8a 4c8d053fee5801 lea r8, [rip + 0x158ee3f]
0x141740b91 488d5da0 lea rbx, [rbp - 0x60]
0x141740b95 4889d9 mov rcx, rbx
0x141740b98 4889fa mov rdx, rdi
0x141740b9b e8703f5aff call 0x140ce4b10
0x141740ba0 4889f1 mov rcx, rsi
0x141740ba3 4889da mov rdx, rbx
0x141740ba6 e845e6d0ff call 0x14144f1f0
0x141740bab 4889f0 mov rax, rsi
0x141740bae 4881c478020000 add rsp, 0x278
0x141740bb5 5b pop rbx
0x141740bb6 5f pop rdi
0x141740bb7 5e pop rsi
0x141740bb8 5d pop rbp
0x141740bb9 c3 ret 
0x141740bba b908000000 mov ecx, 8
0x141740bbf bae0010000 mov edx, 0x1e0
0x141740bc4 e837841901 call 0x1428d9000
0x141740bc9 0f0b ud2 
0x141740bcb 0f1f440000 nop dword ptr [rax + rax]
0x141740bd0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x141740bd5 55 push rbp
0x141740bd6 56 push rsi
0x141740bd7 57 push rdi
0x141740bd8 53 push rbx
0x141740bd9 4883ec28 sub rsp, 0x28
0x141740bdd 488daa80000000 lea rbp, [rdx + 0x80]
0x141740be4 488d4d10 lea rcx, [rbp + 0x10]
0x141740be8 e8c3dc13ff call 0x14087e8b0
0x141740bed 90 nop 
0x141740bee 4883c428 add rsp, 0x28
0x141740bf2 5b pop rbx
0x141740bf3 5f pop rdi
0x141740bf4 5e pop rsi
0x141740bf5 5d pop rbp
0x141740bf6 c3 ret 
