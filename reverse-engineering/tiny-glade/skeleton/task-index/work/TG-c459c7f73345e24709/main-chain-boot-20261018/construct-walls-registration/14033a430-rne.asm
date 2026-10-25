0x14033a430 55 push rbp
0x14033a431 56 push rsi
0x14033a432 57 push rdi
0x14033a433 4881eca0010000 sub rsp, 0x1a0
0x14033a43a 488dac2480000000 lea rbp, [rsp + 0x80]
0x14033a442 48c78518010000feffffff mov qword ptr [rbp + 0x118], 0xfffffffffffffffe
0x14033a44d 4889ce mov rsi, rcx
0x14033a450 0f57c0 xorps xmm0, xmm0
0x14033a453 0f1145f8 movups xmmword ptr [rbp - 8], xmm0
0x14033a457 0f114510 movups xmmword ptr [rbp + 0x10], xmm0
0x14033a45b 0f114528 movups xmmword ptr [rbp + 0x28], xmm0
0x14033a45f 0f114540 movups xmmword ptr [rbp + 0x40], xmm0
0x14033a463 0f114558 movups xmmword ptr [rbp + 0x58], xmm0
0x14033a467 c7456800000000 mov dword ptr [rbp + 0x68], 0
0x14033a46e 0f118590000000 movups xmmword ptr [rbp + 0x90], xmm0
0x14033a475 0f1185a8000000 movups xmmword ptr [rbp + 0xa8], xmm0
0x14033a47c 0f1185c0000000 movups xmmword ptr [rbp + 0xc0], xmm0
0x14033a483 0f1185d8000000 movups xmmword ptr [rbp + 0xd8], xmm0
0x14033a48a 0f1185f0000000 movups xmmword ptr [rbp + 0xf0], xmm0
0x14033a491 c7850001000000000000 mov dword ptr [rbp + 0x100], 0
0x14033a49b 48c745a800000000 mov qword ptr [rbp - 0x58], 0
0x14033a4a3 48c745d800000000 mov qword ptr [rbp - 0x28], 0
0x14033a4ab 48c745e008000000 mov qword ptr [rbp - 0x20], 8
0x14033a4b3 48c745e800000000 mov qword ptr [rbp - 0x18], 0
0x14033a4bb 48c745f010000000 mov qword ptr [rbp - 0x10], 0x10
0x14033a4c3 48c7450810000000 mov qword ptr [rbp + 8], 0x10
0x14033a4cb 48c7452010000000 mov qword ptr [rbp + 0x20], 0x10
0x14033a4d3 48c7453810000000 mov qword ptr [rbp + 0x38], 0x10
0x14033a4db 48c7455010000000 mov qword ptr [rbp + 0x50], 0x10
0x14033a4e3 48b80000000000000080 movabs rax, 0x8000000000000000
0x14033a4ed 48894570 mov qword ptr [rbp + 0x70], rax
0x14033a4f1 488d052d656802 lea rax, [rip + 0x268652d]
0x14033a4f8 48894578 mov qword ptr [rbp + 0x78], rax
0x14033a4fc 48c785800000003b000000 mov qword ptr [rbp + 0x80], 0x3b
0x14033a507 48c7858800000010000000 mov qword ptr [rbp + 0x88], 0x10
0x14033a512 48c785a000000010000000 mov qword ptr [rbp + 0xa0], 0x10
0x14033a51d 48c785b800000010000000 mov qword ptr [rbp + 0xb8], 0x10
0x14033a528 48c785d000000010000000 mov qword ptr [rbp + 0xd0], 0x10
0x14033a533 48c785e800000010000000 mov qword ptr [rbp + 0xe8], 0x10
0x14033a53e c7850801000000000000 mov dword ptr [rbp + 0x108], 0
0x14033a548 66c7850c0100000100 mov word ptr [rbp + 0x10c], 1
0x14033a551 c7851001000000000000 mov dword ptr [rbp + 0x110], 0
0x14033a55b 0fb605a7a62f03 movzx eax, byte ptr [rip + 0x32fa6a7]
0x14033a562 b970010000 mov ecx, 0x170
0x14033a567 ba08000000 mov edx, 8
0x14033a56c e89f962d00 call 0x140613c10
0x14033a571 4885c0 test rax, rax
0x14033a574 7435 je 0x14033a5ab
0x14033a576 4889c7 mov rdi, rax
0x14033a579 488d55a8 lea rdx, [rbp - 0x58]
0x14033a57d 41b870010000 mov r8d, 0x170
0x14033a583 4889c1 mov rcx, rax
0x14033a586 e875965e02 call 0x142923c00
0x14033a58b 4c8d055e606802 lea r8, [rip + 0x268605e]
0x14033a592 4889f1 mov rcx, rsi
0x14033a595 4889fa mov rdx, rdi
0x14033a598 e873a59a00 call 0x140ce4b10
0x14033a59d 4889f0 mov rax, rsi
0x14033a5a0 4881c4a0010000 add rsp, 0x1a0
0x14033a5a7 5f pop rdi
0x14033a5a8 5e pop rsi
0x14033a5a9 5d pop rbp
0x14033a5aa c3 ret 
0x14033a5ab b908000000 mov ecx, 8
0x14033a5b0 ba70010000 mov edx, 0x170
0x14033a5b5 e846ea5902 call 0x1428d9000
0x14033a5ba 0f0b ud2 
0x14033a5bc 0f1f4000 nop dword ptr [rax]
0x14033a5c0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14033a5c5 55 push rbp
0x14033a5c6 56 push rsi
0x14033a5c7 57 push rdi
0x14033a5c8 4883ec20 sub rsp, 0x20
0x14033a5cc 488daa80000000 lea rbp, [rdx + 0x80]
0x14033a5d3 488d4da8 lea rcx, [rbp - 0x58]
0x14033a5d7 e8b4452b00 call 0x1405eeb90
0x14033a5dc 90 nop 
0x14033a5dd 4883c420 add rsp, 0x20
0x14033a5e1 5f pop rdi
0x14033a5e2 5e pop rsi
0x14033a5e3 5d pop rbp
0x14033a5e4 c3 ret 
