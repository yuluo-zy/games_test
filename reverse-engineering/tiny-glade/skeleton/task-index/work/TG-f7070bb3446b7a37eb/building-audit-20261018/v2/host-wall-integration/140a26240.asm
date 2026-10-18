0x140a26240 55 push rbp
0x140a26241 56 push rsi
0x140a26242 57 push rdi
0x140a26243 4881ecc0020000 sub rsp, 0x2c0
0x140a2624a 488dac2480000000 lea rbp, [rsp + 0x80]
0x140a26252 48c78538020000feffffff mov qword ptr [rbp + 0x238], 0xfffffffffffffffe
0x140a2625d 4889ce mov rsi, rcx
0x140a26260 0f57c0 xorps xmm0, xmm0
0x140a26263 0f1145c0 movups xmmword ptr [rbp - 0x40], xmm0
0x140a26267 0f1145d8 movups xmmword ptr [rbp - 0x28], xmm0
0x140a2626b 0f1145f0 movups xmmword ptr [rbp - 0x10], xmm0
0x140a2626f 0f114508 movups xmmword ptr [rbp + 8], xmm0
0x140a26273 0f114520 movups xmmword ptr [rbp + 0x20], xmm0
0x140a26277 c7453000000000 mov dword ptr [rbp + 0x30], 0
0x140a2627e 0f114558 movups xmmword ptr [rbp + 0x58], xmm0
0x140a26282 0f114570 movups xmmword ptr [rbp + 0x70], xmm0
0x140a26286 0f118588000000 movups xmmword ptr [rbp + 0x88], xmm0
0x140a2628d 0f1185a0000000 movups xmmword ptr [rbp + 0xa0], xmm0
0x140a26294 0f1185b8000000 movups xmmword ptr [rbp + 0xb8], xmm0
0x140a2629b c785c800000000000000 mov dword ptr [rbp + 0xc8], 0
0x140a262a5 48c745a000000000 mov qword ptr [rbp - 0x60], 0
0x140a262ad 48c745a808000000 mov qword ptr [rbp - 0x58], 8
0x140a262b5 48c745b000000000 mov qword ptr [rbp - 0x50], 0
0x140a262bd 48c745b810000000 mov qword ptr [rbp - 0x48], 0x10
0x140a262c5 48c745d010000000 mov qword ptr [rbp - 0x30], 0x10
0x140a262cd 48c745e810000000 mov qword ptr [rbp - 0x18], 0x10
0x140a262d5 48c7450010000000 mov qword ptr [rbp], 0x10
0x140a262dd 48c7451810000000 mov qword ptr [rbp + 0x18], 0x10
0x140a262e5 48b80000000000000080 movabs rax, 0x8000000000000000
0x140a262ef 48894538 mov qword ptr [rbp + 0x38], rax
0x140a262f3 488d0d9e440a02 lea rcx, [rip + 0x20a449e]
0x140a262fa 48894d40 mov qword ptr [rbp + 0x40], rcx
0x140a262fe 48c7454850000000 mov qword ptr [rbp + 0x48], 0x50
0x140a26306 48c7455010000000 mov qword ptr [rbp + 0x50], 0x10
0x140a2630e 48c7456810000000 mov qword ptr [rbp + 0x68], 0x10
0x140a26316 48c7858000000010000000 mov qword ptr [rbp + 0x80], 0x10
0x140a26321 48c7859800000010000000 mov qword ptr [rbp + 0x98], 0x10
0x140a2632c 48c785b000000010000000 mov qword ptr [rbp + 0xb0], 0x10
0x140a26337 c785d000000000000000 mov dword ptr [rbp + 0xd0], 0
0x140a26341 66c785d40000000100 mov word ptr [rbp + 0xd4], 1
0x140a2634a 488985d0010000 mov qword ptr [rbp + 0x1d0], rax
0x140a26351 c7853002000000000000 mov dword ptr [rbp + 0x230], 0
0x140a2635b 0fb605a7e8c002 movzx eax, byte ptr [rip + 0x2c0e8a7]
0x140a26362 b998020000 mov ecx, 0x298
0x140a26367 ba08000000 mov edx, 8
0x140a2636c e89fd8beff call 0x140613c10
0x140a26371 4885c0 test rax, rax
0x140a26374 7435 je 0x140a263ab
0x140a26376 4889c7 mov rdi, rax
0x140a26379 488d55a0 lea rdx, [rbp - 0x60]
0x140a2637d 41b898020000 mov r8d, 0x298
0x140a26383 4889c1 mov rcx, rax
0x140a26386 e875d8ef01 call 0x142923c00
0x140a2638b 4c8d05d63f0a02 lea r8, [rip + 0x20a3fd6]
0x140a26392 4889f1 mov rcx, rsi
0x140a26395 4889fa mov rdx, rdi
0x140a26398 e873e72b00 call 0x140ce4b10
0x140a2639d 4889f0 mov rax, rsi
0x140a263a0 4881c4c0020000 add rsp, 0x2c0
0x140a263a7 5f pop rdi
0x140a263a8 5e pop rsi
0x140a263a9 5d pop rbp
0x140a263aa c3 ret 
0x140a263ab b908000000 mov ecx, 8
0x140a263b0 ba98020000 mov edx, 0x298
0x140a263b5 e8462ceb01 call 0x1428d9000
0x140a263ba 0f0b ud2 
0x140a263bc 0f1f4000 nop dword ptr [rax]
0x140a263c0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a263c5 55 push rbp
0x140a263c6 56 push rsi
0x140a263c7 57 push rdi
0x140a263c8 4883ec20 sub rsp, 0x20
0x140a263cc 488daa80000000 lea rbp, [rdx + 0x80]
0x140a263d3 488d4da0 lea rcx, [rbp - 0x60]
0x140a263d7 e8d48fbaff call 0x1405cf3b0
0x140a263dc 90 nop 
0x140a263dd 4883c420 add rsp, 0x20
0x140a263e1 5f pop rdi
0x140a263e2 5e pop rsi
0x140a263e3 5d pop rbp
0x140a263e4 c3 ret 
