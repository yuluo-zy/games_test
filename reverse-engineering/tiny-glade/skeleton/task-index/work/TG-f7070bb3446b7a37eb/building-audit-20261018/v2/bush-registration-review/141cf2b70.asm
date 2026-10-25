0x141cf2b70 55 push rbp
0x141cf2b71 56 push rsi
0x141cf2b72 57 push rdi
0x141cf2b73 4881ec20030000 sub rsp, 0x320
0x141cf2b7a 488dac2480000000 lea rbp, [rsp + 0x80]
0x141cf2b82 48c78598020000feffffff mov qword ptr [rbp + 0x298], 0xfffffffffffffffe
0x141cf2b8d 4889ce mov rsi, rcx
0x141cf2b90 0f57c0 xorps xmm0, xmm0
0x141cf2b93 0f1145c8 movups xmmword ptr [rbp - 0x38], xmm0
0x141cf2b97 0f1145e0 movups xmmword ptr [rbp - 0x20], xmm0
0x141cf2b9b 0f1145f8 movups xmmword ptr [rbp - 8], xmm0
0x141cf2b9f 0f114510 movups xmmword ptr [rbp + 0x10], xmm0
0x141cf2ba3 0f114528 movups xmmword ptr [rbp + 0x28], xmm0
0x141cf2ba7 c7453800000000 mov dword ptr [rbp + 0x38], 0
0x141cf2bae 0f114560 movups xmmword ptr [rbp + 0x60], xmm0
0x141cf2bb2 0f114578 movups xmmword ptr [rbp + 0x78], xmm0
0x141cf2bb6 0f118590000000 movups xmmword ptr [rbp + 0x90], xmm0
0x141cf2bbd 0f1185a8000000 movups xmmword ptr [rbp + 0xa8], xmm0
0x141cf2bc4 0f1185c0000000 movups xmmword ptr [rbp + 0xc0], xmm0
0x141cf2bcb c785d000000000000000 mov dword ptr [rbp + 0xd0], 0
0x141cf2bd5 48c745a800000000 mov qword ptr [rbp - 0x58], 0
0x141cf2bdd 48c745b008000000 mov qword ptr [rbp - 0x50], 8
0x141cf2be5 48c745b800000000 mov qword ptr [rbp - 0x48], 0
0x141cf2bed 48c745c010000000 mov qword ptr [rbp - 0x40], 0x10
0x141cf2bf5 48c745d810000000 mov qword ptr [rbp - 0x28], 0x10
0x141cf2bfd 48c745f010000000 mov qword ptr [rbp - 0x10], 0x10
0x141cf2c05 48c7450810000000 mov qword ptr [rbp + 8], 0x10
0x141cf2c0d 48c7452010000000 mov qword ptr [rbp + 0x20], 0x10
0x141cf2c15 48b80000000000000080 movabs rax, 0x8000000000000000
0x141cf2c1f 48894540 mov qword ptr [rbp + 0x40], rax
0x141cf2c23 488d0db6682401 lea rcx, [rip + 0x12468b6]
0x141cf2c2a 48894d48 mov qword ptr [rbp + 0x48], rcx
0x141cf2c2e 48c745504f000000 mov qword ptr [rbp + 0x50], 0x4f
0x141cf2c36 48c7455810000000 mov qword ptr [rbp + 0x58], 0x10
0x141cf2c3e 48c7457010000000 mov qword ptr [rbp + 0x70], 0x10
0x141cf2c46 48c7858800000010000000 mov qword ptr [rbp + 0x88], 0x10
0x141cf2c51 48c785a000000010000000 mov qword ptr [rbp + 0xa0], 0x10
0x141cf2c5c 48c785b800000010000000 mov qword ptr [rbp + 0xb8], 0x10
0x141cf2c67 c785d800000000000000 mov dword ptr [rbp + 0xd8], 0
0x141cf2c71 66c785dc0000000100 mov word ptr [rbp + 0xdc], 1
0x141cf2c7a 488985e0000000 mov qword ptr [rbp + 0xe0], rax
0x141cf2c81 c7859002000000000000 mov dword ptr [rbp + 0x290], 0
0x141cf2c8b 0fb605771f9401 movzx eax, byte ptr [rip + 0x1941f77]
0x141cf2c92 b9f0020000 mov ecx, 0x2f0
0x141cf2c97 ba08000000 mov edx, 8
0x141cf2c9c e86f0f92fe call 0x140613c10
0x141cf2ca1 4885c0 test rax, rax
0x141cf2ca4 7435 je 0x141cf2cdb
0x141cf2ca6 4889c7 mov rdi, rax
0x141cf2ca9 488d55a8 lea rdx, [rbp - 0x58]
0x141cf2cad 41b8f0020000 mov r8d, 0x2f0
0x141cf2cb3 4889c1 mov rcx, rax
0x141cf2cb6 e8450fc300 call 0x142923c00
0x141cf2cbb 4c8d05d6652401 lea r8, [rip + 0x12465d6]
0x141cf2cc2 4889f1 mov rcx, rsi
0x141cf2cc5 4889fa mov rdx, rdi
0x141cf2cc8 e8431efffe call 0x140ce4b10
0x141cf2ccd 4889f0 mov rax, rsi
0x141cf2cd0 4881c420030000 add rsp, 0x320
0x141cf2cd7 5f pop rdi
0x141cf2cd8 5e pop rsi
0x141cf2cd9 5d pop rbp
0x141cf2cda c3 ret 
0x141cf2cdb b908000000 mov ecx, 8
0x141cf2ce0 baf0020000 mov edx, 0x2f0
0x141cf2ce5 e81663be00 call 0x1428d9000
0x141cf2cea 0f0b ud2 
0x141cf2cec 0f1f4000 nop dword ptr [rax]
0x141cf2cf0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x141cf2cf5 55 push rbp
0x141cf2cf6 56 push rsi
0x141cf2cf7 57 push rdi
0x141cf2cf8 4883ec20 sub rsp, 0x20
0x141cf2cfc 488daa80000000 lea rbp, [rdx + 0x80]
0x141cf2d03 488d4da8 lea rcx, [rbp - 0x58]
0x141cf2d07 e884448efe call 0x1405d7190
0x141cf2d0c 90 nop 
0x141cf2d0d 4883c420 add rsp, 0x20
0x141cf2d11 5f pop rdi
0x141cf2d12 5e pop rsi
0x141cf2d13 5d pop rbp
0x141cf2d14 c3 ret 
