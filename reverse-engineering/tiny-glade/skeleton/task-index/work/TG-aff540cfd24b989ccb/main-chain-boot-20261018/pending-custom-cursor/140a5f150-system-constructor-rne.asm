0x140a5f150 55 push rbp
0x140a5f151 56 push rsi
0x140a5f152 57 push rdi
0x140a5f153 4881ecb0010000 sub rsp, 0x1b0
0x140a5f15a 488dac2480000000 lea rbp, [rsp + 0x80]
0x140a5f162 48c78528010000feffffff mov qword ptr [rbp + 0x128], 0xfffffffffffffffe
0x140a5f16d 4889ce mov rsi, rcx
0x140a5f170 0f57c0 xorps xmm0, xmm0
0x140a5f173 0f1145c8 movups xmmword ptr [rbp - 0x38], xmm0
0x140a5f177 0f1145e0 movups xmmword ptr [rbp - 0x20], xmm0
0x140a5f17b 0f1145f8 movups xmmword ptr [rbp - 8], xmm0
0x140a5f17f 0f114510 movups xmmword ptr [rbp + 0x10], xmm0
0x140a5f183 0f114528 movups xmmword ptr [rbp + 0x28], xmm0
0x140a5f187 c7453800000000 mov dword ptr [rbp + 0x38], 0
0x140a5f18e 0f114560 movups xmmword ptr [rbp + 0x60], xmm0
0x140a5f192 0f114578 movups xmmword ptr [rbp + 0x78], xmm0
0x140a5f196 0f118590000000 movups xmmword ptr [rbp + 0x90], xmm0
0x140a5f19d 0f1185a8000000 movups xmmword ptr [rbp + 0xa8], xmm0
0x140a5f1a4 0f1185c0000000 movups xmmword ptr [rbp + 0xc0], xmm0
0x140a5f1ab c785d000000000000000 mov dword ptr [rbp + 0xd0], 0
0x140a5f1b5 48c745a800000000 mov qword ptr [rbp - 0x58], 0
0x140a5f1bd 48c745b008000000 mov qword ptr [rbp - 0x50], 8
0x140a5f1c5 48c745b800000000 mov qword ptr [rbp - 0x48], 0
0x140a5f1cd 48c745c010000000 mov qword ptr [rbp - 0x40], 0x10
0x140a5f1d5 48c745d810000000 mov qword ptr [rbp - 0x28], 0x10
0x140a5f1dd 48c745f010000000 mov qword ptr [rbp - 0x10], 0x10
0x140a5f1e5 48c7450810000000 mov qword ptr [rbp + 8], 0x10
0x140a5f1ed 48c7452010000000 mov qword ptr [rbp + 0x20], 0x10
0x140a5f1f5 48b80000000000000080 movabs rax, 0x8000000000000000
0x140a5f1ff 48894540 mov qword ptr [rbp + 0x40], rax
0x140a5f203 488d0da64b0702 lea rcx, [rip + 0x2074ba6]
0x140a5f20a 48894d48 mov qword ptr [rbp + 0x48], rcx
0x140a5f20e 48c745502d000000 mov qword ptr [rbp + 0x50], 0x2d
0x140a5f216 48c7455810000000 mov qword ptr [rbp + 0x58], 0x10
0x140a5f21e 48c7457010000000 mov qword ptr [rbp + 0x70], 0x10
0x140a5f226 48c7858800000010000000 mov qword ptr [rbp + 0x88], 0x10
0x140a5f231 48c785a000000010000000 mov qword ptr [rbp + 0xa0], 0x10
0x140a5f23c 48c785b800000010000000 mov qword ptr [rbp + 0xb8], 0x10
0x140a5f247 c785d800000000000000 mov dword ptr [rbp + 0xd8], 0
0x140a5f251 66c785dc0000000100 mov word ptr [rbp + 0xdc], 1
0x140a5f25a 488985e0000000 mov qword ptr [rbp + 0xe0], rax
0x140a5f261 c7852001000000000000 mov dword ptr [rbp + 0x120], 0
0x140a5f26b 0fb6059759bd02 movzx eax, byte ptr [rip + 0x2bd5997]
0x140a5f272 b980010000 mov ecx, 0x180
0x140a5f277 ba08000000 mov edx, 8
0x140a5f27c e88f49bbff call 0x140613c10
0x140a5f281 4885c0 test rax, rax
0x140a5f284 7435 je 0x140a5f2bb
0x140a5f286 4889c7 mov rdi, rax
0x140a5f289 488d55a8 lea rdx, [rbp - 0x58]
0x140a5f28d 41b880010000 mov r8d, 0x180
0x140a5f293 4889c1 mov rcx, rax
0x140a5f296 e86549ec01 call 0x142923c00
0x140a5f29b 4c8d05ce460702 lea r8, [rip + 0x20746ce]
0x140a5f2a2 4889f1 mov rcx, rsi
0x140a5f2a5 4889fa mov rdx, rdi
0x140a5f2a8 e863582800 call 0x140ce4b10
0x140a5f2ad 4889f0 mov rax, rsi
0x140a5f2b0 4881c4b0010000 add rsp, 0x1b0
0x140a5f2b7 5f pop rdi
0x140a5f2b8 5e pop rsi
0x140a5f2b9 5d pop rbp
0x140a5f2ba c3 ret 
0x140a5f2bb b908000000 mov ecx, 8
0x140a5f2c0 ba80010000 mov edx, 0x180
0x140a5f2c5 e8369de701 call 0x1428d9000
0x140a5f2ca 0f0b ud2 
0x140a5f2cc 0f1f4000 nop dword ptr [rax]
0x140a5f2d0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a5f2d5 55 push rbp
0x140a5f2d6 56 push rsi
0x140a5f2d7 57 push rdi
0x140a5f2d8 4883ec20 sub rsp, 0x20
0x140a5f2dc 488daa80000000 lea rbp, [rdx + 0x80]
0x140a5f2e3 488d4da8 lea rcx, [rbp - 0x58]
0x140a5f2e7 e8f460b7ff call 0x1405d53e0
0x140a5f2ec 90 nop 
0x140a5f2ed 4883c420 add rsp, 0x20
0x140a5f2f1 5f pop rdi
0x140a5f2f2 5e pop rsi
0x140a5f2f3 5d pop rbp
0x140a5f2f4 c3 ret 
