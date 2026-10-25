0x1405c6fb0 55 push rbp
0x1405c6fb1 56 push rsi
0x1405c6fb2 57 push rdi
0x1405c6fb3 4881eca0010000 sub rsp, 0x1a0
0x1405c6fba 488dac2480000000 lea rbp, [rsp + 0x80]
0x1405c6fc2 48c78518010000feffffff mov qword ptr [rbp + 0x118], 0xfffffffffffffffe
0x1405c6fcd 4889ce mov rsi, rcx
0x1405c6fd0 0f57c0 xorps xmm0, xmm0
0x1405c6fd3 0f1145f8 movups xmmword ptr [rbp - 8], xmm0
0x1405c6fd7 0f114510 movups xmmword ptr [rbp + 0x10], xmm0
0x1405c6fdb 0f114528 movups xmmword ptr [rbp + 0x28], xmm0
0x1405c6fdf 0f114540 movups xmmword ptr [rbp + 0x40], xmm0
0x1405c6fe3 0f114558 movups xmmword ptr [rbp + 0x58], xmm0
0x1405c6fe7 c7456800000000 mov dword ptr [rbp + 0x68], 0
0x1405c6fee 0f118590000000 movups xmmword ptr [rbp + 0x90], xmm0
0x1405c6ff5 0f1185a8000000 movups xmmword ptr [rbp + 0xa8], xmm0
0x1405c6ffc 0f1185c0000000 movups xmmword ptr [rbp + 0xc0], xmm0
0x1405c7003 0f1185d8000000 movups xmmword ptr [rbp + 0xd8], xmm0
0x1405c700a 0f1185f0000000 movups xmmword ptr [rbp + 0xf0], xmm0
0x1405c7011 c7850001000000000000 mov dword ptr [rbp + 0x100], 0
0x1405c701b 48c745a800000000 mov qword ptr [rbp - 0x58], 0
0x1405c7023 48c745d800000000 mov qword ptr [rbp - 0x28], 0
0x1405c702b 48c745e008000000 mov qword ptr [rbp - 0x20], 8
0x1405c7033 48c745e800000000 mov qword ptr [rbp - 0x18], 0
0x1405c703b 48c745f010000000 mov qword ptr [rbp - 0x10], 0x10
0x1405c7043 48c7450810000000 mov qword ptr [rbp + 8], 0x10
0x1405c704b 48c7452010000000 mov qword ptr [rbp + 0x20], 0x10
0x1405c7053 48c7453810000000 mov qword ptr [rbp + 0x38], 0x10
0x1405c705b 48c7455010000000 mov qword ptr [rbp + 0x50], 0x10
0x1405c7063 48b80000000000000080 movabs rax, 0x8000000000000000
0x1405c706d 48894570 mov qword ptr [rbp + 0x70], rax
0x1405c7071 488d056af14302 lea rax, [rip + 0x243f16a]
0x1405c7078 48894578 mov qword ptr [rbp + 0x78], rax
0x1405c707c 48c785800000003b000000 mov qword ptr [rbp + 0x80], 0x3b
0x1405c7087 48c7858800000010000000 mov qword ptr [rbp + 0x88], 0x10
0x1405c7092 48c785a000000010000000 mov qword ptr [rbp + 0xa0], 0x10
0x1405c709d 48c785b800000010000000 mov qword ptr [rbp + 0xb8], 0x10
0x1405c70a8 48c785d000000010000000 mov qword ptr [rbp + 0xd0], 0x10
0x1405c70b3 48c785e800000010000000 mov qword ptr [rbp + 0xe8], 0x10
0x1405c70be c7850801000000000000 mov dword ptr [rbp + 0x108], 0
0x1405c70c8 66c7850c0100000100 mov word ptr [rbp + 0x10c], 1
0x1405c70d1 c7851001000000000000 mov dword ptr [rbp + 0x110], 0
0x1405c70db 0fb60527db0603 movzx eax, byte ptr [rip + 0x306db27]
0x1405c70e2 b970010000 mov ecx, 0x170
0x1405c70e7 ba08000000 mov edx, 8
0x1405c70ec e81fcb0400 call 0x140613c10
0x1405c70f1 4885c0 test rax, rax
0x1405c70f4 7435 je 0x1405c712b
0x1405c70f6 4889c7 mov rdi, rax
0x1405c70f9 488d55a8 lea rdx, [rbp - 0x58]
0x1405c70fd 41b870010000 mov r8d, 0x170
0x1405c7103 4889c1 mov rcx, rax
0x1405c7106 e8f5ca3502 call 0x142923c00
0x1405c710b 4c8d052eee4302 lea r8, [rip + 0x243ee2e]
0x1405c7112 4889f1 mov rcx, rsi
0x1405c7115 4889fa mov rdx, rdi
0x1405c7118 e8f3d97100 call 0x140ce4b10
0x1405c711d 4889f0 mov rax, rsi
0x1405c7120 4881c4a0010000 add rsp, 0x1a0
0x1405c7127 5f pop rdi
0x1405c7128 5e pop rsi
0x1405c7129 5d pop rbp
0x1405c712a c3 ret 
0x1405c712b b908000000 mov ecx, 8
0x1405c7130 ba70010000 mov edx, 0x170
0x1405c7135 e8c61e3102 call 0x1428d9000
0x1405c713a 0f0b ud2 
0x1405c713c 0f1f4000 nop dword ptr [rax]
0x1405c7140 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1405c7145 55 push rbp
0x1405c7146 56 push rsi
0x1405c7147 57 push rdi
0x1405c7148 4883ec20 sub rsp, 0x20
0x1405c714c 488daa80000000 lea rbp, [rdx + 0x80]
0x1405c7153 488d4da8 lea rcx, [rbp - 0x58]
0x1405c7157 e8347a0200 call 0x1405eeb90
0x1405c715c 90 nop 
0x1405c715d 4883c420 add rsp, 0x20
0x1405c7161 5f pop rdi
0x1405c7162 5e pop rsi
0x1405c7163 5d pop rbp
0x1405c7164 c3 ret 
