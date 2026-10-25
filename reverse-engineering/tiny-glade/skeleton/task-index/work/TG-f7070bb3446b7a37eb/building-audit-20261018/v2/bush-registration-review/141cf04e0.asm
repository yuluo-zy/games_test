0x141cf04e0 55 push rbp
0x141cf04e1 56 push rsi
0x141cf04e2 57 push rdi
0x141cf04e3 4881eca0010000 sub rsp, 0x1a0
0x141cf04ea 488dac2480000000 lea rbp, [rsp + 0x80]
0x141cf04f2 48c78518010000feffffff mov qword ptr [rbp + 0x118], 0xfffffffffffffffe
0x141cf04fd 4889ce mov rsi, rcx
0x141cf0500 0f57c0 xorps xmm0, xmm0
0x141cf0503 0f1145f8 movups xmmword ptr [rbp - 8], xmm0
0x141cf0507 0f114510 movups xmmword ptr [rbp + 0x10], xmm0
0x141cf050b 0f114528 movups xmmword ptr [rbp + 0x28], xmm0
0x141cf050f 0f114540 movups xmmword ptr [rbp + 0x40], xmm0
0x141cf0513 0f114558 movups xmmword ptr [rbp + 0x58], xmm0
0x141cf0517 c7456800000000 mov dword ptr [rbp + 0x68], 0
0x141cf051e 0f118590000000 movups xmmword ptr [rbp + 0x90], xmm0
0x141cf0525 0f1185a8000000 movups xmmword ptr [rbp + 0xa8], xmm0
0x141cf052c 0f1185c0000000 movups xmmword ptr [rbp + 0xc0], xmm0
0x141cf0533 0f1185d8000000 movups xmmword ptr [rbp + 0xd8], xmm0
0x141cf053a 0f1185f0000000 movups xmmword ptr [rbp + 0xf0], xmm0
0x141cf0541 c7850001000000000000 mov dword ptr [rbp + 0x100], 0
0x141cf054b 48c745a800000000 mov qword ptr [rbp - 0x58], 0
0x141cf0553 48c745d800000000 mov qword ptr [rbp - 0x28], 0
0x141cf055b 48c745e008000000 mov qword ptr [rbp - 0x20], 8
0x141cf0563 48c745e800000000 mov qword ptr [rbp - 0x18], 0
0x141cf056b 48c745f010000000 mov qword ptr [rbp - 0x10], 0x10
0x141cf0573 48c7450810000000 mov qword ptr [rbp + 8], 0x10
0x141cf057b 48c7452010000000 mov qword ptr [rbp + 0x20], 0x10
0x141cf0583 48c7453810000000 mov qword ptr [rbp + 0x38], 0x10
0x141cf058b 48c7455010000000 mov qword ptr [rbp + 0x50], 0x10
0x141cf0593 48b80000000000000080 movabs rax, 0x8000000000000000
0x141cf059d 48894570 mov qword ptr [rbp + 0x70], rax
0x141cf05a1 488d05387f2401 lea rax, [rip + 0x1247f38]
0x141cf05a8 48894578 mov qword ptr [rbp + 0x78], rax
0x141cf05ac 48c785800000005b000000 mov qword ptr [rbp + 0x80], 0x5b
0x141cf05b7 48c7858800000010000000 mov qword ptr [rbp + 0x88], 0x10
0x141cf05c2 48c785a000000010000000 mov qword ptr [rbp + 0xa0], 0x10
0x141cf05cd 48c785b800000010000000 mov qword ptr [rbp + 0xb8], 0x10
0x141cf05d8 48c785d000000010000000 mov qword ptr [rbp + 0xd0], 0x10
0x141cf05e3 48c785e800000010000000 mov qword ptr [rbp + 0xe8], 0x10
0x141cf05ee c7850801000000000000 mov dword ptr [rbp + 0x108], 0
0x141cf05f8 66c7850c0100000100 mov word ptr [rbp + 0x10c], 1
0x141cf0601 c7851001000000000000 mov dword ptr [rbp + 0x110], 0
0x141cf060b 0fb605f7459401 movzx eax, byte ptr [rip + 0x19445f7]
0x141cf0612 b970010000 mov ecx, 0x170
0x141cf0617 ba08000000 mov edx, 8
0x141cf061c e8ef3592fe call 0x140613c10
0x141cf0621 4885c0 test rax, rax
0x141cf0624 7435 je 0x141cf065b
0x141cf0626 4889c7 mov rdi, rax
0x141cf0629 488d55a8 lea rdx, [rbp - 0x58]
0x141cf062d 41b870010000 mov r8d, 0x170
0x141cf0633 4889c1 mov rcx, rax
0x141cf0636 e8c535c300 call 0x142923c00
0x141cf063b 4c8d05e67d2401 lea r8, [rip + 0x1247de6]
0x141cf0642 4889f1 mov rcx, rsi
0x141cf0645 4889fa mov rdx, rdi
0x141cf0648 e8c344fffe call 0x140ce4b10
0x141cf064d 4889f0 mov rax, rsi
0x141cf0650 4881c4a0010000 add rsp, 0x1a0
0x141cf0657 5f pop rdi
0x141cf0658 5e pop rsi
0x141cf0659 5d pop rbp
0x141cf065a c3 ret 
0x141cf065b b908000000 mov ecx, 8
0x141cf0660 ba70010000 mov edx, 0x170
0x141cf0665 e89689be00 call 0x1428d9000
0x141cf066a 0f0b ud2 
0x141cf066c 0f1f4000 nop dword ptr [rax]
0x141cf0670 4889542410 mov qword ptr [rsp + 0x10], rdx
0x141cf0675 55 push rbp
0x141cf0676 56 push rsi
0x141cf0677 57 push rdi
0x141cf0678 4883ec20 sub rsp, 0x20
0x141cf067c 488daa80000000 lea rbp, [rdx + 0x80]
0x141cf0683 488d4da8 lea rcx, [rbp - 0x58]
0x141cf0687 e804e58ffe call 0x1405eeb90
0x141cf068c 90 nop 
0x141cf068d 4883c420 add rsp, 0x20
0x141cf0691 5f pop rdi
0x141cf0692 5e pop rsi
0x141cf0693 5d pop rbp
0x141cf0694 c3 ret 
