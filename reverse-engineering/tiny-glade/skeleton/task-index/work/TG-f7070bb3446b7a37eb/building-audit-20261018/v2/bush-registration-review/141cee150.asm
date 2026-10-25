0x141cee150 55 push rbp
0x141cee151 56 push rsi
0x141cee152 57 push rdi
0x141cee153 4881ec40040000 sub rsp, 0x440
0x141cee15a 488dac2480000000 lea rbp, [rsp + 0x80]
0x141cee162 48c785b8030000feffffff mov qword ptr [rbp + 0x3b8], 0xfffffffffffffffe
0x141cee16d 4889ce mov rsi, rcx
0x141cee170 0f57c0 xorps xmm0, xmm0
0x141cee173 0f1145c8 movups xmmword ptr [rbp - 0x38], xmm0
0x141cee177 0f1145e0 movups xmmword ptr [rbp - 0x20], xmm0
0x141cee17b 0f1145f8 movups xmmword ptr [rbp - 8], xmm0
0x141cee17f 0f114510 movups xmmword ptr [rbp + 0x10], xmm0
0x141cee183 0f114528 movups xmmword ptr [rbp + 0x28], xmm0
0x141cee187 c7453800000000 mov dword ptr [rbp + 0x38], 0
0x141cee18e 0f114560 movups xmmword ptr [rbp + 0x60], xmm0
0x141cee192 0f114578 movups xmmword ptr [rbp + 0x78], xmm0
0x141cee196 0f118590000000 movups xmmword ptr [rbp + 0x90], xmm0
0x141cee19d 0f1185a8000000 movups xmmword ptr [rbp + 0xa8], xmm0
0x141cee1a4 0f1185c0000000 movups xmmword ptr [rbp + 0xc0], xmm0
0x141cee1ab c785d000000000000000 mov dword ptr [rbp + 0xd0], 0
0x141cee1b5 48c745a800000000 mov qword ptr [rbp - 0x58], 0
0x141cee1bd 48c745b008000000 mov qword ptr [rbp - 0x50], 8
0x141cee1c5 48c745b800000000 mov qword ptr [rbp - 0x48], 0
0x141cee1cd 48c745c010000000 mov qword ptr [rbp - 0x40], 0x10
0x141cee1d5 48c745d810000000 mov qword ptr [rbp - 0x28], 0x10
0x141cee1dd 48c745f010000000 mov qword ptr [rbp - 0x10], 0x10
0x141cee1e5 48c7450810000000 mov qword ptr [rbp + 8], 0x10
0x141cee1ed 48c7452010000000 mov qword ptr [rbp + 0x20], 0x10
0x141cee1f5 48b80000000000000080 movabs rax, 0x8000000000000000
0x141cee1ff 48894540 mov qword ptr [rbp + 0x40], rax
0x141cee203 488d0d2a942401 lea rcx, [rip + 0x124942a]
0x141cee20a 48894d48 mov qword ptr [rbp + 0x48], rcx
0x141cee20e 48c7455057000000 mov qword ptr [rbp + 0x50], 0x57
0x141cee216 48c7455810000000 mov qword ptr [rbp + 0x58], 0x10
0x141cee21e 48c7457010000000 mov qword ptr [rbp + 0x70], 0x10
0x141cee226 48c7858800000010000000 mov qword ptr [rbp + 0x88], 0x10
0x141cee231 48c785a000000010000000 mov qword ptr [rbp + 0xa0], 0x10
0x141cee23c 48c785b800000010000000 mov qword ptr [rbp + 0xb8], 0x10
0x141cee247 c785d800000000000000 mov dword ptr [rbp + 0xd8], 0
0x141cee251 66c785dc0000000100 mov word ptr [rbp + 0xdc], 1
0x141cee25a 488985d0010000 mov qword ptr [rbp + 0x1d0], rax
0x141cee261 c785b003000000000000 mov dword ptr [rbp + 0x3b0], 0
0x141cee26b 0fb60597699401 movzx eax, byte ptr [rip + 0x1946997]
0x141cee272 b910040000 mov ecx, 0x410
0x141cee277 ba08000000 mov edx, 8
0x141cee27c e88f5992fe call 0x140613c10
0x141cee281 4885c0 test rax, rax
0x141cee284 7435 je 0x141cee2bb
0x141cee286 4889c7 mov rdi, rax
0x141cee289 488d55a8 lea rdx, [rbp - 0x58]
0x141cee28d 41b810040000 mov r8d, 0x410
0x141cee293 4889c1 mov rcx, rax
0x141cee296 e86559c300 call 0x142923c00
0x141cee29b 4c8d057e922401 lea r8, [rip + 0x124927e]
0x141cee2a2 4889f1 mov rcx, rsi
0x141cee2a5 4889fa mov rdx, rdi
0x141cee2a8 e86368fffe call 0x140ce4b10
0x141cee2ad 4889f0 mov rax, rsi
0x141cee2b0 4881c440040000 add rsp, 0x440
0x141cee2b7 5f pop rdi
0x141cee2b8 5e pop rsi
0x141cee2b9 5d pop rbp
0x141cee2ba c3 ret 
0x141cee2bb b908000000 mov ecx, 8
0x141cee2c0 ba10040000 mov edx, 0x410
0x141cee2c5 e836adbe00 call 0x1428d9000
0x141cee2ca 0f0b ud2 
0x141cee2cc 0f1f4000 nop dword ptr [rax]
0x141cee2d0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x141cee2d5 55 push rbp
0x141cee2d6 56 push rsi
0x141cee2d7 57 push rdi
0x141cee2d8 4883ec20 sub rsp, 0x20
0x141cee2dc 488daa80000000 lea rbp, [rdx + 0x80]
0x141cee2e3 488d4da8 lea rcx, [rbp - 0x58]
0x141cee2e7 e844fcb8fe call 0x14087df30
0x141cee2ec 90 nop 
0x141cee2ed 4883c420 add rsp, 0x20
0x141cee2f1 5f pop rdi
0x141cee2f2 5e pop rsi
0x141cee2f3 5d pop rbp
0x141cee2f4 c3 ret 
