0x1402cf8a0 55 push rbp
0x1402cf8a1 56 push rsi
0x1402cf8a2 57 push rdi
0x1402cf8a3 4881eca0010000 sub rsp, 0x1a0
0x1402cf8aa 488dac2480000000 lea rbp, [rsp + 0x80]
0x1402cf8b2 48c78518010000feffffff mov qword ptr [rbp + 0x118], 0xfffffffffffffffe
0x1402cf8bd 4889ce mov rsi, rcx
0x1402cf8c0 0f57c0 xorps xmm0, xmm0
0x1402cf8c3 0f1145f8 movups xmmword ptr [rbp - 8], xmm0
0x1402cf8c7 0f114510 movups xmmword ptr [rbp + 0x10], xmm0
0x1402cf8cb 0f114528 movups xmmword ptr [rbp + 0x28], xmm0
0x1402cf8cf 0f114540 movups xmmword ptr [rbp + 0x40], xmm0
0x1402cf8d3 0f114558 movups xmmword ptr [rbp + 0x58], xmm0
0x1402cf8d7 c7456800000000 mov dword ptr [rbp + 0x68], 0
0x1402cf8de 0f118590000000 movups xmmword ptr [rbp + 0x90], xmm0
0x1402cf8e5 0f1185a8000000 movups xmmword ptr [rbp + 0xa8], xmm0
0x1402cf8ec 0f1185c0000000 movups xmmword ptr [rbp + 0xc0], xmm0
0x1402cf8f3 0f1185d8000000 movups xmmword ptr [rbp + 0xd8], xmm0
0x1402cf8fa 0f1185f0000000 movups xmmword ptr [rbp + 0xf0], xmm0
0x1402cf901 c7850001000000000000 mov dword ptr [rbp + 0x100], 0
0x1402cf90b 48c745a800000000 mov qword ptr [rbp - 0x58], 0
0x1402cf913 48c745d800000000 mov qword ptr [rbp - 0x28], 0
0x1402cf91b 48c745e008000000 mov qword ptr [rbp - 0x20], 8
0x1402cf923 48c745e800000000 mov qword ptr [rbp - 0x18], 0
0x1402cf92b 48c745f010000000 mov qword ptr [rbp - 0x10], 0x10
0x1402cf933 48c7450810000000 mov qword ptr [rbp + 8], 0x10
0x1402cf93b 48c7452010000000 mov qword ptr [rbp + 0x20], 0x10
0x1402cf943 48c7453810000000 mov qword ptr [rbp + 0x38], 0x10
0x1402cf94b 48c7455010000000 mov qword ptr [rbp + 0x50], 0x10
0x1402cf953 48b80000000000000080 movabs rax, 0x8000000000000000
0x1402cf95d 48894570 mov qword ptr [rbp + 0x70], rax
0x1402cf961 488d0575206e02 lea rax, [rip + 0x26e2075]
0x1402cf968 48894578 mov qword ptr [rbp + 0x78], rax
0x1402cf96c 48c7858000000043000000 mov qword ptr [rbp + 0x80], 0x43
0x1402cf977 48c7858800000010000000 mov qword ptr [rbp + 0x88], 0x10
0x1402cf982 48c785a000000010000000 mov qword ptr [rbp + 0xa0], 0x10
0x1402cf98d 48c785b800000010000000 mov qword ptr [rbp + 0xb8], 0x10
0x1402cf998 48c785d000000010000000 mov qword ptr [rbp + 0xd0], 0x10
0x1402cf9a3 48c785e800000010000000 mov qword ptr [rbp + 0xe8], 0x10
0x1402cf9ae c7850801000000000000 mov dword ptr [rbp + 0x108], 0
0x1402cf9b8 66c7850c0100000100 mov word ptr [rbp + 0x10c], 1
0x1402cf9c1 c7851001000000000000 mov dword ptr [rbp + 0x110], 0
0x1402cf9cb 0fb60537523603 movzx eax, byte ptr [rip + 0x3365237]
0x1402cf9d2 b970010000 mov ecx, 0x170
0x1402cf9d7 ba08000000 mov edx, 8
0x1402cf9dc e82f423400 call 0x140613c10
0x1402cf9e1 4885c0 test rax, rax
0x1402cf9e4 7435 je 0x1402cfa1b
0x1402cf9e6 4889c7 mov rdi, rax
0x1402cf9e9 488d55a8 lea rdx, [rbp - 0x58]
0x1402cf9ed 41b870010000 mov r8d, 0x170
0x1402cf9f3 4889c1 mov rcx, rax
0x1402cf9f6 e805426502 call 0x142923c00
0x1402cf9fb 4c8d054e1e6e02 lea r8, [rip + 0x26e1e4e]
0x1402cfa02 4889f1 mov rcx, rsi
0x1402cfa05 4889fa mov rdx, rdi
0x1402cfa08 e80351a100 call 0x140ce4b10
0x1402cfa0d 4889f0 mov rax, rsi
0x1402cfa10 4881c4a0010000 add rsp, 0x1a0
0x1402cfa17 5f pop rdi
0x1402cfa18 5e pop rsi
0x1402cfa19 5d pop rbp
0x1402cfa1a c3 ret 
0x1402cfa1b b908000000 mov ecx, 8
0x1402cfa20 ba70010000 mov edx, 0x170
0x1402cfa25 e8d6956002 call 0x1428d9000
0x1402cfa2a 0f0b ud2 
0x1402cfa2c 0f1f4000 nop dword ptr [rax]
0x1402cfa30 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1402cfa35 55 push rbp
0x1402cfa36 56 push rsi
0x1402cfa37 57 push rdi
0x1402cfa38 4883ec20 sub rsp, 0x20
0x1402cfa3c 488daa80000000 lea rbp, [rdx + 0x80]
0x1402cfa43 488d4da8 lea rcx, [rbp - 0x58]
0x1402cfa47 e844f13100 call 0x1405eeb90
0x1402cfa4c 90 nop 
0x1402cfa4d 4883c420 add rsp, 0x20
0x1402cfa51 5f pop rdi
0x1402cfa52 5e pop rsi
0x1402cfa53 5d pop rbp
0x1402cfa54 c3 ret 
