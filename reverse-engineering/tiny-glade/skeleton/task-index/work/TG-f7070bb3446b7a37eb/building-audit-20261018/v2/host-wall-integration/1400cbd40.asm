0x1400cbd40 55 push rbp
0x1400cbd41 56 push rsi
0x1400cbd42 57 push rdi
0x1400cbd43 53 push rbx
0x1400cbd44 4881ec38050000 sub rsp, 0x538
0x1400cbd4b 488dac2480000000 lea rbp, [rsp + 0x80]
0x1400cbd53 48c785b0040000feffffff mov qword ptr [rbp + 0x4b0], 0xfffffffffffffffe
0x1400cbd5e 4889ce mov rsi, rcx
0x1400cbd61 0fb605a18e5603 movzx eax, byte ptr [rip + 0x3568ea1]
0x1400cbd68 b9c0010000 mov ecx, 0x1c0
0x1400cbd6d ba08000000 mov edx, 8
0x1400cbd72 e8997e5400 call 0x140613c10
0x1400cbd77 4885c0 test rax, rax
0x1400cbd7a 0f844c030000 je 0x1400cc0cc
0x1400cbd80 488985a8040000 mov qword ptr [rbp + 0x4a8], rax
0x1400cbd87 0f57c0 xorps xmm0, xmm0
0x1400cbd8a 0f1145c8 movups xmmword ptr [rbp - 0x38], xmm0
0x1400cbd8e 0f1145e0 movups xmmword ptr [rbp - 0x20], xmm0
0x1400cbd92 0f1145f8 movups xmmword ptr [rbp - 8], xmm0
0x1400cbd96 0f114510 movups xmmword ptr [rbp + 0x10], xmm0
0x1400cbd9a 0f114528 movups xmmword ptr [rbp + 0x28], xmm0
0x1400cbd9e c7453800000000 mov dword ptr [rbp + 0x38], 0
0x1400cbda5 0f114560 movups xmmword ptr [rbp + 0x60], xmm0
0x1400cbda9 0f114578 movups xmmword ptr [rbp + 0x78], xmm0
0x1400cbdad 0f118590000000 movups xmmword ptr [rbp + 0x90], xmm0
0x1400cbdb4 0f1185a8000000 movups xmmword ptr [rbp + 0xa8], xmm0
0x1400cbdbb 0f1185c0000000 movups xmmword ptr [rbp + 0xc0], xmm0
0x1400cbdc2 c785d000000000000000 mov dword ptr [rbp + 0xd0], 0
0x1400cbdcc 48c745a800000000 mov qword ptr [rbp - 0x58], 0
0x1400cbdd4 48c745b008000000 mov qword ptr [rbp - 0x50], 8
0x1400cbddc 48c745b800000000 mov qword ptr [rbp - 0x48], 0
0x1400cbde4 48c745c010000000 mov qword ptr [rbp - 0x40], 0x10
0x1400cbdec 48c745d810000000 mov qword ptr [rbp - 0x28], 0x10
0x1400cbdf4 48c745f010000000 mov qword ptr [rbp - 0x10], 0x10
0x1400cbdfc 48c7450810000000 mov qword ptr [rbp + 8], 0x10
0x1400cbe04 48c7452010000000 mov qword ptr [rbp + 0x20], 0x10
0x1400cbe0c 48bb0000000000000080 movabs rbx, 0x8000000000000000
0x1400cbe16 48895d40 mov qword ptr [rbp + 0x40], rbx
0x1400cbe1a 488d053cb88602 lea rax, [rip + 0x286b83c]
0x1400cbe21 48894548 mov qword ptr [rbp + 0x48], rax
0x1400cbe25 48c745506e000000 mov qword ptr [rbp + 0x50], 0x6e
0x1400cbe2d 48c7455810000000 mov qword ptr [rbp + 0x58], 0x10
0x1400cbe35 48c7457010000000 mov qword ptr [rbp + 0x70], 0x10
0x1400cbe3d 48c7858800000010000000 mov qword ptr [rbp + 0x88], 0x10
0x1400cbe48 48c785a000000010000000 mov qword ptr [rbp + 0xa0], 0x10
0x1400cbe53 48c785b800000010000000 mov qword ptr [rbp + 0xb8], 0x10
0x1400cbe5e c785d800000000000000 mov dword ptr [rbp + 0xd8], 0
0x1400cbe68 66c785dc0000000100 mov word ptr [rbp + 0xdc], 1
0x1400cbe71 48899dd0010000 mov qword ptr [rbp + 0x1d0], rbx
0x1400cbe78 c7855003000000000000 mov dword ptr [rbp + 0x350], 0
0x1400cbe82 0fb605808d5603 movzx eax, byte ptr [rip + 0x3568d80]
0x1400cbe89 b9b0030000 mov ecx, 0x3b0
0x1400cbe8e ba08000000 mov edx, 8
0x1400cbe93 e8787d5400 call 0x140613c10
0x1400cbe98 4885c0 test rax, rax
0x1400cbe9b 0f843a020000 je 0x1400cc0db
0x1400cbea1 4889c7 mov rdi, rax
0x1400cbea4 488d55a8 lea rdx, [rbp - 0x58]
0x1400cbea8 41b8b0030000 mov r8d, 0x3b0
0x1400cbeae 4889c1 mov rcx, rax
0x1400cbeb1 e84a7d8502 call 0x142923c00
0x1400cbeb6 4c8d0513b68602 lea r8, [rip + 0x286b613]
0x1400cbebd 488d8d38040000 lea rcx, [rbp + 0x438]
0x1400cbec4 4889fa mov rdx, rdi
0x1400cbec7 e8448cc100 call 0x140ce4b10
0x1400cbecc 488d8dc8030000 lea rcx, [rbp + 0x3c8]
0x1400cbed3 e878cdf7ff call 0x140048c50
0x1400cbed8 488d8d58030000 lea rcx, [rbp + 0x358]
0x1400cbedf e8fcd0f7ff call 0x140048fe0
0x1400cbee4 488d4da8 lea rcx, [rbp - 0x58]
0x1400cbee8 e823872000 call 0x1402d4610
0x1400cbeed 0f108598040000 movups xmm0, xmmword ptr [rbp + 0x498]
0x1400cbef4 488b85a8040000 mov rax, qword ptr [rbp + 0x4a8]
0x1400cbefb 0f114060 movups xmmword ptr [rax + 0x60], xmm0
0x1400cbeff 0f108588040000 movups xmm0, xmmword ptr [rbp + 0x488]
0x1400cbf06 0f114050 movups xmmword ptr [rax + 0x50], xmm0
0x1400cbf0a 0f108578040000 movups xmm0, xmmword ptr [rbp + 0x478]
0x1400cbf11 0f114040 movups xmmword ptr [rax + 0x40], xmm0
0x1400cbf15 0f108538040000 movups xmm0, xmmword ptr [rbp + 0x438]
0x1400cbf1c 0f108d48040000 movups xmm1, xmmword ptr [rbp + 0x448]
0x1400cbf23 0f109558040000 movups xmm2, xmmword ptr [rbp + 0x458]
0x1400cbf2a 0f109d68040000 movups xmm3, xmmword ptr [rbp + 0x468]
0x1400cbf31 0f115830 movups xmmword ptr [rax + 0x30], xmm3
0x1400cbf35 0f115020 movups xmmword ptr [rax + 0x20], xmm2
0x1400cbf39 0f114810 movups xmmword ptr [rax + 0x10], xmm1
0x1400cbf3d 0f1100 movups xmmword ptr [rax], xmm0
0x1400cbf40 0f1085c8030000 movups xmm0, xmmword ptr [rbp + 0x3c8]
0x1400cbf47 0f108dd8030000 movups xmm1, xmmword ptr [rbp + 0x3d8]
0x1400cbf4e 0f1095e8030000 movups xmm2, xmmword ptr [rbp + 0x3e8]
0x1400cbf55 0f109df8030000 movups xmm3, xmmword ptr [rbp + 0x3f8]
0x1400cbf5c 0f114070 movups xmmword ptr [rax + 0x70], xmm0
0x1400cbf60 0f118880000000 movups xmmword ptr [rax + 0x80], xmm1
0x1400cbf67 0f119090000000 movups xmmword ptr [rax + 0x90], xmm2
0x1400cbf6e 0f1198a0000000 movups xmmword ptr [rax + 0xa0], xmm3
0x1400cbf75 0f108508040000 movups xmm0, xmmword ptr [rbp + 0x408]
0x1400cbf7c 0f1180b0000000 movups xmmword ptr [rax + 0xb0], xmm0
0x1400cbf83 0f108518040000 movups xmm0, xmmword ptr [rbp + 0x418]
0x1400cbf8a 0f1180c0000000 movups xmmword ptr [rax + 0xc0], xmm0
0x1400cbf91 0f108528040000 movups xmm0, xmmword ptr [rbp + 0x428]
0x1400cbf98 0f1180d0000000 movups xmmword ptr [rax + 0xd0], xmm0
0x1400cbf9f 0f1085b8030000 movups xmm0, xmmword ptr [rbp + 0x3b8]
0x1400cbfa6 0f118040010000 movups xmmword ptr [rax + 0x140], xmm0
0x1400cbfad 0f1085a8030000 movups xmm0, xmmword ptr [rbp + 0x3a8]
0x1400cbfb4 0f118030010000 movups xmmword ptr [rax + 0x130], xmm0
0x1400cbfbb 0f108598030000 movups xmm0, xmmword ptr [rbp + 0x398]
0x1400cbfc2 0f118020010000 movups xmmword ptr [rax + 0x120], xmm0
0x1400cbfc9 0f108558030000 movups xmm0, xmmword ptr [rbp + 0x358]
0x1400cbfd0 0f108d68030000 movups xmm1, xmmword ptr [rbp + 0x368]
0x1400cbfd7 0f109578030000 movups xmm2, xmmword ptr [rbp + 0x378]
0x1400cbfde 0f109d88030000 movups xmm3, xmmword ptr [rbp + 0x388]
0x1400cbfe5 0f119810010000 movups xmmword ptr [rax + 0x110], xmm3
0x1400cbfec 0f119000010000 movups xmmword ptr [rax + 0x100], xmm2
0x1400cbff3 0f1188f0000000 movups xmmword ptr [rax + 0xf0], xmm1
0x1400cbffa 0f1180e0000000 movups xmmword ptr [rax + 0xe0], xmm0
0x1400cc001 0f1045a8 movups xmm0, xmmword ptr [rbp - 0x58]
0x1400cc005 0f104db8 movups xmm1, xmmword ptr [rbp - 0x48]
0x1400cc009 0f1055c8 movups xmm2, xmmword ptr [rbp - 0x38]
0x1400cc00d 0f105dd8 movups xmm3, xmmword ptr [rbp - 0x28]
0x1400cc011 0f118050010000 movups xmmword ptr [rax + 0x150], xmm0
0x1400cc018 0f118860010000 movups xmmword ptr [rax + 0x160], xmm1
0x1400cc01f 0f119070010000 movups xmmword ptr [rax + 0x170], xmm2
0x1400cc026 0f119880010000 movups xmmword ptr [rax + 0x180], xmm3
0x1400cc02d 0f1045e8 movups xmm0, xmmword ptr [rbp - 0x18]
0x1400cc031 0f118090010000 movups xmmword ptr [rax + 0x190], xmm0
0x1400cc038 0f1045f8 movups xmm0, xmmword ptr [rbp - 8]
0x1400cc03c 0f1180a0010000 movups xmmword ptr [rax + 0x1a0], xmm0
0x1400cc043 0f104508 movups xmm0, xmmword ptr [rbp + 8]
0x1400cc047 0f1180b0010000 movups xmmword ptr [rax + 0x1b0], xmm0
0x1400cc04e 48c745b004000000 mov qword ptr [rbp - 0x50], 4
0x1400cc056 488945b8 mov qword ptr [rbp - 0x48], rax
0x1400cc05a 48c745c004000000 mov qword ptr [rbp - 0x40], 4
0x1400cc062 48c745c800000000 mov qword ptr [rbp - 0x38], 0
0x1400cc06a 48c745d008000000 mov qword ptr [rbp - 0x30], 8
0x1400cc072 0f57c0 xorps xmm0, xmm0
0x1400cc075 0f1145d8 movups xmmword ptr [rbp - 0x28], xmm0
0x1400cc079 48895da8 mov qword ptr [rbp - 0x58], rbx
0x1400cc07d 488d4de0 lea rcx, [rbp - 0x20]
0x1400cc081 e84a450a00 call 0x1401705d0
0x1400cc086 0f104508 movups xmm0, xmmword ptr [rbp + 8]
0x1400cc08a 0f114660 movups xmmword ptr [rsi + 0x60], xmm0
0x1400cc08e 0f1045f8 movups xmm0, xmmword ptr [rbp - 8]
0x1400cc092 0f114650 movups xmmword ptr [rsi + 0x50], xmm0
0x1400cc096 0f1045e8 movups xmm0, xmmword ptr [rbp - 0x18]
0x1400cc09a 0f114640 movups xmmword ptr [rsi + 0x40], xmm0
0x1400cc09e 0f1045a8 movups xmm0, xmmword ptr [rbp - 0x58]
0x1400cc0a2 0f104db8 movups xmm1, xmmword ptr [rbp - 0x48]
0x1400cc0a6 0f1055c8 movups xmm2, xmmword ptr [rbp - 0x38]
0x1400cc0aa 0f105dd8 movups xmm3, xmmword ptr [rbp - 0x28]
0x1400cc0ae 0f115e30 movups xmmword ptr [rsi + 0x30], xmm3
0x1400cc0b2 0f115620 movups xmmword ptr [rsi + 0x20], xmm2
0x1400cc0b6 0f114e10 movups xmmword ptr [rsi + 0x10], xmm1
0x1400cc0ba 0f1106 movups xmmword ptr [rsi], xmm0
0x1400cc0bd 4889f0 mov rax, rsi
0x1400cc0c0 4881c438050000 add rsp, 0x538
0x1400cc0c7 5b pop rbx
0x1400cc0c8 5f pop rdi
0x1400cc0c9 5e pop rsi
0x1400cc0ca 5d pop rbp
0x1400cc0cb c3 ret 
0x1400cc0cc b908000000 mov ecx, 8
0x1400cc0d1 bac0010000 mov edx, 0x1c0
0x1400cc0d6 e825cf8002 call 0x1428d9000
0x1400cc0db b908000000 mov ecx, 8
0x1400cc0e0 bab0030000 mov edx, 0x3b0
0x1400cc0e5 e816cf8002 call 0x1428d9000
0x1400cc0ea 0f0b ud2 
0x1400cc0ec 0f1f4000 nop dword ptr [rax]
0x1400cc0f0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1400cc0f5 55 push rbp
0x1400cc0f6 56 push rsi
0x1400cc0f7 57 push rdi
0x1400cc0f8 53 push rbx
0x1400cc0f9 4883ec28 sub rsp, 0x28
0x1400cc0fd 488daa80000000 lea rbp, [rdx + 0x80]
0x1400cc104 488d4da8 lea rcx, [rbp - 0x58]
0x1400cc108 e8a3555000 call 0x1405d16b0
0x1400cc10d 90 nop 
0x1400cc10e 4883c428 add rsp, 0x28
0x1400cc112 5b pop rbx
0x1400cc113 5f pop rdi
0x1400cc114 5e pop rsi
0x1400cc115 5d pop rbp
0x1400cc116 c3 ret 
0x1400cc117 660f1f840000000000 nop word ptr [rax + rax]
0x1400cc120 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1400cc125 55 push rbp
0x1400cc126 56 push rsi
0x1400cc127 57 push rdi
0x1400cc128 53 push rbx
0x1400cc129 4883ec28 sub rsp, 0x28
0x1400cc12d 488daa80000000 lea rbp, [rdx + 0x80]
0x1400cc134 bac0010000 mov edx, 0x1c0
0x1400cc139 41b808000000 mov r8d, 8
0x1400cc13f 488b8da8040000 mov rcx, qword ptr [rbp + 0x4a8]
0x1400cc146 e8d57a5400 call 0x140613c20
0x1400cc14b 90 nop 
0x1400cc14c 4883c428 add rsp, 0x28
0x1400cc150 5b pop rbx
0x1400cc151 5f pop rdi
0x1400cc152 5e pop rsi
0x1400cc153 5d pop rbp
0x1400cc154 c3 ret 
0x1400cc155 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1400cc160 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1400cc165 55 push rbp
0x1400cc166 56 push rsi
0x1400cc167 57 push rdi
0x1400cc168 53 push rbx
0x1400cc169 4883ec28 sub rsp, 0x28
0x1400cc16d 488daa80000000 lea rbp, [rdx + 0x80]
0x1400cc174 488d8d38040000 lea rcx, [rbp + 0x438]
0x1400cc17b e800d7f5ff call 0x140029880
0x1400cc180 90 nop 
0x1400cc181 4883c428 add rsp, 0x28
0x1400cc185 5b pop rbx
0x1400cc186 5f pop rdi
0x1400cc187 5e pop rsi
0x1400cc188 5d pop rbp
0x1400cc189 c3 ret 
0x1400cc18a 660f1f440000 nop word ptr [rax + rax]
0x1400cc190 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1400cc195 55 push rbp
0x1400cc196 56 push rsi
0x1400cc197 57 push rdi
0x1400cc198 53 push rbx
0x1400cc199 4883ec28 sub rsp, 0x28
0x1400cc19d 488daa80000000 lea rbp, [rdx + 0x80]
0x1400cc1a4 488d8dc8030000 lea rcx, [rbp + 0x3c8]
0x1400cc1ab e8d0d6f5ff call 0x140029880
0x1400cc1b0 90 nop 
0x1400cc1b1 4883c428 add rsp, 0x28
0x1400cc1b5 5b pop rbx
0x1400cc1b6 5f pop rdi
0x1400cc1b7 5e pop rsi
0x1400cc1b8 5d pop rbp
0x1400cc1b9 c3 ret 
0x1400cc1ba 660f1f440000 nop word ptr [rax + rax]
0x1400cc1c0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1400cc1c5 55 push rbp
0x1400cc1c6 56 push rsi
0x1400cc1c7 57 push rdi
0x1400cc1c8 53 push rbx
0x1400cc1c9 4883ec28 sub rsp, 0x28
0x1400cc1cd 488daa80000000 lea rbp, [rdx + 0x80]
0x1400cc1d4 488d8d58030000 lea rcx, [rbp + 0x358]
0x1400cc1db e8a0d6f5ff call 0x140029880
0x1400cc1e0 90 nop 
0x1400cc1e1 4883c428 add rsp, 0x28
0x1400cc1e5 5b pop rbx
0x1400cc1e6 5f pop rdi
0x1400cc1e7 5e pop rsi
0x1400cc1e8 5d pop rbp
0x1400cc1e9 c3 ret 
0x1400cc1ea 660f1f440000 nop word ptr [rax + rax]
0x1400cc1f0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1400cc1f5 55 push rbp
0x1400cc1f6 56 push rsi
0x1400cc1f7 57 push rdi
0x1400cc1f8 53 push rbx
0x1400cc1f9 4883ec28 sub rsp, 0x28
0x1400cc1fd 488daa80000000 lea rbp, [rdx + 0x80]
0x1400cc204 488d4da8 lea rcx, [rbp - 0x58]
0x1400cc208 e873d6f5ff call 0x140029880
0x1400cc20d 90 nop 
0x1400cc20e 4883c428 add rsp, 0x28
0x1400cc212 5b pop rbx
0x1400cc213 5f pop rdi
0x1400cc214 5e pop rsi
0x1400cc215 5d pop rbp
0x1400cc216 c3 ret 
