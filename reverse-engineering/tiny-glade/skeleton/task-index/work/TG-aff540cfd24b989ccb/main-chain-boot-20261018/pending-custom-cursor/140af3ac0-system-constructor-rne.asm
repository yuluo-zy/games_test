0x140af3ac0 55 push rbp
0x140af3ac1 56 push rsi
0x140af3ac2 57 push rdi
0x140af3ac3 4881ecb0010000 sub rsp, 0x1b0
0x140af3aca 488dac2480000000 lea rbp, [rsp + 0x80]
0x140af3ad2 48c78528010000feffffff mov qword ptr [rbp + 0x128], 0xfffffffffffffffe
0x140af3add 4889ce mov rsi, rcx
0x140af3ae0 0f57c0 xorps xmm0, xmm0
0x140af3ae3 0f1145c8 movups xmmword ptr [rbp - 0x38], xmm0
0x140af3ae7 0f1145e0 movups xmmword ptr [rbp - 0x20], xmm0
0x140af3aeb 0f1145f8 movups xmmword ptr [rbp - 8], xmm0
0x140af3aef 0f114510 movups xmmword ptr [rbp + 0x10], xmm0
0x140af3af3 0f114528 movups xmmword ptr [rbp + 0x28], xmm0
0x140af3af7 c7453800000000 mov dword ptr [rbp + 0x38], 0
0x140af3afe 0f114560 movups xmmword ptr [rbp + 0x60], xmm0
0x140af3b02 0f114578 movups xmmword ptr [rbp + 0x78], xmm0
0x140af3b06 0f118590000000 movups xmmword ptr [rbp + 0x90], xmm0
0x140af3b0d 0f1185a8000000 movups xmmword ptr [rbp + 0xa8], xmm0
0x140af3b14 0f1185c0000000 movups xmmword ptr [rbp + 0xc0], xmm0
0x140af3b1b c785d000000000000000 mov dword ptr [rbp + 0xd0], 0
0x140af3b25 48c745a800000000 mov qword ptr [rbp - 0x58], 0
0x140af3b2d 48c745b008000000 mov qword ptr [rbp - 0x50], 8
0x140af3b35 48c745b800000000 mov qword ptr [rbp - 0x48], 0
0x140af3b3d 48c745c010000000 mov qword ptr [rbp - 0x40], 0x10
0x140af3b45 48c745d810000000 mov qword ptr [rbp - 0x28], 0x10
0x140af3b4d 48c745f010000000 mov qword ptr [rbp - 0x10], 0x10
0x140af3b55 48c7450810000000 mov qword ptr [rbp + 8], 0x10
0x140af3b5d 48c7452010000000 mov qword ptr [rbp + 0x20], 0x10
0x140af3b65 48b80000000000000080 movabs rax, 0x8000000000000000
0x140af3b6f 48894540 mov qword ptr [rbp + 0x40], rax
0x140af3b73 488d0db013ff01 lea rcx, [rip + 0x1ff13b0]
0x140af3b7a 48894d48 mov qword ptr [rbp + 0x48], rcx
0x140af3b7e 48c7455038000000 mov qword ptr [rbp + 0x50], 0x38
0x140af3b86 48c7455810000000 mov qword ptr [rbp + 0x58], 0x10
0x140af3b8e 48c7457010000000 mov qword ptr [rbp + 0x70], 0x10
0x140af3b96 48c7858800000010000000 mov qword ptr [rbp + 0x88], 0x10
0x140af3ba1 48c785a000000010000000 mov qword ptr [rbp + 0xa0], 0x10
0x140af3bac 48c785b800000010000000 mov qword ptr [rbp + 0xb8], 0x10
0x140af3bb7 c785d800000000000000 mov dword ptr [rbp + 0xd8], 0
0x140af3bc1 66c785dc0000000100 mov word ptr [rbp + 0xdc], 1
0x140af3bca 488985e0000000 mov qword ptr [rbp + 0xe0], rax
0x140af3bd1 c7852001000000000000 mov dword ptr [rbp + 0x120], 0
0x140af3bdb 0fb6052710b402 movzx eax, byte ptr [rip + 0x2b41027]
0x140af3be2 b980010000 mov ecx, 0x180
0x140af3be7 ba08000000 mov edx, 8
0x140af3bec e81f00b2ff call 0x140613c10
0x140af3bf1 4885c0 test rax, rax
0x140af3bf4 7435 je 0x140af3c2b
0x140af3bf6 4889c7 mov rdi, rax
0x140af3bf9 488d55a8 lea rdx, [rbp - 0x58]
0x140af3bfd 41b880010000 mov r8d, 0x180
0x140af3c03 4889c1 mov rcx, rax
0x140af3c06 e8f5ffe201 call 0x142923c00
0x140af3c0b 4c8d05460fff01 lea r8, [rip + 0x1ff0f46]
0x140af3c12 4889f1 mov rcx, rsi
0x140af3c15 4889fa mov rdx, rdi
0x140af3c18 e8f30e1f00 call 0x140ce4b10
0x140af3c1d 4889f0 mov rax, rsi
0x140af3c20 4881c4b0010000 add rsp, 0x1b0
0x140af3c27 5f pop rdi
0x140af3c28 5e pop rsi
0x140af3c29 5d pop rbp
0x140af3c2a c3 ret 
0x140af3c2b b908000000 mov ecx, 8
0x140af3c30 ba80010000 mov edx, 0x180
0x140af3c35 e8c653de01 call 0x1428d9000
0x140af3c3a 0f0b ud2 
0x140af3c3c 0f1f4000 nop dword ptr [rax]
0x140af3c40 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140af3c45 55 push rbp
0x140af3c46 56 push rsi
0x140af3c47 57 push rdi
0x140af3c48 4883ec20 sub rsp, 0x20
0x140af3c4c 488daa80000000 lea rbp, [rdx + 0x80]
0x140af3c53 488d4da8 lea rcx, [rbp - 0x58]
0x140af3c57 e88417aeff call 0x1405d53e0
0x140af3c5c 90 nop 
0x140af3c5d 4883c420 add rsp, 0x20
0x140af3c61 5f pop rdi
0x140af3c62 5e pop rsi
0x140af3c63 5d pop rbp
0x140af3c64 c3 ret 
