0x1404e4a70 55 push rbp
0x1404e4a71 56 push rsi
0x1404e4a72 57 push rdi
0x1404e4a73 4881ecf0010000 sub rsp, 0x1f0
0x1404e4a7a 488dac2480000000 lea rbp, [rsp + 0x80]
0x1404e4a82 48c78568010000feffffff mov qword ptr [rbp + 0x168], 0xfffffffffffffffe
0x1404e4a8d 4889ce mov rsi, rcx
0x1404e4a90 0f57c0 xorps xmm0, xmm0
0x1404e4a93 0f114548 movups xmmword ptr [rbp + 0x48], xmm0
0x1404e4a97 0f114560 movups xmmword ptr [rbp + 0x60], xmm0
0x1404e4a9b 0f114578 movups xmmword ptr [rbp + 0x78], xmm0
0x1404e4a9f 0f118590000000 movups xmmword ptr [rbp + 0x90], xmm0
0x1404e4aa6 0f1185a8000000 movups xmmword ptr [rbp + 0xa8], xmm0
0x1404e4aad c785b800000000000000 mov dword ptr [rbp + 0xb8], 0
0x1404e4ab7 0f1185e0000000 movups xmmword ptr [rbp + 0xe0], xmm0
0x1404e4abe 0f1185f8000000 movups xmmword ptr [rbp + 0xf8], xmm0
0x1404e4ac5 0f118510010000 movups xmmword ptr [rbp + 0x110], xmm0
0x1404e4acc 0f118528010000 movups xmmword ptr [rbp + 0x128], xmm0
0x1404e4ad3 0f118540010000 movups xmmword ptr [rbp + 0x140], xmm0
0x1404e4ada c7855001000000000000 mov dword ptr [rbp + 0x150], 0
0x1404e4ae4 48c745a800000000 mov qword ptr [rbp - 0x58], 0
0x1404e4aec 48c7452800000000 mov qword ptr [rbp + 0x28], 0
0x1404e4af4 48c7453008000000 mov qword ptr [rbp + 0x30], 8
0x1404e4afc 48c7453800000000 mov qword ptr [rbp + 0x38], 0
0x1404e4b04 48c7454010000000 mov qword ptr [rbp + 0x40], 0x10
0x1404e4b0c 48c7455810000000 mov qword ptr [rbp + 0x58], 0x10
0x1404e4b14 48c7457010000000 mov qword ptr [rbp + 0x70], 0x10
0x1404e4b1c 48c7858800000010000000 mov qword ptr [rbp + 0x88], 0x10
0x1404e4b27 48c785a000000010000000 mov qword ptr [rbp + 0xa0], 0x10
0x1404e4b32 48b80000000000000080 movabs rax, 0x8000000000000000
0x1404e4b3c 488985c0000000 mov qword ptr [rbp + 0xc0], rax
0x1404e4b43 488d05d3615002 lea rax, [rip + 0x25061d3]
0x1404e4b4a 488985c8000000 mov qword ptr [rbp + 0xc8], rax
0x1404e4b51 48c785d000000035000000 mov qword ptr [rbp + 0xd0], 0x35
0x1404e4b5c 48c785d800000010000000 mov qword ptr [rbp + 0xd8], 0x10
0x1404e4b67 48c785f000000010000000 mov qword ptr [rbp + 0xf0], 0x10
0x1404e4b72 48c7850801000010000000 mov qword ptr [rbp + 0x108], 0x10
0x1404e4b7d 48c7852001000010000000 mov qword ptr [rbp + 0x120], 0x10
0x1404e4b88 48c7853801000010000000 mov qword ptr [rbp + 0x138], 0x10
0x1404e4b93 c7855801000000000000 mov dword ptr [rbp + 0x158], 0
0x1404e4b9d 66c7855c0100000100 mov word ptr [rbp + 0x15c], 1
0x1404e4ba6 c7856001000000000000 mov dword ptr [rbp + 0x160], 0
0x1404e4bb0 0fb60552001503 movzx eax, byte ptr [rip + 0x3150052]
0x1404e4bb7 b9c0010000 mov ecx, 0x1c0
0x1404e4bbc ba08000000 mov edx, 8
0x1404e4bc1 e84af01200 call 0x140613c10
0x1404e4bc6 4885c0 test rax, rax
0x1404e4bc9 7435 je 0x1404e4c00
0x1404e4bcb 4889c7 mov rdi, rax
0x1404e4bce 488d55a8 lea rdx, [rbp - 0x58]
0x1404e4bd2 41b8c0010000 mov r8d, 0x1c0
0x1404e4bd8 4889c1 mov rcx, rax
0x1404e4bdb e820f04302 call 0x142923c00
0x1404e4be0 4c8d05e15e5002 lea r8, [rip + 0x2505ee1]
0x1404e4be7 4889f1 mov rcx, rsi
0x1404e4bea 4889fa mov rdx, rdi
0x1404e4bed e81eff7f00 call 0x140ce4b10
0x1404e4bf2 4889f0 mov rax, rsi
0x1404e4bf5 4881c4f0010000 add rsp, 0x1f0
0x1404e4bfc 5f pop rdi
0x1404e4bfd 5e pop rsi
0x1404e4bfe 5d pop rbp
0x1404e4bff c3 ret 
0x1404e4c00 b908000000 mov ecx, 8
0x1404e4c05 bac0010000 mov edx, 0x1c0
0x1404e4c0a e8f1433f02 call 0x1428d9000
0x1404e4c0f 0f0b ud2 
0x1404e4c11 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1404e4c20 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404e4c25 55 push rbp
0x1404e4c26 56 push rsi
0x1404e4c27 57 push rdi
0x1404e4c28 4883ec20 sub rsp, 0x20
0x1404e4c2c 488daa80000000 lea rbp, [rdx + 0x80]
0x1404e4c33 488d4da8 lea rcx, [rbp - 0x58]
0x1404e4c37 e8948d0f00 call 0x1405dd9d0
0x1404e4c3c 90 nop 
0x1404e4c3d 4883c420 add rsp, 0x20
0x1404e4c41 5f pop rdi
0x1404e4c42 5e pop rsi
0x1404e4c43 5d pop rbp
0x1404e4c44 c3 ret 
