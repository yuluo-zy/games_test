0x1416ea060 55 push rbp
0x1416ea061 4157 push r15
0x1416ea063 4156 push r14
0x1416ea065 4154 push r12
0x1416ea067 56 push rsi
0x1416ea068 57 push rdi
0x1416ea069 53 push rbx
0x1416ea06a 4881ec40010000 sub rsp, 0x140
0x1416ea071 488dac2480000000 lea rbp, [rsp + 0x80]
0x1416ea079 48c785b8000000feffffff mov qword ptr [rbp + 0xb8], 0xfffffffffffffffe
0x1416ea084 4889ce mov rsi, rcx
0x1416ea087 0fb602 movzx eax, byte ptr [rdx]
0x1416ea08a 83f803 cmp eax, 3
0x1416ea08d 8985a0000000 mov dword ptr [rbp + 0xa0], eax
0x1416ea093 48895578 mov qword ptr [rbp + 0x78], rdx
0x1416ea097 0f84ed000000 je 0x1416ea18a
0x1416ea09d 83f805 cmp eax, 5
0x1416ea0a0 0f85cf010000 jne 0x1416ea275
0x1416ea0a6 4c8b7a08 mov r15, qword ptr [rdx + 8]
0x1416ea0aa 4c8b6210 mov r12, qword ptr [rdx + 0x10]
0x1416ea0ae 488b7a18 mov rdi, qword ptr [rdx + 0x18]
0x1416ea0b2 31c0 xor eax, eax
0x1416ea0b4 4d85ff test r15, r15
0x1416ea0b7 0f95c0 setne al
0x1416ea0ba 4889f9 mov rcx, rdi
0x1416ea0bd 490f44cf cmove rcx, r15
0x1416ea0c1 48894520 mov qword ptr [rbp + 0x20], rax
0x1416ea0c5 48c7452800000000 mov qword ptr [rbp + 0x28], 0
0x1416ea0cd 4c897d30 mov qword ptr [rbp + 0x30], r15
0x1416ea0d1 4c896538 mov qword ptr [rbp + 0x38], r12
0x1416ea0d5 48894540 mov qword ptr [rbp + 0x40], rax
0x1416ea0d9 48c7454800000000 mov qword ptr [rbp + 0x48], 0
0x1416ea0e1 4c897d50 mov qword ptr [rbp + 0x50], r15
0x1416ea0e5 4c896558 mov qword ptr [rbp + 0x58], r12
0x1416ea0e9 48894d60 mov qword ptr [rbp + 0x60], rcx
0x1416ea0ed 488d8d80000000 lea rcx, [rbp + 0x80]
0x1416ea0f4 488d5520 lea rdx, [rbp + 0x20]
0x1416ea0f8 e85323d5ff call 0x14143c450
0x1416ea0fd 488b8580000000 mov rax, qword ptr [rbp + 0x80]
0x1416ea104 4885c0 test rax, rax
0x1416ea107 7444 je 0x1416ea14d
0x1416ea109 488b8d90000000 mov rcx, qword ptr [rbp + 0x90]
0x1416ea110 488d1449 lea rdx, [rcx + rcx*2]
0x1416ea114 4c8b84d068010000 mov r8, qword ptr [rax + rdx*8 + 0x168]
0x1416ea11c 4c8b8cd070010000 mov r9, qword ptr [rax + rdx*8 + 0x170]
0x1416ea124 488b9cd078010000 mov rbx, qword ptr [rax + rdx*8 + 0x178]
0x1416ea12c 48c1e105 shl rcx, 5
0x1416ea130 0f100408 movups xmm0, xmmword ptr [rax + rcx]
0x1416ea134 0f104c0810 movups xmm1, xmmword ptr [rax + rcx + 0x10]
0x1416ea139 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
0x1416ea13d 0f294df0 movaps xmmword ptr [rbp - 0x10], xmm1
0x1416ea141 4c89c0 mov rax, r8
0x1416ea144 48f7d8 neg rax
0x1416ea147 0f81b5010000 jno 0x1416ea302
0x1416ea14d c685800000000b mov byte ptr [rbp + 0x80], 0xb
0x1416ea154 488d158d505a01 lea rdx, [rip + 0x15a508d]
0x1416ea15b 4c8d0596505a01 lea r8, [rip + 0x15a5096]
0x1416ea162 488d8d80000000 lea rcx, [rbp + 0x80]
0x1416ea169 e8d2f92001 call 0x1428f9b40
0x1416ea16e 48894608 mov qword ptr [rsi + 8], rax
0x1416ea172 c60601 mov byte ptr [rsi], 1
0x1416ea175 c685b700000001 mov byte ptr [rbp + 0xb7], 1
0x1416ea17c 488d4d20 lea rcx, [rbp + 0x20]
0x1416ea180 e86bdc19ff call 0x140887df0
0x1416ea185 e973020000 jmp 0x1416ea3fd
0x1416ea18a 4c8b7a08 mov r15, qword ptr [rdx + 8]
0x1416ea18e 4c8b6210 mov r12, qword ptr [rdx + 0x10]
0x1416ea192 488b7a18 mov rdi, qword ptr [rdx + 0x18]
0x1416ea196 41b606 mov r14b, 6
0x1416ea199 4c89f8 mov rax, r15
0x1416ea19c 4c89e1 mov rcx, r12
0x1416ea19f 4889fb mov rbx, rdi
0x1416ea1a2 48894520 mov qword ptr [rbp + 0x20], rax
0x1416ea1a6 48894d28 mov qword ptr [rbp + 0x28], rcx
0x1416ea1aa 48895d30 mov qword ptr [rbp + 0x30], rbx
0x1416ea1ae 0f2845c0 movaps xmm0, xmmword ptr [rbp - 0x40]
0x1416ea1b2 0f114539 movups xmmword ptr [rbp + 0x39], xmm0
0x1416ea1b6 0f1045cf movups xmm0, xmmword ptr [rbp - 0x31]
0x1416ea1ba 0f114548 movups xmmword ptr [rbp + 0x48], xmm0
0x1416ea1be 44887538 mov byte ptr [rbp + 0x38], r14b
0x1416ea1c2 c685b700000001 mov byte ptr [rbp + 0xb7], 1
0x1416ea1c9 488d4de0 lea rcx, [rbp - 0x20]
0x1416ea1cd 488d5520 lea rdx, [rbp + 0x20]
0x1416ea1d1 e80a78eeff call 0x1415d19e0
0x1416ea1d6 0fb65de0 movzx ebx, byte ptr [rbp - 0x20]
0x1416ea1da 488b45e8 mov rax, qword ptr [rbp - 0x18]
0x1416ea1de 80fb02 cmp bl, 2
0x1416ea1e1 7439 je 0x1416ea21c
0x1416ea1e3 488b4d00 mov rcx, qword ptr [rbp]
0x1416ea1e7 48898d98000000 mov qword ptr [rbp + 0x98], rcx
0x1416ea1ee 0f1045f0 movups xmm0, xmmword ptr [rbp - 0x10]
0x1416ea1f2 0f118588000000 movups xmmword ptr [rbp + 0x88], xmm0
0x1416ea1f9 48898580000000 mov qword ptr [rbp + 0x80], rax
0x1416ea200 c685b700000001 mov byte ptr [rbp + 0xb7], 1
0x1416ea207 488d8d80000000 lea rcx, [rbp + 0x80]
0x1416ea20e e8eddd49ff call 0x140b88000
0x1416ea213 4885c0 test rax, rax
0x1416ea216 0f849f010000 je 0x1416ea3bb
0x1416ea21c 48894608 mov qword ptr [rsi + 8], rax
0x1416ea220 b001 mov al, 1
0x1416ea222 8806 mov byte ptr [rsi], al
0x1416ea224 8b85a0000000 mov eax, dword ptr [rbp + 0xa0]
0x1416ea22a 83f803 cmp eax, 3
0x1416ea22d 0f84ca010000 je 0x1416ea3fd
0x1416ea233 83f805 cmp eax, 5
0x1416ea236 0f84c1010000 je 0x1416ea3fd
0x1416ea23c 4c89bda0000000 mov qword ptr [rbp + 0xa0], r15
0x1416ea243 4c896578 mov qword ptr [rbp + 0x78], r12
0x1416ea247 4c89e1 mov rcx, r12
0x1416ea24a 4889fa mov rdx, rdi
0x1416ea24d e81eeb19ff call 0x140888d70
0x1416ea252 488b95a0000000 mov rdx, qword ptr [rbp + 0xa0]
0x1416ea259 4885d2 test rdx, rdx
0x1416ea25c 488b4d78 mov rcx, qword ptr [rbp + 0x78]
0x1416ea260 0f8497010000 je 0x1416ea3fd
0x1416ea266 48c1e205 shl rdx, 5
0x1416ea26a 41b808000000 mov r8d, 8
0x1416ea270 e971010000 jmp 0x1416ea3e6
0x1416ea275 0f1002 movups xmm0, xmmword ptr [rdx]
0x1416ea278 0f104a10 movups xmm1, xmmword ptr [rdx + 0x10]
0x1416ea27c 0f294d30 movaps xmmword ptr [rbp + 0x30], xmm1
0x1416ea280 0f294520 movaps xmmword ptr [rbp + 0x20], xmm0
0x1416ea284 488d4de0 lea rcx, [rbp - 0x20]
0x1416ea288 488d5520 lea rdx, [rbp + 0x20]
0x1416ea28c e87ff91d01 call 0x1428c9c10
0x1416ea291 488d15904f5a01 lea rdx, [rip + 0x15a4f90]
0x1416ea298 4c8d05594f5a01 lea r8, [rip + 0x15a4f59]
0x1416ea29f 488d4de0 lea rcx, [rbp - 0x20]
0x1416ea2a3 e858f71d01 call 0x1428c9a00
0x1416ea2a8 48894608 mov qword ptr [rsi + 8], rax
0x1416ea2ac c60601 mov byte ptr [rsi], 1
0x1416ea2af 0fb64520 movzx eax, byte ptr [rbp + 0x20]
0x1416ea2b3 83f803 cmp eax, 3
0x1416ea2b6 0f8241010000 jb 0x1416ea3fd
0x1416ea2bc 0f8411010000 je 0x1416ea3d3
0x1416ea2c2 83f804 cmp eax, 4
0x1416ea2c5 0f8522010000 jne 0x1416ea3ed
0x1416ea2cb 488b4d30 mov rcx, qword ptr [rbp + 0x30]
0x1416ea2cf 488b5538 mov rdx, qword ptr [rbp + 0x38]
0x1416ea2d3 48898da8000000 mov qword ptr [rbp + 0xa8], rcx
0x1416ea2da e891ea19ff call 0x140888d70
0x1416ea2df 488b5528 mov rdx, qword ptr [rbp + 0x28]
0x1416ea2e3 4885d2 test rdx, rdx
0x1416ea2e6 0f8411010000 je 0x1416ea3fd
0x1416ea2ec 48c1e205 shl rdx, 5
0x1416ea2f0 41b808000000 mov r8d, 8
0x1416ea2f6 488b8da8000000 mov rcx, qword ptr [rbp + 0xa8]
0x1416ea2fd e9e4000000 jmp 0x1416ea3e6
0x1416ea302 4c894d70 mov qword ptr [rbp + 0x70], r9
0x1416ea306 4c8985a8000000 mov qword ptr [rbp + 0xa8], r8
0x1416ea30d 0f2845e0 movaps xmm0, xmmword ptr [rbp - 0x20]
0x1416ea311 0f284df0 movaps xmm1, xmmword ptr [rbp - 0x10]
0x1416ea315 0f298d90000000 movaps xmmword ptr [rbp + 0x90], xmm1
0x1416ea31c 0f298580000000 movaps xmmword ptr [rbp + 0x80], xmm0
0x1416ea323 488d4da8 lea rcx, [rbp - 0x58]
0x1416ea327 488d5520 lea rdx, [rbp + 0x20]
0x1416ea32b e82021d5ff call 0x14143c450
0x1416ea330 488b45a8 mov rax, qword ptr [rbp - 0x58]
0x1416ea334 4885c0 test rax, rax
0x1416ea337 7445 je 0x1416ea37e
0x1416ea339 488b4db8 mov rcx, qword ptr [rbp - 0x48]
0x1416ea33d 488d1449 lea rdx, [rcx + rcx*2]
0x1416ea341 4c8b84d078010000 mov r8, qword ptr [rax + rdx*8 + 0x178]
0x1416ea349 4c8945f0 mov qword ptr [rbp - 0x10], r8
0x1416ea34d 0f1084d068010000 movups xmm0, xmmword ptr [rax + rdx*8 + 0x168]
0x1416ea355 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
0x1416ea359 48c1e105 shl rcx, 5
0x1416ea35d 0f100408 movups xmm0, xmmword ptr [rax + rcx]
0x1416ea361 0f104c0810 movups xmm1, xmmword ptr [rax + rcx + 0x10]
0x1416ea366 0f1145f8 movups xmmword ptr [rbp - 8], xmm0
0x1416ea36a 0f114d08 movups xmmword ptr [rbp + 8], xmm1
0x1416ea36e 488b55e0 mov rdx, qword ptr [rbp - 0x20]
0x1416ea372 4889d0 mov rax, rdx
0x1416ea375 48f7d8 neg rax
0x1416ea378 0f8194000000 jno 0x1416ea412
0x1416ea37e 448ab580000000 mov r14b, byte ptr [rbp + 0x80]
0x1416ea385 0f108581000000 movups xmm0, xmmword ptr [rbp + 0x81]
0x1416ea38c 0f2945c0 movaps xmmword ptr [rbp - 0x40], xmm0
0x1416ea390 0f108590000000 movups xmm0, xmmword ptr [rbp + 0x90]
0x1416ea397 0f1145cf movups xmmword ptr [rbp - 0x31], xmm0
0x1416ea39b c685b700000001 mov byte ptr [rbp + 0xb7], 1
0x1416ea3a2 488d4d20 lea rcx, [rbp + 0x20]
0x1416ea3a6 e845da19ff call 0x140887df0
0x1416ea3ab 488b85a8000000 mov rax, qword ptr [rbp + 0xa8]
0x1416ea3b2 488b4d70 mov rcx, qword ptr [rbp + 0x70]
0x1416ea3b6 e9e7fdffff jmp 0x1416ea1a2
0x1416ea3bb 885e01 mov byte ptr [rsi + 1], bl
0x1416ea3be 31c0 xor eax, eax
0x1416ea3c0 8806 mov byte ptr [rsi], al
0x1416ea3c2 8b85a0000000 mov eax, dword ptr [rbp + 0xa0]
0x1416ea3c8 83f803 cmp eax, 3
0x1416ea3cb 0f8562feffff jne 0x1416ea233
0x1416ea3d1 eb2a jmp 0x1416ea3fd
0x1416ea3d3 488b5528 mov rdx, qword ptr [rbp + 0x28]
0x1416ea3d7 4885d2 test rdx, rdx
0x1416ea3da 7421 je 0x1416ea3fd
0x1416ea3dc 488b4d30 mov rcx, qword ptr [rbp + 0x30]
0x1416ea3e0 41b801000000 mov r8d, 1
0x1416ea3e6 e83598f2fe call 0x140613c20
0x1416ea3eb eb10 jmp 0x1416ea3fd
0x1416ea3ed 488d4d28 lea rcx, [rbp + 0x28]
0x1416ea3f1 c685b700000000 mov byte ptr [rbp + 0xb7], 0
0x1416ea3f8 e843eb1aff call 0x140898f40
0x1416ea3fd 4889f0 mov rax, rsi
0x1416ea400 4881c440010000 add rsp, 0x140
0x1416ea407 5b pop rbx
0x1416ea408 5f pop rdi
0x1416ea409 5e pop rsi
0x1416ea40a 415c pop r12
0x1416ea40c 415e pop r14
0x1416ea40e 415f pop r15
0x1416ea410 5d pop rbp
0x1416ea411 c3 ret 
0x1416ea412 730f jae 0x1416ea423
0x1416ea414 488b4de8 mov rcx, qword ptr [rbp - 0x18]
0x1416ea418 41b801000000 mov r8d, 1
0x1416ea41e e8fd97f2fe call 0x140613c20
0x1416ea423 0fb645f8 movzx eax, byte ptr [rbp - 8]
0x1416ea427 83f803 cmp eax, 3
0x1416ea42a 7231 jb 0x1416ea45d
0x1416ea42c 7410 je 0x1416ea43e
0x1416ea42e 488d4d00 lea rcx, [rbp]
0x1416ea432 83f804 cmp eax, 4
0x1416ea435 7521 jne 0x1416ea458
0x1416ea437 e854ef19ff call 0x140889390
0x1416ea43c eb1f jmp 0x1416ea45d
0x1416ea43e 488b5500 mov rdx, qword ptr [rbp]
0x1416ea442 4885d2 test rdx, rdx
0x1416ea445 7416 je 0x1416ea45d
0x1416ea447 488b4d08 mov rcx, qword ptr [rbp + 8]
0x1416ea44b 41b801000000 mov r8d, 1
0x1416ea451 e8ca97f2fe call 0x140613c20
0x1416ea456 eb05 jmp 0x1416ea45d
0x1416ea458 e8e3ea1aff call 0x140898f40
0x1416ea45d c645e00b mov byte ptr [rbp - 0x20], 0xb
0x1416ea461 488d15804d5a01 lea rdx, [rip + 0x15a4d80]
0x1416ea468 4c8d05894d5a01 lea r8, [rip + 0x15a4d89]
0x1416ea46f 488d4de0 lea rcx, [rbp - 0x20]
0x1416ea473 e8c8f62001 call 0x1428f9b40
0x1416ea478 48894608 mov qword ptr [rsi + 8], rax
0x1416ea47c c60601 mov byte ptr [rsi], 1
0x1416ea47f 0fb68580000000 movzx eax, byte ptr [rbp + 0x80]
0x1416ea486 83f803 cmp eax, 3
0x1416ea489 723a jb 0x1416ea4c5
0x1416ea48b 7413 je 0x1416ea4a0
0x1416ea48d 488d8d88000000 lea rcx, [rbp + 0x88]
0x1416ea494 83f804 cmp eax, 4
0x1416ea497 7527 jne 0x1416ea4c0
0x1416ea499 e8f2ee19ff call 0x140889390
0x1416ea49e eb25 jmp 0x1416ea4c5
0x1416ea4a0 488b9588000000 mov rdx, qword ptr [rbp + 0x88]
0x1416ea4a7 4885d2 test rdx, rdx
0x1416ea4aa 7419 je 0x1416ea4c5
0x1416ea4ac 488b8d90000000 mov rcx, qword ptr [rbp + 0x90]
0x1416ea4b3 41b801000000 mov r8d, 1
0x1416ea4b9 e86297f2fe call 0x140613c20
0x1416ea4be eb05 jmp 0x1416ea4c5
0x1416ea4c0 e87bea1aff call 0x140898f40
0x1416ea4c5 488b95a8000000 mov rdx, qword ptr [rbp + 0xa8]
0x1416ea4cc 4885d2 test rdx, rdx
0x1416ea4cf 0f84a0fcffff je 0x1416ea175
0x1416ea4d5 41b801000000 mov r8d, 1
0x1416ea4db 488b4d70 mov rcx, qword ptr [rbp + 0x70]
0x1416ea4df e83c97f2fe call 0x140613c20
0x1416ea4e4 e98cfcffff jmp 0x1416ea175
0x1416ea4e9 0f1f8000000000 nop dword ptr [rax]
0x1416ea4f0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416ea4f5 55 push rbp
0x1416ea4f6 4157 push r15
0x1416ea4f8 4156 push r14
0x1416ea4fa 4154 push r12
0x1416ea4fc 56 push rsi
0x1416ea4fd 57 push rdi
0x1416ea4fe 53 push rbx
0x1416ea4ff 4883ec20 sub rsp, 0x20
0x1416ea503 488daa80000000 lea rbp, [rdx + 0x80]
0x1416ea50a 488d4d20 lea rcx, [rbp + 0x20]
0x1416ea50e e8ddd819ff call 0x140887df0
0x1416ea513 c685b700000001 mov byte ptr [rbp + 0xb7], 1
0x1416ea51a 4883c420 add rsp, 0x20
0x1416ea51e 5b pop rbx
0x1416ea51f 5f pop rdi
0x1416ea520 5e pop rsi
0x1416ea521 415c pop r12
0x1416ea523 415e pop r14
0x1416ea525 415f pop r15
0x1416ea527 5d pop rbp
0x1416ea528 c3 ret 
0x1416ea529 0f1f8000000000 nop dword ptr [rax]
0x1416ea530 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416ea535 55 push rbp
0x1416ea536 4157 push r15
0x1416ea538 4156 push r14
0x1416ea53a 4154 push r12
0x1416ea53c 56 push rsi
0x1416ea53d 57 push rdi
0x1416ea53e 53 push rbx
0x1416ea53f 4883ec20 sub rsp, 0x20
0x1416ea543 488daa80000000 lea rbp, [rdx + 0x80]
0x1416ea54a 4883bda800000000 cmp qword ptr [rbp + 0xa8], 0
0x1416ea552 7416 je 0x1416ea56a
0x1416ea554 41b801000000 mov r8d, 1
0x1416ea55a 488b4d70 mov rcx, qword ptr [rbp + 0x70]
0x1416ea55e 488b95a8000000 mov rdx, qword ptr [rbp + 0xa8]
0x1416ea565 e8b696f2fe call 0x140613c20
0x1416ea56a 90 nop 
0x1416ea56b 4883c420 add rsp, 0x20
0x1416ea56f 5b pop rbx
0x1416ea570 5f pop rdi
0x1416ea571 5e pop rsi
0x1416ea572 415c pop r12
0x1416ea574 415e pop r14
0x1416ea576 415f pop r15
0x1416ea578 5d pop rbp
0x1416ea579 c3 ret 
0x1416ea57a 660f1f440000 nop word ptr [rax + rax]
0x1416ea580 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416ea585 55 push rbp
0x1416ea586 4157 push r15
0x1416ea588 4156 push r14
0x1416ea58a 4154 push r12
0x1416ea58c 56 push rsi
0x1416ea58d 57 push rdi
0x1416ea58e 53 push rbx
0x1416ea58f 4883ec20 sub rsp, 0x20
0x1416ea593 488daa80000000 lea rbp, [rdx + 0x80]
0x1416ea59a 4883bda000000000 cmp qword ptr [rbp + 0xa0], 0
0x1416ea5a2 741a je 0x1416ea5be
0x1416ea5a4 488b95a0000000 mov rdx, qword ptr [rbp + 0xa0]
0x1416ea5ab 48c1e205 shl rdx, 5
0x1416ea5af 41b808000000 mov r8d, 8
0x1416ea5b5 488b4d78 mov rcx, qword ptr [rbp + 0x78]
0x1416ea5b9 e86296f2fe call 0x140613c20
0x1416ea5be 90 nop 
0x1416ea5bf 4883c420 add rsp, 0x20
0x1416ea5c3 5b pop rbx
0x1416ea5c4 5f pop rdi
0x1416ea5c5 5e pop rsi
0x1416ea5c6 415c pop r12
0x1416ea5c8 415e pop r14
0x1416ea5ca 415f pop r15
0x1416ea5cc 5d pop rbp
0x1416ea5cd c3 ret 
0x1416ea5ce 6690 nop 
0x1416ea5d0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416ea5d5 55 push rbp
0x1416ea5d6 4157 push r15
0x1416ea5d8 4156 push r14
0x1416ea5da 4154 push r12
0x1416ea5dc 56 push rsi
0x1416ea5dd 57 push rdi
0x1416ea5de 53 push rbx
0x1416ea5df 4883ec20 sub rsp, 0x20
0x1416ea5e3 488daa80000000 lea rbp, [rdx + 0x80]
0x1416ea5ea 488d4d20 lea rcx, [rbp + 0x20]
0x1416ea5ee e89d3417ff call 0x14085da90
0x1416ea5f3 c685b700000000 mov byte ptr [rbp + 0xb7], 0
0x1416ea5fa 4883c420 add rsp, 0x20
0x1416ea5fe 5b pop rbx
0x1416ea5ff 5f pop rdi
0x1416ea600 5e pop rsi
0x1416ea601 415c pop r12
0x1416ea603 415e pop r14
0x1416ea605 415f pop r15
0x1416ea607 5d pop rbp
0x1416ea608 c3 ret 
0x1416ea609 0f1f8000000000 nop dword ptr [rax]
0x1416ea610 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416ea615 55 push rbp
0x1416ea616 4157 push r15
0x1416ea618 4156 push r14
0x1416ea61a 4154 push r12
0x1416ea61c 56 push rsi
0x1416ea61d 57 push rdi
0x1416ea61e 53 push rbx
0x1416ea61f 4883ec20 sub rsp, 0x20
0x1416ea623 488daa80000000 lea rbp, [rdx + 0x80]
0x1416ea62a 488b5528 mov rdx, qword ptr [rbp + 0x28]
0x1416ea62e 4885d2 test rdx, rdx
0x1416ea631 7416 je 0x1416ea649
0x1416ea633 48c1e205 shl rdx, 5
0x1416ea637 41b808000000 mov r8d, 8
0x1416ea63d 488b8da8000000 mov rcx, qword ptr [rbp + 0xa8]
0x1416ea644 e8d795f2fe call 0x140613c20
0x1416ea649 c685b700000000 mov byte ptr [rbp + 0xb7], 0
0x1416ea650 4883c420 add rsp, 0x20
0x1416ea654 5b pop rbx
0x1416ea655 5f pop rdi
0x1416ea656 5e pop rsi
0x1416ea657 415c pop r12
0x1416ea659 415e pop r14
0x1416ea65b 415f pop r15
0x1416ea65d 5d pop rbp
0x1416ea65e c3 ret 
0x1416ea65f 90 nop 
0x1416ea660 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416ea665 55 push rbp
0x1416ea666 4157 push r15
0x1416ea668 4156 push r14
0x1416ea66a 4154 push r12
0x1416ea66c 56 push rsi
0x1416ea66d 57 push rdi
0x1416ea66e 53 push rbx
0x1416ea66f 4883ec20 sub rsp, 0x20
0x1416ea673 488daa80000000 lea rbp, [rdx + 0x80]
0x1416ea67a 80bdb700000000 cmp byte ptr [rbp + 0xb7], 0
0x1416ea681 7415 je 0x1416ea698
0x1416ea683 80bda000000003 cmp byte ptr [rbp + 0xa0], 3
0x1416ea68a 740c je 0x1416ea698
0x1416ea68c 0fb685a0000000 movzx eax, byte ptr [rbp + 0xa0]
0x1416ea693 83f805 cmp eax, 5
0x1416ea696 750f jne 0x1416ea6a7
0x1416ea698 4883c420 add rsp, 0x20
0x1416ea69c 5b pop rbx
0x1416ea69d 5f pop rdi
0x1416ea69e 5e pop rsi
0x1416ea69f 415c pop r12
0x1416ea6a1 415e pop r14
0x1416ea6a3 415f pop r15
0x1416ea6a5 5d pop rbp
0x1416ea6a6 c3 ret 
0x1416ea6a7 488b4d78 mov rcx, qword ptr [rbp + 0x78]
0x1416ea6ab e8e03317ff call 0x14085da90
0x1416ea6b0 ebe6 jmp 0x1416ea698
0x1416ea6b2 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1416ea6c0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416ea6c5 55 push rbp
0x1416ea6c6 4157 push r15
0x1416ea6c8 4156 push r14
0x1416ea6ca 4154 push r12
0x1416ea6cc 56 push rsi
0x1416ea6cd 57 push rdi
0x1416ea6ce 53 push rbx
0x1416ea6cf 4883ec20 sub rsp, 0x20
0x1416ea6d3 488daa80000000 lea rbp, [rdx + 0x80]
0x1416ea6da 488d8d80000000 lea rcx, [rbp + 0x80]
0x1416ea6e1 e8aa3317ff call 0x14085da90
0x1416ea6e6 90 nop 
0x1416ea6e7 4883c420 add rsp, 0x20
0x1416ea6eb 5b pop rbx
0x1416ea6ec 5f pop rdi
0x1416ea6ed 5e pop rsi
0x1416ea6ee 415c pop r12
0x1416ea6f0 415e pop r14
0x1416ea6f2 415f pop r15
0x1416ea6f4 5d pop rbp
0x1416ea6f5 c3 ret 
