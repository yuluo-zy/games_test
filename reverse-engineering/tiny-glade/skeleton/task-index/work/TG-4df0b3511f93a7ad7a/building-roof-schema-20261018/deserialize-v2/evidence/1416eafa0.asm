0x1416eafa0 55 push rbp
0x1416eafa1 4157 push r15
0x1416eafa3 4156 push r14
0x1416eafa5 4154 push r12
0x1416eafa7 56 push rsi
0x1416eafa8 57 push rdi
0x1416eafa9 53 push rbx
0x1416eafaa 4881ec40010000 sub rsp, 0x140
0x1416eafb1 488dac2480000000 lea rbp, [rsp + 0x80]
0x1416eafb9 48c785b8000000feffffff mov qword ptr [rbp + 0xb8], 0xfffffffffffffffe
0x1416eafc4 4889ce mov rsi, rcx
0x1416eafc7 0fb602 movzx eax, byte ptr [rdx]
0x1416eafca 83f803 cmp eax, 3
0x1416eafcd 8985a0000000 mov dword ptr [rbp + 0xa0], eax
0x1416eafd3 48895578 mov qword ptr [rbp + 0x78], rdx
0x1416eafd7 0f84ed000000 je 0x1416eb0ca
0x1416eafdd 83f805 cmp eax, 5
0x1416eafe0 0f85cf010000 jne 0x1416eb1b5
0x1416eafe6 4c8b7a08 mov r15, qword ptr [rdx + 8]
0x1416eafea 4c8b6210 mov r12, qword ptr [rdx + 0x10]
0x1416eafee 488b7a18 mov rdi, qword ptr [rdx + 0x18]
0x1416eaff2 31c0 xor eax, eax
0x1416eaff4 4d85ff test r15, r15
0x1416eaff7 0f95c0 setne al
0x1416eaffa 4889f9 mov rcx, rdi
0x1416eaffd 490f44cf cmove rcx, r15
0x1416eb001 48894520 mov qword ptr [rbp + 0x20], rax
0x1416eb005 48c7452800000000 mov qword ptr [rbp + 0x28], 0
0x1416eb00d 4c897d30 mov qword ptr [rbp + 0x30], r15
0x1416eb011 4c896538 mov qword ptr [rbp + 0x38], r12
0x1416eb015 48894540 mov qword ptr [rbp + 0x40], rax
0x1416eb019 48c7454800000000 mov qword ptr [rbp + 0x48], 0
0x1416eb021 4c897d50 mov qword ptr [rbp + 0x50], r15
0x1416eb025 4c896558 mov qword ptr [rbp + 0x58], r12
0x1416eb029 48894d60 mov qword ptr [rbp + 0x60], rcx
0x1416eb02d 488d8d80000000 lea rcx, [rbp + 0x80]
0x1416eb034 488d5520 lea rdx, [rbp + 0x20]
0x1416eb038 e81314d5ff call 0x14143c450
0x1416eb03d 488b8580000000 mov rax, qword ptr [rbp + 0x80]
0x1416eb044 4885c0 test rax, rax
0x1416eb047 7444 je 0x1416eb08d
0x1416eb049 488b8d90000000 mov rcx, qword ptr [rbp + 0x90]
0x1416eb050 488d1449 lea rdx, [rcx + rcx*2]
0x1416eb054 4c8b84d068010000 mov r8, qword ptr [rax + rdx*8 + 0x168]
0x1416eb05c 4c8b8cd070010000 mov r9, qword ptr [rax + rdx*8 + 0x170]
0x1416eb064 488b9cd078010000 mov rbx, qword ptr [rax + rdx*8 + 0x178]
0x1416eb06c 48c1e105 shl rcx, 5
0x1416eb070 0f100408 movups xmm0, xmmword ptr [rax + rcx]
0x1416eb074 0f104c0810 movups xmm1, xmmword ptr [rax + rcx + 0x10]
0x1416eb079 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
0x1416eb07d 0f294df0 movaps xmmword ptr [rbp - 0x10], xmm1
0x1416eb081 4c89c0 mov rax, r8
0x1416eb084 48f7d8 neg rax
0x1416eb087 0f81b5010000 jno 0x1416eb242
0x1416eb08d c685800000000b mov byte ptr [rbp + 0x80], 0xb
0x1416eb094 488d154d415a01 lea rdx, [rip + 0x15a414d]
0x1416eb09b 4c8d0556415a01 lea r8, [rip + 0x15a4156]
0x1416eb0a2 488d8d80000000 lea rcx, [rbp + 0x80]
0x1416eb0a9 e892ea2001 call 0x1428f9b40
0x1416eb0ae 48894608 mov qword ptr [rsi + 8], rax
0x1416eb0b2 c60601 mov byte ptr [rsi], 1
0x1416eb0b5 c685b700000001 mov byte ptr [rbp + 0xb7], 1
0x1416eb0bc 488d4d20 lea rcx, [rbp + 0x20]
0x1416eb0c0 e82bcd19ff call 0x140887df0
0x1416eb0c5 e973020000 jmp 0x1416eb33d
0x1416eb0ca 4c8b7a08 mov r15, qword ptr [rdx + 8]
0x1416eb0ce 4c8b6210 mov r12, qword ptr [rdx + 0x10]
0x1416eb0d2 488b7a18 mov rdi, qword ptr [rdx + 0x18]
0x1416eb0d6 41b606 mov r14b, 6
0x1416eb0d9 4c89f8 mov rax, r15
0x1416eb0dc 4c89e1 mov rcx, r12
0x1416eb0df 4889fb mov rbx, rdi
0x1416eb0e2 48894520 mov qword ptr [rbp + 0x20], rax
0x1416eb0e6 48894d28 mov qword ptr [rbp + 0x28], rcx
0x1416eb0ea 48895d30 mov qword ptr [rbp + 0x30], rbx
0x1416eb0ee 0f2845c0 movaps xmm0, xmmword ptr [rbp - 0x40]
0x1416eb0f2 0f114539 movups xmmword ptr [rbp + 0x39], xmm0
0x1416eb0f6 0f1045cf movups xmm0, xmmword ptr [rbp - 0x31]
0x1416eb0fa 0f114548 movups xmmword ptr [rbp + 0x48], xmm0
0x1416eb0fe 44887538 mov byte ptr [rbp + 0x38], r14b
0x1416eb102 c685b700000001 mov byte ptr [rbp + 0xb7], 1
0x1416eb109 488d4de0 lea rcx, [rbp - 0x20]
0x1416eb10d 488d5520 lea rdx, [rbp + 0x20]
0x1416eb111 e8aa72eeff call 0x1415d23c0
0x1416eb116 0fb65de0 movzx ebx, byte ptr [rbp - 0x20]
0x1416eb11a 488b45e8 mov rax, qword ptr [rbp - 0x18]
0x1416eb11e 80fb02 cmp bl, 2
0x1416eb121 7439 je 0x1416eb15c
0x1416eb123 488b4d00 mov rcx, qword ptr [rbp]
0x1416eb127 48898d98000000 mov qword ptr [rbp + 0x98], rcx
0x1416eb12e 0f1045f0 movups xmm0, xmmword ptr [rbp - 0x10]
0x1416eb132 0f118588000000 movups xmmword ptr [rbp + 0x88], xmm0
0x1416eb139 48898580000000 mov qword ptr [rbp + 0x80], rax
0x1416eb140 c685b700000001 mov byte ptr [rbp + 0xb7], 1
0x1416eb147 488d8d80000000 lea rcx, [rbp + 0x80]
0x1416eb14e e8adce49ff call 0x140b88000
0x1416eb153 4885c0 test rax, rax
0x1416eb156 0f849f010000 je 0x1416eb2fb
0x1416eb15c 48894608 mov qword ptr [rsi + 8], rax
0x1416eb160 b001 mov al, 1
0x1416eb162 8806 mov byte ptr [rsi], al
0x1416eb164 8b85a0000000 mov eax, dword ptr [rbp + 0xa0]
0x1416eb16a 83f803 cmp eax, 3
0x1416eb16d 0f84ca010000 je 0x1416eb33d
0x1416eb173 83f805 cmp eax, 5
0x1416eb176 0f84c1010000 je 0x1416eb33d
0x1416eb17c 4c89bda0000000 mov qword ptr [rbp + 0xa0], r15
0x1416eb183 4c896578 mov qword ptr [rbp + 0x78], r12
0x1416eb187 4c89e1 mov rcx, r12
0x1416eb18a 4889fa mov rdx, rdi
0x1416eb18d e8dedb19ff call 0x140888d70
0x1416eb192 488b95a0000000 mov rdx, qword ptr [rbp + 0xa0]
0x1416eb199 4885d2 test rdx, rdx
0x1416eb19c 488b4d78 mov rcx, qword ptr [rbp + 0x78]
0x1416eb1a0 0f8497010000 je 0x1416eb33d
0x1416eb1a6 48c1e205 shl rdx, 5
0x1416eb1aa 41b808000000 mov r8d, 8
0x1416eb1b0 e971010000 jmp 0x1416eb326
0x1416eb1b5 0f1002 movups xmm0, xmmword ptr [rdx]
0x1416eb1b8 0f104a10 movups xmm1, xmmword ptr [rdx + 0x10]
0x1416eb1bc 0f294d30 movaps xmmword ptr [rbp + 0x30], xmm1
0x1416eb1c0 0f294520 movaps xmmword ptr [rbp + 0x20], xmm0
0x1416eb1c4 488d4de0 lea rcx, [rbp - 0x20]
0x1416eb1c8 488d5520 lea rdx, [rbp + 0x20]
0x1416eb1cc e83fea1d01 call 0x1428c9c10
0x1416eb1d1 488d1550405a01 lea rdx, [rip + 0x15a4050]
0x1416eb1d8 4c8d0519405a01 lea r8, [rip + 0x15a4019]
0x1416eb1df 488d4de0 lea rcx, [rbp - 0x20]
0x1416eb1e3 e818e81d01 call 0x1428c9a00
0x1416eb1e8 48894608 mov qword ptr [rsi + 8], rax
0x1416eb1ec c60601 mov byte ptr [rsi], 1
0x1416eb1ef 0fb64520 movzx eax, byte ptr [rbp + 0x20]
0x1416eb1f3 83f803 cmp eax, 3
0x1416eb1f6 0f8241010000 jb 0x1416eb33d
0x1416eb1fc 0f8411010000 je 0x1416eb313
0x1416eb202 83f804 cmp eax, 4
0x1416eb205 0f8522010000 jne 0x1416eb32d
0x1416eb20b 488b4d30 mov rcx, qword ptr [rbp + 0x30]
0x1416eb20f 488b5538 mov rdx, qword ptr [rbp + 0x38]
0x1416eb213 48898da8000000 mov qword ptr [rbp + 0xa8], rcx
0x1416eb21a e851db19ff call 0x140888d70
0x1416eb21f 488b5528 mov rdx, qword ptr [rbp + 0x28]
0x1416eb223 4885d2 test rdx, rdx
0x1416eb226 0f8411010000 je 0x1416eb33d
0x1416eb22c 48c1e205 shl rdx, 5
0x1416eb230 41b808000000 mov r8d, 8
0x1416eb236 488b8da8000000 mov rcx, qword ptr [rbp + 0xa8]
0x1416eb23d e9e4000000 jmp 0x1416eb326
0x1416eb242 4c894d70 mov qword ptr [rbp + 0x70], r9
0x1416eb246 4c8985a8000000 mov qword ptr [rbp + 0xa8], r8
0x1416eb24d 0f2845e0 movaps xmm0, xmmword ptr [rbp - 0x20]
0x1416eb251 0f284df0 movaps xmm1, xmmword ptr [rbp - 0x10]
0x1416eb255 0f298d90000000 movaps xmmword ptr [rbp + 0x90], xmm1
0x1416eb25c 0f298580000000 movaps xmmword ptr [rbp + 0x80], xmm0
0x1416eb263 488d4da8 lea rcx, [rbp - 0x58]
0x1416eb267 488d5520 lea rdx, [rbp + 0x20]
0x1416eb26b e8e011d5ff call 0x14143c450
0x1416eb270 488b45a8 mov rax, qword ptr [rbp - 0x58]
0x1416eb274 4885c0 test rax, rax
0x1416eb277 7445 je 0x1416eb2be
0x1416eb279 488b4db8 mov rcx, qword ptr [rbp - 0x48]
0x1416eb27d 488d1449 lea rdx, [rcx + rcx*2]
0x1416eb281 4c8b84d078010000 mov r8, qword ptr [rax + rdx*8 + 0x178]
0x1416eb289 4c8945f0 mov qword ptr [rbp - 0x10], r8
0x1416eb28d 0f1084d068010000 movups xmm0, xmmword ptr [rax + rdx*8 + 0x168]
0x1416eb295 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
0x1416eb299 48c1e105 shl rcx, 5
0x1416eb29d 0f100408 movups xmm0, xmmword ptr [rax + rcx]
0x1416eb2a1 0f104c0810 movups xmm1, xmmword ptr [rax + rcx + 0x10]
0x1416eb2a6 0f1145f8 movups xmmword ptr [rbp - 8], xmm0
0x1416eb2aa 0f114d08 movups xmmword ptr [rbp + 8], xmm1
0x1416eb2ae 488b55e0 mov rdx, qword ptr [rbp - 0x20]
0x1416eb2b2 4889d0 mov rax, rdx
0x1416eb2b5 48f7d8 neg rax
0x1416eb2b8 0f8194000000 jno 0x1416eb352
0x1416eb2be 448ab580000000 mov r14b, byte ptr [rbp + 0x80]
0x1416eb2c5 0f108581000000 movups xmm0, xmmword ptr [rbp + 0x81]
0x1416eb2cc 0f2945c0 movaps xmmword ptr [rbp - 0x40], xmm0
0x1416eb2d0 0f108590000000 movups xmm0, xmmword ptr [rbp + 0x90]
0x1416eb2d7 0f1145cf movups xmmword ptr [rbp - 0x31], xmm0
0x1416eb2db c685b700000001 mov byte ptr [rbp + 0xb7], 1
0x1416eb2e2 488d4d20 lea rcx, [rbp + 0x20]
0x1416eb2e6 e805cb19ff call 0x140887df0
0x1416eb2eb 488b85a8000000 mov rax, qword ptr [rbp + 0xa8]
0x1416eb2f2 488b4d70 mov rcx, qword ptr [rbp + 0x70]
0x1416eb2f6 e9e7fdffff jmp 0x1416eb0e2
0x1416eb2fb 885e01 mov byte ptr [rsi + 1], bl
0x1416eb2fe 31c0 xor eax, eax
0x1416eb300 8806 mov byte ptr [rsi], al
0x1416eb302 8b85a0000000 mov eax, dword ptr [rbp + 0xa0]
0x1416eb308 83f803 cmp eax, 3
0x1416eb30b 0f8562feffff jne 0x1416eb173
0x1416eb311 eb2a jmp 0x1416eb33d
0x1416eb313 488b5528 mov rdx, qword ptr [rbp + 0x28]
0x1416eb317 4885d2 test rdx, rdx
0x1416eb31a 7421 je 0x1416eb33d
0x1416eb31c 488b4d30 mov rcx, qword ptr [rbp + 0x30]
0x1416eb320 41b801000000 mov r8d, 1
0x1416eb326 e8f588f2fe call 0x140613c20
0x1416eb32b eb10 jmp 0x1416eb33d
0x1416eb32d 488d4d28 lea rcx, [rbp + 0x28]
0x1416eb331 c685b700000000 mov byte ptr [rbp + 0xb7], 0
0x1416eb338 e803dc1aff call 0x140898f40
0x1416eb33d 4889f0 mov rax, rsi
0x1416eb340 4881c440010000 add rsp, 0x140
0x1416eb347 5b pop rbx
0x1416eb348 5f pop rdi
0x1416eb349 5e pop rsi
0x1416eb34a 415c pop r12
0x1416eb34c 415e pop r14
0x1416eb34e 415f pop r15
0x1416eb350 5d pop rbp
0x1416eb351 c3 ret 
0x1416eb352 730f jae 0x1416eb363
0x1416eb354 488b4de8 mov rcx, qword ptr [rbp - 0x18]
0x1416eb358 41b801000000 mov r8d, 1
0x1416eb35e e8bd88f2fe call 0x140613c20
0x1416eb363 0fb645f8 movzx eax, byte ptr [rbp - 8]
0x1416eb367 83f803 cmp eax, 3
0x1416eb36a 7231 jb 0x1416eb39d
0x1416eb36c 7410 je 0x1416eb37e
0x1416eb36e 488d4d00 lea rcx, [rbp]
0x1416eb372 83f804 cmp eax, 4
0x1416eb375 7521 jne 0x1416eb398
0x1416eb377 e814e019ff call 0x140889390
0x1416eb37c eb1f jmp 0x1416eb39d
0x1416eb37e 488b5500 mov rdx, qword ptr [rbp]
0x1416eb382 4885d2 test rdx, rdx
0x1416eb385 7416 je 0x1416eb39d
0x1416eb387 488b4d08 mov rcx, qword ptr [rbp + 8]
0x1416eb38b 41b801000000 mov r8d, 1
0x1416eb391 e88a88f2fe call 0x140613c20
0x1416eb396 eb05 jmp 0x1416eb39d
0x1416eb398 e8a3db1aff call 0x140898f40
0x1416eb39d c645e00b mov byte ptr [rbp - 0x20], 0xb
0x1416eb3a1 488d15403e5a01 lea rdx, [rip + 0x15a3e40]
0x1416eb3a8 4c8d05493e5a01 lea r8, [rip + 0x15a3e49]
0x1416eb3af 488d4de0 lea rcx, [rbp - 0x20]
0x1416eb3b3 e888e72001 call 0x1428f9b40
0x1416eb3b8 48894608 mov qword ptr [rsi + 8], rax
0x1416eb3bc c60601 mov byte ptr [rsi], 1
0x1416eb3bf 0fb68580000000 movzx eax, byte ptr [rbp + 0x80]
0x1416eb3c6 83f803 cmp eax, 3
0x1416eb3c9 723a jb 0x1416eb405
0x1416eb3cb 7413 je 0x1416eb3e0
0x1416eb3cd 488d8d88000000 lea rcx, [rbp + 0x88]
0x1416eb3d4 83f804 cmp eax, 4
0x1416eb3d7 7527 jne 0x1416eb400
0x1416eb3d9 e8b2df19ff call 0x140889390
0x1416eb3de eb25 jmp 0x1416eb405
0x1416eb3e0 488b9588000000 mov rdx, qword ptr [rbp + 0x88]
0x1416eb3e7 4885d2 test rdx, rdx
0x1416eb3ea 7419 je 0x1416eb405
0x1416eb3ec 488b8d90000000 mov rcx, qword ptr [rbp + 0x90]
0x1416eb3f3 41b801000000 mov r8d, 1
0x1416eb3f9 e82288f2fe call 0x140613c20
0x1416eb3fe eb05 jmp 0x1416eb405
0x1416eb400 e83bdb1aff call 0x140898f40
0x1416eb405 488b95a8000000 mov rdx, qword ptr [rbp + 0xa8]
0x1416eb40c 4885d2 test rdx, rdx
0x1416eb40f 0f84a0fcffff je 0x1416eb0b5
0x1416eb415 41b801000000 mov r8d, 1
0x1416eb41b 488b4d70 mov rcx, qword ptr [rbp + 0x70]
0x1416eb41f e8fc87f2fe call 0x140613c20
0x1416eb424 e98cfcffff jmp 0x1416eb0b5
0x1416eb429 0f1f8000000000 nop dword ptr [rax]
0x1416eb430 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416eb435 55 push rbp
0x1416eb436 4157 push r15
0x1416eb438 4156 push r14
0x1416eb43a 4154 push r12
0x1416eb43c 56 push rsi
0x1416eb43d 57 push rdi
0x1416eb43e 53 push rbx
0x1416eb43f 4883ec20 sub rsp, 0x20
0x1416eb443 488daa80000000 lea rbp, [rdx + 0x80]
0x1416eb44a 488d4d20 lea rcx, [rbp + 0x20]
0x1416eb44e e89dc919ff call 0x140887df0
0x1416eb453 c685b700000001 mov byte ptr [rbp + 0xb7], 1
0x1416eb45a 4883c420 add rsp, 0x20
0x1416eb45e 5b pop rbx
0x1416eb45f 5f pop rdi
0x1416eb460 5e pop rsi
0x1416eb461 415c pop r12
0x1416eb463 415e pop r14
0x1416eb465 415f pop r15
0x1416eb467 5d pop rbp
0x1416eb468 c3 ret 
0x1416eb469 0f1f8000000000 nop dword ptr [rax]
0x1416eb470 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416eb475 55 push rbp
0x1416eb476 4157 push r15
0x1416eb478 4156 push r14
0x1416eb47a 4154 push r12
0x1416eb47c 56 push rsi
0x1416eb47d 57 push rdi
0x1416eb47e 53 push rbx
0x1416eb47f 4883ec20 sub rsp, 0x20
0x1416eb483 488daa80000000 lea rbp, [rdx + 0x80]
0x1416eb48a 4883bda800000000 cmp qword ptr [rbp + 0xa8], 0
0x1416eb492 7416 je 0x1416eb4aa
0x1416eb494 41b801000000 mov r8d, 1
0x1416eb49a 488b4d70 mov rcx, qword ptr [rbp + 0x70]
0x1416eb49e 488b95a8000000 mov rdx, qword ptr [rbp + 0xa8]
0x1416eb4a5 e87687f2fe call 0x140613c20
0x1416eb4aa 90 nop 
0x1416eb4ab 4883c420 add rsp, 0x20
0x1416eb4af 5b pop rbx
0x1416eb4b0 5f pop rdi
0x1416eb4b1 5e pop rsi
0x1416eb4b2 415c pop r12
0x1416eb4b4 415e pop r14
0x1416eb4b6 415f pop r15
0x1416eb4b8 5d pop rbp
0x1416eb4b9 c3 ret 
0x1416eb4ba 660f1f440000 nop word ptr [rax + rax]
0x1416eb4c0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416eb4c5 55 push rbp
0x1416eb4c6 4157 push r15
0x1416eb4c8 4156 push r14
0x1416eb4ca 4154 push r12
0x1416eb4cc 56 push rsi
0x1416eb4cd 57 push rdi
0x1416eb4ce 53 push rbx
0x1416eb4cf 4883ec20 sub rsp, 0x20
0x1416eb4d3 488daa80000000 lea rbp, [rdx + 0x80]
0x1416eb4da 4883bda000000000 cmp qword ptr [rbp + 0xa0], 0
0x1416eb4e2 741a je 0x1416eb4fe
0x1416eb4e4 488b95a0000000 mov rdx, qword ptr [rbp + 0xa0]
0x1416eb4eb 48c1e205 shl rdx, 5
0x1416eb4ef 41b808000000 mov r8d, 8
0x1416eb4f5 488b4d78 mov rcx, qword ptr [rbp + 0x78]
0x1416eb4f9 e82287f2fe call 0x140613c20
0x1416eb4fe 90 nop 
0x1416eb4ff 4883c420 add rsp, 0x20
0x1416eb503 5b pop rbx
0x1416eb504 5f pop rdi
0x1416eb505 5e pop rsi
0x1416eb506 415c pop r12
0x1416eb508 415e pop r14
0x1416eb50a 415f pop r15
0x1416eb50c 5d pop rbp
0x1416eb50d c3 ret 
0x1416eb50e 6690 nop 
0x1416eb510 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416eb515 55 push rbp
0x1416eb516 4157 push r15
0x1416eb518 4156 push r14
0x1416eb51a 4154 push r12
0x1416eb51c 56 push rsi
0x1416eb51d 57 push rdi
0x1416eb51e 53 push rbx
0x1416eb51f 4883ec20 sub rsp, 0x20
0x1416eb523 488daa80000000 lea rbp, [rdx + 0x80]
0x1416eb52a 488d4d20 lea rcx, [rbp + 0x20]
0x1416eb52e e85d2517ff call 0x14085da90
0x1416eb533 c685b700000000 mov byte ptr [rbp + 0xb7], 0
0x1416eb53a 4883c420 add rsp, 0x20
0x1416eb53e 5b pop rbx
0x1416eb53f 5f pop rdi
0x1416eb540 5e pop rsi
0x1416eb541 415c pop r12
0x1416eb543 415e pop r14
0x1416eb545 415f pop r15
0x1416eb547 5d pop rbp
0x1416eb548 c3 ret 
0x1416eb549 0f1f8000000000 nop dword ptr [rax]
0x1416eb550 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416eb555 55 push rbp
0x1416eb556 4157 push r15
0x1416eb558 4156 push r14
0x1416eb55a 4154 push r12
0x1416eb55c 56 push rsi
0x1416eb55d 57 push rdi
0x1416eb55e 53 push rbx
0x1416eb55f 4883ec20 sub rsp, 0x20
0x1416eb563 488daa80000000 lea rbp, [rdx + 0x80]
0x1416eb56a 488b5528 mov rdx, qword ptr [rbp + 0x28]
0x1416eb56e 4885d2 test rdx, rdx
0x1416eb571 7416 je 0x1416eb589
0x1416eb573 48c1e205 shl rdx, 5
0x1416eb577 41b808000000 mov r8d, 8
0x1416eb57d 488b8da8000000 mov rcx, qword ptr [rbp + 0xa8]
0x1416eb584 e89786f2fe call 0x140613c20
0x1416eb589 c685b700000000 mov byte ptr [rbp + 0xb7], 0
0x1416eb590 4883c420 add rsp, 0x20
0x1416eb594 5b pop rbx
0x1416eb595 5f pop rdi
0x1416eb596 5e pop rsi
0x1416eb597 415c pop r12
0x1416eb599 415e pop r14
0x1416eb59b 415f pop r15
0x1416eb59d 5d pop rbp
0x1416eb59e c3 ret 
0x1416eb59f 90 nop 
0x1416eb5a0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416eb5a5 55 push rbp
0x1416eb5a6 4157 push r15
0x1416eb5a8 4156 push r14
0x1416eb5aa 4154 push r12
0x1416eb5ac 56 push rsi
0x1416eb5ad 57 push rdi
0x1416eb5ae 53 push rbx
0x1416eb5af 4883ec20 sub rsp, 0x20
0x1416eb5b3 488daa80000000 lea rbp, [rdx + 0x80]
0x1416eb5ba 80bdb700000000 cmp byte ptr [rbp + 0xb7], 0
0x1416eb5c1 7415 je 0x1416eb5d8
0x1416eb5c3 80bda000000003 cmp byte ptr [rbp + 0xa0], 3
0x1416eb5ca 740c je 0x1416eb5d8
0x1416eb5cc 0fb685a0000000 movzx eax, byte ptr [rbp + 0xa0]
0x1416eb5d3 83f805 cmp eax, 5
0x1416eb5d6 750f jne 0x1416eb5e7
0x1416eb5d8 4883c420 add rsp, 0x20
0x1416eb5dc 5b pop rbx
0x1416eb5dd 5f pop rdi
0x1416eb5de 5e pop rsi
0x1416eb5df 415c pop r12
0x1416eb5e1 415e pop r14
0x1416eb5e3 415f pop r15
0x1416eb5e5 5d pop rbp
0x1416eb5e6 c3 ret 
0x1416eb5e7 488b4d78 mov rcx, qword ptr [rbp + 0x78]
0x1416eb5eb e8a02417ff call 0x14085da90
0x1416eb5f0 ebe6 jmp 0x1416eb5d8
0x1416eb5f2 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1416eb600 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416eb605 55 push rbp
0x1416eb606 4157 push r15
0x1416eb608 4156 push r14
0x1416eb60a 4154 push r12
0x1416eb60c 56 push rsi
0x1416eb60d 57 push rdi
0x1416eb60e 53 push rbx
0x1416eb60f 4883ec20 sub rsp, 0x20
0x1416eb613 488daa80000000 lea rbp, [rdx + 0x80]
0x1416eb61a 488d8d80000000 lea rcx, [rbp + 0x80]
0x1416eb621 e86a2417ff call 0x14085da90
0x1416eb626 90 nop 
0x1416eb627 4883c420 add rsp, 0x20
0x1416eb62b 5b pop rbx
0x1416eb62c 5f pop rdi
0x1416eb62d 5e pop rsi
0x1416eb62e 415c pop r12
0x1416eb630 415e pop r14
0x1416eb632 415f pop r15
0x1416eb634 5d pop rbp
0x1416eb635 c3 ret 
