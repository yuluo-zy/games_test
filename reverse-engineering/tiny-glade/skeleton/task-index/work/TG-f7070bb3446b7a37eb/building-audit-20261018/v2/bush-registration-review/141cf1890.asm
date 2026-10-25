0x141cf1890 55 push rbp
0x141cf1891 56 push rsi
0x141cf1892 57 push rdi
0x141cf1893 4881ecf0020000 sub rsp, 0x2f0
0x141cf189a 488dac2480000000 lea rbp, [rsp + 0x80]
0x141cf18a2 48c78568020000feffffff mov qword ptr [rbp + 0x268], 0xfffffffffffffffe
0x141cf18ad 4889ce mov rsi, rcx
0x141cf18b0 0f57c0 xorps xmm0, xmm0
0x141cf18b3 0f1145c8 movups xmmword ptr [rbp - 0x38], xmm0
0x141cf18b7 0f1145e0 movups xmmword ptr [rbp - 0x20], xmm0
0x141cf18bb 0f1145f8 movups xmmword ptr [rbp - 8], xmm0
0x141cf18bf 0f114510 movups xmmword ptr [rbp + 0x10], xmm0
0x141cf18c3 0f114528 movups xmmword ptr [rbp + 0x28], xmm0
0x141cf18c7 c7453800000000 mov dword ptr [rbp + 0x38], 0
0x141cf18ce 0f114560 movups xmmword ptr [rbp + 0x60], xmm0
0x141cf18d2 0f114578 movups xmmword ptr [rbp + 0x78], xmm0
0x141cf18d6 0f118590000000 movups xmmword ptr [rbp + 0x90], xmm0
0x141cf18dd 0f1185a8000000 movups xmmword ptr [rbp + 0xa8], xmm0
0x141cf18e4 0f1185c0000000 movups xmmword ptr [rbp + 0xc0], xmm0
0x141cf18eb c785d000000000000000 mov dword ptr [rbp + 0xd0], 0
0x141cf18f5 48c745a800000000 mov qword ptr [rbp - 0x58], 0
0x141cf18fd 48c745b008000000 mov qword ptr [rbp - 0x50], 8
0x141cf1905 48c745b800000000 mov qword ptr [rbp - 0x48], 0
0x141cf190d 48c745c010000000 mov qword ptr [rbp - 0x40], 0x10
0x141cf1915 48c745d810000000 mov qword ptr [rbp - 0x28], 0x10
0x141cf191d 48c745f010000000 mov qword ptr [rbp - 0x10], 0x10
0x141cf1925 48c7450810000000 mov qword ptr [rbp + 8], 0x10
0x141cf192d 48c7452010000000 mov qword ptr [rbp + 0x20], 0x10
0x141cf1935 48b80000000000000080 movabs rax, 0x8000000000000000
0x141cf193f 48894540 mov qword ptr [rbp + 0x40], rax
0x141cf1943 488d0d33742401 lea rcx, [rip + 0x1247433]
0x141cf194a 48894d48 mov qword ptr [rbp + 0x48], rcx
0x141cf194e 48c7455049000000 mov qword ptr [rbp + 0x50], 0x49
0x141cf1956 48c7455810000000 mov qword ptr [rbp + 0x58], 0x10
0x141cf195e 48c7457010000000 mov qword ptr [rbp + 0x70], 0x10
0x141cf1966 48c7858800000010000000 mov qword ptr [rbp + 0x88], 0x10
0x141cf1971 48c785a000000010000000 mov qword ptr [rbp + 0xa0], 0x10
0x141cf197c 48c785b800000010000000 mov qword ptr [rbp + 0xb8], 0x10
0x141cf1987 c785d800000000000000 mov dword ptr [rbp + 0xd8], 0
0x141cf1991 66c785dc0000000100 mov word ptr [rbp + 0xdc], 1
0x141cf199a 488985e0000000 mov qword ptr [rbp + 0xe0], rax
0x141cf19a1 c7856002000000000000 mov dword ptr [rbp + 0x260], 0
0x141cf19ab 0fb60557329401 movzx eax, byte ptr [rip + 0x1943257]
0x141cf19b2 b9c0020000 mov ecx, 0x2c0
0x141cf19b7 ba08000000 mov edx, 8
0x141cf19bc e84f2292fe call 0x140613c10
0x141cf19c1 4885c0 test rax, rax
0x141cf19c4 7435 je 0x141cf19fb
0x141cf19c6 4889c7 mov rdi, rax
0x141cf19c9 488d55a8 lea rdx, [rbp - 0x58]
0x141cf19cd 41b8c0020000 mov r8d, 0x2c0
0x141cf19d3 4889c1 mov rcx, rax
0x141cf19d6 e82522c300 call 0x142923c00
0x141cf19db 4c8d05c6722401 lea r8, [rip + 0x12472c6]
0x141cf19e2 4889f1 mov rcx, rsi
0x141cf19e5 4889fa mov rdx, rdi
0x141cf19e8 e82331fffe call 0x140ce4b10
0x141cf19ed 4889f0 mov rax, rsi
0x141cf19f0 4881c4f0020000 add rsp, 0x2f0
0x141cf19f7 5f pop rdi
0x141cf19f8 5e pop rsi
0x141cf19f9 5d pop rbp
0x141cf19fa c3 ret 
0x141cf19fb b908000000 mov ecx, 8
0x141cf1a00 bac0020000 mov edx, 0x2c0
0x141cf1a05 e8f675be00 call 0x1428d9000
0x141cf1a0a 0f0b ud2 
0x141cf1a0c 0f1f4000 nop dword ptr [rax]
0x141cf1a10 4889542410 mov qword ptr [rsp + 0x10], rdx
0x141cf1a15 55 push rbp
0x141cf1a16 56 push rsi
0x141cf1a17 57 push rdi
0x141cf1a18 4883ec20 sub rsp, 0x20
0x141cf1a1c 488daa80000000 lea rbp, [rdx + 0x80]
0x141cf1a23 488d4da8 lea rcx, [rbp - 0x58]
0x141cf1a27 e864578efe call 0x1405d7190
0x141cf1a2c 90 nop 
0x141cf1a2d 4883c420 add rsp, 0x20
0x141cf1a31 5f pop rdi
0x141cf1a32 5e pop rsi
0x141cf1a33 5d pop rbp
0x141cf1a34 c3 ret 
