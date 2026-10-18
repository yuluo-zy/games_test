0x141cefa80 55 push rbp
0x141cefa81 56 push rsi
0x141cefa82 57 push rdi
0x141cefa83 4881ecb0010000 sub rsp, 0x1b0
0x141cefa8a 488dac2480000000 lea rbp, [rsp + 0x80]
0x141cefa92 48c78528010000feffffff mov qword ptr [rbp + 0x128], 0xfffffffffffffffe
0x141cefa9d 4889ce mov rsi, rcx
0x141cefaa0 0f57c0 xorps xmm0, xmm0
0x141cefaa3 0f114508 movups xmmword ptr [rbp + 8], xmm0
0x141cefaa7 0f114520 movups xmmword ptr [rbp + 0x20], xmm0
0x141cefaab 0f114538 movups xmmword ptr [rbp + 0x38], xmm0
0x141cefaaf 0f114550 movups xmmword ptr [rbp + 0x50], xmm0
0x141cefab3 0f114568 movups xmmword ptr [rbp + 0x68], xmm0
0x141cefab7 c7457800000000 mov dword ptr [rbp + 0x78], 0
0x141cefabe 0f1185a0000000 movups xmmword ptr [rbp + 0xa0], xmm0
0x141cefac5 0f1185b8000000 movups xmmword ptr [rbp + 0xb8], xmm0
0x141cefacc 0f1185d0000000 movups xmmword ptr [rbp + 0xd0], xmm0
0x141cefad3 0f1185e8000000 movups xmmword ptr [rbp + 0xe8], xmm0
0x141cefada 0f118500010000 movups xmmword ptr [rbp + 0x100], xmm0
0x141cefae1 c7851001000000000000 mov dword ptr [rbp + 0x110], 0
0x141cefaeb 48c745a800000000 mov qword ptr [rbp - 0x58], 0
0x141cefaf3 48c745e800000000 mov qword ptr [rbp - 0x18], 0
0x141cefafb 48c745f008000000 mov qword ptr [rbp - 0x10], 8
0x141cefb03 48c745f800000000 mov qword ptr [rbp - 8], 0
0x141cefb0b 48c7450010000000 mov qword ptr [rbp], 0x10
0x141cefb13 48c7451810000000 mov qword ptr [rbp + 0x18], 0x10
0x141cefb1b 48c7453010000000 mov qword ptr [rbp + 0x30], 0x10
0x141cefb23 48c7454810000000 mov qword ptr [rbp + 0x48], 0x10
0x141cefb2b 48c7456010000000 mov qword ptr [rbp + 0x60], 0x10
0x141cefb33 48b80000000000000080 movabs rax, 0x8000000000000000
0x141cefb3d 48898580000000 mov qword ptr [rbp + 0x80], rax
0x141cefb44 488d0515852401 lea rax, [rip + 0x1248515]
0x141cefb4b 48898588000000 mov qword ptr [rbp + 0x88], rax
0x141cefb52 48c785900000005b000000 mov qword ptr [rbp + 0x90], 0x5b
0x141cefb5d 48c7859800000010000000 mov qword ptr [rbp + 0x98], 0x10
0x141cefb68 48c785b000000010000000 mov qword ptr [rbp + 0xb0], 0x10
0x141cefb73 48c785c800000010000000 mov qword ptr [rbp + 0xc8], 0x10
0x141cefb7e 48c785e000000010000000 mov qword ptr [rbp + 0xe0], 0x10
0x141cefb89 48c785f800000010000000 mov qword ptr [rbp + 0xf8], 0x10
0x141cefb94 c7851801000000000000 mov dword ptr [rbp + 0x118], 0
0x141cefb9e 66c7851c0100000100 mov word ptr [rbp + 0x11c], 1
0x141cefba7 c7852001000000000000 mov dword ptr [rbp + 0x120], 0
0x141cefbb1 0fb60551509401 movzx eax, byte ptr [rip + 0x1945051]
0x141cefbb8 b980010000 mov ecx, 0x180
0x141cefbbd ba08000000 mov edx, 8
0x141cefbc2 e8494092fe call 0x140613c10
0x141cefbc7 4885c0 test rax, rax
0x141cefbca 7435 je 0x141cefc01
0x141cefbcc 4889c7 mov rdi, rax
0x141cefbcf 488d55a8 lea rdx, [rbp - 0x58]
0x141cefbd3 41b880010000 mov r8d, 0x180
0x141cefbd9 4889c1 mov rcx, rax
0x141cefbdc e81f40c300 call 0x142923c00
0x141cefbe1 4c8d05c0832401 lea r8, [rip + 0x12483c0]
0x141cefbe8 4889f1 mov rcx, rsi
0x141cefbeb 4889fa mov rdx, rdi
0x141cefbee e81d4ffffe call 0x140ce4b10
0x141cefbf3 4889f0 mov rax, rsi
0x141cefbf6 4881c4b0010000 add rsp, 0x1b0
0x141cefbfd 5f pop rdi
0x141cefbfe 5e pop rsi
0x141cefbff 5d pop rbp
0x141cefc00 c3 ret 
0x141cefc01 b908000000 mov ecx, 8
0x141cefc06 ba80010000 mov edx, 0x180
0x141cefc0b e8f093be00 call 0x1428d9000
0x141cefc10 0f0b ud2 
0x141cefc12 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x141cefc20 4889542410 mov qword ptr [rsp + 0x10], rdx
0x141cefc25 55 push rbp
0x141cefc26 56 push rsi
0x141cefc27 57 push rdi
0x141cefc28 4883ec20 sub rsp, 0x20
0x141cefc2c 488daa80000000 lea rbp, [rdx + 0x80]
0x141cefc33 488d4da8 lea rcx, [rbp - 0x58]
0x141cefc37 e8344290fe call 0x1405f3e70
0x141cefc3c 90 nop 
0x141cefc3d 4883c420 add rsp, 0x20
0x141cefc41 5f pop rdi
0x141cefc42 5e pop rsi
0x141cefc43 5d pop rbp
0x141cefc44 c3 ret 
