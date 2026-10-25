0x1400ece00 55 push rbp
0x1400ece01 56 push rsi
0x1400ece02 57 push rdi
0x1400ece03 4881ec80010000 sub rsp, 0x180
0x1400ece0a 488dac2480000000 lea rbp, [rsp + 0x80]
0x1400ece12 48c785f8000000feffffff mov qword ptr [rbp + 0xf8], 0xfffffffffffffffe
0x1400ece1d 4889ce mov rsi, rcx
0x1400ece20 0f57c0 xorps xmm0, xmm0
0x1400ece23 0f1145d8 movups xmmword ptr [rbp - 0x28], xmm0
0x1400ece27 0f1145f0 movups xmmword ptr [rbp - 0x10], xmm0
0x1400ece2b 0f114508 movups xmmword ptr [rbp + 8], xmm0
0x1400ece2f 0f114520 movups xmmword ptr [rbp + 0x20], xmm0
0x1400ece33 0f114538 movups xmmword ptr [rbp + 0x38], xmm0
0x1400ece37 c7454800000000 mov dword ptr [rbp + 0x48], 0
0x1400ece3e 0f114570 movups xmmword ptr [rbp + 0x70], xmm0
0x1400ece42 0f118588000000 movups xmmword ptr [rbp + 0x88], xmm0
0x1400ece49 0f1185a0000000 movups xmmword ptr [rbp + 0xa0], xmm0
0x1400ece50 0f1185b8000000 movups xmmword ptr [rbp + 0xb8], xmm0
0x1400ece57 0f1185d0000000 movups xmmword ptr [rbp + 0xd0], xmm0
0x1400ece5e c785e000000000000000 mov dword ptr [rbp + 0xe0], 0
0x1400ece68 48c745a000000000 mov qword ptr [rbp - 0x60], 0
0x1400ece70 48c745b800000000 mov qword ptr [rbp - 0x48], 0
0x1400ece78 48c745c008000000 mov qword ptr [rbp - 0x40], 8
0x1400ece80 48c745c800000000 mov qword ptr [rbp - 0x38], 0
0x1400ece88 48c745d010000000 mov qword ptr [rbp - 0x30], 0x10
0x1400ece90 48c745e810000000 mov qword ptr [rbp - 0x18], 0x10
0x1400ece98 48c7450010000000 mov qword ptr [rbp], 0x10
0x1400ecea0 48c7451810000000 mov qword ptr [rbp + 0x18], 0x10
0x1400ecea8 48c7453010000000 mov qword ptr [rbp + 0x30], 0x10
0x1400eceb0 48b80000000000000080 movabs rax, 0x8000000000000000
0x1400eceba 48894550 mov qword ptr [rbp + 0x50], rax
0x1400ecebe 488d0522058502 lea rax, [rip + 0x2850522]
0x1400ecec5 48894558 mov qword ptr [rbp + 0x58], rax
0x1400ecec9 48c745604b000000 mov qword ptr [rbp + 0x60], 0x4b
0x1400eced1 48c7456810000000 mov qword ptr [rbp + 0x68], 0x10
0x1400eced9 48c7858000000010000000 mov qword ptr [rbp + 0x80], 0x10
0x1400ecee4 48c7859800000010000000 mov qword ptr [rbp + 0x98], 0x10
0x1400eceef 48c785b000000010000000 mov qword ptr [rbp + 0xb0], 0x10
0x1400ecefa 48c785c800000010000000 mov qword ptr [rbp + 0xc8], 0x10
0x1400ecf05 c785e800000000000000 mov dword ptr [rbp + 0xe8], 0
0x1400ecf0f 66c785ec0000000100 mov word ptr [rbp + 0xec], 1
0x1400ecf18 c785f000000000000000 mov dword ptr [rbp + 0xf0], 0
0x1400ecf22 0fb605e07c5403 movzx eax, byte ptr [rip + 0x3547ce0]
0x1400ecf29 b958010000 mov ecx, 0x158
0x1400ecf2e ba08000000 mov edx, 8
0x1400ecf33 e8d86c5200 call 0x140613c10
0x1400ecf38 4885c0 test rax, rax
0x1400ecf3b 7435 je 0x1400ecf72
0x1400ecf3d 4889c7 mov rdi, rax
0x1400ecf40 488d55a0 lea rdx, [rbp - 0x60]
0x1400ecf44 41b858010000 mov r8d, 0x158
0x1400ecf4a 4889c1 mov rcx, rax
0x1400ecf4d e8ae6c8302 call 0x142923c00
0x1400ecf52 4c8d054f028502 lea r8, [rip + 0x285024f]
0x1400ecf59 4889f1 mov rcx, rsi
0x1400ecf5c 4889fa mov rdx, rdi
0x1400ecf5f e8ac7bbf00 call 0x140ce4b10
0x1400ecf64 4889f0 mov rax, rsi
0x1400ecf67 4881c480010000 add rsp, 0x180
0x1400ecf6e 5f pop rdi
0x1400ecf6f 5e pop rsi
0x1400ecf70 5d pop rbp
0x1400ecf71 c3 ret 
0x1400ecf72 b908000000 mov ecx, 8
0x1400ecf77 ba58010000 mov edx, 0x158
0x1400ecf7c e87fc07e02 call 0x1428d9000
0x1400ecf81 0f0b ud2 
0x1400ecf83 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1400ecf90 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1400ecf95 55 push rbp
0x1400ecf96 56 push rsi
0x1400ecf97 57 push rdi
0x1400ecf98 4883ec20 sub rsp, 0x20
0x1400ecf9c 488daa80000000 lea rbp, [rdx + 0x80]
0x1400ecfa3 488d4da0 lea rcx, [rbp - 0x60]
0x1400ecfa7 e854824f00 call 0x1405e5200
0x1400ecfac 90 nop 
0x1400ecfad 4883c420 add rsp, 0x20
0x1400ecfb1 5f pop rdi
0x1400ecfb2 5e pop rsi
0x1400ecfb3 5d pop rbp
0x1400ecfb4 c3 ret 
